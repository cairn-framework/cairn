//! Test-coverage integrity check.
//!
//! Emits [`CAIRN_TEST_COVERAGE_MISSING`] for synced modules whose reconciled
//! Rust source contains no `#[cfg(test)]` marker. Ghost nodes (no code) are
//! exempt, mirroring the contract-pointer rule. A node tagged
//! `no-test-coverage` is skipped. The finding is a Warning: advisory by
//! default, gated via the existing `cairn scan --strict` exit-code promotion.
//! See `design.md` in `meta/changes/cairn-test-coverage-gate` for rationale.

use std::{fs, path::Path};

use super::graph::{Finding, FindingSeverity, Graph, NodeState};

/// Emits `CAIRN_TEST_COVERAGE_MISSING` for synced modules whose reconciled
/// Rust source contains no `#[cfg(test)]` marker.
pub(crate) fn validate_test_coverage(graph: &mut Graph, root: &Path) {
    const EXEMPT_TAG: &str = "no-test-coverage";
    const MARKER: &str = "#[cfg(test)]";
    for node in graph.nodes.values() {
        // Ghost nodes (declared, no code) owe no tests.
        if node.state == NodeState::Ghost {
            continue;
        }
        // Rust-first: only `.rs` files carry the `#[cfg(test)]` convention
        // today. A node with no reconciled `.rs` files is out of scope.
        let Some(first_rust) = node.files.iter().find(|f| is_rust_source(f)) else {
            continue;
        };
        if node.tags.iter().any(|tag| tag == EXEMPT_TAG) {
            continue;
        }
        let has_tests = node.files.iter().filter(|f| is_rust_source(f)).any(|file| {
            fs::read_to_string(root.join(file))
                .map(|content| content.contains(MARKER))
                .unwrap_or(false)
        });
        if !has_tests {
            graph.findings.push(Finding {
                code: "CAIRN_TEST_COVERAGE_MISSING".to_owned(),
                severity: FindingSeverity::Warning,
                message: format!(
                    "synced module `{}` has no #[cfg(test)] coverage in its reconciled source",
                    node.id
                ),
                node: Some(node.id.clone()),
                target: None,
                path: Some(first_rust.clone()),
            });
        }
    }
}

/// True when `path` has a `.rs` extension (case-insensitive).
fn is_rust_source(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        artefacts::contract::ContractSet,
        blueprint::{Ast, Edge, Node, NodeKind, Span},
        map::{build_graph, graph::FindingSeverity},
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

    fn leaf_with_path(id: &str, path: &str) -> Node {
        let mut n = leaf(id);
        n.paths = vec![path.to_owned()];
        n
    }

    fn ast(nodes: Vec<Node>, edges: Vec<Edge>) -> Ast {
        Ast { nodes, edges }
    }

    fn codes(g: &Graph) -> Vec<&str> {
        g.findings.iter().map(|f| f.code.as_str()).collect()
    }

    fn build_with_files(
        root: &std::path::Path,
        a: &Ast,
        claimed: &mut BTreeMap<String, Vec<String>>,
    ) -> Graph {
        build_graph(a, root, &ContractSet::default(), claimed, Vec::new())
    }

    fn build(a: &Ast) -> Graph {
        let dir = tempfile::tempdir().unwrap();
        build_graph(
            a,
            dir.path(),
            &ContractSet::default(),
            &mut BTreeMap::new(),
            Vec::new(),
        )
    }

    #[test]
    fn test_coverage_present_no_finding() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("mod.rs"),
            "fn foo() {}\n#[cfg(test)]\nmod tests {\n    #[test] fn t() {}\n}\n",
        )
        .unwrap();
        let a = ast(vec![leaf("app.api")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["mod.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_TEST_COVERAGE_MISSING"),
            "module with #[cfg(test)] must not be flagged: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn test_coverage_missing_synced_emits_warning() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("mod.rs"), "fn foo() {}\n").unwrap();
        let a = ast(vec![leaf("app.api")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["mod.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        let flagged = codes(&g);
        assert!(
            flagged.contains(&"CAIRN_TEST_COVERAGE_MISSING"),
            "synced module without #[cfg(test)] must be flagged: {flagged:?}"
        );
        let finding = g
            .findings
            .iter()
            .find(|f| f.code == "CAIRN_TEST_COVERAGE_MISSING")
            .unwrap();
        assert_eq!(finding.severity, FindingSeverity::Warning);
        assert_eq!(finding.node.as_deref(), Some("app.api"));
    }

    #[test]
    fn test_coverage_ghost_node_exempt() {
        // Path declared but absent + no claimed files => Ghost state => exempt.
        let a = ast(vec![leaf_with_path("app.api", "src/none.rs")], vec![]);
        let g = build(&a);
        assert!(
            !codes(&g).contains(&"CAIRN_TEST_COVERAGE_MISSING"),
            "ghost node must be exempt: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn test_coverage_tagged_module_exempt() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("mod.rs"), "fn foo() {}\n").unwrap();
        let mut n = leaf("app.api");
        n.tags = vec!["no-test-coverage".to_owned()];
        let a = ast(vec![n], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.api".to_owned(), vec!["mod.rs".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_TEST_COVERAGE_MISSING"),
            "no-test-coverage tag must exempt the node: {:?}",
            codes(&g)
        );
    }

    #[test]
    fn test_coverage_non_rust_files_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("style.css"), "body { }").unwrap();
        let a = ast(vec![leaf("app.ui")], vec![]);
        let mut claimed = BTreeMap::new();
        claimed.insert("app.ui".to_owned(), vec!["style.css".to_owned()]);
        let g = build_with_files(root, &a, &mut claimed);
        assert!(
            !codes(&g).contains(&"CAIRN_TEST_COVERAGE_MISSING"),
            "non-Rust module must be skipped (Rust-first): {:?}",
            codes(&g)
        );
    }
}
