//! Roadmap projection: live todos in dependency tiers
//! (`dec.todo-relationship-model` ruling 5).
//!
//! The roadmap is computed, never authored: todos whose status is not
//! `done`, tiered topologically from `blocked_by` alone (`parent` groups,
//! it never orders), grouped by `parent` inside a tier, ordered by the
//! shared `WorkItem` rank. With zero declared edges the projection
//! degenerates to one tier holding every live todo, which is correct and
//! renders. Members of a `blocked_by` cycle cannot be tiered; they land
//! together in one final tier and the scanner's cycle Error names them.

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
    let live: Vec<&Todo> = todos
        .iter()
        .filter(|todo| todo.status != TodoStatus::Done)
        .collect();
    let stems: BTreeMap<String, &Todo> = live
        .iter()
        .filter_map(|todo| stem_of(&todo.path).map(|stem| (stem, *todo)))
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
            rank: WorkItem::from_todo(todo).rank,
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

/// 1-based tier per live stem: 1 + the deepest tier among its live
/// blockers, computed by repeated relaxation. Blockers that are `done` or
/// dangling gate nothing. Any stems never settled (a `blocked_by` cycle)
/// share one final tier so the projection always terminates and renders.
fn tier_numbers<'a>(stems: &'a BTreeMap<String, &'a Todo>) -> BTreeMap<&'a str, u32> {
    let mut tier: BTreeMap<&str, u32> = BTreeMap::new();
    loop {
        let mut settled_one = false;
        for (stem, todo) in stems {
            if tier.contains_key(stem.as_str()) {
                continue;
            }
            let live_blockers: Vec<&str> = todo
                .blocked_by
                .iter()
                .filter(|target| stems.contains_key(*target))
                .map(String::as_str)
                .collect();
            let deepest = live_blockers
                .iter()
                .map(|blocker| tier.get(blocker).copied())
                .collect::<Option<Vec<u32>>>();
            if let Some(depths) = deepest {
                tier.insert(stem, depths.into_iter().max().unwrap_or(0) + 1);
                settled_one = true;
            }
        }
        if !settled_one {
            break;
        }
    }
    // Cycle members never settle; lump them one past the deepest tier.
    let overflow = tier.values().max().copied().unwrap_or(0) + 1;
    for stem in stems.keys() {
        tier.entry(stem.as_str()).or_insert(overflow);
    }
    tier
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
