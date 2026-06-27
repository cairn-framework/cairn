//! Validate change proposals against the current graph.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

/// Validates change references against current truth.
#[must_use]
pub fn validate_change(change: &Change, graph: &Graph) -> Vec<String> {
    let mut errors = change.findings.clone();
    let mut existing_nodes = graph.nodes.keys().cloned().collect::<BTreeSet<_>>();
    let mut added_nodes = BTreeSet::new();
    let mut touched_nodes = BTreeSet::new();
    for node in &change.delta.added_nodes {
        if !added_nodes.insert(node.id.clone()) || existing_nodes.contains(&node.id) {
            errors.push(format!("node `{}` has duplicate add operation", node.id));
        }
    }
    for rename in &change.delta.renamed_nodes {
        if !existing_nodes.contains(&rename.from) {
            errors.push(format!("renamed node `{}` does not exist", rename.from));
        }
        if existing_nodes.contains(&rename.to) || added_nodes.contains(&rename.to) {
            errors.push(format!(
                "renamed node target `{}` already exists",
                rename.to
            ));
        }
        mark_node_touch(&mut touched_nodes, &rename.from, &mut errors);
        existing_nodes.remove(&rename.from);
        existing_nodes.insert(rename.to.clone());
    }
    for id in &change.delta.removed_nodes {
        if !existing_nodes.contains(id) {
            errors.push(format!("removed node `{id}` does not exist"));
        }
        mark_node_touch(&mut touched_nodes, id, &mut errors);
        existing_nodes.remove(id);
    }
    for node in &change.delta.modified_nodes {
        if !existing_nodes.contains(&node.id) {
            errors.push(format!("modified node `{}` does not exist", node.id));
        }
        mark_node_touch(&mut touched_nodes, &node.id, &mut errors);
    }
    existing_nodes.extend(added_nodes);
    validate_edges(&change.delta, &existing_nodes, graph, &mut errors);
    // Artefact references resolve against the union of the pre- and post-delta
    // graphs: an added contract may reference a node the change adds, while a
    // removal's artefact may still reference the node being removed.
    let mut artefact_nodes = existing_nodes.clone();
    artefact_nodes.extend(graph.nodes.keys().cloned());
    validate_artefacts(&change.artefacts, &artefact_nodes, &mut errors);
    errors
}

pub(super) fn validate_edges(
    delta: &BlueprintDelta,
    available_nodes: &BTreeSet<String>,
    graph: &Graph,
    errors: &mut Vec<String>,
) {
    for edge in delta
        .added_edges
        .iter()
        .chain(delta.modified_edges.iter())
        .chain(delta.renamed_edges.iter().map(|rename| &rename.to))
    {
        if !available_nodes.contains(&edge.from) || !available_nodes.contains(&edge.to) {
            errors.push(format!(
                "edge `{}` -> `{}` references missing endpoint",
                edge.from, edge.to
            ));
        }
    }
    for edge in delta
        .removed_edges
        .iter()
        .chain(delta.modified_edges.iter())
        .chain(delta.renamed_edges.iter().map(|rename| &rename.from))
    {
        if !graph_edge_exists(graph, edge) {
            errors.push(format!(
                "edge `{}` -> `{}` ({}) does not exist",
                edge.from, edge.to, edge.description
            ));
        }
    }
}

pub(super) fn graph_edge_exists(graph: &Graph, edge: &Edge) -> bool {
    graph.outbound.get(&edge.from).is_some_and(|edges| {
        edges
            .iter()
            .any(|existing| existing.to == edge.to && existing.description == edge.description)
    })
}

pub(super) fn mark_node_touch(touched: &mut BTreeSet<String>, id: &str, errors: &mut Vec<String>) {
    if !touched.insert(id.to_owned()) {
        errors.push(format!("node `{id}` has conflicting operations"));
    }
}

pub(super) fn validate_artefacts(
    artefacts: &[ArtefactOperation],
    available_nodes: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let mut targets = BTreeSet::<PathBuf>::new();
    for artefact in artefacts {
        if !targets.insert(artefact.target_path.clone()) {
            errors.push(format!(
                "artefact `{}` has duplicate operations",
                artefact.target_path.display()
            ));
        }
        match artefact.operation {
            ChangeOperation::Added => {
                if artefact.target_path.exists() {
                    errors.push(format!(
                        "added artefact target `{}` already exists",
                        artefact.target_path.display()
                    ));
                }
            }
            ChangeOperation::Modified | ChangeOperation::Removed => {
                if !artefact.target_path.exists() {
                    errors.push(format!(
                        "{:?} artefact target `{}` does not exist",
                        artefact.operation,
                        artefact.target_path.display()
                    ));
                }
            }
            ChangeOperation::Renamed => {
                if artefact.renamed_from.is_none() {
                    errors.push(format!(
                        "renamed artefact `{}` is missing renamed_from",
                        artefact.change_path.display()
                    ));
                }
                if let Some(source) = &artefact.renamed_from
                    && !source.exists()
                {
                    errors.push(format!(
                        "renamed artefact source `{}` does not exist",
                        source.display()
                    ));
                }
            }
        }
        validate_artefact_refs(artefact, available_nodes, errors);
    }
}

pub(super) fn validate_artefact_refs(
    artefact: &ArtefactOperation,
    available_nodes: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let parsed = frontmatter::parse(&artefact.content);
    for key in ["node", "nodes"] {
        let ids = if key == "node" {
            parsed
                .values
                .get(key)
                .map(|value| vec![value.clone()])
                .unwrap_or_default()
        } else {
            parsed.lists.get(key).cloned().unwrap_or_default()
        };
        for id in ids {
            if !available_nodes.contains(&id) {
                errors.push(format!(
                    "artefact `{}` references unknown node `{id}`",
                    artefact.change_path.display()
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests;
