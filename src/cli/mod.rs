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

/// Command safety class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafetyClass {
    /// Read-only command.
    ReadOnly,
    /// Mutating command.
    Mutating,
}

/// Command metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandMetadata {
    /// Command name.
    pub name: &'static str,
    /// Request type identity.
    pub request: &'static str,
    /// Response type identity.
    pub response: &'static str,
    /// Safety class.
    pub safety: SafetyClass,
}

/// CLI execution result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliResult {
    /// Process exit code.
    pub code: u8,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
}

const COMMAND_REGISTRY: [CommandMetadata; 19] = [
    CommandMetadata {
        name: "get",
        request: "NodeRequest",
        response: "NodeResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "neighbourhood",
        request: "NodeRequest",
        response: "NeighbourhoodResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "contract",
        request: "NodeRequest",
        response: "ContractResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "todos",
        request: "ArtefactNodeRequest",
        response: "TodosResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "decisions",
        request: "ArtefactNodeRequest",
        response: "DecisionsResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "research",
        request: "NodeRequest",
        response: "ResearchResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "sources",
        request: "NodeRequest",
        response: "SourcesResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "rationale",
        request: "NodeRequest",
        response: "RationaleResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "status",
        request: "StatusRequest",
        response: "StatusResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "files",
        request: "NodeRequest",
        response: "FilesResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "dependents",
        request: "DependencyRequest",
        response: "DependencyResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "depends",
        request: "DependencyRequest",
        response: "DependencyResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "order",
        request: "OrderRequest",
        response: "OrderResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "lint",
        request: "LintRequest",
        response: "LintResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "scan",
        request: "ScanRequest",
        response: "ScanResponse",
        safety: SafetyClass::Mutating,
    },
    CommandMetadata {
        name: "init",
        request: "InitRequest",
        response: "InitResponse",
        safety: SafetyClass::Mutating,
    },
    CommandMetadata {
        name: "ui",
        request: "UiRequest",
        response: "UiServerResponse",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "hook",
        request: "HookRequest",
        response: "HookReport",
        safety: SafetyClass::ReadOnly,
    },
    CommandMetadata {
        name: "archive",
        request: "ArchiveRequest",
        response: "ArchiveResponse",
        safety: SafetyClass::Mutating,
    },
];

/// Returns Phase 1 command registry.
#[must_use]
pub const fn registry() -> &'static [CommandMetadata] {
    &COMMAND_REGISTRY
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
    match parsed.command.as_str() {
        "get" => render_get(parsed, &scan_result),
        "neighbourhood" => render_neighbourhood(parsed, &scan_result),
        "files" => render_files(parsed, &scan_result),
        "todos" => render_todos(parsed, &scan_result),
        "decisions" => render_decisions(parsed, &scan_result),
        "research" => render_research(parsed, &scan_result),
        "sources" => render_sources(parsed, &scan_result),
        "rationale" => render_rationale(parsed, &scan_result),
        "status" => Ok(render_status(parsed, &scan_result, root)),
        "hook" => return run_hook_command(parsed, root, &scan_result, legacy_warning),
        "dependents" | "depends" => render_dependencies(parsed, &scan_result),
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

fn run_hook_command(
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

fn run_archive_command(parsed: &ParsedArgs, root: &Path, legacy_warning: String) -> CliResult {
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

fn parse_hook_kind(value: &str) -> Option<HookKind> {
    match value {
        "structural" => Some(HookKind::Structural),
        "interface" => Some(HookKind::Interface),
        "tension" => Some(HookKind::Tension),
        "all" => Some(HookKind::All),
        _ => None,
    }
}

fn legacy_blueprint_warning(root: &Path) -> String {
    if root.join("cairn.blueprint").exists() && root.join("cairn.dsl").exists() {
        "warning: `cairn.dsl` is unused; remove it or rename remaining references to `cairn.blueprint`\n".to_owned()
    } else {
        String::new()
    }
}

fn run_ui_command(parsed: &ParsedArgs) -> CliResult {
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

fn render_get(parsed: &ParsedArgs, scan_result: &scanner::ScanResult) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        query::get(&scan_result.graph, node)
            .map(|response| render_node(&response.node, parsed.json))
    })
}

fn render_neighbourhood(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        query::neighbourhood(&scan_result.graph, node).map(|response| {
            let include_todos = parsed.command_args.iter().any(|arg| arg == "--include-todos");
            let include_research = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-research");
            let include_reviews = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-reviews");
            let include_deprecated = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-deprecated-decisions");
            let include_changes = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-changes");
            let node_ids = neighbourhood_ids(&scan_result.graph, &response.node.id);
            let decisions = scan_result
                .artefacts
                .decisions
                .iter()
                .filter(|decision| {
                    decision.nodes.iter().any(|node| node_ids.contains(node))
                        && (decision.status == DecisionStatus::Accepted || include_deprecated)
                })
                .cloned()
                .collect::<Vec<_>>();
            let todos = if include_todos {
                scan_result
                    .artefacts
                    .todos
                    .iter()
                    .filter(|todo| node_ids.contains(&todo.node))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let research = if include_research {
                research_for_nodes(scan_result, &node_ids)
            } else {
                Vec::new()
            };
            let reviews = if include_reviews {
                scan_result
                    .artefacts
                    .reviews
                    .iter()
                    .filter(|review| node_ids.contains(&review.node))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            if parsed.json {
                let active_changes = if include_changes {
                    ",\"active_changes\":[]"
                } else {
                    ""
                };
                format!(
                    "{{\"node\":{},\"inbound\":{},\"outbound\":{},\"contracts\":{},\"decisions\":{},\"todos\":{},\"research\":{},\"reviews\":{}{active_changes}}}\n",
                    node_json(&response.node),
                    string_array_json(&response.inbound),
                    string_array_json(&response.outbound),
                    string_array_json(&response.node.contracts),
                    decisions_json(&decisions),
                    todos_json(&todos),
                    research_json(&research),
                    reviews_json(&reviews)
                )
            } else {
                let active_changes = if include_changes {
                    "\nActive changes:\nNone"
                } else {
                    ""
                };
                format!(
                    "Node: {}\nInbound:\n{}\nOutbound:\n{}\nContracts:\n{}\nAccepted decisions:\n{}\nTodos:\n{}\nResearch:\n{}\nReviews:\n{}{active_changes}\n",
                    response.node.id,
                    lines(&response.inbound),
                    lines(&response.outbound),
                    lines(&response.node.contracts),
                    lines(&decisions.iter().map(decision_line).collect::<Vec<_>>()),
                    lines(&todos.iter().map(todo_line).collect::<Vec<_>>()),
                    lines(&research.iter().map(research_line).collect::<Vec<_>>()),
                    lines(&reviews.iter().map(review_line).collect::<Vec<_>>())
                )
            }
        })
    })
}

fn render_files(parsed: &ParsedArgs, scan_result: &scanner::ScanResult) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node_record = scan_result.graph.resolve(node)?;
        let target_reports_for_node: Vec<_> = scan_result
            .target_reports
            .iter()
            .filter(|r| r.target_id.node_id == node_record.id)
            .collect();
        let has_multi_target = target_reports_for_node.len() > 1;
        if parsed.json {
            let targets_json = if target_reports_for_node.is_empty() {
                "[]".to_string()
            } else {
                let items: Vec<String> = target_reports_for_node
                    .iter()
                    .map(|r| {
                        format!(
                            "{{\"path\":\"{}\",\"language\":\"{}\",\"reconciler_id\":\"{}\",\"files\":{},\"hash\":\"{}\"}}",
                            esc(&r.target_id.path.to_string_lossy()),
                            r.language.as_str(),
                            r.reconciler_id.0,
                            string_array_json(&r.claimed_files),
                            esc(&r.hash)
                        )
                    })
                    .collect();
                    format!("[{}]", items.join(","))
            };
            if has_multi_target {
                Ok(format!(
                    "{{\"node\":\"{}\",\"targets\":{}}}\n",
                    esc(&node_record.id),
                    targets_json
                ))
            } else {
                Ok(format!(
                    "{{\"node\":\"{}\",\"files\":{},\"targets\":{}}}\n",
                    esc(&node_record.id),
                    string_array_json(&node_record.files),
                    targets_json
                ))
            }
        } else {
            let mut output = format!("Files for {}:\n", node_record.id);
            if has_multi_target {
                for r in &target_reports_for_node {
                    use std::fmt::Write;
                    writeln!(
                        output,
                        "  {} ({}): {}",
                        r.target_id.path.display(),
                        r.language.as_str(),
                        r.claimed_files.join(", ")
                    ).unwrap();
                    writeln!(output, "    reconciler: {}", r.reconciler_id.0).unwrap();
                    writeln!(output, "    hash: {}", r.hash).unwrap();
                }
            } else if let Some(r) = target_reports_for_node.first() {
                use std::fmt::Write;
                writeln!(
                    output,
                    "  {}: {}",
                    r.target_id.path.display(),
                    r.claimed_files.join(", ")
                ).unwrap();
                writeln!(output, "  language: {}", r.language.as_str()).unwrap();
                writeln!(output, "  reconciler: {}", r.reconciler_id.0).unwrap();
                writeln!(output, "  hash: {}", r.hash).unwrap();
            } else {
                use std::fmt::Write;
                writeln!(output, "  {}", lines(&node_record.files)).unwrap();
            }
            output.push('\n');
            Ok(output)
        }
    })
}

fn render_todos(parsed: &ParsedArgs, scan_result: &scanner::ScanResult) -> Result<String, Finding> {
    let status = flag_value(&parsed.command_args, "--status").and_then(parse_todo_status_filter);
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let todos = scan_result
            .artefacts
            .todos
            .iter()
            .filter(|todo| {
                todo.node == node.id && status.is_none_or(|filter| todo.status == filter)
            })
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"todos\":{}}}\n",
                esc(&node.id),
                todos_json(&todos)
            )
        } else {
            format!(
                "Todos for {}:\n{}\n",
                node.id,
                lines(&todos.iter().map(todo_line).collect::<Vec<_>>())
            )
        })
    })
}

fn render_decisions(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let status =
        flag_value(&parsed.command_args, "--status").and_then(parse_decision_status_filter);
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.nodes.contains(&node.id)
                    && status.is_none_or(|filter| decision.status == filter)
            })
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"decisions\":{}}}\n",
                esc(&node.id),
                decisions_json(&decisions)
            )
        } else {
            format!(
                "Decisions for {}:\n{}\n",
                node.id,
                lines(&decisions.iter().map(decision_line).collect::<Vec<_>>())
            )
        })
    })
}

fn render_research(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]));
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"research\":{}}}\n",
                esc(&node.id),
                research_json(&research)
            )
        } else {
            format!(
                "Research for {}:\n{}\n",
                node.id,
                lines(&research.iter().map(research_line).collect::<Vec<_>>())
            )
        })
    })
}

fn render_sources(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]));
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"sources\":{}}}\n",
                esc(&node.id),
                sources_json(&sources)
            )
        } else {
            format!(
                "Sources for {}:\n{}\n",
                node.id,
                lines(&sources.iter().map(source_line).collect::<Vec<_>>())
            )
        })
    })
}

fn render_rationale(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let node_ids = neighbourhood_ids(&scan_result.graph, &node.id);
        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.status == DecisionStatus::Accepted
                    && decision.nodes.iter().any(|node| node_ids.contains(node))
            })
            .cloned()
            .collect::<Vec<_>>();
        let research_ids = decisions
            .iter()
            .flat_map(|decision| decision.informed_by.iter())
            .cloned()
            .collect::<BTreeSet<_>>();
        let source_ids = decisions
            .iter()
            .flat_map(|decision| decision.informed_by.iter())
            .cloned()
            .chain(
                scan_result
                    .artefacts
                    .research
                    .iter()
                    .filter(|research| research_ids.contains(&research.id))
                    .flat_map(|research| research.sources.iter().cloned()),
            )
            .collect::<BTreeSet<_>>();
        let research = scan_result
            .artefacts
            .research
            .iter()
            .filter(|research| research_ids.contains(&research.id))
            .cloned()
            .collect::<Vec<_>>();
        let sources = scan_result
            .artefacts
            .sources
            .iter()
            .filter(|source| source_ids.contains(&source.id))
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"decisions\":{},\"research\":{},\"sources\":{}}}\n",
                esc(&node.id),
                decisions_json(&decisions),
                research_json(&research),
                sources_json(&sources)
            )
        } else {
            format!(
                "Rationale for {}:\nDecisions:\n{}\nResearch:\n{}\nSources:\n{}\n",
                node.id,
                lines(&decisions.iter().map(decision_line).collect::<Vec<_>>()),
                lines(&research.iter().map(research_line).collect::<Vec<_>>()),
                lines(&sources.iter().map(source_line).collect::<Vec<_>>())
            )
        })
    })
}

fn render_status(parsed: &ParsedArgs, scan_result: &scanner::ScanResult, root: &Path) -> String {
    let open = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open || todo.status == TodoStatus::InProgress)
        .cloned()
        .collect::<Vec<_>>();
    let log_entries = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            content
                .lines()
                .rev()
                .take(5)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if parsed.json {
        format!(
            "{{\"active_changes\":[],\"open_todos\":{},\"recent_log_entries\":{}}}\n",
            todos_json(&open),
            string_array_json(&log_entries)
        )
    } else {
        format!(
            "Status:\nActive changes:\nNone\nOpen todos:\n{}\nRecent log entries:\n{}\n",
            lines(&open.iter().map(todo_line).collect::<Vec<_>>()),
            lines(&log_entries)
        )
    }
}

fn render_dependencies(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let transitive = parsed.command_args.iter().any(|arg| arg == "--transitive");
    node_arg(&parsed.command_args).and_then(|node| {
        let response = if parsed.command == "depends" {
            query::depends(&scan_result.graph, node, transitive)
        } else {
            query::dependents(&scan_result.graph, node, transitive)
        }?;
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"nodes\":{}}}\n",
                esc(&response.node),
                string_array_json(&response.nodes)
            )
        } else {
            format!("{}:\n{}\n", response.node, lines(&response.nodes))
        })
    })
}

fn requires_valid_map(command: &str) -> bool {
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

fn init_project(root: &Path) -> CliResult {
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

fn node_arg(args: &[String]) -> Result<&str, Finding> {
    args.get(1).map(String::as_str).ok_or_else(|| Finding {
        code: "CAIRN_CLI_MISSING_NODE".to_owned(),
        severity: FindingSeverity::Error,
        message: "node argument is required".to_owned(),
        node: None,
        path: None,
    })
}

fn render_node(node: &NodeRecord, json: bool) -> String {
    if json {
        format!("{}\n", node_json(node))
    } else {
        format!(
            "ID: {}\nName: {}\nDescription: {}\nState: {:?}\n",
            node.id, node.name, node.description, node.state
        )
    }
}

fn render_findings(findings: &[Finding], json: bool) -> String {
    if json {
        format!(
            "{{\"findings\":[{}]}}\n",
            findings
                .iter()
                .map(finding_json)
                .collect::<Vec<_>>()
                .join(",")
        )
    } else if findings.is_empty() {
        "Findings:\nNone\n".to_owned()
    } else {
        format!(
            "Findings:\n{}\n",
            findings
                .iter()
                .map(|finding| format!(
                    "{:?}: {} {}",
                    finding.severity, finding.code, finding.message
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn node_json(node: &NodeRecord) -> String {
    format!(
        "{{\"id\":\"{}\",\"name\":\"{}\",\"description\":\"{}\",\"state\":\"{:?}\",\"children\":{},\"files\":{}}}",
        esc(&node.id),
        esc(&node.name),
        esc(&node.description),
        node.state,
        string_array_json(&node.children),
        string_array_json(&node.files)
    )
}

fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{:?}\",\"message\":\"{}\"}}",
        esc(&finding.code),
        finding.severity,
        esc(&finding.message)
    )
}

fn todos_json(todos: &[Todo]) -> String {
    format!(
        "[{}]",
        todos
            .iter()
            .map(|todo| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"status\":\"{}\",\"created\":\"{}\",\"satisfies\":\"{}\"}}",
                    esc(&todo.path),
                    esc(&todo.node),
                    todo_status(todo.status),
                    esc(&todo.created),
                    esc(todo.satisfies.as_deref().unwrap_or(""))
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn decisions_json(decisions: &[Decision]) -> String {
    format!(
        "[{}]",
        decisions
            .iter()
            .map(|decision| {
                format!(
                    "{{\"id\":\"{}\",\"status\":\"{}\",\"nodes\":{},\"informed_by\":{},\"supersedes\":{},\"refines\":{},\"related\":{}}}",
                    esc(&decision.id),
                    decision_status(decision.status),
                    string_array_json(&decision.nodes),
                    string_array_json(&decision.informed_by),
                    string_array_json(&decision.supersedes),
                    string_array_json(&decision.refines),
                    string_array_json(&decision.related)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn research_json(research: &[Research]) -> String {
    format!(
        "[{}]",
        research
            .iter()
            .map(|item| {
                format!(
                    "{{\"id\":\"{}\",\"nodes\":{},\"sources\":{},\"date\":\"{}\"}}",
                    esc(&item.id),
                    string_array_json(&item.nodes),
                    string_array_json(&item.sources),
                    esc(&item.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn reviews_json(reviews: &[Review]) -> String {
    format!(
        "[{}]",
        reviews
            .iter()
            .map(|review| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"review_type\":\"{}\",\"date\":\"{}\",\"reviewer\":\"{}\"}}",
                    esc(&review.path),
                    esc(&review.node),
                    review_type(review.review_type),
                    esc(&review.date),
                    esc(&review.reviewer)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn sources_json(sources: &[Source]) -> String {
    format!(
        "[{}]",
        sources
            .iter()
            .map(|source| {
                format!(
                    "{{\"id\":\"{}\",\"file\":\"{}\",\"verification\":\"{}\",\"type\":\"{}\",\"date\":\"{}\"}}",
                    esc(&source.id),
                    esc(&source.file),
                    source_verification(source.verification),
                    esc(&source.source_type),
                    esc(&source.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn neighbourhood_ids(graph: &crate::map::Graph, node: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::from([node.to_owned()]);
    if let Some(edges) = graph.inbound.get(node) {
        ids.extend(edges.iter().map(|edge| edge.from.clone()));
    }
    if let Some(edges) = graph.outbound.get(node) {
        ids.extend(edges.iter().map(|edge| edge.to.clone()));
    }
    ids
}

fn research_for_nodes(
    scan_result: &scanner::ScanResult,
    nodes: &BTreeSet<String>,
) -> Vec<Research> {
    scan_result
        .artefacts
        .research
        .iter()
        .filter(|research| research.nodes.iter().any(|node| nodes.contains(node)))
        .cloned()
        .collect()
}

fn sources_for_nodes(scan_result: &scanner::ScanResult, nodes: &BTreeSet<String>) -> Vec<Source> {
    let source_ids = scan_result
        .artefacts
        .research
        .iter()
        .filter(|research| research.nodes.iter().any(|node| nodes.contains(node)))
        .flat_map(|research| research.sources.iter().cloned())
        .chain(
            scan_result
                .artefacts
                .decisions
                .iter()
                .filter(|decision| decision.nodes.iter().any(|node| nodes.contains(node)))
                .flat_map(|decision| decision.informed_by.iter().cloned()),
        )
        .collect::<BTreeSet<_>>();
    scan_result
        .artefacts
        .sources
        .iter()
        .filter(|source| source_ids.contains(&source.id))
        .cloned()
        .collect()
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find_map(|pair| (pair[0] == flag).then_some(pair[1].as_str()))
}

fn parse_todo_status_filter(value: &str) -> Option<TodoStatus> {
    match value {
        "open" => Some(TodoStatus::Open),
        "in_progress" => Some(TodoStatus::InProgress),
        "done" => Some(TodoStatus::Done),
        "blocked" => Some(TodoStatus::Blocked),
        _ => None,
    }
}

fn parse_decision_status_filter(value: &str) -> Option<DecisionStatus> {
    match value {
        "proposed" => Some(DecisionStatus::Proposed),
        "accepted" => Some(DecisionStatus::Accepted),
        "deprecated" => Some(DecisionStatus::Deprecated),
        "superseded" => Some(DecisionStatus::Superseded),
        _ => None,
    }
}

fn todo_line(todo: &Todo) -> String {
    format!("{} [{}] {}", todo.node, todo_status(todo.status), todo.path)
}

fn decision_line(decision: &Decision) -> String {
    format!(
        "{} [{}] {}",
        decision.id,
        decision_status(decision.status),
        decision.nodes.join(", ")
    )
}

fn research_line(research: &Research) -> String {
    format!("{} sources: {}", research.id, research.sources.join(", "))
}

fn review_line(review: &Review) -> String {
    format!(
        "{} [{}] {}",
        review.node,
        review_type(review.review_type),
        review.path
    )
}

fn source_line(source: &Source) -> String {
    format!(
        "{} [{}] {}",
        source.id,
        source_verification(source.verification),
        source.file
    )
}

const fn todo_status(status: TodoStatus) -> &'static str {
    match status {
        TodoStatus::Open => "open",
        TodoStatus::InProgress => "in_progress",
        TodoStatus::Done => "done",
        TodoStatus::Blocked => "blocked",
    }
}

const fn decision_status(status: DecisionStatus) -> &'static str {
    match status {
        DecisionStatus::Proposed => "proposed",
        DecisionStatus::Accepted => "accepted",
        DecisionStatus::Deprecated => "deprecated",
        DecisionStatus::Superseded => "superseded",
    }
}

const fn review_type(review_type: ReviewType) -> &'static str {
    match review_type {
        ReviewType::Human => "human",
        ReviewType::AgentIntrospective => "agent_introspective",
        ReviewType::AgentCrossModel => "agent_cross_model",
    }
}

const fn source_verification(verification: SourceVerification) -> &'static str {
    match verification {
        SourceVerification::Verified => "verified",
        SourceVerification::External => "external",
        SourceVerification::Unverified => "unverified",
    }
}

fn findings_output(json: bool, findings: &[Finding]) -> CliResult {
    CliResult {
        code: 1,
        stdout: render_findings(findings, json),
        stderr: String::new(),
    }
}

fn finding_output(json: bool, finding: Finding) -> CliResult {
    findings_output(json, &[finding])
}

fn error_output(json: bool, code: &str, message: &str) -> CliResult {
    let finding = Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Error,
        message: message.to_owned(),
        node: None,
        path: None,
    };
    finding_output(json, finding)
}

fn ok(stdout: String) -> CliResult {
    CliResult {
        code: 0,
        stdout,
        stderr: String::new(),
    }
}

fn err(code: u8, message: &str) -> CliResult {
    CliResult {
        code,
        stdout: String::new(),
        stderr: format!("{message}\n"),
    }
}

fn string_array_json(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", esc(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn lines(values: &[String]) -> String {
    if values.is_empty() {
        "None".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("- {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
