//! Read-only loader for the beads issue backlog (`.beads/issues.jsonl`).
//!
//! Beads remains the single source of truth for tasks. Cairn only *reads* the
//! passive JSONL export so the dev loop can surface ready work and resolve a
//! bead id to the graph node it touches (via a `cairn-node:<id>` label). Cairn
//! never writes here; this module has no mutating surface.

use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

/// Priority assigned when a bead omits the field (treated as lowest urgency).
const DEFAULT_PRIORITY: i64 = 99;

const fn default_priority() -> i64 {
    DEFAULT_PRIORITY
}

/// One backlog item parsed from the beads export.
///
/// Unknown JSON fields are ignored, so the full bead schema can evolve without
/// breaking the reader.
#[derive(Clone, Debug, Deserialize)]
pub struct BacklogItem {
    /// Stable bead id (e.g. `cairn-kb0`).
    pub id: String,
    /// Short title.
    #[serde(default)]
    pub title: String,
    /// Lifecycle status (`open`, `closed`, ...).
    #[serde(default)]
    pub status: String,
    /// Priority; lower is more urgent (`0` = highest).
    #[serde(default = "default_priority")]
    pub priority: i64,
    /// Issue type (`task`, `bug`, ...).
    #[serde(default)]
    pub issue_type: String,
    /// Full description / body (used by `cairn get <bead>` as a `bd show` substitute).
    #[serde(default)]
    pub description: String,
    /// Free-form labels.
    #[serde(default)]
    pub labels: Vec<String>,
    /// Dependency edges declared by the bead (blockers, parent-child).
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

/// A dependency edge: `id` depends on `depends_on_id` with relationship `dep_type`.
#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    /// The bead that must be resolved first.
    #[serde(default)]
    pub depends_on_id: String,
    /// Relationship type; only `blocks` gates readiness (`parent-child` does not).
    #[serde(rename = "type", default)]
    pub dep_type: String,
}

impl BacklogItem {
    /// Returns the graph node this bead is bound to, if it carries a
    /// `cairn-node:<id>` label. Tasks without the label are unlinked.
    #[must_use]
    pub fn linked_node(&self) -> Option<&str> {
        self.labels
            .iter()
            .find_map(|label| label.strip_prefix("cairn-node:"))
    }

    /// Whether this item is open (available work).
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.status == "open"
    }

    /// Whether an open `blocks`-type dependency keeps this item from being ready.
    /// A blocker that is closed or absent from the export does not block, so
    /// missing data never hides available work.
    #[must_use]
    pub fn is_blocked(&self, open_ids: &HashSet<&str>) -> bool {
        self.dependencies
            .iter()
            .filter(|dep| dep.dep_type == "blocks")
            .any(|dep| open_ids.contains(dep.depends_on_id.as_str()))
    }
}

/// Reads all backlog items from `<root>/.beads/issues.jsonl`.
///
/// Returns an empty vec when the export is absent or unreadable; malformed
/// lines are skipped so a single bad record never blanks the backlog.
#[must_use]
pub fn read(root: &Path) -> Vec<BacklogItem> {
    let path = root.join(".beads").join("issues.jsonl");
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| serde_json::from_str::<BacklogItem>(line).ok())
        .collect()
}

/// Open items, most urgent first (priority ascending, then id), ready to pick up.
#[must_use]
pub fn ready(items: &[BacklogItem]) -> Vec<&BacklogItem> {
    let open_ids: HashSet<&str> = items
        .iter()
        .filter(|item| item.is_open())
        .map(|item| item.id.as_str())
        .collect();
    let mut open: Vec<&BacklogItem> = items
        .iter()
        .filter(|item| item.is_open() && !item.is_blocked(&open_ids))
        .collect();
    open.sort_by(|a, b| a.priority.cmp(&b.priority).then_with(|| a.id.cmp(&b.id)));
    open
}

/// Finds a single item by exact id, regardless of status.
#[must_use]
pub fn find(root: &Path, id: &str) -> Option<BacklogItem> {
    read(root).into_iter().find(|item| item.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_export(dir: &Path, lines: &[&str]) {
        let beads = dir.join(".beads");
        std::fs::create_dir_all(&beads).unwrap();
        std::fs::write(beads.join("issues.jsonl"), lines.join("\n")).unwrap();
    }

    fn tmpdir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cairn-backlog-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn read_skips_malformed_and_parses_objects() {
        let dir = tmpdir("read");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-a","title":"A","status":"open","priority":2,"issue_type":"task"}"#,
                "not json",
                r#"{"id":"cairn-b","title":"B","status":"closed","priority":0,"issue_type":"task"}"#,
            ],
        );
        let items = read(&dir);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, "cairn-a");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_missing_export_is_empty() {
        let dir = tmpdir("missing");
        assert!(read(&dir).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ready_filters_open_and_sorts_by_priority() {
        let dir = tmpdir("ready");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-low","status":"open","priority":3}"#,
                r#"{"id":"cairn-done","status":"closed","priority":0}"#,
                r#"{"id":"cairn-hi","status":"open","priority":1}"#,
            ],
        );
        let items = read(&dir);
        let ready = ready(&items);
        assert_eq!(ready.len(), 2);
        assert_eq!(ready[0].id, "cairn-hi");
        assert_eq!(ready[1].id, "cairn-low");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn linked_node_reads_cairn_node_label() {
        let dir = tmpdir("link");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-x","status":"open","labels":["beads","cairn-node:cairn.kernel.cli"]}"#,
                r#"{"id":"cairn-y","status":"open","labels":["refactor"]}"#,
            ],
        );
        let items = read(&dir);
        assert_eq!(items[0].linked_node(), Some("cairn.kernel.cli"));
        assert_eq!(items[1].linked_node(), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn find_returns_any_status_by_id() {
        let dir = tmpdir("find");
        write_export(
            &dir,
            &[r#"{"id":"cairn-z","title":"Z","status":"closed","priority":0}"#],
        );
        assert_eq!(find(&dir, "cairn-z").unwrap().title, "Z");
        assert!(find(&dir, "cairn-missing").is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ready_excludes_beads_blocked_by_open_beads() {
        let dir = tmpdir("blocked");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-blk","status":"open","priority":0,"dependencies":[{"issue_id":"cairn-blk","depends_on_id":"cairn-gate","type":"blocks"}]}"#,
                r#"{"id":"cairn-gate","status":"open","priority":5}"#,
                r#"{"id":"cairn-go","status":"open","priority":1}"#,
            ],
        );
        let items = read(&dir);
        let ids: Vec<&str> = ready(&items).iter().map(|item| item.id.as_str()).collect();
        assert!(!ids.contains(&"cairn-blk"), "blocked bead must be excluded");
        assert_eq!(ids.first(), Some(&"cairn-go"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ready_includes_bead_whose_blocker_is_closed() {
        let dir = tmpdir("unblocked");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-now","status":"open","priority":0,"dependencies":[{"issue_id":"cairn-now","depends_on_id":"cairn-done","type":"blocks"}]}"#,
                r#"{"id":"cairn-done","status":"closed","priority":5}"#,
            ],
        );
        let items = read(&dir);
        let ids: Vec<&str> = ready(&items).iter().map(|item| item.id.as_str()).collect();
        assert!(
            ids.contains(&"cairn-now"),
            "bead with a closed blocker is ready"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ready_ignores_parent_child_dependencies() {
        let dir = tmpdir("parentchild");
        write_export(
            &dir,
            &[
                r#"{"id":"cairn-child","status":"open","priority":0,"dependencies":[{"issue_id":"cairn-child","depends_on_id":"cairn-parent","type":"parent-child"}]}"#,
                r#"{"id":"cairn-parent","status":"open","priority":5}"#,
            ],
        );
        let items = read(&dir);
        let ids: Vec<&str> = ready(&items).iter().map(|item| item.id.as_str()).collect();
        assert!(
            ids.contains(&"cairn-child"),
            "parent-child is not a blocker"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
