// cairn:allow-large-module reason: query dispatch hub for many tools (execute/execute_with_scan/execute_data_with_scan) plus scan-aware wrappers and envelope/error serialisation; kept cohesive as the single query entry point.
//! Shared structured query API used by CLI JSON output and MCP.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs,
    path::Path,
};

use serde_json::{Value, json};

use crate::{
    artefacts::{
        contract::Contract,
        registry::{
            Decision, DecisionStatus, Research, Review, Source, SourceVerification, Todo,
            TodoStatus,
        },
    },
    changes,
    hooks::{self, ExitDecision, HookKind},
    map::{
        graph::{Finding, FindingSeverity, NodeRecord},
        query,
    },
    scanner::{self, config},
};

mod change_queries;
mod gates;
mod handlers;
mod registry;
mod serialise;
mod util;

use change_queries::dispatch_change_tool;
pub(crate) use handlers::{
    CleanItem, NextSelection, decision_summary, from_finding_action, health_json,
    open_native_todos, remediate_actions_raw, remediate_json, select_next, work_item_for_selection,
};
pub use handlers::{
    RemediateResponse, StatusActiveChange, StatusResponse, StatusTodo, WorkItem, WorkItemSource,
};
use handlers::{beads_json, blueprint_json, ui_meta_json};
use handlers::{
    bundle_json, context_json, contract_json, decisions_response_json, dependency_json,
    docstring_json, files_json, frontier_json, graph_response_json, hook_json, islands_json,
    locate_json, neighbourhood_json, rationale_json, research_response_json, sources_response_json,
    status_json, todos_response_json,
};
use registry::{metadata_for_tool, registry_slice};
use serialise::{backlog_item_detail_json, findings_json, node_json, relevant_rules};
use util::{finding_error, findings_error, load_for, required};

/// Schema version stamped on every query-API JSON `data` payload.
///
/// Both the CLI `--json` surface (which prints `data` directly) and the MCP
/// envelope (which wraps `data`) carry this version on the top-level data
/// object so consumers can branch on the output contract uniformly.
pub const SCHEMA_VERSION: u32 = 7;

/// Tool safety class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafetyClass {
    /// Tool reads project state without mutation.
    ReadOnly,
    /// Tool may mutate project state or generated artefacts.
    Mutating,
}

/// Query tool metadata shared by CLI and MCP registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolMetadata {
    /// CLI command name.
    pub cli_name: &'static str,
    /// MCP tool name.
    pub mcp_name: &'static str,
    /// Request schema identity.
    pub request_schema: &'static str,
    /// Response schema identity.
    pub response_schema: &'static str,
    /// Tool safety class.
    pub safety: SafetyClass,
    /// Human-readable one-line description, shown in CLI help and the
    /// single source of truth for command documentation.
    pub description: &'static str,
}

/// Structured query request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryRequest {
    /// Tool or command name.
    pub tool: String,
    /// Optional node name or ID.
    pub node: Option<String>,
    /// Optional symbol name for `locate` exact-name lookup.
    pub symbol: Option<String>,
    /// Optional change ID.
    pub change: Option<String>,
    /// Optional old node ID for rename operations.
    pub old_id: Option<String>,
    /// Optional new node ID for rename operations.
    pub new_id: Option<String>,
    /// Optional status filter.
    pub status: Option<String>,
    /// Optional language for docstring generation.
    pub language: Option<String>,
    /// Optional query flags.
    pub flags: BTreeSet<QueryFlag>,
    /// Explicitly allow a mutating tool invocation.
    pub mutating: bool,
}

/// Optional query flags.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum QueryFlag {
    /// Include transitive dependency traversal.
    Transitive,
    /// Traverse inbound dependencies (dependents) instead of outbound (depends).
    Inbound,
    /// Include the opt-in `symbols` field in `get` responses.
    Symbols,
    /// Include todos in neighbourhood responses.
    IncludeTodos,
    /// Include research in neighbourhood responses.
    IncludeResearch,
    /// Include reviews in neighbourhood responses.
    IncludeReviews,
    /// Include deprecated decisions in neighbourhood responses.
    IncludeDeprecatedDecisions,
    /// Include active change summaries in neighbourhood responses.
    IncludeChanges,
    /// Force overwrite of existing state.
    Force,
    /// Accept the edited version of a draft instead of the generated text.
    Edited,
}

impl QueryRequest {
    fn has(&self, flag: QueryFlag) -> bool {
        self.flags.contains(&flag)
    }
}

/// Structured successful query response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryResponse {
    /// Project context from `cairn.config.yaml`.
    pub project_context: String,
    /// Relevant configured rules for the response.
    pub rules: BTreeMap<String, String>,
    /// Tool-specific data.
    pub data: Value,
    /// Relevant findings.
    pub findings: Vec<Finding>,
}
/// Wire finding projection emitted inside the query envelope.
#[derive(Clone, Debug, schemars::JsonSchema)]
pub struct EnvelopeFinding {
    /// Stable finding code.
    pub code: String,
    /// Severity label.
    pub severity: String,
    /// Human-readable message.
    pub message: String,
    /// Optional node identifier, always present on the wire.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub node: Option<String>,
    /// Optional file path, always present on the wire.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub path: Option<String>,
    /// Deferring decision id, always present on the wire: the accepted
    /// decision the finding is deferred by, or null when live.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub deferred_by: Option<String>,
    /// Parking todo id, always present on the wire: the `blocked` todo whose
    /// `defers:` reference parks this Info finding, or null when live.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub parked_by: Option<String>,
}

/// Versioned, heterogeneous query data carried by the outer envelope.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct VersionedDataSchema {
    /// Wire schema version.
    pub schema_version: u32,
    /// Tool-specific payload properties.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Schema projection for the successful MCP query response envelope.
#[derive(Clone, Debug, schemars::JsonSchema)]
pub struct ResponseEnvelopeSchema {
    /// Project context from configuration.
    pub project_context: String,
    /// Relevant configured rules.
    pub rules: BTreeMap<String, String>,
    /// Versioned tool-specific data.
    pub data: VersionedDataSchema,
    /// Relevant findings.
    pub findings: Vec<EnvelopeFinding>,
}

fn nullable_string_schema(
    generator: &mut schemars::r#gen::SchemaGenerator,
) -> schemars::schema::Schema {
    generator.subschema_for::<Option<String>>()
}

/// Stable query error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryError {
    /// Stable machine-readable error code.
    pub code: String,
    /// Human-readable error message.
    pub message: String,
    /// Optional source span or path.
    pub source_span: Option<String>,
    /// Optional remediation text.
    pub remediation: Option<String>,
}

impl fmt::Display for QueryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for QueryError {}

/// Returns the shared query tool registry.
#[must_use]
pub const fn registry() -> &'static [ToolMetadata] {
    registry_slice()
}

/// Canonical implementations live in `serialise`; these re-exports make
/// them accessible as `crate::query_api::<name>` so the CLI render layer
/// shares them instead of keeping copies (todo.simplify-dedup-format-util).
/// `requires_valid_map` is additionally used by both the CLI dispatch loop
/// and the MCP query-API path to gate commands on a clean graph.
pub(crate) use gates::format_gates;
pub(crate) use serialise::{
    accepted_decision_ids, decision_status, neighbourhood_ids, parse_decision_status_filter,
    requires_valid_map, research_for_nodes, todo_status,
};

/// Returns tools visible for a server configuration.
#[must_use]
pub fn visible_tools(allow_mutating: bool) -> Vec<ToolMetadata> {
    registry_slice()
        .iter()
        .copied()
        .filter(|tool| allow_mutating || tool.safety == SafetyClass::ReadOnly)
        .collect()
}

/// Executes a query and composes project context and relevant rules.
///
/// # Errors
///
/// Returns a stable query error when loading, validation, or query execution fails.
pub fn execute(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
) -> Result<QueryResponse, QueryError> {
    let metadata = metadata_for_tool(&request.tool).ok_or_else(|| QueryError {
        code: "CAIRN_QUERY_UNKNOWN_TOOL".to_owned(),
        message: format!("unknown query tool `{}`", request.tool),
        source_span: None,
        remediation: Some(
            "Call tools/list or `cairn --help` to inspect supported queries.".to_owned(),
        ),
    })?;
    if metadata.safety == SafetyClass::Mutating && !request.mutating {
        return Err(QueryError {
            code: "CAIRN_QUERY_MUTATION_NOT_ALLOWED".to_owned(),
            message: format!("tool `{}` requires an explicit mutating flag", request.tool),
            source_span: None,
            remediation: Some(
                "Set `mutating` to true and start the MCP server with mutating tools enabled."
                    .to_owned(),
            ),
        });
    }

    let loaded_config = config::load(root).map_err(|error| QueryError {
        code: error.code,
        message: error.message,
        source_span: Some(root.join("cairn.config.yaml").display().to_string()),
        remediation: None,
    })?;
    let mut data = execute_data(
        root,
        blueprint_path,
        changes_dir,
        request,
        metadata,
        &loaded_config,
    )?;
    // Stamp every command's data payload with the schema version so the CLI
    // `--json` output and the MCP envelope share one versioned contract. Every
    // `execute_data` arm returns a JSON object; assert that so a future arm
    // returning a non-object (which would silently skip the stamp) is caught.
    debug_assert!(
        matches!(data, Value::Object(_)),
        "execute_data must return a JSON object so the schema_version stamp applies",
    );
    if let Value::Object(map) = &mut data {
        map.insert("schema_version".to_owned(), json!(SCHEMA_VERSION));
    }
    let rules = relevant_rules(&loaded_config.rules, &request.tool);
    Ok(QueryResponse {
        project_context: loaded_config.context,
        rules,
        data,
        findings: Vec::new(),
    })
}
/// Thin wrapper for the HTTP server: runs a read-only query against an already
/// computed scan result, preserving the server's cached `ScanResult` for latency.
///
/// # Errors
///
/// Returns a stable query error when the tool is unknown, the config fails to
/// load, or query execution against the supplied scan result fails.
pub fn execute_with_scan(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    scan_result: &scanner::ScanResult,
) -> Result<QueryResponse, QueryError> {
    let metadata = metadata_for_tool(&request.tool).ok_or_else(|| QueryError {
        code: "CAIRN_QUERY_UNKNOWN_TOOL".to_owned(),
        message: format!("unknown tool `{}`", request.tool),
        source_span: None,
        remediation: None,
    })?;
    if metadata.safety == SafetyClass::Mutating && !request.mutating {
        return Err(QueryError {
            code: "CAIRN_QUERY_MUTATION_NOT_ALLOWED".to_owned(),
            message: "this tool mutates state and requires the mutating flag".to_owned(),
            source_span: None,
            remediation: None,
        });
    }
    let loaded_config = config::load(root).map_err(|error| QueryError {
        code: error.code,
        message: error.message,
        source_span: Some(root.join("cairn.config.yaml").display().to_string()),
        remediation: None,
    })?;
    let mut data = execute_data_with_scan(
        root,
        blueprint_path,
        changes_dir,
        request,
        metadata,
        &loaded_config,
        scan_result,
        false,
    )?;
    debug_assert!(
        matches!(data, Value::Object(_)),
        "query data payload must be a JSON object so we can stamp schema_version"
    );
    if let Value::Object(map) = &mut data {
        map.insert("schema_version".to_owned(), json!(SCHEMA_VERSION));
    }
    let rules = relevant_rules(&loaded_config.rules, &request.tool);
    Ok(QueryResponse {
        project_context: loaded_config.context,
        rules,
        data,
        findings: Vec::new(),
    })
}

/// Runs the `lint` query and returns its raw findings without JSON
/// serialisation. Shared by consumers that need the finding set directly
/// (the LSP background rescan) so they reuse the spine's lint operation
/// instead of scanning the project independently.
///
/// # Errors
///
/// Returns a stable query error when the project fails to load.
pub(crate) fn lint_findings(
    root: &Path,
    blueprint_path: &Path,
) -> Result<Vec<Finding>, QueryError> {
    let scan_result = util::load_for("lint", root, blueprint_path)?;
    Ok(query::lint(&scan_result.graph).findings)
}

/// Converts a query response into the MCP response envelope.
#[must_use]
pub fn envelope_json(response: &QueryResponse) -> Value {
    json!({
        "project_context": response.project_context,
        "rules": response.rules,
        "data": response.data,
        "findings": findings_json(&response.findings),
    })
}

/// Converts a query error into structured JSON.
#[must_use]
pub fn error_json(error: &QueryError) -> Value {
    json!({
        "code": error.code,
        "message": error.message,
        "source_span": error.source_span,
        "remediation": error.remediation,
    })
}
fn execute_data(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    metadata: ToolMetadata,
    loaded_config: &config::Config,
) -> Result<Value, QueryError> {
    let scan_result = load_for(metadata.cli_name, root, blueprint_path)?;
    execute_data_with_scan(
        root,
        blueprint_path,
        changes_dir,
        request,
        metadata,
        loaded_config,
        &scan_result,
        true,
    )
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)] // Reason: query dispatch hub for many tools
fn execute_data_with_scan(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    metadata: ToolMetadata,
    loaded_config: &config::Config,
    scan_result: &scanner::ScanResult,
    enforce_valid_map: bool,
) -> Result<Value, QueryError> {
    if let Some(result) = dispatch_change_tool(root, blueprint_path, changes_dir, request, metadata)
    {
        return result;
    }
    if enforce_valid_map && requires_valid_map(metadata.cli_name) && scan_result.graph.has_errors()
    {
        return Err(findings_error(&scan_result.graph.findings));
    }
    match metadata.cli_name {
        "get" => {
            let id = required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?;
            query::get(&scan_result.graph, id).map_or_else(
                |finding| {
                    crate::state::backlog::find(root, id).map_or_else(
                        || Err(finding_error(finding)),
                        |item| Ok(backlog_item_detail_json(&item)),
                    )
                },
                |node| {
                    let mut value =
                        node_json(&node.node, request.flags.contains(&QueryFlag::Symbols));
                    value["decisions"] = json!(accepted_decision_ids(scan_result, &node.node.id));
                    Ok(value)
                },
            )
        }
        "ui_meta" => Ok(ui_meta_json()),
        "blueprint" => Ok(blueprint_json(blueprint_path)),
        "beads" => Ok(beads_json(
            root,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        )),
        "neighbourhood" => neighbourhood_json(root, changes_dir, scan_result, request),
        "contract" => contract_json(
            scan_result,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        ),
        "docstring" => docstring_json(scan_result, request),
        "files" => files_json(
            scan_result,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        ),
        "locate" => Ok(locate_json(
            scan_result,
            required(
                request.symbol.as_ref(),
                "CAIRN_QUERY_MISSING_SYMBOL",
                "symbol",
            )?,
        )),
        "bundle" => bundle_json(
            root,
            scan_result,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        ),
        "deps" => dependency_json(scan_result, request, !request.has(QueryFlag::Inbound)),
        "order" => query::order(&scan_result.graph)
            .map(|response| json!({ "nodes": response.nodes }))
            .map_err(|findings| findings_error(&findings)),
        "islands" => Ok(islands_json(scan_result)),
        "frontier" => frontier_json(scan_result),
        "graph" => Ok(graph_response_json(scan_result)),
        "lint" | "scan" => {
            let response = query::lint(&scan_result.graph);
            Ok(json!({
                "findings": findings_json(&response.findings),
                "strict_green": crate::map::graph::strict_green(&response.findings),
            }))
        }
        "status" => Ok(status_json(root, changes_dir, scan_result)),
        "context" => Ok(context_json(root, scan_result, loaded_config)),
        "rationale" => rationale_json(
            root,
            scan_result,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        ),
        "todos" => todos_response_json(root, scan_result, request),
        "decisions" => decisions_response_json(root, scan_result, request),
        "research" => research_response_json(
            root,
            scan_result,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        ),
        "sources" => sources_response_json(
            root,
            scan_result,
            required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?,
        ),
        "hook" => hook_json(root, changes_dir, scan_result, request),
        "health" => Ok(health_json(root, changes_dir, scan_result)),
        "remediate" => Ok(remediate_json(root, changes_dir, scan_result)),
        "draft create" => {
            let node_id = required(request.node.as_ref(), "CAIRN_QUERY_MISSING_NODE", "node")?;
            let settings =
                crate::summariser::SummariserSettings::load(root).map_err(|e| QueryError {
                    code: "CAIRN_SUMMARISER_CONFIG_ERROR".to_owned(),
                    message: e,
                    source_span: None,
                    remediation: None,
                })?;
            let backend: Box<dyn crate::summariser::SummariserBackend> = match &settings.mode {
                crate::summariser::SummariserMode::Disabled => {
                    return Err(QueryError {
                        code: "CAIRN_SUMMARISER_DISABLED".to_owned(),
                        message: "summariser is disabled in cairn.config.yaml".to_owned(),
                        source_span: None,
                        remediation: Some(
                            "set summariser.mode to local_command or hosted_api".to_owned(),
                        ),
                    });
                }
                crate::summariser::SummariserMode::LocalCommand { command, args, .. } => Box::new(
                    crate::summariser::LocalCommandBackend::new(command.clone(), args.clone()),
                ),
                crate::summariser::SummariserMode::Hosted { adapter } => {
                    let config = crate::summariser::HostedConfig {
                        adapter: adapter.clone(),
                        base_url: None,
                        timeout_ms: None,
                    };
                    Box::new(crate::summariser::HostedBackend::new(config))
                }
            };
            let prompt_request = crate::summariser::build_request(
                node_id,
                "contract",
                &format!(
                    "draft-{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos()
                ),
                &scan_result.graph,
                loaded_config,
                root,
                &scan_result.contracts,
                settings.max_prompt_bytes,
                settings.max_sample_bytes_per_file,
            )
            .map_err(|e| QueryError {
                code: "CAIRN_SUMMARISER_PROMPT_ERROR".to_owned(),
                message: e.to_string(),
                source_span: None,
                remediation: None,
            })?;
            let timeout = std::time::Duration::from_millis(settings.timeout_ms);
            let store = crate::summariser::DraftStore::new(root.join(".cairn/state/summariser"));
            let draft_id = format!(
                "draft-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            );
            let created_at = {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap();
                format!(
                    "{}T{:02}:{:02}:{:02}Z",
                    "2024-01-15",
                    (now.as_secs() / 3600) % 24,
                    (now.as_secs() / 60) % 60,
                    now.as_secs() % 60
                )
            };
            let result = crate::summariser::generate(
                backend.as_ref(),
                &prompt_request,
                timeout,
                &store,
                &draft_id,
                &created_at,
            )
            .map_err(|e| QueryError {
                code: "CAIRN_SUMMARISER_GENERATION_FAILED".to_owned(),
                message: e.to_string(),
                source_span: None,
                remediation: None,
            })?;
            Ok(json!({ "id": result, "status": "pending" }))
        }
        "watch" => {
            let events = crate::watch::diff_findings(&[], &scan_result.graph.findings);
            Ok(json!({ "events": events }))
        }
        _ => Err(QueryError {
            code: "CAIRN_QUERY_UNIMPLEMENTED_TOOL".to_owned(),
            message: format!("tool `{}` is registered but not implemented", request.tool),
            source_span: None,
            remediation: None,
        }),
    }
}

#[cfg(test)]
mod strict_green_tests;
#[cfg(test)]
mod tests;
