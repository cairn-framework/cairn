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
        Decision, DecisionStatus, Research, Review, ReviewType, Source, SourceVerification, Todo,
        TodoStatus,
    },
    hooks::{self, HookKind},
    map::{
        graph::{Finding, FindingSeverity, NodeRecord},
        query,
    },
    scanner, ui, version_label,
};

pub use crate::query_api::SafetyClass;

/// Command metadata.
mod accept;
mod commands;
pub mod export;
mod format;
mod render;

use commands::{
    init_project, legacy_blueprint_warning, requires_valid_map, run_archive_command,
    run_hook_command, run_onboard_command, run_shared_json_command, run_ui_command,
};
use format::{
    err, error_output, esc, finding_output, findings_output, lines, node_arg, ok, render_findings,
    string_array_json,
};
use render::{
    render_context, render_decisions, render_dependencies, render_files, render_get,
    render_neighbourhood, render_rationale, render_research, render_sources, render_status,
    render_todos,
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
    if parsed.command == "init" {
        let from_code = parsed.command_args.iter().any(|a| a == "--from-code");
        if from_code {
            let force = parsed.command_args.iter().any(|a| a == "--force");
            return match crate::brownfield::init::run_init_from_code(Path::new("."), force) {
                Ok(change_id) => ok(format!(
                    "brownfield init complete; change written to openspec/changes/{change_id}/\n"
                )),
                Err(e) => err(1, &e.to_string()),
            };
        }
        return init_project(Path::new("."));
    }
    if parsed.command == "refine" {
        return match crate::brownfield::refine::run_refine(Path::new(".")) {
            Ok(change_id) => ok(format!(
                "refine complete; change written to openspec/changes/{change_id}/\n"
            )),
            Err(e) => err(1, &e.to_string()),
        };
    }
    if parsed.command == "ui" {
        return run_ui_command(&parsed);
    }
    if parsed.command == "accept" {
        let change_id = parsed.command_args.get(1).map(String::as_str);
        return crate::cli::accept::run_accept_gate(change_id);
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
    if parsed.command == "check" {
        if parsed.json {
            return err(
                1,
                "--json: unknown flag for `cairn check`; use `cairn lint --json` for JSON output",
            );
        }
        if !parsed.file.exists() {
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
                return error_output(
                    parsed.json,
                    "CAIRN_COMMAND_FAILED",
                    "no blueprint file was found; rename `cairn.dsl` to `cairn.blueprint`",
                );
            }
            return ok(
                "No cairn.blueprint found. Inspection has nothing to look at.\n\
                 Run `cairn init` to scaffold a blueprint, then re-run `cairn check`.\n"
                    .to_owned(),
            );
        }
    }
    run_project_command(&parsed)
}

struct ParsedArgs {
    json: bool,
    file: PathBuf,
    changes_dir: PathBuf,
    command: String,
    command_args: Vec<String>,
}

fn parse_args(args: &[String]) -> Result<ParsedArgs, CliResult> {
    let mut json = false;
    let mut file = PathBuf::from("cairn.blueprint");
    let mut changes_dir = PathBuf::from("meta/changes");
    let mut command_args = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--json" => json = true,
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
        return Err(err(2, "usage: cairn <command> [--file path] [--json]"));
    };
    Ok(ParsedArgs {
        json,
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
    if parsed.command == "archive" {
        return run_archive_command(parsed, root, legacy_warning);
    }
    if parsed.json && uses_shared_json(parsed.command.as_str()) {
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

fn render_loaded_project_command(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
    legacy_warning: String,
) -> CliResult {
    match parsed.command.as_str() {
        "get" => render_get(parsed, scan_result),
        "neighbourhood" => render_neighbourhood(parsed, scan_result),
        "files" => render_files(parsed, scan_result),
        "todos" => render_todos(parsed, scan_result),
        "decisions" => render_decisions(parsed, scan_result),
        "research" => render_research(parsed, scan_result),
        "sources" => render_sources(parsed, scan_result),
        "rationale" => render_rationale(parsed, scan_result),
        "status" => Ok(render_status(parsed, scan_result, root)),
        "context" => Ok(render_context(scan_result)),
        "hook" => return run_hook_command(parsed, root, scan_result, legacy_warning),
        "changes" | "show" | "docstring" | "rename" => {
            return err(2, "this command currently requires --json");
        }
        "dependents" | "depends" => render_dependencies(parsed, scan_result),
        "contract" => node_arg(&parsed.command_args).and_then(|node| {
            let node = scan_result.graph.resolve(node)?;
            let body = node
                .contracts
                .iter()
                .find_map(|path| scan_result.contracts.contracts.get(path))
                .filter(|contract| contract.node == node.id)
                .map(|contract| contract.body.clone())
                .unwrap_or_default();
            Ok(if parsed.json {
                format!(
                    "{{\"node\":\"{}\",\"contract\":\"{}\"}}\n",
                    esc(&node.id),
                    esc(&body)
                )
            } else {
                format!("Contract for {}:\n{}\n", node.id, body)
            })
        }),
        "order" => match query::order(&scan_result.graph) {
            Ok(response) => Ok(if parsed.json {
                format!("{{\"nodes\":{}}}\n", string_array_json(&response.nodes))
            } else {
                format!("Order:\n{}\n", lines(&response.nodes))
            }),
            Err(findings) => return findings_output(parsed.json, &findings),
        },
        "lint" | "scan" => {
            let response = query::lint(&scan_result.graph);
            let code = u8::from(
                response
                    .findings
                    .iter()
                    .any(|finding| finding.severity == FindingSeverity::Error),
            );
            let stdout = render_findings(&response.findings, parsed.json);
            return CliResult {
                code,
                stdout,
                stderr: legacy_warning,
            };
        }
        "check" => {
            let response = query::lint(&scan_result.graph);
            let target_node = parsed.command_args.get(1).map(String::as_str);
            let findings: Vec<_> = response
                .findings
                .iter()
                .filter(|f| target_node.is_none_or(|t| f.node.as_deref().is_some_and(|n| n == t)))
                .cloned()
                .collect();
            let stdout = render_findings(&findings, false);
            return CliResult {
                code: 0,
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

/// Command names not in the query registry but handled by the CLI.
const EXTRA_CLI_COMMANDS: &[&str] = &["accept", "check", "export", "onboard", "refine"];

/// Returns all command names the CLI recognises.
fn all_command_names() -> Vec<&'static str> {
    let mut names: Vec<&str> = registry().iter().map(|t| t.cli_name).collect();
    for cmd in EXTRA_CLI_COMMANDS {
        if !names.contains(cmd) {
            names.push(cmd);
        }
    }
    names.sort_unstable();
    names
}

/// Short description for each CLI command.
fn command_description(name: &str) -> &'static str {
    match name {
        "accept" => "Run acceptance gate for a change",
        "archive" => "Archive a completed change",
        "changes" => "List active changes",
        "check" => "Inspect findings for a node or project",
        "context" => "Structured project overview for agents",
        "contract" => "Show the contract for a node",
        "decisions" => "List decisions linked to a node",
        "dependents" => "List nodes that depend on a given node",
        "depends" => "List nodes a given node depends on",
        "docstring" => "Generate a docstring for a node",
        "export" => "Export project data",
        "files" => "List files owned by a node",
        "get" => "Inspect a node by ID",
        "hook" => "Run reconciliation hooks",
        "init" => "Scaffold a new cairn project",
        "lint" => "Lint the blueprint and report findings",
        "neighbourhood" => "Show a node and its neighbours",
        "onboard" => "Suggest blueprint entries for orphaned files",
        "refine" => "Re-run brownfield discovery and write a timestamped change",
        "order" => "Topological order of all nodes",
        "rationale" => "Show rationale chain for a node",
        "rename" => "Rename a node ID across the project",
        "research" => "List research linked to a node",
        "scan" => "Scan the project and report findings",
        "show" => "Show details of a change",
        "sources" => "List sources linked to a node",
        "status" => "Show project status summary",
        "todos" => "List todos linked to a node",
        "ui" => "Launch the web UI",
        _ => "",
    }
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
    out.push_str("  --version             Print version\n");
    out.push_str("  -h, --help            Print this help\n");
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
    let message = match best {
        Some((suggestion, dist)) if dist <= 2 => {
            format!("unknown command '{input}'. Did you mean '{suggestion}'?")
        }
        _ => format!(
            "unknown command '{input}'. Available commands: {}",
            names.join(", ")
        ),
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
            | "dependents"
            | "depends"
            | "order"
            | "lint"
            | "scan"
            | "status"
            | "rationale"
            | "todos"
            | "decisions"
            | "research"
            | "sources"
            | "changes"
            | "show"
            | "hook"
            | "rename"
            | "context"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{LazyLock, Mutex},
        time::{SystemTime, UNIX_EPOCH},
    };

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
            ("dependents", vec!["dependents", "app.api"]),
            ("depends", vec!["depends", "app.api"]),
            ("contract", vec!["contract", "app.api"]),
            ("order", vec!["order"]),
            ("lint", vec!["lint"]),
            ("scan", vec!["scan"]),
            ("hook", vec!["hook", "all"]),
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
    fn test_cli_change_commands_and_error_paths() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("change-commands")?;
        write_project(&root)?;
        write_change(&root)?;

        let changes = run_in(&root, &["--json", "changes"]);
        assert_eq!(changes.code, 0);
        assert!(changes.stdout.contains("phase-7.5a-test-fortification"));

        let show = run_in(&root, &["--json", "show", "phase-7.5a-test-fortification"]);
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

        let archive = run_in(&root, &["archive", "phase-7.5a-test-fortification"]);
        assert_eq!(archive.code, 1);
        assert!(archive.stderr.contains("not available"));

        let missing = run_in(&root, &["get"]);
        assert_eq!(missing.code, 1);
        assert!(missing.stdout.contains("CAIRN_CLI_MISSING_NODE"));

        let unknown = run_in(&root, &["unknown"]);
        assert_eq!(unknown.code, 2);
        assert!(unknown.stderr.contains("unknown command"));

        Ok(())
    }

    #[test]
    fn test_cli_init_and_version_commands() -> Result<(), Box<dyn std::error::Error>> {
        let root = temp_root("init")?;
        let init = run_in(&root, &["init"]);
        assert_eq!(init.code, 0);
        assert!(root.join("cairn.blueprint").exists());
        assert!(root.join("cairn.config.yaml").exists());

        let version = run(&["--version".to_owned()]);
        assert_eq!(version.code, 0);
        assert!(version.stdout.contains("cairn "));

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
        let _guard = TEST_CWD_LOCK.lock().expect("lock cwd");
        let old = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(root).expect("set cwd");
        let result = run(args);
        std::env::set_current_dir(old).expect("restore cwd");
        result
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
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"ctx\"\nrules: {}\n",
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
            // Verify several command names appear.
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
        // The existing test at test_cli_change_commands_and_error_paths
        // checks unknown.code == 2 and stderr contains "unknown command".
        // Verify the new message still matches.
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

    static TEST_CWD_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
}
