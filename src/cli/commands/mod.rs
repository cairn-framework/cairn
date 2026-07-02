//! Shared CLI command helpers and re-exports of per-command implementations.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::format::flag_value;
use super::*;

mod archive;
mod change;
mod decision;
mod feedback;
mod gap;
mod hook;
mod import;
mod onboard;
mod project;
mod watch;
mod workspace;

pub(crate) use archive::run_archive_command;
pub(crate) use change::run_change_new;
pub(crate) use decision::run_decision_command;
pub(crate) use feedback::run_feedback_command;
pub(crate) use gap::run_gap_command;
pub(crate) use hook::run_hook_command;
pub(crate) use import::run_import_openspec;
pub(crate) use onboard::run_onboard_command;
pub(crate) use project::{init_project, run_ui_command};
pub(crate) use watch::run_watch_command;
pub(crate) use workspace::run_workspace_command;

pub(crate) fn run_shared_json_command(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
) -> CliResult {
    let request = shared_request(parsed);
    let changes_dir = root.join(&parsed.changes_dir);
    match crate::query_api::execute(root, &parsed.file, &changes_dir, &request) {
        Ok(response) => CliResult {
            code: shared_exit_code(parsed.command.as_str(), &response.data),
            stdout: format!("{}\n", response.data),
            stderr: legacy_warning,
        },
        Err(error) => CliResult {
            code: 1,
            stdout: format!("{{\"error\":{}}}\n", crate::query_api::error_json(&error)),
            stderr: legacy_warning,
        },
    }
}

pub(crate) fn shared_request(parsed: &ParsedArgs) -> crate::query_api::QueryRequest {
    let arg = |index: usize| parsed.command_args.get(index).cloned();
    crate::query_api::QueryRequest {
        tool: parsed.command.clone(),
        node: arg(1),
        change: arg(1),
        old_id: arg(1),
        new_id: arg(2),
        status: flag_value(&parsed.command_args, "--status")
            .or_else(|| {
                parsed
                    .command_args
                    .get(1)
                    .map(String::as_str)
                    .filter(|_| parsed.command == "hook")
            })
            .map(ToOwned::to_owned),
        language: flag_value(&parsed.command_args, "--language").map(ToOwned::to_owned),
        flags: shared_flags(&parsed.command_args),
        mutating: matches!(
            parsed.command.as_str(),
            "scan" | "rename" | "draft_discard" | "draft_edit" | "draft_accept" | "summarise"
        ),
    }
}

pub(crate) fn shared_flags(args: &[String]) -> BTreeSet<crate::query_api::QueryFlag> {
    let mut flags = BTreeSet::new();
    let pairs = [
        ("--transitive", crate::query_api::QueryFlag::Transitive),
        ("--include-todos", crate::query_api::QueryFlag::IncludeTodos),
        (
            "--include-research",
            crate::query_api::QueryFlag::IncludeResearch,
        ),
        (
            "--include-reviews",
            crate::query_api::QueryFlag::IncludeReviews,
        ),
        (
            "--include-deprecated-decisions",
            crate::query_api::QueryFlag::IncludeDeprecatedDecisions,
        ),
        (
            "--include-changes",
            crate::query_api::QueryFlag::IncludeChanges,
        ),
        ("--edited", crate::query_api::QueryFlag::Edited),
    ];
    for (argument, flag) in pairs {
        if args.iter().any(|value| value == argument) {
            flags.insert(flag);
        }
    }
    flags
}

pub(crate) fn shared_exit_code(command: &str, data: &serde_json::Value) -> u8 {
    if !matches!(command, "lint" | "scan" | "hook") {
        return 0;
    }
    let findings = data
        .get("findings")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten();
    u8::from(
        findings
            .filter_map(|finding| finding.get("severity"))
            .any(|severity| {
                // Cycle 4: severity wire format is now lowercase per
                // FindingSeverity::name(). Compare to "error" rather
                // than the legacy PascalCase "Error".
                severity.as_str().is_some_and(|value| value == "error")
            }),
    )
}

pub(crate) fn legacy_blueprint_warning(root: &Path) -> String {
    if root.join("cairn.blueprint").exists() && root.join("cairn.dsl").exists() {
        "warning: `cairn.dsl` is unused; remove it or rename remaining references to `cairn.blueprint`\n".to_owned()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_api::QueryFlag;
    use crate::query_api::requires_valid_map;

    // ── requires_valid_map ────────────────────────────────────────────────────

    #[test]
    fn test_requires_valid_map_neighbourhood_is_missing() {
        // "neighbourhood" queries node neighbours from the graph — identical in
        // graph-dependency to "get" and "depends", both of which ARE in the
        // requires_valid_map list.  Omitting it means a broken graph produces
        // a confusing "node not found" error instead of "map has integrity errors".
        assert!(
            requires_valid_map("neighbourhood"),
            "neighbourhood queries graph neighbours and must require a valid map"
        );
    }

    #[test]
    fn test_requires_valid_map_all_listed_commands_return_true() {
        for cmd in &[
            "get",
            "neighbourhood",
            "files",
            "dependents",
            "depends",
            "contract",
            "docstring",
            "order",
            "todos",
            "decisions",
            "research",
            "sources",
            "rationale",
            "status",
        ] {
            assert!(
                requires_valid_map(cmd),
                "expected requires_valid_map({cmd:?}) to be true"
            );
        }
    }

    #[test]
    fn test_requires_valid_map_non_query_commands_return_false() {
        for cmd in &["scan", "lint", "init", "onboard", "watch", "hook", "export"] {
            assert!(
                !requires_valid_map(cmd),
                "expected requires_valid_map({cmd:?}) to be false"
            );
        }
    }

    // ── legacy_blueprint_warning ──────────────────────────────────────────────

    #[test]
    fn test_legacy_blueprint_warning_both_files_warns() {
        let dir = std::env::temp_dir().join(format!(
            "cairn-cmd-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("cairn.blueprint"), "").unwrap();
        std::fs::write(dir.join("cairn.dsl"), "").unwrap();
        let warn = legacy_blueprint_warning(&dir);
        assert!(
            !warn.is_empty(),
            "both files present must produce a warning"
        );
        assert!(warn.contains("cairn.dsl"));
    }

    #[test]
    fn test_legacy_blueprint_warning_blueprint_only_no_warning() {
        let dir = std::env::temp_dir().join(format!(
            "cairn-cmd-test-bonly-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("cairn.blueprint"), "").unwrap();
        assert!(
            legacy_blueprint_warning(&dir).is_empty(),
            "only cairn.blueprint must produce no warning"
        );
    }

    // ── shared_exit_code ──────────────────────────────────────────────────────

    #[test]
    fn test_shared_exit_code_non_lint_always_zero() {
        let data = serde_json::json!({"findings": [{"severity": "error"}]});
        assert_eq!(shared_exit_code("get", &data), 0);
        assert_eq!(shared_exit_code("neighbourhood", &data), 0);
        assert_eq!(shared_exit_code("export", &data), 0);
    }

    #[test]
    fn test_shared_exit_code_lint_with_error_severity_returns_one() {
        let data = serde_json::json!({
            "findings": [{"severity": "error"}, {"severity": "warning"}]
        });
        assert_eq!(shared_exit_code("lint", &data), 1);
        assert_eq!(shared_exit_code("scan", &data), 1);
        assert_eq!(shared_exit_code("hook", &data), 1);
    }

    #[test]
    fn test_shared_exit_code_lint_with_warnings_only_returns_zero() {
        let data = serde_json::json!({
            "findings": [{"severity": "warning"}, {"severity": "info"}]
        });
        assert_eq!(shared_exit_code("lint", &data), 0);
    }

    #[test]
    fn test_shared_exit_code_uppercase_error_severity_not_counted() {
        // Wire format uses lowercase "error"; "Error" (PascalCase) is legacy.
        // The function checks lowercase only — a legacy client sending "Error"
        // would get exit code 0, not 1.  This is documented behavior.
        let data = serde_json::json!({"findings": [{"severity": "Error"}]});
        assert_eq!(shared_exit_code("lint", &data), 0);
    }

    // ── shared_flags ──────────────────────────────────────────────────────────

    #[test]
    fn test_shared_flags_known_flag_sets_correct_flag() {
        let args: Vec<String> = vec!["--transitive".to_owned(), "app.api".to_owned()];
        let flags = shared_flags(&args);
        assert!(flags.contains(&QueryFlag::Transitive));
    }

    #[test]
    fn test_shared_flags_unknown_arg_produces_empty_set() {
        let args: Vec<String> = vec!["--unknown-flag".to_owned()];
        let flags = shared_flags(&args);
        assert!(flags.is_empty());
    }

    #[test]
    fn test_shared_flags_multiple_flags_all_set() {
        let args: Vec<String> = vec![
            "--transitive".to_owned(),
            "--include-todos".to_owned(),
            "app.api".to_owned(),
        ];
        let flags = shared_flags(&args);
        assert!(flags.contains(&QueryFlag::Transitive));
        assert!(flags.contains(&QueryFlag::IncludeTodos));
        assert_eq!(flags.len(), 2);
    }
}
