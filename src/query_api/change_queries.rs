//! Dispatchers for change-related query tools (archive, validate, apply, rename).
// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use util::command_error;

pub(super) fn dispatch_change_tool(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    metadata: ToolMetadata,
) -> Option<Result<Value, QueryError>> {
    match metadata.cli_name {
        "archive" => {
            let change = match required(request.change.as_ref(), "change") {
                Ok(change) => change,
                Err(error) => return Some(Err(error)),
            };
            let conflict_findings = hooks::detect_active_change_conflicts(changes_dir);
            if !conflict_findings.is_empty() {
                return Some(Err(findings_error(&conflict_findings)));
            }
            Some(
                changes::archive(root, blueprint_path, change)
                    .map(|report| {
                        json!({
                            "archive_path": report.archive_path.to_string_lossy(),
                            "summary": report.summary,
                        })
                    })
                    .map_err(command_error),
            )
        }
        "rename" => {
            let old_id = match required(request.old_id.as_ref(), "old_id") {
                Ok(value) => value,
                Err(error) => return Some(Err(error)),
            };
            let new_id = match required(request.new_id.as_ref(), "new_id") {
                Ok(value) => value,
                Err(error) => return Some(Err(error)),
            };
            Some(
                changes::create_rename_change(root, blueprint_path, old_id, new_id)
                    .map(|change| change_json(&change))
                    .map_err(command_error),
            )
        }
        "changes" => Some(discover_changes(root)),
        "show" => Some(show_change(root, request.change.as_ref())),
        _ => None,
    }
}

pub(super) fn discover_changes(root: &Path) -> Result<Value, QueryError> {
    let changes = changes::discover(root).map_err(|error| QueryError {
        code: "CAIRN_CHANGES_DISCOVERY_FAILED".to_owned(),
        message: error.to_string(),
        source_span: Some(root.join("meta/changes").display().to_string()),
        remediation: None,
    })?;
    Ok(json!({ "changes": changes.iter().map(change_json).collect::<Vec<_>>() }))
}

pub(super) fn show_change(root: &Path, change: Option<&String>) -> Result<Value, QueryError> {
    let change_id = required(change, "change")?;
    let changes = changes::discover(root).map_err(|error| QueryError {
        code: "CAIRN_CHANGES_DISCOVERY_FAILED".to_owned(),
        message: error.to_string(),
        source_span: Some(root.join("meta/changes").display().to_string()),
        remediation: None,
    })?;
    changes
        .iter()
        .find(|candidate| candidate.id == change_id)
        .map(change_json)
        .ok_or_else(|| QueryError {
            code: "CAIRN_CHANGE_NOT_FOUND".to_owned(),
            message: format!("change `{change_id}` was not found"),
            source_span: Some(root.join("meta/changes").display().to_string()),
            remediation: None,
        })
}

pub(super) fn change_json(change: &changes::Change) -> Value {
    json!({
        "id": change.id,
        "path": change.path.to_string_lossy(),
        "title": change.title,
        "proposal": change.proposal,
        "design": change.design,
        "summary": changes::operation_summary(change),
        "findings": change.findings,
        "delta": {
            "added_nodes": change.delta.added_nodes.len(),
            "modified_nodes": change.delta.modified_nodes.len(),
            "removed_nodes": change.delta.removed_nodes,
            "renamed_nodes": change.delta.renamed_nodes.iter().map(|rename| {
                json!({ "from": rename.from, "to": rename.to })
            }).collect::<Vec<_>>(),
            "added_edges": change.delta.added_edges.len(),
            "modified_edges": change.delta.modified_edges.len(),
            "removed_edges": change.delta.removed_edges.len(),
            "renamed_edges": change.delta.renamed_edges.len(),
        },
        "artefacts": change.artefacts.iter().map(|operation| {
            json!({
                "operation": format!("{:?}", operation.operation),
                "change_path": operation.change_path.to_string_lossy(),
                "target_path": operation.target_path.to_string_lossy(),
                "renamed_from": operation.renamed_from.as_ref().map(|path| path.to_string_lossy().to_string()),
            })
        }).collect::<Vec<_>>(),
    })
}
