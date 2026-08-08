// cairn:allow-large-module reason: integrity algorithms and their cohesive graph invariant tests remain together
//! Reusable integrity algorithms.
use super::graph::{Finding, FindingSeverity, Graph};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn dependency_components(graph: &Graph) -> Vec<Vec<&str>> {
    let mut outbound: BTreeMap<&str, Vec<&str>> = graph
        .nodes
        .keys()
        .map(|id| (id.as_str(), Vec::new()))
        .collect();
    let mut inbound: BTreeMap<&str, Vec<&str>> = graph
        .nodes
        .keys()
        .map(|id| (id.as_str(), Vec::new()))
        .collect();

    for (from, edges) in &graph.outbound {
        let Some(targets) = outbound.get_mut(from.as_str()) else {
            continue;
        };
        for edge in edges {
            if let Some(reverse_targets) = inbound.get_mut(edge.to.as_str()) {
                targets.push(edge.to.as_str());
                reverse_targets.push(from.as_str());
            }
        }
        targets.sort_unstable();
        targets.dedup();
    }
    for targets in inbound.values_mut() {
        targets.sort_unstable();
        targets.dedup();
    }

    let mut visited = BTreeSet::new();
    let mut finish = Vec::with_capacity(graph.nodes.len());
    for &start in outbound.keys() {
        if visited.contains(start) {
            continue;
        }
        let mut stack = vec![(start, false)];
        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                finish.push(node);
                continue;
            }
            if !visited.insert(node) {
                continue;
            }
            stack.push((node, true));
            for &next in outbound[node].iter().rev() {
                if !visited.contains(next) {
                    stack.push((next, false));
                }
            }
        }
    }

    let mut assigned = BTreeSet::new();
    let mut components = Vec::new();
    for &start in finish.iter().rev() {
        if !assigned.insert(start) {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            component.push(node);
            for &next in &inbound[node] {
                if assigned.insert(next) {
                    stack.push(next);
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort_by(|left, right| left[0].cmp(right[0]));
    components
}

fn is_cyclic_component(component: &[&str], graph: &Graph) -> bool {
    component.len() > 1
        || graph
            .outbound
            .get(component[0])
            .into_iter()
            .flatten()
            .any(|edge| edge.to == component[0])
}

fn representative_cycle<'a>(graph: &'a Graph, component: &[&'a str]) -> Vec<&'a str> {
    let start = component[0];
    let members: BTreeSet<&str> = component.iter().copied().collect();
    let mut parent = BTreeMap::from([(start, start)]);
    let mut queue = VecDeque::from([start]);
    while let Some(node) = queue.pop_front() {
        let Some(edges) = graph.outbound.get(node) else {
            continue;
        };
        let mut neighbors: Vec<&str> = edges
            .iter()
            .map(|edge| edge.to.as_str())
            .filter(|next| members.contains(next))
            .collect();
        neighbors.sort_unstable();
        neighbors.dedup();
        for next in neighbors {
            if next == start {
                let mut path = vec![node];
                let mut current = node;
                while current != start {
                    current = parent[current];
                    path.push(current);
                }
                path.reverse();
                path.push(start);
                return path;
            }
            if !parent.contains_key(next) {
                parent.insert(next, node);
                queue.push_back(next);
            }
        }
    }
    debug_assert!(false, "cyclic component has no representative cycle");
    unreachable!("cyclic component has no representative cycle");
}

fn dependency_cycle_data(graph: &Graph) -> (Vec<Finding>, BTreeMap<&str, usize>) {
    let mut findings = Vec::new();
    let mut membership = BTreeMap::new();
    for component in dependency_components(graph) {
        if !is_cyclic_component(&component, graph) {
            continue;
        }
        let component_id = findings.len();
        for &node in &component {
            membership.insert(node, component_id);
        }
        let path = representative_cycle(graph, &component);
        findings.push(Finding {
            code: "CAIRN_ORDER_CYCLE".to_owned(),
            severity: FindingSeverity::Error,
            message: format!("dependency cycle: {}", path.join(" -> ")),
            node: Some(component[0].to_owned()),
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        });
    }
    (findings, membership)
}

/// Finds dependency cycles without blocking basic graph construction.
#[must_use]
pub fn cycle_findings(graph: &Graph) -> Vec<Finding> {
    dependency_cycle_data(graph).0
}
fn combined_cycle_finding(
    indegree: &BTreeMap<&str, usize>,
    succ: &BTreeMap<&str, Vec<&str>>,
) -> Finding {
    let mut outdeg: BTreeMap<&str, usize> = BTreeMap::new();
    let mut pred: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (&before, afters) in succ {
        if !indegree.contains_key(before) {
            continue;
        }
        for &after in afters {
            if indegree.contains_key(after) {
                *outdeg.entry(before).or_insert(0) += 1;
                pred.entry(after).or_default().push(before);
            }
        }
    }
    let mut peel: Vec<&str> = indegree
        .keys()
        .filter(|id| !outdeg.contains_key(*id))
        .copied()
        .collect();
    let mut stuck: BTreeSet<&str> = indegree.keys().copied().collect();
    while let Some(leaf) = peel.pop() {
        stuck.remove(leaf);
        for &parent in pred.get(leaf).into_iter().flatten() {
            if let Some(degree) = outdeg.get_mut(parent) {
                *degree -= 1;
                if *degree == 0 {
                    outdeg.remove(parent);
                    peel.push(parent);
                }
            }
        }
    }
    let stuck: Vec<&str> = stuck.into_iter().collect();
    Finding {
        code: "CAIRN_ORDER_CYCLE".to_owned(),
        severity: FindingSeverity::Error,
        message: format!(
            "containment and dependency constraints are cyclic among: {}",
            stuck.join(", ")
        ),
        node: stuck.first().map(|s| (*s).to_owned()),
        target: None,
        path: None,
        deferred_by: None,
        parked_by: None,
    }
}

/// Computes a deterministic topological order for the graph.
///
/// Ordering constraints:
/// 1. Dependency edges: a node's dependencies sort before it.
/// 2. Containment edges: children sort before their parent.
/// 3. Ties break by node id, so the order is independent of declaration or
///    key order.
///
/// Both edge kinds are hard constraints. Dependency cycles are reported as
/// dependency findings, while this combined pass still checks for additional
/// cycles involving containment edges. A contradiction between them (for
/// example a node depending on its own container's parent chain) is reported
/// as a cycle, the same as a pure dependency cycle.
///
/// # Errors
///
/// Returns cycle findings when the dependency graph or the combined dependency
/// and containment constraints are cyclic.
pub fn topological_order(graph: &Graph) -> Result<Vec<String>, Vec<Finding>> {
    let (mut cycles, dependency_cycle_membership) = dependency_cycle_data(graph);
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
            if let (Some(from_component), Some(to_component)) = (
                dependency_cycle_membership.get(from.as_str()),
                dependency_cycle_membership.get(edge.to.as_str()),
            ) && from_component == to_component
            {
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
            // Internal dependency SCC edges were removed above, so a
            // deadlock here necessarily includes a containment edge.
            cycles.push(combined_cycle_finding(&indegree, &succ));
            return Err(cycles);
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
    if cycles.is_empty() {
        Ok(order)
    } else {
        Err(cycles)
    }
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
        // a→b, a→c, b→d, c→d; shared dependency, no cycle.
        let g = make_graph(
            &["a", "b", "c", "d"],
            &[("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")],
        );
        assert!(cycle_findings(&g).is_empty());
    }

    #[test]
    fn test_cycle_findings_acyclic_shared_dependency_triangle_returns_no_findings() {
        let g = make_graph(
            &["api", "auth", "db"],
            &[("api", "auth"), ("api", "db"), ("auth", "db")],
        );
        assert!(
            cycle_findings(&g).is_empty(),
            "shared dependency DAG must not be reported as a cycle"
        );
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
    fn test_cycle_findings_enumerates_disjoint_components_in_id_order() {
        let g = make_graph(
            &["b2", "a1", "b1", "a2"],
            &[("b2", "b1"), ("b1", "b2"), ("a2", "a1"), ("a1", "a2")],
        );
        let findings = cycle_findings(&g);
        assert_eq!(
            findings.len(),
            2,
            "one finding per cyclic SCC: {findings:?}"
        );
        assert_eq!(
            findings
                .iter()
                .map(|finding| finding.node.as_deref())
                .collect::<Vec<_>>(),
            vec![Some("a1"), Some("b1")]
        );
    }

    #[test]
    fn test_cycle_findings_collapses_multiple_simple_cycles_per_component() {
        let first = make_graph(
            &["c", "a", "b"],
            &[("a", "c"), ("c", "a"), ("a", "b"), ("b", "a")],
        );
        let permuted = make_graph(
            &["b", "c", "a"],
            &[("b", "a"), ("a", "b"), ("c", "a"), ("a", "c")],
        );

        let first_findings = cycle_findings(&first);
        let permuted_findings = cycle_findings(&permuted);
        assert_eq!(first_findings.len(), 1);
        assert_eq!(permuted_findings, first_findings);
    }

    #[test]
    fn test_topological_order_reports_dependency_and_containment_cycles() {
        let g = with_containment(
            make_graph(
                &["ancestor", "child", "cycle-a", "cycle-b"],
                &[
                    ("cycle-a", "cycle-b"),
                    ("cycle-b", "cycle-a"),
                    ("child", "ancestor"),
                ],
            ),
            &[("ancestor", "child")],
        );
        let err = topological_order(&g).expect_err("both contradictions must be reported");
        assert_eq!(
            err.iter()
                .filter(|finding| finding.message.starts_with("dependency cycle:"))
                .count(),
            1,
            "expected dependency SCC finding: {err:?}"
        );
        assert_eq!(
            err.iter()
                .filter(|finding| {
                    finding
                        .message
                        .starts_with("containment and dependency constraints are cyclic")
                })
                .count(),
            1,
            "expected combined-constraint finding: {err:?}"
        );
    }

    #[test]
    fn test_cycle_findings_detects_self_loop() {
        // A self-loop (a→a) is a cycle with one node.
        // The SCC pass must retain this singleton self-loop.
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

    #[test]
    fn test_topological_order_cycle_finding_excludes_downstream_nodes() {
        // "parent" contains "child" (cycle with dep child->parent); "aft"
        // merely depends on "child" so it is blocked downstream but is NOT
        // part of the contradiction. The finding must name only the cycle.
        let g = with_containment(
            make_graph(
                &["parent", "child", "aft"],
                &[("child", "parent"), ("aft", "child")],
            ),
            &[("parent", "child")],
        );
        let err = topological_order(&g).expect_err("contradiction must be Err");
        let msg = &err[0].message;
        assert!(msg.contains("child") && msg.contains("parent"), "{msg}");
        assert!(
            !msg.contains("aft"),
            "downstream node reported as cyclic: {msg}"
        );
        assert_ne!(err[0].node.as_deref(), Some("aft"), "{:?}", err[0].node);
    }
}
