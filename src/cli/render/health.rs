//! Health query renderer.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::query_api;

pub(crate) fn render_health(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let health = query_api::health_json(root, &changes_dir, scan_result);
    if parsed.json {
        format!("{health}\n")
    } else {
        format_health_human(
            &health,
            scan_result
                .graph
                .findings
                .iter()
                .filter(|f| f.severity == FindingSeverity::Error)
                .count(),
            scan_result
                .graph
                .findings
                .iter()
                .filter(|f| f.severity == FindingSeverity::Warning)
                .count(),
        )
    }
}

fn format_health_human(
    health: &serde_json::Value,
    scan_errors: usize,
    scan_warnings: usize,
) -> String {
    let clean = health
        .get("clean")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let summary = health.get("summary").unwrap_or(&serde_json::Value::Null);
    let total_errors = summary
        .get("total_errors")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let total_warnings = summary
        .get("total_warnings")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let total_info = summary
        .get("total_info")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let modules = summary.get("modules").unwrap_or(&serde_json::Value::Null);
    let synced = modules
        .get("synced")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let ghost = modules
        .get("ghost")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let orphaned = modules
        .get("orphaned")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let mut lines = Vec::new();
    if clean {
        lines.push("Health: clean".to_owned());
    } else {
        lines.push("Health: needs attention".to_owned());
    }
    lines.push(format!(
        "  errors: {total_errors}, warnings: {total_warnings}, info: {total_info}"
    ));
    lines.push(format!(
        "  modules: {synced} synced, {ghost} ghost, {orphaned} orphaned"
    ));
    if scan_errors > 0 {
        lines.push(format!("  scan errors: {scan_errors}"));
    }
    if scan_warnings > 0 {
        lines.push(format!("  scan warnings: {scan_warnings}"));
    }
    lines.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn health_json(
        clean: bool,
        total_errors: u64,
        total_warnings: u64,
        total_info: u64,
    ) -> serde_json::Value {
        serde_json::json!({
            "clean": clean,
            "summary": {
                "total_errors": total_errors,
                "total_warnings": total_warnings,
                "total_info": total_info,
                "modules": { "synced": 1, "ghost": 0, "orphaned": 0 }
            }
        })
    }

    #[test]
    fn format_health_human_clean() {
        let rendered = format_health_human(&health_json(true, 0, 0, 0), 0, 0);
        assert!(rendered.contains("Health: clean"));
        assert!(rendered.contains("errors: 0, warnings: 0, info: 0"));
    }

    #[test]
    fn format_health_human_unclean() {
        let rendered = format_health_human(&health_json(false, 1, 2, 3), 0, 0);
        assert!(rendered.contains("Health: needs attention"));
        assert!(rendered.contains("errors: 1, warnings: 2, info: 3"));
    }

    #[test]
    fn format_health_human_includes_scan_counts() {
        let rendered = format_health_human(&health_json(false, 1, 0, 0), 2, 1);
        assert!(rendered.contains("scan errors: 2"));
        assert!(rendered.contains("scan warnings: 1"));
    }

    #[test]
    fn format_health_human_omits_zero_scan_counts() {
        let rendered = format_health_human(&health_json(true, 0, 0, 0), 0, 0);
        assert!(!rendered.contains("scan errors"));
        assert!(!rendered.contains("scan warnings"));
    }

    #[test]
    fn format_health_human_defaults_missing_summary() {
        let rendered = format_health_human(&serde_json::json!({"clean": false}), 0, 0);
        assert!(rendered.contains("Health: needs attention"));
        assert!(rendered.contains("errors: 0, warnings: 0, info: 0"));
    }
}
