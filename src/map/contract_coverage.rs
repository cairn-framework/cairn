//! Contract-coverage integrity check.
//!
//! Implements the spec.md:318 integrity rule: every leaf node should have a
//! contract. Emits [`CAIRN_CONTRACT_LEAF_UNCOVERED`] (registry code CK003) for a
//! synced leaf node (no children) that owns code yet declares no contract
//! pointer. Container/parent nodes are exempt (the rule is per-leaf), ghost
//! nodes owe no contract until they carry code, a node that already declares a
//! contract pointer is handled by [`validate_contracts`](super::build), and a
//! node tagged `no-contract` is skipped.
//!
//! The finding is a Warning: advisory by default, gated via the existing
//! `cairn scan --strict` exit-code promotion, mirroring the test-coverage gate.

use super::graph::{Finding, FindingSeverity, Graph, NodeState};

/// Emits `CAIRN_CONTRACT_LEAF_UNCOVERED` for synced leaf nodes that own code
/// but declare no contract pointer.
pub(crate) fn validate_contract_coverage(graph: &mut Graph) {
    const EXEMPT_TAG: &str = "no-contract";
    for node in graph.nodes.values() {
        // The rule is per leaf: container nodes describe structure, not a unit
        // of code with its own contract.
        if !node.children.is_empty() {
            continue;
        }
        // Ghost nodes (declared, no code) owe no contract until they sync.
        if node.state != NodeState::Synced {
            continue;
        }
        // A leaf with no claimed files is a pure declaration (e.g. an actor),
        // not a code unit that owes a contract.
        if node.files.is_empty() {
            continue;
        }
        // A declared-but-missing contract pointer is `validate_contracts`'s job.
        if !node.contracts.is_empty() {
            continue;
        }
        if node.tags.iter().any(|tag| tag == EXEMPT_TAG) {
            continue;
        }
        graph.findings.push(Finding {
            code: "CAIRN_CONTRACT_LEAF_UNCOVERED".to_owned(),
            severity: FindingSeverity::Warning,
            message: format!("leaf node `{}` declares no contract", node.id),
            node: Some(node.id.clone()),
            target: None,
            path: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        artefacts::contract::ContractSet,
        blueprint::{Ast, Edge, Node, NodeKind, Span},
        map::build_graph,
    };

    use super::*;

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

    fn ast(nodes: Vec<Node>, edges: Vec<Edge>) -> Ast {
        Ast { nodes, edges }
    }

    fn codes(g: &Graph) -> Vec<&str> {
        g.findings.iter().map(|f| f.code.as_str()).collect()
    }

    /// Builds a graph after writing the given `(path, contents)` files under a
    /// temp root and claiming them for the matching node id, so leaves reconcile
    /// to `Synced` with files.
    fn build_with_file(node_id: &str, path: &str, a: &Ast) -> Graph {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(path), "// code").unwrap();
        let mut claimed = BTreeMap::new();
        claimed.insert(node_id.to_owned(), vec![path.to_owned()]);
        build_graph(
            a,
            dir.path(),
            &ContractSet::default(),
            &mut claimed,
            Vec::new(),
        )
    }

    #[test]
    fn uncovered_synced_leaf_emits_warning() {
        let mut node = leaf("app.api");
        node.paths = vec!["api.rs".to_owned()];
        let g = build_with_file("app.api", "api.rs", &ast(vec![node], vec![]));
        let finding = g
            .findings
            .iter()
            .find(|f| f.code == "CAIRN_CONTRACT_LEAF_UNCOVERED")
            .expect("uncovered synced leaf must warn");
        assert_eq!(finding.severity, FindingSeverity::Warning);
        assert_eq!(finding.node.as_deref(), Some("app.api"));
    }

    #[test]
    fn covered_leaf_no_warning() {
        let mut node = leaf("app.api");
        node.paths = vec!["api.rs".to_owned()];
        node.contracts = vec!["contracts/app.api.md".to_owned()];
        let g = build_with_file("app.api", "api.rs", &ast(vec![node], vec![]));
        assert!(
            !codes(&g).contains(&"CAIRN_CONTRACT_LEAF_UNCOVERED"),
            "leaf declaring a contract must not warn: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn ghost_leaf_exempt() {
        // Declares a path that does not exist and claims no files -> Ghost.
        let mut node = leaf("app.api");
        node.paths = vec!["missing.rs".to_owned()];
        let dir = tempfile::tempdir().unwrap();
        let g = build_graph(
            &ast(vec![node], vec![]),
            dir.path(),
            &ContractSet::default(),
            &mut BTreeMap::new(),
            Vec::new(),
        );
        assert!(
            !codes(&g).contains(&"CAIRN_CONTRACT_LEAF_UNCOVERED"),
            "ghost leaf must be exempt: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn parent_node_exempt() {
        // The parent owns code AND has a child: only the `children` guard keeps
        // it exempt, so this regresses loudly if that guard is dropped.
        let mut parent = leaf("app");
        parent.paths = vec!["app.rs".to_owned()];
        parent.owns_files = true;
        parent.children = vec![leaf("app.api")];
        let g = build_with_file("app", "app.rs", &ast(vec![parent], vec![]));
        assert_eq!(
            g.resolve("app").unwrap().files,
            vec!["app.rs"],
            "test must exercise a parent that owns files"
        );
        let parent_warned = g
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_CONTRACT_LEAF_UNCOVERED" && f.node.as_deref() == Some("app"));
        assert!(
            !parent_warned,
            "container node must be exempt: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn tagged_node_exempt() {
        let mut node = leaf("app.api");
        node.paths = vec!["api.rs".to_owned()];
        node.tags = vec!["no-contract".to_owned()];
        let g = build_with_file("app.api", "api.rs", &ast(vec![node], vec![]));
        assert!(
            !codes(&g).contains(&"CAIRN_CONTRACT_LEAF_UNCOVERED"),
            "no-contract tag must exempt the node: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn fileless_leaf_exempt() {
        // A synced leaf with no claimed files (pure declaration) owes no contract.
        let node = leaf("app.actor");
        let g = build_graph(
            &ast(vec![node], vec![]),
            tempfile::tempdir().unwrap().path(),
            &ContractSet::default(),
            &mut BTreeMap::new(),
            Vec::new(),
        );
        assert!(
            !codes(&g).contains(&"CAIRN_CONTRACT_LEAF_UNCOVERED"),
            "fileless leaf must be exempt: {:?}",
            codes(&g)
        );
    }
}
