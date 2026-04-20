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
                if &edge.to == start && path.len() > 1 {
                    let mut cycle = path.clone();
                    cycle.push(start.clone());
                    findings.push(Finding {
                        code: "CAIRN_ORDER_CYCLE".to_owned(),
                        severity: FindingSeverity::Error,
                        message: format!("dependency cycle: {}", cycle.join(" -> ")),
                        node: Some(start.clone()),
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
