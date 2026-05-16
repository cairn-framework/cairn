// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn node_arg(args: &[String]) -> Result<&str, Finding> {
    args.get(1).map(String::as_str).ok_or_else(|| Finding {
        code: "CAIRN_CLI_MISSING_NODE".to_owned(),
        severity: FindingSeverity::Error,
        message: "node argument is required".to_owned(),
        node: None,
        path: None,
    })
}

pub(super) fn render_node(node: &NodeRecord, json: bool) -> String {
    if json {
        format!("{}\n", node_json(node))
    } else {
        format!(
            "ID: {}\nName: {}\nDescription: {}\nState: {:?}\n",
            node.id, node.name, node.description, node.state
        )
    }
}

pub(super) fn render_findings(findings: &[Finding], json: bool) -> String {
    if json {
        format!(
            "{{\"findings\":[{}]}}\n",
            findings
                .iter()
                .map(finding_json)
                .collect::<Vec<_>>()
                .join(",")
        )
    } else if findings.is_empty() {
        format!(
            "Findings:\n{}\n",
            super::copy::lookup("empty-states.no-findings")
        )
    } else {
        format!(
            "Findings:\n{}\n",
            findings
                .iter()
                .map(|finding| format!(
                    "{:?}: {} {}",
                    finding.severity, finding.code, finding.message
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub(super) fn node_json(node: &NodeRecord) -> String {
    format!(
        "{{\"id\":\"{}\",\"name\":\"{}\",\"description\":\"{}\",\"state\":\"{:?}\",\"children\":{},\"files\":{}}}",
        esc(&node.id),
        esc(&node.name),
        esc(&node.description),
        node.state,
        string_array_json(&node.children),
        string_array_json(&node.files)
    )
}

pub(super) fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{}\",\"message\":\"{}\"}}",
        esc(&finding.code),
        finding.severity.name(),
        esc(&finding.message)
    )
}

pub(super) fn todos_json(todos: &[Todo]) -> String {
    format!(
        "[{}]",
        todos
            .iter()
            .map(|todo| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"status\":\"{}\",\"created\":\"{}\",\"satisfies\":\"{}\"}}",
                    esc(&todo.path),
                    esc(&todo.node),
                    todo_status(todo.status),
                    esc(&todo.created),
                    esc(todo.satisfies.as_deref().unwrap_or(""))
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn decisions_json(decisions: &[Decision]) -> String {
    format!(
        "[{}]",
        decisions
            .iter()
            .map(|decision| {
                format!(
                    "{{\"id\":\"{}\",\"status\":\"{}\",\"nodes\":{},\"informed_by\":{},\"supersedes\":{},\"refines\":{},\"related\":{}}}",
                    esc(&decision.id),
                    decision_status(decision.status),
                    string_array_json(&decision.nodes),
                    string_array_json(&decision.informed_by),
                    string_array_json(&decision.supersedes),
                    string_array_json(&decision.refines),
                    string_array_json(&decision.related)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn research_json(research: &[Research]) -> String {
    format!(
        "[{}]",
        research
            .iter()
            .map(|item| {
                format!(
                    "{{\"id\":\"{}\",\"nodes\":{},\"sources\":{},\"date\":\"{}\"}}",
                    esc(&item.id),
                    string_array_json(&item.nodes),
                    string_array_json(&item.sources),
                    esc(&item.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn reviews_json(reviews: &[Review]) -> String {
    format!(
        "[{}]",
        reviews
            .iter()
            .map(|review| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"review_type\":\"{}\",\"date\":\"{}\",\"reviewer\":\"{}\"}}",
                    esc(&review.path),
                    esc(&review.node),
                    review_type(review.review_type),
                    esc(&review.date),
                    esc(&review.reviewer)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn sources_json(sources: &[Source]) -> String {
    format!(
        "[{}]",
        sources
            .iter()
            .map(|source| {
                format!(
                    "{{\"id\":\"{}\",\"file\":\"{}\",\"verification\":\"{}\",\"type\":\"{}\",\"date\":\"{}\"}}",
                    esc(&source.id),
                    esc(&source.file),
                    source_verification(source.verification),
                    esc(&source.source_type),
                    esc(&source.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
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

pub(super) fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find_map(|pair| (pair[0] == flag).then_some(pair[1].as_str()))
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

pub(super) fn todo_line(todo: &Todo) -> String {
    format!("{} [{}] {}", todo.node, todo_status(todo.status), todo.path)
}

pub(super) fn decision_line(decision: &Decision) -> String {
    format!(
        "{} [{}] {}",
        decision.id,
        decision_status(decision.status),
        decision.nodes.join(", ")
    )
}

pub(super) fn research_line(research: &Research) -> String {
    format!("{} sources: {}", research.id, research.sources.join(", "))
}

pub(super) fn review_line(review: &Review) -> String {
    format!(
        "{} [{}] {}",
        review.node,
        review_type(review.review_type),
        review.path
    )
}

pub(super) fn source_line(source: &Source) -> String {
    format!(
        "{} [{}] {}",
        source.id,
        source_verification(source.verification),
        source.file
    )
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

pub(super) const fn review_type(review_type: ReviewType) -> &'static str {
    match review_type {
        ReviewType::Human => "human",
        ReviewType::AgentIntrospective => "agent_introspective",
        ReviewType::AgentCrossModel => "agent_cross_model",
    }
}

pub(super) const fn source_verification(verification: SourceVerification) -> &'static str {
    match verification {
        SourceVerification::Verified => "verified",
        SourceVerification::External => "external",
        SourceVerification::Unverified => "unverified",
    }
}

pub(super) fn findings_output(json: bool, findings: &[Finding]) -> CliResult {
    CliResult {
        code: 1,
        stdout: render_findings(findings, json),
        stderr: String::new(),
    }
}

pub(super) fn finding_output(json: bool, finding: Finding) -> CliResult {
    findings_output(json, &[finding])
}

pub(super) fn error_output(json: bool, code: &str, message: &str) -> CliResult {
    let finding = Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Error,
        message: message.to_owned(),
        node: None,
        path: None,
    };
    finding_output(json, finding)
}

pub(super) fn ok(stdout: String) -> CliResult {
    CliResult {
        code: 0,
        stdout,
        stderr: String::new(),
    }
}

/// Build an error `CliResult` with the given exit code and message.
///
/// # Exit codes
///
/// - **0**: success (clean, no findings)
/// - **1**: success with advisory findings, or operational error
/// - **2**: argument/usage error (bad args, unknown command)
pub(super) fn err(code: u8, message: &str) -> CliResult {
    CliResult {
        code,
        stdout: String::new(),
        stderr: format!("{message}\n"),
    }
}

pub(super) fn string_array_json(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", esc(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn lines(values: &[String]) -> String {
    if values.is_empty() {
        "None".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("- {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub(super) fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('\u{08}', "\\b")
        .replace('\u{0C}', "\\f")
}
