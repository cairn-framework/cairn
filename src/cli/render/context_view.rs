//! Container-level rollup and depth/scope projection for `cairn context`.
//!
//! `cairn context` renders the whole graph. On a large monorepo a flat
//! full-depth dump does not scale, so this module projects the graph to a
//! bounded view: every leaf-to-leaf edge rolls up to the deepest shown
//! ancestor of each endpoint, subtrees past the depth cap collapse to a count
//! with a drill-down command, and `--scope <node>` expands one subtree at full
//! detail. The text renderer in `project::render_context` delegates here; the
//! `cairn context --json` endpoint is unchanged and remains the full-detail
//! escape hatch.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::map::{Graph, NodeRecord, NodeState};

/// Default depth cap: the system node plus its direct children (top two
/// levels). Deeper subtrees collapse to a count.
const DEFAULT_DEPTH: usize = 1;

/// Parsed `cairn context` view options.
pub(crate) struct ContextOpts {
    /// Maximum node level shown before a subtree collapses.
    pub depth: usize,
    /// When set, render only this subtree at full detail.
    pub scope: Option<String>,
}

impl ContextOpts {
    /// Parses `--depth <N|all>` and `--scope <node>` from the command args.
    /// A missing or malformed depth value falls back to the default.
    pub(crate) fn parse(command_args: &[String]) -> Self {
        let mut depth = DEFAULT_DEPTH;
        let mut scope = None;
        let mut iter = command_args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--depth" => {
                    if let Some(value) = iter.next() {
                        depth = parse_depth(value);
                    }
                }
                "--scope" => scope = iter.next().cloned(),
                _ => {}
            }
        }
        Self { depth, scope }
    }
}

fn parse_depth(value: &str) -> usize {
    if value.eq_ignore_ascii_case("all") {
        usize::MAX
    } else {
        value.parse().unwrap_or(DEFAULT_DEPTH)
    }
}

/// Renders the `Structure:` body: either a scoped subtree or the rolled-up
/// bounded view. `prefix` is the system-id prefix (for example `cairn.`) used
/// to shorten identifiers for display.
pub(crate) fn render_structure(graph: &Graph, opts: &ContextOpts, prefix: &str) -> String {
    match &opts.scope {
        Some(scope) => render_scope(graph, scope, prefix),
        None => render_rolled(graph, opts.depth, prefix),
    }
}

fn render_rolled(graph: &Graph, depth: usize, prefix: &str) -> String {
    let rolled = rolled_edges(graph, depth);
    let hidden = hidden_counts(graph, depth);
    let mut out = String::new();
    for node in graph.nodes.values() {
        if node_level(graph, &node.id) > depth {
            continue;
        }
        let _ = writeln!(out, "  {}{}", short(prefix, &node.id), state_suffix(node));
        if let Some(count) = hidden.get(&node.id) {
            let _ = writeln!(
                out,
                "    ({count} nested node(s) collapsed; cairn context --scope {})",
                short(prefix, &node.id)
            );
        }
        for (target, label) in rolled.get(&node.id).into_iter().flatten() {
            write_edge(&mut out, prefix, target, label);
        }
    }
    out
}

fn render_scope(graph: &Graph, scope: &str, prefix: &str) -> String {
    let Some(root_id) = resolve_scope(graph, scope, prefix) else {
        return format!("  (no node matches --scope {scope})\n");
    };
    let mut out = String::new();
    for node in graph.nodes.values() {
        if !in_subtree(graph, &node.id, &root_id) {
            continue;
        }
        let _ = writeln!(out, "  {}{}", short(prefix, &node.id), state_suffix(node));
        for edge in graph.outbound.get(&node.id).into_iter().flatten() {
            write_edge(&mut out, prefix, &edge.to, &edge.description);
        }
    }
    out
}

/// Collapses every edge to the `(ancestor-at-depth, ancestor-at-depth)` pair of
/// its endpoints, dropping intra-subtree self-edges and merging labels. Returns
/// a map from the displayed source node to its sorted `(target, label)` edges.
fn rolled_edges(graph: &Graph, depth: usize) -> BTreeMap<String, Vec<(String, String)>> {
    let mut pairs: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    for edges in graph.outbound.values() {
        for edge in edges {
            let from = ancestor_at(graph, &edge.from, depth);
            let to = ancestor_at(graph, &edge.to, depth);
            if from == to {
                continue;
            }
            let labels = pairs.entry((from, to)).or_default();
            if !edge.description.is_empty() {
                labels.insert(edge.description.clone());
            }
        }
    }
    let mut grouped: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for ((from, to), labels) in pairs {
        let label = labels.into_iter().collect::<Vec<_>>().join("; ");
        grouped.entry(from).or_default().push((to, label));
    }
    grouped
}

/// Counts, per displayed node, how many descendants are hidden below the depth
/// cap. Each hidden node is attributed to its ancestor at the cap.
fn hidden_counts(graph: &Graph, depth: usize) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for node in graph.nodes.values() {
        if node_level(graph, &node.id) > depth {
            let ancestor = ancestor_at(graph, &node.id, depth);
            *counts.entry(ancestor).or_insert(0) += 1;
        }
    }
    counts
}

fn write_edge(out: &mut String, prefix: &str, target: &str, label: &str) {
    if label.is_empty() {
        let _ = writeln!(out, "    -> {}", short(prefix, target));
    } else {
        let _ = writeln!(out, "    -> {}  # {}", short(prefix, target), label);
    }
}

fn state_suffix(node: &NodeRecord) -> String {
    if node.state == NodeState::Synced {
        String::new()
    } else {
        format!(" [{:?}]", node.state)
    }
}

fn short(prefix: &str, id: &str) -> String {
    id.strip_prefix(prefix).unwrap_or(id).to_owned()
}

/// Returns the ancestor of `id` at `depth`, or `id` itself when its level is at
/// or above the cap.
fn ancestor_at(graph: &Graph, id: &str, depth: usize) -> String {
    let mut chain = vec![id.to_owned()];
    let mut current = id.to_owned();
    while let Some(parent) = graph.nodes.get(&current).and_then(|n| n.parent.clone()) {
        chain.push(parent.clone());
        current = parent;
    }
    let level = chain.len() - 1;
    if level <= depth {
        return id.to_owned();
    }
    chain[level - depth].clone()
}

fn node_level(graph: &Graph, id: &str) -> usize {
    let mut level = 0;
    let mut current = id.to_owned();
    while let Some(parent) = graph.nodes.get(&current).and_then(|n| n.parent.clone()) {
        level += 1;
        current = parent;
    }
    level
}

fn resolve_scope(graph: &Graph, scope: &str, prefix: &str) -> Option<String> {
    if graph.nodes.contains_key(scope) {
        return Some(scope.to_owned());
    }
    let full = format!("{prefix}{scope}");
    graph.nodes.contains_key(&full).then_some(full)
}

fn in_subtree(graph: &Graph, id: &str, root: &str) -> bool {
    let mut current = id.to_owned();
    loop {
        if current == root {
            return true;
        }
        match graph.nodes.get(&current).and_then(|n| n.parent.clone()) {
            Some(parent) => current = parent,
            None => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::{NodeKind, Span};
    use crate::map::graph::EdgeRef;

    fn node(id: &str, kind: NodeKind, parent: Option<&str>) -> NodeRecord {
        NodeRecord {
            kind,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            parent: parent.map(str::to_owned),
            children: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    /// Builds a graph: system `app` with container `app.k` holding leaves
    /// `app.k.a` and `app.k.b`, plus top-level leaf `app.x`. Edge
    /// `app.k.a -> app.x` should roll up to `k -> x` at the default depth.
    fn sample_graph() -> Graph {
        let nodes = [
            node("app", NodeKind::System, None),
            node("app.k", NodeKind::Container, Some("app")),
            node("app.k.a", NodeKind::Module, Some("app.k")),
            node("app.k.b", NodeKind::Module, Some("app.k")),
            node("app.x", NodeKind::Module, Some("app")),
        ];
        let mut outbound: BTreeMap<String, Vec<EdgeRef>> = BTreeMap::new();
        outbound.insert(
            "app.k.a".to_owned(),
            vec![
                EdgeRef {
                    from: "app.k.a".to_owned(),
                    to: "app.x".to_owned(),
                    description: "calls".to_owned(),
                },
                EdgeRef {
                    from: "app.k.a".to_owned(),
                    to: "app.k.b".to_owned(),
                    description: "internal".to_owned(),
                },
            ],
        );
        Graph {
            nodes: nodes.into_iter().map(|n| (n.id.clone(), n)).collect(),
            names: BTreeMap::new(),
            outbound,
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    #[test]
    fn test_default_depth_collapses_container_children_with_drilldown() {
        let graph = sample_graph();
        let opts = ContextOpts {
            depth: 1,
            scope: None,
        };
        let out = render_structure(&graph, &opts, "app.");
        assert!(out.contains("  k\n"), "container shown: {out}");
        assert!(!out.contains("  k.a"), "leaf must be collapsed: {out}");
        assert!(
            out.contains("(2 nested node(s) collapsed; cairn context --scope k)"),
            "drill-down hint missing: {out}"
        );
    }

    #[test]
    fn test_leaf_edge_rolls_up_to_container_edge_with_label() {
        let graph = sample_graph();
        let opts = ContextOpts {
            depth: 1,
            scope: None,
        };
        let out = render_structure(&graph, &opts, "app.");
        assert!(
            out.contains("  k\n") && out.contains("    -> x  # calls"),
            "rolled k -> x edge missing: {out}"
        );
    }

    #[test]
    fn test_intra_container_edge_is_dropped_at_rollup() {
        let graph = sample_graph();
        let opts = ContextOpts {
            depth: 1,
            scope: None,
        };
        let out = render_structure(&graph, &opts, "app.");
        // `app.k.a -> app.k.b` collapses to k -> k and must not appear.
        assert!(!out.contains("# internal"), "self-edge leaked: {out}");
    }

    #[test]
    fn test_full_depth_shows_leaves_and_real_edges() {
        let graph = sample_graph();
        let opts = ContextOpts {
            depth: usize::MAX,
            scope: None,
        };
        let out = render_structure(&graph, &opts, "app.");
        assert!(out.contains("  k.a\n"), "leaf shown at full depth: {out}");
        assert!(out.contains("    -> x  # calls"), "real edge: {out}");
        assert!(!out.contains("collapsed"), "nothing collapses: {out}");
    }

    #[test]
    fn test_scope_renders_only_subtree_at_full_detail() {
        let graph = sample_graph();
        let opts = ContextOpts {
            depth: 1,
            scope: Some("k".to_owned()),
        };
        let out = render_structure(&graph, &opts, "app.");
        assert!(out.contains("  k\n"), "scope root shown: {out}");
        assert!(out.contains("  k.a\n"), "subtree leaf shown: {out}");
        assert!(
            out.contains("    -> k.b  # internal"),
            "internal edge: {out}"
        );
        assert!(!out.contains("  x\n"), "out-of-scope node excluded: {out}");
    }

    #[test]
    fn test_scope_unknown_node_reports_no_match() {
        let graph = sample_graph();
        let opts = ContextOpts {
            depth: 1,
            scope: Some("nope".to_owned()),
        };
        let out = render_structure(&graph, &opts, "app.");
        assert!(out.contains("no node matches --scope nope"), "{out}");
    }

    #[test]
    fn test_parse_depth_all_and_default() {
        assert_eq!(ContextOpts::parse(&["context".to_owned()]).depth, 1);
        let all =
            ContextOpts::parse(&["context".to_owned(), "--depth".to_owned(), "all".to_owned()]);
        assert_eq!(all.depth, usize::MAX);
        let bad =
            ContextOpts::parse(&["context".to_owned(), "--depth".to_owned(), "xyz".to_owned()]);
        assert_eq!(bad.depth, 1, "malformed depth falls back to default");
    }

    #[test]
    fn test_parse_scope_value() {
        let opts = ContextOpts::parse(&[
            "context".to_owned(),
            "--scope".to_owned(),
            "app.k".to_owned(),
        ]);
        assert_eq!(opts.scope.as_deref(), Some("app.k"));
    }
}
