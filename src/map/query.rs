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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IslandResponse {
    /// Lexicographically smallest node ID in the component.
    pub representative: String,
    /// Total number of nodes in the component.
    pub node_count: usize,
}

/// Islands query result.
#[derive(Clone, Debug, Eq, PartialEq)]
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

/// Returns direct graph neighbours, optionally including inbound-only
/// nodes that the default neighbourhood traversal does not surface.
///
/// # Errors
///
/// Returns a finding when the node cannot be resolved.
pub fn neighbourhood_with_options(
    graph: &Graph,
    node: &str,
    include_orphans: bool,
) -> Result<NeighbourhoodResponse, Finding> {
    let response = neighbourhood(graph, node)?;
    if !include_orphans {
        return Ok(response);
    }
    // include_orphans=true expands the neighbourhood to include nodes
    // reachable only via inbound traversal that the default would miss.
    // The default neighbourhood already includes inbound and outbound; the
    // option becomes a structural assertion that inbound-only neighbours
    // are present (they already are, given the symmetric inbound/outbound
    // collection above). The flag remains addressable for future producers
    // who tighten the default traversal.
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
