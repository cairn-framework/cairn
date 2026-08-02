//! Roadmap projection: live todos in dependency tiers
//! (`dec.todo-relationship-model` ruling 5).
//!
//! The roadmap is computed, never authored: todos whose status is not
//! `done`, tiered topologically from `blocked_by` alone (`parent` groups,
//! it never orders), grouped by `parent` inside a tier, ordered by the
//! shared `WorkItem` rank. With zero declared edges the projection
//! degenerates to one tier holding every live todo, which is correct and
//! renders. Members of a `blocked_by` cycle occupy one tier as a unit
//! (the projection tiers the cycle's condensation, and the scanner's
//! cycle Error names the members); their dependants tier strictly after.

// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::{relative_path, title_from_body, todo_status};
use super::super::*;
use super::work_item::WorkItem;
use std::collections::BTreeMap;

/// One live todo inside a roadmap tier.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct RoadmapItem {
    /// Canonical todo reference: the filename stem, `todo.<slug>`.
    pub stem: String,
    /// First level-one heading of the body, or a fallback.
    pub title: String,
    /// Referenced node.
    pub node: String,
    /// Todo status token.
    pub status: String,
    /// Root-relative source path.
    pub path: String,
    /// Containing todo stem; grouping only, never order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Declared blockers (todo stems).
    pub blocked_by: Vec<String>,
    /// Cross-source `WorkItem` rank; lower is more urgent.
    pub rank: u32,
}

/// One dependency tier: every member's live blockers sit in earlier tiers.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct RoadmapTier {
    /// 1-based tier number; tier 1 has no live blockers.
    pub tier: u32,
    /// Members ordered by rank, then parent group, then stem.
    pub items: Vec<RoadmapItem>,
}

/// Wire shape of the `roadmap` query response.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct RoadmapResponse {
    /// Tiers ordered shallowest first.
    pub tiers: Vec<RoadmapTier>,
}

/// Dispatch shim for the `roadmap` tool.
pub(crate) fn roadmap_json(root: &std::path::Path, scan_result: &scanner::ScanResult) -> Value {
    let response = roadmap_response(root, &scan_result.artefacts.todos);
    serde_json::to_value(response).expect("RoadmapResponse serialises")
}

/// Builds the projection from the loaded todo set alone.
pub(crate) fn roadmap_response(root: &std::path::Path, todos: &[Todo]) -> RoadmapResponse {
    let stems: BTreeMap<String, &Todo> = todos
        .iter()
        .filter(|todo| todo.status != TodoStatus::Done)
        .filter_map(|todo| stem_of(&todo.path).map(|stem| (stem, todo)))
        .collect();
    let tier_of = tier_numbers(&stems);
    let mut tiers: BTreeMap<u32, Vec<RoadmapItem>> = BTreeMap::new();
    for (stem, todo) in &stems {
        let item = RoadmapItem {
            stem: stem.clone(),
            title: title_from_body(&todo.body, "Todo"),
            node: todo.node.clone(),
            status: todo_status(todo.status).to_owned(),
            path: relative_path(&todo.path, root),
            parent: todo.parent.clone(),
            blocked_by: todo.blocked_by.clone(),
            rank: WorkItem::TODO_RANK,
        };
        tiers.entry(tier_of[stem.as_str()]).or_default().push(item);
    }
    let tiers = tiers
        .into_iter()
        .map(|(tier, mut items)| {
            items.sort_by(|a, b| {
                let a_group = a.parent.as_deref().unwrap_or(&a.stem);
                let b_group = b.parent.as_deref().unwrap_or(&b.stem);
                a.rank
                    .cmp(&b.rank)
                    .then_with(|| a_group.cmp(b_group))
                    .then_with(|| a.stem.cmp(&b.stem))
            });
            RoadmapTier { tier, items }
        })
        .collect();
    RoadmapResponse { tiers }
}

/// 1-based tier per live stem, computed over the strongly-connected-
/// component condensation of the live `blocked_by` graph: a cyclic
/// component occupies one tier as a unit (its members block each other,
/// so no order among them exists), and every dependant tiers strictly
/// after the deepest component it waits on. Blockers that are `done` or
/// dangling gate nothing. The condensation is always acyclic, so the
/// projection terminates and renders on any input.
fn tier_numbers<'a>(stems: &'a BTreeMap<String, &'a Todo>) -> BTreeMap<&'a str, u32> {
    let component = component_ids(stems);
    // Condensed adjacency and depth per component: edges between distinct
    // components only.
    let mut deps: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (stem, todo) in stems {
        let from = component[stem.as_str()];
        deps.entry(from).or_default();
        for target in &todo.blocked_by {
            if let Some(&to) = component.get(target.as_str())
                && to != from
            {
                deps.entry(from).or_default().insert(to);
            }
        }
    }
    let mut level: BTreeMap<usize, u32> = BTreeMap::new();
    loop {
        let mut settled_one = false;
        for (id, blockers) in &deps {
            if level.contains_key(id) {
                continue;
            }
            let deepest = blockers
                .iter()
                .map(|blocker| level.get(blocker).copied())
                .collect::<Option<Vec<u32>>>();
            if let Some(depths) = deepest {
                level.insert(*id, depths.into_iter().max().unwrap_or(0) + 1);
                settled_one = true;
            }
        }
        if !settled_one {
            break;
        }
    }
    stems
        .keys()
        .map(|stem| (stem.as_str(), level[&component[stem.as_str()]]))
        .collect()
}

/// Strongly-connected-component id per stem (Kosaraju: post-order over the
/// live graph, then a reverse-order sweep over its transpose).
fn component_ids<'a>(stems: &'a BTreeMap<String, &'a Todo>) -> BTreeMap<&'a str, usize> {
    let edges = |stem: &str| -> Vec<&'a str> {
        stems[stem]
            .blocked_by
            .iter()
            .filter(|target| stems.contains_key(*target))
            .map(String::as_str)
            .collect()
    };
    let mut order: Vec<&str> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for start in stems.keys() {
        if !seen.insert(start.as_str()) {
            continue;
        }
        let mut stack: Vec<(&str, usize)> = vec![(start.as_str(), 0)];
        while let Some((node, child)) = stack.last_mut() {
            let node = *node;
            if let Some(target) = edges(node).get(*child).copied() {
                *child += 1;
                if seen.insert(target) {
                    stack.push((target, 0));
                }
            } else {
                order.push(node);
                stack.pop();
            }
        }
    }
    let mut reversed: BTreeMap<&str, Vec<&str>> =
        stems.keys().map(|k| (k.as_str(), Vec::new())).collect();
    for stem in stems.keys() {
        for target in edges(stem) {
            reversed.entry(target).or_default().push(stem.as_str());
        }
    }
    let mut component: BTreeMap<&str, usize> = BTreeMap::new();
    let mut next = 0usize;
    for root in order.iter().rev() {
        if component.contains_key(root) {
            continue;
        }
        let mut stack = vec![*root];
        while let Some(node) = stack.pop() {
            if component.contains_key(node) {
                continue;
            }
            component.insert(node, next);
            stack.extend(reversed.get(node).into_iter().flatten().copied());
        }
        next += 1;
    }
    component
}

/// A todo's canonical reference: its filename stem.
fn stem_of(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_owned)
}

#[cfg(test)]
mod tests;
