// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use std::fmt::Write as _;

use super::apply::{atomic_write, replace_exact_id};
use super::*;

pub(super) fn read_to_string(path: &Path, findings: &mut Vec<String>) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        findings.push(format!("failed to read `{}`: {error}", path.display()));
        String::new()
    })
}

pub(super) fn proposal_title(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.strip_prefix("# Proposal:")
            .or_else(|| line.strip_prefix("# "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn copy_referencing_artefacts(
    root: &Path,
    change_path: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<(), String> {
    let meta = root.join("meta");
    if !meta.exists() {
        return Ok(());
    }
    copy_referencing_artefacts_from(root, change_path, &meta, old_id, new_id)
}

fn copy_referencing_artefacts_from(
    root: &Path,
    change_path: &Path,
    dir: &Path,
    old_id: &str,
    new_id: &str,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .strip_prefix(root)
                .is_ok_and(|relative| relative.starts_with("meta/changes"))
            {
                continue;
            }
            copy_referencing_artefacts_from(root, change_path, &path, old_id, new_id)?;
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "md") {
            continue;
        }
        let content = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        if !frontmatter_references(&content, old_id) {
            continue;
        }
        let relative = path
            .strip_prefix(root.join("meta"))
            .map_err(|error| error.to_string())?;
        let target = change_path.join(relative);
        let updated = update_frontmatter_reference(&content, old_id, new_id);
        let updated = insert_operation(&updated, "modified", None);
        atomic_write(&target, &updated)?;
    }
    Ok(())
}

pub(super) fn frontmatter_references(source: &str, id: &str) -> bool {
    let parsed = frontmatter::parse(source);
    parsed.values.values().any(|value| value == id)
        || parsed
            .lists
            .values()
            .any(|values| values.iter().any(|value| value == id))
}

pub(super) fn update_frontmatter_reference(source: &str, old_id: &str, new_id: &str) -> String {
    let mut in_frontmatter = false;
    let mut seen_start = false;
    let mut output = Vec::new();
    for line in source.lines() {
        if !seen_start && line.trim() == "---" {
            seen_start = true;
            in_frontmatter = true;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            output.push(line.to_owned());
            continue;
        }
        if in_frontmatter {
            output.push(line.replace(old_id, new_id));
        } else {
            output.push(line.to_owned());
        }
    }
    format!("{}\n", output.join("\n"))
}

pub(super) fn insert_operation(
    source: &str,
    operation: &str,
    renamed_from: Option<&Path>,
) -> String {
    let mut output = Vec::new();
    let mut inserted = false;
    for line in source.lines() {
        output.push(line.to_owned());
        if !inserted && line.trim() == "---" {
            output.push(format!("operation: {operation}"));
            if let Some(path) = renamed_from {
                output.push(format!("renamed_from: {}", path.display()));
            }
            inserted = true;
        }
    }
    format!("{}\n", output.join("\n"))
}

pub(super) fn artefact_content_refs(source: &str, ids: &BTreeSet<String>) -> bool {
    let parsed = frontmatter::parse(source);
    parsed.values.values().any(|value| ids.contains(value))
        || parsed
            .lists
            .values()
            .any(|values| values.iter().any(|value| ids.contains(value)))
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
