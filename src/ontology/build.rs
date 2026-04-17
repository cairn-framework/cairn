//! Ontology graph builder.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::{
    artefacts::contract::ContractSet,
    dsl::{Ast, Edge, Node},
};

use super::graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};

/// Builds a graph from parsed DSL, contracts, claimed files, and findings.
#[must_use]
pub fn build_graph(
    ast: &Ast,
    root: &Path,
    contracts: &ContractSet,
    claimed_files: &BTreeMap<String, Vec<String>>,
    external_findings: Vec<Finding>,
) -> Graph {
    let mut graph = Graph {
        nodes: BTreeMap::new(),
        names: BTreeMap::new(),
        outbound: BTreeMap::new(),
        inbound: BTreeMap::new(),
        findings: external_findings,
    };
    for node in &ast.nodes {
        insert_node(&mut graph, node, None, root, claimed_files);
    }
    validate_edges(&mut graph, &ast.edges);
    validate_ids(&mut graph);
    validate_path_ties(&mut graph);
    validate_contracts(&mut graph, root, contracts);
    graph
}

fn insert_node(
    graph: &mut Graph,
    node: &Node,
    parent: Option<&str>,
    root: &Path,
    claimed_files: &BTreeMap<String, Vec<String>>,
) {
    let is_internal = !node.children.is_empty();
    let owns_files = !is_internal || node.owns_files;
    let files = claimed_files.get(&node.id).cloned().unwrap_or_default();
    let state = if node.paths.is_empty()
        || node.paths.iter().any(|path| root.join(path).exists())
        || !files.is_empty()
    {
        NodeState::Synced
    } else {
        NodeState::Ghost
    };
    let children = node.children.iter().map(|child| child.id.clone()).collect();
    if graph.nodes.contains_key(&node.id) {
        graph.findings.push(Finding {
            code: "CAIRN_INTEGRITY_DUPLICATE_ID".to_owned(),
            severity: FindingSeverity::Error,
            message: format!("duplicate node id `{}`", node.id),
            node: Some(node.id.clone()),
            path: None,
        });
        return;
    }
    graph
        .names
        .entry(node.name.clone())
        .or_default()
        .push(node.id.clone());
    graph.nodes.insert(
        node.id.clone(),
        NodeRecord {
            kind: node.kind,
            id: node.id.clone(),
            name: node.name.clone(),
            description: node.description.clone(),
            tags: node.tags.clone(),
            parent: parent.map(ToOwned::to_owned),
            children,
            paths: node.paths.clone(),
            owns_files,
            contracts: node.contracts.clone(),
            state,
            files,
            span: node.span.clone(),
        },
    );
    for child in &node.children {
        insert_node(graph, child, Some(&node.id), root, claimed_files);
    }
}

fn validate_edges(graph: &mut Graph, edges: &[Edge]) {
    for edge in edges {
        if !graph.nodes.contains_key(&edge.from) || !graph.nodes.contains_key(&edge.to) {
            graph.findings.push(Finding {
                code: "CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT".to_owned(),
                severity: FindingSeverity::Error,
                message: format!(
                    "edge references missing endpoint `{}` -> `{}`",
                    edge.from, edge.to
                ),
                node: None,
                path: None,
            });
            continue;
        }
        let edge_ref = EdgeRef {
            from: edge.from.clone(),
            to: edge.to.clone(),
            description: edge.description.clone(),
        };
        graph
            .outbound
            .entry(edge.from.clone())
            .or_default()
            .push(edge_ref.clone());
        graph
            .inbound
            .entry(edge.to.clone())
            .or_default()
            .push(edge_ref);
    }
}

fn validate_ids(graph: &mut Graph) {
    for id in graph.nodes.keys() {
        if !id
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '-')
        {
            graph.findings.push(Finding {
                code: "CAIRN_INTEGRITY_INVALID_ID".to_owned(),
                severity: FindingSeverity::Error,
                message: format!("node id `{id}` must be lowercase dotted identifier"),
                node: Some(id.clone()),
                path: None,
            });
        }
    }
}

fn validate_path_ties(graph: &mut Graph) {
    let mut seen = BTreeMap::<String, Vec<String>>::new();
    for node in graph.nodes.values() {
        if node.owns_files {
            for path in &node.paths {
                seen.entry(path.clone()).or_default().push(node.id.clone());
            }
        }
    }
    for (path, ids) in seen {
        if ids.len() > 1 {
            graph.findings.push(Finding {
                code: "CAIRN_INTEGRITY_PATH_TIE".to_owned(),
                severity: FindingSeverity::Error,
                message: format!(
                    "path `{path}` is owned by multiple nodes: {}",
                    ids.join(", ")
                ),
                node: None,
                path: Some(path),
            });
        }
    }
}

fn validate_contracts(graph: &mut Graph, root: &Path, contracts: &ContractSet) {
    let ids = graph.nodes.keys().cloned().collect::<BTreeSet<_>>();
    for contract in contracts.contracts.values() {
        if !ids.contains(&contract.node) {
            graph.findings.push(Finding {
                code: "CAIRN_CONTRACT_UNKNOWN_NODE".to_owned(),
                severity: FindingSeverity::Error,
                message: format!("contract references unknown node `{}`", contract.node),
                node: Some(contract.node.clone()),
                path: Some(contract.path.clone()),
            });
        }
    }
    let nodes = graph.nodes.values().cloned().collect::<Vec<_>>();
    for node in nodes {
        for pointer in &node.contracts {
            let full = root.join(pointer);
            if !full.exists() {
                graph.findings.push(Finding {
                    code: "CAIRN_CONTRACT_MISSING".to_owned(),
                    severity: if node.state == NodeState::Ghost {
                        FindingSeverity::Warning
                    } else {
                        FindingSeverity::Error
                    },
                    message: format!("contract pointer `{pointer}` is missing for `{}`", node.id),
                    node: Some(node.id.clone()),
                    path: Some(pointer.clone()),
                });
            }
        }
    }
}
