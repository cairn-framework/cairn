//! CLI registry, command execution, and renderers.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    ontology::{
        graph::{Finding, FindingSeverity, NodeRecord},
        query,
    },
    scanner, version_label,
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

/// Returns Phase 1 command registry.
#[must_use]
pub const fn registry() -> &'static [CommandMetadata] {
    &[
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
    ]
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
    run_project_command(&parsed)
}

struct ParsedArgs {
    json: bool,
    file: PathBuf,
    command: String,
    command_args: Vec<String>,
}

fn parse_args(args: &[String]) -> Result<ParsedArgs, CliResult> {
    let mut json = false;
    let mut file = PathBuf::from("cairn.dsl");
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
            value => command_args.push(value.to_owned()),
        }
    }
    let Some(command) = command_args.first().map(String::as_str) else {
        return Err(err(2, "usage: cairn <command> [--file path] [--json]"));
    };
    Ok(ParsedArgs {
        json,
        file,
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
    let scan_result = if parsed.command == "scan" {
        scanner::scan(root, &parsed.file)
    } else {
        scanner::load_project(root, &parsed.file)
    };
    let scan_result = match scan_result {
        Ok(result) => result,
        Err(error) => return error_output(parsed.json, "CAIRN_COMMAND_FAILED", &error),
    };
    match parsed.command.as_str() {
        "get" => render_get(parsed, &scan_result),
        "neighbourhood" => render_neighbourhood(parsed, &scan_result),
        "files" => render_files(parsed, &scan_result),
        "dependents" | "depends" => render_dependencies(parsed, &scan_result),
        "contract" => node_arg(&parsed.command_args).and_then(|node| {
            let node = scan_result.graph.resolve(node)?;
            let body = node
                .contracts
                .iter()
                .find_map(|path| scan_result.contracts.contracts.get(path))
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
                stderr: String::new(),
            };
        }
        _ => return err(2, "unknown command"),
    }
    .map_or_else(
        |finding| finding_output(parsed.json, finding),
        |stdout| CliResult {
            code: 0,
            stdout,
            stderr: String::new(),
        },
    )
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
            if parsed.json {
                format!(
                    "{{\"node\":{},\"inbound\":{},\"outbound\":{}}}\n",
                    node_json(&response.node),
                    string_array_json(&response.inbound),
                    string_array_json(&response.outbound)
                )
            } else {
                format!(
                    "Node: {}\nInbound:\n{}\nOutbound:\n{}\n",
                    response.node.id,
                    lines(&response.inbound),
                    lines(&response.outbound)
                )
            }
        })
    })
}

fn render_files(parsed: &ParsedArgs, scan_result: &scanner::ScanResult) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        query::files(&scan_result.graph, node).map(|response| {
            if parsed.json {
                format!(
                    "{{\"node\":\"{}\",\"files\":{}}}\n",
                    esc(&response.node),
                    string_array_json(&response.files)
                )
            } else {
                format!("Files for {}:\n{}\n", response.node, lines(&response.files))
            }
        })
    })
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

fn init_project(root: &Path) -> CliResult {
    let writes = [
        (
            "cairn.dsl",
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
