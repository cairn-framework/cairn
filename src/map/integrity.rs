//! Reusable integrity algorithms.

use std::collections::{BTreeSet, VecDeque};

use super::graph::{Finding, FindingSeverity, Graph};

/// Finds dependency cycles without blocking basic graph construction.
#[must_use]
pub fn cycle_findings(graph: &Graph) -> Vec<Finding> {
    let mut findings = Vec::new();
    for start in graph.nodes.keys() {
        let mut queue = VecDeque::from([(start.clone(), vec![start.clone()])]);
        while let Some((current, path)) = queue.pop_front() {
            let Some(edges) = graph.outbound.get(&current) else {
                continue;
            };
            for edge in edges {
                if &edge.to == start {
                    let mut cycle = path.clone();
                    cycle.push(start.clone());
                    findings.push(Finding {
                        code: "CAIRN_ORDER_CYCLE".to_owned(),
                        severity: FindingSeverity::Error,
                        message: format!("dependency cycle: {}", cycle.join(" -> ")),
                        node: Some(start.clone()),
                        target: None,
                        path: None,
                    });
                    return findings;
                }
                if !path.contains(&edge.to) {
                    let mut next_path = path.clone();
                    next_path.push(edge.to.clone());
                    queue.push_back((edge.to.clone(), next_path));
                }
            }
        }
    }
    findings
}

/// Computes a topological order for the dependency graph.
///
/// # Errors
///
/// Returns cycle findings when the dependency graph is cyclic.
pub fn topological_order(graph: &Graph) -> Result<Vec<String>, Vec<Finding>> {
    let cycles = cycle_findings(graph);
    if !cycles.is_empty() {
        return Err(cycles);
    }
    let mut visited = BTreeSet::new();
    let mut order = Vec::new();
    for id in graph.nodes.keys() {
        visit(id, graph, &mut visited, &mut order);
    }
    Ok(order)
}

fn visit(id: &str, graph: &Graph, visited: &mut BTreeSet<String>, order: &mut Vec<String>) {
    if !visited.insert(id.to_owned()) {
        return;
    }
    if let Some(edges) = graph.outbound.get(id) {
        for edge in edges {
            visit(&edge.to, graph, visited, order);
        }
    }
    order.push(id.to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::{NodeKind, Span};
    use crate::map::graph::{EdgeRef, NodeRecord, NodeState};
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
            span: Span::point("test", 1, 1),
        }
    }

    fn make_graph(ids: &[&str], edges: &[(&str, &str)]) -> Graph {
        let nodes = ids
            .iter()
            .map(|id| ((*id).to_owned(), bare_node(id)))
            .collect();
        let mut outbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
        let mut inbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
        for (from, to) in edges {
            let e = EdgeRef {
                from: (*from).to_owned(),
                to: (*to).to_owned(),
                description: "dep".to_owned(),
            };
            outbound
                .entry((*from).to_owned())
                .or_default()
                .push(e.clone());
            inbound.entry((*to).to_owned()).or_default().push(e);
        }
        Graph {
            nodes,
            names: BTreeMap::new(),
            outbound,
            inbound,
            findings: Vec::new(),
        }
    }

    fn has_cycle_code(findings: &[Finding]) -> bool {
        findings.iter().any(|f| f.code == "CAIRN_ORDER_CYCLE")
    }

    // ── cycle_findings ────────────────────────────────────────────────────────

    #[test]
    fn test_cycle_findings_empty_graph_returns_no_findings() {
        let g = make_graph(&[], &[]);
        assert!(cycle_findings(&g).is_empty());
    }

    #[test]
    fn test_cycle_findings_acyclic_linear_chain_returns_no_findings() {
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        assert!(cycle_findings(&g).is_empty());
    }

    #[test]
    fn test_cycle_findings_acyclic_diamond_returns_no_findings() {
        // a→b, a→c, b→d, c→d — shared dependency, no cycle.
        let g = make_graph(
            &["a", "b", "c", "d"],
            &[("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")],
        );
        assert!(cycle_findings(&g).is_empty());
    }

    #[test]
    fn test_cycle_findings_two_node_cycle_returns_cairn_order_cycle() {
        let g = make_graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
        let findings = cycle_findings(&g);
        assert!(
            has_cycle_code(&findings),
            "expected CAIRN_ORDER_CYCLE finding"
        );
    }

    #[test]
    fn test_cycle_findings_three_node_cycle_returns_cairn_order_cycle() {
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c"), ("c", "a")]);
        let findings = cycle_findings(&g);
        assert!(has_cycle_code(&findings));
    }

    #[test]
    fn test_cycle_findings_detects_self_loop() {
        // A self-loop (a→a) is a cycle with one node.
        // The BFS condition `path.len() > 1` currently suppresses this — red test.
        let g = make_graph(&["a"], &[("a", "a")]);
        let findings = cycle_findings(&g);
        assert!(
            has_cycle_code(&findings),
            "self-loop a→a must produce CAIRN_ORDER_CYCLE; got: {findings:?}"
        );
    }

    #[test]
    fn test_cycle_findings_message_includes_path() {
        let g = make_graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
        let findings = cycle_findings(&g);
        assert!(!findings.is_empty());
        // Message must contain at least one of the cycle nodes.
        assert!(
            findings[0].message.contains("->"),
            "cycle message must include arrow-separated path: {}",
            findings[0].message
        );
    }

    #[test]
    fn test_cycle_findings_node_field_is_set() {
        let g = make_graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
        let findings = cycle_findings(&g);
        assert!(!findings.is_empty());
        assert!(
            findings[0].node.is_some(),
            "cycle finding must set the node field"
        );
    }

    // ── topological_order ─────────────────────────────────────────────────────

    #[test]
    fn test_topological_order_empty_graph_returns_empty_vec() {
        let g = make_graph(&[], &[]);
        let order = topological_order(&g).expect("empty graph is acyclic");
        assert!(order.is_empty());
    }

    #[test]
    fn test_topological_order_isolated_nodes_all_present() {
        let g = make_graph(&["x", "y", "z"], &[]);
        let order = topological_order(&g).expect("no edges, no cycle");
        let mut got = order.clone();
        got.sort();
        assert_eq!(got, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_topological_order_linear_chain_build_order() {
        // a→b→c: b must come before a, c before b (build order: deps first).
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let order = topological_order(&g).expect("acyclic");
        let pos = |id: &str| order.iter().position(|n| n == id).unwrap();
        assert!(pos("c") < pos("b"), "c must come before b (c is b's dep)");
        assert!(pos("b") < pos("a"), "b must come before a (b is a's dep)");
    }

    #[test]
    fn test_topological_order_all_nodes_present_in_output() {
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let mut order = topological_order(&g).expect("acyclic");
        order.sort();
        assert_eq!(order, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_topological_order_cycle_returns_err_with_findings() {
        let g = make_graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
        let err = topological_order(&g).unwrap_err();
        assert!(!err.is_empty(), "cyclic graph must return non-empty Err");
        assert!(has_cycle_code(&err));
    }

    #[test]
    fn test_topological_order_self_loop_returns_err() {
        // After the cycle_findings self-loop fix, this must also return Err.
        let g = make_graph(&["a"], &[("a", "a")]);
        let result = topological_order(&g);
        assert!(
            result.is_err(),
            "self-loop must cause topological_order to return Err"
        );
    }
}
