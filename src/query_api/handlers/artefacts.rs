//! Artefact query handlers for todos, decisions, research, and sources.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::util::*;
use super::super::*;

pub(crate) fn todos_response_json(
    root: &std::path::Path,
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
        .map(|todo| todo_enriched_json(todo, root))
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "todos": todos }))
}

pub(crate) fn decisions_response_json(
    root: &std::path::Path,
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
        .map(|decision| decision_enriched_json(decision, root))
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "decisions": decisions }))
}

pub(crate) fn research_response_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(|research| research_enriched_json(research, root))
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "research": research }))
}

pub(crate) fn sources_response_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(|source| source_enriched_json(source, root))
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "sources": sources }))
}
