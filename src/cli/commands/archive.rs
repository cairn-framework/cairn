//! CLI archive command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn run_archive_command(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
) -> CliResult {
    let Some(change_id) = parsed.command_args.get(1) else {
        return err(2, "usage: cairn archive <change-id>");
    };
    let changes_dir = root.join(&parsed.changes_dir);
    let conflict_findings = hooks::detect_active_change_conflicts(&changes_dir);
    if !conflict_findings.is_empty() {
        return CliResult {
            code: 1,
            stdout: render_findings(&conflict_findings, parsed.json),
            stderr: legacy_warning,
        };
    }
    match crate::changes::archive(root, parsed.file.as_path(), change_id) {
        Ok(report) => {
            let stdout = if parsed.json {
                format!(
                    "{{\"command\":\"archive\",\"status\":\"ok\",\"data\":{{\"archive_path\":\"{}\",\"summary\":\"{}\"}}}}\n",
                    esc(&report.archive_path.to_string_lossy()),
                    esc(&report.summary)
                )
            } else {
                format!(
                    "Archived `{change_id}` to {}\n{}\n",
                    report.archive_path.display(),
                    report.summary
                )
            };
            CliResult {
                code: 0,
                stdout,
                stderr: legacy_warning,
            }
        }
        Err(message) => {
            let mut result = error_output(parsed.json, "CAIRN_COMMAND_FAILED", &message);
            result.stderr = legacy_warning;
            result
        }
    }
}
