//! Node-level query handlers for contracts, docstrings, files, and rationale.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::util::*;
use super::super::*;

pub(crate) fn contract_json(
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

fn single_contract_json(contract: &Contract) -> Value {
    json!({
        "path": contract.path,
        "node": contract.node,
        "declared_by": contract.declared_by,
        "body": contract.body,
    })
}

pub(crate) fn docstring_json(
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

pub(crate) fn files_json(
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

pub(crate) fn rationale_json(
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
