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
        "title": title_from_body(&contract.body, "Contract"),
        "body": contract.body,
    })
}

pub(crate) fn docstring_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(
            request.node.as_ref(),
            "CAIRN_QUERY_MISSING_NODE",
            "node",
        )?)
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
            let mut target = json!({
                "path": report.target_id.path.to_string_lossy(),
                "language": report.language.as_str(),
                "reconciler_id": report.reconciler_id.0,
                "files": report.claimed_files,
            });
            if let Some(hash) = &report.hash {
                target["hash"] = json!(hash);
            }
            target
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node_record.id,
        "files": node_record.files,
        "targets": targets,
    }))
}

pub(crate) fn rationale_json(
    root: &std::path::Path,
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
        .map(|research| research_enriched_json(research, root))
        .collect::<Vec<_>>();
    let sources = scan_result
        .artefacts
        .sources
        .iter()
        .filter(|source| source_ids.contains(&source.id))
        .map(|source| source_enriched_json(source, root))
        .collect::<Vec<_>>();
    let decisions = decisions
        .iter()
        .map(|decision| {
            let mut value = decision_enriched_json(decision, root);
            // A decision pulled in through a neighbour node is transitive;
            // label it with the neighbour IDs it arrived via so consumers can
            // tell it apart from decisions naming the queried node directly.
            if !decision.nodes.contains(&node.id) {
                let via = decision
                    .nodes
                    .iter()
                    .filter(|id| node_ids.contains(*id))
                    .cloned()
                    .collect::<Vec<_>>();
                value["via"] = json!(via);
            }
            value
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node.id,
        "decisions": decisions,
        "research": research,
        "sources": sources,
    }))
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path};

    use super::*;
    use crate::{
        artefacts::{
            contract::ContractSet,
            registry::{ArtefactSet, Decision},
        },
        blueprint::{Ast, Edge, Node, NodeKind, Span},
        map::build_graph,
        scanner::ScanResult,
    };

    fn span() -> Span {
        Span::point("test.blueprint", 1, 1)
    }

    fn leaf(id: &str) -> Node {
        Node {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: span(),
        }
    }

    fn decision(id: &str, node: &str) -> Decision {
        Decision {
            id: id.to_owned(),
            path: format!("meta/decisions/{id}.md"),
            nodes: vec![node.to_owned()],
            status: DecisionStatus::Accepted,
            date: "2026-07-16".to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
            ratification: crate::artefacts::registry::RatificationTier::Binding,
            affects: Vec::new(),
            ratified_by_machine: false,
            receipts: Vec::new(),
        }
    }

    fn scan_with(nodes: Vec<Node>, edges: Vec<Edge>, decisions: Vec<Decision>) -> ScanResult {
        let ast = Ast { nodes, edges };
        let contracts = ContractSet::default();
        let mut claimed = BTreeMap::new();
        let graph = build_graph(&ast, Path::new("."), &contracts, &mut claimed, Vec::new());
        ScanResult {
            graph,
            target_hashes: BTreeMap::new(),
            interface_hash: String::new(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
            target_reports: Vec::new(),
            contracts,
            artefacts: ArtefactSet {
                decisions,
                ..ArtefactSet::default()
            },
        }
    }

    #[test]
    fn rationale_labels_neighbour_sourced_decisions_as_transitive() {
        let edge = Edge {
            from: "app.api".to_owned(),
            to: "app.db".to_owned(),
            description: String::new(),
            span: span(),
        };
        let scan = scan_with(
            vec![leaf("app.api"), leaf("app.db")],
            vec![edge],
            vec![
                decision("dec.direct", "app.api"),
                decision("dec.neighbour", "app.db"),
            ],
        );
        let result = rationale_json(Path::new("."), &scan, "app.api").expect("must resolve");
        let decisions = result["decisions"].as_array().expect("decisions array");
        let direct = decisions
            .iter()
            .find(|value| value["id"] == "dec.direct")
            .expect("direct decision present");
        assert!(
            direct.get("via").is_none(),
            "direct decision must carry no via label: {direct}"
        );
        let transitive = decisions
            .iter()
            .find(|value| value["id"] == "dec.neighbour")
            .expect("neighbour decision present");
        assert_eq!(
            transitive["via"],
            json!(["app.db"]),
            "neighbour-sourced decision must name the neighbour it arrived via"
        );
    }
}
