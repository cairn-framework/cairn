//! Reusable integrity algorithms.
use super::graph::{Finding, FindingSeverity, Graph};
use std::collections::{BTreeMap, BTreeSet};
/// Finds dependency cycles without blocking basic graph construction.
#[must_use]
#[allow(clippy::missing_panics_doc)] // Reason: infallible unwrap; entry was just inserted via or_insert
pub fn cycle_findings(graph: &Graph) -> Vec<Finding> {
    // 0 = white (unvisited), 1 = gray (in stack), 2 = black (done)
    let mut color: BTreeMap<&str, u8> = BTreeMap::new();
    let mut stack: Vec<&str> = Vec::new();
    for start in graph.nodes.keys().map(String::as_str) {
        if color.get(start).copied().unwrap_or(0) != 0 {
            continue;
        }
        stack.push(start);
        while let Some(node) = stack.last().copied() {
            match color.entry(node).or_insert(0) {
                0 => {
                    *color.get_mut(node).unwrap() = 1;
                    if let Some(edges) = graph.outbound.get(node) {
                        for edge in edges {
                            let to = edge.to.as_str();
                            match color.get(to).copied().unwrap_or(0) {
                                1 => {
                                    // Cycle found: extract cycle from stack
                                    let cycle: Vec<_> = stack
                                        .iter()
                                        .skip_while(|&n| *n != to)
                                        .copied()
                                        .chain(std::iter::once(to))
                                        .collect();
                                    return vec![Finding {
                                        code: "CAIRN_ORDER_CYCLE".to_owned(),
                                        severity: FindingSeverity::Error,
                                        message: format!(
                                            "dependency cycle: {}",
                                            cycle.join(" -> ")
                                        ),
                                        node: Some(start.to_owned()),
                                        target: None,
                                        path: None,
                                    }];
                                }
                                2 => {}
                                _ => {
                                    stack.push(to);
                                }
                            }
                        }
                    }
                }
                1 => {
                    *color.get_mut(node).unwrap() = 2;
                    stack.pop();
                }
                _ => {
                    stack.pop();
                }
            }
        }
    }
    Vec::new()
}

/// Computes a deterministic topological order for the graph.
///
/// Ordering constraints:
/// 1. Dependency edges: a node's dependencies sort before it.
/// 2. Containment edges: children sort before their parent.
/// 3. Ties break by node id, so the order is independent of declaration or
///    key order.
///
/// Both edge kinds are hard constraints: a contradiction between them (for
/// example a node depending on its own container's parent chain) is reported
/// as a cycle, the same as a pure dependency cycle.
///
/// # Errors
///
/// Returns cycle findings when the combined dependency and containment
/// constraints are cyclic.
pub fn topological_order(graph: &Graph) -> Result<Vec<String>, Vec<Finding>> {
    let cycles = cycle_findings(graph);
    if !cycles.is_empty() {
        return Err(cycles);
    }
    // Precedence successors: `before -> [after]`. Dependency edge a->b means
    // b precedes a; containment means child precedes parent. Both are hard.
    let mut succ: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut indegree: BTreeMap<&str, usize> =
        graph.nodes.keys().map(|id| (id.as_str(), 0)).collect();
    for (from, edges) in &graph.outbound {
        for edge in edges {
            if !indegree.contains_key(edge.to.as_str()) {
                continue;
            }
            if let Some(deg) = indegree.get_mut(from.as_str()) {
                *deg += 1;
                succ.entry(edge.to.as_str())
                    .or_default()
                    .push(from.as_str());
            }
        }
    }
    for node in graph.nodes.values() {
        for child in &node.children {
            if !indegree.contains_key(child.as_str()) {
                continue;
            }
            if let Some(deg) = indegree.get_mut(node.id.as_str()) {
                *deg += 1;
                succ.entry(child.as_str())
                    .or_default()
                    .push(node.id.as_str());
            }
        }
    }
    let mut ready: BTreeSet<&str> = indegree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(id, _)| *id)
        .collect();
    let mut order: Vec<String> = Vec::with_capacity(graph.nodes.len());
    while order.len() < graph.nodes.len() {
        let Some(&next) = ready.iter().next() else {
            // Deadlock: the dependency graph alone is acyclic (checked
            // above), so containment contradicts a dependency edge. Report
            // the stuck nodes as a cycle rather than inventing an order.
            let mut stuck: Vec<&str> = indegree.keys().copied().collect();
            stuck.sort_unstable();
            return Err(vec![Finding {
                code: "CAIRN_ORDER_CYCLE".to_owned(),
                severity: FindingSeverity::Error,
                message: format!(
                    "containment and dependency constraints are cyclic among: {}",
                    stuck.join(", ")
                ),
                node: stuck.first().map(|s| (*s).to_owned()),
                target: None,
                path: None,
            }]);
        };
        ready.remove(next);
        indegree.remove(next);
        order.push(next.to_owned());
        for succ_id in succ.remove(next).unwrap_or_default() {
            if let Some(deg) = indegree.get_mut(succ_id) {
                *deg -= 1;
                if *deg == 0 {
                    ready.insert(succ_id);
                }
            }
        }
    }
    Ok(order)
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
            symbols: Vec::new(),
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

    fn with_containment(mut g: Graph, links: &[(&str, &str)]) -> Graph {
        for (parent, child) in links {
            g.nodes
                .get_mut(*parent)
                .unwrap()
                .children
                .push((*child).to_owned());
            g.nodes.get_mut(*child).unwrap().parent = Some((*parent).to_owned());
        }
        g
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
        let mut got = order;
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

    #[test]
    fn test_topological_order_children_before_parent() {
        // Containment only, no dependency edges: children sort before parent.
        // IDs chosen so lexicographic key order alone would put the parent
        // first ("app" < "ymir" < "zed").
        let g = with_containment(
            make_graph(&["app", "ymir", "zed"], &[]),
            &[("app", "ymir"), ("app", "zed")],
        );
        let order = topological_order(&g).expect("acyclic");
        let pos = |id: &str| order.iter().position(|n| n == id).unwrap();
        assert!(
            pos("ymir") < pos("app"),
            "child ymir before parent: {order:?}"
        );
        assert!(
            pos("zed") < pos("app"),
            "child zed before parent: {order:?}"
        );
    }

    #[test]
    fn test_topological_order_nested_containment_deepest_first() {
        // "alpha" contains "midge" contains "zeta": deepest first, even though
        // key order puts the root first.
        let g = with_containment(
            make_graph(&["alpha", "midge", "zeta"], &[]),
            &[("alpha", "midge"), ("midge", "zeta")],
        );
        let order = topological_order(&g).expect("acyclic");
        let pos = |id: &str| order.iter().position(|n| n == id).unwrap();
        assert!(pos("zeta") < pos("midge"), "{order:?}");
        assert!(pos("midge") < pos("alpha"), "{order:?}");
    }

    #[test]
    fn test_topological_order_containment_dep_conflict_is_cycle_error() {
        // Child depends on its own parent: containment says child first,
        // the dependency edge says parent first. Contradictory constraints
        // are a cycle error, not a silently invented order.
        let g = with_containment(
            make_graph(&["parent", "child"], &[("child", "parent")]),
            &[("parent", "child")],
        );
        let err = topological_order(&g).expect_err("contradictory constraints must be Err");
        assert!(has_cycle_code(&err), "expected CAIRN_ORDER_CYCLE: {err:?}");
    }

    #[test]
    fn test_topological_order_containment_and_deps_combined() {
        // "app" contains "web" and "zlib"; "web" depends on "zlib".
        let g = with_containment(
            make_graph(&["app", "web", "zlib"], &[("web", "zlib")]),
            &[("app", "web"), ("app", "zlib")],
        );
        let order = topological_order(&g).expect("acyclic");
        let pos = |id: &str| order.iter().position(|n| n == id).unwrap();
        assert!(pos("zlib") < pos("web"), "{order:?}");
        assert!(pos("web") < pos("app"), "{order:?}");
        assert!(pos("zlib") < pos("app"), "{order:?}");
    }
}
