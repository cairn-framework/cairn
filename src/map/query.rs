// cairn:allow-large-module reason: hub for typed query responses (get, neighbourhood, files, depends/dependents, graph, order, lint, islands, neighbourhood_with_options) plus their unit tests; per-query splits already exist for renderers in src/cli/render.rs but the typed surface lives together.
//! Typed query services over map graphs.

use super::{
    graph::{Finding, Graph, NodeRecord},
    integrity,
};

/// Node response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeResponse {
    /// Node record.
    pub node: NodeRecord,
}

/// Neighbourhood response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeighbourhoodResponse {
    /// Center node.
    pub node: NodeRecord,
    /// Upstream dependency IDs.
    pub inbound: Vec<String>,
    /// Downstream dependency IDs.
    pub outbound: Vec<String>,
}

/// File list response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilesResponse {
    /// Node ID.
    pub node: String,
    /// Claimed files.
    pub files: Vec<String>,
}

/// Dependency response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyResponse {
    /// Node ID.
    pub node: String,
    /// Dependency IDs.
    pub nodes: Vec<String>,
}

/// Graph explorer edge kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphEdgeKind {
    /// Parent-to-child ownership edge.
    Ownership,
    /// Declared dependency edge.
    Dependency,
}

/// Graph explorer edge response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphEdgeResponse {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Edge kind.
    pub kind: GraphEdgeKind,
    /// Human-readable edge description.
    pub description: String,
}

/// Graph explorer response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphResponse {
    /// All map nodes.
    pub nodes: Vec<NodeRecord>,
    /// Ownership and dependency edges.
    pub edges: Vec<GraphEdgeResponse>,
}

/// Order response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderResponse {
    /// Ordered node IDs.
    pub nodes: Vec<String>,
}

/// Lint response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LintResponse {
    /// Findings.
    pub findings: Vec<Finding>,
}

/// One connected component of the map graph.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct IslandResponse {
    /// Representative node ID (lexicographically smallest in component).
    pub representative: String,
    /// Number of nodes in this component.
    pub node_count: usize,
}

/// Islands query result.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct IslandsResponse {
    /// Wire schema version.
    pub schema_version: u32,
    /// One entry per connected component, ordered by representative ID.
    pub islands: Vec<IslandResponse>,
}

/// Wire schema version for the islands response.
pub const ISLANDS_SCHEMA_VERSION: u32 = 1;

/// Resolves and returns a node.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn get(graph: &Graph, node: &str) -> Result<NodeResponse, Finding> {
    Ok(NodeResponse {
        node: graph.resolve(node)?.clone(),
    })
}

/// Returns direct graph neighbours.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn neighbourhood(graph: &Graph, node: &str) -> Result<NeighbourhoodResponse, Finding> {
    let node_record = graph.resolve(node)?.clone();
    Ok(NeighbourhoodResponse {
        inbound: graph
            .inbound
            .get(&node_record.id)
            .map(|edges| edges.iter().map(|edge| edge.from.clone()).collect())
            .unwrap_or_default(),
        outbound: graph
            .outbound
            .get(&node_record.id)
            .map(|edges| edges.iter().map(|edge| edge.to.clone()).collect())
            .unwrap_or_default(),
        node: node_record,
    })
}

/// Returns files claimed by a node.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn files(graph: &Graph, node: &str) -> Result<FilesResponse, Finding> {
    let node = graph.resolve(node)?;
    Ok(FilesResponse {
        node: node.id.clone(),
        files: node.files.clone(),
    })
}

/// Returns nodes the subject depends on.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn depends(graph: &Graph, node: &str, transitive: bool) -> Result<DependencyResponse, Finding> {
    let node = graph.resolve(node)?;
    Ok(DependencyResponse {
        node: node.id.clone(),
        nodes: collect(graph, &node.id, transitive, true),
    })
}

/// Returns nodes that depend on the subject.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn dependents(
    graph: &Graph,
    node: &str,
    transitive: bool,
) -> Result<DependencyResponse, Finding> {
    let node = graph.resolve(node)?;
    Ok(DependencyResponse {
        node: node.id.clone(),
        nodes: collect(graph, &node.id, transitive, false),
    })
}

/// Returns the graph explorer structural graph.
#[must_use]
pub fn graph(graph: &Graph) -> GraphResponse {
    let nodes = graph.nodes.values().cloned().collect::<Vec<_>>();
    let ownership = nodes
        .iter()
        .filter_map(|node| node.parent.as_ref().map(|parent| (parent, &node.id)))
        .map(|(from, to)| GraphEdgeResponse {
            from: from.clone(),
            to: to.clone(),
            kind: GraphEdgeKind::Ownership,
            description: "owns".to_owned(),
        });
    let dependencies = graph
        .outbound
        .values()
        .flatten()
        .map(|edge| GraphEdgeResponse {
            from: edge.from.clone(),
            to: edge.to.clone(),
            kind: GraphEdgeKind::Dependency,
            description: edge.description.clone(),
        });
    let mut edges = ownership.chain(dependencies).collect::<Vec<_>>();
    edges.sort_by(|left, right| {
        (
            &left.from,
            &left.to,
            edge_kind_name(left.kind),
            &left.description,
        )
            .cmp(&(
                &right.from,
                &right.to,
                edge_kind_name(right.kind),
                &right.description,
            ))
    });
    GraphResponse { nodes, edges }
}

/// Returns dependency order or cycle findings.
///
/// # Errors
///
/// Returns cycle findings when the dependency graph is cyclic.
pub fn order(graph: &Graph) -> Result<OrderResponse, Vec<Finding>> {
    integrity::topological_order(graph).map(|nodes| OrderResponse { nodes })
}

/// Returns grouped lint findings, including cycles.
#[must_use]
pub fn lint(graph: &Graph) -> LintResponse {
    let mut findings = graph.findings.clone();
    findings.extend(integrity::cycle_findings(graph));
    LintResponse { findings }
}

/// Returns the connected-component breakdown of the entire map.
///
/// Edges are treated as undirected for the purposes of grouping. Each
/// island carries a node count and a representative (the lexicographically
/// smallest node ID inside the component). The response is versioned via
/// `schema_version` per the test contract.
#[must_use]
pub fn islands(graph: &Graph) -> IslandsResponse {
    let component_index = compute_components(graph);
    let mut groups: std::collections::BTreeMap<usize, Vec<String>> =
        std::collections::BTreeMap::new();
    for (id, idx) in &component_index {
        groups.entry(*idx).or_default().push(id.clone());
    }
    let mut islands: Vec<IslandResponse> = groups
        .values()
        .map(|members| {
            let mut sorted = members.clone();
            sorted.sort();
            let representative = sorted.first().cloned().unwrap_or_default();
            IslandResponse {
                representative,
                node_count: sorted.len(),
            }
        })
        .collect();
    islands.sort_by(|a, b| a.representative.cmp(&b.representative));
    IslandsResponse {
        schema_version: ISLANDS_SCHEMA_VERSION,
        islands,
    }
}

/// Returns direct graph neighbours.
///
/// When `include_orphans` is `false` the response carries only the
/// outbound-edge neighbours, matching the spec scenario "with
/// `include_orphans: false` includes only the outbound-edge neighbour".
/// When `true` the inbound neighbours (nodes that depend on `node` but
/// are not reachable forward from it) are also included.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn neighbourhood_with_options(
    graph: &Graph,
    node: &str,
    include_orphans: bool,
) -> Result<NeighbourhoodResponse, Finding> {
    let mut response = neighbourhood(graph, node)?;
    if !include_orphans {
        response.inbound.clear();
    }
    Ok(response)
}

fn compute_components(graph: &Graph) -> std::collections::BTreeMap<String, usize> {
    let mut index: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut next: usize = 0;
    for id in graph.nodes.keys() {
        if index.contains_key(id) {
            continue;
        }
        bfs_component(graph, id, next, &mut index);
        next += 1;
    }
    index
}

fn bfs_component(
    graph: &Graph,
    start: &str,
    component: usize,
    index: &mut std::collections::BTreeMap<String, usize>,
) {
    let mut frontier: std::collections::VecDeque<String> = std::collections::VecDeque::new();
    frontier.push_back(start.to_owned());
    index.insert(start.to_owned(), component);
    while let Some(current) = frontier.pop_front() {
        for next in undirected_neighbours(graph, &current) {
            if index.contains_key(&next) {
                continue;
            }
            index.insert(next.clone(), component);
            frontier.push_back(next);
        }
    }
}

fn undirected_neighbours(graph: &Graph, id: &str) -> Vec<String> {
    // Connected-component grouping treats both DEPENDENCY and
    // OWNERSHIP edges as undirected. Cycle 3 reasoning: the spec
    // scenario "a map whose nodes form a single connected component"
    // (specs/query/spec.md) reads naturally for ownership-rooted maps;
    // restricting to dependency edges only would split typical N-node
    // ownership trees into N islands, breaking the scenario verbatim.
    // Phase-2.5's edge-kind taxonomy (Ownership vs Dependency) remains
    // useful for rendering but is not the right axis for connectivity.
    let mut out = Vec::new();
    if let Some(edges) = graph.outbound.get(id) {
        for edge in edges {
            out.push(edge.to.clone());
        }
    }
    if let Some(edges) = graph.inbound.get(id) {
        for edge in edges {
            out.push(edge.from.clone());
        }
    }
    if let Some(node) = graph.nodes.get(id) {
        if let Some(parent) = &node.parent {
            out.push(parent.clone());
        }
        for child in &node.children {
            out.push(child.clone());
        }
    }
    out
}

const fn edge_kind_name(kind: GraphEdgeKind) -> &'static str {
    match kind {
        GraphEdgeKind::Ownership => "ownership",
        GraphEdgeKind::Dependency => "dependency",
    }
}

fn collect(graph: &Graph, id: &str, transitive: bool, outbound: bool) -> Vec<String> {
    let edges = if outbound {
        graph.outbound.get(id)
    } else {
        graph.inbound.get(id)
    };
    let mut nodes = Vec::new();
    if let Some(edges) = edges {
        for edge in edges {
            let next = if outbound { &edge.to } else { &edge.from };
            nodes.push(next.clone());
            if transitive {
                nodes.extend(collect(graph, next, true, outbound));
            }
        }
    }
    nodes.sort();
    nodes.dedup();
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::{NodeKind, Span};
    use crate::map::graph::{EdgeRef, NodeRecord, NodeState};
    use std::collections::BTreeMap;

    fn node(id: &str, parent: Option<&str>, children: &[&str]) -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            parent: parent.map(str::to_owned),
            children: children.iter().map(|c| (*c).to_owned()).collect(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    /// Cycle 3: ownership-only graph forms a single connected component.
    /// Locks the spec scenario "a map whose nodes form a single connected
    /// component" against future regressions where someone removes the
    /// parent/children traversal from `undirected_neighbours`.
    #[test]
    fn islands_single_component_via_ownership_only() {
        let mut nodes = BTreeMap::new();
        nodes.insert("root".to_owned(), node("root", None, &["a", "b"]));
        nodes.insert("a".to_owned(), node("a", Some("root"), &[]));
        nodes.insert("b".to_owned(), node("b", Some("root"), &[]));
        let graph = Graph {
            nodes,
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        };
        let resp = islands(&graph);
        assert_eq!(resp.islands.len(), 1);
        assert_eq!(resp.islands[0].node_count, 3);
        assert_eq!(resp.islands[0].representative, "a");
    }

    /// Cycle 3: two dependency-disconnected components remain two
    /// islands. Locks the disconnected-subgraph scenario.
    #[test]
    fn islands_split_two_dependency_components() {
        let mut nodes = BTreeMap::new();
        nodes.insert("a".to_owned(), node("a", None, &[]));
        nodes.insert("b".to_owned(), node("b", None, &[]));
        nodes.insert("c".to_owned(), node("c", None, &[]));
        nodes.insert("d".to_owned(), node("d", None, &[]));
        let mut outbound = BTreeMap::new();
        outbound.insert(
            "a".to_owned(),
            vec![EdgeRef {
                from: "a".to_owned(),
                to: "b".to_owned(),
                description: "calls".to_owned(),
            }],
        );
        outbound.insert(
            "c".to_owned(),
            vec![EdgeRef {
                from: "c".to_owned(),
                to: "d".to_owned(),
                description: "calls".to_owned(),
            }],
        );
        let mut inbound = BTreeMap::new();
        inbound.insert(
            "b".to_owned(),
            vec![EdgeRef {
                from: "a".to_owned(),
                to: "b".to_owned(),
                description: "calls".to_owned(),
            }],
        );
        inbound.insert(
            "d".to_owned(),
            vec![EdgeRef {
                from: "c".to_owned(),
                to: "d".to_owned(),
                description: "calls".to_owned(),
            }],
        );
        let graph = Graph {
            nodes,
            names: BTreeMap::new(),
            outbound,
            inbound,
            findings: Vec::new(),
        };
        let resp = islands(&graph);
        assert_eq!(resp.islands.len(), 2);
        let reps: Vec<&str> = resp
            .islands
            .iter()
            .map(|i| i.representative.as_str())
            .collect();
        assert_eq!(reps, vec!["a", "c"]);
    }

    /// Cycle 3 reviewer F2: two ownership-only trees with no
    /// dependency edges remain two separate islands. Catches a
    /// regression that would flatten ownership-disjoint trees into
    /// one component.
    #[test]
    fn islands_two_ownership_trees_stay_separate() {
        let mut nodes = BTreeMap::new();
        nodes.insert("r1".to_owned(), node("r1", None, &["a"]));
        nodes.insert("a".to_owned(), node("a", Some("r1"), &[]));
        nodes.insert("r2".to_owned(), node("r2", None, &["b"]));
        nodes.insert("b".to_owned(), node("b", Some("r2"), &[]));
        let graph = Graph {
            nodes,
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        };
        let resp = islands(&graph);
        assert_eq!(resp.islands.len(), 2);
    }

    /// Cycle 3: `include_orphans` contract pinned against the default
    /// neighbourhood query.
    #[test]
    fn neighbourhood_with_options_diverges_against_default() {
        let mut nodes = BTreeMap::new();
        nodes.insert("anchor".to_owned(), node("anchor", None, &[]));
        nodes.insert("out".to_owned(), node("out", None, &[]));
        nodes.insert("inb".to_owned(), node("inb", None, &[]));
        let mut outbound = BTreeMap::new();
        outbound.insert(
            "anchor".to_owned(),
            vec![EdgeRef {
                from: "anchor".to_owned(),
                to: "out".to_owned(),
                description: "calls".to_owned(),
            }],
        );
        let mut inbound = BTreeMap::new();
        inbound.insert(
            "anchor".to_owned(),
            vec![EdgeRef {
                from: "inb".to_owned(),
                to: "anchor".to_owned(),
                description: "depends-on".to_owned(),
            }],
        );
        let mut names = BTreeMap::new();
        for id in &["anchor", "out", "inb"] {
            names.insert((*id).to_owned(), vec![(*id).to_owned()]);
        }
        let graph = Graph {
            nodes,
            names,
            outbound,
            inbound,
            findings: Vec::new(),
        };
        let with_orphans = neighbourhood_with_options(&graph, "anchor", true).expect("with");
        let default = neighbourhood(&graph, "anchor").expect("default");
        assert_eq!(with_orphans.inbound, default.inbound);
        assert_eq!(with_orphans.outbound, default.outbound);
        let no_orphans = neighbourhood_with_options(&graph, "anchor", false).expect("no orphans");
        assert!(no_orphans.inbound.is_empty());
        assert_eq!(no_orphans.outbound, default.outbound);
    }
}
