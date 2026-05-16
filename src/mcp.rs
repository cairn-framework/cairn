//! Minimal MCP stdio transport for Cairn query tools.

use std::{
    collections::BTreeSet,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use serde_json::{Value, json};

use crate::query_api::{self, QueryFlag, QueryRequest, SafetyClass};

/// MCP server configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerConfig {
    /// Project root.
    pub root: PathBuf,
    /// Blueprint path.
    pub blueprint_path: PathBuf,
    /// Active changes directory.
    pub changes_dir: PathBuf,
    /// Whether mutating tools are listed and callable.
    pub allow_mutating_tools: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            blueprint_path: PathBuf::from("cairn.blueprint"),
            changes_dir: PathBuf::from("meta/changes"),
            allow_mutating_tools: false,
        }
    }
}

/// Runs the MCP server over newline-delimited stdio JSON-RPC.
///
/// # Errors
///
/// Returns an I/O error when stdin or stdout cannot be read or written.
pub fn serve_stdio(config: &ServerConfig) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    serve(stdin.lock(), &mut stdout, config)
}

/// Serves MCP requests from a buffered reader into a writer.
///
/// # Errors
///
/// Returns an I/O error when request reading or response writing fails.
pub fn serve<R, W>(reader: R, writer: &mut W, config: &ServerConfig) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let response = handle_line(&line, config);
        writeln!(writer, "{response}")?;
        writer.flush()?;
    }
    Ok(())
}

/// Handles a single JSON-RPC request line.
#[must_use]
pub fn handle_line(line: &str, config: &ServerConfig) -> Value {
    let request = match serde_json::from_str::<Value>(line) {
        Ok(value) => value,
        Err(error) => {
            return jsonrpc_error(
                &Value::Null,
                -32700,
                "Parse error",
                &json!({ "code": "CAIRN_MCP_PARSE_ERROR", "message": error.to_string() }),
            );
        }
    };
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let method = request.get("method").and_then(Value::as_str).unwrap_or("");
    match method {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "serverInfo": { "name": "cairn-mcp", "version": crate::package_version() },
                "capabilities": { "tools": {} },
            },
        }),
        "tools/list" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": { "tools": tools_json(config.allow_mutating_tools) },
        }),
        "tools/call" => call_tool(&id, &request, config),
        _ => jsonrpc_error(
            &id,
            -32601,
            "Method not found",
            &json!({ "code": "CAIRN_MCP_METHOD_NOT_FOUND", "message": format!("unknown method `{method}`") }),
        ),
    }
}

/// Parses command line arguments for the MCP binary.
///
/// # Errors
///
/// Returns a message when an option is missing its value or is unknown.
pub fn config_from_args(args: &[String]) -> Result<ServerConfig, String> {
    let mut config = ServerConfig::default();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--allow-mutating-tools" => config.allow_mutating_tools = true,
            "--root" => {
                let Some(value) = iter.next() else {
                    return Err("--root requires a path".to_owned());
                };
                config.root = PathBuf::from(value);
            }
            "--file" => {
                let Some(value) = iter.next() else {
                    return Err("--file requires a path".to_owned());
                };
                config.blueprint_path = PathBuf::from(value);
            }
            "--changes-dir" => {
                let Some(value) = iter.next() else {
                    return Err("--changes-dir requires a path".to_owned());
                };
                config.changes_dir = PathBuf::from(value);
            }
            "--help" | "-h" => return Err(help_text()),
            other => return Err(format!("unknown option `{other}`\n\n{}", help_text())),
        }
    }
    Ok(config)
}

fn call_tool(id: &Value, request: &Value, config: &ServerConfig) -> Value {
    let params = request.get("params").unwrap_or(&Value::Null);
    let Some(name) = params.get("name").and_then(Value::as_str) else {
        return jsonrpc_error(
            id,
            -32602,
            "Invalid params",
            &json!({ "code": "CAIRN_MCP_MISSING_TOOL", "message": "`params.name` is required" }),
        );
    };
    let arguments = params.get("arguments").unwrap_or(&Value::Null);
    let query = request_from_arguments(name, arguments, config.allow_mutating_tools);
    let root = config.root.as_path();
    let blueprint_path = resolve_path(root, &config.blueprint_path);
    let changes_dir = resolve_path(root, &config.changes_dir);
    match query_api::execute(root, &blueprint_path, &changes_dir, &query) {
        Ok(response) => {
            let text = query_api::envelope_json(&response).to_string();
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "content": [{ "type": "text", "text": text }] },
            })
        }
        Err(error) => jsonrpc_error(id, -32000, &error.message, &query_api::error_json(&error)),
    }
}

fn request_from_arguments(
    name: &str,
    arguments: &Value,
    allow_mutating_tools: bool,
) -> QueryRequest {
    QueryRequest {
        tool: name.to_owned(),
        node: string_arg(arguments, "node").or_else(|| string_arg(arguments, "id")),
        change: string_arg(arguments, "change").or_else(|| string_arg(arguments, "change_id")),
        old_id: string_arg(arguments, "old_id"),
        new_id: string_arg(arguments, "new_id"),
        status: string_arg(arguments, "status").or_else(|| string_arg(arguments, "kind")),
        language: string_arg(arguments, "language"),
        flags: argument_flags(arguments),
        mutating: allow_mutating_tools && bool_arg(arguments, "mutating"),
    }
}

fn argument_flags(arguments: &Value) -> BTreeSet<QueryFlag> {
    let mut flags = BTreeSet::new();
    let pairs = [
        ("transitive", QueryFlag::Transitive),
        ("include_todos", QueryFlag::IncludeTodos),
        ("include_research", QueryFlag::IncludeResearch),
        ("include_reviews", QueryFlag::IncludeReviews),
        (
            "include_deprecated_decisions",
            QueryFlag::IncludeDeprecatedDecisions,
        ),
        ("include_changes", QueryFlag::IncludeChanges),
        ("force", QueryFlag::Force),
    ];
    for (argument, flag) in pairs {
        if bool_arg(arguments, argument) {
            flags.insert(flag);
        }
    }
    flags
}

fn tools_json(allow_mutating: bool) -> Vec<Value> {
    query_api::visible_tools(allow_mutating)
        .iter()
        .map(|tool| {
            json!({
                "name": tool.mcp_name,
                "description": tool_description(tool.cli_name, tool.safety),
                "inputSchema": input_schema(tool.request_schema),
                "annotations": {
                    "readOnlyHint": tool.safety == SafetyClass::ReadOnly,
                    "destructiveHint": tool.safety == SafetyClass::Mutating,
                },
                "cairn": {
                    "cli_name": tool.cli_name,
                    "request_schema": tool.request_schema,
                    "response_schema": tool.response_schema,
                    "safety": match tool.safety {
                        SafetyClass::ReadOnly => "read_only",
                        SafetyClass::Mutating => "mutating",
                    },
                },
            })
        })
        .collect()
}

fn input_schema(schema: &str) -> Value {
    let properties = match schema {
        "NodeRequest" | "ArtefactNodeRequest" => json!({
            "node": { "type": "string" },
        }),
        "NeighbourhoodRequest" => json!({
            "node": { "type": "string" },
            "include_todos": { "type": "boolean" },
            "include_research": { "type": "boolean" },
            "include_reviews": { "type": "boolean" },
            "include_deprecated_decisions": { "type": "boolean" },
            "include_changes": { "type": "boolean" },
        }),
        "DependencyRequest" => json!({
            "node": { "type": "string" },
            "transitive": { "type": "boolean" },
        }),
        "ShowChangeRequest" => json!({
            "change": { "type": "string" },
        }),
        "RenameRequest" => json!({
            "old_id": { "type": "string" },
            "new_id": { "type": "string" },
            "mutating": { "type": "boolean" },
        }),
        "ArchiveRequest" => json!({
            "change": { "type": "string" },
            "mutating": { "type": "boolean" },
        }),
        "DocstringRequest" => json!({
            "node": { "type": "string" },
            "language": { "type": "string" },
        }),
        "InitFromCodeRequest" => json!({
            "force": { "type": "boolean" },
            "mutating": { "type": "boolean" },
        }),
        "RefineRequest" => json!({
            "mutating": { "type": "boolean" },
        }),
        _ => json!({}),
    };
    json!({
        "type": "object",
        "properties": properties,
        "additionalProperties": true,
    })
}

fn tool_description(name: &str, safety: SafetyClass) -> String {
    let safety = match safety {
        SafetyClass::ReadOnly => "read-only",
        SafetyClass::Mutating => "mutating",
    };
    format!("Cairn `{name}` query ({safety}).")
}

fn jsonrpc_error(id: &Value, code: i64, message: &str, data: &Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
            "data": data,
        },
    })
}

fn string_arg(arguments: &Value, name: &str) -> Option<String> {
    arguments
        .get(name)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn bool_arg(arguments: &Value, name: &str) -> bool {
    arguments
        .get(name)
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn resolve_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn help_text() -> String {
    "usage: cairn-mcp [--root path] [--file path] [--changes-dir path] [--allow-mutating-tools]"
        .to_owned()
}
