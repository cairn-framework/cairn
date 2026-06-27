//! Project-wide status and context query handlers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::*;
use super::graph::count_findings;

pub(crate) fn status_json(root: &Path, scan_result: &scanner::ScanResult) -> Value {
    let open = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open || todo.status == TodoStatus::InProgress)
        .map(todo_json)
        .collect::<Vec<_>>();
    let log_entries: Vec<String> = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            content
                .lines()
                .rev()
                .take(5)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let backlog = crate::state::backlog::read(root);
    let next_recommended = crate::state::backlog::ready(&backlog)
        .first()
        .map_or(Value::Null, |top| backlog_item_json(top));
    json!({
        "active_changes": [],
        "open_todos": open,
        "recent_log_entries": log_entries,
        "next_recommended": next_recommended,
    })
}

pub(crate) fn context_json(
    root: &Path,
    scan_result: &scanner::ScanResult,
    config: &scanner::config::Config,
) -> Value {
    let system_name = scan_result
        .graph
        .nodes
        .values()
        .find(|n| n.kind == crate::blueprint::ast::NodeKind::System)
        .map_or("unknown", |n| n.name.as_str());

    let edge_count: usize = scan_result.graph.outbound.values().map(Vec::len).sum();

    let system_description = scan_result
        .graph
        .nodes
        .values()
        .find(|n| n.kind == crate::blueprint::ast::NodeKind::System)
        .map_or("", |n| n.description.as_str());

    let nodes: Vec<Value> = scan_result
        .graph
        .nodes
        .values()
        .map(|n| {
            let kind = match n.kind {
                crate::blueprint::ast::NodeKind::System => "system",
                crate::blueprint::ast::NodeKind::Container => "container",
                crate::blueprint::ast::NodeKind::Module => "module",
                crate::blueprint::ast::NodeKind::Actor => "actor",
            };
            let state = match n.state {
                crate::map::graph::NodeState::Synced => "synced",
                crate::map::graph::NodeState::Ghost => "ghost",
                crate::map::graph::NodeState::Orphaned => "orphaned",
            };
            json!({
                "id": n.id,
                "name": n.name,
                "kind": kind,
                "state": state,
                "paths": n.paths,
                "children": n.children,
            })
        })
        .collect();

    let edges: Vec<Value> = scan_result
        .graph
        .outbound
        .values()
        .flatten()
        .map(|e| {
            json!({
                "source": e.from,
                "target": e.to,
                "label": e.description,
            })
        })
        .collect();

    let (errors, warnings, info) = count_findings(&scan_result.graph.findings);
    let backlog = crate::state::backlog::read(root);
    let ready = crate::state::backlog::ready(&backlog);
    let backlog_ready: Vec<Value> = ready.iter().map(|item| backlog_item_json(item)).collect();

    json!({
        "system_name": system_name,
        "system_description": system_description,
        "project_context": config.context,
        "node_count": scan_result.graph.nodes.len(),
        "edge_count": edge_count,
        "nodes": nodes,
        "edges": edges,
        "artefact_counts": {
            "contracts": scan_result.artefacts.contracts.contracts.len(),
            "decisions": scan_result.artefacts.decisions.len(),
            "todos": scan_result.artefacts.todos.len(),
            "research": scan_result.artefacts.research.len(),
            "reviews": scan_result.artefacts.reviews.len(),
            "sources": scan_result.artefacts.sources.len(),
        },
        "finding_counts": {
            "error": errors,
            "warning": warnings,
            "info": info,
        },
        "backlog": {
            "ready_count": ready.len(),
            "ready": backlog_ready,
        },
    })
}
