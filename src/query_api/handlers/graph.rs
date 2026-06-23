//! Graph topology query handlers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::util::*;
use super::super::*;

pub(crate) fn count_findings(findings: &[crate::map::graph::Finding]) -> (usize, usize, usize) {
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

pub(crate) fn neighbourhood_json(
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

pub(crate) fn dependency_json(
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

pub(crate) fn islands_json(scan_result: &scanner::ScanResult) -> Value {
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
    json!({ "islands": islands })
}
