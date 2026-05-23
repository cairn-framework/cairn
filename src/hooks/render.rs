//! Hook report renderers.

use std::fmt::Write;

use crate::{
    hooks::{ExitDecision, HookKind, HookReport},
    map::graph::Finding,
};

/// Renders a hook report as human-readable text.
#[must_use]
pub fn render_human(report: &HookReport) -> String {
    let mut output = format!(
        "Hook: {}\nDecision: {}\nElapsed: {}ms\n",
        hook_name(report.kind),
        decision_name(report.decision),
        report.elapsed_ms
    );
    if report.findings.is_empty() {
        output.push_str("Findings:\nNone\n");
    } else {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            let _ = writeln!(
                output,
                "{:?}: {} {}",
                finding.severity, finding.code, finding.message
            );
        }
    }
    output
}

/// Renders a hook report as JSON.
#[must_use]
pub fn render_json(report: &HookReport) -> String {
    let output_paths_json = report
        .output_paths
        .iter()
        .map(|p| format!("\"{}\"", esc(p)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"kind\":\"{}\",\"decision\":\"{}\",\"exit_code\":{},\"elapsed_ms\":{},\"findings\":[{}],\"conflict_findings\":[{}],\"output_paths\":[{}]}}\n",
        hook_name(report.kind),
        decision_name(report.decision),
        report.exit_code(),
        report.elapsed_ms,
        report
            .findings
            .iter()
            .map(finding_json)
            .collect::<Vec<_>>()
            .join(","),
        report
            .conflict_findings
            .iter()
            .map(finding_json)
            .collect::<Vec<_>>()
            .join(","),
        output_paths_json
    )
}

const fn hook_name(kind: HookKind) -> &'static str {
    match kind {
        HookKind::Structural => "structural",
        HookKind::Interface => "interface",
        HookKind::Tension => "tension",
        HookKind::ArchitectureDecision => "architecture-decision",
        HookKind::All => "all",
    }
}

const fn decision_name(decision: ExitDecision) -> &'static str {
    match decision {
        ExitDecision::Pass => "pass",
        ExitDecision::Block => "block",
    }
}

fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{}\",\"message\":\"{}\",\"node\":{},\"path\":{}}}",
        esc(&finding.code),
        finding.severity.name(),
        esc(&finding.message),
        optional_json(finding.node.as_deref()),
        optional_json(finding.path.as_deref())
    )
}

fn optional_json(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_owned(), |value| format!("\"{}\"", esc(value)))
}

fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hooks::{ExitDecision, HookKind, HookReport};
    use crate::map::graph::{Finding, FindingSeverity};

    fn err_finding(code: &str) -> Finding {
        Finding {
            code: code.to_owned(),
            severity: FindingSeverity::Error,
            message: "test error".to_owned(),
            node: None,
            target: None,
            path: None,
        }
    }

    fn bare_report(kind: HookKind, decision: ExitDecision) -> HookReport {
        HookReport {
            kind,
            findings: Vec::new(),
            conflict_findings: Vec::new(),
            decision,
            elapsed_ms: 0,
            output_paths: Vec::new(),
        }
    }

    // ── hook_name / decision_name ─────────────────────────────────────────────

    #[test]
    fn test_hook_name_all_variants() {
        assert_eq!(hook_name(HookKind::Structural), "structural");
        assert_eq!(hook_name(HookKind::Interface), "interface");
        assert_eq!(hook_name(HookKind::Tension), "tension");
        assert_eq!(
            hook_name(HookKind::ArchitectureDecision),
            "architecture-decision"
        );
        assert_eq!(hook_name(HookKind::All), "all");
    }

    #[test]
    fn test_decision_name_pass_is_lowercase_pass() {
        assert_eq!(decision_name(ExitDecision::Pass), "pass");
    }

    #[test]
    fn test_decision_name_block_is_lowercase_block() {
        // The CLI orphaned file used "Fail"; the real hook engine uses "block".
        assert_eq!(decision_name(ExitDecision::Block), "block");
    }

    // ── optional_json ─────────────────────────────────────────────────────────

    #[test]
    fn test_optional_json_none_produces_null() {
        assert_eq!(optional_json(None), "null");
    }

    #[test]
    fn test_optional_json_some_produces_quoted_string() {
        assert_eq!(optional_json(Some("app.api")), "\"app.api\"");
    }

    #[test]
    fn test_optional_json_some_with_quote_is_escaped() {
        assert_eq!(optional_json(Some(r#"say "hi""#)), r#""say \"hi\"""#);
    }

    // ── esc ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_esc_backslash_escaped_first() {
        // '\\' must be escaped before '"' to prevent double-escaping.
        assert_eq!(esc("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_esc_double_quote() {
        assert_eq!(esc(r#"a"b"#), r#"a\"b"#);
    }

    #[test]
    fn test_esc_newline() {
        assert_eq!(esc("a\nb"), r"a\nb");
    }

    // ── render_human ──────────────────────────────────────────────────────────

    #[test]
    fn test_render_human_no_findings_shows_none() {
        let report = bare_report(HookKind::Structural, ExitDecision::Pass);
        let out = render_human(&report);
        assert!(out.contains("Hook: structural"), "kind: {out:?}");
        assert!(out.contains("Decision: pass"), "decision: {out:?}");
        assert!(out.contains("Findings:\nNone"), "empty findings: {out:?}");
    }

    #[test]
    fn test_render_human_with_findings_lists_them() {
        let mut report = bare_report(HookKind::Interface, ExitDecision::Block);
        report.findings = vec![err_finding("CAIRN_TEST_ERROR")];
        let out = render_human(&report);
        assert!(out.contains("CAIRN_TEST_ERROR"), "finding code: {out:?}");
        // Human output uses Debug format for severity: PascalCase.
        assert!(
            out.contains("Error:") || out.contains("Error "),
            "PascalCase severity in human output: {out:?}"
        );
    }

    // ── render_json ───────────────────────────────────────────────────────────

    #[test]
    fn test_render_json_structure() {
        let report = bare_report(HookKind::Structural, ExitDecision::Pass);
        let json = render_json(&report);
        assert!(json.starts_with('{'), "must start with {{: {json:?}");
        assert!(json.trim_end().ends_with('}'), "must end with }}: {json:?}");
        assert!(json.contains("\"kind\":\"structural\""), "kind: {json:?}");
        assert!(json.contains("\"decision\":\"pass\""), "decision: {json:?}");
        assert!(
            json.contains("\"exit_code\":0"),
            "exit_code Pass=0: {json:?}"
        );
    }

    #[test]
    fn test_render_json_block_exit_code_is_one() {
        let report = bare_report(HookKind::Interface, ExitDecision::Block);
        let json = render_json(&report);
        assert!(
            json.contains("\"exit_code\":1"),
            "exit_code Block=1: {json:?}"
        );
        assert!(
            json.contains("\"decision\":\"block\""),
            "decision block: {json:?}"
        );
    }

    #[test]
    fn test_render_json_finding_node_null_when_absent() {
        let mut report = bare_report(HookKind::Structural, ExitDecision::Pass);
        report.findings = vec![err_finding("CODE")];
        let json = render_json(&report);
        assert!(
            json.contains("\"node\":null"),
            "absent node → null: {json:?}"
        );
    }

    #[test]
    fn test_render_json_finding_node_present_when_set() {
        let mut report = bare_report(HookKind::Structural, ExitDecision::Pass);
        report.findings = vec![Finding {
            node: Some("app.api".to_owned()),
            ..err_finding("CODE")
        }];
        let json = render_json(&report);
        assert!(
            json.contains("\"node\":\"app.api\""),
            "node present: {json:?}"
        );
    }

    #[test]
    fn test_render_json_output_paths_serialized() {
        // RED test: render_json hardcodes output_paths:[] regardless of the
        // actual report.output_paths field. This must be fixed.
        let mut report = bare_report(HookKind::Structural, ExitDecision::Pass);
        report.output_paths = vec!["map.md".to_owned()];
        let json = render_json(&report);
        assert!(
            json.contains("map.md"),
            "output_paths must be serialized into JSON, not hardcoded to []: {json:?}"
        );
    }
}
