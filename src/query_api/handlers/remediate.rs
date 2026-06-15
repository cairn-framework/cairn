//! Hook, health, and remediation query handlers.
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::*;
use super::graph::count_findings;

pub(crate) fn hook_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let kind = match request.status.as_deref().unwrap_or("all") {
        "structural" => HookKind::Structural,
        "interface" => HookKind::Interface,
        "tension" => HookKind::Tension,
        "architecture-decision" => HookKind::ArchitectureDecision,
        "all" => HookKind::All,
        other => {
            return Err(QueryError {
                code: "CAIRN_QUERY_INVALID_HOOK_KIND".to_owned(),
                message: format!("unknown hook kind `{other}`"),
                source_span: None,
                remediation: Some(
                    "Use structural, interface, tension, architecture-decision, or all.".to_owned(),
                ),
            });
        }
    };
    let report = crate::hooks::run(kind, root, changes_dir, scan_result);
    Ok(json!({
        "kind": hook_kind_name(report.kind),
        "decision": hook_decision_name(report.decision),
        "findings": findings_json(&report.findings),
        "exit_code": report.exit_code(),
    }))
}

pub(crate) fn health_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> Value {
    let lint_response = query::lint(&scan_result.graph);
    let (lint_errors, lint_warnings, lint_info) = count_findings(&lint_response.findings);
    let hook_report =
        crate::hooks::run(crate::hooks::HookKind::All, root, changes_dir, scan_result);
    let (hook_errors, hook_warnings, hook_info) = count_findings(&hook_report.findings);
    let total_errors = lint_errors + hook_errors;
    let total_warnings = lint_warnings + hook_warnings;
    let total_info = lint_info + hook_info;
    let clean = total_errors == 0 && hook_report.decision == crate::hooks::ExitDecision::Pass;
    let mut synced = 0usize;
    let mut ghost = 0usize;
    let mut orphaned = 0usize;
    for node in scan_result.graph.nodes.values() {
        match node.state {
            crate::map::NodeState::Synced => synced += 1,
            crate::map::NodeState::Ghost => ghost += 1,
            crate::map::NodeState::Orphaned => orphaned += 1,
        }
    }
    json!({
        "clean": clean,
        "summary": {
            "total_errors": total_errors,
            "total_warnings": total_warnings,
            "info": total_info,
            "modules": {
                "synced": synced,
                "ghost": ghost,
                "orphaned": orphaned,
            },
        },
        "lint": {
            "errors": lint_errors,
            "warnings": lint_warnings,
            "info": lint_info,
            "findings": findings_json(&lint_response.findings),
        },
        "hooks": {
            "decision": hook_decision_name(hook_report.decision),
            "errors": hook_errors,
            "warnings": hook_warnings,
            "info": hook_info,
            "findings": findings_json(&hook_report.findings),
        },
    })
}

// Reason: remediate action generation naturally spans many finding codes and
// action branches; extracting each branch would fragment the remediation logic.
#[allow(clippy::too_many_lines)]
pub(crate) fn remediate_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> Value {
    let lint_response = query::lint(&scan_result.graph);
    let hook_report =
        crate::hooks::run(crate::hooks::HookKind::All, root, changes_dir, scan_result);
    let mut actions: Vec<Value> = Vec::new();
    let mut has_orphans = false;
    let mut has_ghosts = false;
    let mut has_interface_changes = false;
    let mut has_missing_decisions = false;
    let mut has_parse_errors = false;
    let mut has_contract_issues = false;
    let mut has_decision_issues = false;
    let mut has_todo_issues = false;
    let mut has_source_issues = false;
    let mut has_research_issues = false;
    let mut has_order_issues = false;
    let mut has_gitignored_paths = false;
    let mut summarise_nodes: Vec<String> = Vec::new();
    let mut decision_nodes: Vec<String> = Vec::new();
    let mut gitignored_nodes: Vec<String> = Vec::new();
    for finding in &lint_response.findings {
        match finding.code.as_str() {
            "CAIRN_RECONCILE_ORPHANED_FILE" => has_orphans = true,
            "CAIRN_INTEGRITY_DUPLICATE_ID"
            | "CAIRN_INTEGRITY_INVALID_ID"
            | "CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT"
            | "CAIRN_INTEGRITY_PATH_TIE"
            | "CAIRN_PARSE_UNEXPECTED_TOKEN"
            | "CAIRN_PARSE_UNTERMINATED_STRING"
            | "CAIRN_IO_READ_BLUEPRINT"
            | "CAIRN_BLUEPRINT_LEGACY_EXTENSION" => has_parse_errors = true,
            "CAIRN_CONTRACT_MISSING"
            | "CAIRN_CONTRACT_MISSING_NODE"
            | "CAIRN_CONTRACT_UNKNOWN_NODE"
            | "CAIRN_CONTRACT_WRONG_NODE"
            | "CAIRN_CONTRACT_READ_FAILED" => {
                has_orphans = true;
                has_contract_issues = true;
                if let Some(node) = &finding.node {
                    summarise_nodes.push(node.clone());
                }
            }
            "CAIRN_DECISION_ORPHANED"
            | "CAIRN_DECISION_MISSING_NODES"
            | "CAIRN_DECISION_REFERENCE_UNKNOWN"
            | "CAIRN_DECISION_STATUS_INVALID"
            | "CAIRN_DECISION_SUPERSEDES_STATUS"
            | "CAIRN_DECISION_UNKNOWN_PROVENANCE" => {
                has_decision_issues = true;
            }
            "CAIRN_TODO_ORPHAN_NODE" | "CAIRN_TODO_STATUS_INVALID" => {
                has_todo_issues = true;
            }
            "CAIRN_SOURCE_ORPHAN"
            | "CAIRN_SOURCE_UNVERIFIED"
            | "CAIRN_SOURCE_READ_FAILED"
            | "CAIRN_SOURCE_VERIFICATION_INVALID"
            | "CAIRN_SOURCE_INDEX_GAP" => {
                has_source_issues = true;
            }
            "CAIRN_RESEARCH_MISSING_SOURCES" | "CAIRN_RESEARCH_UNKNOWN_SOURCE" => {
                has_research_issues = true;
            }
            "CAIRN_ORDER_CYCLE" => {
                has_order_issues = true;
            }
            "CAIRN_PATH_GITIGNORED" => {
                has_gitignored_paths = true;
                if let Some(node) = &finding.node
                    && !gitignored_nodes.contains(node)
                {
                    gitignored_nodes.push(node.clone());
                }
            }
            "CAIRN_RECONCILE_RUST_LANGUAGE"
            | "CAIRN_RECONCILE_GO_LANGUAGE"
            | "CAIRN_RECONCILE_PYTHON_LANGUAGE"
            | "CAIRN_RECONCILE_TS_LANGUAGE"
            | "CAIRN_RECONCILE_PARSE_RUST"
            | "CAIRN_RECONCILE_PARSE_GO"
            | "CAIRN_RECONCILE_PARSE_PYTHON"
            | "CAIRN_RECONCILE_PARSE_TS"
            | "CAIRN_RECONCILE_READ_DIR"
            | "CAIRN_RECONCILE_READ_DIR_ENTRY"
            | "CAIRN_RECONCILE_READ_SOURCE" => {
                has_orphans = true;
            }
            _ => {}
        }
    }
    for finding in &hook_report.findings {
        match finding.code.as_str() {
            "CAIRN_INTERFACE_HASH_CHANGED" => {
                has_interface_changes = true;
                for node_id in scan_result.target_hashes.keys() {
                    if !summarise_nodes.contains(node_id) {
                        summarise_nodes.push(node_id.clone());
                    }
                }
            }
            "CAIRN_BLUEPRINT_CHANGE_NO_DECISION" | "CAIRN_PROVENANCE_NO_DECISION" => {
                has_missing_decisions = true;
                if let Some(node) = &finding.node
                    && !decision_nodes.contains(node)
                {
                    decision_nodes.push(node.clone());
                }
            }
            _ => {}
        }
    }
    for node in scan_result.graph.nodes.values() {
        match node.state {
            crate::map::NodeState::Ghost => has_ghosts = true,
            crate::map::NodeState::Orphaned => has_orphans = true,
            crate::map::NodeState::Synced => {}
        }
    }
    if has_parse_errors {
        actions.push(json!({
            "priority": 1,
            "action": "fix_blueprint",
            "command": "cairn lint",
            "description": "The blueprint has parse errors or integrity issues that must be fixed manually before other actions can succeed.",
            "nodes": [],
        }));
    }
    if has_orphans && scan_result.graph.nodes.is_empty() {
        actions.push(json!({
            "priority": 2,
            "action": "init_from_code",
            "command": "cairn init --from-code",
            "description": "No blueprint structure exists but source files were found. Generate an initial blueprint from the existing code.",
            "nodes": [],
        }));
    } else if has_orphans || has_ghosts {
        actions.push(json!({
            "priority": 2,
            "action": "refine",
            "command": "cairn refine",
            "description": "The blueprint has drifted from the code (ghost or orphaned modules). Generate a delta to reconcile the differences.",
            "nodes": [],
        }));
    }
    if has_gitignored_paths {
        actions.push(json!({
            "priority": 2,
            "action": "fix_gitignored_path",
            "command": "cairn lint",
            "description": "One or more declared paths match a .gitignore pattern and will appear as Ghost nodes. Un-ignore the path in .gitignore, or correct the path declaration in the blueprint.",
            "nodes": gitignored_nodes,
        }));
    }
    if has_interface_changes && !summarise_nodes.is_empty() {
        let commands: Vec<String> = summarise_nodes
            .iter()
            .map(|n| format!("cairn summarise {n}"))
            .collect();
        actions.push(json!({
            "priority": 3,
            "action": "summarise",
            "command": commands.join("; "),
            "description": "Interface hashes have changed. Run the summariser on affected nodes to update contracts.",
            "nodes": summarise_nodes,
        }));
    }
    if has_contract_issues {
        actions.push(json!({
            "priority": 3,
            "action": "fix_contracts",
            "command": "cairn lint",
            "description": "Contract artefacts have issues (missing, wrong node, or unknown node). Review and fix contract assignments.",
            "nodes": [],
        }));
    }
    if has_missing_decisions && !decision_nodes.is_empty() {
        actions.push(json!({
            "priority": 3,
            "action": "add_decision",
            "command": format!("cairn change new <change-id>  // affected: {}", decision_nodes.join(", ")),
            "description": "Blueprint changes require a recorded decision. Create a change directory with a decision artefact.",
            "nodes": decision_nodes,
        }));
    } else if has_missing_decisions {
        actions.push(json!({
            "priority": 3,
            "action": "add_decision",
            "command": "cairn change new <change-id>",
            "description": "Blueprint changes require a recorded decision. Create a change directory with a decision artefact.",
            "nodes": [],
        }));
    }
    if has_decision_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_decisions",
            "command": "cairn decisions",
            "description": "Decision artefacts have issues (orphaned, invalid status, or missing nodes). Review and fix decision files.",
            "nodes": [],
        }));
    }
    if has_todo_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_todos",
            "command": "cairn todos",
            "description": "Todo artefacts have issues (orphan node or invalid status). Review and fix todo files.",
            "nodes": [],
        }));
    }
    if has_source_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_sources",
            "command": "cairn sources",
            "description": "Source artefacts have issues (orphan, unverified, or read failed). Review and fix source files.",
            "nodes": [],
        }));
    }
    if has_research_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_research",
            "command": "cairn research",
            "description": "Research artefacts have issues (missing sources or unknown source). Review and fix research files.",
            "nodes": [],
        }));
    }
    if has_order_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_order",
            "command": "cairn order",
            "description": "A dependency cycle was detected in the blueprint. Review and fix dependency edges.",
            "nodes": [],
        }));
    }
    if actions.is_empty() {
        actions.push(json!({
            "priority": 0,
            "action": "none",
            "command": "",
            "description": "No remediation actions are required. The project is in good shape.",
            "nodes": [],
        }));
    }
    actions.sort_by_key(|a| {
        a.get("priority")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(99)
    });
    json!({
        "actions": actions,
        "total_actions": actions.len(),
    })
}
