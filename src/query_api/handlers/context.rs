//! Session-continuity context projection: the structured overview plus the
//! shared "waiting on you" and "where work was left" surfaces.
use std::path::Path;

use serde_json::{Value, json};

use super::super::scanner;
use super::graph::count_findings;
use super::{select_next, work_item_for_selection};

/// Shared "where work was left" projection for both context surfaces:
/// in-progress native todos, active change directories, and in-progress
/// backlog beads, so a session opens where the last one stopped.
pub(crate) fn where_left(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> Value {
    let in_progress: Vec<Value> = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == crate::artefacts::registry::TodoStatus::InProgress)
        .map(|todo| {
            json!({
                "stem": std::path::Path::new(&todo.path)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or(&todo.path),
                "node": todo.node,
                "path": todo.path,
            })
        })
        .collect();
    let active_changes: Vec<Value> = crate::changes::discover(root, changes_dir)
        .unwrap_or_default()
        .iter()
        .map(|change| json!({ "id": change.id, "title": change.title }))
        .collect();
    // Beads stay a read-only derived view (dec.beads-task-layer): an
    // in-progress bead is active work and belongs in the continuity block.
    let in_progress_backlog: Vec<Value> = crate::state::backlog::read(root)
        .iter()
        .filter(|item| item.status == "in_progress")
        .map(|item| json!({ "id": item.id, "title": item.title }))
        .collect();
    json!({
        "in_progress": in_progress,
        "active_changes": active_changes,
        "in_progress_backlog": in_progress_backlog,
    })
}

pub(crate) fn context_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
    config: &scanner::config::Config,
) -> Result<Value, super::super::QueryError> {
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

    let nodes = context_nodes(scan_result);
    let edges = context_edges(scan_result);

    let (errors, warnings, info) = count_findings(&scan_result.graph.findings);
    let backlog = crate::state::backlog::read(root);
    let ready = crate::state::backlog::ready(&backlog);
    let backlog_ready: Vec<Value> = ready.iter().map(|item| item.to_json()).collect();
    let pending = super::pending::pending_rows(root, scan_result)?;
    let pending_briefings: Vec<Value> = pending
        .iter()
        .map(|row| serde_json::to_value(row).expect("PendingDecision serialises"))
        .collect();
    let next_recommended = work_item_for_selection(&select_next(root, changes_dir, scan_result));
    let left = where_left(root, changes_dir, scan_result);

    Ok(json!({
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
        "next_recommended": next_recommended.clone(),
        "waiting_on_you": {
            "pending": pending_briefings,
            "where_left": left,
            "next_recommended": next_recommended,
        },
    }))
}
fn context_nodes(scan_result: &scanner::ScanResult) -> Vec<Value> {
    scan_result
        .graph
        .nodes
        .values()
        .map(|node| {
            let kind = match node.kind {
                crate::blueprint::ast::NodeKind::System => "system",
                crate::blueprint::ast::NodeKind::Container => "container",
                crate::blueprint::ast::NodeKind::Module => "module",
                crate::blueprint::ast::NodeKind::Actor => "actor",
            };
            let state = match node.state {
                crate::map::graph::NodeState::Synced => "synced",
                crate::map::graph::NodeState::Ghost => "ghost",
                crate::map::graph::NodeState::Orphaned => "orphaned",
            };
            json!({
                "id": node.id,
                "name": node.name,
                "kind": kind,
                "state": state,
                "paths": node.paths,
                "children": node.children,
            })
        })
        .collect()
}

fn context_edges(scan_result: &scanner::ScanResult) -> Vec<Value> {
    scan_result
        .graph
        .outbound
        .values()
        .flatten()
        .map(|edge| {
            json!({
                "source": edge.from,
                "target": edge.to,
                "label": edge.description,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_json_includes_pending_briefing_and_recommendation() {
        let tmp =
            std::env::temp_dir().join(format!("cairn-context-pending-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join("meta/decisions")).unwrap();
        std::fs::write(
            tmp.join("cairn.blueprint"),
            "System Test \"T\" id \"t\" {\n    decisions \"./meta/decisions\"\n}\n",
        )
        .unwrap();
        std::fs::write(
            tmp.join("meta/decisions/dec.waiting.md"),
            "---\nid: dec.waiting\nnodes: [t]\nstatus: proposed\nratification: binding\ndate: 2026-01-01\n---\n# Waiting\n\n## Decision\n\nKeep the queue clear.\n",
        )
        .unwrap();
        let scan = crate::scanner::load_project(&tmp, &tmp.join("cairn.blueprint")).unwrap();
        let context = context_json(
            &tmp,
            &tmp.join("meta/changes"),
            &scan,
            &scanner::config::Config::default(),
        )
        .unwrap();
        let briefing = &context["waiting_on_you"]["pending"][0];
        assert_eq!(briefing["id"], "dec.waiting");
        assert_eq!(briefing["ruling_summary"], "Keep the queue clear.");
        assert!(briefing["age_days"].as_i64().is_some_and(|age| age > 0));
        assert_eq!(briefing["reopen_command"], "cairn pending dec.waiting");
        assert!(
            briefing["ruling_prompt"]
                .as_str()
                .is_some_and(|prompt| !prompt.is_empty()),
            "wire carries the ruling instruction"
        );
        // Continuity fixtures and assertions live in tests/pending_queue.rs
        // (context_opens_with_where_work_was_left_on_both_surfaces); this
        // test owns the briefing serialization only.
        assert!(context["waiting_on_you"]["where_left"].is_object());
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
