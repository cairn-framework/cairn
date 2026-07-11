//! UI API endpoint handlers.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use serialise::*;

pub(super) fn status_json(project: &scanner::ScanResult) -> String {
    let findings = query::lint(&project.graph).findings;
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Warning)
        .count();
    let infos = findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Info)
        .count();
    format!(
        "{{\"nodes\":{},\"edges\":{},\"findings\":{},\"errors\":{errors},\"warnings\":{warnings},\"infos\":{infos},\"interface_hash\":\"{}\"}}",
        project.graph.nodes.len(),
        project.graph.outbound.values().map(Vec::len).sum::<usize>(),
        findings.len(),
        esc(&project.interface_hash)
    )
}

pub(super) fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{}\",\"message\":\"{}\",\"node\":{},\"path\":{}}}",
        esc(&finding.code),
        severity_name(finding.severity),
        esc(&finding.message),
        optional_json(finding.node.as_deref()),
        optional_json(finding.path.as_deref())
    )
}

pub(super) fn project_finding(message: String) -> Finding {
    Finding {
        code: "CAIRN_UI_PROJECT_LOAD_FAILED".to_owned(),
        severity: FindingSeverity::Error,
        message,
        node: None,
        target: None,
        path: None,
    }
}
