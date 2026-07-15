//! Map graph builder.

use crate::{
    artefacts::contract::ContractSet,
    blueprint::{Ast, Edge, Node},
};
use std::{collections::BTreeMap, path::Path};

use super::graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};

/// Builds a graph from parsed blueprint, contracts, claimed files, and findings.
#[must_use]
pub fn build_graph(
    ast: &Ast,
    root: &Path,
    contracts: &ContractSet,
    claimed_files: &mut BTreeMap<String, Vec<String>>,
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
    super::test_coverage::validate_test_coverage(&mut graph, root);
    super::contract_coverage::validate_contract_coverage(&mut graph);
    super::spec_rule_coverage::validate_spec_rule_coverage(&mut graph, root);
    graph
}

fn insert_node(
    graph: &mut Graph,
    node: &Node,
    parent: Option<&str>,
    root: &Path,
    claimed_files: &mut BTreeMap<String, Vec<String>>,
) {
    let is_internal = !node.children.is_empty();
    let owns_files = !is_internal || node.owns_files;
    let files = claimed_files.remove(&node.id).unwrap_or_default();
    let state = derive_node_state(node, owns_files, &files, root);
    let children = node.children.iter().map(|child| child.id.clone()).collect();
    if graph.nodes.contains_key(&node.id) {
        graph.findings.push(Finding {
            code: "CAIRN_INTEGRITY_DUPLICATE_ID".to_owned(),
            severity: FindingSeverity::Error,
            message: format!("duplicate node id `{}`", node.id),
            node: Some(node.id.clone()),
            target: None,
            path: None,
            deferred_by: None,
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
            symbols: Vec::new(),
            span: node.span.clone(),
        },
    );
    for child in &node.children {
        insert_node(graph, child, Some(&node.id), root, claimed_files);
    }
}

/// Derives [`NodeState`] for a node from its paths, file ownership, and claims.
///
/// - No path declarations → [`NodeState::Synced`] (pure declaration / actor).
/// - Owns claimed source files → [`NodeState::Synced`].
/// - File-owning node with zero claimed files → [`NodeState::Ghost`], whether
///   the path is missing or is empty scaffolding (gh:#238). Empty scaffolding
///   is not implementation; matches "declared structure, no code yet" from
///   `dec.ghost-rule-tracking`.
/// - Non-owning container whose path exists → [`NodeState::Synced`] (children
///   own the files under most-specific-owner semantics).
/// - Non-owning container with no existing path → [`NodeState::Ghost`].
fn derive_node_state(node: &Node, owns_files: bool, files: &[String], root: &Path) -> NodeState {
    if node.paths.is_empty() || !files.is_empty() {
        return NodeState::Synced;
    }
    if owns_files {
        // Leaf (or owns_files container) with declared paths and no claims.
        return NodeState::Ghost;
    }
    if node.paths.iter().any(|path| root.join(path).exists()) {
        NodeState::Synced
    } else {
        NodeState::Ghost
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
                target: None,
                path: None,
                deferred_by: None,
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
                message: format!(
                    "node id `{id}` must be a lowercase dotted identifier (a-z, 0-9, `.`, `-`; underscores are not allowed)"
                ),
                node: Some(id.clone()),
                target: None,
                path: None,
                deferred_by: None,
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
                target: None,
                path: Some(path),
                deferred_by: None,
            });
        }
    }
}

fn validate_contracts(graph: &mut Graph, root: &Path, contracts: &ContractSet) {
    for contract in contracts.contracts.values() {
        if !graph.nodes.contains_key(&contract.node) {
            graph.findings.push(Finding {
                code: "CAIRN_CONTRACT_UNKNOWN_NODE".to_owned(),
                severity: FindingSeverity::Error,
                message: format!("contract references unknown node `{}`", contract.node),
                node: Some(contract.node.clone()),
                target: None,
                path: Some(contract.path.clone()),
                deferred_by: None,
            });
        }
    }
    for node in graph.nodes.values() {
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
                    target: None,
                    path: Some(pointer.clone()),
                    deferred_by: None,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests;
