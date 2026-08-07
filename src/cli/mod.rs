// cairn:allow-large-module reason: CLI dispatch hub for many subcommands; the natural seam (per-command modules) already exists for newer commands like export and accept; legacy commands grew here historically and a refactor will land in a future phase.
//! CLI registry, command execution, and renderers.

use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    artefacts::registry::{
        Decision, DecisionStatus, Research, Review, ReviewType, Todo, TodoStatus,
    },
    hooks::{self, HookKind},
    map::{
        graph::{Finding, FindingSeverity, NodeRecord},
        query,
    },
    scanner, ui, version_label,
};

pub use crate::query_api::SafetyClass;
use crate::query_api::requires_valid_map;

/// Command metadata.
mod accept;
mod commands;
pub mod export;
mod format;
mod help;
mod render;

// Re-exported so existing `super::copy::`/`crate::cli::copy::` call sites
// across cli submodules resolve unchanged; the module itself lives at
// `crate::copy` (shared with query_api, which also needs copy lookups).
pub(crate) use crate::copy;
pub(crate) use format::render_finding_lines;

use commands::{
    atomic_write, init_project, install_default_pack, legacy_blueprint_warning,
    preflight_wire_check, run_archive_command, run_archive_command_with_path, run_baseline_command,
    run_change_new, run_coord_command, run_decision_command, run_draft_command,
    run_feedback_command, run_gap_command, run_hook_command, run_hook_lifecycle_command,
    run_import_openspec, run_lease_command, run_onboard_command, run_pack_command,
    run_ruling_command, run_shared_json_command, run_todo_command, run_ui_command,
    run_watch_command, run_workspace_command, wire_agent_guide,
};
use format::{
    err, error_output, esc, finding_json, finding_output, findings_output, flag_value, lines,
    node_arg, ok, render_findings,
};
use render::{
    render_backlog, render_brief, render_bundle, render_changes, render_context, render_decisions,
    render_dependencies, render_files, render_get, render_health, render_locate,
    render_neighbourhood, render_next, render_pending_detail, render_rationale, render_remediate,
    render_research, render_show, render_sources, render_status, render_todos,
};

/// Shared CLI command metadata.
pub type CommandMetadata = crate::query_api::ToolMetadata;

/// CLI execution result.
pub struct CliResult {
    /// Process exit code.
    pub code: u8,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
}

/// Returns Phase 1 command registry.
#[must_use]
pub const fn registry() -> &'static [CommandMetadata] {
    crate::query_api::registry()
}

const BROWNFIELD_APPLIED_MARKER: &str = ".cairn/state/brownfield-init-applied";

/// Install the owned base pack, then append the optional orientation pointer.
/// Both operations run only after project scaffolding or brownfield apply has
/// succeeded.
fn bootstrap_agent(root: &Path, wire_file: Option<&str>, wire: bool) -> CliResult {
    let pack = install_default_pack(root, false);
    if pack.code != 0 {
        return pack;
    }
    if !wire {
        return pack;
    }
    let wired = wire_agent_guide(root, wire_file);
    if wired.code != 0 {
        return wired;
    }
    ok(format!("{}\n{}", pack.stdout.trim_end(), wired.stdout))
}

fn finish_brownfield_apply(
    mut result: CliResult,
    archive_path: Option<&Path>,
    root: &Path,
    change_id: &str,
    wire_file: Option<&str>,
    wire: bool,
    json: bool,
) -> CliResult {
    if result.code != 0 {
        result.stderr = format!(
            "brownfield init discovery succeeded, but applying change `{change_id}` failed\n{}",
            result.stderr
        );
        return result;
    }
    let scaffold = init_project(root, wire);
    if scaffold.code != 0 {
        return scaffold;
    }
    let Some(archive_path) = archive_path else {
        return err(
            1,
            &copy::lookup("init.err-completion-marker")
                .replace("{detail}", "the archive command returned no destination"),
        );
    };
    if let Err(marker_error) = record_brownfield_apply(root, archive_path) {
        return marker_error;
    }
    let bootstrap = bootstrap_agent(root, wire_file, wire);
    if bootstrap.code != 0 {
        return bootstrap;
    }
    if json {
        return result;
    }
    result.stdout = format!(
        "brownfield init complete; change `{change_id}` applied to cairn.blueprint\n{}\n{}",
        result.stdout.trim_end(),
        bootstrap.stdout
    );
    result
}

/// Publish success only after archive/apply has returned success. A retry may
/// bootstrap from this marker even if the first pack install or wire failed.
fn record_brownfield_apply(root: &Path, archive_path: &Path) -> Result<(), CliResult> {
    let Some(name) = archive_path.file_name().and_then(|name| name.to_str()) else {
        return Err(err(
            1,
            &copy::lookup("init.err-completion-marker")
                .replace("{detail}", "the archived change name is invalid"),
        ));
    };
    let parent = root.join(".cairn/state");
    let marker = root.join(BROWNFIELD_APPLIED_MARKER);
    atomic_write(&parent, &marker, name).map_err(|detail| {
        err(
            1,
            &copy::lookup("init.err-completion-marker").replace("{detail}", &detail),
        )
    })
}

fn completed_brownfield_archive(root: &Path) -> Option<PathBuf> {
    if root.join("meta/changes/brownfield-init").exists() {
        return None;
    }
    let marker = fs::read_to_string(root.join(BROWNFIELD_APPLIED_MARKER)).ok()?;
    let name = marker.trim();
    if !name.ends_with("-brownfield-init") || Path::new(name).components().count() != 1 {
        return None;
    }
    let archive_path = root.join("meta/changes/archive").join(name);
    archive_path.is_dir().then_some(archive_path)
}

fn resume_brownfield_bootstrap(
    root: &Path,
    archive_path: &Path,
    wire_file: Option<&str>,
    wire: bool,
    json: bool,
) -> CliResult {
    let scaffold = init_project(root, wire);
    if scaffold.code != 0 {
        return scaffold;
    }
    let bootstrap = bootstrap_agent(root, wire_file, wire);
    if bootstrap.code != 0 {
        return bootstrap;
    }
    let summary = copy::lookup("init.brownfield-already-applied");
    if json {
        return ok(format!(
            "{{\"command\":\"archive\",\"status\":\"ok\",\"data\":{{\"archive_path\":\"{}\",\"summary\":\"{}\"}}}}\n",
            esc(&archive_path.to_string_lossy()),
            esc(summary)
        ));
    }
    ok(format!("{summary}\n{}", bootstrap.stdout))
}

/// Executes CLI arguments.
#[must_use]
// Reason: CLI dispatch hub for many subcommands; natural seam is per-command
// modules which already exist for newer commands.
#[allow(clippy::too_many_lines)]
pub fn run(args: &[String]) -> CliResult {
    if args == ["--version"] {
        return ok(format!("{}\n", version_label()));
    }
    if args.is_empty() {
        return ok(help_text());
    }
    if let Some(target) = help::help_request(args) {
        match target {
            help::HelpTarget::Global => return ok(help_text()),
            help::HelpTarget::Command(name) => {
                if let Some(page) = help::command_help_text(name) {
                    return ok(page);
                }
                return unknown_command_error(name);
            }
        }
    }
    if let Err(message) = help::validate_command_flags(args) {
        return err(2, &message);
    }
    let parsed = match parse_args(args) {
        Ok(parsed) => parsed,
        Err(result) => return result,
    };
    let project_root = parsed
        .file
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if parsed.command == "init" {
        let from_code = parsed.command_args.iter().any(|a| a == "--from-code");
        let apply = parsed.command_args.iter().any(|a| a == "--apply");
        let wire = parsed.command_args.iter().any(|a| a == "--wire");
        if apply && !from_code {
            return err(2, "usage: cairn init --from-code --apply");
        }
        // Discovery without application has no project scaffold to wire.
        if wire && from_code && !apply {
            return err(2, copy::lookup("init.wire.err-from-code-conflict"));
        }
        // Extract an optional explicit target file: `--wire CLAUDE.md`.
        // `wire_file` is None (auto-detect) when --wire has no following arg.
        let wire_file: Option<&str> = wire
            .then(|| {
                let idx = parsed.command_args.iter().position(|a| a == "--wire")?;
                parsed
                    .command_args
                    .get(idx + 1)
                    .filter(|a| !a.starts_with("--"))
                    .map(String::as_str)
            })
            .flatten();
        // Preflight: validate the wire target and check for symlink escapes
        // before scaffolding so an invalid path does not create project files.
        if wire {
            let preflight = preflight_wire_check(project_root, wire_file);
            if preflight.code != 0 {
                return preflight;
            }
        }
        let force = parsed.command_args.iter().any(|a| a == "--force");
        if from_code
            && apply
            && !force
            && let Some(archive_path) = completed_brownfield_archive(project_root)
        {
            return resume_brownfield_bootstrap(
                project_root,
                &archive_path,
                wire_file,
                wire,
                parsed.json,
            );
        }
        if from_code {
            return match crate::brownfield::init::run_init_from_code(project_root, force) {
                Ok(change_id) => {
                    if apply {
                        // Delegate to the archive command so `--apply` shares
                        // the conflict gate and path handling of `change apply`.
                        // Use the paths init actually wrote (it hardcodes
                        // cairn.blueprint and meta/changes), not --file /
                        // --changes-dir overrides.
                        let legacy_warning = legacy_blueprint_warning(project_root);
                        let archive_parsed = delegated_archive_args(
                            &parsed,
                            project_root.join("cairn.blueprint"),
                            std::path::PathBuf::from("meta/changes"),
                            &change_id,
                        );
                        let (archive_result, archive_path) = run_archive_command_with_path(
                            &archive_parsed,
                            project_root,
                            legacy_warning,
                        );
                        finish_brownfield_apply(
                            archive_result,
                            archive_path.as_deref(),
                            project_root,
                            &change_id,
                            wire_file,
                            wire,
                            parsed.json,
                        )
                    } else {
                        ok(format!(
                            "brownfield init complete; change written to meta/changes/{change_id}/\n"
                        ))
                    }
                }
                Err(e) => err(1, &e.to_string()),
            };
        }
        let result = init_project(project_root, wire);
        if result.code != 0 {
            return result;
        }
        let bootstrap = bootstrap_agent(project_root, wire_file, wire);
        if bootstrap.code != 0 {
            return bootstrap;
        }
        return ok(format!(
            "{}\n{}",
            result.stdout.trim_end(),
            bootstrap.stdout
        ));
    }
    if parsed.command == "import-openspec" {
        return run_import_openspec(project_root, parsed.json);
    }
    if parsed.command == "refine" {
        return match crate::brownfield::refine::run_refine(project_root) {
            Ok(Some(change_id)) => ok(format!(
                "refine complete; change written to meta/changes/{change_id}/\n"
            )),
            Ok(None) => ok("refine complete; no changes detected\n".to_owned()),
            Err(e) => err(1, &e.to_string()),
        };
    }
    if parsed.command == "ui" {
        return run_ui_command(&parsed);
    }
    if parsed.command == "export" {
        return export::run(
            &parsed.command_args,
            &parsed.file,
            &parsed.changes_dir,
            parsed.json,
        );
    }
    if parsed.command == "onboard" {
        return run_onboard_command(&parsed);
    }
    if parsed.command == "feedback" {
        return run_feedback_command(&parsed, project_root);
    }
    if parsed.command == "watch" {
        let opts = match crate::watch::WatchOpts::from_args(&parsed.command_args[1..]) {
            Err(e) => {
                return CliResult {
                    code: 1,
                    stdout: String::new(),
                    stderr: format!("watch: {e}"),
                };
            }
            Ok(o) => o,
        };
        return run_watch_command(project_root, &opts);
    }

    if parsed.command == "change" {
        return run_change_command(&parsed, project_root);
    }
    if parsed.command == "decision" {
        return run_decision_command(&parsed, project_root);
    }
    if parsed.command == "todo" {
        return run_todo_command(&parsed, project_root);
    }
    if parsed.command == "coord" {
        return run_coord_command(&parsed, project_root);
    }
    if parsed.command == "ruling" {
        return run_ruling_command(&parsed, project_root);
    }
    if parsed.command == "lease" {
        return run_lease_command(&parsed, project_root);
    }
    if parsed.command == "baseline" {
        return run_baseline_command(&parsed, project_root);
    }
    if parsed.command == "pack" {
        return run_pack_command(&parsed, project_root);
    }
    if parsed.command == "workspace" {
        return run_workspace_command(&parsed, project_root);
    }
    if parsed.command == "lint"
        && parsed.command_args.iter().any(|a| a == "--node")
        && !parsed.file.exists()
    {
        // Cycle 3 fix: preserve the legacy `cairn.dsl` migration
        // warning that run_project_command emits at line 145-148.
        // Without this, a user mid-migration from cairn.dsl to
        // cairn.blueprint would see "Run `cairn init`" instead of
        // the rename guidance, and `init` would scaffold over the
        // existing declaration.
        let root = parsed
            .file
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        if parsed.file.ends_with("cairn.blueprint") && root.join("cairn.dsl").exists() {
            if parsed.json {
                return ok(format!(
                    "{{\"command\":\"lint\",\"status\":\"error\",\"data\":{{\"findings\":[{}]}}}}\n",
                    finding_json(&Finding {
                        code: "CAIRN_COMMAND_FAILED".to_owned(),
                        severity: FindingSeverity::Error,
                        message:
                            "no blueprint file was found; rename `cairn.dsl` to `cairn.blueprint`"
                                .to_owned(),
                        node: None,
                        target: None,
                        path: None,
                        deferred_by: None,
                        parked_by: None,
                    })
                ));
            }
            return error_output(
                false,
                "CAIRN_COMMAND_FAILED",
                "no blueprint file was found; rename `cairn.dsl` to `cairn.blueprint`",
            );
        }
        if parsed.json {
            return ok(format!(
                "{{\"command\":\"lint\",\"status\":\"ok\",\"data\":{{\"findings\":[{}]}}}}\n",
                finding_json(&Finding {
                    code: "CAIRN_NO_BLUEPRINT".to_owned(),
                    severity: FindingSeverity::Info,
                    message: "no cairn.blueprint found; run `cairn init` to create one".to_owned(),
                    node: None,
                    target: None,
                    path: None,
                    deferred_by: None,
                    parked_by: None,
                })
            ));
        }

        let body = copy::lookup("empty-states.cli-no-blueprint.body");
        let cta = copy::lookup("empty-states.cli-no-blueprint.cta");
        return ok(format!("{body}\n{cta}\n"));
    }
    run_project_command(&parsed)
}

fn run_change_command(parsed: &ParsedArgs, project_root: &Path) -> CliResult {
    let subcommand = parsed.command_args.get(1).map(String::as_str);
    // `--dry-run` is a recognised `accept` flag, never a change id. Only that
    // exact token is skipped: a typo'd flag stays positional and still fails.
    let dry_run =
        subcommand == Some("accept") && parsed.command_args.iter().any(|arg| arg == "--dry-run");
    let change_id = parsed
        .command_args
        .iter()
        .skip(2)
        .find(|arg| !(dry_run && arg.as_str() == "--dry-run"))
        .map(String::as_str);
    match subcommand {
        Some("new") => {
            let Some(id) = change_id else {
                return err(2, "usage: cairn change new <change-id>");
            };
            run_change_new(project_root, id)
        }
        Some("list") => {
            if parsed.json {
                let root = project_root;
                let legacy_warning = legacy_blueprint_warning(root);
                let request = crate::query_api::QueryRequest {
                    at: None,
                    since: None,
                    tool: "changes".to_owned(),
                    node: None,
                    symbol: None,
                    change: None,
                    old_id: None,
                    new_id: None,
                    status: None,
                    language: None,
                    flags: std::collections::BTreeSet::new(),
                    mutating: false,
                };
                return crate::cli::commands::execute_json_request(
                    parsed,
                    root,
                    legacy_warning,
                    &request,
                );
            }
            let changes_dir = project_root.join(&parsed.changes_dir);
            ok(render_changes(project_root, &changes_dir))
        }
        Some("show") => {
            let Some(id) = change_id else {
                return err(2, "usage: cairn change show <change-id>");
            };
            if parsed.json {
                let root = project_root;
                let legacy_warning = legacy_blueprint_warning(root);
                let request = crate::query_api::QueryRequest {
                    at: None,
                    since: None,
                    tool: "show".to_owned(),
                    node: None,
                    symbol: None,
                    change: Some(id.to_owned()),
                    old_id: None,
                    new_id: None,
                    status: None,
                    language: None,
                    flags: std::collections::BTreeSet::new(),
                    mutating: false,
                };
                return crate::cli::commands::execute_json_request(
                    parsed,
                    root,
                    legacy_warning,
                    &request,
                );
            }
            // render_show reads change id from command_args[1]; synthesise
            // a view of args as the old top-level `show <id>` shape.
            let show_parsed = delegated_show_args(parsed, id);
            match render_show(&show_parsed, project_root) {
                Ok(stdout) => ok(stdout),
                Err(finding) => error_output(parsed.json, &finding.code, &finding.message),
            }
        }
        Some("accept") => {
            crate::cli::accept::run_accept_gate(project_root, change_id, parsed.json, dry_run)
        }
        Some(cmd @ ("archive" | "apply")) => {
            let Some(id) = change_id else {
                return err(2, &format!("usage: cairn change {cmd} <change-id>"));
            };
            let root = project_root;
            let legacy_warning = legacy_blueprint_warning(root);
            // archive reads change id from command_args[1]; synthesise
            // the old top-level `archive <id>` shape.
            let archive_parsed =
                delegated_archive_args(parsed, parsed.file.clone(), parsed.changes_dir.clone(), id);
            run_archive_command(&archive_parsed, root, legacy_warning)
        }
        _ => err(
            2,
            "usage: cairn change <new|list|show|accept|apply|archive> [args]",
        ),
    }
}

fn delegated_archive_args(
    parsed: &ParsedArgs,
    file: std::path::PathBuf,
    changes_dir: std::path::PathBuf,
    id: &str,
) -> ParsedArgs {
    ParsedArgs {
        json: parsed.json,
        strict: parsed.strict,
        file,
        changes_dir,
        command: "archive".to_owned(),
        command_args: vec!["archive".to_owned(), id.to_owned()],
        verbose: parsed.verbose,
        brief: parsed.brief,
    }
}

fn delegated_show_args(parsed: &ParsedArgs, id: &str) -> ParsedArgs {
    ParsedArgs {
        json: parsed.json,
        strict: parsed.strict,
        file: parsed.file.clone(),
        changes_dir: parsed.changes_dir.clone(),
        command: "show".to_owned(),
        command_args: vec!["show".to_owned(), id.to_owned()],
        verbose: parsed.verbose,
        brief: parsed.brief,
    }
}

#[cfg(test)]
mod delegation_tests {
    use super::*;
    fn parent(verbose: bool, brief: bool) -> ParsedArgs {
        ParsedArgs {
            json: false,
            strict: false,
            verbose,
            brief,
            file: std::path::PathBuf::from("cairn.blueprint"),
            changes_dir: std::path::PathBuf::from("meta/changes"),
            command: "change".to_owned(),
            command_args: vec!["change".to_owned()],
        }
    }
    #[test]
    fn delegated_archive_args_propagates_flags() {
        let p = parent(true, true);
        let a = delegated_archive_args(
            &p,
            std::path::PathBuf::from("f"),
            std::path::PathBuf::from("c"),
            "x",
        );
        assert!(a.verbose && a.brief && a.command == "archive");
        let p2 = parent(false, false);
        let a2 = delegated_archive_args(
            &p2,
            std::path::PathBuf::from("f"),
            std::path::PathBuf::from("c"),
            "x",
        );
        assert!(!a2.verbose && !a2.brief);
    }
    #[test]
    fn delegated_show_args_propagates_flags() {
        let p = parent(true, false);
        let s = delegated_show_args(&p, "x");
        assert!(s.verbose && !s.brief && s.command == "show");
    }
}
// Reason: CLI parsing keeps independent output and validation mode flags together.
#[allow(clippy::struct_excessive_bools)]
struct ParsedArgs {
    json: bool,
    strict: bool,
    verbose: bool,
    brief: bool,
    file: PathBuf,
    changes_dir: PathBuf,
    command: String,
    command_args: Vec<String>,
}

fn parse_args(args: &[String]) -> Result<ParsedArgs, CliResult> {
    let mut json = false;

    let mut strict = false;
    let mut verbose = false;
    let mut brief = false;
    let mut file = PathBuf::from("cairn.blueprint");
    let mut changes_dir = PathBuf::from("meta/changes");
    let mut command_args = Vec::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--json" => json = true,
            "--strict" => strict = true,
            "--verbose" => verbose = true,
            "--brief" => brief = true,
            "--file" => {
                let Some(value) = iter.next() else {
                    return Err(err(2, "--file requires a path"));
                };
                file = PathBuf::from(value);
            }
            "--changes-dir" => {
                let Some(value) = iter.next() else {
                    return Err(err(2, "--changes-dir requires a path"));
                };
                changes_dir = PathBuf::from(value);
            }
            value => command_args.push(value.to_owned()),
        }
    }
    let Some(command) = command_args.first().map(String::as_str) else {
        return Err(err(2, copy::lookup("errors.usage")));
    };

    Ok(ParsedArgs {
        json,
        strict,
        verbose,
        brief,
        file,
        changes_dir,
        command: command.to_owned(),
        command_args,
    })
}
fn run_project_command(parsed: &ParsedArgs) -> CliResult {
    let root = parsed
        .file
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if parsed.file.ends_with("cairn.blueprint")
        && !parsed.file.exists()
        && root.join("cairn.dsl").exists()
    {
        return error_output(
            parsed.json,
            "CAIRN_COMMAND_FAILED",
            "no blueprint file was found; rename `cairn.dsl` to `cairn.blueprint`",
        );
    }
    let legacy_warning = legacy_blueprint_warning(root);
    if parsed.command == "hook"
        && matches!(
            parsed.command_args.get(1).map(String::as_str),
            Some("install" | "status" | "uninstall")
        )
    {
        return run_hook_lifecycle_command(parsed, root);
    }
    let grep_decisions =
        parsed.command == "decisions" && parsed.command_args.iter().any(|arg| arg == "--grep");
    let node_scoped_lint =
        parsed.command == "lint" && parsed.command_args.iter().any(|arg| arg == "--node");
    if parsed.command == "deps"
        && let Some(direction) = flag_value(&parsed.command_args, "--direction")
        && direction != "in"
        && direction != "out"
    {
        return error_output(
            parsed.json,
            "CAIRN_COMMAND_FAILED",
            &format!("invalid --direction value `{direction}`; expected `in` or `out`"),
        );
    }
    if parsed.command == "draft" {
        return run_draft_command(parsed, root, legacy_warning);
    }
    if parsed.json
        && uses_shared_json(parsed.command.as_str())
        && !grep_decisions
        && !node_scoped_lint
    {
        return run_shared_json_command(parsed, root, legacy_warning);
    }
    let scan_result = if parsed.command == "scan" {
        scanner::scan(root, &parsed.file)
    } else {
        scanner::load_project(root, &parsed.file)
    };
    let scan_result = match scan_result {
        Ok(result) => result,
        Err(error) => return error_output(parsed.json, "CAIRN_COMMAND_FAILED", &error),
    };
    if requires_valid_map(parsed.command.as_str()) && scan_result.graph.has_errors() {
        return findings_output(parsed.json, parsed.verbose, &scan_result.graph.findings);
    }
    render_loaded_project_command(parsed, root, &scan_result, legacy_warning)
}

// Reason: project-loaded command routing has many one-liner arms; each is
// already delegated to its own render function.
#[allow(clippy::too_many_lines)]
fn render_loaded_project_command(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
    legacy_warning: String,
) -> CliResult {
    match parsed.command.as_str() {
        "get" => render_get(parsed, root, scan_result),
        "neighbourhood" => render_neighbourhood(parsed, root, scan_result),
        "files" => render_files(parsed, root),
        "locate" => render_locate(parsed, root, scan_result),
        "bundle" => render_bundle(parsed, root, scan_result),
        "gap" => return run_gap_command(parsed, root, scan_result),
        "todos" => render_todos(parsed, root),
        "decisions" => render_decisions(parsed, scan_result),
        "research" => render_research(parsed, root),
        "sources" => render_sources(parsed, root),
        "rationale" => render_rationale(parsed, root),
        "status" => Ok(render_status(parsed, scan_result, root)),
        "context" => render_context(parsed, root, scan_result),
        "backlog" => render_backlog(parsed, root, scan_result),
        "hook" => return run_hook_command(parsed, root, scan_result, legacy_warning),
        "health" => Ok(render_health(parsed, root, scan_result)),
        "remediate" => Ok(render_remediate(parsed, root, scan_result)),
        "next" => Ok(render_next(parsed, root, scan_result)),
        "brief" => Ok(render_brief(parsed, root, scan_result)),
        "docstring" | "rename" => {
            return err(2, "this command currently requires --json");
        }
        "contract" => node_arg(&parsed.command_args).and_then(|node| {
            let node = scan_result.graph.resolve(node)?;
            let body = node
                .contracts
                .iter()
                .find_map(|path| scan_result.contracts.contracts.get(path))
                .filter(|contract| contract.node == node.id)
                .map(|contract| contract.body.clone())
                .unwrap_or_default();
            Ok(format!("Contract for {}:\n{}\n", node.id, body))
        }),
        "islands" => {
            let response = query::islands(&scan_result.graph);
            let mut out = String::new();
            for (i, island) in response.islands.iter().enumerate() {
                let _ = writeln!(
                    out,
                    "Island {}: {} ({} node{})",
                    i + 1,
                    island.representative,
                    island.node_count,
                    if island.node_count == 1 { "" } else { "s" }
                );
            }
            Ok(out)
        }
        "order" => match query::order(&scan_result.graph) {
            Ok(response) => Ok(format!("Order:\n{}\n", lines(&response.nodes))),
            Err(findings) => return findings_output(parsed.json, parsed.verbose, &findings),
        },
        "frontier" => match query::frontier(&scan_result.graph) {
            Ok(response) => {
                let mut out = String::from("Ready:\n");
                if response.ready.is_empty() {
                    out.push_str("  (none)\n");
                } else {
                    for entry in &response.ready {
                        let _ = writeln!(out, "  {} (tier {})", entry.node, entry.tier);
                    }
                }
                out.push_str("\nBlocked:\n");
                if response.blocked.is_empty() {
                    out.push_str("  (none)\n");
                } else {
                    for entry in &response.blocked {
                        let _ = writeln!(
                            out,
                            "  {}: blocked by {}",
                            entry.node,
                            entry.blocking.join(", ")
                        );
                    }
                }
                Ok(out)
            }
            Err(findings) => return findings_output(parsed.json, parsed.verbose, &findings),
        },
        "roadmap" => {
            let response =
                crate::query_api::roadmap_response(root, &scan_result.artefacts.todos);
            let mut out = String::new();
            if response.tiers.is_empty() {
                let _ = writeln!(out, "{}", copy::lookup("roadmap.empty"));
            }
            for tier in &response.tiers {
                let _ = writeln!(
                    out,
                    "{}",
                    copy::lookup("roadmap.tier-header").replace("{tier}", &tier.tier.to_string())
                );
                for item in &tier.items {
                    let parent = item
                        .parent
                        .as_deref()
                        .map(|parent| format!(" [{parent}]"))
                        .unwrap_or_default();
                    let _ = writeln!(
                        out,
                        "  {} ({}){parent} {}",
                        item.stem, item.status, item.path
                    );
                }
            }
            Ok(out)
        }
        "pending" => match crate::query_api::pending_rows(root, scan_result) {
            Ok(rows) => {
                if let Some(id) = parsed.command_args.get(1) {
                    rows.iter().find(|row| row.id == *id).map_or_else(
                        || {
                            Err(Finding {
                                code: "CAIRN_COMMAND_FAILED".to_owned(),
                                severity: FindingSeverity::Error,
                                message: copy::lookup("pending.not-found").replace("{id}", id),
                                node: None,
                                target: None,
                                path: None,
                                deferred_by: None,
                                parked_by: None,
                            })
                        },
                        |row| Ok(render_pending_detail(row)),
                    )
                } else {
                    let mut out = format!("{}\n", copy::lookup("pending.header"));
                    if rows.is_empty() {
                        let _ = writeln!(out, "{}", copy::lookup("pending.none"));
                    } else {
                        for row in &rows {
                            let key = if row.changed_since_review {
                                "pending.row-changed"
                            } else {
                                "pending.row"
                            };
                            let _ = writeln!(
                                out,
                                "{}",
                                copy::lookup(key)
                                    .replace("{id}", &row.id)
                                    .replace("{age}", &row.age_days.to_string())
                                    .replace("{ratification}", row.ratification.as_str())
                                    .replace("{nodes}", &row.nodes.join(", "))
                            );
                        }
                    }
                    Ok(out)
                }
            }
            Err(error) => return error_output(parsed.json, &error.code, &error.message),
        },
        "deps" => render_dependencies(parsed, root),
        // Spine ops (webui-first): the human rendering is the pretty canonical
        // JSON; the primary consumers are the webui and --json callers.
        "ui_meta" | "blueprint" | "beads" | "graph" => {
            let request = crate::query_api::QueryRequest {
                tool: parsed.command.clone(),
                node: parsed.command_args.get(1).cloned(),
                ..Default::default()
            };
            match crate::query_api::execute_with_scan(
                root,
                &parsed.file,
                &root.join(&parsed.changes_dir),
                &request,
                scan_result,
            ) {
                Ok(response) => Ok(format!(
                    "{}\n",
                    serde_json::to_string_pretty(&response.data).unwrap_or_default()
                )),
                Err(error) => {
                    return error_output(parsed.json, "CAIRN_COMMAND_FAILED", &error.message)
                }
            }
        }
        "lint" | "scan" => {
            let response = query::lint(&scan_result.graph);
            if parsed.command == "lint"
                && parsed.command_args.iter().any(|a| a == "--node")
            {
                // `cairn lint --node <id>` is the folded spelling of the
                // former `cairn check <node>`: node-scoped, non-blocking.
                let target_node = flag_value(&parsed.command_args, "--node");
                let findings: Vec<_> = response
                    .findings
                    .iter()
                    .filter(|f| {
                        target_node.is_none_or(|t| f.node.as_deref().is_some_and(|n| n == t))
                    })
                    .cloned()
                    .collect();
                let has_errors = findings
                    .iter()
                    .any(|f| f.severity == FindingSeverity::Error);
                let stdout = if parsed.json {
                    format!(
                        "{{\"command\":\"lint\",\"status\":\"{}\",\"data\":{{\"findings\":[{}]}}}}\n",
                        if has_errors { "error" } else { "ok" },
                        findings.iter().map(finding_json).collect::<Vec<_>>().join(",")
                    )
                } else {
                render_findings(&findings, false, parsed.verbose)
                };
                return CliResult {
                    code: u8::from(has_errors),
                    stdout,
                    stderr: legacy_warning,
                };
            }
            let code = if parsed.strict {
                u8::from(!crate::map::graph::strict_green(&response.findings))
            } else {
                u8::from(
                    response
                        .findings
                        .iter()
                        .any(|finding| finding.severity == FindingSeverity::Error),
                )
            };
            let stdout = render_findings(&response.findings, parsed.json, parsed.verbose);
            return CliResult {
                code,
                stdout,
                stderr: legacy_warning,
            };
        }
        other => return unknown_command_error(other),
    }
    .map_or_else(
        |finding| finding_output(parsed.json, parsed.verbose, finding),
        |stdout| CliResult {
            code: 0,
            stdout,
            stderr: legacy_warning,
        },
    )
}

/// Commands handled by the CLI but not present in `query_api::registry`
/// (no JSON request/response on the query wire). The registry remains the
/// single source of truth for every query-api tool's name and description;
/// this table covers only the genuinely CLI-only commands, which surface in
/// help without any entry in a registry-derived list.
struct CliOnlyCommand {
    /// CLI command name.
    name: &'static str,
    /// Human-readable one-line description, shown in CLI help.
    description: &'static str,
}

const CLI_ONLY_COMMANDS: &[CliOnlyCommand] = &[
    CliOnlyCommand {
        name: "backlog",
        description: "List beads (issues) linked to a node",
    },
    CliOnlyCommand {
        name: "baseline",
        description: "Record or drop a node's contract baseline without a summariser",
    },
    CliOnlyCommand {
        name: "brief",
        description: "Fused next-unit brief: task, decisions, contract, gates",
    },
    CliOnlyCommand {
        name: "change",
        description: "Manage changes: new, list, show, accept, apply, archive",
    },
    CliOnlyCommand {
        name: "coord",
        description: "Verify or compact the family-local coordination fact store",
    },
    CliOnlyCommand {
        name: "decision",
        description: "Scaffold a new decision artefact",
    },
    CliOnlyCommand {
        name: "draft",
        description: "Manage draft proposals: list, show, edit, discard, accept, create",
    },
    CliOnlyCommand {
        name: "export",
        description: "Export project data",
    },
    CliOnlyCommand {
        name: "feedback",
        description: "Record cairn friction and get an upstream issue link",
    },
    CliOnlyCommand {
        name: "gap",
        description: "Log an unresolved question as a proposed decision artefact",
    },
    CliOnlyCommand {
        name: "import-openspec",
        description: "Migrate openspec changes to meta/changes",
    },
    CliOnlyCommand {
        name: "lease",
        description: "Coordination lease and driver-singleton facts: lease list",
    },
    CliOnlyCommand {
        name: "next",
        description: "Show the next ready unit of work",
    },
    CliOnlyCommand {
        name: "onboard",
        description: "Suggest blueprint entries for orphaned files",
    },
    CliOnlyCommand {
        name: "pack",
        description: "Install, update, inspect, or remove the agent pack",
    },
    CliOnlyCommand {
        name: "ruling",
        description: "Coordination ruling facts: ruling list, ruling show <fact-id>",
    },
    CliOnlyCommand {
        name: "todo",
        description: "Scaffold todo artefacts and edit their status and relationship links",
    },
    CliOnlyCommand {
        name: "workspace",
        description: "Aggregate status, lint, and frontier queries across a cairn.workspace",
    },
];

/// MCP-only tools that should not appear in CLI command lists.
const MCP_ONLY_TOOLS: &[&str] = &["init_from_code"];

/// Top-level spellings retired under `cairn change <sub>`; kept out of
/// CLI help but retained on the wire.
pub(crate) const RETIRED_TOP_LEVEL: &[&str] = &["accept", "archive", "changes", "show"];

/// Returns all command names the CLI recognises: every non-compound,
/// non-MCP-only, non-retired registry tool plus the CLI-only table.
#[must_use]
pub fn all_command_names() -> Vec<&'static str> {
    let mut names: Vec<&str> = registry()
        .iter()
        .filter(|t| !MCP_ONLY_TOOLS.contains(&t.cli_name))
        // Compound cli_names (e.g. "draft list") are subcommands, not
        // top-level commands.
        .filter(|t| !t.cli_name.contains(' '))
        .filter(|t| !RETIRED_TOP_LEVEL.contains(&t.cli_name))
        .map(|t| t.cli_name)
        .collect();
    for cmd in CLI_ONLY_COMMANDS {
        if !names.contains(&cmd.name) {
            names.push(cmd.name);
        }
    }
    names.sort_unstable();
    names.dedup();
    names
}

/// Returns the human-readable description for a CLI command.
///
/// Sourced from `query_api::registry()` for every query-api tool and from
/// the CLI-only table otherwise. The registry is the single source of truth,
/// so adding a registry operation makes it appear in CLI help with no
/// hand-maintained list edit.
pub(crate) fn command_description(name: &str) -> &'static str {
    if let Some(tool) = registry().iter().find(|t| t.cli_name == name) {
        return tool.description;
    }
    CLI_ONLY_COMMANDS
        .iter()
        .find(|c| c.name == name)
        .map_or("", |c| c.description)
}

/// Generates the `--help` output for the CLI.
fn help_text() -> String {
    let mut out = format!(
        "{}\n\nUsage: cairn <command> [options]\n\nCommands:\n",
        version_label()
    );
    let names = all_command_names();
    let max_width = names.iter().map(|n| n.len()).max().unwrap_or(0);
    for name in &names {
        let desc = command_description(name);
        let _ = writeln!(out, "  {name:<max_width$}  {desc}");
    }
    out.push_str("\nOptions:\n");
    out.push_str("  --file <path>         Blueprint file (default: cairn.blueprint)\n");
    out.push_str("  --changes-dir <path>  Changes directory (default: meta/changes)\n");
    out.push_str("  --json                Output in JSON format\n");
    out.push_str("  --strict              Exit 1 on Warning findings (scan/lint)\n");
    out.push_str("  --depth <N|all>       context: cap structure depth (default 1)\n");
    out.push_str("  --scope <node>        context: full detail for one subtree\n");
    out.push_str("  --version             Print version\n");
    out.push_str("  -h, --help            Print this help\n");
    out.push_str("\nExit codes:\n");
    out.push_str("  0  Success; no blocking findings\n");
    out.push_str("  1  Blocking findings, or command failed\n");
    out.push_str("  2  Usage error (unknown command, missing argument)\n");
    out
}

/// Levenshtein edit distance between two strings.
fn edit_distance(a: &str, b: &str) -> usize {
    let b_len = b.len();
    let mut previous: Vec<usize> = (0..=b_len).collect();
    let mut current = vec![0; b_len + 1];
    for (i, a_char) in a.chars().enumerate() {
        current[0] = i + 1;
        for (j, b_char) in b.chars().enumerate() {
            let cost = usize::from(a_char != b_char);
            current[j + 1] = (previous[j] + cost)
                .min(current[j] + 1)
                .min(previous[j + 1] + 1);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[b_len]
}

/// Builds an error message for an unknown command, suggesting close matches.
fn unknown_command_error(input: &str) -> CliResult {
    let names = all_command_names();
    let best = names
        .iter()
        .map(|name| (*name, edit_distance(input, name)))
        .min_by_key(|(_, dist)| *dist);
    let base = copy::lookup("errors.unknown-command");
    let message = match best {
        Some((suggestion, dist)) if dist <= 2 => {
            format!("{base} '{input}'. Did you mean '{suggestion}'?")
        }
        _ => format!("{base} '{input}'. Available commands: {}", names.join(", ")),
    };
    err(2, &message)
}

fn uses_shared_json(command: &str) -> bool {
    matches!(
        command,
        "get"
            | "neighbourhood"
            | "contract"
            | "docstring"
            | "files"
            | "locate"
            | "bundle"
            | "deps"
            | "order"
            | "islands"
            | "frontier"
            | "graph"
            | "lint"
            | "scan"
            | "status"
            | "pending"
            | "roadmap"
            | "rationale"
            | "todos"
            | "decisions"
            | "research"
            | "sources"
            // changes/show retired under `cairn change list|show`
            | "hook"
            | "rename"
            | "context"
            | "health"
            | "remediate"
            | "ui_meta"
            | "blueprint"
            | "beads"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_cli_core_commands_support_human_and_json_output()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("core-commands")?;
        write_project(&root)?;
        let cases = [
            ("get", vec!["get", "app.api"]),
            (
                "neighbourhood",
                vec!["neighbourhood", "app.api", "--include-todos"],
            ),
            ("files", vec!["files", "app.api"]),
            ("todos", vec!["todos", "app.api"]),
            ("decisions", vec!["decisions", "app.api"]),
            ("research", vec!["research", "app.api"]),
            ("sources", vec!["sources", "app.api"]),
            ("rationale", vec!["rationale", "app.api"]),
            ("status", vec!["status"]),
            ("context", vec!["context"]),
            ("deps (in)", vec!["deps", "app.api", "--direction", "in"]),
            ("deps (out)", vec!["deps", "app.api"]),
            ("contract", vec!["contract", "app.api"]),
            ("bundle", vec!["bundle", "app.api"]),
            ("order", vec!["order"]),
            ("frontier", vec!["frontier"]),
            ("pending", vec!["pending"]),
            ("lint", vec!["lint"]),
            ("scan", vec!["scan"]),
            ("hook", vec!["hook", "all"]),
            ("ui_meta", vec!["ui_meta"]),
            ("blueprint", vec!["blueprint"]),
            ("beads", vec!["beads", "app.api"]),
            ("graph", vec!["graph"]),
        ];

        for (name, command) in cases {
            let human = run_in(&root, &command);
            assert_eq!(human.code, 0, "{name} human stderr: {}", human.stderr);
            assert!(!human.stdout.is_empty(), "{name} human output");

            let mut json_command = vec!["--json".to_owned()];
            json_command.extend(command.iter().map(|value| (*value).to_owned()));
            let json = run_in_str(&root, &json_command);
            assert_eq!(json.code, 0, "{name} json stderr: {}", json.stderr);
            assert!(
                json.stdout.trim_start().starts_with('{'),
                "{name} json output"
            );
        }

        Ok(())
    }
    #[test]
    fn test_registry_tools_surface_in_cli_help_without_hand_list() {
        // Proof of the derived property: every non-compound, non-MCP-only,
        // non-retired query-api tool must reach CLI help purely via
        // `query_api::registry()`, with no entry in the CLI-only hand table.
        let help = help_text();
        for tool in registry() {
            if tool.cli_name.contains(' ') {
                continue; // compound subcommand, not a top-level name
            }
            if MCP_ONLY_TOOLS.contains(&tool.cli_name) {
                continue;
            }
            if RETIRED_TOP_LEVEL.contains(&tool.cli_name) {
                continue;
            }
            assert!(
                all_command_names().contains(&tool.cli_name),
                "registry tool `{}` missing from CLI command names",
                tool.cli_name
            );
            assert!(
                help.contains(tool.cli_name),
                "registry tool `{}` missing from CLI help text",
                tool.cli_name
            );
            assert!(
                !tool.description.is_empty(),
                "registry tool `{}` missing a description",
                tool.cli_name
            );
            // The CLI-only hand table must not duplicate registry tools, or
            // the claim that the registry alone drives help would be hollow.
            assert!(
                !CLI_ONLY_COMMANDS.iter().any(|c| c.name == tool.cli_name),
                "registry tool `{}` is also in the CLI-only table",
                tool.cli_name
            );
        }
    }

    #[test]
    fn test_deps_rejects_invalid_direction_value() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("deps-bad-direction")?;
        write_project(&root)?;
        for args in [
            vec!["deps", "app.api", "--direction", "sideways"],
            vec!["--json", "deps", "app.api", "--direction", "sideways"],
        ] {
            let result = run_in(&root, &args);
            assert_eq!(result.code, 1, "invalid direction must fail: {args:?}");
            assert!(
                result.stdout.contains("expected `in` or `out`"),
                "usage guidance missing: {}",
                result.stdout
            );
        }
        Ok(())
    }

    #[test]
    fn test_cli_change_commands_and_error_paths() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("change-commands")?;
        write_project(&root)?;
        write_change(&root)?;

        let changes = run_in(&root, &["--json", "change", "list"]);
        assert_eq!(changes.code, 0);
        assert!(changes.stdout.contains("phase-7.5a-test-fortification"));

        let show = run_in(
            &root,
            &["--json", "change", "show", "phase-7.5a-test-fortification"],
        );
        assert_eq!(show.code, 0);
        assert!(
            show.stdout
                .contains("\"title\":\"Phase 7.5a Test Fortification\"")
        );

        let rename = run_in(&root, &["--json", "rename", "app.api", "app.api.v2"]);
        assert_eq!(rename.code, 0);
        assert!(
            rename
                .stdout
                .contains("\"id\":\"rename-app.api-to-app.api.v2\"")
        );

        let archive_usage = run_in(&root, &["change", "archive"]);
        assert_eq!(archive_usage.code, 2);
        assert!(archive_usage.stderr.contains("usage: cairn change archive"));

        let missing = run_in(&root, &["get"]);
        assert_eq!(missing.code, 1);
        assert!(missing.stdout.contains("CAIRN_CLI_MISSING_NODE"));

        let unknown = run_in(&root, &["unknown"]);
        assert_eq!(unknown.code, 2);
        assert!(unknown.stderr.contains("unknown command"));

        Ok(())
    }

    #[test]
    fn test_cli_archive_moves_completed_change() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("archive-move")?;
        write_project(&root)?;
        write_change(&root)?;

        let archive = run_in(
            &root,
            &["change", "archive", "phase-7.5a-test-fortification"],
        );
        assert_eq!(archive.code, 0, "stderr: {}", archive.stderr);
        assert!(archive.stdout.contains("Archived"));
        assert!(
            !root
                .join("meta/changes/phase-7.5a-test-fortification")
                .exists(),
            "active change directory must move out of meta/changes"
        );
        let archive_root = root.join("meta/changes/archive");
        assert!(archive_root.exists(), "archive directory must be created");
        let moved = fs::read_dir(&archive_root)?
            .filter_map(Result::ok)
            .any(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .ends_with("phase-7.5a-test-fortification")
            });
        assert!(moved, "change must be archived under a dated directory");

        Ok(())
    }

    #[test]
    fn test_cli_archive_json_envelope() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("archive-json")?;
        write_project(&root)?;
        write_change(&root)?;

        let archive = run_in(
            &root,
            &[
                "--json",
                "change",
                "archive",
                "phase-7.5a-test-fortification",
            ],
        );
        assert_eq!(archive.code, 0, "stderr: {}", archive.stderr);
        assert!(archive.stdout.contains("\"command\":\"archive\""));
        assert!(archive.stdout.contains("\"status\":\"ok\""));
        assert!(archive.stdout.contains("\"archive_path\":"));

        Ok(())
    }

    #[test]
    fn test_cli_apply_aliases_archive() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("apply-alias")?;
        write_project(&root)?;
        write_change(&root)?;

        // `change apply` without an id shows a usage error.
        let usage = run_in(&root, &["change", "apply"]);
        assert_eq!(usage.code, 2);
        assert!(usage.stderr.contains("usage: cairn change apply"));

        // `change apply <id>` archives the change, same as `change archive`.
        let apply = run_in(&root, &["change", "apply", "phase-7.5a-test-fortification"]);
        assert_eq!(apply.code, 0, "stderr: {}", apply.stderr);
        assert!(apply.stdout.contains("Archived"));
        assert!(
            !root
                .join("meta/changes/phase-7.5a-test-fortification")
                .exists(),
            "active change directory must move out of meta/changes"
        );
        let archive_root = root.join("meta/changes/archive");
        assert!(archive_root.exists(), "archive directory must be created");
        let moved = fs::read_dir(&archive_root)?
            .filter_map(Result::ok)
            .any(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .ends_with("phase-7.5a-test-fortification")
            });
        assert!(moved, "change must be archived under a dated directory");

        Ok(())
    }

    #[test]
    fn test_cli_init_and_version_commands() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init")?;
        let init = run_in(&root, &["init"]);
        assert_eq!(init.code, 0);
        assert!(init.stdout.contains("Next steps"));
        assert!(root.join("cairn.blueprint").exists());
        assert!(root.join("cairn.config.yaml").exists());
        let guide = fs::read_to_string(root.join(".cairn/AGENTS.md"))?;
        assert!(guide.contains("cairn feedback"));
        assert!(
            root.join(".cairn/state/agent-pack.json").exists(),
            "init must delegate pack ownership to the lifecycle engine"
        );

        let version = run(&["--version".to_owned()]);
        assert_eq!(version.code, 0);
        assert!(version.stdout.contains("cairn "));

        Ok(())
    }

    #[test]
    fn test_cli_init_from_code_apply_populates_blueprint() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = temp_root("init-from-code-apply")?;
        fs::create_dir_all(root.join("src/alpha"))?;
        for i in 0..3 {
            fs::write(root.join(format!("src/alpha/f{i}.rs")), "pub fn f() {}\n")?;
        }
        let result = run_in(&root, &["init", "--from-code", "--apply"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        assert!(
            result.stdout.contains("applied to cairn.blueprint"),
            "stdout: {}",
            result.stdout
        );
        let blueprint = fs::read_to_string(root.join("cairn.blueprint"))?;
        assert!(
            blueprint.contains(r#"id "src.alpha""#),
            "blueprint must gain discovered nodes: {blueprint}"
        );
        assert!(
            !root.join("meta/changes/brownfield-init").exists(),
            "change must be archived out of meta/changes"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_from_code_apply_json_emits_archive_envelope()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-from-code-apply-json")?;
        fs::create_dir_all(root.join("src/alpha"))?;
        for i in 0..3 {
            fs::write(root.join(format!("src/alpha/f{i}.rs")), "pub fn f() {}\n")?;
        }
        let result = run_in(&root, &["--json", "init", "--from-code", "--apply"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        let parsed: serde_json::Value = serde_json::from_str(result.stdout.trim())
            .unwrap_or_else(|e| panic!("json mode must emit valid JSON ({e}): {}", result.stdout));
        assert_eq!(parsed["command"], "archive");
        assert!(result.stderr.is_empty(), "stderr: {}", result.stderr);
        Ok(())
    }

    #[test]
    fn test_cli_init_apply_without_from_code_is_usage_error()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-apply-alone")?;
        let result = run_in(&root, &["init", "--apply"]);
        assert_eq!(result.code, 2);
        assert!(
            result
                .stderr
                .contains("usage: cairn init --from-code --apply")
        );
        assert!(
            !root.join("cairn.blueprint").exists(),
            "usage error must not scaffold the project"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_appends_reference_to_agents_md() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = temp_root("init-wire")?;
        let result = run_in(&root, &["init", "--wire"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        assert!(
            result.stdout.contains("Wired"),
            "init --wire must report the wiring: {}",
            result.stdout
        );
        let agents = fs::read_to_string(root.join("AGENTS.md"))?;
        assert!(
            agents.contains("cairn:agent-guide-begin"),
            "init --wire must append the orientation block to AGENTS.md"
        );
        assert!(
            agents.contains(".cairn/AGENTS.md"),
            "wired block must reference the agent guide"
        );
        assert!(
            root.join(".cairn/state/agent-pack.json").exists(),
            "init --wire must publish the pack ownership manifest"
        );
        assert!(
            root.join(".claude/skills/cairn-dev/SKILL.md").exists(),
            "init --wire must install the canonical router"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_with_unapplied_from_code_is_rejected()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-from-code")?;
        let result = run_in(&root, &["init", "--from-code", "--wire"]);
        assert_eq!(result.code, 2);
        assert!(
            result.stderr.contains("--from-code --apply --wire"),
            "must require applying discovery before wiring: {}",
            result.stderr
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_from_code_apply_wire_bootstraps_after_apply()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-from-code-apply-wire")?;
        fs::create_dir_all(root.join("src/alpha"))?;
        for i in 0..3 {
            fs::write(root.join(format!("src/alpha/f{i}.rs")), "pub fn f() {}\n")?;
        }

        let unrelated_archive = root.join("meta/changes/archive/9999-12-31-other-brownfield-init");
        fs::create_dir_all(&unrelated_archive)?;

        let result = run_in(&root, &["init", "--from-code", "--apply", "--wire"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        assert!(
            fs::read_to_string(root.join("cairn.blueprint"))?.contains(r#"id "src.alpha""#),
            "discovered blueprint must be applied before bootstrap"
        );
        assert!(
            root.join(".cairn/state/agent-pack.json").exists(),
            "brownfield bootstrap must publish ownership"
        );
        assert!(
            root.join(".claude/skills/cairn-dev/SKILL.md").exists(),
            "brownfield bootstrap must install the router"
        );
        assert!(
            fs::read_to_string(root.join("AGENTS.md"))?.contains("cairn:agent-guide-begin"),
            "brownfield bootstrap must wire the orientation pointer"
        );
        assert_ne!(
            fs::read_to_string(root.join(BROWNFIELD_APPLIED_MARKER))?,
            "9999-12-31-other-brownfield-init",
            "completion must name the archive returned by this apply"
        );
        fs::remove_dir(unrelated_archive)?;
        let manifest_before = fs::read(root.join(".cairn/state/agent-pack.json"))?;
        let agents_before = fs::read_to_string(root.join("AGENTS.md"))?;
        let repeated = run_in(&root, &["init", "--from-code", "--apply", "--wire"]);
        assert_eq!(repeated.code, 0, "re-run stderr: {}", repeated.stderr);
        assert_eq!(
            fs::read(root.join(".cairn/state/agent-pack.json"))?,
            manifest_before,
            "re-run must preserve the ownership ledger bytes"
        );
        assert_eq!(
            fs::read_to_string(root.join("AGENTS.md"))?,
            agents_before,
            "re-run must not duplicate the wire block"
        );
        assert!(
            !root.join("meta/changes/brownfield-init").exists(),
            "re-run must not leave a duplicate active change"
        );

        let repeated_json = run_in(
            &root,
            &["--json", "init", "--from-code", "--apply", "--wire"],
        );
        assert_eq!(repeated_json.code, 0);
        let envelope: serde_json::Value = serde_json::from_str(&repeated_json.stdout)?;
        assert_eq!(envelope["command"], "archive");
        Ok(())
    }

    #[test]
    fn test_failed_brownfield_apply_never_bootstraps_or_wires()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-from-code-failed-apply")?;
        let failed = CliResult {
            code: 1,
            stdout: String::new(),
            stderr: "conflict".to_owned(),
        };
        let result =
            finish_brownfield_apply(failed, None, &root, "brownfield-init", None, true, false);
        assert_eq!(result.code, 1);
        assert!(!root.join(".cairn/state/agent-pack.json").exists());
        assert!(!root.join(".claude/skills/cairn-dev/SKILL.md").exists());
        assert!(!root.join("AGENTS.md").exists());
        Ok(())
    }

    #[test]
    fn test_post_archive_failure_never_becomes_a_bootstrap_success()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-from-code-post-archive-failure")?;
        fs::create_dir_all(root.join("src/alpha"))?;
        fs::write(root.join("src/alpha/lib.rs"), "pub fn f() {}\n")?;
        fs::create_dir(root.join("map.json"))?;

        let first = run_in(&root, &["init", "--from-code", "--apply", "--wire"]);
        assert_ne!(first.code, 0, "the post-apply scan must fail");
        assert!(
            fs::read_dir(root.join("meta/changes/archive"))?
                .next()
                .is_some(),
            "the scenario must reach the post-rename failure window"
        );
        assert!(!root.join(BROWNFIELD_APPLIED_MARKER).exists());
        assert!(!root.join(".cairn/state/agent-pack.json").exists());
        assert!(!root.join("AGENTS.md").exists());

        let retry = run_in(&root, &["init", "--from-code", "--apply", "--wire"]);
        assert_ne!(
            retry.code, 0,
            "an incomplete archive must fail closed rather than bootstrap"
        );
        assert!(!root.join(".cairn/state/agent-pack.json").exists());
        assert!(!root.join("AGENTS.md").exists());
        Ok(())
    }

    #[test]
    fn test_active_brownfield_change_is_not_hidden_by_an_older_completion()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-from-code-active-after-archive")?;
        fs::create_dir_all(root.join("src/alpha"))?;
        fs::write(root.join("src/alpha/lib.rs"), "pub fn f() {}\n")?;
        assert_eq!(
            run_in(&root, &["init", "--from-code", "--apply", "--wire"]).code,
            0
        );
        assert_eq!(run_in(&root, &["init", "--from-code"]).code, 0);
        assert!(root.join("meta/changes/brownfield-init").exists());

        let retry = run_in(&root, &["init", "--from-code", "--apply", "--wire"]);
        assert_ne!(
            retry.code, 0,
            "a current active change must not be bypassed by an older marker"
        );
        assert!(root.join("meta/changes/brownfield-init").exists());
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_prefers_existing_claude_md() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-claude")?;
        fs::write(root.join("CLAUDE.md"), "# Existing project rules\n")?;
        let result = run_in(&root, &["init", "--wire"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        let claude = fs::read_to_string(root.join("CLAUDE.md"))?;
        assert!(
            claude.contains("cairn:agent-guide-begin"),
            "wire must target CLAUDE.md when it exists"
        );
        assert!(
            !root.join("AGENTS.md").exists(),
            "must not create AGENTS.md when CLAUDE.md exists"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_explicit_target() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-explicit")?;
        let result = run_in(&root, &["init", "--wire", ".cursor/rules/cairn.md"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        let file = fs::read_to_string(root.join(".cursor/rules/cairn.md"))?;
        assert!(
            file.contains("cairn:agent-guide-begin"),
            "explicit --wire <path> must write the orientation block to that path"
        );
        // Auto-detect targets must not have been created.
        assert!(
            !root.join("AGENTS.md").exists(),
            "explicit target must suppress auto-detect file creation"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_invalid_target_does_not_scaffold()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-invalid")?;
        let result = run_in(&root, &["init", "--wire", "/etc/CLAUDE.md"]);
        assert_eq!(result.code, 2);
        assert!(
            !root.join("cairn.blueprint").exists(),
            "invalid --wire target must not scaffold the project"
        );
        assert!(
            !root.join(".cairn").exists(),
            "invalid --wire target must not create .cairn/"
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn test_cli_init_wire_symlink_target_does_not_scaffold()
    -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::symlink;
        let root = temp_root("init-wire-symlink")?;
        // Create a symlinked CLAUDE.md pointing outside the project.
        let outside = tempfile::tempdir()?;
        std::fs::write(outside.path().join("CLAUDE.md"), "# outside\n")?;
        symlink(outside.path().join("CLAUDE.md"), root.join("CLAUDE.md"))?;

        let result = run_in(&root, &["init", "--wire"]);
        assert_eq!(result.code, 1, "must reject symlink target");
        assert!(
            !root.join("cairn.blueprint").exists(),
            "symlink --wire target must not scaffold the project"
        );
        assert!(
            !root.join(".cairn").exists(),
            "symlink --wire target must not create .cairn/"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_preserves_onboarding_steps() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-next-steps")?;
        let result = run_in(&root, &["init", "--wire"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        assert!(
            result.stdout.contains("Wired"),
            "init --wire must report the wiring result"
        );
        // Other onboarding steps must be preserved, not discarded.
        assert!(
            result.stdout.contains("cairn scan"),
            "init --wire must preserve the scan step in next-steps"
        );
        // The wire step must not appear (wire-aware variant is used).
        assert!(
            !result.stdout.contains("cairn init --wire"),
            "init --wire must not include the redundant wire step: {}",
            result.stdout
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_directory_target_does_not_scaffold()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-dir")?;
        let result = run_in(&root, &["init", "--wire", "."]);
        assert_eq!(result.code, 1, "must reject directory target");
        assert!(
            !root.join("cairn.blueprint").exists(),
            "directory --wire target must not scaffold the project"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_blueprint_collision_does_not_scaffold()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-blueprint")?;
        let result = run_in(&root, &["init", "--wire", "cairn.blueprint"]);
        assert_eq!(result.code, 2, "must reject scaffold file target");
        assert!(
            !root.join("cairn.blueprint").exists(),
            "scaffold-colliding --wire target must not scaffold the project"
        );
        assert!(
            !root.join(".cairn").exists(),
            "scaffold-colliding --wire target must not create .cairn/"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_cairn_dir_collision_does_not_scaffold()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-cairn-dir")?;
        let result = run_in(&root, &["init", "--wire", ".cairn/AGENTS.md"]);
        assert_eq!(result.code, 2, "must reject .cairn/ target");
        assert!(
            !root.join("cairn.blueprint").exists(),
            ".cairn/ --wire target must not scaffold the project"
        );
        assert!(
            !root.join(".cairn").exists(),
            ".cairn/ --wire target must not create .cairn/"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_wire_cairn_directory_collision_does_not_scaffold()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init-wire-cairn")?;
        let result = run_in(&root, &["init", "--wire", ".cairn"]);
        assert_eq!(result.code, 2, "must reject .cairn directory target");
        assert!(
            !root.join("cairn.blueprint").exists(),
            ".cairn --wire target must not scaffold the project"
        );
        assert!(
            !root.join(".cairn").exists(),
            ".cairn --wire target must not create .cairn/"
        );
        Ok(())
    }

    #[test]
    fn test_cli_init_from_code_apply_ignores_path_overrides()
    -> Result<(), Box<dyn std::error::Error>> {
        // init --from-code writes cairn.blueprint and meta/changes regardless
        // of --file/--changes-dir, so --apply must archive from those paths.
        let root = temp_root("init-from-code-apply-overrides")?;
        fs::create_dir_all(root.join("src/alpha"))?;
        for i in 0..3 {
            fs::write(root.join(format!("src/alpha/f{i}.rs")), "pub fn f() {}\n")?;
        }
        let result = run_in(
            &root,
            &[
                "--file",
                &root.join("alt.blueprint").to_string_lossy(),
                "--changes-dir",
                &root.join("custom-changes").to_string_lossy(),
                "init",
                "--from-code",
                "--apply",
            ],
        );
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        let blueprint = fs::read_to_string(root.join("cairn.blueprint"))?;
        assert!(
            blueprint.contains(r#"id "src.alpha""#),
            "blueprint must gain discovered nodes: {blueprint}"
        );
        assert!(!root.join("meta/changes/brownfield-init").exists());
        Ok(())
    }

    #[test]
    fn test_cli_feedback_command() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("feedback")?;

        let missing = run_in(&root, &["feedback"]);
        assert_eq!(missing.code, 2);
        assert!(missing.stderr.contains("usage: cairn feedback"));

        let first = run_in(&root, &["feedback", "scan said X,", "expected Y"]);
        assert_eq!(first.code, 0);
        assert!(first.stdout.contains(".cairn/feedback.md"));
        assert!(first.stdout.contains(
            "https://github.com/cairn-framework/cairn/issues/new?labels=feedback&title=scan%20said"
        ));

        let second = run_in(&root, &["--json", "feedback", "ui crashed"]);
        assert_eq!(second.code, 0);
        assert!(second.stdout.contains("\"command\":\"feedback\""));
        assert!(second.stdout.contains("\"issue_url\":"));
        assert!(!second.stdout.contains("\"area\":"));

        let structured = run_in(
            &root,
            &[
                "--json",
                "feedback",
                "--area",
                "scanner",
                "--severity",
                "high",
                "lint hid a finding",
            ],
        );
        assert_eq!(structured.code, 0);
        assert!(structured.stdout.contains("\"area\":\"scanner\""));
        assert!(structured.stdout.contains("\"severity\":\"high\""));
        // Flags stay out of the generated title and land in the body.
        assert!(structured.stdout.contains("title=lint%20hid%20a%20finding"));
        assert!(
            structured.stdout.contains(
                "body=lint%20hid%20a%20finding%0A%0Aarea%3A%20scanner%0Aseverity%3A%20high"
            )
        );

        let missing_value = run_in(&root, &["feedback", "broke", "--severity"]);
        assert_eq!(missing_value.code, 2);
        assert!(missing_value.stderr.contains("--severity requires a value"));

        let log = fs::read_to_string(root.join(".cairn/feedback.md"))?;
        assert!(log.starts_with("# Cairn feedback log"));
        assert!(log.contains("scan said X, expected Y"));
        assert!(log.contains("ui crashed"));
        assert!(log.contains("lint hid a finding\n\narea: scanner\nseverity: high"));

        Ok(())
    }

    #[test]
    fn test_cli_ui_command_surfaces_option_errors() {
        let result = run(&["ui".to_owned(), "--port".to_owned()]);
        assert_eq!(result.code, 2);
        assert!(result.stderr.contains("--port requires a value"));
    }

    /// Writes a project whose summariser is off, so nothing in these tests can
    /// reach a backend or mint a draft.
    fn write_project_summariser_disabled(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        write_project(root)?;
        fs::write(
            root.join("cairn.config.yaml"),
            "summariser:\n  mode: disabled\n",
        )?;
        Ok(())
    }

    #[test]
    fn test_cli_baseline_record_works_with_the_summariser_disabled()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-record")?;
        write_project_summariser_disabled(&root)?;

        let result = run_in(&root, &["baseline", "record", "app.api"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);

        let raw = fs::read_to_string(root.join(".cairn/state/contract-baselines.json"))?;
        let value: serde_json::Value = serde_json::from_str(&raw)?;
        assert_eq!(value["version"], 1);
        assert_eq!(value["nodes"]["app.api"]["kind"], "Container");
        assert_eq!(value["nodes"]["app.api"]["parent"], "app");
        assert_eq!(value["nodes"]["app.api"]["edges"][0], "app.core");
        assert!(
            value["nodes"]["app.api"].get("paths").is_none(),
            "reduced record must not carry paths: {raw}"
        );
        assert!(
            !root.join(".cairn/state/summariser").exists(),
            "recording must not generate a draft"
        );
        Ok(())
    }

    #[test]
    fn test_cli_baseline_record_json_payload() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-record-json")?;
        write_project_summariser_disabled(&root)?;

        let result = run_in(&root, &["--json", "baseline", "record", "app.api"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        let value: serde_json::Value = serde_json::from_str(&result.stdout)?;
        assert_eq!(value["node"], "app.api");
        assert_eq!(value["action"], "record");
        assert_eq!(value["baseline"]["kind"], "Container");
        assert_eq!(value["baseline"]["parent"], "app");
        Ok(())
    }

    #[test]
    fn test_cli_baseline_record_reports_an_unreadable_state_file()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-state-unreadable")?;
        write_project_summariser_disabled(&root)?;
        let state = root.join(".cairn/state/contract-baselines.json");
        fs::create_dir_all(state.parent().expect("state dir"))?;
        // A future schema version the current reader refuses, so the
        // read-modify-write cycle must abort before touching the file.
        let raw = r#"{"version":2,"nodes":{}}"#;
        fs::write(&state, raw)?;

        let result = run_in(&root, &["baseline", "record", "app.api"]);
        assert_eq!(result.code, 1);
        assert!(
            result
                .stderr
                .contains("failed to read .cairn/state/contract-baselines.json"),
            "stderr: {}",
            result.stderr
        );
        assert_eq!(fs::read_to_string(&state)?, raw);
        Ok(())
    }

    #[test]
    fn test_cli_baseline_drop_refuses_a_live_entry() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-drop-live")?;
        write_project_summariser_disabled(&root)?;
        assert_eq!(run_in(&root, &["baseline", "record", "app.api"]).code, 0);
        let before = fs::read(root.join(".cairn/state/contract-baselines.json"))?;

        let result = run_in(&root, &["baseline", "drop", "app.api"]);
        assert_eq!(result.code, 1);
        assert_eq!(
            result.stderr.trim(),
            copy::lookup("baseline.still-live").replace("{node}", "app.api")
        );
        assert_eq!(
            fs::read(root.join(".cairn/state/contract-baselines.json"))?,
            before,
            "a refused drop must not rewrite the file"
        );
        Ok(())
    }

    #[test]
    fn test_cli_baseline_drop_prunes_an_inert_entry() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-drop-inert")?;
        write_project_summariser_disabled(&root)?;
        assert_eq!(run_in(&root, &["baseline", "record", "app.api"]).code, 0);
        // Removing the contract makes the entry inert: no shape left to review.
        fs::remove_file(root.join("meta/contracts/api.md"))?;

        let result = run_in(&root, &["baseline", "drop", "app.api"]);
        assert_eq!(result.code, 0, "stderr: {}", result.stderr);
        let raw = fs::read_to_string(root.join(".cairn/state/contract-baselines.json"))?;
        let value: serde_json::Value = serde_json::from_str(&raw)?;
        assert!(value["nodes"].as_object().unwrap().is_empty(), "{raw}");
        Ok(())
    }

    #[test]
    fn test_cli_baseline_rejects_unknown_node_and_bad_usage()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-usage")?;
        write_project_summariser_disabled(&root)?;

        let unknown = run_in(&root, &["baseline", "record", "app.nope"]);
        assert_eq!(unknown.code, 1);
        assert_eq!(
            unknown.stderr.trim(),
            copy::lookup("baseline.node-not-declared").replace("{node}", "app.nope")
        );
        assert!(
            !root.join(".cairn/state/contract-baselines.json").exists(),
            "a rejected record must not create the state file"
        );

        for args in [
            vec!["baseline"],
            vec!["baseline", "record"],
            vec!["baseline", "sideways", "app.api"],
            vec!["baseline", "record", "app.api", "extra"],
        ] {
            let result = run_in(&root, &args);
            assert_eq!(result.code, 2, "{args:?}");
            assert_eq!(
                result.stderr.trim(),
                copy::lookup("baseline.usage"),
                "{args:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn test_cli_scan_reports_node_shape_drift_and_never_writes_the_baseline()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("baseline-drift")?;
        write_project_summariser_disabled(&root)?;
        assert_eq!(run_in(&root, &["baseline", "record", "app.api"]).code, 0);
        let recorded = fs::read(root.join(".cairn/state/contract-baselines.json"))?;

        // Drop the outbound edge the baseline was recorded against.
        let blueprint = fs::read_to_string(root.join("cairn.blueprint"))?;
        fs::write(
            root.join("cairn.blueprint"),
            blueprint.replace("app.api -> app.core \"reports\"\n", ""),
        )?;

        let scan = run_in(&root, &["scan"]);
        assert_eq!(scan.code, 0, "a Warning must not fail the plain gate");
        let lint: serde_json::Value =
            serde_json::from_str(&run_in(&root, &["lint", "--json"]).stdout)?;
        let drift = lint["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .find(|f| f["code"] == "CAIRN_CONTRACT_NODE_SHAPE_DRIFT")
            .expect("drift finding");
        assert_eq!(drift["severity"], "warning");
        assert_eq!(drift["node"], "app.api");
        // `target` is not part of the findings wire format; the changed fields
        // reach the reader through the resolved message.
        assert!(
            drift["message"]
                .as_str()
                .expect("message")
                .contains("`edges` changed"),
            "{drift}"
        );

        assert_eq!(
            run_in(&root, &["scan", "--strict"]).code,
            1,
            "a Warning must fail the strict gate"
        );
        let strict_json = run_in(&root, &["scan", "--strict", "--json"]);
        assert_eq!(
            strict_json.code, 1,
            "the JSON path must honour --strict on a Warning"
        );
        let strict_data: serde_json::Value = serde_json::from_str(&strict_json.stdout)?;
        assert_eq!(
            strict_data
                .get("strict_green")
                .and_then(serde_json::Value::as_bool),
            Some(false),
            "a Warning set must publish strict_green false on the wire"
        );
        assert_eq!(
            fs::read(root.join(".cairn/state/contract-baselines.json"))?,
            recorded,
            "scanning must never rewrite the baseline file"
        );
        Ok(())
    }

    /// `write_project` variant reconciling to zero findings: sources under
    /// test coverage, every leaf under a contract and a decision. Tests that
    /// assert on the strict gate need this explicit baseline so their own
    /// additions are provably the only standing finding set.
    fn write_project_strict_green(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        write_project(root)?;
        fs::write(
            root.join("src/api/lib.rs"),
            "pub fn serve() {}\n#[cfg(test)]\nmod tests {}\n",
        )?;
        fs::write(
            root.join("src/core/lib.rs"),
            "pub fn core() {}\n#[cfg(test)]\nmod tests {}\n",
        )?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Module Core "core" id "app.core" {
        path "./src/core"
        contract "./meta/contracts/core.md"
    }
    Container Api "api" id "app.api" {
        path "./src/api"
        contract "./meta/contracts/api.md"
        todos "./meta/todos"
        decisions "./meta/decisions"
        research "./meta/research"
        sources "./meta/sources"
    }
}
app.api -> app.core "reports"
"#,
        )?;
        fs::write(
            root.join("meta/contracts/core.md"),
            "---\nnode: app.core\n---\n# Core Contract\n",
        )?;
        fs::write(
            root.join("meta/decisions/api.md"),
            "---\nid: dec.api\nnodes: [app.api, app.core]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [res.api]\n---\n# Decision\n",
        )?;
        Ok(())
    }

    #[test]
    fn test_cli_lint_parks_the_unverified_pair_behind_a_blocked_todo()
    -> Result<(), Box<dyn std::error::Error>> {
        // `todo.lint-selection-folding` item 1a acceptance fixture: exactly
        // the two CAIRN_SOURCE_UNVERIFIED Info findings, parked by one
        // blocked todo whose `defers:` references both, end to end from
        // frontmatter to the lint wire under an absolute project root.
        let root = temp_root("parked-pair")?;
        write_project_strict_green(&root)?;
        let baseline: serde_json::Value =
            serde_json::from_str(&run_in(&root, &["lint", "--json"]).stdout)?;
        assert_eq!(
            baseline["findings"].as_array().map(Vec::len),
            Some(0),
            "the baseline fixture must stand at zero findings: {baseline}"
        );
        write_unverified_pair(&root)?;
        let parking = "---\nnode: app.api\nstatus: blocked\ncreated: 2026-07-29\ndefers:\n  - CAIRN_SOURCE_UNVERIFIED meta/sources/a.md\n  - CAIRN_SOURCE_UNVERIFIED meta/sources/b.md\n---\n# Park\n\nblocked on upstream mode work\n";
        fs::write(root.join("meta/todos/todo.park-sources.md"), parking)?;

        let lint: serde_json::Value =
            serde_json::from_str(&run_in(&root, &["lint", "--json"]).stdout)?;
        let findings = lint["findings"].as_array().expect("findings array");
        assert_eq!(
            findings.len(),
            2,
            "exactly the parked pair may stand: {lint}"
        );
        let unverified: Vec<_> = findings
            .iter()
            .filter(|f| f["code"] == "CAIRN_SOURCE_UNVERIFIED")
            .collect();
        assert_eq!(unverified.len(), 2, "reporting is untouched: {lint}");
        for finding in &unverified {
            assert_eq!(
                finding["parked_by"], "todo.park-sources",
                "the wire must name the parking todo: {finding}"
            );
            assert_eq!(finding["severity"], "info");
        }
        assert!(
            !findings.iter().any(|f| f["code"]
                .as_str()
                .is_some_and(|c| c.starts_with("CAIRN_TODO_DEFERS"))),
            "matching references raise nothing: {lint}"
        );
        assert_eq!(
            run_in(&root, &["scan", "--strict"]).code,
            0,
            "a parked Info set must keep the strict gate green"
        );
        let human = run_in(&root, &["lint"]).stdout;
        assert_eq!(
            human.matches("(parked by todo.park-sources)").count(),
            2,
            "each parked finding still prints, naming its todo: {human}"
        );

        // Unblocking the todo dissolves the park but keeps the references.
        let unblocked = parking.replace("status: blocked", "status: open");
        fs::write(root.join("meta/todos/todo.park-sources.md"), &unblocked)?;
        let lint: serde_json::Value =
            serde_json::from_str(&run_in(&root, &["lint", "--json"]).stdout)?;
        for finding in lint["findings"].as_array().expect("findings array") {
            if finding["code"] == "CAIRN_SOURCE_UNVERIFIED" {
                assert_eq!(
                    finding["parked_by"],
                    serde_json::Value::Null,
                    "an open todo parks nothing: {finding}"
                );
            }
        }

        Ok(())
    }

    /// Two unverified sources cited by research, so the pair stands alone
    /// with no orphan warnings: the parked-pair acceptance substrate.
    fn write_unverified_pair(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for name in ["a", "b"] {
            fs::write(
                root.join(format!("meta/sources/{name}.md")),
                format!(
                    "---\nid: src.{name}\nfile: docs-source.txt\nverification: unverified\ntype: note\ndate: 2026-03-19\n---\n# Source\n"
                ),
            )?;
        }
        fs::write(
            root.join("meta/research/api.md"),
            "---\nid: res.api\nnodes: [app.api]\ndate: 2026-03-20\nsources: [src.api, src.a, src.b]\n---\n# Research\n",
        )?;
        Ok(())
    }

    #[test]
    fn test_cli_lint_stale_defers_reference_turns_strict_red()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("parked-stale")?;
        write_project_strict_green(&root)?;
        write_unverified_pair(&root)?;
        // Reference `a` still matches; `gone` matches nothing.
        fs::write(
            root.join("meta/todos/todo.park-sources.md"),
            "---\nnode: app.api\nstatus: blocked\ncreated: 2026-07-29\ndefers:\n  - CAIRN_SOURCE_UNVERIFIED meta/sources/a.md\n  - CAIRN_SOURCE_UNVERIFIED meta/sources/gone.md\n---\n# Park\n",
        )?;
        let lint: serde_json::Value =
            serde_json::from_str(&run_in(&root, &["lint", "--json"]).stdout)?;
        let stale = lint["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .find(|f| f["code"] == "CAIRN_TODO_DEFERS_UNMATCHED")
            .expect("stale reference finding")
            .clone();
        assert_eq!(stale["severity"], "warning");
        // Parking is per-finding classification, not a whole-set verdict: the
        // still-matching reference keeps its finding parked while the stale
        // one turns strict red, so this state discriminates `parked_by` from
        // the strict-green fold (which is off here).
        assert_eq!(
            lint["strict_green"], false,
            "the stale-reference Warning must turn the published verdict red"
        );
        let still_parked = lint["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .find(|f| {
                f["code"] == "CAIRN_SOURCE_UNVERIFIED"
                    && f["path"].as_str().is_some_and(|p| p.ends_with("a.md"))
            })
            .expect("finding for the still-matching reference")
            .clone();
        assert_eq!(
            still_parked["parked_by"], "todo.park-sources",
            "a parked finding stays parked under a strict-red set: {still_parked}"
        );
        assert_eq!(
            run_in(&root, &["scan", "--strict"]).code,
            1,
            "a stale park must not hide behind the gate"
        );
        Ok(())
    }

    fn run_in(root: &Path, args: &[&str]) -> CliResult {
        let owned: Vec<String> = args.iter().map(|s| (*s).to_owned()).collect();
        run_in_str(root, &owned)
    }

    fn run_in_str(root: &Path, args: &[String]) -> CliResult {
        // Never mutate process-global CWD. Parallel cargo-test races on
        // set_current_dir even under a mutex when other tests or panic
        // paths leave the process in a foreign directory. Inject absolute
        // --file / --changes-dir so project_root resolves from the path.
        let mut owned = args.to_vec();
        if !owned.iter().any(|a| a == "--file") {
            owned.insert(0, "--file".to_owned());
            owned.insert(
                1,
                root.join("cairn.blueprint").to_string_lossy().into_owned(),
            );
        }
        if !owned.iter().any(|a| a == "--changes-dir") {
            owned.insert(0, "--changes-dir".to_owned());
            owned.insert(1, root.join("meta/changes").to_string_lossy().into_owned());
        }
        run(&owned)
    }

    fn write_project(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(root.join("src/api"))?;
        fs::create_dir_all(root.join("src/core"))?;
        fs::create_dir_all(root.join("meta/contracts"))?;
        fs::create_dir_all(root.join("meta/todos"))?;
        fs::create_dir_all(root.join("meta/decisions"))?;
        fs::create_dir_all(root.join("meta/research"))?;
        fs::create_dir_all(root.join("meta/sources"))?;
        fs::create_dir_all(root.join("meta/changes"))?;
        fs::create_dir_all(root.join(".cairn"))?;
        fs::write(root.join("src/api/lib.rs"), "pub fn serve() {}\n")?;
        fs::write(root.join("src/core/lib.rs"), "pub fn core() {}\n")?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Module Core "core" id "app.core" {
        path "./src/core"
    }
    Container Api "api" id "app.api" {
        path "./src/api"
        contract "./meta/contracts/api.md"
        todos "./meta/todos"
        decisions "./meta/decisions"
        research "./meta/research"
        sources "./meta/sources"
    }
}
app.api -> app.core "reports"
"#,
        )?;
        fs::write(
            root.join("cairn.config.yaml"),
            "ignore:\n  - target\ncontext: \"ctx\"\nrules: {}\n",
        )?;
        fs::write(
            root.join("meta/contracts/api.md"),
            "---\nnode: app.api\n---\n# API Contract\n",
        )?;
        fs::write(
            root.join("meta/todos/todo.api.md"),
            "---\nnode: app.api\nstatus: open\ncreated: 2026-04-01\n---\n# Todo\n",
        )?;
        fs::write(
            root.join("meta/decisions/api.md"),
            "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [res.api]\n---\n# Decision\n",
        )?;
        fs::write(
            root.join("meta/research/api.md"),
            "---\nid: res.api\nnodes: [app.api]\ndate: 2026-03-20\nsources: [src.api]\n---\n# Research\n",
        )?;
        fs::write(root.join("docs-source.txt"), "source\n")?;
        fs::write(
            root.join("meta/sources/api.md"),
            "---\nid: src.api\nfile: docs-source.txt\nsha256: b8bb034f9b63bd0254fbc7c157cae746c75853f4643d6cea844dc48ddb57f522\nverification: verified\ntype: note\ndate: 2026-03-19\n---\n# Source\n",
        )?;
        fs::write(root.join(".cairn/log.md"), "- first log\n")?;
        Ok(())
    }

    fn write_change(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let change = root
            .join("meta/changes")
            .join("phase-7.5a-test-fortification");
        fs::create_dir_all(&change)?;
        fs::write(
            change.join("proposal.md"),
            "# Proposal: Phase 7.5a Test Fortification\n",
        )?;
        fs::write(change.join("blueprint.delta"), "")?;
        Ok(())
    }

    fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("cairn-cli-tests-{name}-{suffix}"));
        fs::create_dir_all(&root)?;
        Ok(root)
    }

    #[test]
    fn test_help_flag_returns_code_zero_with_command_names() {
        for flag in &["--help", "-h"] {
            let result = run(&[flag.to_string()]);
            assert_eq!(result.code, 0, "{flag} should exit 0");
            assert!(result.stderr.is_empty(), "{flag} should have no stderr");
            assert!(
                result.stdout.contains("cairn"),
                "{flag} should show program name"
            );
            for cmd in &["scan", "get", "lint", "init", "context"] {
                assert!(
                    result.stdout.contains(cmd),
                    "{flag} output should list '{cmd}'"
                );
            }
        }
    }

    #[test]
    fn test_help_flag_with_other_args() {
        let result = run(&["scan".to_owned(), "--help".to_owned()]);
        assert_eq!(result.code, 0, "--help with command should still show help");
        // Per-command help: command-specific usage, not the global catalogue.
        assert!(
            result.stdout.contains("Usage:") && result.stdout.contains("scan"),
            "expected per-command scan help, got:
{}",
            result.stdout
        );
        assert!(
            !result.stdout.contains(
                "Commands:
  backlog"
            ),
            "scan --help must not fall back to the global command list"
        );
    }

    #[test]
    fn test_no_args_shows_help() {
        let result = run(&[]);
        assert_eq!(result.code, 0, "no args should show help");
        assert!(result.stdout.contains("Usage:"));
    }

    #[test]
    fn test_help_documents_exit_codes() {
        let result = run(&["--help".to_string()]);
        assert!(
            result.stdout.contains("Exit codes"),
            "--help must document exit codes; got:\n{}",
            result.stdout
        );
        assert!(
            result.stdout.contains('0'),
            "--help must document exit code 0"
        );
        assert!(
            result.stdout.contains('1'),
            "--help must document exit code 1"
        );
    }

    #[test]
    fn test_help_documents_strict_flag() {
        let result = run(&["--help".to_string()]);
        assert!(
            result.stdout.contains("--strict"),
            "--help must document --strict flag"
        );
    }

    #[test]
    fn test_unknown_command_suggests_close_match() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("suggest-close")?;
        write_project(&root)?;
        let result = run_in(&root, &["scn"]);
        assert_eq!(result.code, 2);
        assert!(
            result.stderr.contains("Did you mean 'scan'?"),
            "should suggest 'scan' for 'scn', got: {}",
            result.stderr
        );
        Ok(())
    }

    #[test]
    fn test_unknown_command_lists_available_when_distant() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = temp_root("suggest-distant")?;
        write_project(&root)?;
        let result = run_in(&root, &["zzzznotacommand"]);
        assert_eq!(result.code, 2);
        assert!(
            result.stderr.contains("Available commands:"),
            "should list available commands for distant input, got: {}",
            result.stderr
        );
        assert!(result.stderr.contains("scan"));
        Ok(())
    }

    #[test]
    fn test_unknown_command_preserves_existing_behaviour() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = temp_root("suggest-preserve")?;
        write_project(&root)?;
        let result = run_in(&root, &["unknown"]);
        assert_eq!(result.code, 2);
        assert!(result.stderr.contains("unknown command"));
        Ok(())
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("scan", "scan"), 0);
        assert_eq!(edit_distance("scn", "scan"), 1);
        assert_eq!(edit_distance("sca", "scan"), 1);
        assert_eq!(edit_distance("scam", "scan"), 1);
        assert_eq!(edit_distance("lint", "init"), 2);
        assert_eq!(edit_distance("abc", "xyz"), 3);
    }

    #[test]
    fn test_check_json_output_is_valid_json() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("check-json")?;
        write_project(&root)?;
        let result = run_in(&root, &["--json", "lint", "--node", "app.api"]);
        assert_eq!(result.code, 0, "check json stderr: {}", result.stderr);
        let parsed: serde_json::Value = serde_json::from_str(result.stdout.trim())
            .unwrap_or_else(|e| panic!("invalid JSON from check --json: {e}\n{}", result.stdout));
        assert_eq!(parsed["command"], "lint");
        assert_eq!(parsed["status"], "ok");
        assert!(parsed["data"]["findings"].is_array());
        Ok(())
    }

    #[test]
    fn test_check_json_with_target_node() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("check-json-node")?;
        write_project(&root)?;
        let result = run_in(&root, &["--json", "lint", "--node", "app.api"]);
        assert_eq!(result.code, 0, "check json stderr: {}", result.stderr);
        let parsed: serde_json::Value =
            serde_json::from_str(result.stdout.trim()).unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from check --json app.api: {e}\n{}",
                    result.stdout
                )
            });
        assert_eq!(parsed["command"], "lint");
        Ok(())
    }

    #[test]
    fn test_onboard_json_output_is_valid_json() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("onboard-json")?;
        write_project(&root)?;
        let result = run_in(&root, &["--json", "onboard"]);
        assert_eq!(result.code, 0, "onboard json stderr: {}", result.stderr);
        let parsed: serde_json::Value = serde_json::from_str(result.stdout.trim())
            .unwrap_or_else(|e| panic!("invalid JSON from onboard --json: {e}\n{}", result.stdout));
        assert_eq!(parsed["command"], "onboard");
        assert_eq!(parsed["status"], "ok");
        assert!(parsed["data"].is_object());
        Ok(())
    }

    #[test]
    fn test_check_output_for_empty_states() -> Result<(), Box<dyn std::error::Error>> {
        let no_bp_root = temp_root("check-no-blueprint")?;
        let no_bp = run_in(&no_bp_root, &["lint", "--node", "app.api"]);
        assert_eq!(no_bp.code, 0);
        insta::assert_snapshot!("lint_node_no_blueprint", no_bp.stdout);

        let clean_root = temp_root("check-clean-map")?;
        fs::create_dir_all(clean_root.join("src"))?;
        fs::write(
            clean_root.join("src/lib.rs"),
            "pub fn main() {}\n#[cfg(test)]\nmod tests {}\n",
        )?;
        fs::write(
            clean_root.join("cairn.blueprint"),
            "System Clean \"clean project\" id \"clean\" {\n    Module Only \"only module\" id \"clean.only\" {\n        path \"./src\"\n        contract \"./contracts/clean.only.md\"\n    }\n}\n",
        )?;
        fs::create_dir_all(clean_root.join("contracts"))?;
        fs::write(
            clean_root.join("contracts/clean.only.md"),
            "---\nnode: clean.only\n---\n\n# Only\n",
        )?;
        fs::write(
            clean_root.join("cairn.config.yaml"),
            "context: \"ctx\"\nrules: {}\n",
        )?;
        let clean_result = run_in(&clean_root, &["lint", "--node", "clean.only"]);
        assert_eq!(clean_result.code, 0);
        insta::assert_snapshot!("lint_node_clean_map", clean_result.stdout);

        Ok(())
    }

    #[test]
    fn test_cli_draft_list_and_show_json() -> Result<(), Box<dyn std::error::Error>> {
        use crate::summariser::{Draft, DraftHeader, DraftStore, PendingDraft};

        let root = temp_root("draft-commands")?;
        write_project(&root)?;

        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store.write(&Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-001".to_owned(),
                node_id: "app.api".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "---\nnode: app.api\n---\n# Draft".to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        }))?;

        let drafts = run_in(&root, &["--json", "draft", "list"]);
        assert_eq!(drafts.code, 0, "draft list json stderr: {}", drafts.stderr);
        let parsed: serde_json::Value =
            serde_json::from_str(drafts.stdout.trim()).unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from draft list --json: {e}\n{}",
                    drafts.stdout
                )
            });
        let draft_array = parsed
            .get("drafts")
            .and_then(|v| v.as_array())
            .expect("drafts array");
        assert_eq!(draft_array.len(), 1);
        assert_eq!(draft_array[0]["id"], "draft-001");

        let show = run_in(&root, &["--json", "draft", "show", "draft-001"]);
        assert_eq!(show.code, 0, "draft show json stderr: {}", show.stderr);
        let parsed: serde_json::Value =
            serde_json::from_str(show.stdout.trim()).unwrap_or_else(|e| {
                panic!("invalid JSON from draft show --json: {e}\n{}", show.stdout)
            });
        assert_eq!(parsed["id"], "draft-001");
        assert_eq!(parsed["status"], "pending");

        Ok(())
    }

    #[test]
    #[allow(clippy::too_many_lines)] // Reason: test covers four related commands in one logical flow
    fn test_cli_draft_mutating_commands_json() -> Result<(), Box<dyn std::error::Error>> {
        use crate::summariser::{Draft, DraftHeader, DraftStore, PendingDraft};

        let root = temp_root("draft-mutating-commands")?;
        write_project(&root)?;

        let store = DraftStore::new(root.join(".cairn/state/summariser"));

        // draft discard
        store.write(&Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-001".to_owned(),
                node_id: "app.api".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "---\nnode: app.api\n---\n# Draft".to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        }))?;
        let discard = run_in(&root, &["--json", "draft", "discard", "draft-001"]);
        assert_eq!(
            discard.code, 0,
            "draft discard json stderr: {}",
            discard.stderr
        );
        let parsed: serde_json::Value =
            serde_json::from_str(discard.stdout.trim()).unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from draft discard --json: {e}\n{}",
                    discard.stdout
                )
            });
        assert_eq!(parsed["id"], "draft-001");
        assert_eq!(parsed["status"], "discarded");

        // draft edit
        store.write(&Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-002".to_owned(),
                node_id: "app.api".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "---\nnode: app.api\n---\n# Draft".to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        }))?;
        let edit = run_in(&root, &["--json", "draft", "edit", "draft-002"]);
        assert_eq!(edit.code, 0, "draft edit json stderr: {}", edit.stderr);
        let parsed: serde_json::Value =
            serde_json::from_str(edit.stdout.trim()).unwrap_or_else(|e| {
                panic!("invalid JSON from draft edit --json: {e}\n{}", edit.stdout)
            });
        assert_eq!(parsed["id"], "draft-002");
        assert_eq!(parsed["status"], "editable");

        // draft accept
        store.write(&Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-003".to_owned(),
                node_id: "app.api".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "---\nnode: app.api\n---\n# Accepted Draft".to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        }))?;
        let accept = run_in(&root, &["--json", "draft", "accept", "draft-003"]);
        assert_eq!(
            accept.code, 0,
            "draft accept json stderr: {}",
            accept.stderr
        );
        let parsed: serde_json::Value =
            serde_json::from_str(accept.stdout.trim()).unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from draft accept --json: {e}\n{}",
                    accept.stdout
                )
            });
        assert_eq!(parsed["id"], "draft-003");
        assert_eq!(parsed["status"], "accepted");

        // draft accept --edited
        store.write(&Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-004".to_owned(),
                node_id: "app.api".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "---\nnode: app.api\n---\n# Generated".to_owned(),
                created_at: "2024-01-15T10:30:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        }))?;
        std::fs::create_dir_all(root.join(".cairn/state/summariser/editable"))?;
        std::fs::write(
            store.editable_path("draft-004", "contract"),
            "---\nnode: app.api\n---\n# Edited Draft",
        )?;
        let accept_edited = run_in(
            &root,
            &["--json", "draft", "accept", "draft-004", "--edited"],
        );
        assert_eq!(
            accept_edited.code, 0,
            "draft accept --edited json stderr: {}",
            accept_edited.stderr
        );
        let parsed: serde_json::Value = serde_json::from_str(accept_edited.stdout.trim())
            .unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from draft accept --edited: {e}\n{}",
                    accept_edited.stdout
                )
            });
        assert_eq!(parsed["id"], "draft-004");
        assert_eq!(parsed["status"], "accepted");

        Ok(())
    }

    #[test]
    fn test_cli_old_draft_spellings_rejected() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("old-draft-spellings")?;
        write_project(&root)?;

        for old in [
            "drafts",
            "draft_show",
            "draft_edit",
            "draft_discard",
            "draft_accept",
            "summarise",
        ] {
            let result = run_in(&root, &["--json", old]);
            assert_eq!(
                result.code, 2,
                "old spelling `{old}` should fail with exit code 2"
            );
            assert!(
                result.stderr.contains("unknown command"),
                "old spelling `{old}` should produce unknown command error: {}",
                result.stderr
            );
        }

        Ok(())
    }

    #[test]
    fn test_cli_draft_create_disabled_by_default() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("draft-create-disabled")?;
        write_project(&root)?;

        let result = run_in(&root, &["--json", "draft", "create", "app.api"]);
        assert_eq!(
            result.code, 1,
            "draft create json stderr: {}",
            result.stderr
        );
        let parsed: serde_json::Value =
            serde_json::from_str(result.stdout.trim()).unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from draft create --json: {e}\n{}",
                    result.stdout
                )
            });
        assert_eq!(parsed["error"]["code"], "CAIRN_SUMMARISER_DISABLED");

        Ok(())
    }
    #[test]
    fn test_cli_import_openspec_migrates_phases() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("import-openspec")?;
        write_project(&root)?;

        // Set up legacy openspec changes with two phases and archive.
        let changes_dir = root.join("meta/changes");
        fs::create_dir_all(changes_dir.join("phase-7-test"))?;
        fs::write(
            changes_dir.join("phase-7-test/proposal.md"),
            "# Proposal: Phase 7 Test\n",
        )?;
        fs::write(
            changes_dir.join("phase-7-test/design.md"),
            "# Design: Phase 7 Test\n",
        )?;
        fs::write(
            changes_dir.join("phase-7-test/tasks.md"),
            "- [ ] Task one\n",
        )?;
        fs::create_dir_all(changes_dir.join("phase-7-test/specs"))?;
        fs::write(changes_dir.join("phase-7-test/specs/spec.md"), "# Spec\n")?;
        fs::create_dir_all(changes_dir.join("phase-8-test"))?;
        fs::write(
            changes_dir.join("phase-8-test/proposal.md"),
            "# Proposal: Phase 8 Test\n",
        )?;
        fs::create_dir_all(changes_dir.join("archive/old-phase"))?;
        fs::write(
            changes_dir.join("archive/old-phase/proposal.md"),
            "# Proposal: Old Phase\n",
        )?;

        let result = run_in(&root, &["--json", "import-openspec"]);
        assert_eq!(result.code, 0, "import-openspec stderr: {}", result.stderr);

        // Verify phases migrated.
        assert!(root.join("meta/changes/phase-7-test/proposal.md").exists());
        assert!(root.join("meta/changes/phase-7-test/design.md").exists());
        assert!(root.join("meta/changes/phase-7-test/tasks.md").exists());
        assert!(
            root.join("meta/changes/phase-7-test/specs/spec.md")
                .exists()
        );
        assert!(root.join("meta/changes/phase-8-test/proposal.md").exists());

        // Verify archive copied.
        assert!(
            root.join("meta/changes/archive/old-phase/proposal.md")
                .exists()
        );

        Ok(())
    }

    #[test]
    fn test_contract_and_order_json_served_by_query_api() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = temp_root("contract-order-json")?;
        write_project(&root)?;

        // `contract --json` is served by query_api, which emits a `contracts`
        // field. The removed CLI-local branch omitted it, so its presence
        // proves the shared path is the single JSON source of truth.
        let contract = run_in(&root, &["--json", "contract", "app.api"]);
        assert_eq!(
            contract.code, 0,
            "contract json stderr: {}",
            contract.stderr
        );
        let parsed: serde_json::Value = serde_json::from_str(contract.stdout.trim())
            .unwrap_or_else(|e| {
                panic!(
                    "invalid JSON from contract --json: {e}\n{}",
                    contract.stdout
                )
            });
        assert_eq!(parsed["node"], "app.api");
        assert!(
            parsed.get("contracts").is_some(),
            "contract --json must carry query_api's `contracts` field, got: {}",
            contract.stdout
        );

        // `order --json` is served by query_api: a `nodes` array.
        let order = run_in(&root, &["--json", "order"]);
        assert_eq!(order.code, 0, "order json stderr: {}", order.stderr);
        let parsed: serde_json::Value = serde_json::from_str(order.stdout.trim())
            .unwrap_or_else(|e| panic!("invalid JSON from order --json: {e}\n{}", order.stdout));
        assert!(
            parsed["nodes"].is_array(),
            "order --json must expose a `nodes` array, got: {}",
            order.stdout
        );

        // The human paths still render their plain-text headers.
        let contract_human = run_in(&root, &["contract", "app.api"]);
        assert_eq!(contract_human.code, 0);
        assert!(
            contract_human.stdout.starts_with("Contract for app.api"),
            "human contract output: {}",
            contract_human.stdout
        );
        let order_human = run_in(&root, &["order"]);
        assert_eq!(order_human.code, 0);
        assert!(
            order_human.stdout.starts_with("Order:"),
            "human order output: {}",
            order_human.stdout
        );

        Ok(())
    }

    #[test]
    fn test_change_show_reports_task_progress() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("change-progress")?;
        write_project(&root)?;
        let change_dir = root.join("meta/changes/demo");
        fs::create_dir_all(&change_dir)?;
        fs::write(change_dir.join("proposal.md"), "# Proposal: Demo\n")?;
        fs::write(
            change_dir.join("tasks.md"),
            "- [x] one\n- [x] two\n- [ ] three\n",
        )?;

        let json = run_in(&root, &["--json", "change", "show", "demo"]);
        assert_eq!(json.code, 0, "change show stderr: {}", json.stderr);
        let parsed: serde_json::Value = serde_json::from_str(json.stdout.trim())
            .unwrap_or_else(|e| panic!("invalid JSON from change show: {e}\n{}", json.stdout));
        assert_eq!(parsed["progress"]["completed"], 2);
        assert_eq!(parsed["progress"]["total"], 3);
        assert_eq!(parsed["progress"]["remaining"], 1);

        let human = run_in(&root, &["change", "show", "demo"]);
        assert_eq!(human.code, 0, "change show stderr: {}", human.stderr);
        assert!(
            human.stdout.contains("Tasks: 2/3 complete"),
            "human change show must report task progress, got: {}",
            human.stdout
        );

        Ok(())
    }

    #[test]
    fn test_change_accept_dry_run_previews_without_consuming_the_change_id()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("change-accept-dry-run")?;
        write_project(&root)?;
        fs::write(
            root.join("cairn.config.yaml"),
            "gates:\n  - name: touch sentinel\n    command: touch sentinel.txt\n",
        )?;

        let result = run_in(&root, &["--json", "change", "accept", "--dry-run", "demo"]);
        assert_eq!(result.code, 0, "accept stderr: {}", result.stderr);
        assert!(
            !root.join("sentinel.txt").exists(),
            "dry run must not run gate commands"
        );
        // The regression: `--dry-run` used to be parsed as the change id, so the
        // lint step named the flag instead of `demo`.
        assert!(
            result
                .stdout
                .contains("\"test\":\"cairn lint --strict demo\",\"state\":\"planned\""),
            "dry run must keep `demo` as the change id, got: {}",
            result.stdout
        );
        assert!(
            result.stdout.contains("\"gate_outcome\":\"preview\""),
            "dry run reports a preview outcome, got: {}",
            result.stdout
        );

        Ok(())
    }
}
