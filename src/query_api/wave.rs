//! Derived write-sets and the disjointness test
//! (`res.parallel-dispatch-rung-3` Part 3, clause 3 phase 0).
//!
//! A unit's write-set is the containment closure of its anchor (the node
//! plus descendants via `children`; dependency edges are rung 1 Order and
//! deliberately excluded), mapped to file prefixes through declared paths,
//! with more-specific outside owners subtracted. Composition reads the
//! working tree; recompute-equality at `ruling run` covers drift, replacing
//! commit-pinning (clause 1). Phase 0 is deterministic and blind: hotspot
//! paths sit in no unit's derived set, so every derived write-set is
//! stamped `completeness: "partial"` naming the uncovered hotspot prefixes.

// Reason: the wave composer (todo.plan-identity-wave-composer) consumes this
// module and lands next on this branch; until then only its tests call it.
#![allow(dead_code)]

use crate::map::graph::Graph;
use crate::map::paths::{is_component_prefix, trim_dot};

/// The blueprint tag naming a hotspot owner node.
const HOTSPOT_TAG: &str = "hotspot";

/// A unit's derived write surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WriteSet {
    /// Sorted, `./`-trimmed prefixes with no trailing slash.
    pub includes: Vec<String>,
    /// Prefixes inside `includes` owned more specifically by nodes outside
    /// the closure; not part of the unit's surface.
    pub excludes: Vec<String>,
    /// `derived` or `unresolved`.
    pub resolution: &'static str,
    /// Why derivation failed closed, when it did.
    pub unresolved_reason: Option<String>,
    /// Always `partial` in phase 0.
    pub completeness: &'static str,
    /// Names the uncovered hotspot prefixes.
    pub completeness_reason: String,
}

/// The hotspot prefixes: paths of nodes tagged `@hotspot`, sorted.
pub(crate) fn hotspot_prefixes(graph: &Graph) -> Vec<String> {
    let mut prefixes: Vec<String> = graph
        .nodes
        .values()
        .filter(|node| node.tags.iter().any(|tag| tag == HOTSPOT_TAG))
        .flat_map(|node| node.paths.iter().map(|path| trim_dot(path)))
        .collect();
    prefixes.sort();
    prefixes.dedup();
    prefixes
}

fn completeness_reason(hotspots: &[String]) -> String {
    if hotspots.is_empty() {
        "phase 0 derivation is blind to hotspot edits".to_owned()
    } else {
        format!(
            "phase 0 derivation cannot attribute hotspot edits; uncovered prefixes: {}",
            hotspots.join(", ")
        )
    }
}

fn unresolved(graph: &Graph, reason: String) -> WriteSet {
    WriteSet {
        includes: vec![".".to_owned()],
        excludes: Vec::new(),
        resolution: "unresolved",
        unresolved_reason: Some(reason),
        completeness: "partial",
        completeness_reason: completeness_reason(&hotspot_prefixes(graph)),
    }
}

/// Derives the write-set for a unit anchored at `anchor`.
///
/// Fails closed and visibly: an unresolvable anchor, an `owns_files: true`
/// closure node with no declared paths, or a closure declaring no paths at
/// all yields the universal prefix `.` with `resolution: "unresolved"` and
/// a reason; the unit stays in the preview and dispatches alone.
pub(crate) fn derive_write_set(graph: &Graph, anchor: &str) -> WriteSet {
    if !graph.nodes.contains_key(anchor) {
        return unresolved(graph, format!("anchor `{anchor}` is not a graph node"));
    }

    // Containment closure: the anchor plus descendants via `children`.
    let mut closure = std::collections::BTreeSet::new();
    let mut queue = vec![anchor.to_owned()];
    while let Some(id) = queue.pop() {
        if !closure.insert(id.clone()) {
            continue;
        }
        if let Some(node) = graph.nodes.get(&id) {
            queue.extend(node.children.iter().cloned());
        }
    }

    let mut includes = Vec::new();
    for id in &closure {
        let Some(node) = graph.nodes.get(id) else {
            continue;
        };
        if node.owns_files && node.paths.is_empty() {
            return unresolved(
                graph,
                format!("closure node `{id}` declares owns-files with no paths"),
            );
        }
        includes.extend(node.paths.iter().map(|path| trim_dot(path)));
    }
    includes.sort();
    includes.dedup();
    if includes.is_empty() {
        return unresolved(
            graph,
            format!("the closure of `{anchor}` declares no paths"),
        );
    }

    // Subtract prefixes owned more specifically by nodes outside the
    // closure: the hotspot owners bite here, vanishing from every other
    // unit's surface.
    let mut excludes: Vec<String> = graph
        .nodes
        .values()
        .filter(|node| !closure.contains(&node.id))
        .flat_map(|node| node.paths.iter().map(|path| trim_dot(path)))
        .filter(|outside| {
            includes
                .iter()
                .any(|inside| inside != outside && is_component_prefix(inside, outside))
        })
        .collect();
    excludes.sort();
    excludes.dedup();

    WriteSet {
        includes,
        excludes,
        resolution: "derived",
        unresolved_reason: None,
        completeness: "partial",
        completeness_reason: completeness_reason(&hotspot_prefixes(graph)),
    }
}

/// True when a prefix is covered by either side's exclusion list.
fn excluded(prefix: &str, a: &WriteSet, b: &WriteSet) -> bool {
    a.excludes
        .iter()
        .chain(b.excludes.iter())
        .any(|excluded| is_component_prefix(excluded, prefix))
}

/// Component-boundary disjointness: no include of one set equals or
/// contains an include of the other, in either direction, unless the deeper
/// prefix is subtracted by an exclusion.
pub(crate) fn write_sets_disjoint(a: &WriteSet, b: &WriteSet) -> bool {
    for left in &a.includes {
        for right in &b.includes {
            let overlaps = is_component_prefix(left, right) || is_component_prefix(right, left);
            if !overlaps {
                continue;
            }
            let deeper = if right.len() >= left.len() {
                right
            } else {
                left
            };
            if !excluded(deeper, a, b) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::NodeKind;
    use crate::map::graph::{NodeRecord, NodeState};
    use std::collections::BTreeMap;

    fn node(id: &str, children: &[&str], paths: &[&str], tags: &[&str]) -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: tags.iter().map(ToString::to_string).collect(),
            parent: None,
            children: children.iter().map(ToString::to_string).collect(),
            paths: paths.iter().map(ToString::to_string).collect(),
            owns_files: false,
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            symbols: Vec::new(),
            span: crate::blueprint::Span::point(String::new(), 1, 1),
        }
    }

    fn graph(nodes: Vec<NodeRecord>) -> Graph {
        Graph {
            nodes: nodes.into_iter().map(|n| (n.id.clone(), n)).collect(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    #[test]
    fn sibling_prefixes_do_not_overlap() {
        let g = graph(vec![
            node("app.ui", &[], &["./src/ui"], &[]),
            node("app.assets", &[], &["./src/ui_assets"], &[]),
        ]);
        let ui = derive_write_set(&g, "app.ui");
        let assets = derive_write_set(&g, "app.assets");
        assert_eq!(ui.includes, vec!["src/ui"]);
        assert_eq!(ui.resolution, "derived");
        assert!(
            write_sets_disjoint(&ui, &assets),
            "src/ui does not overlap src/ui_assets"
        );
    }

    #[test]
    fn closure_includes_descendants_and_conflicts_with_parents() {
        let g = graph(vec![
            node("app", &["app.core", "app.util"], &[], &[]),
            node("app.core", &[], &["./src/core"], &[]),
            node("app.util", &[], &["./src/util"], &[]),
        ]);
        let whole = derive_write_set(&g, "app");
        assert_eq!(whole.includes, vec!["src/core", "src/util"]);
        let core = derive_write_set(&g, "app.core");
        assert!(
            !write_sets_disjoint(&whole, &core),
            "a parent-closure unit conflicts with a child unit"
        );
    }

    #[test]
    fn unresolvable_anchor_yields_universal_prefix_with_reason() {
        let g = graph(vec![node("app.core", &[], &["./src/core"], &[])]);
        let ws = derive_write_set(&g, "app.missing");
        assert_eq!(ws.includes, vec!["."]);
        assert_eq!(ws.resolution, "unresolved");
        assert!(
            ws.unresolved_reason
                .as_deref()
                .is_some_and(|r| r.contains("app.missing")),
            "{ws:?}"
        );
        // The universal prefix conflicts with everything: the unit
        // dispatches alone rather than vanishing.
        let core = derive_write_set(&g, "app.core");
        assert!(!write_sets_disjoint(&ws, &core));
    }

    #[test]
    fn outside_more_specific_owner_is_subtracted() {
        let g = graph(vec![
            node("app.docs", &[], &["./docs"], &[]),
            node("app.registries", &[], &["./docs/registries"], &["hotspot"]),
            node("app.core", &[], &["./src/core"], &[]),
        ]);
        let docs = derive_write_set(&g, "app.docs");
        assert_eq!(docs.excludes, vec!["docs/registries"]);
        let registries = derive_write_set(&g, "app.registries");
        assert!(
            write_sets_disjoint(&docs, &registries),
            "the subtracted hotspot no longer collides with the broad owner"
        );
        assert!(
            docs.completeness_reason.contains("docs/registries"),
            "the partial stamp names the hotspot prefixes: {}",
            docs.completeness_reason
        );
        assert_eq!(docs.completeness, "partial");
    }
}
