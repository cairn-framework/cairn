//! Reusable integrity algorithms.
use super::graph::{Finding, FindingSeverity, Graph};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type DependencyAdjacency<'a> = (
    BTreeMap<&'a str, Vec<&'a str>>,
    BTreeMap<&'a str, Vec<&'a str>>,
);

fn dependency_adjacency(graph: &Graph) -> DependencyAdjacency<'_> {
    let mut outbound: BTreeMap<&str, Vec<&str>> = graph
        .nodes
        .keys()
        .map(|id| (id.as_str(), Vec::new()))
        .collect();
    let mut inbound = outbound.clone();
    for (from, edges) in &graph.outbound {
        let Some(targets) = outbound.get_mut(from.as_str()) else {
            continue;
        };
        for edge in edges {
            let Some(reverse_targets) = inbound.get_mut(edge.to.as_str()) else {
                continue;
            };
            targets.push(edge.to.as_str());
            reverse_targets.push(from.as_str());
        }
        targets.sort_unstable();
        targets.dedup();
    }
    for targets in inbound.values_mut() {
        targets.sort_unstable();
        targets.dedup();
    }
    (outbound, inbound)
}

fn kosaraju_components(outbound: &[Vec<usize>], inbound: &[Vec<usize>]) -> Vec<Vec<usize>> {
    debug_assert_eq!(outbound.len(), inbound.len());
    let mut visited = vec![false; outbound.len()];
    let mut finish = Vec::with_capacity(outbound.len());
    for start in 0..outbound.len() {
        if visited[start] {
            continue;
        }
        let mut stack = vec![(start, false)];
        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                finish.push(node);
                continue;
            }
            if visited[node] {
                continue;
            }
            visited[node] = true;
            stack.push((node, true));
            for &next in outbound[node].iter().rev() {
                if !visited[next] {
                    stack.push((next, false));
                }
            }
        }
    }

    let mut assigned = vec![false; inbound.len()];
    let mut components = Vec::new();
    for &start in finish.iter().rev() {
        if assigned[start] {
            continue;
        }
        assigned[start] = true;
        let mut component = Vec::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            component.push(node);
            for &next in &inbound[node] {
                if !assigned[next] {
                    assigned[next] = true;
                    stack.push(next);
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort_by_key(|component| component[0]);
    components
}

fn dependency_components(graph: &Graph) -> Vec<Vec<&str>> {
    let (dependency_outbound, dependency_inbound) = dependency_adjacency(graph);
    let nodes: Vec<&str> = dependency_outbound.keys().copied().collect();
    let indices: BTreeMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(index, &node)| (node, index))
        .collect();
    let to_index = |node: &str| {
        indices
            .get(node)
            .copied()
            .expect("dependency adjacency node has an index")
    };
    let outbound: Vec<Vec<usize>> = nodes
        .iter()
        .map(|node| {
            dependency_outbound
                .get(node)
                .into_iter()
                .flatten()
                .map(|target| to_index(target))
                .collect()
        })
        .collect();
    let inbound: Vec<Vec<usize>> = nodes
        .iter()
        .map(|node| {
            dependency_inbound
                .get(node)
                .into_iter()
                .flatten()
                .map(|target| to_index(target))
                .collect()
        })
        .collect();
    kosaraju_components(&outbound, &inbound)
        .into_iter()
        .map(|component| component.into_iter().map(|index| nodes[index]).collect())
        .collect()
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

fn dependency_cycle_data(graph: &Graph) -> (Vec<Finding>, Vec<Vec<&str>>) {
    let components = dependency_components(graph);
    let mut findings = Vec::new();
    for component in &components {
        if !is_cyclic_component(component, graph) {
            continue;
        }
        let path = representative_cycle(graph, component);
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
    (findings, components)
}

/// Finds dependency cycles without blocking basic graph construction.
#[must_use]
pub fn cycle_findings(graph: &Graph) -> Vec<Finding> {
    dependency_cycle_data(graph).0
}

fn component_membership<'a>(components: &[Vec<&'a str>]) -> BTreeMap<&'a str, usize> {
    let mut membership = BTreeMap::new();
    for (component_id, component) in components.iter().enumerate() {
        for &node in component {
            membership.insert(node, component_id);
        }
    }
    membership
}

fn condensed_successors<'a>(
    graph: &'a Graph,
    components: &[Vec<&'a str>],
    membership: &BTreeMap<&'a str, usize>,
) -> BTreeMap<usize, Vec<usize>> {
    let mut succ: BTreeMap<usize, Vec<usize>> =
        (0..components.len()).map(|id| (id, Vec::new())).collect();
    for (from, edges) in &graph.outbound {
        let Some(&from_component) = membership.get(from.as_str()) else {
            continue;
        };
        for edge in edges {
            let Some(&to_component) = membership.get(edge.to.as_str()) else {
                continue;
            };
            if from_component != to_component {
                succ.entry(to_component).or_default().push(from_component);
            }
        }
    }
    for node in graph.nodes.values() {
        let Some(&parent) = membership.get(node.id.as_str()) else {
            continue;
        };
        for child in &node.children {
            let Some(&child_component) = membership.get(child.as_str()) else {
                continue;
            };
            succ.entry(child_component).or_default().push(parent);
        }
    }
    for afters in succ.values_mut() {
        afters.sort_unstable();
        afters.dedup();
    }
    succ
}

fn quotient_adjacency(
    succ: &BTreeMap<usize, Vec<usize>>,
    count: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut outbound = vec![Vec::new(); count];
    let mut inbound = vec![Vec::new(); count];
    for (&before, afters) in succ {
        for &after in afters {
            outbound[before].push(after);
            inbound[after].push(before);
        }
    }
    for neighbors in outbound.iter_mut().chain(inbound.iter_mut()) {
        neighbors.sort_unstable();
        neighbors.dedup();
    }
    (outbound, inbound)
}

fn stuck_components(succ: &BTreeMap<usize, Vec<usize>>, count: usize) -> BTreeSet<usize> {
    let (outbound, inbound) = quotient_adjacency(succ, count);
    kosaraju_components(&outbound, &inbound)
        .into_iter()
        .filter(|component| component.len() > 1 || outbound[component[0]].contains(&component[0]))
        .flatten()
        .collect()
}

fn combined_cycle_finding(
    components: &[Vec<&str>],
    succ: &BTreeMap<usize, Vec<usize>>,
) -> Option<Finding> {
    let stuck = stuck_components(succ, components.len());
    if stuck.is_empty() {
        return None;
    }
    let mut nodes: Vec<&str> = stuck
        .iter()
        .flat_map(|id| components[*id].iter().copied())
        .collect();
    nodes.sort_unstable();
    Some(Finding {
        code: "CAIRN_ORDER_CYCLE".to_owned(),
        severity: FindingSeverity::Error,
        message: format!(
            "containment and dependency constraints are cyclic among: {}",
            nodes.join(", ")
        ),
        node: nodes.first().map(|node| (*node).to_owned()),
        target: None,
        path: None,
        deferred_by: None,
        parked_by: None,
    })
}

fn combined_constraint_cycle<'a>(graph: &'a Graph, components: &[Vec<&'a str>]) -> Option<Finding> {
    let membership = component_membership(components);
    let succ = condensed_successors(graph, components, &membership);
    combined_cycle_finding(components, &succ)
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
    let (mut cycles, components) = dependency_cycle_data(graph);
    if let Some(finding) = combined_constraint_cycle(graph, &components) {
        cycles.push(finding);
    }
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
            unreachable!("acyclic constraints must have a ready node");
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
mod tests;
