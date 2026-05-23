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
    use crate::map::graph::{EdgeRef, FindingSeverity, NodeRecord, NodeState};
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

    // ── graph builder helpers ─────────────────────────────────────────────────

    fn edge(from: &str, to: &str, desc: &str) -> EdgeRef {
        EdgeRef {
            from: from.to_owned(),
            to: to.to_owned(),
            description: desc.to_owned(),
        }
    }

    /// Build a simple graph from a node-id list and directed edge pairs.
    /// Parent/children are derived from the ownership edges.
    fn make_graph(node_ids: &[&str], dep_edges: &[(&str, &str)]) -> Graph {
        let mut nodes: BTreeMap<String, NodeRecord> = node_ids
            .iter()
            .map(|id| ((*id).to_owned(), node(id, None, &[])))
            .collect();
        let mut outbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
        let mut inbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
        for (from, to) in dep_edges {
            let e = edge(from, to, "dep");
            outbound
                .entry((*from).to_owned())
                .or_default()
                .push(e.clone());
            inbound.entry((*to).to_owned()).or_default().push(e);
        }
        // Wire parent/children for ownership semantics (not used by dep queries,
        // but needed for graph() ownership-edge rendering).
        for (id, node_rec) in &mut nodes {
            if let Some(edges) = outbound.get(id) {
                node_rec.children = edges.iter().map(|e| e.to.clone()).collect();
            }
        }
        for (id, edges) in &inbound {
            if let (Some(node_rec), Some(e)) = (nodes.get_mut(id), edges.first()) {
                node_rec.parent = Some(e.from.clone());
            }
        }
        Graph {
            nodes,
            names: BTreeMap::new(),
            outbound,
            inbound,
            findings: Vec::new(),
        }
    }

    // ── get ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_get_returns_node_for_known_id() {
        let g = make_graph(&["app.api"], &[]);
        let resp = get(&g, "app.api").expect("should resolve");
        assert_eq!(resp.node.id, "app.api");
    }

    #[test]
    fn test_get_unknown_id_returns_query_finding() {
        let g = make_graph(&[], &[]);
        let err = get(&g, "missing").unwrap_err();
        assert_eq!(err.code, "CAIRN_QUERY_NODE_NOT_FOUND");
    }

    // ── neighbourhood ─────────────────────────────────────────────────────────

    #[test]
    fn test_neighbourhood_isolated_node_has_empty_edge_lists() {
        let g = make_graph(&["a"], &[]);
        let resp = neighbourhood(&g, "a").expect("resolve");
        assert!(resp.inbound.is_empty());
        assert!(resp.outbound.is_empty());
    }

    #[test]
    fn test_neighbourhood_includes_outbound_and_inbound() {
        // a → b ← c
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("c", "b")]);
        let resp = neighbourhood(&g, "b").expect("resolve");
        // b has two inbound (a, c) and no outbound
        let mut inbound = resp.inbound.clone();
        inbound.sort();
        assert_eq!(inbound, vec!["a", "c"]);
        assert!(resp.outbound.is_empty());
    }

    #[test]
    fn test_neighbourhood_node_id_returned_on_response() {
        let g = make_graph(&["x"], &[]);
        let resp = neighbourhood(&g, "x").expect("resolve");
        assert_eq!(resp.node.id, "x");
    }

    // ── files ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_files_returns_empty_list_for_node_with_no_claims() {
        let g = make_graph(&["a"], &[]);
        let resp = files(&g, "a").expect("resolve");
        assert_eq!(resp.node, "a");
        assert!(resp.files.is_empty());
    }

    // ── depends ───────────────────────────────────────────────────────────────

    #[test]
    fn test_depends_direct_returns_immediate_neighbours_only() {
        // a → b → c  — direct query on 'a' returns only 'b', not 'c'.
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let resp = depends(&g, "a", false).expect("resolve");
        assert_eq!(resp.nodes, vec!["b"]);
    }

    #[test]
    fn test_depends_transitive_chain() {
        // a → b → c — transitive from 'a' includes both b and c.
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let resp = depends(&g, "a", true).expect("resolve");
        assert_eq!(resp.nodes, vec!["b", "c"]);
    }

    #[test]
    fn test_depends_transitive_diamond_deduplicates() {
        // a → b → d
        // a → c → d
        // Transitive from 'a': should list d exactly once, not twice.
        let g = make_graph(
            &["a", "b", "c", "d"],
            &[("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")],
        );
        let resp = depends(&g, "a", true).expect("resolve");
        // Sorted + deduped: b, c, d — d must appear exactly once.
        assert_eq!(resp.nodes, vec!["b", "c", "d"]);
    }

    #[test]
    fn test_depends_no_outbound_edges_returns_empty() {
        let g = make_graph(&["a"], &[]);
        let resp = depends(&g, "a", true).expect("resolve");
        assert!(resp.nodes.is_empty());
    }

    #[test]
    fn test_depends_unknown_node_returns_error() {
        let g = make_graph(&[], &[]);
        let err = depends(&g, "x", false).unwrap_err();
        assert_eq!(err.code, "CAIRN_QUERY_NODE_NOT_FOUND");
    }

    // ── dependents ────────────────────────────────────────────────────────────

    #[test]
    fn test_dependents_direct_returns_immediate_callers_only() {
        // a → b  — direct dependents of 'b' is ['a'], not transitively more.
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let resp = dependents(&g, "b", false).expect("resolve");
        assert_eq!(resp.nodes, vec!["a"]);
    }

    #[test]
    fn test_dependents_transitive_chain() {
        // a → b → c  — transitive dependents of 'c' are a and b.
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let resp = dependents(&g, "c", true).expect("resolve");
        assert_eq!(resp.nodes, vec!["a", "b"]);
    }

    #[test]
    fn test_dependents_leaf_node_has_no_dependents() {
        let g = make_graph(&["a", "b"], &[("a", "b")]);
        let resp = dependents(&g, "a", true).expect("resolve");
        assert!(resp.nodes.is_empty());
    }

    // ── graph ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_graph_query_includes_dependency_edges() {
        let g = make_graph(&["a", "b"], &[("a", "b")]);
        let resp = graph(&g);
        let dep_edges: Vec<_> = resp
            .edges
            .iter()
            .filter(|e| e.kind == GraphEdgeKind::Dependency)
            .collect();
        assert_eq!(dep_edges.len(), 1);
        assert_eq!(dep_edges[0].from, "a");
        assert_eq!(dep_edges[0].to, "b");
    }

    #[test]
    fn test_graph_query_edges_sorted_deterministically() {
        // Two edges: a→c and a→b. After sort they must appear in (a,b) then (a,c) order.
        let g = make_graph(&["a", "b", "c"], &[("a", "c"), ("a", "b")]);
        let resp = graph(&g);
        let dep_edges: Vec<_> = resp
            .edges
            .iter()
            .filter(|e| e.kind == GraphEdgeKind::Dependency)
            .collect();
        assert_eq!(dep_edges.len(), 2);
        assert_eq!(dep_edges[0].to, "b");
        assert_eq!(dep_edges[1].to, "c");
    }

    // ── order ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_order_linear_chain_returns_topological_order() {
        // a → b → c (a depends on b, b depends on c).
        // order() returns build-order: dependencies come first.
        // So c must appear before b, and b before a.
        let g = make_graph(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
        let resp = order(&g).expect("acyclic graph must order");
        let pos = |id: &str| resp.nodes.iter().position(|n| n == id).unwrap();
        assert!(
            pos("c") < pos("b"),
            "leaf must precede its caller in build order"
        );
        assert!(pos("b") < pos("a"), "b must precede a in build order");
    }

    #[test]
    fn test_order_cycle_returns_error() {
        // a → b → a — must return Err, not hang.
        let g = make_graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
        let result = order(&g);
        assert!(result.is_err(), "cyclic graph must return Err from order()");
    }

    // ── lint ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_lint_includes_existing_graph_findings() {
        let mut g = make_graph(&["a"], &[]);
        g.findings.push(Finding {
            code: "TEST_FINDING".to_owned(),
            severity: FindingSeverity::Warning,
            message: "test".to_owned(),
            node: None,
            target: None,
            path: None,
        });
        let resp = lint(&g);
        assert!(
            resp.findings.iter().any(|f| f.code == "TEST_FINDING"),
            "lint must include pre-existing graph findings"
        );
    }

    #[test]
    fn test_lint_detects_cycles_via_cycle_findings() {
        // a → b → a — lint must add cycle findings (no CAIRN_CYCLE_* finding from
        // order alone; lint calls cycle_findings which does).
        let g = make_graph(&["a", "b"], &[("a", "b"), ("b", "a")]);
        let resp = lint(&g);
        assert!(
            !resp.findings.is_empty(),
            "lint on cyclic graph must return findings"
        );
    }
}
