//! Todo relationship validation (`dec.todo-relationship-model` ruling 4).
//!
//! Resolves `blocked_by:`/`parent:` entries against loaded todo stems and
//! `related:` entries against the registry's resolvable identities, then
//! checks the two relationship graphs for cycles (per graph, never across
//! their union) and the status-contradiction advisory's two forms. The
//! decision artefact is the single normative copy of these semantics; this
//! module implements them without restating them.
//!
//! Cycles report one Error per cyclic strongly connected component,
//! naming every member. Elementary-cycle enumeration is deliberately
//! avoided: its output is exponential in authored input, and a component
//! keeps reporting on every scan until the whole knot is untangled, so
//! nothing is ever silently missing. Every finding carries a
//! field-qualified `target` so the scanner's post-load deduplication
//! (keyed on code, node, path, and target) collapses only genuine
//! duplicates, never two distinct references or components sharing a file.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::io::{error, info, warning};
use crate::map::graph::Finding;
use std::collections::BTreeMap;

/// Status and source path per loaded todo stem.
type StemIndex = BTreeMap<String, (TodoStatus, String)>;

/// Directed adjacency over resolved todo stems.
type Graph = BTreeMap<String, Vec<String>>;

/// Validates todo relationship references, cycles, and status
/// contradictions across the loaded todo set.
pub(super) fn validate_todo_relations(set: &mut ArtefactSet) {
    let stems: StemIndex = set
        .todos
        .iter()
        .filter_map(|todo| stem_of(&todo.path).map(|stem| (stem, (todo.status, todo.path.clone()))))
        .collect();
    let mut findings = Vec::new();
    reference_findings(set, &stems, &mut findings);
    cycle_findings(set, &stems, &mut findings);
    set.findings.append(&mut findings);
}

/// Dangling-reference Warnings plus the status-contradiction advisory for
/// every todo.
fn reference_findings(set: &ArtefactSet, stems: &StemIndex, findings: &mut Vec<Finding>) {
    let decision_ids: BTreeSet<&str> = set.decisions.iter().map(|d| d.id.as_str()).collect();
    let research_ids: BTreeSet<&str> = set.research.iter().map(|r| r.id.as_str()).collect();
    let source_ids: BTreeSet<&str> = set.sources.iter().map(|s| s.id.as_str()).collect();
    for todo in &set.todos {
        let Some(stem) = stem_of(&todo.path) else {
            continue;
        };
        for (field, target) in todo
            .blocked_by
            .iter()
            .map(|target| ("blocked_by", target))
            .chain(todo.parent.iter().map(|target| ("parent", target)))
        {
            if !stems.contains_key(target) {
                findings.push(unknown_target(&stem, field, target, todo));
            }
        }
        for target in &todo.related {
            let resolves = stems.contains_key(target)
                || decision_ids.contains(target.as_str())
                || research_ids.contains(target.as_str())
                || source_ids.contains(target.as_str());
            if !resolves {
                findings.push(unknown_target(&stem, "related", target, todo));
            }
        }
        findings.extend(contradiction(todo, &stem, stems));
    }
}

fn unknown_target(stem: &str, field: &str, target: &str, todo: &Todo) -> Finding {
    let mut finding = warning(
        "CAIRN_TODO_RELATION_UNKNOWN",
        format!("todo `{stem}` names unknown {field} target `{target}`"),
        None,
        Some(todo.path.clone()),
    );
    finding.target = Some(format!("{field}:{target}"));
    finding
}

/// One Error per cyclic component, detected per graph, never across their
/// union (`dec.todo-relationship-model` ruling 4).
fn cycle_findings(set: &ArtefactSet, stems: &StemIndex, findings: &mut Vec<Finding>) {
    let blocked_edges = edges(set, stems, |todo| todo.blocked_by.clone());
    let parent_edges = edges(set, stems, |todo| todo.parent.clone().into_iter().collect());
    for (field, graph) in [("blocked_by", blocked_edges), ("parent", parent_edges)] {
        for members in cyclic_components(&graph) {
            let listed = members.join(", ");
            let mut finding = error(
                "CAIRN_TODO_RELATION_CYCLE",
                format!("todos form a `{field}` cycle involving: {listed}"),
                None,
                stems.get(&members[0]).map(|(_, path)| path.clone()),
            );
            finding.target = Some(format!("{field}:{listed}"));
            findings.push(finding);
        }
    }
}

/// The status-contradiction advisory, two forms (`dec.todo-relationship-model`
/// ruling 4). A blocker is unresolved while its status is anything but
/// `done`; a dangling blocker has no status and triggers neither form (its
/// Warning already fired). Only `open` is flagged on the downstream side.
fn contradiction(todo: &Todo, stem: &str, stems: &StemIndex) -> Option<Finding> {
    let status_of = |target: &String| stems.get(target).map(|(status, _)| *status);
    match todo.status {
        TodoStatus::Blocked => {
            let all_done = !todo.blocked_by.is_empty()
                && todo
                    .blocked_by
                    .iter()
                    .all(|target| status_of(target) == Some(TodoStatus::Done));
            all_done.then(|| {
                info(
                    "CAIRN_TODO_STATUS_CONTRADICTION",
                    format!("todo `{stem}` is blocked while every declared blocker is done"),
                    None,
                    Some(todo.path.clone()),
                )
            })
        }
        TodoStatus::Open => todo
            .blocked_by
            .iter()
            .find(|target| status_of(target).is_some_and(|status| status != TodoStatus::Done))
            .map(|target| {
                info(
                    "CAIRN_TODO_STATUS_CONTRADICTION",
                    format!("todo `{stem}` is open while declared blocker `{target}` is not done"),
                    None,
                    Some(todo.path.clone()),
                )
            }),
        TodoStatus::InProgress | TodoStatus::Done => None,
    }
}

/// Adjacency over resolved stems only: a dangling reference cannot form a
/// cycle, and its Warning already covers it.
fn edges(set: &ArtefactSet, stems: &StemIndex, field: impl Fn(&Todo) -> Vec<String>) -> Graph {
    set.todos
        .iter()
        .filter_map(|todo| {
            let stem = stem_of(&todo.path)?;
            let targets: Vec<String> = field(todo)
                .into_iter()
                .filter(|target| stems.contains_key(target))
                .collect();
            Some((stem, targets))
        })
        .collect()
}

/// Members of every cyclic strongly connected component (two or more
/// members, or one member with a self-edge), each sorted, components
/// ordered by smallest member. Kosaraju's two-pass sweep: a post-order
/// over the graph, then a reverse-order sweep over its transpose.
fn cyclic_components(graph: &Graph) -> Vec<Vec<String>> {
    let order = post_order(graph);
    let transpose = transpose(graph);
    let mut assigned: BTreeSet<&str> = BTreeSet::new();
    let mut cyclic: Vec<Vec<String>> = Vec::new();
    for root in order.iter().rev() {
        if assigned.contains(root.as_str()) {
            continue;
        }
        // Every node reachable in the transpose that finished no later
        // than `root` belongs to root's component.
        let mut members: Vec<&str> = Vec::new();
        let mut stack = vec![root.as_str()];
        while let Some(node) = stack.pop() {
            if !assigned.insert(node) {
                continue;
            }
            members.push(node);
            stack.extend(
                transpose
                    .get(node)
                    .into_iter()
                    .flatten()
                    .map(String::as_str),
            );
        }
        let self_loop =
            members.len() == 1 && graph[members[0]].iter().any(|next| next == members[0]);
        if members.len() > 1 || self_loop {
            let mut owned: Vec<String> = members.iter().map(|m| (*m).to_owned()).collect();
            owned.sort_unstable();
            cyclic.push(owned);
        }
    }
    cyclic.sort_unstable();
    cyclic
}

/// Iterative depth-first post-order over every node.
fn post_order(graph: &Graph) -> Vec<String> {
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let mut order: Vec<String> = Vec::new();
    for start in graph.keys() {
        if visited.contains(start.as_str()) {
            continue;
        }
        let mut stack: Vec<(&str, usize)> = vec![(start.as_str(), 0)];
        visited.insert(start.as_str());
        while let Some((node, child)) = stack.last_mut() {
            let node = *node;
            if let Some(target) = graph[node].get(*child) {
                *child += 1;
                if visited.insert(target.as_str()) {
                    stack.push((target.as_str(), 0));
                }
            } else {
                order.push(node.to_owned());
                stack.pop();
            }
        }
    }
    order
}

/// Edge-reversed copy of the graph.
fn transpose(graph: &Graph) -> Graph {
    let mut reversed: Graph = graph.keys().map(|k| (k.clone(), Vec::new())).collect();
    for (from, targets) in graph {
        for target in targets {
            reversed
                .entry(target.clone())
                .or_default()
                .push(from.clone());
        }
    }
    reversed
}

/// A todo's canonical reference: its filename stem, `todo.<slug>`
/// (`dec.todo-relationship-model` ruling 2).
fn stem_of(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_owned)
}

#[cfg(test)]
mod tests;
