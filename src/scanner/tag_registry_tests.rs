//! Tests for unregistered blueprint tag findings.

use std::collections::BTreeMap;

use super::checks::check_tag_registry;
use crate::{
    blueprint::{NodeKind, Span},
    map::graph::{FindingSeverity, Graph, NodeRecord, NodeState},
    scanner::config::TagRegistry,
};

fn graph_with_tags(tags: Vec<&str>) -> Graph {
    let node = NodeRecord {
        kind: NodeKind::Module,
        id: "app.api".to_owned(),
        name: "app.api".to_owned(),
        description: String::new(),
        tags: tags.into_iter().map(str::to_owned).collect(),
        parent: None,
        children: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        state: NodeState::Synced,
        files: Vec::new(),
        symbols: Vec::new(),
        span: Span::point("test", 1, 1),
    };
    Graph {
        nodes: [(node.id.clone(), node)].into_iter().collect(),
        names: BTreeMap::new(),
        outbound: BTreeMap::new(),
        inbound: BTreeMap::new(),
        findings: Vec::new(),
    }
}

#[test]
fn unregistered_tag_emits_info_finding_once_per_node_tag() {
    let mut graph = graph_with_tags(vec!["known", "missing", "missing"]);
    let registry = TagRegistry::parse("tags:\n  known:\n    description: A known tag\n").unwrap();

    check_tag_registry(&mut graph, &registry);

    let findings: Vec<_> = graph
        .findings
        .iter()
        .filter(|finding| finding.code == "CAIRN_TAG_UNREGISTERED")
        .collect();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, FindingSeverity::Info);
    assert_eq!(findings[0].node.as_deref(), Some("app.api"));
    assert!(findings[0].message.contains("missing"));
}
