// cairn:allow-large-module reason: single dispatch table for change-directory mutating tools; each branch is a thin adapter to a changes or brownfield submodule, so splitting would fragment the CLI-to-domain mapping.
//! Dispatch for change-directory mutating query tools.
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
        "init_from_code" => {
            let force = request.has(QueryFlag::Force);
            Some(
                crate::brownfield::init::run_init_from_code(root, force)
                    .map(|change_id| {
                        json!({
                            "command": "init_from_code",
                            "status": "ok",
                            "data": { "change_id": change_id },
                        })
                    })
                    .map_err(|e| QueryError {
                        code: "CAIRN_COMMAND_FAILED".to_owned(),
                        message: e.to_string(),
                        source_span: None,
                        remediation: None,
                    }),
            )
        }
        "refine" => Some(
            crate::brownfield::refine::run_refine(root)
                .map(|change_id| {
                    json!({
                        "command": "refine",
                        "status": "ok",
                        "data": { "change_id": change_id },
                    })
                })
                .map_err(|e| QueryError {
                    code: "CAIRN_COMMAND_FAILED".to_owned(),
                    message: e.to_string(),
                    source_span: None,
                    remediation: None,
                }),
        ),
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
        "drafts" => Some(list_drafts(root)),
        "draft_show" => Some(show_draft(root, request.node.as_ref())),
        "draft_discard" => Some(discard_draft(root, request.node.as_ref())),
        "draft_edit" => Some(edit_draft(root, request.node.as_ref())),
        "draft_accept" => Some(accept_draft(root, blueprint_path, request)),
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

fn list_drafts(root: &Path) -> Result<Value, QueryError> {
    let store = crate::summariser::DraftStore::new(root.join(".cairn/state/summariser"));
    let drafts = store.list().map_err(|e| QueryError {
        code: "CAIRN_DRAFTS_LIST_FAILED".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    Ok(json!({
        "drafts": drafts.iter().map(draft_summary_json).collect::<Vec<_>>(),
    }))
}

fn show_draft(root: &Path, draft_id: Option<&String>) -> Result<Value, QueryError> {
    let id = required(draft_id, "node")?;
    let store = crate::summariser::DraftStore::new(root.join(".cairn/state/summariser"));
    let draft = store.read(id).map_err(|e| QueryError {
        code: "CAIRN_DRAFT_NOT_FOUND".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    Ok(draft_detail_json(&draft))
}

fn discard_draft(root: &Path, draft_id: Option<&String>) -> Result<Value, QueryError> {
    let id = required(draft_id, "node")?;
    let store = crate::summariser::DraftStore::new(root.join(".cairn/state/summariser"));
    let draft = store.read(id).map_err(|e| QueryError {
        code: "CAIRN_DRAFT_NOT_FOUND".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    crate::summariser::validate_transition(
        draft.status(),
        crate::summariser::DraftStatus::Discarded,
    )
    .map_err(|e| QueryError {
        code: "CAIRN_DRAFT_INVALID_TRANSITION".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    let mut header = match &draft {
        crate::summariser::Draft::Pending(d) => d.header.clone(),
        crate::summariser::Draft::Editable(d) => d.header.clone(),
        crate::summariser::Draft::Accepted(d) => d.header().clone(),
        crate::summariser::Draft::Discarded(d) => d.header.clone(),
    };
    header
        .transitions
        .push(crate::summariser::TransitionRecord {
            from: draft.status(),
            to: crate::summariser::DraftStatus::Discarded,
            at: now_rfc3339(),
        });
    let discarded = crate::summariser::Draft::Discarded(crate::summariser::DiscardedDraft {
        header,
        reason: None,
    });
    store
        .overwrite(&discarded)
        .map_err(|e| command_error(e.to_string()))?;
    Ok(json!({
        "id": id,
        "status": "discarded",
    }))
}

fn edit_draft(root: &Path, draft_id: Option<&String>) -> Result<Value, QueryError> {
    let id = required(draft_id, "node")?;
    let store = crate::summariser::DraftStore::new(root.join(".cairn/state/summariser"));
    let draft = store.read(id).map_err(|e| QueryError {
        code: "CAIRN_DRAFT_NOT_FOUND".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    crate::summariser::validate_transition(
        draft.status(),
        crate::summariser::DraftStatus::Editable,
    )
    .map_err(|e| QueryError {
        code: "CAIRN_DRAFT_INVALID_TRANSITION".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    let editable_path = store
        .write_editable(&draft)
        .map_err(|e| command_error(e.to_string()))?;
    let mut header = match &draft {
        crate::summariser::Draft::Pending(d) => d.header.clone(),
        crate::summariser::Draft::Editable(d) => d.header.clone(),
        crate::summariser::Draft::Accepted(d) => d.header().clone(),
        crate::summariser::Draft::Discarded(d) => d.header.clone(),
    };
    header
        .transitions
        .push(crate::summariser::TransitionRecord {
            from: draft.status(),
            to: crate::summariser::DraftStatus::Editable,
            at: now_rfc3339(),
        });
    let editable = crate::summariser::Draft::Editable(crate::summariser::EditableDraft {
        header,
        editable_path: editable_path.to_string_lossy().to_string(),
    });
    store
        .overwrite(&editable)
        .map_err(|e| command_error(e.to_string()))?;
    Ok(json!({
        "id": id,
        "status": "editable",
        "editable_path": editable_path,
    }))
}

fn accept_draft(
    root: &Path,
    blueprint_path: &Path,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let id = required(request.node.as_ref(), "node")?;
    let edited = request.has(QueryFlag::Edited);
    crate::summariser::accept(root, id, blueprint_path, edited).map_err(|e| QueryError {
        code: "CAIRN_DRAFT_ACCEPT_FAILED".to_owned(),
        message: e.to_string(),
        source_span: None,
        remediation: None,
    })?;
    Ok(json!({
        "id": id,
        "status": "accepted",
    }))
}

fn now_rfc3339() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time before epoch");
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        "2024-01-15",
        (now.as_secs() / 3600) % 24,
        (now.as_secs() / 60) % 60,
        now.as_secs() % 60
    )
}

fn draft_summary_json(draft: &crate::summariser::Draft) -> Value {
    let (status, node_id, created_at) = match draft {
        crate::summariser::Draft::Pending(d) => (
            "pending",
            d.header.node_id.clone(),
            d.header.created_at.clone(),
        ),
        crate::summariser::Draft::Editable(d) => (
            "editable",
            d.header.node_id.clone(),
            d.header.created_at.clone(),
        ),
        crate::summariser::Draft::Accepted(d) => (
            "accepted",
            d.header().node_id.clone(),
            d.header().created_at.clone(),
        ),
        crate::summariser::Draft::Discarded(d) => (
            "discarded",
            d.header.node_id.clone(),
            d.header.created_at.clone(),
        ),
    };
    json!({
        "id": draft.id(),
        "status": status,
        "node_id": node_id,
        "created_at": created_at,
    })
}

fn draft_detail_json(draft: &crate::summariser::Draft) -> Value {
    match draft {
        crate::summariser::Draft::Pending(d) => json!({
            "id": d.header.id,
            "status": "pending",
            "node_id": d.header.node_id,
            "artefact_type": d.header.artefact_type,
            "draft_text": d.header.draft_text,
            "created_at": d.header.created_at,
        }),
        crate::summariser::Draft::Editable(d) => json!({
            "id": d.header.id,
            "status": "editable",
            "node_id": d.header.node_id,
            "artefact_type": d.header.artefact_type,
            "draft_text": d.header.draft_text,
            "created_at": d.header.created_at,
            "editable_path": d.editable_path,
        }),
        crate::summariser::Draft::Accepted(d) => json!({
            "id": d.header().id,
            "status": "accepted",
            "node_id": d.header().node_id,
            "artefact_type": d.header().artefact_type,
            "draft_text": d.header().draft_text,
            "created_at": d.header().created_at,
            "accepted_interface_hash": d.accepted_interface_hash(),
        }),
        crate::summariser::Draft::Discarded(d) => json!({
            "id": d.header.id,
            "status": "discarded",
            "node_id": d.header.node_id,
            "artefact_type": d.header.artefact_type,
            "draft_text": d.header.draft_text,
            "created_at": d.header.created_at,
            "reason": d.reason,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summariser::{Draft, DraftHeader, DraftStore, PendingDraft};

    fn temp_root_with_drafts(drafts: &[Draft]) -> std::path::PathBuf {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        for draft in drafts {
            store.write(draft).unwrap();
        }
        std::mem::forget(dir);
        root
    }

    fn sample_pending_draft(id: &str, node_id: &str, text: &str) -> Draft {
        Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: id.to_owned(),
                node_id: node_id.to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: text.to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        })
    }

    fn sample_discarded_draft(id: &str, node_id: &str, text: &str) -> Draft {
        use crate::summariser::DiscardedDraft;
        Draft::Discarded(DiscardedDraft {
            header: DraftHeader {
                id: id.to_owned(),
                node_id: node_id.to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: text.to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
            reason: None,
        })
    }

    #[test]
    fn test_list_drafts_empty_store() {
        let dir = tempfile::tempdir().unwrap();
        let result = list_drafts(dir.path()).unwrap();
        let drafts = result.get("drafts").unwrap().as_array().unwrap();
        assert!(drafts.is_empty());
    }

    #[test]
    fn test_list_drafts_returns_summaries() {
        let root = temp_root_with_drafts(&[
            sample_pending_draft("draft-001", "app.auth", "auth contract"),
            sample_pending_draft("draft-002", "app.core", "core contract"),
        ]);
        let result = list_drafts(&root).unwrap();
        let drafts = result.get("drafts").unwrap().as_array().unwrap();
        assert_eq!(drafts.len(), 2);
        assert_eq!(drafts[0].get("id").unwrap().as_str().unwrap(), "draft-001");
        assert_eq!(
            drafts[0].get("status").unwrap().as_str().unwrap(),
            "pending"
        );
        assert_eq!(
            drafts[0].get("node_id").unwrap().as_str().unwrap(),
            "app.auth"
        );
        assert_eq!(drafts[1].get("id").unwrap().as_str().unwrap(), "draft-002");
    }

    #[test]
    fn test_show_draft_returns_full_draft() {
        let root =
            temp_root_with_drafts(&[sample_pending_draft("draft-003", "app.ui", "ui contract")]);
        let result = show_draft(&root, Some(&"draft-003".to_owned())).unwrap();
        assert_eq!(result.get("id").unwrap().as_str().unwrap(), "draft-003");
        assert_eq!(result.get("status").unwrap().as_str().unwrap(), "pending");
        assert_eq!(
            result.get("draft_text").unwrap().as_str().unwrap(),
            "ui contract"
        );
        assert_eq!(result.get("node_id").unwrap().as_str().unwrap(), "app.ui");
    }

    #[test]
    fn test_show_draft_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let result = show_draft(dir.path(), None);
        assert!(result.is_err(), "expected error for missing draft id");
    }

    #[test]
    fn test_show_draft_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        let result = show_draft(dir.path(), Some(&"missing".to_owned()));
        assert!(result.is_err(), "expected error for missing draft");
    }

    #[test]
    fn test_discard_draft_transitions_to_discarded() {
        let root =
            temp_root_with_drafts(&[sample_pending_draft("draft-005", "app.api", "api contract")]);
        let result = discard_draft(&root, Some(&"draft-005".to_owned())).unwrap();
        assert_eq!(result.get("id").unwrap().as_str().unwrap(), "draft-005");
        assert_eq!(result.get("status").unwrap().as_str().unwrap(), "discarded");

        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let draft = store.read("draft-005").unwrap();
        assert!(matches!(draft, Draft::Discarded(_)));
        assert_eq!(draft.id(), "draft-005");
    }

    #[test]
    fn test_discard_draft_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let result = discard_draft(dir.path(), None);
        assert!(result.is_err(), "expected error for missing draft id");
    }

    #[test]
    fn test_discard_draft_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        let result = discard_draft(dir.path(), Some(&"missing".to_owned()));
        assert!(result.is_err(), "expected error for missing draft");
    }

    #[test]
    fn test_edit_draft_writes_editable_file() {
        let root =
            temp_root_with_drafts(&[sample_pending_draft("draft-006", "app.db", "db contract")]);
        let result = edit_draft(&root, Some(&"draft-006".to_owned())).unwrap();
        assert_eq!(result.get("id").unwrap().as_str().unwrap(), "draft-006");
        assert_eq!(result.get("status").unwrap().as_str().unwrap(), "editable");
        let path = result.get("editable_path").unwrap().as_str().unwrap();
        assert!(path.ends_with("draft-006.md"));
        assert!(std::path::Path::new(path).exists());

        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let draft = store.read("draft-006").unwrap();
        assert!(matches!(draft, Draft::Editable(_)));
        assert_eq!(draft.id(), "draft-006");
    }

    #[test]
    fn test_edit_draft_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let result = edit_draft(dir.path(), None);
        assert!(result.is_err(), "expected error for missing draft id");
    }

    #[test]
    fn test_edit_draft_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        let result = edit_draft(dir.path(), Some(&"missing".to_owned()));
        assert!(result.is_err(), "expected error for missing draft");
    }

    #[test]
    fn test_accept_draft_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let req = QueryRequest::default();
        let result = accept_draft(dir.path(), dir.path(), &req);
        assert!(result.is_err(), "expected error for missing draft id");
    }

    #[test]
    fn test_accept_draft_not_found_errors() {
        let dir = tempfile::tempdir().unwrap();
        let req = QueryRequest {
            node: Some("missing".to_owned()),
            ..QueryRequest::default()
        };
        let result = accept_draft(dir.path(), dir.path(), &req);
        assert!(result.is_err(), "expected error for missing draft");
    }

    #[test]
    fn test_discard_already_discarded_draft_is_rejected() {
        let root = temp_root_with_drafts(&[sample_discarded_draft(
            "draft-007",
            "app.api",
            "api contract",
        )]);
        let result = discard_draft(&root, Some(&"draft-007".to_owned()));
        assert!(
            result.is_err(),
            "expected error for discarding already-discarded draft"
        );
        let err = result.unwrap_err();
        assert!(err.message.contains("invalid transition"));
    }

    #[test]
    fn test_discard_already_accepted_draft_is_rejected() {
        use crate::summariser::AcceptedDraft;
        let header = DraftHeader {
            id: "draft-008".to_owned(),
            node_id: "app.api".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "api contract".to_owned(),
            created_at: "2024-01-15T10:30:00Z".to_owned(),
            transitions: Vec::new(),
            metadata: None,
        };
        let accepted = Draft::Accepted(AcceptedDraft::new(header, "hash123".to_owned()).unwrap());
        let root = temp_root_with_drafts(&[accepted]);
        let result = discard_draft(&root, Some(&"draft-008".to_owned()));
        assert!(
            result.is_err(),
            "expected error for discarding already-accepted draft"
        );
        let err = result.unwrap_err();
        assert!(err.message.contains("invalid transition"));
    }

    #[test]
    fn test_discard_records_transition() {
        let root =
            temp_root_with_drafts(&[sample_pending_draft("draft-009", "app.api", "api contract")]);
        discard_draft(&root, Some(&"draft-009".to_owned())).unwrap();

        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let draft = store.read("draft-009").unwrap();
        match draft {
            Draft::Discarded(d) => {
                assert_eq!(d.header.transitions.len(), 1);
                assert_eq!(
                    d.header.transitions[0].from,
                    crate::summariser::DraftStatus::Pending
                );
                assert_eq!(
                    d.header.transitions[0].to,
                    crate::summariser::DraftStatus::Discarded
                );
            }
            other => panic!("expected Discarded draft, got {other:?}"),
        }
    }

    #[test]
    fn test_edit_accepted_draft_is_rejected() {
        use crate::summariser::AcceptedDraft;
        let header = DraftHeader {
            id: "draft-010".to_owned(),
            node_id: "app.api".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "api contract".to_owned(),
            created_at: "2024-01-15T10:30:00Z".to_owned(),
            transitions: Vec::new(),
            metadata: None,
        };
        let accepted = Draft::Accepted(AcceptedDraft::new(header, "hash456".to_owned()).unwrap());
        let root = temp_root_with_drafts(&[accepted]);
        let result = edit_draft(&root, Some(&"draft-010".to_owned()));
        assert!(
            result.is_err(),
            "expected error for editing already-accepted draft"
        );
        let err = result.unwrap_err();
        assert!(err.message.contains("invalid transition"));
    }

    #[test]
    fn test_edit_discarded_draft_is_rejected() {
        let root = temp_root_with_drafts(&[sample_discarded_draft(
            "draft-011",
            "app.api",
            "api contract",
        )]);
        let result = edit_draft(&root, Some(&"draft-011".to_owned()));
        assert!(
            result.is_err(),
            "expected error for editing discarded draft"
        );
        let err = result.unwrap_err();
        assert!(err.message.contains("invalid transition"));
    }

    #[test]
    fn test_edit_records_transition() {
        let root =
            temp_root_with_drafts(&[sample_pending_draft("draft-012", "app.api", "api contract")]);
        edit_draft(&root, Some(&"draft-012".to_owned())).unwrap();

        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let draft = store.read("draft-012").unwrap();
        match draft {
            Draft::Editable(d) => {
                assert_eq!(d.header.transitions.len(), 1);
                assert_eq!(
                    d.header.transitions[0].from,
                    crate::summariser::DraftStatus::Pending
                );
                assert_eq!(
                    d.header.transitions[0].to,
                    crate::summariser::DraftStatus::Editable
                );
            }
            other => panic!("expected Editable draft, got {other:?}"),
        }
    }
}
