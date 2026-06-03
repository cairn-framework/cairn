// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn node_arg(args: &[String]) -> Result<&str, Finding> {
    args.get(1).map(String::as_str).ok_or_else(|| Finding {
        code: "CAIRN_CLI_MISSING_NODE".to_owned(),
        severity: FindingSeverity::Error,
        message: "node argument is required".to_owned(),
        node: None,
        target: None,
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
            super::copy::lookup("empty-states.cli-clean-map.body")
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
        target: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    // ── esc ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_esc_empty_string() {
        assert_eq!(esc(""), "");
    }

    #[test]
    fn test_esc_no_special_chars() {
        assert_eq!(esc("hello world"), "hello world");
    }

    #[test]
    fn test_esc_backslash() {
        assert_eq!(esc("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_esc_double_quote() {
        assert_eq!(esc(r#"say "hi""#), r#"say \"hi\""#);
    }

    #[test]
    fn test_esc_newline() {
        assert_eq!(esc("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_esc_carriage_return() {
        assert_eq!(esc("a\rb"), "a\\rb");
    }

    #[test]
    fn test_esc_tab() {
        assert_eq!(esc("a\tb"), "a\\tb");
    }

    #[test]
    fn test_esc_backspace() {
        assert_eq!(esc("a\u{08}b"), "a\\bb");
    }

    #[test]
    fn test_esc_form_feed() {
        assert_eq!(esc("a\u{0C}b"), "a\\fb");
    }

    #[test]
    fn test_esc_backslash_is_escaped_first_so_replacements_do_not_double_escape() {
        // Input: the two chars '\' and 'n' (a literal backslash followed by 'n').
        // esc must produce '\\n' (JSON: literal backslash then 'n'), NOT '\\\\n'.
        // If the newline-replacement step ran before the backslash step it would
        // turn '\\' (already escaped backslash) into '\\\\' — double escaping.
        assert_eq!(esc("\\n"), "\\\\n");
    }

    #[test]
    fn test_esc_all_specials_in_one_value() {
        let input = "a\\\"\n\r\t\u{08}\u{0C}z";
        let want = "a\\\\\\\"\\n\\r\\t\\b\\fz";
        assert_eq!(esc(input), want);
    }

    // ── string_array_json ────────────────────────────────────────────────────

    #[test]
    fn test_string_array_json_empty() {
        assert_eq!(string_array_json(&[]), "[]");
    }

    #[test]
    fn test_string_array_json_single() {
        assert_eq!(string_array_json(&["a".to_owned()]), "[\"a\"]");
    }

    #[test]
    fn test_string_array_json_multiple() {
        assert_eq!(
            string_array_json(&["a".to_owned(), "b".to_owned()]),
            "[\"a\",\"b\"]"
        );
    }

    #[test]
    fn test_string_array_json_value_with_quote_is_escaped() {
        assert_eq!(string_array_json(&["a\"b".to_owned()]), "[\"a\\\"b\"]");
    }

    // ── lines ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_lines_empty_returns_none() {
        assert_eq!(lines(&[]), "None");
    }

    #[test]
    fn test_lines_single_value() {
        assert_eq!(lines(&["foo".to_owned()]), "- foo");
    }

    #[test]
    fn test_lines_multiple_values_joined_by_newline() {
        assert_eq!(lines(&["a".to_owned(), "b".to_owned()]), "- a\n- b");
    }

    // ── flag_value ───────────────────────────────────────────────────────────

    #[test]
    fn test_flag_value_found() {
        let args = args(&["--format", "json"]);
        assert_eq!(flag_value(&args, "--format"), Some("json"));
    }

    #[test]
    fn test_flag_value_not_present_returns_none() {
        let args = args(&["--node", "app.api"]);
        assert_eq!(flag_value(&args, "--format"), None);
    }

    #[test]
    fn test_flag_value_flag_at_last_position_returns_none_not_panic() {
        // flag is present but has no following argument.
        let args = args(&["--node", "app.api", "--format"]);
        assert_eq!(flag_value(&args, "--format"), None);
    }

    #[test]
    fn test_flag_value_empty_args_returns_none() {
        assert_eq!(flag_value(&[], "--format"), None);
    }

    #[test]
    fn test_flag_value_returns_first_occurrence_when_repeated() {
        let args = args(&["--format", "json", "--format", "mermaid"]);
        assert_eq!(flag_value(&args, "--format"), Some("json"));
    }

    // ── parse_todo_status_filter ─────────────────────────────────────────────

    #[test]
    fn test_parse_todo_status_filter_all_variants() {
        assert_eq!(parse_todo_status_filter("open"), Some(TodoStatus::Open));
        assert_eq!(
            parse_todo_status_filter("in_progress"),
            Some(TodoStatus::InProgress)
        );
        assert_eq!(parse_todo_status_filter("done"), Some(TodoStatus::Done));
        assert_eq!(
            parse_todo_status_filter("blocked"),
            Some(TodoStatus::Blocked)
        );
    }

    #[test]
    fn test_parse_todo_status_filter_unknown_returns_none() {
        assert_eq!(parse_todo_status_filter("in-progress"), None);
        assert_eq!(parse_todo_status_filter(""), None);
        assert_eq!(parse_todo_status_filter("Open"), None); // case-sensitive
    }

    // ── parse_decision_status_filter ─────────────────────────────────────────

    #[test]
    fn test_parse_decision_status_filter_all_variants() {
        assert_eq!(
            parse_decision_status_filter("proposed"),
            Some(DecisionStatus::Proposed)
        );
        assert_eq!(
            parse_decision_status_filter("accepted"),
            Some(DecisionStatus::Accepted)
        );
        assert_eq!(
            parse_decision_status_filter("deprecated"),
            Some(DecisionStatus::Deprecated)
        );
        assert_eq!(
            parse_decision_status_filter("superseded"),
            Some(DecisionStatus::Superseded)
        );
    }

    #[test]
    fn test_parse_decision_status_filter_unknown_returns_none() {
        assert_eq!(parse_decision_status_filter("Accepted"), None);
        assert_eq!(parse_decision_status_filter(""), None);
    }

    // ── status display strings ────────────────────────────────────────────────

    #[test]
    fn test_todo_status_roundtrip() {
        for (status, name) in [
            (TodoStatus::Open, "open"),
            (TodoStatus::InProgress, "in_progress"),
            (TodoStatus::Done, "done"),
            (TodoStatus::Blocked, "blocked"),
        ] {
            assert_eq!(todo_status(status), name);
            assert_eq!(parse_todo_status_filter(name), Some(status));
        }
    }

    #[test]
    fn test_decision_status_roundtrip() {
        for (status, name) in [
            (DecisionStatus::Proposed, "proposed"),
            (DecisionStatus::Accepted, "accepted"),
            (DecisionStatus::Deprecated, "deprecated"),
            (DecisionStatus::Superseded, "superseded"),
        ] {
            assert_eq!(decision_status(status), name);
            assert_eq!(parse_decision_status_filter(name), Some(status));
        }
    }

    #[test]
    fn test_review_type_display_strings() {
        assert_eq!(review_type(ReviewType::Human), "human");
        assert_eq!(
            review_type(ReviewType::AgentIntrospective),
            "agent_introspective"
        );
        assert_eq!(
            review_type(ReviewType::AgentCrossModel),
            "agent_cross_model"
        );
    }

    #[test]
    fn test_source_verification_display_strings() {
        assert_eq!(
            source_verification(SourceVerification::Verified),
            "verified"
        );
        assert_eq!(
            source_verification(SourceVerification::External),
            "external"
        );
        assert_eq!(
            source_verification(SourceVerification::Unverified),
            "unverified"
        );
    }

    // ── node_arg ─────────────────────────────────────────────────────────────

    #[test]
    fn test_node_arg_returns_second_element() {
        // args[0] is the subcommand name, args[1] is the node id.
        let a = args(&["node", "app.api"]);
        assert_eq!(node_arg(&a), Ok("app.api"));
    }

    #[test]
    fn test_node_arg_missing_returns_cli_missing_node_finding() {
        let a = args(&["node"]);
        let err = node_arg(&a).unwrap_err();
        assert_eq!(err.code, "CAIRN_CLI_MISSING_NODE");
    }

    #[test]
    fn test_node_arg_empty_slice_returns_error() {
        let err = node_arg(&[]).unwrap_err();
        assert_eq!(err.code, "CAIRN_CLI_MISSING_NODE");
    }

    // ── err / ok ─────────────────────────────────────────────────────────────

    #[test]
    fn test_err_result_has_given_exit_code_and_stderr_message() {
        let result = err(2, "bad args");
        assert_eq!(result.code, 2);
        assert_eq!(result.stderr, "bad args\n");
        assert_eq!(result.stdout, "");
    }

    #[test]
    fn test_ok_result_has_exit_code_zero_and_stdout_message() {
        let result = ok("done\n".to_owned());
        assert_eq!(result.code, 0);
        assert_eq!(result.stdout, "done\n");
        assert_eq!(result.stderr, "");
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    fn args(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| (*s).to_owned()).collect()
    }
}
