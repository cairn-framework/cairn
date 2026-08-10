//! Human and JSON rendering for the decision-evidence index.

use std::fmt::Write as _;

use crate::copy;

use super::{Evidence, EvidenceIndex, SCHEMA_VERSION};

/// Renders the index as human-readable text.
#[must_use]
pub fn render_human(report: &EvidenceIndex) -> String {
    if report.bound.is_empty() && report.unbound.is_empty() {
        return format!("{}\n", copy::lookup("onboard.decisions.empty"));
    }

    let mut out = format!(
        "{} ({} bound, {} unbound):\n",
        copy::lookup("onboard.decisions.heading"),
        report.bound.len(),
        report.unbound.len(),
    );

    if !report.bound.is_empty() {
        let _ = write!(
            out,
            "\n--- {} ---\n",
            copy::lookup("onboard.decisions.bound-heading")
        );
        for item in &report.bound {
            let _ = write!(
                out,
                "\n  {} [{}] -> {}\n",
                located(&item.evidence),
                item.evidence.kind.as_str(),
                item.node,
            );
            push_detail(&mut out, &item.evidence);
        }
    }

    if !report.unbound.is_empty() {
        let _ = write!(
            out,
            "\n--- {} ---\n{}\n",
            copy::lookup("onboard.decisions.unbound-heading"),
            copy::lookup("onboard.decisions.unbound-note"),
        );
        for item in &report.unbound {
            let _ = write!(out, "\n  {} [{}]\n", located(item), item.kind.as_str());
            push_detail(&mut out, item);
        }
    }

    out
}

/// Renders the index as the stable machine-readable data object.
#[must_use]
pub fn render_json(report: &EvidenceIndex) -> String {
    let bound: Vec<serde_json::Value> = report
        .bound
        .iter()
        .map(|item| {
            let mut value = evidence_json(&item.evidence);
            value["node"] = serde_json::Value::String(item.node.clone());
            value
        })
        .collect();
    let unbound: Vec<serde_json::Value> = report.unbound.iter().map(evidence_json).collect();

    let envelope = serde_json::json!({
        "schema_version": SCHEMA_VERSION,
        "bound": bound,
        "unbound": unbound,
        "bound_count": report.bound.len(),
        "unbound_count": report.unbound.len(),
    });
    format!("{envelope}\n")
}

fn evidence_json(item: &Evidence) -> serde_json::Value {
    serde_json::json!({
        "kind": item.kind.as_str(),
        "path": item.path,
        "line": item.line,
        "detail": item.detail,
    })
}

fn located(item: &Evidence) -> String {
    match item.line {
        Some(line) => format!("{}:{line}", item.path),
        None => item.path.clone(),
    }
}

fn push_detail(out: &mut String, item: &Evidence) {
    if !item.detail.is_empty() {
        let _ = writeln!(out, "    {}", item.detail);
    }
}
