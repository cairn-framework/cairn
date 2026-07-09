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
        Decision, DecisionStatus, Research, Review, ReviewType, Source, Todo, TodoStatus,
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
mod render;

// Re-exported so existing `super::copy::`/`crate::cli::copy::` call sites
// across cli submodules resolve unchanged; the module itself lives at
// `crate::copy` (shared with query_api, which also needs copy lookups).
pub(crate) use crate::copy;

use commands::{
    init_project, legacy_blueprint_warning, run_archive_command, run_change_new,
    run_decision_command, run_draft_command, run_feedback_command, run_gap_command,
    run_hook_command, run_import_openspec, run_onboard_command, run_shared_json_command,
    run_todo_command, run_ui_command, run_watch_command, run_workspace_command,
};
use format::{
    err, error_output, esc, finding_json, finding_output, findings_output, flag_value, lines,
    node_arg, ok, render_findings,
};
use render::{
    render_backlog, render_brief, render_bundle, render_changes, render_context, render_decisions,
    render_dependencies, render_files, render_get, render_health, render_neighbourhood,
    render_next, render_rationale, render_remediate, render_research, render_show, render_sources,
    render_status, render_todos,
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

/// Executes CLI arguments.
#[must_use]
// Reason: CLI dispatch hub for many subcommands; natural seam is per-command
// modules which already exist for newer commands.
#[allow(clippy::too_many_lines)]
pub fn run(args: &[String]) -> CliResult {
    if args == ["--version"] {
        return ok(format!("{}\n", version_label()));
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        return ok(help_text());
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
        if from_code {
            let force = parsed.command_args.iter().any(|a| a == "--force");
            return match crate::brownfield::init::run_init_from_code(project_root, force) {
                Ok(change_id) => ok(format!(
                    "brownfield init complete; change written to meta/changes/{change_id}/\n"
                )),
                Err(e) => err(1, &e.to_string()),
            };
        }
        return init_project(project_root);
    }
    if parsed.command == "import-openspec" {
        return run_import_openspec(project_root, parsed.json);
    }
    if parsed.command == "refine" {
        return match crate::brownfield::refine::run_refine(project_root) {
            Ok(change_id) => ok(format!(
                "refine complete; change written to meta/changes/{change_id}/\n"
            )),
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
    let change_id = parsed.command_args.get(2).map(String::as_str);
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
                    tool: "changes".to_owned(),
                    node: None,
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
                    tool: "show".to_owned(),
                    node: None,
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
            let show_parsed = ParsedArgs {
                json: parsed.json,
                strict: parsed.strict,
                file: parsed.file.clone(),
                changes_dir: parsed.changes_dir.clone(),
                command: "show".to_owned(),
                command_args: vec!["show".to_owned(), id.to_owned()],
            };
            match render_show(&show_parsed, project_root) {
                Ok(stdout) => ok(stdout),
                Err(finding) => error_output(parsed.json, &finding.code, &finding.message),
            }
        }
        Some("accept") => crate::cli::accept::run_accept_gate(change_id, parsed.json),
        Some("archive") => {
            let Some(id) = change_id else {
                return err(2, "usage: cairn change archive <change-id>");
            };
            let root = project_root;
            let legacy_warning = legacy_blueprint_warning(root);
            // archive reads change id from command_args[1]; synthesise
            // the old top-level `archive <id>` shape.
            let archive_parsed = ParsedArgs {
                json: parsed.json,
                strict: parsed.strict,
                file: parsed.file.clone(),
                changes_dir: parsed.changes_dir.clone(),
                command: "archive".to_owned(),
                command_args: vec!["archive".to_owned(), id.to_owned()],
            };
            run_archive_command(&archive_parsed, root, legacy_warning)
        }
        _ => err(
            2,
            "usage: cairn change <new|list|show|accept|archive> [args]",
        ),
    }
}
struct ParsedArgs {
    json: bool,
    strict: bool,
    file: PathBuf,
    changes_dir: PathBuf,
    command: String,
    command_args: Vec<String>,
}

fn parse_args(args: &[String]) -> Result<ParsedArgs, CliResult> {
    let mut json = false;

    let mut strict = false;
    let mut file = PathBuf::from("cairn.blueprint");
    let mut changes_dir = PathBuf::from("meta/changes");
    let mut command_args = Vec::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--json" => json = true,
            "--strict" => strict = true,
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
        return findings_output(parsed.json, &scan_result.graph.findings);
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
        "files" => render_files(parsed, scan_result),
        "bundle" => render_bundle(parsed, scan_result),
        "gap" => return run_gap_command(parsed, root, scan_result),
        "todos" => render_todos(parsed, root),
        "decisions" => render_decisions(parsed, scan_result),
        "research" => render_research(parsed, root),
        "sources" => render_sources(parsed, root),
        "rationale" => render_rationale(parsed, scan_result),
        "status" => Ok(render_status(parsed, scan_result, root)),
        "context" => Ok(render_context(parsed, root, scan_result)),
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
            Err(findings) => return findings_output(parsed.json, &findings),
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
            Err(findings) => return findings_output(parsed.json, &findings),
        },
        "deps" => render_dependencies(parsed, root),
        // Spine ops (webui-first): the human rendering is the pretty canonical
        // JSON; the primary consumers are the webui and --json callers.
        "ui_meta" | "blueprint" | "beads" => {
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
                    render_findings(&findings, false)
                };
                return CliResult {
                    code: u8::from(has_errors),
                    stdout,
                    stderr: legacy_warning,
                };
            }
            let has_error = response
                .findings
                .iter()
                .any(|finding| finding.severity == FindingSeverity::Error);
            let has_warning = response
                .findings
                .iter()
                .any(|finding| finding.severity == FindingSeverity::Warning);
            let code = if parsed.strict {
                u8::from(has_error || has_warning)
            } else {
                u8::from(has_error)
            };
            let stdout = render_findings(&response.findings, parsed.json);
            return CliResult {
                code,
                stdout,
                stderr: legacy_warning,
            };
        }
        other => return unknown_command_error(other),
    }
    .map_or_else(
        |finding| finding_output(parsed.json, finding),
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
        name: "brief",
        description: "Fused next-unit brief: task, decisions, contract, gates",
    },
    CliOnlyCommand {
        name: "change",
        description: "Manage changes: new, list, show, accept, archive",
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
        name: "next",
        description: "Show the next ready unit of work",
    },
    CliOnlyCommand {
        name: "onboard",
        description: "Suggest blueprint entries for orphaned files",
    },
    CliOnlyCommand {
        name: "todo",
        description: "Scaffold a new todo artefact",
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
const RETIRED_TOP_LEVEL: &[&str] = &["accept", "archive", "changes", "show"];

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
fn command_description(name: &str) -> &'static str {
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
            | "bundle"
            | "deps"
            | "order"
            | "islands"
            | "frontier"
            | "lint"
            | "scan"
            | "status"
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
            ("lint", vec!["lint"]),
            ("scan", vec!["scan"]),
            ("hook", vec!["hook", "all"]),
            ("ui_meta", vec!["ui_meta"]),
            ("blueprint", vec!["blueprint"]),
            ("beads", vec!["beads", "app.api"]),
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
    fn test_cli_init_and_version_commands() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init")?;
        let init = run_in(&root, &["init"]);
        assert_eq!(init.code, 0);
        assert!(init.stdout.contains("Next steps"));
        assert!(root.join("cairn.blueprint").exists());
        assert!(root.join("cairn.config.yaml").exists());
        let guide = fs::read_to_string(root.join(".cairn/AGENTS.md"))?;
        assert!(guide.contains("cairn feedback"));

        let version = run(&["--version".to_owned()]);
        assert_eq!(version.code, 0);
        assert!(version.stdout.contains("cairn "));

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

        let log = fs::read_to_string(root.join(".cairn/feedback.md"))?;
        assert!(log.starts_with("# Cairn feedback log"));
        assert!(log.contains("scan said X, expected Y"));
        assert!(log.contains("ui crashed"));

        Ok(())
    }

    #[test]
    fn test_cli_ui_command_surfaces_option_errors() {
        let result = run(&["ui".to_owned(), "--port".to_owned()]);
        assert_eq!(result.code, 2);
        assert!(result.stderr.contains("--port requires a value"));
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
            root.join("meta/decisions/dec.api.md"),
            "---\nid: dec.api\nnodes: [app.api]\nstatus: accepted\ndate: 2026-04-01\ninformed_by: [res.api]\n---\n# Decision\n",
        )?;
        fs::write(
            root.join("meta/research/res.api.md"),
            "---\nid: res.api\nnodes: [app.api]\ndate: 2026-03-20\nsources: [src.api]\n---\n# Research\n",
        )?;
        fs::write(root.join("docs-source.txt"), "source\n")?;
        fs::write(
            root.join("meta/sources/src.api.md"),
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
        assert!(result.stdout.contains("Commands:"));
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
}
