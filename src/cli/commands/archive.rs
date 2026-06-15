//! CLI archive command implementation.
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
    err(
        1,
        &format!(
            "archive `{change_id}` is not available until the change archive engine is installed"
        ),
    )
}
