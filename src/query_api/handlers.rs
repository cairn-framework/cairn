// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use serialise::{
    decision_json, findings_json, hook_decision_name, hook_kind_name, neighbourhood_ids,
    parse_decision_status_filter, parse_todo_status_filter, research_for_nodes, research_json,
    review_json, source_json, sources_for_nodes, todo_json,
};
use util::required;

fn count_findings(findings: &[crate::map::graph::Finding]) -> (usize, usize, usize) {
    let mut errors = 0usize;
    let mut warnings = 0usize;
    let mut info = 0usize;
    for f in findings {
        match f.severity {
            FindingSeverity::Error => errors += 1,
            FindingSeverity::Warning => warnings += 1,
            FindingSeverity::Info => info += 1,
        }
    }
    (errors, warnings, info)
}

pub(super) fn neighbourhood_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let response =
        query::neighbourhood(&scan_result.graph, required(request.node.as_ref(), "node")?)
            .map_err(finding_error)?;
    let node_ids = neighbourhood_ids(&scan_result.graph, &response.node.id);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.nodes.iter().any(|node| node_ids.contains(node))
                && (decision.status == DecisionStatus::Accepted
                    || request.has(QueryFlag::IncludeDeprecatedDecisions))
        })
        .cloned()
        .collect::<Vec<_>>();
    let todos = if request.has(QueryFlag::IncludeTodos) {
        scan_result
            .artefacts
            .todos
            .iter()
            .filter(|todo| node_ids.contains(&todo.node))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let research = if request.has(QueryFlag::IncludeResearch) {
        research_for_nodes(scan_result, &node_ids)
    } else {
        Vec::new()
    };
    let reviews = if request.has(QueryFlag::IncludeReviews) {
        scan_result
            .artefacts
            .reviews
            .iter()
            .filter(|review| node_ids.contains(&review.node))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let mut data = json!({
        "node": node_json(&response.node),
        "inbound": response.inbound,
        "outbound": response.outbound,
        "contracts": response.node.contracts,
        "decisions": decisions.iter().map(decision_json).collect::<Vec<_>>(),
        "todos": todos.iter().map(todo_json).collect::<Vec<_>>(),
        "research": research.iter().map(research_json).collect::<Vec<_>>(),
        "reviews": reviews.iter().map(review_json).collect::<Vec<_>>(),
    });
    if request.has(QueryFlag::IncludeChanges) {
        data["active_changes"] = json!([]);
    }
    let error_count = scan_result
        .graph
        .findings
        .iter()
        .filter(|f| f.severity == FindingSeverity::Error)
        .count();
    if error_count > 0 {
        data["warnings"] = json!([format!(
            "scan has {error_count} error(s); graph may be incomplete"
        )]);
    }
    Ok(data)
}

pub(super) fn contract_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let contracts = node
        .contracts
        .iter()
        .filter_map(|path| scan_result.contracts.contracts.get(path))
        .filter(|contract| contract.node == node.id)
        .map(single_contract_json)
        .collect::<Vec<_>>();
    let body = contracts
        .first()
        .and_then(|contract| contract.get("body"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    Ok(json!({ "node": node.id, "contract": body, "contracts": contracts }))
}

pub(super) fn single_contract_json(contract: &Contract) -> Value {
    json!({
        "path": contract.path,
        "node": contract.node,
        "declared_by": contract.declared_by,
        "body": contract.body,
    })
}

pub(super) fn docstring_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(request.node.as_ref(), "node")?)
        .map_err(finding_error)?;
    let language = request.language.as_deref().unwrap_or("rust");
    let depends = query::depends(&scan_result.graph, &node.id, false)
        .map_err(finding_error)?
        .nodes;
    let prefix = match language {
        "python" => "#",
        "typescript" | "go" => "//",
        _ => "//!",
    };
    let lines = [
        format!("{prefix} {}", node.name),
        prefix.to_string(),
        format!("{prefix} Cairn-ID: {}", node.id),
        format!("{prefix} Cairn-Description: {}", node.description),
        format!("{prefix} Cairn-Depends: {}", depends.join(", ")),
        format!("{prefix} Cairn-Tags: {}", node.tags.join(", ")),
    ];
    Ok(json!({
        "node": node.id,
        "language": language,
        "docstring": lines.join("\n"),
    }))
}

pub(super) fn files_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node_record = scan_result.graph.resolve(node).map_err(finding_error)?;
    let targets = scan_result
        .target_reports
        .iter()
        .filter(|report| report.target_id.node_id == node_record.id)
        .map(|report| {
            json!({
                "path": report.target_id.path.to_string_lossy(),
                "language": report.language.as_str(),
                "reconciler_id": report.reconciler_id.0,
                "files": report.claimed_files,
                "hash": report.hash,
            })
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node_record.id,
        "files": node_record.files,
        "targets": targets,
    }))
}

pub(super) fn dependency_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
    outbound: bool,
) -> Result<Value, QueryError> {
    let node = required(request.node.as_ref(), "node")?;
    let response = if outbound {
        query::depends(&scan_result.graph, node, request.has(QueryFlag::Transitive))
    } else {
        query::dependents(&scan_result.graph, node, request.has(QueryFlag::Transitive))
    }
    .map_err(finding_error)?;
    Ok(json!({ "node": response.node, "nodes": response.nodes }))
}

pub(super) fn status_json(root: &Path, scan_result: &scanner::ScanResult) -> Value {
    let open = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open || todo.status == TodoStatus::InProgress)
        .map(todo_json)
        .collect::<Vec<_>>();
    let log_entries: Vec<String> = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            content
                .lines()
                .rev()
                .take(5)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();
    json!({
        "active_changes": [],
        "open_todos": open,
        "recent_log_entries": log_entries,
    })
}

pub(super) fn context_json(
    scan_result: &scanner::ScanResult,
    config: &scanner::config::Config,
) -> Value {
    let system_name = scan_result
        .graph
        .nodes
        .values()
        .find(|n| n.kind == crate::blueprint::ast::NodeKind::System)
        .map_or("unknown", |n| n.name.as_str());

    let edge_count: usize = scan_result.graph.outbound.values().map(Vec::len).sum();

    let system_description = scan_result
        .graph
        .nodes
        .values()
        .find(|n| n.kind == crate::blueprint::ast::NodeKind::System)
        .map_or("", |n| n.description.as_str());

    let nodes: Vec<Value> = scan_result
        .graph
        .nodes
        .values()
        .map(|n| {
            let kind = match n.kind {
                crate::blueprint::ast::NodeKind::System => "system",
                crate::blueprint::ast::NodeKind::Container => "container",
                crate::blueprint::ast::NodeKind::Module => "module",
                crate::blueprint::ast::NodeKind::Actor => "actor",
            };
            let state = match n.state {
                crate::map::graph::NodeState::Synced => "synced",
                crate::map::graph::NodeState::Ghost => "ghost",
                crate::map::graph::NodeState::Orphaned => "orphaned",
            };
            json!({
                "id": n.id,
                "name": n.name,
                "kind": kind,
                "state": state,
                "paths": n.paths,
                "children": n.children,
            })
        })
        .collect();

    let (errors, warnings, info) = count_findings(&scan_result.graph.findings);

    json!({
        "system_name": system_name,
        "system_description": system_description,
        "project_context": config.context,
        "node_count": scan_result.graph.nodes.len(),
        "edge_count": edge_count,
        "nodes": nodes,
        "artefact_counts": {
            "contracts": scan_result.artefacts.contracts.contracts.len(),
            "decisions": scan_result.artefacts.decisions.len(),
            "todos": scan_result.artefacts.todos.len(),
            "research": scan_result.artefacts.research.len(),
            "reviews": scan_result.artefacts.reviews.len(),
            "sources": scan_result.artefacts.sources.len(),
        },
        "finding_counts": {
            "error": errors,
            "warning": warnings,
            "info": info,
        },
    })
}

pub(super) fn rationale_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let node_ids = neighbourhood_ids(&scan_result.graph, &node.id);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.status == DecisionStatus::Accepted
                && decision.nodes.iter().any(|id| node_ids.contains(id))
        })
        .cloned()
        .collect::<Vec<_>>();
    let research_ids = decisions
        .iter()
        .flat_map(|decision| decision.informed_by.iter())
        .cloned()
        .collect::<BTreeSet<_>>();
    let source_ids = decisions
        .iter()
        .flat_map(|decision| decision.informed_by.iter())
        .cloned()
        .chain(
            scan_result
                .artefacts
                .research
                .iter()
                .filter(|research| research_ids.contains(&research.id))
                .flat_map(|research| research.sources.iter().cloned()),
        )
        .collect::<BTreeSet<_>>();
    let research = scan_result
        .artefacts
        .research
        .iter()
        .filter(|research| research_ids.contains(&research.id))
        .map(research_json)
        .collect::<Vec<_>>();
    let sources = scan_result
        .artefacts
        .sources
        .iter()
        .filter(|source| source_ids.contains(&source.id))
        .map(source_json)
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node.id,
        "decisions": decisions.iter().map(decision_json).collect::<Vec<_>>(),
        "research": research,
        "sources": sources,
    }))
}

pub(super) fn todos_response_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(request.node.as_ref(), "node")?)
        .map_err(finding_error)?;
    let status = request.status.as_deref().and_then(parse_todo_status_filter);
    let todos = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.node == node.id && status.is_none_or(|filter| todo.status == filter))
        .map(todo_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "todos": todos }))
}

pub(super) fn decisions_response_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(request.node.as_ref(), "node")?)
        .map_err(finding_error)?;
    let status = request
        .status
        .as_deref()
        .and_then(parse_decision_status_filter);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.nodes.contains(&node.id)
                && status.is_none_or(|filter| decision.status == filter)
        })
        .map(decision_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "decisions": decisions }))
}

pub(super) fn research_response_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(research_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "research": research }))
}

pub(super) fn sources_response_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(source_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "sources": sources }))
}

pub(super) fn islands_json(scan_result: &scanner::ScanResult) -> Value {
    let response = query::islands(&scan_result.graph);
    let islands: Vec<Value> = response
        .islands
        .iter()
        .map(|island| {
            json!({
                "representative": island.representative,
                "node_count": island.node_count,
            })
        })
        .collect();
    json!({
        "schema_version": response.schema_version,
        "islands": islands,
    })
}

pub(super) fn hook_json(
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
