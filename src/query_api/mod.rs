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
mod handlers;
mod registry;
mod serialise;
mod util;

use change_queries::dispatch_change_tool;
use handlers::{
    context_json, contract_json, decisions_response_json, dependency_json, docstring_json,
    files_json, hook_json, islands_json, neighbourhood_json, rationale_json,
    research_response_json, sources_response_json, status_json, todos_response_json,
};
use registry::{metadata_for_tool, registry_slice};
use serialise::{findings_json, node_json, relevant_rules, requires_valid_map};
use util::{finding_error, findings_error, load_for, required};

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
}

/// Structured query request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryRequest {
    /// Tool or command name.
    pub tool: String,
    /// Optional node name or ID.
    pub node: Option<String>,
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
    let data = execute_data(
        root,
        blueprint_path,
        changes_dir,
        request,
        metadata,
        &loaded_config,
    )?;
    let rules = relevant_rules(&loaded_config.rules, &request.tool);
    Ok(QueryResponse {
        project_context: loaded_config.context,
        rules,
        data,
        findings: Vec::new(),
    })
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
#[allow(clippy::too_many_lines)] // Reason: query dispatch hub for many tools
fn execute_data(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    metadata: ToolMetadata,
    loaded_config: &config::Config,
) -> Result<Value, QueryError> {
    if let Some(result) = dispatch_change_tool(root, blueprint_path, changes_dir, request, metadata)
    {
        return result;
    }

    let scan_result = load_for(metadata.cli_name, root, blueprint_path)?;
    if requires_valid_map(metadata.cli_name) && scan_result.graph.has_errors() {
        return Err(findings_error(&scan_result.graph.findings));
    }
    match metadata.cli_name {
        "get" => {
            let node = query::get(&scan_result.graph, required(request.node.as_ref(), "node")?)
                .map_err(finding_error)?;
            Ok(node_json(&node.node))
        }
        "neighbourhood" => neighbourhood_json(&scan_result, request),
        "contract" => contract_json(&scan_result, required(request.node.as_ref(), "node")?),
        "docstring" => docstring_json(&scan_result, request),
        "files" => files_json(&scan_result, required(request.node.as_ref(), "node")?),
        "dependents" => dependency_json(&scan_result, request, false),
        "depends" => dependency_json(&scan_result, request, true),
        "order" => query::order(&scan_result.graph)
            .map(|response| json!({ "nodes": response.nodes }))
            .map_err(|findings| findings_error(&findings)),
        "islands" => Ok(islands_json(&scan_result)),
        "lint" | "scan" => {
            let response = query::lint(&scan_result.graph);
            Ok(json!({ "findings": findings_json(&response.findings) }))
        }
        "status" => Ok(status_json(root, &scan_result)),
        "context" => Ok(context_json(&scan_result, loaded_config)),
        "rationale" => rationale_json(&scan_result, required(request.node.as_ref(), "node")?),
        "todos" => todos_response_json(&scan_result, request),
        "decisions" => decisions_response_json(&scan_result, request),
        "research" => {
            research_response_json(&scan_result, required(request.node.as_ref(), "node")?)
        }
        "sources" => sources_response_json(&scan_result, required(request.node.as_ref(), "node")?),
        "hook" => hook_json(root, changes_dir, &scan_result, request),
        "summarise" => {
            let node_id = required(request.node.as_ref(), "node")?;
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
        _ => Err(QueryError {
            code: "CAIRN_QUERY_UNIMPLEMENTED_TOOL".to_owned(),
            message: format!("tool `{}` is registered but not implemented", request.tool),
            source_span: None,
            remediation: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_lists_known_tools() {
        let tools = registry();
        assert!(!tools.is_empty());
        assert!(tools.iter().any(|t| t.cli_name == "get"));
        assert!(tools.iter().any(|t| t.cli_name == "scan"));
    }

    #[test]
    fn test_visible_tools_filter_mutating_entries() {
        let visible = visible_tools(false);
        assert!(visible.iter().all(|t| t.safety == SafetyClass::ReadOnly));
    }

    #[test]
    fn test_envelope_json_wraps_response() {
        let response = QueryResponse {
            project_context: "ctx".to_owned(),
            rules: BTreeMap::new(),
            data: json!({"key": "value"}),
            findings: Vec::new(),
        };
        let envelope = envelope_json(&response);
        assert_eq!(
            envelope.get("project_context").unwrap().as_str().unwrap(),
            "ctx"
        );
        assert!(envelope.get("data").is_some());
    }

    #[test]
    fn test_error_json_includes_optional_fields() {
        let error = QueryError {
            code: "TEST".to_owned(),
            message: "msg".to_owned(),
            source_span: Some("span".to_owned()),
            remediation: Some("fix".to_owned()),
        };
        let json = error_json(&error);
        assert_eq!(json.get("code").unwrap().as_str().unwrap(), "TEST");
        assert_eq!(json.get("source_span").unwrap().as_str().unwrap(), "span");
        assert_eq!(json.get("remediation").unwrap().as_str().unwrap(), "fix");
    }

    #[test]
    fn test_execute_rejects_unknown_or_invalid_requests() {
        let request = QueryRequest {
            tool: "nonexistent".to_owned(),
            ..QueryRequest::default()
        };
        let result = execute(
            Path::new("."),
            Path::new("cairn.blueprint"),
            Path::new("meta/changes"),
            &request,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_returns_node_json_for_valid_request() {
        let tmp = std::env::temp_dir().join(format!("cairn-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(
            tmp.join("cairn.blueprint"),
            "System Test \"T\" id \"t\" {\n}\n",
        );
        let request = QueryRequest {
            tool: "status".to_owned(),
            ..QueryRequest::default()
        };
        let result = execute(
            &tmp,
            &tmp.join("cairn.blueprint"),
            &tmp.join("meta/changes"),
            &request,
        );
        assert!(
            result.is_ok(),
            "execute must succeed for valid request: {result:?}"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
