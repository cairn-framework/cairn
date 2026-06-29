//! Loads active change directories into the artefact set.

use std::path::Path;

use super::io::path_string;
use super::{ArtefactSet, ChangeRecord};

/// Load active change directories from the conventional `meta/changes/` folder
/// into the artefact set.
///
/// Unlike the other loaders this is not pointer-driven: `meta/changes/` is a
/// fixed convention path, not a blueprint artefact pointer. The folder is
/// enumerated directly here (the same approach the export builder takes) rather
/// than via `changes::discover`, which would couple the typed artefact registry
/// to the change-application module and risk a `scanner`/`changes` cycle. Only a
/// text-only [`ChangeRecord`] is retained; deltas and artefact operations stay
/// in the `changes` module.
pub(super) fn load_changes(root: &Path, set: &mut ArtefactSet) {
    let changes_root = root.join("meta/changes");
    let Ok(entries) = std::fs::read_dir(&changes_root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        if path.file_name().is_some_and(|name| name == "archive") {
            continue;
        }
        let proposal_path = path.join("proposal.md");
        let Ok(proposal) = std::fs::read_to_string(&proposal_path) else {
            continue;
        };
        let id = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_owned();
        let title = proposal_title(&proposal).unwrap_or_else(|| id.clone());
        let design = std::fs::read_to_string(path.join("design.md")).ok();
        set.changes.push(ChangeRecord {
            id,
            path: path_string(&path),
            title,
            proposal,
            design,
        });
    }
    set.changes.sort_by(|left, right| left.id.cmp(&right.id));
}

/// Extracts the proposal title from the first markdown heading, mirroring the
/// `changes` module's convention (`# Proposal:` or a bare `# ` heading).
fn proposal_title(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.strip_prefix("# Proposal:")
            .or_else(|| line.strip_prefix("# "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_changes_loads_active_skips_archive_and_proposalless()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let root = dir.path();
        // Active change with proposal + design.
        fs::create_dir_all(root.join("meta/changes/cairn-aaa"))?;
        fs::write(
            root.join("meta/changes/cairn-aaa/proposal.md"),
            "# Proposal: First change\n\nbody\n",
        )?;
        fs::write(
            root.join("meta/changes/cairn-aaa/design.md"),
            "# Design\n\nhow\n",
        )?;
        // Active change with a bare heading and no design.
        fs::create_dir_all(root.join("meta/changes/cairn-bbb"))?;
        fs::write(
            root.join("meta/changes/cairn-bbb/proposal.md"),
            "# Bare title\n\nbody\n",
        )?;
        // Archived change: must be excluded.
        fs::create_dir_all(root.join("meta/changes/archive/cairn-old"))?;
        fs::write(
            root.join("meta/changes/archive/cairn-old/proposal.md"),
            "# Proposal: Old\n",
        )?;
        // Directory without a proposal: must be excluded.
        fs::create_dir_all(root.join("meta/changes/cairn-nope"))?;

        let mut set = ArtefactSet::default();
        load_changes(root, &mut set);

        let ids: Vec<&str> = set.changes.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids, vec!["cairn-aaa", "cairn-bbb"], "sorted active changes");
        assert_eq!(set.changes[0].title, "First change");
        assert_eq!(set.changes[0].design.as_deref(), Some("# Design\n\nhow\n"));
        assert_eq!(set.changes[1].title, "Bare title");
        assert!(set.changes[1].design.is_none(), "no design.md");
        assert!(set.changes[0].proposal.contains("body"));
        Ok(())
    }
}
