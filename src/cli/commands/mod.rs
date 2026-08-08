//! Shared CLI command helpers and re-exports of per-command implementations.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::format::{flag_value, positional_node};
use super::*;

mod archive;
mod baseline;
mod change;
mod coord;
mod decision;
mod feedback;
mod gap;
mod hook;
mod import;
mod onboard;
mod pack;
mod pack_assets;
mod pack_campaign;
mod pack_campaign_lock;
mod pack_harness;
mod pack_manifest;
mod pack_report;
mod project;
mod ruling_run;
mod todo;
mod watch;
mod wire;
mod workspace;

pub(crate) use archive::{run_archive_command, run_archive_command_with_path};
pub(crate) use baseline::run_baseline_command;
pub(crate) use change::run_change_new;
pub(crate) use coord::{run_coord_command, run_lease_command, run_ruling_command};
pub(crate) use decision::run_decision_command;
pub(crate) use feedback::run_feedback_command;
pub(crate) use gap::run_gap_command;
pub(crate) use hook::{run_hook_command, run_hook_lifecycle_command};
pub(crate) use import::run_import_openspec;
pub(crate) use onboard::run_onboard_command;
pub(crate) use pack::{install_default_pack, run_pack_command};
pub(crate) use project::{init_project, run_ui_command};
pub(crate) use todo::run_todo_command;
pub(crate) use watch::run_watch_command;
pub(crate) use wire::{atomic_write, contained_path, preflight_wire_check, wire_agent_guide};
pub(crate) use workspace::run_workspace_command;

pub(crate) fn run_draft_command(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
) -> CliResult {
    let subcommand = parsed.command_args.get(1).map(String::as_str);
    let tool = match subcommand {
        Some("list") => "draft list",
        Some("show") => "draft show",
        Some("edit") => "draft edit",
        Some("discard") => "draft discard",
        Some("accept") => "draft accept",
        Some("create") => "draft create",
        _ => {
            return err(
                2,
                "usage: cairn draft <list|show|edit|discard|accept|create> [args]",
            );
        }
    };
    if !parsed.json {
        return err(2, "this command currently requires --json");
    }
    let node = parsed.command_args.get(2).cloned();
    let request = crate::query_api::QueryRequest {
        at: None,
        since: None,
        tool: tool.to_owned(),
        node,
        symbol: None,
        change: None,
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags: shared_flags(&parsed.command_args),
        mutating: matches!(subcommand, Some("discard" | "edit" | "accept" | "create")),
    };
    execute_json_request(parsed, root, legacy_warning, &request)
}

pub(crate) fn run_shared_json_command(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
) -> CliResult {
    let request = shared_request(parsed);
    execute_json_request(parsed, root, legacy_warning, &request)
}

/// Executes a query-API request and formats the JSON envelope for the CLI.
///
/// `request.tool` equals `parsed.command` for shared commands and the
/// compound `cli_name` (e.g. `draft list`) for subcommand dispatch; either way
/// [`shared_exit_code`] resolves the correct exit code.
pub(crate) fn execute_json_request(
    parsed: &ParsedArgs,
    root: &Path,
    legacy_warning: String,
    request: &crate::query_api::QueryRequest,
) -> CliResult {
    let changes_dir = root.join(&parsed.changes_dir);
    match crate::query_api::execute(root, &parsed.file, &changes_dir, request) {
        Ok(response) => {
            let stdout = if request.tool == "locate" {
                format!("{}\n", response.data["matches"])
            } else {
                format!("{}\n", response.data)
            };
            CliResult {
                code: shared_exit_code(&request.tool, &response.data, parsed.strict),
                stdout,
                stderr: legacy_warning,
            }
        }
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
        at: None,
        since: None,
        tool: parsed.command.clone(),
        // Flag tokens and their values are never a node, so the node is the
        // first positional token even in a flag-first invocation (e.g.
        // `cairn --json todos --status open app.kernel` resolves app.kernel).
        node: positional_node(&parsed.command_args).cloned(),
        symbol: (parsed.command == "locate")
            .then(|| positional_node(&parsed.command_args).cloned())
            .flatten(),
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
        flags: {
            let mut flags = shared_flags(&parsed.command_args);
            if parsed.command == "deps"
                && flag_value(&parsed.command_args, "--direction") == Some("in")
            {
                flags.insert(crate::query_api::QueryFlag::Inbound);
            }
            flags
        },
        mutating: matches!(parsed.command.as_str(), "scan" | "rename"),
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
        ("--symbols", crate::query_api::QueryFlag::Symbols),
    ];
    for (argument, flag) in pairs {
        if args.iter().any(|value| value == argument) {
            flags.insert(flag);
        }
    }
    flags
}

/// Exit code for a shared-JSON command from its emitted `data` payload.
///
/// Under `--strict`, `lint`/`scan` read the published `strict_green` field so
/// the exit code and the wire verdict cannot disagree; a payload without the
/// field fails closed, treating a Warning as blocking. Otherwise (and for
/// `hook`) exit 1 keys on an `error`-severity finding.
pub(crate) fn shared_exit_code(command: &str, data: &serde_json::Value, strict: bool) -> u8 {
    if !matches!(command, "lint" | "scan" | "hook") {
        return 0;
    }
    let strict_lint = strict && matches!(command, "lint" | "scan");
    if strict_lint
        && let Some(green) = data
            .get("strict_green")
            .and_then(serde_json::Value::as_bool)
    {
        return u8::from(!green);
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
                severity
                    .as_str()
                    .is_some_and(|value| value == "error" || (strict_lint && value == "warning"))
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
            "deps",
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
        assert_eq!(shared_exit_code("get", &data, false), 0);
        assert_eq!(shared_exit_code("neighbourhood", &data, false), 0);
        assert_eq!(shared_exit_code("export", &data, false), 0);
    }

    #[test]
    fn test_shared_exit_code_lint_with_error_severity_returns_one() {
        let data = serde_json::json!({
            "findings": [{"severity": "error"}, {"severity": "warning"}]
        });
        assert_eq!(shared_exit_code("lint", &data, false), 1);
        assert_eq!(shared_exit_code("scan", &data, false), 1);
        assert_eq!(shared_exit_code("hook", &data, false), 1);
    }

    #[test]
    fn test_shared_exit_code_lint_with_warnings_only_returns_zero() {
        let data = serde_json::json!({
            "findings": [{"severity": "warning"}, {"severity": "info"}]
        });
        assert_eq!(shared_exit_code("lint", &data, false), 0);
    }

    #[test]
    fn test_shared_exit_code_strict_reads_published_strict_green() {
        let data = serde_json::json!({
            "findings": [{"severity": "warning"}],
            "strict_green": false
        });
        assert_eq!(
            shared_exit_code("lint", &data, true),
            1,
            "strict must exit 1 when the wire publishes strict_green false"
        );
        assert_eq!(
            shared_exit_code("scan", &data, true),
            1,
            "scan --strict --json must honour the strict flag"
        );
        assert_eq!(
            shared_exit_code("lint", &data, false),
            0,
            "without --strict a warning-only set stays exit 0"
        );
    }

    #[test]
    fn test_shared_exit_code_strict_green_true_exits_zero() {
        let data = serde_json::json!({
            "findings": [{"severity": "info"}],
            "strict_green": true
        });
        assert_eq!(shared_exit_code("lint", &data, true), 0);
    }

    #[test]
    fn test_shared_exit_code_strict_without_published_verdict_fails_closed() {
        // A payload lacking `strict_green` (older or foreign wire) must not
        // let a warning-only set pass the strict gate.
        let data = serde_json::json!({
            "findings": [{"severity": "warning"}]
        });
        assert_eq!(
            shared_exit_code("lint", &data, true),
            1,
            "strict without a published verdict must treat a warning as blocking"
        );
        assert_eq!(
            shared_exit_code("lint", &data, false),
            0,
            "the fail-closed fallback applies only under --strict"
        );
    }

    #[test]
    fn test_shared_exit_code_hook_ignores_strict_flag() {
        // `--strict` is documented for scan/lint only; hook keys on errors.
        let data = serde_json::json!({
            "findings": [{"severity": "warning"}],
            "strict_green": false
        });
        assert_eq!(shared_exit_code("hook", &data, true), 0);
    }

    #[test]
    fn test_shared_exit_code_uppercase_error_severity_not_counted() {
        // Wire format uses lowercase "error"; "Error" (PascalCase) is legacy.
        // The function checks lowercase only — a legacy client sending "Error"
        // would get exit code 0, not 1.  This is documented behavior.
        let data = serde_json::json!({"findings": [{"severity": "Error"}]});
        assert_eq!(shared_exit_code("lint", &data, false), 0);
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
