// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn load_for(
    command: &str,
    root: &Path,
    blueprint_path: &Path,
) -> Result<scanner::ScanResult, QueryError> {
    let result = if command == "scan" {
        scanner::scan(root, blueprint_path)
    } else {
        scanner::load_project(root, blueprint_path)
    };
    result.map_err(|message| QueryError {
        code: "CAIRN_COMMAND_FAILED".to_owned(),
        message,
        source_span: Some(blueprint_path.display().to_string()),
        remediation: None,
    })
}

pub(super) fn required<'a>(value: Option<&'a String>, name: &str) -> Result<&'a str, QueryError> {
    value.map(String::as_str).ok_or_else(|| QueryError {
        code: format!("CAIRN_QUERY_MISSING_{}", name.to_ascii_uppercase()),
        message: format!("`{name}` is required"),
        source_span: None,
        remediation: None,
    })
}

pub(super) fn findings_error(findings: &[Finding]) -> QueryError {
    let message = findings
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ");
    QueryError {
        code: findings.first().map_or_else(
            || "CAIRN_QUERY_FINDINGS".to_owned(),
            |finding| finding.code.clone(),
        ),
        message,
        source_span: findings.first().and_then(|finding| finding.path.clone()),
        remediation: None,
    }
}

pub(super) fn finding_error(finding: Finding) -> QueryError {
    QueryError {
        code: finding.code,
        message: finding.message,
        source_span: finding.path,
        remediation: None,
    }
}

pub(super) fn command_error(message: String) -> QueryError {
    QueryError {
        code: "CAIRN_COMMAND_FAILED".to_owned(),
        message,
        source_span: None,
        remediation: None,
    }
}
