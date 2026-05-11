// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use serialise::{
    decision_json, findings_json, hook_decision_name, hook_kind_name, neighbourhood_ids,
    parse_decision_status_filter, parse_todo_status_filter, research_for_nodes, research_json,
    review_json, source_json, sources_for_nodes, todo_json,
};
use util::required;

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

    let nodes: Vec<Value> = scan_result
        .graph
        .nodes
        .values()
        .map(|n| {
            json!({
                "id": n.id,
                "name": n.name,
                "kind": format!("{:?}", n.kind),
                "state": format!("{:?}", n.state),
                "paths": n.paths,
                "children": n.children,
            })
        })
        .collect();

    let (errors, warnings, info) = {
        let mut e = 0usize;
        let mut w = 0usize;
        let mut i = 0usize;
        for f in &scan_result.graph.findings {
            match f.severity {
                FindingSeverity::Error => e += 1,
                FindingSeverity::Warning => w += 1,
                FindingSeverity::Info => i += 1,
            }
        }
        (e, w, i)
    };

    json!({
        "system_name": system_name,
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
        "all" => HookKind::All,
        other => {
            return Err(QueryError {
                code: "CAIRN_QUERY_INVALID_HOOK_KIND".to_owned(),
                message: format!("unknown hook kind `{other}`"),
                source_span: None,
                remediation: Some("Use structural, interface, tension, or all.".to_owned()),
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
