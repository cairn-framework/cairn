//! Change directory discovery, delta parsing, validation, and archive support.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::{
    artefacts::frontmatter,
    blueprint::{Ast, Edge, Node, NodeKind, parser::parse_str},
    map::Graph,
    scanner,
};

mod apply;
mod artefact_ops;
mod delta;
mod rename;
mod types;
mod validate;

use apply::{
    append_archive_log, apply_archive, archive_path, atomic_write, mutation_paths,
    replace_exact_id, restore_snapshots, snapshot_paths,
};
use artefact_ops::parse_artefact_operations;
pub use delta::parse_blueprint_delta;
use rename::{artefact_content_refs, copy_referencing_artefacts, proposal_title, read_to_string};
pub(crate) use types::*;
pub use validate::validate_change;

/// Discovers active changes under `meta/changes`.
///
/// # Errors
///
/// Returns an I/O error if `meta/changes` exists but cannot be read.
pub fn discover(root: &Path) -> io::Result<Vec<Change>> {
    let changes_root = root.join("meta/changes");
    if !changes_root.exists() {
        return Ok(Vec::new());
    }
    let mut changes = Vec::new();
    for entry in fs::read_dir(changes_root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == "archive") {
            continue;
        }
        if !path.join("proposal.md").exists() {
            continue;
        }
        changes.push(load_change(root, path));
    }
    changes.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(changes)
}

/// Loads one active change from disk.
#[must_use]
pub fn load_change(root: &Path, path: PathBuf) -> Change {
    let id = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_owned();
    let mut findings = Vec::new();
    let proposal = read_to_string(&path.join("proposal.md"), &mut findings);
    let title = proposal_title(&proposal).unwrap_or_else(|| id.clone());
    let design_path = path.join("design.md");
    let design = design_path
        .exists()
        .then(|| read_to_string(&design_path, &mut findings));
    let delta_path = path.join("blueprint.delta");
    let delta_source = if delta_path.exists() {
        read_to_string(&delta_path, &mut findings)
    } else {
        String::new()
    };
    let delta = parse_blueprint_delta(&delta_path.display().to_string(), &delta_source)
        .map_err(|message| findings.push(message))
        .unwrap_or_default();
    let artefacts = parse_artefact_operations(root, &path, &mut findings);
    Change {
        id,
        path,
        title,
        proposal,
        design,
        delta,
        artefacts,
        findings,
    }
}

/// Applies and archives an active change.
///
/// # Errors
///
/// Returns an error when validation, mutation, scan, or archive movement fails.
pub fn archive(
    root: &Path,
    blueprint_path: &Path,
    change_id: &str,
) -> Result<ArchiveReport, String> {
    let changes = discover(root).map_err(|error| error.to_string())?;
    let Some(change) = changes.into_iter().find(|change| change.id == change_id) else {
        return Err(format!("change `{change_id}` was not found"));
    };
    let before = scanner::load_project(root, blueprint_path)?;
    let validation = validate_change(&change, &before.graph);
    if !validation.is_empty() {
        return Err(validation.join("; "));
    }
    let mut mutated = mutation_paths(root, blueprint_path, &change);
    mutated.sort();
    mutated.dedup();
    let snapshots = snapshot_paths(&mutated).map_err(|error| error.to_string())?;
    if let Err(error) = apply_archive(root, blueprint_path, &change).and_then(|()| {
        let scan = scanner::load_project(root, blueprint_path)?;
        if scan.graph.has_errors() {
            let messages = scan
                .graph
                .findings
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ");
            Err(messages)
        } else {
            Ok(())
        }
    }) {
        restore_snapshots(&snapshots)
            .map_err(|restore| format!("archive failed: {error}; rollback failed: {restore}"))?;
        return Err(error);
    }
    let archive_path = archive_path(root, &change.id);
    if archive_path.exists() {
        restore_snapshots(&snapshots).map_err(|restore| {
            format!(
                "archive destination `{}` already exists; rollback failed: {restore}",
                archive_path.display()
            )
        })?;
        return Err(format!(
            "archive destination `{}` already exists",
            archive_path.display()
        ));
    }
    if let Some(parent) = archive_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::rename(&change.path, &archive_path).map_err(|error| error.to_string())?;
    scanner::scan(root, blueprint_path)?;
    append_archive_log(root, &change)?;
    Ok(ArchiveReport {
        archive_path,
        summary: operation_summary(&change),
    })
}

/// Creates a reviewable rename change without mutating current truth.
///
/// # Errors
///
/// Returns an error when the old node is missing, the target exists, or files
/// cannot be written.
pub fn create_rename_change(
    root: &Path,
    blueprint_path: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<Change, String> {
    let scan = scanner::load_project(root, blueprint_path)?;
    if !scan.graph.nodes.contains_key(old_id) {
        return Err(format!("node `{old_id}` was not found"));
    }
    if scan.graph.nodes.contains_key(new_id) {
        return Err(format!("node `{new_id}` already exists"));
    }
    let change_id = format!("rename-{old_id}-to-{new_id}");
    let change_path = root.join("meta/changes").join(&change_id);
    if change_path.exists() {
        return Err(format!("change `{change_id}` already exists"));
    }
    fs::create_dir_all(&change_path).map_err(|error| error.to_string())?;
    atomic_write(
        &change_path.join("proposal.md"),
        &format!("# Proposal: Rename {old_id} to {new_id}\n\nRename `{old_id}` to `{new_id}`.\n"),
    )?;
    let mut delta = format!("## RENAMED Nodes\n- {old_id} -> {new_id}\n");
    let mut changed_edges = Vec::new();
    for edges in scan.graph.outbound.values() {
        for edge in edges {
            if edge.from == old_id || edge.to == old_id {
                changed_edges.push(edge.clone());
            }
        }
    }
    if !changed_edges.is_empty() {
        delta.push_str("\n## RENAMED Edges\n");
        for edge in changed_edges {
            let new_from = replace_exact_id(&edge.from, old_id, new_id);
            let new_to = replace_exact_id(&edge.to, old_id, new_id);
            let _ = writeln!(
                delta,
                "{} -> {} {:?} => {} -> {} {:?}",
                edge.from, edge.to, edge.description, new_from, new_to, edge.description
            );
        }
    }
    atomic_write(&change_path.join("blueprint.delta"), &delta)?;
    copy_referencing_artefacts(root, &change_path, old_id, new_id)?;
    Ok(load_change(root, change_path))
}

/// Returns a human-readable operation count summary.
#[must_use]
pub fn operation_summary(change: &Change) -> String {
    let mut counts = BTreeMap::<String, usize>::new();
    for (label, count) in [
        ("added_nodes", change.delta.added_nodes.len()),
        ("modified_nodes", change.delta.modified_nodes.len()),
        ("removed_nodes", change.delta.removed_nodes.len()),
        ("renamed_nodes", change.delta.renamed_nodes.len()),
        ("added_edges", change.delta.added_edges.len()),
        ("modified_edges", change.delta.modified_edges.len()),
        ("removed_edges", change.delta.removed_edges.len()),
        ("renamed_edges", change.delta.renamed_edges.len()),
    ] {
        if count > 0 {
            counts.insert(label.to_owned(), count);
        }
    }
    for artefact in &change.artefacts {
        let key = format!("{:?}_artefacts", artefact.operation).to_lowercase();
        *counts.entry(key).or_default() += 1;
    }
    if counts.is_empty() {
        "no operations".to_owned()
    } else {
        counts
            .into_iter()
            .map(|(label, count)| format!("{count} {label}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Returns proposed operations related to a node or direct neighbour.
#[must_use]
pub fn operations_for_nodes(changes: &[Change], nodes: &BTreeSet<String>) -> Vec<String> {
    let mut lines = Vec::new();
    for change in changes {
        for node in &change.delta.added_nodes {
            if nodes.contains(&node.id) {
                lines.push(format!("{}: added node {}", change.id, node.id));
            }
        }
        for node in &change.delta.modified_nodes {
            if nodes.contains(&node.id) {
                lines.push(format!("{}: modified node {}", change.id, node.id));
            }
        }
        for id in &change.delta.removed_nodes {
            if nodes.contains(id) {
                lines.push(format!("{}: removed node {id}", change.id));
            }
        }
        for rename in &change.delta.renamed_nodes {
            if nodes.contains(&rename.from) || nodes.contains(&rename.to) {
                lines.push(format!(
                    "{}: renamed node {} -> {}",
                    change.id, rename.from, rename.to
                ));
            }
        }
        for edge in change
            .delta
            .added_edges
            .iter()
            .chain(change.delta.modified_edges.iter())
            .chain(change.delta.removed_edges.iter())
        {
            if nodes.contains(&edge.from) || nodes.contains(&edge.to) {
                lines.push(format!(
                    "{}: edge {} -> {} ({})",
                    change.id, edge.from, edge.to, edge.description
                ));
            }
        }
        for rename in &change.delta.renamed_edges {
            if nodes.contains(&rename.from.from)
                || nodes.contains(&rename.from.to)
                || nodes.contains(&rename.to.from)
                || nodes.contains(&rename.to.to)
            {
                lines.push(format!(
                    "{}: renamed edge {} -> {} ({})",
                    change.id, rename.from.from, rename.from.to, rename.from.description
                ));
            }
        }
        for artefact in &change.artefacts {
            if artefact_content_refs(&artefact.content, nodes) {
                lines.push(format!(
                    "{}: {:?} artefact {}",
                    change.id,
                    artefact.operation,
                    artefact.target_path.display()
                ));
            }
        }
    }
    lines
}

/// Renders active changes for generated `map.md`.
#[must_use]
pub fn active_changes_lines(changes: &[Change]) -> Vec<String> {
    changes
        .iter()
        .map(|change| {
            format!(
                "{} - {} ({})",
                change.id,
                change.title,
                operation_summary(change)
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{
        apply::apply_blueprint_delta, delta::parse_blueprint_delta, validate::validate_change,
    };
    use std::time::{SystemTime, UNIX_EPOCH};

    // ── helpers ───────────────────────────────────────────────────────────────

    fn empty_change(id: &str) -> Change {
        Change {
            id: id.to_owned(),
            path: PathBuf::from(format!("meta/changes/{id}")),
            title: id.to_owned(),
            proposal: String::new(),
            design: None,
            delta: BlueprintDelta::default(),
            artefacts: Vec::new(),
            findings: Vec::new(),
        }
    }

    fn leaf_node(id: &str) -> Node {
        Node {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: crate::blueprint::Span::point("test", 1, 1),
        }
    }

    fn edge(from: &str, to: &str) -> Edge {
        Edge {
            from: from.to_owned(),
            to: to.to_owned(),
            description: "dep".to_owned(),
            span: crate::blueprint::Span::point("test", 1, 1),
        }
    }

    fn nodes_set(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|s| (*s).to_owned()).collect()
    }

    // ── operation_summary ─────────────────────────────────────────────────────

    #[test]
    fn test_operation_summary_empty_returns_no_operations() {
        assert_eq!(operation_summary(&empty_change("c")), "no operations");
    }

    #[test]
    fn test_operation_summary_single_category() {
        let mut c = empty_change("c");
        c.delta.added_nodes = vec![leaf_node("a"), leaf_node("b")];
        assert_eq!(operation_summary(&c), "2 added_nodes");
    }

    #[test]
    fn test_operation_summary_multiple_categories_are_sorted_alphabetically() {
        let mut c = empty_change("c");
        c.delta.added_nodes = vec![leaf_node("a")];
        c.delta.removed_nodes = vec!["b".to_owned()];
        // BTreeMap sorts keys: "added_nodes" < "removed_nodes"
        assert_eq!(operation_summary(&c), "1 added_nodes, 1 removed_nodes");
    }

    #[test]
    fn test_operation_summary_artefact_key_uses_lowercase_debug_variant() {
        let mut c = empty_change("c");
        c.artefacts.push(ArtefactOperation {
            operation: ChangeOperation::Added,
            change_path: PathBuf::from("meta/changes/c/foo.md"),
            target_path: PathBuf::from("meta/decisions/foo.md"),
            renamed_from: None,
            content: String::new(),
        });
        // format!("{:?}", Added) = "Added" → to_lowercase → "added" → "added_artefacts"
        assert_eq!(operation_summary(&c), "1 added_artefacts");
    }

    // ── active_changes_lines ──────────────────────────────────────────────────

    #[test]
    fn test_active_changes_lines_format() {
        let mut c = empty_change("phase-1");
        c.title = "Add API node".to_owned();
        c.delta.added_nodes = vec![leaf_node("a")];
        let lines = active_changes_lines(&[c]);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "phase-1 - Add API node (1 added_nodes)");
    }

    #[test]
    fn test_active_changes_lines_empty_returns_empty_vec() {
        assert!(active_changes_lines(&[]).is_empty());
    }

    // ── operations_for_nodes ──────────────────────────────────────────────────

    #[test]
    fn test_operations_for_nodes_added_node_in_set() {
        let mut c = empty_change("c");
        c.delta.added_nodes = vec![leaf_node("app.api")];
        let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("added node app.api"), "{lines:?}");
    }

    #[test]
    fn test_operations_for_nodes_removed_node_in_set() {
        let mut c = empty_change("c");
        c.delta.removed_nodes = vec!["app.api".to_owned()];
        let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("removed node app.api"), "{lines:?}");
    }

    #[test]
    fn test_operations_for_nodes_renamed_node_matches_from_or_to() {
        let mk = || {
            let mut c = empty_change("c");
            c.delta.renamed_nodes = vec![Rename {
                from: "app.api".to_owned(),
                to: "app.http".to_owned(),
            }];
            c
        };
        // Matches via "from" side.
        let by_from = operations_for_nodes(&[mk()], &nodes_set(&["app.api"]));
        assert!(!by_from.is_empty(), "must match via from: {by_from:?}");
        // Matches via "to" side.
        let by_to = operations_for_nodes(&[mk()], &nodes_set(&["app.http"]));
        assert!(!by_to.is_empty(), "must match via to: {by_to:?}");
    }

    #[test]
    fn test_operations_for_nodes_edge_endpoint_in_set() {
        let mut c = empty_change("c");
        c.delta.added_edges = vec![edge("app.api", "app.db")];
        let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
        assert!(
            !lines.is_empty(),
            "must match edge from endpoint: {lines:?}"
        );
    }

    #[test]
    fn test_operations_for_nodes_renamed_edge_is_included() {
        // renamed_edges must appear in operations_for_nodes when an endpoint
        // is in the queried node set. This is the RED test: the current
        // implementation chains added/modified/removed edges but silently
        // skips renamed_edges entirely.
        let mut c = empty_change("c");
        c.delta.renamed_edges = vec![EdgeRename {
            from: edge("app.api", "app.db"),
            to: edge("app.http", "app.db"),
        }];
        let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
        assert!(
            !lines.is_empty(),
            "renamed edge affecting app.api must appear in operations_for_nodes output"
        );
    }

    #[test]
    fn test_operations_for_nodes_no_match_returns_empty() {
        let mut c = empty_change("c");
        c.delta.added_nodes = vec![leaf_node("other.node")];
        let lines = operations_for_nodes(&[c], &nodes_set(&["app.api"]));
        assert!(lines.is_empty());
    }

    #[test]
    fn test_parse_blueprint_delta_added_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let delta = parse_blueprint_delta(
            "change.delta",
            r#"## ADDED Nodes
System App "desc" id "app" {
    Module Api "api" id "app.api" {}
}

## ADDED Edges
app.api -> app "reports"
"#,
        )?;

        let rendered = apply_blueprint_delta("", &delta)?;

        assert!(rendered.contains("System App"));
        assert!(rendered.contains("app.api -> app \"reports\""));
        Ok(())
    }

    #[test]
    fn test_parse_blueprint_delta_modified_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let base = r#"System App "desc" id "app" {
    Module Api "old" id "app.api" {}
}
"#;
        let delta = parse_blueprint_delta(
            "change.delta",
            r#"## MODIFIED Nodes
Module Api "new" id "app.api" {}
"#,
        )?;

        let rendered = apply_blueprint_delta(base, &delta)?;

        assert!(rendered.contains("Module Api \"new\" id \"app.api\""));
        Ok(())
    }

    #[test]
    fn test_parse_blueprint_delta_removed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let base = r#"System App "desc" id "app" {
    Module Api "old" id "app.api" {}
}
app.api -> app "reports"
"#;
        let delta = parse_blueprint_delta(
            "change.delta",
            r#"## REMOVED Nodes
- app.api

## REMOVED Edges
app.api -> app "reports"
"#,
        )?;

        let rendered = apply_blueprint_delta(base, &delta)?;

        assert!(!rendered.contains("app.api"));
        Ok(())
    }

    #[test]
    fn test_parse_blueprint_delta_renamed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let base = r#"System App "desc" id "app" {
    Module Api "old" id "app.api" {}
}
app.api -> app "reports"
"#;
        let delta = parse_blueprint_delta(
            "change.delta",
            r"## RENAMED Nodes
- app.api -> app.http
",
        )?;

        let rendered = apply_blueprint_delta(base, &delta)?;

        assert!(rendered.contains("id \"app.http\""));
        assert!(rendered.contains("app.http -> app \"reports\""));
        Ok(())
    }

    #[test]
    fn test_validate_change_detects_conflicting_operations()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("conflict")?;
        write_project(&root)?;
        let graph = scanner::load_project(&root, &root.join("cairn.blueprint"))?.graph;
        let change = Change {
            id: "phase-7.5a-test-fortification".to_owned(),
            path: root.join("meta/changes/phase-7.5a-test-fortification"),
            title: "test".to_owned(),
            proposal: String::new(),
            design: None,
            delta: BlueprintDelta {
                modified_nodes: vec![Node {
                    kind: NodeKind::Module,
                    name: "Api".to_owned(),
                    description: "desc".to_owned(),
                    id: "app.api".to_owned(),
                    tags: Vec::new(),
                    paths: Vec::new(),
                    owns_files: false,
                    contracts: Vec::new(),
                    raw_fields: Vec::new(),
                    children: Vec::new(),
                    span: crate::blueprint::Span::point("cairn.blueprint", 1, 1),
                }],
                removed_nodes: vec!["app.api".to_owned()],
                ..BlueprintDelta::default()
            },
            artefacts: Vec::new(),
            findings: Vec::new(),
        };

        let errors = validate_change(&change, &graph);

        assert!(
            errors
                .iter()
                .any(|error| error.contains("conflicting operations"))
        );
        Ok(())
    }

    fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(root.join("meta/changes"))?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Module Api "desc" id "app.api" {}
}
"#,
        )?;
        fs::write(
            root.join("cairn.config.yaml"),
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"\"\nrules: {}\n",
        )?;
        Ok(())
    }

    fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("cairn-changes-tests-{name}-{suffix}"));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
