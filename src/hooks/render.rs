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
    format!(
        "{{\"kind\":\"{}\",\"decision\":\"{}\",\"exit_code\":{},\"elapsed_ms\":{},\"findings\":[{}],\"conflict_findings\":[{}],\"output_paths\":[]}}\n",
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
            .join(",")
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
