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
    contract_json, decisions_response_json, dependency_json, docstring_json, files_json, hook_json,
    neighbourhood_json, rationale_json, research_response_json, sources_response_json, status_json,
    todos_response_json,
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
    let data = execute_data(root, blueprint_path, changes_dir, request, metadata)?;
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
fn execute_data(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    metadata: ToolMetadata,
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
        "lint" | "scan" => {
            let response = query::lint(&scan_result.graph);
            Ok(json!({ "findings": findings_json(&response.findings) }))
        }
        "status" => Ok(status_json(root, &scan_result)),
        "rationale" => rationale_json(&scan_result, required(request.node.as_ref(), "node")?),
        "todos" => todos_response_json(&scan_result, request),
        "decisions" => decisions_response_json(&scan_result, request),
        "research" => {
            research_response_json(&scan_result, required(request.node.as_ref(), "node")?)
        }
        "sources" => sources_response_json(&scan_result, required(request.node.as_ref(), "node")?),
        "hook" => hook_json(root, changes_dir, &scan_result, request),
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
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn test_registry_lists_known_tools() {
        let names = registry()
            .iter()
            .map(|tool| tool.cli_name)
            .collect::<Vec<_>>();
        assert!(names.contains(&"get"));
        assert!(names.contains(&"archive"));
        assert!(!names.contains(&"missing"));
    }

    #[test]
    fn test_visible_tools_filter_mutating_entries() {
        let readonly = visible_tools(false);
        let all = visible_tools(true);

        assert!(
            readonly
                .iter()
                .all(|tool| tool.safety == SafetyClass::ReadOnly)
        );
        assert!(all.iter().any(|tool| tool.cli_name == "scan"));
        assert!(!readonly.iter().any(|tool| tool.cli_name == "scan"));
    }

    #[test]
    fn test_execute_returns_node_json_for_valid_request() -> Result<(), Box<dyn Error>> {
        let root = temp_root("execute-ok")?;
        write_project(&root)?;
        let response = execute(
            &root,
            &root.join("cairn.blueprint"),
            &root.join("meta/changes"),
            &QueryRequest {
                tool: "get".to_owned(),
                node: Some("app.api".to_owned()),
                ..QueryRequest::default()
            },
        )?;

        assert_eq!(response.data["id"], "app.api");
        assert_eq!(response.data["name"], "Api");

        Ok(())
    }

    #[test]
    fn test_execute_rejects_unknown_or_invalid_requests() -> Result<(), Box<dyn Error>> {
        let root = temp_root("execute-error")?;
        write_project(&root)?;

        let missing = execute(
            &root,
            &root.join("cairn.blueprint"),
            &root.join("meta/changes"),
            &QueryRequest {
                tool: "get".to_owned(),
                ..QueryRequest::default()
            },
        )
        .expect_err("missing node should fail");
        assert_eq!(missing.code, "CAIRN_QUERY_MISSING_NODE");

        let unknown = execute(
            &root,
            &root.join("cairn.blueprint"),
            &root.join("meta/changes"),
            &QueryRequest {
                tool: "missing".to_owned(),
                ..QueryRequest::default()
            },
        )
        .expect_err("unknown tool should fail");
        assert_eq!(unknown.code, "CAIRN_QUERY_UNKNOWN_TOOL");

        Ok(())
    }

    #[test]
    fn test_envelope_json_wraps_response() {
        let response = QueryResponse {
            project_context: "ctx".to_owned(),
            rules: BTreeMap::from([("decision".to_owned(), "keep".to_owned())]),
            data: json!({ "ok": true }),
            findings: Vec::new(),
        };

        let json = envelope_json(&response);

        assert_eq!(json["project_context"], "ctx");
        assert_eq!(json["data"]["ok"], true);
        assert_eq!(json["rules"]["decision"], "keep");
    }

    #[test]
    fn test_error_json_includes_optional_fields() {
        let json = error_json(&QueryError {
            code: "CAIRN_QUERY_FAILED".to_owned(),
            message: "failed".to_owned(),
            source_span: Some("cairn.blueprint:1".to_owned()),
            remediation: Some("fix it".to_owned()),
        });

        assert_eq!(json["code"], "CAIRN_QUERY_FAILED");
        assert_eq!(json["source_span"], "cairn.blueprint:1");
        assert_eq!(json["remediation"], "fix it");
    }

    fn write_project(root: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(root.join("src/api"))?;
        fs::create_dir_all(root.join("meta/contracts"))?;
        fs::create_dir_all(root.join("meta/changes"))?;
        fs::write(root.join("src/api/lib.rs"), "pub fn serve() {}\n")?;
        fs::write(
            root.join("cairn.blueprint"),
            r#"System App "desc" id "app" {
    Container Api "desc" id "app.api" {
        path "./src/api"
        contract "./meta/contracts/api.md"
    }
}
"#,
        )?;
        fs::write(
            root.join("cairn.config.yaml"),
            "reconcilers:\n  - id: rust-code\n    version: phase-1\n    config:\n      ignore:\n        - target\ncontext: \"ctx\"\nrules:\n  contract: keep\n",
        )?;
        fs::write(
            root.join("meta/contracts/api.md"),
            "---\nnode: app.api\n---\n# API Contract\n",
        )?;
        Ok(())
    }

    fn temp_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
        let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("cairn-query-api-{name}-{suffix}"));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
