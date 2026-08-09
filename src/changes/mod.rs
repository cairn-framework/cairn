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
    blueprint::{Edge, EdgeProvenance, Node, NodeKind, parser::parse_str},
    map::Graph,
    scanner,
};

mod apply;
mod artefact_ops;
mod delta;
mod rename;
mod types;
mod validate;

use crate::persist;
use apply::{
    append_archive_log, apply_archive, archive_path, mutation_paths, restore_snapshots,
    snapshot_paths,
};
use artefact_ops::parse_artefact_operations;
pub use delta::parse_blueprint_delta;
use rename::{artefact_content_refs, copy_referencing_artefacts, proposal_title, read_to_string};
pub(crate) use types::*;
pub use validate::validate_change;

/// Writes one canonical edge declaration, including its optional provenance.
pub(crate) fn write_edge_line(
    out: &mut impl std::fmt::Write,
    from: &str,
    to: &str,
    description: &str,
    provenance: EdgeProvenance,
) -> std::fmt::Result {
    if let Some(marker) = provenance.marker() {
        writeln!(out, "{from} -> {to} {description:?} @{marker}")
    } else {
        writeln!(out, "{from} -> {to} {description:?}")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EdgeIdentity<'a> {
    from: &'a str,
    to: &'a str,
    description: &'a str,
    provenance: EdgeProvenance,
}

pub(crate) fn edge_identity<'a>(
    from: &'a str,
    to: &'a str,
    description: &'a str,
    provenance: EdgeProvenance,
) -> EdgeIdentity<'a> {
    EdgeIdentity {
        from,
        to,
        description,
        provenance,
    }
}

pub(crate) fn edge_identity_for_edge(edge: &Edge) -> EdgeIdentity<'_> {
    edge_identity(&edge.from, &edge.to, &edge.description, edge.provenance)
}

/// Compares full edge identity for remove and rename operations.
pub(crate) fn same_edge(left: &Edge, right: &Edge) -> bool {
    same_edge_fields(edge_identity_for_edge(left), edge_identity_for_edge(right))
}

pub(crate) fn same_edge_fields(left: EdgeIdentity<'_>, right: EdgeIdentity<'_>) -> bool {
    left == right
}

/// Compares the stable identity used by an edge modification.
///
/// A modification may change the description, but endpoints and provenance
/// must match. Marker changes therefore require an explicit delta edge.
pub(crate) fn same_edge_for_modify(left: &Edge, right: &Edge) -> bool {
    same_edge_fields_for_modify(edge_identity_for_edge(left), edge_identity_for_edge(right))
}

pub(crate) fn same_edge_fields_for_modify(left: EdgeIdentity<'_>, right: EdgeIdentity<'_>) -> bool {
    left.from == right.from && left.to == right.to && left.provenance == right.provenance
}

pub(crate) fn replace_exact_id(value: &str, old_id: &str, new_id: &str) -> String {
    if value == old_id {
        new_id.to_owned()
    } else {
        value.to_owned()
    }
}

pub(crate) fn id_after_node_renames(mut id: String, renames: &[Rename]) -> String {
    for rename in renames {
        id = replace_exact_id(&id, &rename.from, &rename.to);
    }
    id
}

/// Rewrites edge endpoints through the node rename map used by a delta.
pub(crate) fn edge_after_node_renames(mut edge: Edge, renames: &[Rename]) -> Edge {
    edge.from = id_after_node_renames(edge.from, renames);
    edge.to = id_after_node_renames(edge.to, renames);
    edge
}

fn write_edge_rename_line(
    out: &mut impl std::fmt::Write,
    edge: &crate::map::graph::EdgeRef,
    new_from: &str,
    new_to: &str,
) -> std::fmt::Result {
    let mut old_edge = String::new();
    let _ = write_edge_line(
        &mut old_edge,
        &edge.from,
        &edge.to,
        &edge.description,
        edge.provenance,
    );
    let mut new_edge = String::new();
    let _ = write_edge_line(
        &mut new_edge,
        new_from,
        new_to,
        &edge.description,
        edge.provenance,
    );
    writeln!(out, "{} => {}", old_edge.trim_end(), new_edge.trim_end())
}

/// Discovers active changes under `changes_dir` (the resolved
/// `--changes-dir` path, `meta/changes` by default).
///
/// # Errors
///
/// Returns an I/O error if `changes_dir` exists but cannot be read.
pub fn discover(root: &Path, changes_dir: &Path) -> io::Result<Vec<Change>> {
    if !changes_dir.exists() {
        return Ok(Vec::new());
    }
    let mut changes = Vec::new();
    for entry in fs::read_dir(changes_dir)? {
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
    let progress = load_task_progress(&path, &mut findings);
    Change {
        id,
        path,
        title,
        proposal,
        design,
        delta,
        artefacts,
        progress,
        findings,
    }
}

/// Reads `tasks.md` checkbox progress for a change directory.
///
/// A missing file simply means the change tracks no tasks. Any other read
/// failure is recorded as a finding, like the other change files, rather than
/// silently reported as zero progress.
fn load_task_progress(path: &Path, findings: &mut Vec<String>) -> crate::artefacts::TaskProgress {
    let tasks_path = path.join("tasks.md");
    match fs::read_to_string(&tasks_path) {
        Ok(source) => crate::artefacts::count_tasks(&source),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            crate::artefacts::TaskProgress::default()
        }
        Err(error) => {
            findings.push(format!("failed to read {}: {error}", tasks_path.display()));
            crate::artefacts::TaskProgress::default()
        }
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
    changes_dir: &Path,
    change_id: &str,
) -> Result<ArchiveReport, String> {
    let changes = discover(root, changes_dir).map_err(|error| error.to_string())?;
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
    if let Err(error) = apply_archive(blueprint_path, &change).and_then(|()| {
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
    let archive_path = archive_path(changes_dir, &change.id);
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
    persist::atomic_write(
        &change_path.join("proposal.md"),
        &format!("# Proposal: Rename {old_id} to {new_id}\n\nRename `{old_id}` to `{new_id}`.\n"),
    )
    .map_err(|error| error.to_string())?;
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
            let _ = write_edge_rename_line(&mut delta, &edge, &new_from, &new_to);
        }
    }
    persist::atomic_write(&change_path.join("blueprint.delta"), &delta)
        .map_err(|error| error.to_string())?;
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
mod edge_provenance_tests;
#[cfg(test)]
mod tests;
