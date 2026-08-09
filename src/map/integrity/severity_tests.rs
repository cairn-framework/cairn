//! Regression tests for provenance-aware cycle severity.
use super::{cycle_findings, topological_order};
use crate::blueprint::{EdgeProvenance, NodeKind, Span};
use crate::map::graph::{EdgeRef, FindingSeverity, Graph, NodeRecord, NodeState};
use std::collections::BTreeMap;

fn bare_node(id: &str) -> NodeRecord {
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
        symbols: Vec::new(),
        span: Span::point("test", 1, 1),
    }
}

fn make_graph_with_provenance(ids: &[&str], edges: &[(&str, &str, EdgeProvenance)]) -> Graph {
    let nodes = ids
        .iter()
        .map(|id| ((*id).to_owned(), bare_node(id)))
        .collect();
    let mut outbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
    let mut inbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
    for (from, to, provenance) in edges {
        let edge = EdgeRef {
            from: (*from).to_owned(),
            to: (*to).to_owned(),
            description: "dep".to_owned(),
            provenance: *provenance,
        };
        outbound
            .entry((*from).to_owned())
            .or_default()
            .push(edge.clone());
        inbound.entry((*to).to_owned()).or_default().push(edge);
    }
    Graph {
        nodes,
        names: BTreeMap::new(),
        outbound,
        inbound,
        findings: Vec::new(),
    }
}

fn with_containment(mut graph: Graph, links: &[(&str, &str)]) -> Graph {
    for (parent, child) in links {
        graph
            .nodes
            .get_mut(*parent)
            .unwrap()
            .children
            .push((*child).to_owned());
        graph.nodes.get_mut(*child).unwrap().parent = Some((*parent).to_owned());
    }
    graph
}

#[test]
fn all_inferred_component_is_advisory() {
    let graph = make_graph_with_provenance(
        &["a", "b"],
        &[
            ("a", "b", EdgeProvenance::Inferred),
            ("b", "a", EdgeProvenance::Inferred),
        ],
    );
    let findings = cycle_findings(&graph);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, FindingSeverity::Info);
}

#[test]
fn mixed_components_report_both_severities() {
    let graph = make_graph_with_provenance(
        &["a", "b", "c", "d"],
        &[
            ("a", "b", EdgeProvenance::Inferred),
            ("b", "a", EdgeProvenance::Inferred),
            ("c", "d", EdgeProvenance::HandDeclared),
            ("d", "c", EdgeProvenance::HandDeclared),
        ],
    );
    let findings = cycle_findings(&graph);
    assert_eq!(findings.len(), 2, "every cyclic SCC must be reported");
    assert_eq!(
        findings
            .iter()
            .map(|finding| finding.severity)
            .collect::<Vec<_>>(),
        vec![FindingSeverity::Info, FindingSeverity::Error]
    );
}

#[test]
fn component_severity_checks_every_internal_edge() {
    let graph = make_graph_with_provenance(
        &["a", "b", "c"],
        &[
            ("a", "b", EdgeProvenance::Inferred),
            ("b", "a", EdgeProvenance::Inferred),
            ("a", "c", EdgeProvenance::Inferred),
            ("c", "a", EdgeProvenance::HandDeclared),
        ],
    );
    let findings = cycle_findings(&graph);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, FindingSeverity::Error);
    assert_eq!(
        findings[0].message, "dependency cycle: a -> b -> a",
        "the hand edge is intentionally absent from the printed path: {findings:?}"
    );
}

#[test]
fn inferred_component_is_stable_under_permutation() {
    let first = make_graph_with_provenance(
        &["c", "a", "b"],
        &[
            ("a", "c", EdgeProvenance::Inferred),
            ("c", "a", EdgeProvenance::Inferred),
            ("a", "b", EdgeProvenance::Inferred),
            ("b", "a", EdgeProvenance::Inferred),
        ],
    );
    let permuted = make_graph_with_provenance(
        &["b", "c", "a"],
        &[
            ("b", "a", EdgeProvenance::Inferred),
            ("a", "b", EdgeProvenance::Inferred),
            ("c", "a", EdgeProvenance::Inferred),
            ("a", "c", EdgeProvenance::Inferred),
        ],
    );
    assert_eq!(cycle_findings(&permuted), cycle_findings(&first));
    assert_eq!(cycle_findings(&first)[0].severity, FindingSeverity::Info);
}

#[test]
fn inferred_self_loop_is_advisory() {
    let graph =
        make_graph_with_provenance(&["self"], &[("self", "self", EdgeProvenance::Inferred)]);
    let findings = cycle_findings(&graph);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, FindingSeverity::Info);
}

#[test]
fn containment_contradiction_stays_blocking() {
    let graph = with_containment(
        make_graph_with_provenance(
            &["ancestor", "child", "cycle-a", "cycle-b"],
            &[
                ("cycle-a", "cycle-b", EdgeProvenance::Inferred),
                ("cycle-b", "cycle-a", EdgeProvenance::Inferred),
                ("child", "ancestor", EdgeProvenance::HandDeclared),
            ],
        ),
        &[("ancestor", "child")],
    );
    let findings = topological_order(&graph).expect_err("contradiction must remain blocking");
    let dependency = findings
        .iter()
        .find(|finding| finding.message.starts_with("dependency cycle:"))
        .expect("discovery cycle finding");
    let containment = findings
        .iter()
        .find(|finding| {
            finding
                .message
                .starts_with("containment and dependency constraints are cyclic")
        })
        .expect("containment contradiction finding");
    assert_eq!(dependency.severity, FindingSeverity::Info);
    assert_eq!(containment.severity, FindingSeverity::Error);
}
