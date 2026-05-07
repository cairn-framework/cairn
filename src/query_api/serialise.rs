// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn node_json(node: &NodeRecord) -> Value {
    json!({
        "id": node.id,
        "kind": format!("{:?}", node.kind),
        "name": node.name,
        "description": node.description,
        "tags": node.tags,
        "parent": node.parent,
        "children": node.children,
        "paths": node.paths,
        "owns_files": node.owns_files,
        "contracts": node.contracts,
        "state": format!("{:?}", node.state),
        "files": node.files,
        "span": {
            "file": node.span.file,
            "line": node.span.line,
            "column": node.span.column,
            "end_line": node.span.end_line,
            "end_column": node.span.end_column,
        },
    })
}

pub(super) fn todo_json(todo: &Todo) -> Value {
    json!({
        "path": todo.path,
        "node": todo.node,
        "status": todo_status(todo.status),
        "created": todo.created,
        "satisfies": todo.satisfies,
    })
}

pub(super) fn decision_json(decision: &Decision) -> Value {
    json!({
        "id": decision.id,
        "status": decision_status(decision.status),
        "nodes": decision.nodes,
        "informed_by": decision.informed_by,
        "supersedes": decision.supersedes,
        "refines": decision.refines,
        "related": decision.related,
    })
}

pub(super) fn research_json(research: &Research) -> Value {
    json!({
        "id": research.id,
        "nodes": research.nodes,
        "sources": research.sources,
        "date": research.date,
    })
}

pub(super) fn review_json(review: &Review) -> Value {
    json!({
        "path": review.path,
        "node": review.node,
        "review_type": format!("{:?}", review.review_type),
        "date": review.date,
        "reviewer": review.reviewer,
    })
}

pub(super) fn source_json(source: &Source) -> Value {
    json!({
        "id": source.id,
        "file": source.file,
        "verification": source_verification(source.verification),
        "type": source.source_type,
        "date": source.date,
    })
}

pub(super) fn findings_json(findings: &[Finding]) -> Vec<Value> {
    findings
        .iter()
        .map(|finding| {
            json!({
                "code": finding.code,
                "severity": finding.severity.name(),
                "message": finding.message,
                "node": finding.node,
                "path": finding.path,
            })
        })
        .collect()
}

pub(super) fn neighbourhood_ids(graph: &crate::map::Graph, node: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::from([node.to_owned()]);
    if let Some(edges) = graph.inbound.get(node) {
        ids.extend(edges.iter().map(|edge| edge.from.clone()));
    }
    if let Some(edges) = graph.outbound.get(node) {
        ids.extend(edges.iter().map(|edge| edge.to.clone()));
    }
    ids
}

pub(super) fn research_for_nodes(
    scan_result: &scanner::ScanResult,
    nodes: &BTreeSet<String>,
) -> Vec<Research> {
    scan_result
        .artefacts
        .research
        .iter()
        .filter(|research| research.nodes.iter().any(|node| nodes.contains(node)))
        .cloned()
        .collect()
}

pub(super) fn sources_for_nodes(
    scan_result: &scanner::ScanResult,
    nodes: &BTreeSet<String>,
) -> Vec<Source> {
    let source_ids = scan_result
        .artefacts
        .research
        .iter()
        .filter(|research| research.nodes.iter().any(|node| nodes.contains(node)))
        .flat_map(|research| research.sources.iter().cloned())
        .chain(
            scan_result
                .artefacts
                .decisions
                .iter()
                .filter(|decision| decision.nodes.iter().any(|node| nodes.contains(node)))
                .flat_map(|decision| decision.informed_by.iter().cloned()),
        )
        .collect::<BTreeSet<_>>();
    scan_result
        .artefacts
        .sources
        .iter()
        .filter(|source| source_ids.contains(&source.id))
        .cloned()
        .collect()
}

pub(super) fn relevant_rules(
    rules: &BTreeMap<String, String>,
    tool: &str,
) -> BTreeMap<String, String> {
    let key = match tool.strip_prefix("cairn_").unwrap_or(tool) {
        "todos" => Some("todo"),
        "decisions" | "rationale" => Some("decision"),
        "research" => Some("research"),
        "sources" => Some("source"),
        "contract" => Some("contract"),
        "show_change" | "changes" => Some("change"),
        _ => None,
    };
    key.and_then(|key| rules.get(key).map(|value| (key.to_owned(), value.clone())))
        .into_iter()
        .collect()
}

pub(super) fn requires_valid_map(command: &str) -> bool {
    matches!(
        command,
        "get"
            | "neighbourhood"
            | "files"
            | "dependents"
            | "depends"
            | "contract"
            | "docstring"
            | "order"
            | "todos"
            | "decisions"
            | "research"
            | "sources"
            | "rationale"
            | "status"
    )
}

pub(super) fn parse_todo_status_filter(value: &str) -> Option<TodoStatus> {
    match value {
        "open" => Some(TodoStatus::Open),
        "in_progress" => Some(TodoStatus::InProgress),
        "done" => Some(TodoStatus::Done),
        "blocked" => Some(TodoStatus::Blocked),
        _ => None,
    }
}

pub(super) fn parse_decision_status_filter(value: &str) -> Option<DecisionStatus> {
    match value {
        "proposed" => Some(DecisionStatus::Proposed),
        "accepted" => Some(DecisionStatus::Accepted),
        "deprecated" => Some(DecisionStatus::Deprecated),
        "superseded" => Some(DecisionStatus::Superseded),
        _ => None,
    }
}

pub(super) const fn todo_status(status: TodoStatus) -> &'static str {
    match status {
        TodoStatus::Open => "open",
        TodoStatus::InProgress => "in_progress",
        TodoStatus::Done => "done",
        TodoStatus::Blocked => "blocked",
    }
}

pub(super) const fn decision_status(status: DecisionStatus) -> &'static str {
    match status {
        DecisionStatus::Proposed => "proposed",
        DecisionStatus::Accepted => "accepted",
        DecisionStatus::Deprecated => "deprecated",
        DecisionStatus::Superseded => "superseded",
    }
}

pub(super) const fn source_verification(verification: SourceVerification) -> &'static str {
    match verification {
        SourceVerification::Verified => "verified",
        SourceVerification::External => "external",
        SourceVerification::Unverified => "unverified",
    }
}

pub(super) const fn hook_kind_name(kind: HookKind) -> &'static str {
    match kind {
        HookKind::Structural => "structural",
        HookKind::Interface => "interface",
        HookKind::Tension => "tension",
        HookKind::All => "all",
    }
}

pub(super) const fn hook_decision_name(decision: ExitDecision) -> &'static str {
    match decision {
        ExitDecision::Pass => "pass",
        ExitDecision::Block => "block",
    }
}
