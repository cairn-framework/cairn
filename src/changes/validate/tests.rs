//! Tests for change validation against the current graph.

// Reason: test module covers many independent validation branches
#![allow(clippy::too_many_lines)]
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::blueprint::{NodeKind, Span, ast::Edge as BpEdge};
use crate::map::graph::{EdgeRef, Graph, NodeRecord, NodeState};

use super::*;

// ── Fixtures ──────────────────────────────────────────────────────────────

fn empty_graph() -> Graph {
    Graph {
        nodes: BTreeMap::new(),
        names: BTreeMap::new(),
        outbound: BTreeMap::new(),
        inbound: BTreeMap::new(),
        findings: Vec::new(),
    }
}

fn graph_with_nodes(ids: &[&str]) -> Graph {
    let mut g = empty_graph();
    for &id in ids {
        g.nodes.insert(
            id.to_owned(),
            NodeRecord {
                kind: NodeKind::Module,
                id: id.to_owned(),
                name: id.to_owned(),
                description: String::new(),
                tags: Vec::new(),
                parent: None,
                children: Vec::new(),
                paths: Vec::new(),
                owns_files: false,
                contracts: Vec::new(),
                state: NodeState::Synced,
                files: Vec::new(),
                span: Span::point("cairn.blueprint", 1, 1),
            },
        );
    }
    g
}

fn graph_with_edge(from: &str, to: &str, description: &str) -> Graph {
    let mut g = graph_with_nodes(&[from, to]);
    g.outbound
        .entry(from.to_owned())
        .or_default()
        .push(EdgeRef {
            from: from.to_owned(),
            to: to.to_owned(),
            description: description.to_owned(),
        });
    g
}

fn bp_edge(from: &str, to: &str, description: &str) -> BpEdge {
    BpEdge {
        from: from.to_owned(),
        to: to.to_owned(),
        description: description.to_owned(),
        span: Span::point("cairn.blueprint", 1, 1),
    }
}

fn bp_node(id: &str) -> Node {
    Node {
        kind: NodeKind::Module,
        name: id.to_owned(),
        description: String::new(),
        id: id.to_owned(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: Vec::new(),
        span: Span::point("cairn.blueprint", 1, 1),
    }
}

fn empty_change() -> Change {
    Change {
        id: "test".to_owned(),
        path: PathBuf::from("/tmp/test"),
        title: "Test".to_owned(),
        proposal: String::new(),
        design: None,
        delta: BlueprintDelta::default(),
        artefacts: Vec::new(),
        findings: Vec::new(),
    }
}

fn artefact(operation: ChangeOperation, target: &str, content: &str) -> ArtefactOperation {
    ArtefactOperation {
        operation,
        change_path: PathBuf::from("change/design.md"),
        target_path: PathBuf::from(target),
        renamed_from: None,
        content: content.to_owned(),
    }
}

// ── Node operations ───────────────────────────────────────────────────────

#[test]
fn test_valid_change_has_no_errors() {
    let mut change = empty_change();
    change.delta.added_nodes = vec![bp_node("app.new")];
    let graph = graph_with_nodes(&["app.existing"]);
    assert!(validate_change(&change, &graph).is_empty());
}

#[test]
fn test_added_node_duplicate_in_delta() {
    let mut change = empty_change();
    change.delta.added_nodes = vec![bp_node("app.x"), bp_node("app.x")];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("duplicate add")),
        "duplicate add in delta must produce error; got: {errors:?}"
    );
}

#[test]
fn test_added_node_already_in_graph() {
    let mut change = empty_change();
    change.delta.added_nodes = vec![bp_node("app.existing")];
    let graph = graph_with_nodes(&["app.existing"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("duplicate add")),
        "adding a node that exists in graph must error; got: {errors:?}"
    );
}

#[test]
fn test_renamed_node_from_nonexistent() {
    let mut change = empty_change();
    change.delta.renamed_nodes = vec![Rename {
        from: "app.ghost".to_owned(),
        to: "app.new".to_owned(),
    }];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("does not exist")),
        "rename of nonexistent node must error; got: {errors:?}"
    );
}

#[test]
fn test_renamed_node_to_existing_id() {
    let mut change = empty_change();
    change.delta.renamed_nodes = vec![Rename {
        from: "app.a".to_owned(),
        to: "app.b".to_owned(),
    }];
    let graph = graph_with_nodes(&["app.a", "app.b"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("already exists")),
        "rename target that already exists must error; got: {errors:?}"
    );
}

#[test]
fn test_renamed_node_to_concurrently_added_id() {
    let mut change = empty_change();
    change.delta.added_nodes = vec![bp_node("app.fresh")];
    change.delta.renamed_nodes = vec![Rename {
        from: "app.old".to_owned(),
        to: "app.fresh".to_owned(),
    }];
    let graph = graph_with_nodes(&["app.old"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("already exists")),
        "rename to an added ID must error; got: {errors:?}"
    );
}

#[test]
fn test_removed_node_nonexistent() {
    let mut change = empty_change();
    change.delta.removed_nodes = vec!["app.ghost".to_owned()];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("does not exist")),
        "removing nonexistent node must error; got: {errors:?}"
    );
}

#[test]
fn test_modified_node_nonexistent() {
    let mut change = empty_change();
    change.delta.modified_nodes = vec![bp_node("app.ghost")];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("does not exist")),
        "modifying nonexistent node must error; got: {errors:?}"
    );
}

#[test]
fn test_conflicting_operations_on_same_node() {
    let mut change = empty_change();
    change.delta.renamed_nodes = vec![Rename {
        from: "app.a".to_owned(),
        to: "app.b".to_owned(),
    }];
    // Also remove app.a — conflicts with the rename.
    change.delta.removed_nodes = vec!["app.a".to_owned()];
    let graph = graph_with_nodes(&["app.a"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("conflicting operations")),
        "same node touched twice must produce conflicting-operations error; got: {errors:?}"
    );
}

// ── Edge operations ───────────────────────────────────────────────────────

#[test]
fn test_added_edge_missing_endpoint_errors() {
    let mut change = empty_change();
    change.delta.added_edges = vec![bp_edge("app.a", "app.missing", "uses")];
    let graph = graph_with_nodes(&["app.a"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("missing endpoint")),
        "edge to nonexistent node must error; got: {errors:?}"
    );
}

#[test]
fn test_added_edge_with_new_node_succeeds() {
    // An edge can reference a node being added in the same change.
    let mut change = empty_change();
    change.delta.added_nodes = vec![bp_node("app.new")];
    change.delta.added_edges = vec![bp_edge("app.existing", "app.new", "uses")];
    let graph = graph_with_nodes(&["app.existing"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().all(|e| !e.contains("missing endpoint")),
        "edge to newly-added node must succeed; got: {errors:?}"
    );
}

#[test]
fn test_removed_edge_nonexistent_errors() {
    let mut change = empty_change();
    change.delta.removed_edges = vec![bp_edge("app.a", "app.b", "uses")];
    let graph = graph_with_nodes(&["app.a", "app.b"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("does not exist")),
        "removing nonexistent edge must error; got: {errors:?}"
    );
}

#[test]
fn test_removed_edge_existing_succeeds() {
    let mut change = empty_change();
    change.delta.removed_edges = vec![bp_edge("app.a", "app.b", "uses")];
    let graph = graph_with_edge("app.a", "app.b", "uses");
    let errors = validate_change(&change, &graph);
    assert!(
        errors.is_empty(),
        "removing an existing edge must produce no errors; got: {errors:?}"
    );
}

// ── Artefact operations ───────────────────────────────────────────────────

#[test]
fn test_artefact_duplicate_target_errors() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("design.md");
    std::fs::write(&target, "existing").unwrap();
    let mut change = empty_change();
    change.artefacts = vec![
        ArtefactOperation {
            operation: ChangeOperation::Modified,
            change_path: PathBuf::from("a.md"),
            target_path: target.clone(),
            renamed_from: None,
            content: String::new(),
        },
        ArtefactOperation {
            operation: ChangeOperation::Modified,
            change_path: PathBuf::from("b.md"),
            target_path: target,
            renamed_from: None,
            content: String::new(),
        },
    ];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("duplicate operations")),
        "duplicate artefact target must error; got: {errors:?}"
    );
}

#[test]
fn test_renamed_artefact_missing_renamed_from_errors() {
    let mut change = empty_change();
    change.artefacts = vec![artefact(ChangeOperation::Renamed, "/tmp/x.md", "")];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("missing renamed_from")),
        "renamed artefact without renamed_from must error; got: {errors:?}"
    );
}

// ── Artefact node reference validation ────────────────────────────────────

#[test]
fn test_artefact_refs_unknown_node_errors() {
    let mut change = empty_change();
    change.artefacts = vec![artefact(
        ChangeOperation::Added,
        "/tmp/new.md",
        "---\nnode: app.ghost\n---\n",
    )];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("unknown node")),
        "artefact referencing unknown node must error; got: {errors:?}"
    );
}

#[test]
fn test_artefact_refs_known_node_succeeds() {
    let mut change = empty_change();
    change.artefacts = vec![artefact(
        ChangeOperation::Added,
        "/tmp/new.md",
        "---\nnode: app.real\n---\n",
    )];
    let graph = graph_with_nodes(&["app.real"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.is_empty(),
        "artefact referencing known node must produce no errors; got: {errors:?}"
    );
}

#[test]
fn test_artefact_refs_nodes_list_unknown_errors() {
    let mut change = empty_change();
    change.artefacts = vec![artefact(
        ChangeOperation::Added,
        "/tmp/new.md",
        "---\nnodes:\n- app.real\n- app.ghost\n---\n",
    )];
    let graph = graph_with_nodes(&["app.real"]);
    let errors = validate_change(&change, &graph);
    assert!(
        errors.iter().any(|e| e.contains("unknown node")),
        "nodes list with unknown entry must error; got: {errors:?}"
    );
}

#[test]
fn test_artefact_refs_added_node_in_same_change_succeeds() {
    // An added contract may reference a node the same change adds (the brownfield
    // round-trip case). It must validate against the post-delta graph.
    let mut change = empty_change();
    change.delta.added_nodes = vec![bp_node("app.new")];
    change.artefacts = vec![artefact(
        ChangeOperation::Added,
        "/tmp/new.md",
        "---\noperation: added\nnode: app.new\n---\n",
    )];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.is_empty(),
        "artefact referencing a same-change-added node must not error; got: {errors:?}"
    );
}

#[test]
fn test_artefact_refs_removed_node_still_resolves() {
    // Removing a node and an artefact that references it must not raise an
    // unknown-node error: the reference resolves against the pre-delta graph.
    let mut change = empty_change();
    change.delta.removed_nodes = vec!["app.old".to_owned()];
    change.artefacts = vec![artefact(
        ChangeOperation::Removed,
        "/tmp/old.md",
        "---\noperation: removed\nnode: app.old\n---\n",
    )];
    let errors = validate_change(&change, &graph_with_nodes(&["app.old"]));
    assert!(
        !errors.iter().any(|e| e.contains("unknown node")),
        "reference to a removed node must still resolve; got: {errors:?}"
    );
}

// ── Pre-existing findings propagated ──────────────────────────────────────

#[test]
fn test_existing_findings_are_propagated() {
    let mut change = empty_change();
    change.findings = vec!["pre-existing parse error".to_owned()];
    let errors = validate_change(&change, &empty_graph());
    assert!(
        errors.iter().any(|e| e.contains("pre-existing")),
        "load-time findings must appear in validate output; got: {errors:?}"
    );
}
