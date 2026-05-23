// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::format::flag_value;
use super::*;

pub(super) fn run_shared_json_command(
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

pub(super) fn shared_request(parsed: &ParsedArgs) -> crate::query_api::QueryRequest {
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

pub(super) fn shared_flags(args: &[String]) -> BTreeSet<crate::query_api::QueryFlag> {
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

pub(super) fn shared_exit_code(command: &str, data: &serde_json::Value) -> u8 {
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

pub(super) fn run_hook_command(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
    legacy_warning: String,
) -> CliResult {
    let Some(kind) = parsed
        .command_args
        .get(1)
        .and_then(|value| parse_hook_kind(value))
    else {
        return err(2, "usage: cairn hook <structural|interface|tension|all>");
    };
    let changes_dir = root.join(&parsed.changes_dir);
    let report = hooks::run(kind, root, &changes_dir, scan_result);
    CliResult {
        code: report.exit_code(),
        stdout: if parsed.json {
            hooks::render_json(&report)
        } else {
            hooks::render_human(&report)
        },
        stderr: legacy_warning,
    }
}

pub(super) fn run_archive_command(
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

pub(super) fn parse_hook_kind(value: &str) -> Option<HookKind> {
    match value {
        "structural" => Some(HookKind::Structural),
        "interface" => Some(HookKind::Interface),
        "tension" => Some(HookKind::Tension),
        "architecture-decision" => Some(HookKind::ArchitectureDecision),
        "all" => Some(HookKind::All),
        _ => None,
    }
}

pub(super) fn run_onboard_command(parsed: &ParsedArgs) -> CliResult {
    let root = parsed
        .file
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let (blueprint_path, _temp_dir) = if parsed.file.exists() {
        (parsed.file.clone(), None)
    } else {
        let dir = std::env::temp_dir().join(format!(
            "cairn-onboard-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        let _ = fs::create_dir_all(&dir);
        let stub = dir.join("cairn.blueprint");
        let _ = fs::write(&stub, "System Stub \"onboard stub\" id \"stub\" {\n}\n");
        (stub, Some(dir))
    };

    match crate::scanner::load_project(root, &blueprint_path) {
        Ok(result) => {
            let report = crate::brownfield::onboard::analyze(&result.graph.findings);
            let output = if parsed.json {
                let inner = crate::brownfield::onboard::render_json(&report);
                let inner = inner.trim();
                format!("{{\"command\":\"onboard\",\"status\":\"ok\",\"data\":{inner}}}\n")
            } else {
                crate::brownfield::onboard::render_human(&report)
            };
            CliResult {
                code: 0,
                stdout: output,
                stderr: String::new(),
            }
        }
        Err(error) => {
            if parsed.json {
                CliResult {
                    code: 1,
                    stdout: format!(
                        "{{\"command\":\"onboard\",\"status\":\"error\",\"data\":{{\"message\":\"{}\"}}}}\n",
                        super::format::esc(&error)
                    ),
                    stderr: String::new(),
                }
            } else {
                err(1, &error)
            }
        }
    }
}

pub(super) fn legacy_blueprint_warning(root: &Path) -> String {
    if root.join("cairn.blueprint").exists() && root.join("cairn.dsl").exists() {
        "warning: `cairn.dsl` is unused; remove it or rename remaining references to `cairn.blueprint`\n".to_owned()
    } else {
        String::new()
    }
}

pub(super) fn run_ui_command(parsed: &ParsedArgs) -> CliResult {
    match ui::UiOptions::from_args(&parsed.command_args) {
        Ok(mut options) => {
            options.blueprint_path.clone_from(&parsed.file);
            ui::serve_current_thread(options).map_or_else(
                |error| err(1, &error.to_string()),
                |message| ok(format!("{message}\n")),
            )
        }
        Err(message) => err(2, &message),
    }
}

pub(super) fn requires_valid_map(command: &str) -> bool {
    matches!(
        command,
        "get"
            | "neighbourhood"
            | "files"
            | "dependents"
            | "depends"
            | "contract"
            | "order"
            | "todos"
            | "decisions"
            | "research"
            | "sources"
            | "rationale"
            | "status"
    )
}

pub(super) fn init_project(root: &Path) -> CliResult {
    let writes = [
        (
            "cairn.blueprint",
            "System Example \"Starter architecture\" id \"example\" {\n    Module App \"Starter app\" id \"example.app\" {\n        path \"./src\"\n    }\n}\n",
        ),
        (
            "cairn.config.yaml",
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"\"\nrules: {}\n",
        ),
        ("meta/contracts/.gitkeep", ""),
        (".cairn/state/.gitkeep", ""),
    ];
    for (path, content) in writes {
        let full = root.join(path);
        if let Some(parent) = full.parent()
            && let Err(error) = fs::create_dir_all(parent)
        {
            return err(
                1,
                &format!("failed to create {}: {error}", parent.display()),
            );
        }
        if !full.exists() && fs::write(&full, content).is_err() {
            return err(1, &format!("failed to write {}", full.display()));
        }
    }
    ok("initialized Cairn project\n".to_owned())
}

pub(super) fn run_change_new(root: &Path, change_id: &str) -> CliResult {
    if change_id.is_empty()
        || !change_id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return err(
            2,
            "change ID must be kebab-case (lowercase letters, digits, hyphens only)",
        );
    }
    let change_dir = root.join("meta/changes").join(change_id);
    if change_dir.exists() {
        return err(
            1,
            &format!("change directory already exists: {}", change_dir.display()),
        );
    }
    if let Err(error) = fs::create_dir_all(&change_dir) {
        return err(1, &format!("failed to create change directory: {error}"));
    }

    let proposal = format!(
        "# Proposal: {change_id}\n\n## Motivation\n\nDescribe the problem this change solves.\n\n## Scope\n\n- What this change covers\n\n## Out of scope\n\n- What this change does not cover\n",
    );
    if let Err(error) = fs::write(change_dir.join("proposal.md"), proposal) {
        return err(1, &format!("failed to write proposal.md: {error}"));
    }

    let design = format!(
        "# Design: {change_id}\n\n## Approach\n\nHigh-level approach to the solution.\n\n## Changes\n\nADDED:\n- New components\n\nMODIFIED:\n- Existing components\n\nREMOVED:\n- Obsolete components\n\nRENAMED:\n- Components with new names\n",
    );
    if let Err(error) = fs::write(change_dir.join("design.md"), design) {
        return err(1, &format!("failed to write design.md: {error}"));
    }

    let tasks =
        format!("# Tasks: {change_id}\n\n- [ ] Task one\n- [ ] Task two\n- [ ] Task three\n",);
    if let Err(error) = fs::write(change_dir.join("tasks.md"), tasks) {
        return err(1, &format!("failed to write tasks.md: {error}"));
    }

    if let Err(error) = fs::create_dir_all(change_dir.join("specs")) {
        return err(1, &format!("failed to create specs directory: {error}"));
    }

    if let Ok(config) = crate::scanner::config::load(root)
        && config.state_backend == "beads"
    {
        let beads = crate::state::BeadsStateBackend::new(root.to_path_buf());
        match beads.create_change_epic(change_id) {
            Ok(bead_id) => {
                let _ = fs::write(change_dir.join(".bead-id"), &bead_id);
                let tasks_content =
                    fs::read_to_string(change_dir.join("tasks.md")).unwrap_or_default();
                let task_lines: Vec<&str> = tasks_content
                    .lines()
                    .filter_map(|line| line.strip_prefix("- [ ] "))
                    .collect();
                if !task_lines.is_empty() {
                    match beads.create_task_beads(&bead_id, &task_lines) {
                        Ok(task_ids) => {
                            let _ =
                                fs::write(change_dir.join(".task-bead-ids"), task_ids.join("\n"));
                        }
                        Err(error) => {
                            eprintln!("warning: failed to create task beads: {error}");
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("warning: failed to create beads epic: {error}");
            }
        }
    }

    ok(format!(
        "created change directory at meta/changes/{change_id}/\n"
    ))
}
/// List tasks for a change backed by beads.
pub(super) fn run_change_tasks(root: &Path, change_id: &str) -> CliResult {
    let change_dir = root.join("meta/changes").join(change_id);
    if !change_dir.exists() {
        return err(
            1,
            &format!("change directory not found: {}", change_dir.display()),
        );
    }
    let bead_id_path = change_dir.join(".bead-id");
    if !bead_id_path.exists() {
        return err(1, "change has no beads backing; tasks are in tasks.md only");
    }
    let bead_id = match fs::read_to_string(&bead_id_path) {
        Ok(id) => id.trim().to_owned(),
        Err(error) => return err(1, &format!("failed to read .bead-id: {error}")),
    };
    let beads = crate::state::BeadsStateBackend::new(root.to_path_buf());
    match beads.list_child_tasks(&bead_id) {
        Ok(tasks) => {
            if tasks.is_empty() {
                return ok("no tasks found\n".to_owned());
            }
            let mut out = String::new();
            for (id, title) in tasks {
                let _ = std::fmt::Write::write_fmt(&mut out, format_args!("{id}: {title}\n"));
            }
            ok(out)
        }
        Err(error) => err(1, &format!("failed to list tasks: {error}")),
    }
}

/// Claim a change and all its open tasks.
pub(super) fn run_change_apply(root: &Path, change_id: &str) -> CliResult {
    let change_dir = root.join("meta/changes").join(change_id);
    if !change_dir.exists() {
        return err(
            1,
            &format!("change directory not found: {}", change_dir.display()),
        );
    }
    let bead_id_path = change_dir.join(".bead-id");
    if !bead_id_path.exists() {
        return err(1, "change has no beads backing; apply is not supported");
    }
    let bead_id = match fs::read_to_string(&bead_id_path) {
        Ok(id) => id.trim().to_owned(),
        Err(error) => return err(1, &format!("failed to read .bead-id: {error}")),
    };
    let beads = crate::state::BeadsStateBackend::new(root.to_path_buf());
    match beads.claim_change(&bead_id) {
        Ok(()) => ok(format!("claimed change {change_id} and its tasks\n")),
        Err(error) => err(1, &format!("failed to claim change: {error}")),
    }
}

/// Migrate openspec changes to the meta/changes directory.
pub(super) fn run_import_openspec(root: &Path, json: bool) -> CliResult {
    let openspec_dir = root.join("openspec/changes");
    let meta_dir = root.join("meta/changes");

    if !openspec_dir.exists() {
        return err(1, "no openspec/changes directory found");
    }

    if let Err(error) = fs::create_dir_all(&meta_dir) {
        return err(1, &format!("failed to create meta/changes: {error}"));
    }

    let mut migrated = Vec::new();
    let mut copied_archive = false;

    let entries = match fs::read_dir(&openspec_dir) {
        Ok(entries) => entries,
        Err(error) => return err(1, &format!("failed to read openspec/changes: {error}")),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(error) => {
                eprintln!("warning: failed to read directory entry: {error}");
                continue;
            }
        };

        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip hidden files and the archive directory (handled separately).
        if name_str.starts_with('.') || name_str == "archive" {
            continue;
        }

        let source = entry.path();
        let target = meta_dir.join(&name);

        if !source.is_dir() {
            continue;
        }

        // Copy directory recursively.
        if let Err(error) = copy_dir_all(&source, &target) {
            return err(1, &format!("failed to copy {name_str}: {error}"));
        }

        migrated.push(name_str.into_owned());
    }

    // Copy archive directory if it exists.
    let archive_source = openspec_dir.join("archive");
    if archive_source.exists() {
        let archive_target = meta_dir.join("archive");
        if let Err(error) = copy_dir_all(&archive_source, &archive_target) {
            return err(1, &format!("failed to copy archive: {error}"));
        }
        copied_archive = true;
    }

    if json {
        let response = serde_json::json!({
            "command": "import-openspec",
            "status": "ok",
            "data": {
                "migrated": migrated,
                "archive_copied": copied_archive,
            }
        });
        return ok(format!("{response}\n"));
    }

    let mut out = format!("migrated {} phase(s)\n", migrated.len());
    for name in &migrated {
        let _ = std::fmt::Write::write_str(&mut out, &format!("  {name}\n"));
    }
    if copied_archive {
        let _ = std::fmt::Write::write_str(&mut out, "archive copied\n");
    }
    ok(out)
}

fn copy_dir_all(source: impl AsRef<Path>, target: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target_path = target.as_ref().join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), &target_path)?;
        } else {
            fs::copy(entry.path(), &target_path)?;
        }
    }
    Ok(())
}

/// Watch for finding changes and emit newline-delimited JSON events.
pub(super) fn run_watch_command(root: &Path, opts: &crate::watch::WatchOpts) -> CliResult {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let blueprint = root.join("cairn.blueprint");

    // --once: single scan, emit all findings as added, exit.
    if opts.once {
        let findings = match crate::scanner::scan(root, &blueprint) {
            Ok(result) => result.graph.findings,
            Err(error) => {
                return err(1, &format!("scan failed: {error}"));
            }
        };
        let events = crate::watch::diff_findings(&[], &findings);
        for event in events {
            match serde_json::to_string(&event) {
                Ok(line) => println!("{line}"),
                Err(error) => eprintln!("json error: {error}"),
            }
        }
        return ok(String::new());
    }

    let stop = Arc::new(AtomicBool::new(false));
    let shutdown = Arc::clone(&stop);
    if let Err(error) = ctrlc::set_handler(move || {
        shutdown.store(true, Ordering::SeqCst);
    }) {
        return err(1, &format!("failed to set Ctrl-C handler: {error}"));
    }

    // Initial scan.
    let mut previous = match crate::scanner::scan(root, &blueprint) {
        Ok(result) => result.graph.findings,
        Err(error) => {
            return err(1, &format!("initial scan failed: {error}"));
        }
    };

    let interval = std::time::Duration::from_secs(opts.interval_secs);

    while !stop.load(Ordering::SeqCst) {
        std::thread::sleep(interval);

        if stop.load(Ordering::SeqCst) {
            break;
        }

        let current = match crate::scanner::scan(root, &blueprint) {
            Ok(result) => result.graph.findings,
            Err(error) => {
                eprintln!("scan error: {error}");
                continue;
            }
        };

        let events = crate::watch::diff_findings(&previous, &current);
        for event in events {
            match serde_json::to_string(&event) {
                Ok(line) => println!("{line}"),
                Err(error) => eprintln!("json error: {error}"),
            }
        }

        previous = current;
    }

    ok("watch stopped\n".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_api::QueryFlag;

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
            "files",
            "dependents",
            "depends",
            "contract",
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

    // ── parse_hook_kind ───────────────────────────────────────────────────────

    #[test]
    fn test_parse_hook_kind_all_valid_strings() {
        use crate::hooks::HookKind;
        assert_eq!(parse_hook_kind("structural"), Some(HookKind::Structural));
        assert_eq!(parse_hook_kind("interface"), Some(HookKind::Interface));
        assert_eq!(parse_hook_kind("tension"), Some(HookKind::Tension));
        assert_eq!(
            parse_hook_kind("architecture-decision"),
            Some(HookKind::ArchitectureDecision)
        );
        assert_eq!(parse_hook_kind("all"), Some(HookKind::All));
    }

    #[test]
    fn test_parse_hook_kind_unknown_returns_none() {
        assert!(parse_hook_kind("unknown").is_none());
        assert!(parse_hook_kind("Structural").is_none()); // case-sensitive
        assert!(parse_hook_kind("").is_none());
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
