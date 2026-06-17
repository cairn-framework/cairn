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
            target: None,
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
                target: None,
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
                message: format!(
                    "node id `{id}` must be a lowercase dotted identifier (a-z, 0-9, `.`, `-`; underscores are not allowed)"
                ),
                node: Some(id.clone()),
                target: None,
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
                target: None,
                path: Some(path),
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
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artefacts::contract::ContractSet;
    use crate::blueprint::{Ast, Edge, Node, NodeKind, Span};
    use std::collections::BTreeMap;

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

    fn leaf_with_path(id: &str, path: &str) -> Node {
        Node {
            paths: vec![path.to_owned()],
            ..leaf(id)
        }
    }

    fn edge(from: &str, to: &str) -> Edge {
        Edge {
            from: from.to_owned(),
            to: to.to_owned(),
            description: "dep".to_owned(),
            span: span(),
        }
    }

    fn ast(nodes: Vec<Node>, edges: Vec<Edge>) -> Ast {
        Ast { nodes, edges }
    }

    fn build(ast: &Ast) -> Graph {
        build_graph(
            ast,
            Path::new("/nonexistent"),
            &ContractSet::default(),
            &mut BTreeMap::new(),
            Vec::new(),
        )
    }

    fn codes(graph: &Graph) -> Vec<&str> {
        graph.findings.iter().map(|f| f.code.as_str()).collect()
    }

    // ── node insertion ────────────────────────────────────────────────────────

    #[test]
    fn test_build_single_node_appears_in_graph() {
        let a = ast(vec![leaf("app.api")], vec![]);
        let g = build(&a);
        assert!(g.nodes.contains_key("app.api"));
        assert!(g.findings.is_empty(), "no findings: {:#?}", g.findings);
    }

    #[test]
    fn test_build_node_fields_propagated_correctly() {
        let mut n = leaf("app.api");
        n.description = "The API".to_owned();
        n.tags = vec!["public".to_owned()];
        let a = ast(vec![n], vec![]);
        let g = build(&a);
        let node = g.nodes.get("app.api").unwrap();
        assert_eq!(node.description, "The API");
        assert_eq!(node.tags, vec!["public"]);
    }

    #[test]
    fn test_build_name_index_populated() {
        let a = ast(vec![leaf("app.api")], vec![]);
        let g = build(&a);
        assert_eq!(g.names.get("app.api"), Some(&vec!["app.api".to_owned()]));
    }

    // ── duplicate id ──────────────────────────────────────────────────────────

    #[test]
    fn test_build_duplicate_node_id_emits_integrity_error() {
        let a = ast(vec![leaf("app.api"), leaf("app.api")], vec![]);
        let g = build(&a);
        assert!(
            codes(&g).contains(&"CAIRN_INTEGRITY_DUPLICATE_ID"),
            "duplicate id must emit CAIRN_INTEGRITY_DUPLICATE_ID: {:?}",
            codes(&g)
        );
    }

    // ── parent / children wiring ──────────────────────────────────────────────

    #[test]
    fn test_build_child_node_has_parent_set() {
        let parent = Node {
            children: vec![leaf("app.api")],
            ..leaf("app")
        };
        let a = ast(vec![parent], vec![]);
        let g = build(&a);
        let child = g.nodes.get("app.api").unwrap();
        assert_eq!(child.parent, Some("app".to_owned()));
    }

    #[test]
    fn test_build_parent_children_field_lists_child_ids() {
        let parent = Node {
            children: vec![leaf("app.api"), leaf("app.worker")],
            ..leaf("app")
        };
        let a = ast(vec![parent], vec![]);
        let g = build(&a);
        let parent_rec = g.nodes.get("app").unwrap();
        let mut children = parent_rec.children.clone();
        children.sort();
        assert_eq!(children, vec!["app.api", "app.worker"]);
    }

    // ── edges ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_build_valid_edge_populates_outbound_and_inbound() {
        let a = ast(vec![leaf("a"), leaf("b")], vec![edge("a", "b")]);
        let g = build(&a);
        assert!(g.outbound.contains_key("a"), "outbound must have 'a'");
        assert!(g.inbound.contains_key("b"), "inbound must have 'b'");
        assert_eq!(g.outbound["a"][0].to, "b");
        assert_eq!(g.inbound["b"][0].from, "a");
        assert!(g.findings.is_empty());
    }

    #[test]
    fn test_build_edge_with_missing_from_emits_invalid_endpoint() {
        let a = ast(vec![leaf("b")], vec![edge("missing", "b")]);
        let g = build(&a);
        assert!(
            codes(&g).contains(&"CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT"),
            "missing from-node must emit error: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn test_build_edge_with_missing_to_emits_invalid_endpoint() {
        let a = ast(vec![leaf("a")], vec![edge("a", "missing")]);
        let g = build(&a);
        assert!(codes(&g).contains(&"CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT"));
    }

    // ── id validation ─────────────────────────────────────────────────────────

    #[test]
    fn test_build_valid_id_produces_no_integrity_finding() {
        // Valid: lowercase, digits, dots, hyphens.
        for id in &["app.api", "app-v2", "app.api-v2", "a1.b2"] {
            let a = ast(vec![leaf(id)], vec![]);
            let g = build(&a);
            assert!(
                !codes(&g).contains(&"CAIRN_INTEGRITY_INVALID_ID"),
                "id `{id}` must be valid, got findings: {:?}",
                codes(&g)
            );
        }
    }

    #[test]
    fn test_build_uppercase_id_emits_invalid_id() {
        let a = ast(vec![leaf("App.Api")], vec![]);
        let g = build(&a);
        assert!(
            codes(&g).contains(&"CAIRN_INTEGRITY_INVALID_ID"),
            "uppercase id must emit CAIRN_INTEGRITY_INVALID_ID: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn test_build_underscore_in_id_emits_invalid_id() {
        // Underscores are not in the allowed set (only '.', '-', lower, digit).
        let a = ast(vec![leaf("app_api")], vec![]);
        let g = build(&a);
        assert!(
            codes(&g).contains(&"CAIRN_INTEGRITY_INVALID_ID"),
            "underscore in id must emit CAIRN_INTEGRITY_INVALID_ID: {:?}",
            codes(&g)
        );
    }

    // ── node state ────────────────────────────────────────────────────────────

    #[test]
    fn test_build_node_with_no_paths_is_synced() {
        // A node with no path declarations is considered Synced (not Ghost).
        let a = ast(vec![leaf("app.api")], vec![]);
        let g = build(&a);
        assert_eq!(g.nodes["app.api"].state, NodeState::Synced);
    }

    #[test]
    fn test_build_node_with_nonexistent_path_is_ghost() {
        // Path declared but does not exist on disk → Ghost.
        let a = ast(vec![leaf_with_path("app.api", "src/api")], vec![]);
        let g = build(&a);
        assert_eq!(
            g.nodes["app.api"].state,
            NodeState::Ghost,
            "non-existent declared path must produce Ghost state"
        );
    }

    // ── path tie ──────────────────────────────────────────────────────────────

    #[test]
    fn test_build_path_tie_emits_integrity_error() {
        // Two leaf nodes (owns_files=true) declaring the same path.
        let a = ast(
            vec![
                leaf_with_path("app.api", "src/api"),
                leaf_with_path("app.db", "src/api"),
            ],
            vec![],
        );
        let g = build(&a);
        assert!(
            codes(&g).contains(&"CAIRN_INTEGRITY_PATH_TIE"),
            "two nodes owning same path must emit CAIRN_INTEGRITY_PATH_TIE: {:?}",
            codes(&g)
        );
    }

    // ── external findings ─────────────────────────────────────────────────────

    #[test]
    fn test_build_external_findings_are_preserved() {
        let external = Finding {
            code: "EXTERNAL_FINDING".to_owned(),
            severity: FindingSeverity::Warning,
            message: "pre-existing".to_owned(),
            node: None,
            target: None,
            path: None,
        };
        let a = ast(vec![leaf("app.api")], vec![]);
        let g = build_graph(
            &a,
            Path::new("/nonexistent"),
            &ContractSet::default(),
            &mut BTreeMap::new(),
            vec![external],
        );
        assert!(
            codes(&g).contains(&"EXTERNAL_FINDING"),
            "external findings must be preserved"
        );
    }
}
