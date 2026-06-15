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

#[cfg(test)]
mod tests {
    //! Unit tests for query API utility helpers.

    use crate::map::graph::{Finding, FindingSeverity};

    use super::*;

    fn sample_finding(code: &str, message: &str, path: Option<&str>) -> Finding {
        Finding {
            code: code.to_owned(),
            severity: FindingSeverity::Error,
            message: message.to_owned(),
            node: None,
            target: None,
            path: path.map(std::convert::Into::into),
        }
    }

    #[test]
    fn required_returns_value_when_present() {
        let value = "present".to_owned();
        assert_eq!(required(Some(&value), "field").unwrap(), "present");
    }

    #[test]
    fn required_reports_missing_field() {
        let err = required(None, "field").unwrap_err();
        assert_eq!(err.code, "CAIRN_QUERY_MISSING_FIELD");
        assert_eq!(err.message, "`field` is required");
        assert!(err.source_span.is_none());
        assert!(err.remediation.is_none());
    }

    #[test]
    fn findings_error_empty_slice_uses_generic_code() {
        let err = findings_error(&[]);
        assert_eq!(err.code, "CAIRN_QUERY_FINDINGS");
        assert_eq!(err.message, "");
        assert!(err.source_span.is_none());
    }

    #[test]
    fn findings_error_uses_first_finding_code_and_path() {
        let finding = sample_finding("F1", "first", Some("/a"));
        let err = findings_error(std::slice::from_ref(&finding));
        assert_eq!(err.code, "F1");
        assert_eq!(err.message, "F1: first");
        assert_eq!(err.source_span.as_deref(), Some("/a"));
    }

    #[test]
    fn findings_error_joins_multiple_finding_messages() {
        let first = sample_finding("F1", "one", None);
        let second = sample_finding("F2", "two", Some("/b"));
        let err = findings_error(&[first, second]);
        assert_eq!(err.code, "F1");
        assert_eq!(err.message, "F1: one; F2: two");
        assert!(err.source_span.is_none());
    }

    #[test]
    fn finding_error_maps_fields() {
        let finding = sample_finding("F3", "msg", Some("/c"));
        let err = finding_error(finding);
        assert_eq!(err.code, "F3");
        assert_eq!(err.message, "msg");
        assert_eq!(err.source_span.as_deref(), Some("/c"));
        assert!(err.remediation.is_none());
    }

    #[test]
    fn command_error_wraps_message() {
        let err = command_error("boom".to_owned());
        assert_eq!(err.code, "CAIRN_COMMAND_FAILED");
        assert_eq!(err.message, "boom");
        assert!(err.source_span.is_none());
    }
}
