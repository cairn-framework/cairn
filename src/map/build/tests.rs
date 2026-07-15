//! Unit tests for map graph builder.

use super::*;
use crate::artefacts::contract::ContractSet;
use crate::blueprint::{Ast, Edge, Node, NodeKind, Span};
use crate::map::query;
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

fn build_at(ast: &Ast, root: &Path, claimed: &mut BTreeMap<String, Vec<String>>) -> Graph {
    build_graph(ast, root, &ContractSet::default(), claimed, Vec::new())
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

#[test]
fn test_build_existing_empty_path_is_ghost() {
    // Declared path exists but owns zero source files → Ghost (gh:#238).
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/empty")).unwrap();
    let a = ast(vec![leaf_with_path("app.empty", "src/empty")], vec![]);
    let g = build_at(&a, dir.path(), &mut BTreeMap::new());
    assert_eq!(
        g.nodes["app.empty"].state,
        NodeState::Ghost,
        "empty declared directory must produce Ghost, got {:?}",
        g.nodes["app.empty"].state
    );
    assert!(g.nodes["app.empty"].files.is_empty());
}

#[test]
fn test_build_existing_path_with_claimed_files_is_synced() {
    // A node that owns real source files stays Synced.
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/real")).unwrap();
    let a = ast(vec![leaf_with_path("app.real", "src/real")], vec![]);
    let mut claimed = BTreeMap::new();
    claimed.insert("app.real".to_owned(), vec!["src/real/lib.rs".to_owned()]);
    let g = build_at(&a, dir.path(), &mut claimed);
    assert_eq!(
        g.nodes["app.real"].state,
        NodeState::Synced,
        "node with claimed source files must stay Synced"
    );
    assert_eq!(
        g.nodes["app.real"].files,
        vec!["src/real/lib.rs".to_owned()]
    );
}

#[test]
fn test_build_container_without_own_files_stays_synced() {
    // Container path exists; descendants own the files. Do not flag Ghost.
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/app/leaf")).unwrap();
    let parent = Node {
        paths: vec!["src/app".to_owned()],
        children: vec![leaf_with_path("app.leaf", "src/app/leaf")],
        ..leaf("app")
    };
    let a = ast(vec![parent], vec![]);
    let mut claimed = BTreeMap::new();
    claimed.insert(
        "app.leaf".to_owned(),
        vec!["src/app/leaf/mod.rs".to_owned()],
    );
    let g = build_at(&a, dir.path(), &mut claimed);
    assert_eq!(
        g.nodes["app"].state,
        NodeState::Synced,
        "container whose children own files must not be Ghost"
    );
    assert_eq!(g.nodes["app.leaf"].state, NodeState::Synced);
}

#[test]
fn test_build_container_empty_child_is_ghost() {
    // Parent container stays Synced; empty leaf child is Ghost.
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/app/empty")).unwrap();
    let parent = Node {
        paths: vec!["src/app".to_owned()],
        children: vec![leaf_with_path("app.empty", "src/app/empty")],
        ..leaf("app")
    };
    let a = ast(vec![parent], vec![]);
    let g = build_at(&a, dir.path(), &mut BTreeMap::new());
    assert_eq!(
        g.nodes["app"].state,
        NodeState::Synced,
        "container not owning files stays Synced"
    );
    assert_eq!(
        g.nodes["app.empty"].state,
        NodeState::Ghost,
        "empty leaf under a container must be Ghost"
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
        deferred_by: None,
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

#[test]
fn test_empty_path_ghost_surfaces_in_get_and_frontier() {
    // Projection contract: empty scaffolding is Ghost, so `get` reports Ghost
    // and `frontier` lists the node as ready (gh:#238).
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/empty")).unwrap();
    let a = ast(vec![leaf_with_path("app.empty", "src/empty")], vec![]);
    let g = build_at(&a, dir.path(), &mut BTreeMap::new());
    assert_eq!(g.nodes["app.empty"].state, NodeState::Ghost);

    let got = query::get(&g, "app.empty").expect("node must resolve");
    assert_eq!(
        got.node.state,
        NodeState::Ghost,
        "get projection must report Ghost for empty scaffolding"
    );
    assert!(got.node.files.is_empty());

    let frontier = query::frontier(&g).expect("acyclic graph");
    assert!(
        frontier.ready.iter().any(|e| e.node == "app.empty"),
        "frontier must include empty-path Ghost as ready; ready={:?} blocked={:?}",
        frontier.ready,
        frontier.blocked
    );
    assert!(
        frontier.blocked.iter().all(|e| e.node != "app.empty"),
        "empty-path Ghost must not be blocked without deps"
    );
}

#[test]
fn test_empty_path_ghost_counted_in_health_modules() {
    // health summary.modules is derived from NodeState; empty scaffolding must
    // contribute to ghost, not synced (gh:#238).
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/empty")).unwrap();
    std::fs::create_dir_all(dir.path().join("src/real")).unwrap();
    let a = ast(
        vec![
            leaf_with_path("app.empty", "src/empty"),
            leaf_with_path("app.real", "src/real"),
        ],
        vec![],
    );
    let mut claimed = BTreeMap::new();
    claimed.insert("app.real".to_owned(), vec!["src/real/lib.rs".to_owned()]);
    let g = build_at(&a, dir.path(), &mut claimed);

    let mut synced = 0usize;
    let mut ghost = 0usize;
    let mut orphaned = 0usize;
    for node in g.nodes.values() {
        match node.state {
            NodeState::Synced => synced += 1,
            NodeState::Ghost => ghost += 1,
            NodeState::Orphaned => orphaned += 1,
        }
    }
    assert_eq!(ghost, 1, "empty scaffolding must count as ghost");
    assert_eq!(synced, 1, "node with claimed files must count as synced");
    assert_eq!(orphaned, 0);
}
