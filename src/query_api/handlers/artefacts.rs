//! Artefact query handlers for todos, decisions, research, and sources.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::serialise::*;
use super::super::util::*;
use super::super::*;

pub(crate) fn todos_response_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let status = request.status.as_deref().and_then(parse_todo_status_filter);
    // No node: project-wide listing (`node` is null on the wire). With a
    // node: the node's own todos plus all descendants', where descent
    // follows the graph's containment (`children`) edges.
    let (node_id, scope) = match request.node.as_ref() {
        Some(node) => {
            let node = scan_result.graph.resolve(node).map_err(finding_error)?;
            (
                json!(node.id),
                Some(containment_scope(&scan_result.graph, &node.id)),
            )
        }
        None => (Value::Null, None),
    };
    let selected: Vec<&Todo> = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| {
            scope.as_ref().is_none_or(|ids| ids.contains(&todo.node))
                && status.is_none_or(|filter| todo.status == filter)
        })
        .collect();
    let relation_statuses = relation_statuses(&scan_result.artefacts, &selected);
    let todos = selected
        .into_iter()
        .map(|todo| todo_enriched_json(todo, root))
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node_id,
        "todos": todos,
        "relation_statuses": relation_statuses,
    }))
}

/// Collects `id` plus every descendant reachable through the graph's
/// containment (`children`) edges.
fn containment_scope(graph: &crate::map::Graph, id: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut stack = vec![id.to_owned()];
    while let Some(current) = stack.pop() {
        if ids.insert(current.clone())
            && let Some(node) = graph.nodes.get(&current)
        {
            stack.extend(node.children.iter().cloned());
        }
    }
    ids
}

/// Status token for every resolvable relationship target named by the
/// selected todos: todo stems map to todo status, decision ids to decision
/// status (`dec.todo-relationship-model` ruling 2's resolvable set, less
/// research and sources, which carry no status). Lets renderers annotate
/// edges without a second scan.
fn relation_statuses(
    artefacts: &crate::artefacts::registry::ArtefactSet,
    selected: &[&Todo],
) -> BTreeMap<String, String> {
    let todo_statuses: BTreeMap<&str, &'static str> = artefacts
        .todos
        .iter()
        .filter_map(|todo| {
            std::path::Path::new(&todo.path)
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .map(|stem| (stem, todo_status(todo.status)))
        })
        .collect();
    let decision_statuses: BTreeMap<&str, &'static str> = artefacts
        .decisions
        .iter()
        .map(|decision| (decision.id.as_str(), decision_status(decision.status)))
        .collect();
    let mut out = BTreeMap::new();
    for todo in selected {
        for target in todo
            .blocked_by
            .iter()
            .chain(todo.parent.iter())
            .chain(todo.related.iter())
        {
            if out.contains_key(target) {
                continue;
            }
            if let Some(status) = todo_statuses
                .get(target.as_str())
                .or_else(|| decision_statuses.get(target.as_str()))
            {
                out.insert(target.clone(), (*status).to_owned());
            }
        }
    }
    out
}

pub(crate) fn decisions_response_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(
            request.node.as_ref(),
            "CAIRN_QUERY_MISSING_NODE",
            "node",
        )?)
        .map_err(finding_error)?;
    let status = request
        .status
        .as_deref()
        .and_then(parse_decision_status_filter);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.nodes.contains(&node.id)
                && status.is_none_or(|filter| decision.status == filter)
        })
        .map(|decision| decision_enriched_json(decision, root))
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node.id,
        "decisions": decisions,
        "decision_index": decision_index_json(&scan_result.artefacts.decisions),
    }))
}

pub(crate) fn research_response_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(|research| research_enriched_json(research, root))
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "research": research }))
}

pub(crate) fn sources_response_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(|source| source_enriched_json(source, root))
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "sources": sources }))
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path};

    use super::*;
    use crate::{
        artefacts::{
            contract::ContractSet,
            registry::{ArtefactSet, Todo, TodoStatus},
        },
        blueprint::{Ast, Node, NodeKind, Span},
        map::build_graph,
        scanner::ScanResult,
    };

    fn leaf(id: &str) -> Node {
        Node {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("test.blueprint", 1, 1),
        }
    }

    fn todo(path: &str, node: &str, status: TodoStatus) -> Todo {
        Todo {
            path: path.to_owned(),
            node: node.to_owned(),
            status,
            created: "2026-07-16".to_owned(),
            satisfies: None,
            blocked_by: Vec::new(),
            parent: None,
            related: Vec::new(),
            defers: Vec::new(),
            body: "# Todo".to_owned(),
        }
    }

    /// `app` contains `app.kernel`, which contains `app.kernel.cli`;
    /// `app.other` is a sibling of `app.kernel`. One todo per node except
    /// `app`, plus a done todo on the container itself.
    fn scan_with_todo_tree() -> ScanResult {
        let mut kernel = leaf("app.kernel");
        kernel.children = vec![leaf("app.kernel.cli")];
        let mut app = leaf("app");
        app.kind = NodeKind::System;
        app.children = vec![kernel, leaf("app.other")];
        let ast = Ast {
            nodes: vec![app],
            edges: Vec::new(),
        };
        let contracts = ContractSet::default();
        let mut claimed = BTreeMap::new();
        let graph = build_graph(&ast, Path::new("."), &contracts, &mut claimed, Vec::new());
        ScanResult {
            graph,
            target_hashes: BTreeMap::new(),
            interface_hash: String::new(),
            blueprint_snapshot: crate::scanner::state::BlueprintSnapshot::default(),
            target_reports: Vec::new(),
            contracts,
            artefacts: ArtefactSet {
                todos: vec![
                    todo("meta/todos/todo.kernel.md", "app.kernel", TodoStatus::Open),
                    todo(
                        "meta/todos/todo.kernel-done.md",
                        "app.kernel",
                        TodoStatus::Done,
                    ),
                    todo("meta/todos/todo.cli.md", "app.kernel.cli", TodoStatus::Open),
                    todo("meta/todos/todo.other.md", "app.other", TodoStatus::Open),
                ],
                ..ArtefactSet::default()
            },
        }
    }

    fn request(node: Option<&str>, status: Option<&str>) -> QueryRequest {
        QueryRequest {
            tool: "todos".to_owned(),
            node: node.map(ToOwned::to_owned),
            status: status.map(ToOwned::to_owned),
            ..QueryRequest::default()
        }
    }

    fn todo_nodes(data: &Value) -> Vec<String> {
        data["todos"]
            .as_array()
            .expect("todos must be an array")
            .iter()
            .map(|todo| todo["node"].as_str().unwrap_or_default().to_owned())
            .collect()
    }

    #[test]
    fn todos_without_node_lists_project_wide() {
        let scan = scan_with_todo_tree();
        let data = todos_response_json(Path::new("."), &scan, &request(None, None))
            .expect("bare todos must succeed");
        assert!(data["node"].is_null(), "project-wide node must be null");
        assert_eq!(
            todo_nodes(&data),
            vec!["app.kernel", "app.kernel", "app.kernel.cli", "app.other"]
        );
    }

    #[test]
    fn todos_without_node_honours_status_filter() {
        let scan = scan_with_todo_tree();
        let data = todos_response_json(Path::new("."), &scan, &request(None, Some("done")))
            .expect("bare todos with status must succeed");
        assert_eq!(todo_nodes(&data), vec!["app.kernel"]);
    }

    #[test]
    fn todos_for_container_include_descendants() {
        let scan = scan_with_todo_tree();
        let data = todos_response_json(Path::new("."), &scan, &request(Some("app.kernel"), None))
            .expect("container todos must succeed");
        assert_eq!(data["node"], "app.kernel");
        assert_eq!(
            todo_nodes(&data),
            vec!["app.kernel", "app.kernel", "app.kernel.cli"],
            "container listing must include its own todos plus descendants', not siblings'"
        );
    }

    #[test]
    fn todos_for_leaf_list_only_that_node() {
        let scan = scan_with_todo_tree();
        let data = todos_response_json(
            Path::new("."),
            &scan,
            &request(Some("app.kernel.cli"), None),
        )
        .expect("leaf todos must succeed");
        assert_eq!(todo_nodes(&data), vec!["app.kernel.cli"]);
    }

    #[test]
    fn todos_unknown_node_still_errors() {
        let scan = scan_with_todo_tree();
        let err = todos_response_json(Path::new("."), &scan, &request(Some("app.bogus"), None))
            .expect_err("unknown node must error");
        assert_eq!(err.code, "CAIRN_QUERY_NODE_NOT_FOUND");
    }
}
