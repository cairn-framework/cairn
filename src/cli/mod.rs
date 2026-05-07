// cairn:allow-large-module reason: CLI dispatch hub for many subcommands; the natural seam (per-command modules) already exists for newer commands like export and accept; legacy commands grew here historically and a refactor will land in a future phase.
//! CLI registry, command execution, and renderers.

use std::{
    collections::BTreeSet,
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
    run_hook_command, run_shared_json_command, run_ui_command,
};
use format::{
    err, error_output, esc, finding_output, findings_output, lines, node_arg, ok, render_findings,
    string_array_json,
};
use render::{
    render_decisions, render_dependencies, render_files, render_get, render_neighbourhood,
    render_rationale, render_research, render_sources, render_status, render_todos,
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
    let parsed = match parse_args(args) {
        Ok(parsed) => parsed,
        Err(result) => return result,
    };
    if parsed.command == "init" {
        return init_project(Path::new("."));
    }
    if parsed.command == "ui" {
        return run_ui_command(&parsed);
    }
    if parsed.command == "accept" {
        let change_id = parsed.command_args.get(1).map(String::as_str);
        return crate::cli::accept::run_accept_gate(change_id);
    }
    if parsed.command == "export" {
        return export::run(&parsed.command_args, &parsed.file, &parsed.changes_dir);
    }
    if parsed.command == "check" && parsed.json {
        return err(
            1,
            "cairn check has no JSON mode; use `cairn lint --json` for JSON output",
        );
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
            if parsed.json {
                return err(
                    1,
                    "cairn check has no JSON mode; use `cairn lint --json` for JSON output",
                );
            }
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
        _ => return err(2, "unknown command"),
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

    static TEST_CWD_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
}
