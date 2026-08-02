//! JSON serialisation helpers for query responses.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn node_json(node: &NodeRecord, include_symbols: bool) -> Value {
    let mut value = json!({
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
    });
    if include_symbols {
        value["symbols"] = json!(node.symbols);
    }
    value
}

pub(super) fn backlog_item_detail_json(item: &crate::state::backlog::BacklogItem) -> Value {
    let mut value = item.to_json();
    value["description"] = json!(item.description);
    value
}

pub(super) fn todo_json(todo: &Todo) -> Value {
    json!({
        "path": todo.path,
        "node": todo.node,
        "status": todo_status(todo.status),
        "created": todo.created,
        "satisfies": todo.satisfies,
        "blocked_by": todo.blocked_by,
        "parent": todo.parent,
        "related": todo.related,
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
        "ratification": ratification_tier(decision.ratification),
        "ratified_by": ratified_by_wire(decision),
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

/// Extracts the first level-one Markdown heading from a body, falling back to
/// the provided fallback title. Moved from `src/ui/api.rs` so the webui and
/// `query_api` share one canonical title extraction.
pub(crate) fn title_from_body(body: &str, fallback: &str) -> String {
    body.lines()
        .find_map(|line| line.trim().strip_prefix("# "))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or(fallback)
        .to_owned()
}

/// Strips the project root from an absolute path so JSON wires expose stable
/// root-relative paths. Returns the original path unchanged if it is not below
/// root.
pub(crate) fn relative_path(path: &str, root: &std::path::Path) -> String {
    let path = std::path::Path::new(path);
    path.strip_prefix(root).map_or_else(
        |_| path.to_string_lossy().into_owned(),
        |p| p.to_string_lossy().into_owned(),
    )
}

/// Enriched todo wire used by the webui, CLI `--json`, and MCP. Keeps the
/// shared `todo_json` fields and adds the `title` and `body` the UI renders.
pub(super) fn todo_enriched_json(todo: &Todo, root: &std::path::Path) -> Value {
    let mut value = todo_json(todo);
    value["path"] = json!(relative_path(&todo.path, root));
    value["title"] = json!(title_from_body(&todo.body, "Artefact"));
    value["body"] = json!(todo.body);
    value
}

/// Enriched decision wire. Keeps the shared `decision_json` fields and adds
/// `path`, `title`, `date`, `revisited`, `revisit_triggers`, and `body`.
pub(super) fn decision_enriched_json(decision: &Decision, root: &std::path::Path) -> Value {
    let mut value = decision_json(decision);
    value["path"] = json!(relative_path(&decision.path, root));
    value["title"] = json!(title_from_body(&decision.body, "Artefact"));
    value["date"] = json!(decision.date);
    value["revisited"] = json!(decision.revisited);
    value["revisit_triggers"] = json!(decision.revisit_triggers);
    value["body"] = json!(decision.body);
    value
}

/// Enriched research wire. Keeps the shared `research_json` fields and adds
/// `path`, `title`, and `body`.
pub(super) fn research_enriched_json(research: &Research, root: &std::path::Path) -> Value {
    let mut value = research_json(research);
    value["path"] = json!(relative_path(&research.path, root));
    value["title"] = json!(title_from_body(&research.body, "Artefact"));
    value["body"] = json!(research.body);
    value
}

/// Enriched source wire. Keeps the shared `source_json` fields and adds `path`,
/// `title`, and `body`.
pub(super) fn source_enriched_json(source: &Source, root: &std::path::Path) -> Value {
    let mut value = source_json(source);
    value["path"] = json!(relative_path(&source.path, root));
    value["title"] = json!(title_from_body(&source.body, "Artefact"));
    value["body"] = json!(source.body);
    value
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
                "deferred_by": finding.deferred_by,
                "parked_by": finding.parked_by,
            })
        })
        .collect()
}

/// IDs of accepted decisions that reference the node directly. Shared by the
/// `get` JSON payload and the human `cairn get` rendering so both surfaces
/// carry the same accepted-decision pointers.
pub(crate) fn accepted_decision_ids(
    scan_result: &scanner::ScanResult,
    node_id: &str,
) -> Vec<String> {
    scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.status == DecisionStatus::Accepted
                && decision.nodes.iter().any(|node| node == node_id)
        })
        .map(|decision| decision.id.clone())
        .collect()
}

pub(crate) fn neighbourhood_ids(graph: &crate::map::Graph, node: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::from([node.to_owned()]);
    if let Some(edges) = graph.inbound.get(node) {
        ids.extend(edges.iter().map(|edge| edge.from.clone()));
    }
    if let Some(edges) = graph.outbound.get(node) {
        ids.extend(edges.iter().map(|edge| edge.to.clone()));
    }
    ids
}

pub(crate) fn research_for_nodes(
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

pub(crate) fn sources_for_nodes(
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

pub(crate) fn requires_valid_map(command: &str) -> bool {
    matches!(
        command,
        "get"
            | "neighbourhood"
            | "files"
            | "deps"
            | "contract"
            | "docstring"
            | "order"
            | "todos"
            | "decisions"
            | "research"
            | "sources"
            | "rationale"
            | "status"
            | "locate"
    )
}

pub(crate) fn parse_todo_status_filter(value: &str) -> Option<TodoStatus> {
    match value {
        "open" => Some(TodoStatus::Open),
        "in_progress" => Some(TodoStatus::InProgress),
        "done" => Some(TodoStatus::Done),
        "blocked" => Some(TodoStatus::Blocked),
        _ => None,
    }
}

pub(crate) fn parse_decision_status_filter(value: &str) -> Option<DecisionStatus> {
    match value {
        "proposed" => Some(DecisionStatus::Proposed),
        "accepted" => Some(DecisionStatus::Accepted),
        "deprecated" => Some(DecisionStatus::Deprecated),
        "superseded" => Some(DecisionStatus::Superseded),
        _ => None,
    }
}

pub(crate) const fn todo_status(status: TodoStatus) -> &'static str {
    match status {
        TodoStatus::Open => "open",
        TodoStatus::InProgress => "in_progress",
        TodoStatus::Done => "done",
        TodoStatus::Blocked => "blocked",
    }
}

/// Wire spelling for decision ratification tiers.
pub(crate) const fn ratification_tier(
    tier: crate::artefacts::registry::RatificationTier,
) -> &'static str {
    match tier {
        crate::artefacts::registry::RatificationTier::Local => "local",
        crate::artefacts::registry::RatificationTier::Binding => "binding",
    }
}

/// Wire value for who signed a decision: `machine` whenever the marker is
/// present (a recorded fact at any status), `maintainer` only once accepted
/// without the marker, and absent otherwise: a proposed decision has no
/// signer yet (`todo.decision-ratification-tiers`).
pub(crate) fn ratified_by_wire(decision: &Decision) -> Option<&'static str> {
    if decision.ratified_by_machine {
        Some("machine")
    } else if decision.status == DecisionStatus::Accepted {
        Some("maintainer")
    } else {
        None
    }
}

pub(crate) const fn decision_status(status: DecisionStatus) -> &'static str {
    match status {
        DecisionStatus::Proposed => "proposed",
        DecisionStatus::Accepted => "accepted",
        DecisionStatus::Deprecated => "deprecated",
        DecisionStatus::Superseded => "superseded",
    }
}

pub(crate) const fn source_verification(verification: SourceVerification) -> &'static str {
    match verification {
        SourceVerification::Verified => "verified",
        SourceVerification::External => "external",
        SourceVerification::Unverified => "unverified",
        SourceVerification::Tracked => "tracked",
    }
}

pub(super) const fn hook_kind_name(kind: HookKind) -> &'static str {
    match kind {
        HookKind::Structural => "structural",
        HookKind::Interface => "interface",
        HookKind::Tension => "tension",
        HookKind::ArchitectureDecision => "architecture-decision",
        HookKind::All => "all",
    }
}

pub(super) const fn hook_decision_name(decision: ExitDecision) -> &'static str {
    match decision {
        ExitDecision::Pass => "pass",
        ExitDecision::Block => "block",
    }
}

#[cfg(test)]
mod tests;
