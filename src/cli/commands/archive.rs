//! CLI archive command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn run_archive_command(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
) -> CliResult {
    run_archive_command_with_path(parsed, root, legacy_warning).0
}

/// Archive while retaining the exact destination returned by the mutation
/// engine. Bootstrap callers use this as success authority rather than
/// guessing a path from directory contents.
pub(crate) fn run_archive_command_with_path(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
) -> (CliResult, Option<PathBuf>) {
    let Some(change_id) = parsed.command_args.get(1) else {
        return (err(2, "usage: cairn change archive <change-id>"), None);
    };
    let changes_dir = root.join(&parsed.changes_dir);
    let conflict_findings = hooks::detect_active_change_conflicts(&changes_dir);
    if !conflict_findings.is_empty() {
        return (
            CliResult {
                code: 1,
                stdout: render_findings(&conflict_findings, parsed.json, parsed.verbose),
                stderr: legacy_warning,
            },
            None,
        );
    }
    match crate::changes::archive(root, parsed.file.as_path(), &changes_dir, change_id) {
        Ok(report) => {
            let archive_path = report.archive_path.clone();
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
            (
                CliResult {
                    code: 0,
                    stdout,
                    stderr: legacy_warning,
                },
                Some(archive_path),
            )
        }
        Err(message) => {
            let mut result = error_output(parsed.json, "CAIRN_COMMAND_FAILED", &message);
            result.stderr = legacy_warning;
            (result, None)
        }
    }
}
