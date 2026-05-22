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
