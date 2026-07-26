//! Hook, health, and remediation query handlers.
// Reason: child module imports re-exported public surface from parent via use super::*
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
            "total_info": total_info,
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
pub(crate) fn remediate_actions_raw(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> Vec<Value> {
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
    let mut has_oversized_modules = false;
    let mut summarise_nodes: Vec<String> = Vec::new();
    let mut decision_nodes: Vec<String> = Vec::new();
    let mut gitignored_nodes: Vec<String> = Vec::new();
    let mut oversized_nodes: Vec<String> = Vec::new();
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
            | "CAIRN_RECONCILE_READ_SOURCE"
            | "CAIRN_RECONCILE_LANGUAGE_UNKNOWN"
            | "CAIRN_RECONCILE_EMPTY_TARGET" => {
                has_orphans = true;
            }
            "CAIRN_MODULE_OVERSIZED" => {
                has_oversized_modules = true;
                if let Some(node) = &finding.node
                    && !oversized_nodes.contains(node)
                {
                    oversized_nodes.push(node.clone());
                }
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
            "description": crate::copy::lookup("remediate.actions.fix_blueprint"),
            "nodes": [],
        }));
    }
    if has_orphans && scan_result.graph.nodes.is_empty() {
        actions.push(json!({
            "priority": 2,
            "action": "init_from_code",
            "command": "cairn init --from-code",
            "description": crate::copy::lookup("remediate.actions.init_from_code"),
            "nodes": [],
        }));
    } else if has_orphans || has_ghosts {
        actions.push(json!({
            "priority": 2,
            "action": "refine",
            "command": "cairn refine",
            "description": crate::copy::lookup("remediate.actions.refine"),
            "nodes": [],
        }));
    }
    if has_gitignored_paths {
        actions.push(json!({
            "priority": 2,
            "action": "fix_gitignored_path",
            "command": "cairn lint",
            "description": crate::copy::lookup("remediate.actions.fix_gitignored_path"),
            "nodes": gitignored_nodes,
        }));
    }
    if has_interface_changes && !summarise_nodes.is_empty() {
        let commands: Vec<String> = summarise_nodes
            .iter()
            .map(|n| format!("cairn draft create {n} --json"))
            .collect();
        actions.push(json!({
            "priority": 3,
            "action": "summarise",
            "command": commands.join("; "),
            "description": crate::copy::lookup("remediate.actions.summarise"),
            "nodes": summarise_nodes,
        }));
    }
    if has_contract_issues {
        actions.push(json!({
            "priority": 3,
            "action": "fix_contracts",
            "command": "cairn lint",
            "description": crate::copy::lookup("remediate.actions.fix_contracts"),
            "nodes": [],
        }));
    }
    if has_missing_decisions && !decision_nodes.is_empty() {
        actions.push(json!({
            "priority": 3,
            "action": "add_decision",
            "command": format!("cairn change new <change-id>  // affected: {}", decision_nodes.join(", ")),
            "description": crate::copy::lookup("remediate.actions.add_decision"),
            "nodes": decision_nodes,
        }));
    } else if has_missing_decisions {
        actions.push(json!({
            "priority": 3,
            "action": "add_decision",
            "command": "cairn change new <change-id>",
            "description": crate::copy::lookup("remediate.actions.add_decision"),
            "nodes": [],
        }));
    }
    if has_decision_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_decisions",
            "command": "cairn decisions",
            "description": crate::copy::lookup("remediate.actions.fix_decisions"),
            "nodes": [],
        }));
    }
    if has_todo_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_todos",
            "command": "cairn todos",
            "description": crate::copy::lookup("remediate.actions.fix_todos"),
            "nodes": [],
        }));
    }
    if has_source_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_sources",
            "command": "cairn sources",
            "description": crate::copy::lookup("remediate.actions.fix_sources"),
            "nodes": [],
        }));
    }
    if has_research_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_research",
            "command": "cairn research",
            "description": crate::copy::lookup("remediate.actions.fix_research"),
            "nodes": [],
        }));
    }
    if has_order_issues {
        actions.push(json!({
            "priority": 4,
            "action": "fix_order",
            "command": "cairn order",
            "description": crate::copy::lookup("remediate.actions.fix_order"),
            "nodes": [],
        }));
    }
    if has_oversized_modules {
        actions.push(json!({
            "priority": 4,
            "action": "split_module",
            "command": "cairn lint",
            "description": crate::copy::lookup("remediate.actions.split_module"),
            "nodes": oversized_nodes,
        }));
    }
    if actions.is_empty() {
        actions.push(json!({
            "priority": 0,
            "action": "none",
            "command": "",
            "description": crate::copy::lookup("remediate.actions.none"),
            "nodes": [],
        }));
    }
    actions.sort_by_key(|a| {
        a.get("priority")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(99)
    });
    actions
}

/// Wire shape of the `remediate` response: remediation actions projected to
/// the shared work-item shape, plus the count.
///
/// `schema_version` is last so appending it here lines up with the byte
/// order `execute`/`execute_with_scan` already produce by stamping it onto
/// the JSON object after the handler returns; see `query_api::SCHEMA_VERSION`.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct RemediateResponse {
    /// Ordered remediation actions, one per addressable finding.
    pub actions: Vec<WorkItem>,
    /// Number of actions in `actions`.
    pub total_actions: usize,
    /// Wire schema version stamped on every query-API response.
    pub schema_version: u32,
}

/// Returns remediation actions projected to the shared work-item wire shape.
pub(crate) fn remediate_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> Value {
    let actions = remediate_actions_raw(root, changes_dir, scan_result);
    let projected: Vec<WorkItem> = actions
        .iter()
        .filter_map(super::work_item::from_finding_action)
        .collect();
    let response = RemediateResponse {
        total_actions: projected.len(),
        actions: projected,
        schema_version: super::super::SCHEMA_VERSION,
    };
    serde_json::to_value(response).expect("RemediateResponse serialises")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn emitted_remediation_description_uses_copy_entry() {
        let mut scan_result = scanner::ScanResult {
            graph: crate::map::Graph {
                nodes: BTreeMap::new(),
                names: BTreeMap::new(),
                outbound: BTreeMap::new(),
                inbound: BTreeMap::new(),
                findings: Vec::new(),
            },
            artefacts: crate::artefacts::registry::ArtefactSet::default(),
            contracts: crate::artefacts::contract::ContractSet::default(),
            interface_hash: String::new(),
            target_reports: Vec::new(),
            target_hashes: scanner::state::TargetHashes::default(),
            blueprint_snapshot: scanner::state::BlueprintSnapshot::default(),
        };
        let actions =
            remediate_actions_raw(Path::new("."), Path::new("meta/changes"), &scan_result);
        let description = actions
            .iter()
            .find(|action| action["action"] == "none")
            .and_then(|action| action["description"].as_str());

        assert_eq!(
            description,
            Some(crate::copy::lookup("remediate.actions.none"))
        );
        assert_ne!(description, Some("remediate.actions.none"));

        for action in [
            "fix_blueprint",
            "init_from_code",
            "refine",
            "fix_gitignored_path",
            "summarise",
            "fix_contracts",
            "add_decision",
            "fix_decisions",
            "fix_todos",
            "fix_sources",
            "fix_research",
            "fix_order",
            "split_module",
            "none",
        ] {
            let key = format!("remediate.actions.{action}");
            assert_ne!(crate::copy::lookup(&key), key);
        }

        scan_result.graph.findings.push(crate::map::Finding {
            code: "CAIRN_ORDER_CYCLE".to_owned(),
            severity: crate::map::FindingSeverity::Error,
            message: "cycle".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
        });
        let actions =
            remediate_actions_raw(Path::new("."), Path::new("meta/changes"), &scan_result);
        let description = actions
            .iter()
            .find(|action| action["action"] == "fix_order")
            .and_then(|action| action["description"].as_str());
        assert_eq!(
            description,
            Some(crate::copy::lookup("remediate.actions.fix_order"))
        );
    }
}
