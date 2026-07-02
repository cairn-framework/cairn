//! `cairn bundle`: composes everything an agent needs to implement a node.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::util::*;
use super::super::*;

/// Composes everything an agent needs to implement a ghost node: contract
/// body (or `missing`), decisions naming the node, their rationale chain
/// (research/sources), dependency interfaces from outbound neighbours'
/// extracted symbols, and the standing quality gates.
pub(crate) fn bundle_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;

    let mut missing = Vec::new();
    let contract = node
        .contracts
        .iter()
        .filter_map(|path| scan_result.contracts.contracts.get(path))
        .find(|contract| contract.node == node.id)
        .map(|contract| contract.body.clone());
    if contract.is_none() {
        missing.push("contract");
    }

    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.status == DecisionStatus::Accepted && decision.nodes.contains(&node.id)
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

    let dependencies = scan_result
        .graph
        .outbound
        .get(&node.id)
        .into_iter()
        .flatten()
        .filter_map(|edge| scan_result.graph.nodes.get(&edge.to))
        .map(|target| {
            json!({
                "node": target.id,
                "symbols": target.symbols,
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "node": node.id,
        "contract": contract,
        "missing": missing,
        "decisions": decisions.iter().map(decision_json).collect::<Vec<_>>(),
        "rationale": { "research": research, "sources": sources },
        "dependencies": dependencies,
        "gates": crate::copy::lookup("brief.gates"),
    }))
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path};

    use super::*;
    use crate::{
        artefacts::{
            contract::{Contract, ContractSet},
            registry::{ArtefactSet, Decision},
        },
        blueprint::{Ast, Edge, Node, NodeKind, Span},
        map::build_graph,
        reconcile::{SymbolKind, SymbolRecord},
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
            date: "2026-07-02".to_owned(),
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
        }
    }

    fn scan_with(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        contracts: ContractSet,
        decisions: Vec<Decision>,
    ) -> ScanResult {
        let ast = Ast { nodes, edges };
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
    fn bundle_reports_missing_contract_for_uncontracted_node() {
        let scan = scan_with(
            vec![leaf("app.api")],
            Vec::new(),
            ContractSet::default(),
            Vec::new(),
        );
        let result = bundle_json(&scan, "app.api").expect("bundle must resolve a known node");
        assert!(result["contract"].is_null());
        assert_eq!(result["missing"], json!(["contract"]));
    }

    #[test]
    fn bundle_returns_finding_error_for_unknown_node() {
        let scan = scan_with(Vec::new(), Vec::new(), ContractSet::default(), Vec::new());
        let err = bundle_json(&scan, "app.bogus").expect_err("unknown node must error");
        assert_eq!(err.code, "CAIRN_QUERY_NODE_NOT_FOUND");
    }

    #[test]
    fn bundle_composes_contract_decisions_and_dependencies() {
        let mut contracts = ContractSet::default();
        contracts.contracts.insert(
            "meta/contracts/api.md".to_owned(),
            Contract {
                path: "meta/contracts/api.md".to_owned(),
                declared_by: "app.api".to_owned(),
                node: "app.api".to_owned(),
                body: "# Contract body".to_owned(),
                interface: Vec::new(),
            },
        );
        let mut api = leaf("app.api");
        api.contracts = vec!["meta/contracts/api.md".to_owned()];
        let mut db = leaf("app.db");
        db.tags = vec!["no-contract".to_owned()];
        let edge = Edge {
            from: "app.api".to_owned(),
            to: "app.db".to_owned(),
            description: String::new(),
            span: span(),
        };
        let mut scan = scan_with(
            vec![api, db],
            vec![edge],
            contracts,
            vec![decision("dec.api-shape", "app.api")],
        );
        scan.graph
            .nodes
            .get_mut("app.db")
            .unwrap()
            .symbols
            .push(SymbolRecord {
                name: "connect".to_owned(),
                kind: SymbolKind::Function,
                signature: "pub fn connect() -> Db".to_owned(),
                file: "src/db.rs".to_owned(),
                line: 1,
                end_line: 1,
            });

        let result = bundle_json(&scan, "app.api").expect("bundle must succeed");
        assert_eq!(result["contract"], json!("# Contract body"));
        assert_eq!(result["missing"], json!(Vec::<String>::new()));
        assert_eq!(result["decisions"][0]["id"], json!("dec.api-shape"));
        assert_eq!(result["dependencies"][0]["node"], json!("app.db"));
        assert_eq!(
            result["dependencies"][0]["symbols"][0]["signature"],
            json!("pub fn connect() -> Db")
        );
        assert!(!result["gates"].as_str().unwrap().is_empty());
    }
}
