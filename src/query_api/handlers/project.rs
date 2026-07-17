//! Project-wide status and context query handlers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::*;
use super::graph::count_findings;
use super::next_selection::{select_next, work_item_for_selection};

pub(crate) fn status_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
) -> Value {
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
    let next_recommended = work_item_for_selection(&select_next(root, changes_dir, scan_result))
        .map_or(Value::Null, |item| {
            serde_json::to_value(item).expect("WorkItem serialises")
        });
    let active_changes = crate::changes::discover(root, changes_dir)
        .unwrap_or_default()
        .iter()
        .map(|change| {
            json!({
                "id": change.id,
                "title": change.title,
                "summary": crate::changes::operation_summary(change),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "active_changes": active_changes,
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
    let backlog_ready: Vec<Value> = ready.iter().map(|item| item.to_json()).collect();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_json_next_recommended_matches_shared_selection() {
        let tmp =
            std::env::temp_dir().join(format!("cairn-status-selection-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".beads")).unwrap();
        std::fs::create_dir_all(tmp.join("meta/todos")).unwrap();
        std::fs::write(
            tmp.join("cairn.blueprint"),
            "System Test \"T\" id \"t\" {\n    Container Work \"work\" id \"t.work\" {\n        path \"./meta/todos\"\n        todos \"./meta/todos\"\n    }\n}\n",
        )
        .unwrap();
        std::fs::write(
            tmp.join("meta/todos/todo.next.md"),
            "---\nnode: t.work\nstatus: open\ncreated: 2026-01-01\n---\n# Do todo\n",
        )
        .unwrap();
        std::fs::write(
            tmp.join(".beads/issues.jsonl"),
            "{\"id\":\"cairn-aaa\",\"title\":\"Do thing\",\"status\":\"open\",\"priority\":2}\n",
        )
        .unwrap();
        let changes = tmp.join("meta/changes");
        let scan = crate::scanner::load_project(&tmp, &tmp.join("cairn.blueprint")).unwrap();
        let status = status_json(&tmp, &changes, &scan);
        let expected = work_item_for_selection(&select_next(&tmp, &changes, &scan))
            .map_or(Value::Null, |item| serde_json::to_value(item).unwrap());
        assert_eq!(status["next_recommended"], expected);
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
