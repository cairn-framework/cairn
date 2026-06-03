// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

/// Validates change references against current truth.
#[must_use]
pub fn validate_change(change: &Change, graph: &Graph) -> Vec<String> {
    let mut errors = change.findings.clone();
    let mut existing_nodes = graph.nodes.keys().cloned().collect::<BTreeSet<_>>();
    let mut added_nodes = BTreeSet::new();
    let mut touched_nodes = BTreeSet::new();
    for node in &change.delta.added_nodes {
        if !added_nodes.insert(node.id.clone()) || existing_nodes.contains(&node.id) {
            errors.push(format!("node `{}` has duplicate add operation", node.id));
        }
    }
    for rename in &change.delta.renamed_nodes {
        if !existing_nodes.contains(&rename.from) {
            errors.push(format!("renamed node `{}` does not exist", rename.from));
        }
        if existing_nodes.contains(&rename.to) || added_nodes.contains(&rename.to) {
            errors.push(format!(
                "renamed node target `{}` already exists",
                rename.to
            ));
        }
        mark_node_touch(&mut touched_nodes, &rename.from, &mut errors);
        existing_nodes.remove(&rename.from);
        existing_nodes.insert(rename.to.clone());
    }
    for id in &change.delta.removed_nodes {
        if !existing_nodes.contains(id) {
            errors.push(format!("removed node `{id}` does not exist"));
        }
        mark_node_touch(&mut touched_nodes, id, &mut errors);
        existing_nodes.remove(id);
    }
    for node in &change.delta.modified_nodes {
        if !existing_nodes.contains(&node.id) {
            errors.push(format!("modified node `{}` does not exist", node.id));
        }
        mark_node_touch(&mut touched_nodes, &node.id, &mut errors);
    }
    existing_nodes.extend(added_nodes);
    validate_edges(&change.delta, &existing_nodes, graph, &mut errors);
    validate_artefacts(&change.artefacts, graph, &mut errors);
    errors
}

pub(super) fn validate_edges(
    delta: &BlueprintDelta,
    available_nodes: &BTreeSet<String>,
    graph: &Graph,
    errors: &mut Vec<String>,
) {
    for edge in delta
        .added_edges
        .iter()
        .chain(delta.modified_edges.iter())
        .chain(delta.renamed_edges.iter().map(|rename| &rename.to))
    {
        if !available_nodes.contains(&edge.from) || !available_nodes.contains(&edge.to) {
            errors.push(format!(
                "edge `{}` -> `{}` references missing endpoint",
                edge.from, edge.to
            ));
        }
    }
    for edge in delta
        .removed_edges
        .iter()
        .chain(delta.modified_edges.iter())
        .chain(delta.renamed_edges.iter().map(|rename| &rename.from))
    {
        if !graph_edge_exists(graph, edge) {
            errors.push(format!(
                "edge `{}` -> `{}` ({}) does not exist",
                edge.from, edge.to, edge.description
            ));
        }
    }
}

pub(super) fn graph_edge_exists(graph: &Graph, edge: &Edge) -> bool {
    graph.outbound.get(&edge.from).is_some_and(|edges| {
        edges
            .iter()
            .any(|existing| existing.to == edge.to && existing.description == edge.description)
    })
}

pub(super) fn mark_node_touch(touched: &mut BTreeSet<String>, id: &str, errors: &mut Vec<String>) {
    if !touched.insert(id.to_owned()) {
        errors.push(format!("node `{id}` has conflicting operations"));
    }
}

pub(super) fn validate_artefacts(
    artefacts: &[ArtefactOperation],
    graph: &Graph,
    errors: &mut Vec<String>,
) {
    let mut targets = BTreeSet::<PathBuf>::new();
    for artefact in artefacts {
        if !targets.insert(artefact.target_path.clone()) {
            errors.push(format!(
                "artefact `{}` has duplicate operations",
                artefact.target_path.display()
            ));
        }
        match artefact.operation {
            ChangeOperation::Added => {
                if artefact.target_path.exists() {
                    errors.push(format!(
                        "added artefact target `{}` already exists",
                        artefact.target_path.display()
                    ));
                }
            }
            ChangeOperation::Modified | ChangeOperation::Removed => {
                if !artefact.target_path.exists() {
                    errors.push(format!(
                        "{:?} artefact target `{}` does not exist",
                        artefact.operation,
                        artefact.target_path.display()
                    ));
                }
            }
            ChangeOperation::Renamed => {
                if artefact.renamed_from.is_none() {
                    errors.push(format!(
                        "renamed artefact `{}` is missing renamed_from",
                        artefact.change_path.display()
                    ));
                }
                if let Some(source) = &artefact.renamed_from
                    && !source.exists()
                {
                    errors.push(format!(
                        "renamed artefact source `{}` does not exist",
                        source.display()
                    ));
                }
            }
        }
        validate_artefact_refs(artefact, graph, errors);
    }
}

pub(super) fn validate_artefact_refs(
    artefact: &ArtefactOperation,
    graph: &Graph,
    errors: &mut Vec<String>,
) {
    let parsed = frontmatter::parse(&artefact.content);
    for key in ["node", "nodes"] {
        let ids = if key == "node" {
            parsed
                .values
                .get(key)
                .map(|value| vec![value.clone()])
                .unwrap_or_default()
        } else {
            parsed.lists.get(key).cloned().unwrap_or_default()
        };
        for id in ids {
            if !graph.nodes.contains_key(&id) {
                errors.push(format!(
                    "artefact `{}` references unknown node `{id}`",
                    artefact.change_path.display()
                ));
            }
        }
    }
}

#[cfg(test)]
// Reason: test module for validate_change covers many independent branches.
mod tests {
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
}
