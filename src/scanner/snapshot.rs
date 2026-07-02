//! Persistent, deterministic map snapshot (`map.json`).
//!
//! Implements `dec.persistent-map-snapshot`: a committed, machine-readable
//! measurement record of the reconciled graph, alongside `map.md`. Building
//! is pure; writing lives here so `scan()` can call it the same way it calls
//! `outputs::write_map`.

use std::{fs, io, path::Path};

use crate::{
    blueprint::NodeKind,
    map::graph::{Finding, Graph, NodeState},
    reconcile::SymbolRecord,
};

/// Wire schema version for `map.json`. Bump on any breaking shape change.
pub const SCHEMA_VERSION: u32 = 1;

/// Committed machine-readable map snapshot. Deterministic: `BTreeMap`
/// ordering, no timestamps.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MapSnapshot {
    /// Wire schema version, starts at 1.
    pub schema_version: u32,
    /// Aggregate interface hash for this scan.
    pub interface_hash: String,
    /// Nodes in `BTreeMap` id order.
    pub nodes: Vec<SnapshotNode>,
    /// All dependency edges.
    pub edges: Vec<SnapshotEdge>,
    /// All findings from the scan.
    pub findings: Vec<Finding>,
}

/// One node's snapshot record.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotNode {
    /// Stable node ID.
    pub id: String,
    /// Lowercase node kind (`system`, `container`, `module`, `actor`).
    pub kind: String,
    /// Human-readable name.
    pub name: String,
    /// Lowercase reconciliation state (`synced`, `ghost`, `orphaned`).
    pub state: String,
    /// Declared paths.
    pub paths: Vec<String>,
    /// Claimed files.
    pub files: Vec<String>,
    /// Contract pointers.
    pub contracts: Vec<String>,
    /// Extracted public symbols for this node.
    pub symbols: Vec<SymbolRecord>,
}

/// One dependency edge.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotEdge {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Edge description.
    pub description: String,
}

/// Builds a deterministic snapshot from a reconciled graph.
#[must_use]
pub fn build(graph: &Graph, interface_hash: &str) -> MapSnapshot {
    let nodes = graph
        .nodes
        .values()
        .map(|node| SnapshotNode {
            id: node.id.clone(),
            kind: node_kind_label(node.kind).to_owned(),
            name: node.name.clone(),
            state: node_state_label(node.state).to_owned(),
            paths: node.paths.clone(),
            files: node.files.clone(),
            contracts: node.contracts.clone(),
            symbols: node.symbols.clone(),
        })
        .collect();
    let mut edges = Vec::new();
    for edge_list in graph.outbound.values() {
        for edge in edge_list {
            edges.push(SnapshotEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                description: edge.description.clone(),
            });
        }
    }
    MapSnapshot {
        schema_version: SCHEMA_VERSION,
        interface_hash: interface_hash.to_owned(),
        nodes,
        edges,
        findings: graph.findings.clone(),
    }
}

/// Writes `map.json` at the project root.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be serialised or written.
pub fn write(root: &Path, snapshot: &MapSnapshot) -> io::Result<()> {
    let mut body = serde_json::to_string_pretty(snapshot)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    body.push('\n');
    let path = root.join("map.json");
    if let Ok(existing) = fs::read_to_string(&path)
        && existing == body
    {
        return Ok(());
    }
    fs::write(path, body)
}

const fn node_kind_label(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "system",
        NodeKind::Container => "container",
        NodeKind::Module => "module",
        NodeKind::Actor => "actor",
    }
}

const fn node_state_label(state: NodeState) -> &'static str {
    match state {
        NodeState::Synced => "synced",
        NodeState::Ghost => "ghost",
        NodeState::Orphaned => "orphaned",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blueprint::Span,
        map::graph::{EdgeRef, NodeRecord},
    };
    use std::collections::BTreeMap;

    fn bare_node(id: &str, state: NodeState) -> NodeRecord {
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
            symbols: Vec::new(),
            contracts: Vec::new(),
            state,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn empty_graph() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    #[test]
    fn test_build_snapshot_has_schema_version_one() {
        let graph = empty_graph();
        let snapshot = build(&graph, "hash");
        assert_eq!(snapshot.schema_version, 1);
        assert!(snapshot.nodes.is_empty());
        assert!(snapshot.edges.is_empty());
    }

    #[test]
    fn test_build_snapshot_maps_node_fields() {
        let mut graph = empty_graph();
        graph
            .nodes
            .insert("app".to_owned(), bare_node("app", NodeState::Synced));
        graph.outbound.insert(
            "app".to_owned(),
            vec![EdgeRef {
                from: "app".to_owned(),
                to: "app.db".to_owned(),
                description: "uses".to_owned(),
            }],
        );
        let snapshot = build(&graph, "abc123");
        assert_eq!(snapshot.interface_hash, "abc123");
        assert_eq!(snapshot.nodes.len(), 1);
        assert_eq!(snapshot.nodes[0].id, "app");
        assert_eq!(snapshot.nodes[0].kind, "module");
        assert_eq!(snapshot.nodes[0].state, "synced");
        assert_eq!(snapshot.edges.len(), 1);
        assert_eq!(snapshot.edges[0].from, "app");
        assert_eq!(snapshot.edges[0].to, "app.db");
    }

    #[test]
    fn test_write_is_deterministic_and_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let mut graph = empty_graph();
        graph
            .nodes
            .insert("app".to_owned(), bare_node("app", NodeState::Ghost));
        let snapshot = build(&graph, "hash1");
        write(dir.path(), &snapshot).unwrap();
        let first = fs::read_to_string(dir.path().join("map.json")).unwrap();
        write(dir.path(), &snapshot).unwrap();
        let second = fs::read_to_string(dir.path().join("map.json")).unwrap();
        assert_eq!(first, second);
        assert!(first.contains("\"schema_version\": 1"));
    }
}
