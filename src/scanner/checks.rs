//! Blueprint change, provenance, claims, and gitignored-path checks emitted during scanning.

use std::{collections::BTreeSet, path::Path};

use super::{ArtefactSet, Graph, blueprint, config, state};
use crate::artefacts::registry::DecisionStatus;

pub(crate) fn check_blueprint_change_decisions(
    graph: &mut Graph,
    artefacts: &ArtefactSet,
    current: &state::BlueprintSnapshot,
    previous: &state::BlueprintSnapshot,
) {
    if previous.is_empty() {
        return;
    }
    if artefacts.decisions.is_empty() {
        return;
    }

    let covered: BTreeSet<&str> = artefacts
        .decisions
        .iter()
        .filter(|d| {
            matches!(
                d.status,
                DecisionStatus::Proposed | DecisionStatus::Accepted | DecisionStatus::Superseded
            )
        })
        .flat_map(|d| d.nodes.iter().map(String::as_str))
        .collect();

    let mut emitted: BTreeSet<String> = BTreeSet::new();
    let mut emit = |node_id: &str| {
        if !covered.contains(node_id) && emitted.insert(node_id.to_owned()) {
            graph.findings.push(crate::map::graph::Finding {
                code: "CAIRN_BLUEPRINT_CHANGE_NO_DECISION".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "blueprint shape changed for node `{node_id}` but no decision artefact covers it"
                ),
                node: Some(node_id.to_owned()),
                target: None,
                path: None,
            });
        }
    };

    // Added nodes.
    for id in current.nodes.keys() {
        if !previous.nodes.contains_key(id) {
            emit(id);
        }
    }
    // Removed nodes.
    for id in previous.nodes.keys() {
        if !current.nodes.contains_key(id) {
            emit(id);
        }
    }
    // Structural changes: parent or kind changed. Path-only changes are not gated.
    for (id, cur_fp) in &current.nodes {
        if let Some(prev_fp) = previous.nodes.get(id)
            && (cur_fp.parent != prev_fp.parent || cur_fp.kind != prev_fp.kind)
        {
            emit(id);
        }
    }
    // Dependency-edge drift. Schema v2 added per-node outbound-edge tracking; a
    // pre-v2 baseline recorded no edges, so skip until a v2 snapshot is written
    // to avoid flagging every edge as new on the first scan after upgrade.
    if previous.version >= 2 {
        for (id, cur_fp) in &current.nodes {
            if let Some(prev_fp) = previous.nodes.get(id)
                && cur_fp.edges != prev_fp.edges
            {
                emit(id);
            }
        }
    }
}

pub(crate) fn check_provenance_coverage(graph: &mut Graph, artefacts: &ArtefactSet) {
    if artefacts.decisions.is_empty() {
        return;
    }
    let covered: BTreeSet<&str> = artefacts
        .decisions
        .iter()
        .flat_map(|d| d.nodes.iter().map(String::as_str))
        .collect();
    for node in graph.nodes.values() {
        if node.children.is_empty() && !covered.contains(node.id.as_str()) {
            graph.findings.push(crate::map::graph::Finding {
                code: "CAIRN_PROVENANCE_NO_DECISION".to_owned(),
                severity: crate::map::graph::FindingSeverity::Warning,
                message: format!(
                    "node `{}` has no decision artefact explaining why it exists",
                    node.id
                ),
                node: Some(node.id.clone()),
                target: None,
                path: None,
            });
        }
    }
}

pub(crate) fn check_claims(graph: &mut Graph, artefacts: &ArtefactSet, root: &Path) {
    use std::collections::BTreeSet;
    for decision in &artefacts.decisions {
        let Some(claims) = &decision.claims else {
            continue;
        };
        if !matches!(claims.mode, crate::artefacts::ClaimsMode::Exhaustive) {
            continue;
        }
        let folder = root.join(&claims.folder);
        let actual: BTreeSet<String> = if let Ok(entries) = std::fs::read_dir(&folder) {
            entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|ft| ft.is_file()))
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        } else {
            graph.findings.push(crate::map::graph::Finding {
                code: "CA003".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "decision `{}` claims exhaustive file list for folder `{}` which does not exist or is unreadable",
                    decision.id, claims.folder
                ),
                node: Some(decision.nodes.first().cloned().unwrap_or_default()),
                target: None,
                path: Some(decision.path.clone()),
            });
            continue;
        };
        let claimed: BTreeSet<String> = claims.items.iter().cloned().collect();
        let missing: Vec<_> = actual.difference(&claimed).cloned().collect();
        let extra: Vec<_> = claimed.difference(&actual).cloned().collect();
        if !missing.is_empty() || !extra.is_empty() {
            let mut parts = Vec::new();
            if !missing.is_empty() {
                parts.push(format!("missing from claim: {}", missing.join(", ")));
            }
            if !extra.is_empty() {
                parts.push(format!("extra in claim: {}", extra.join(", ")));
            }
            graph.findings.push(crate::map::graph::Finding {
                code: "CA003".to_owned(),
                severity: crate::map::graph::FindingSeverity::Error,
                message: format!(
                    "decision `{}` exhaustive file claim for `{}` does not match actual contents: {}",
                    decision.id,
                    claims.folder,
                    parts.join("; ")
                ),
                node: Some(decision.nodes.first().cloned().unwrap_or_default()),
                target: None,
                path: Some(decision.path.clone()),
            });
        }
    }
}

pub(crate) fn check_gitignored_paths(graph: &mut Graph, ast: &blueprint::Ast, ignores: &[String]) {
    let mut emit_for = |node: &blueprint::Node| {
        for path in &node.paths {
            let rel = path.trim_start_matches("./").trim_start_matches('/');
            if config::is_ignored(rel, ignores) {
                graph.findings.push(crate::map::graph::Finding {
                    code: "CAIRN_PATH_GITIGNORED".to_owned(),
                    severity: crate::map::graph::FindingSeverity::Warning,
                    message: format!(
                        "node `{}` declares path `{path}` which matches a .gitignore pattern; will appear as a Ghost node",
                        node.id
                    ),
                    node: Some(node.id.clone()),
                    target: None,
                    path: Some(path.clone()),
                });
            }
        }
    };
    visit_nodes(&ast.nodes, &mut emit_for);
}

fn visit_nodes<F: FnMut(&blueprint::Node)>(nodes: &[blueprint::Node], f: &mut F) {
    let mut stack: Vec<&blueprint::Node> = nodes.iter().collect();
    while let Some(node) = stack.pop() {
        f(node);
        for child in &node.children {
            stack.push(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::{Node, NodeKind, Span};

    fn leaf(id: &str) -> Node {
        Node {
            kind: NodeKind::Module,
            name: id.to_owned(),
            description: String::new(),
            id: id.to_owned(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn node(id: &str, children: Vec<Node>) -> Node {
        Node {
            children,
            ..leaf(id)
        }
    }

    #[test]
    fn visit_nodes_collects_all_nodes_in_tree() {
        let tree = vec![node(
            "root",
            vec![
                node("child-a", vec![node("grandchild", Vec::new())]),
                node("child-b", Vec::new()),
            ],
        )];

        let mut visited = Vec::new();
        visit_nodes(&tree, &mut |n| visited.push(n.id.clone()));

        // Order is stack-based and not part of the contract; assert the set.
        visited.sort();
        assert_eq!(visited, vec!["child-a", "child-b", "grandchild", "root"]);
    }

    #[test]
    fn visit_nodes_empty_input_invokes_callback_never() {
        let mut visited = Vec::new();
        visit_nodes(&[], &mut |n| visited.push(n.id.clone()));
        assert!(visited.is_empty());
    }
}
