// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split
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
        graph::{Finding, NodeRecord},
        query,
    },
    scanner::{self, config},
};

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
    &TOOL_REGISTRY
}

/// Returns tools visible for a server configuration.
#[must_use]
pub fn visible_tools(allow_mutating: bool) -> Vec<ToolMetadata> {
    TOOL_REGISTRY
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

const TOOL_REGISTRY: [ToolMetadata; 23] = [
    tool(
        "get",
        "cairn_get",
        "NodeRequest",
        "NodeResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "neighbourhood",
        "cairn_neighbourhood",
        "NeighbourhoodRequest",
        "NeighbourhoodResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "contract",
        "cairn_contract",
        "NodeRequest",
        "ContractResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "docstring",
        "cairn_docstring",
        "DocstringRequest",
        "DocstringResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "files",
        "cairn_files",
        "NodeRequest",
        "FilesResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "dependents",
        "cairn_dependents",
        "DependencyRequest",
        "DependencyResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "depends",
        "cairn_depends",
        "DependencyRequest",
        "DependencyResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "order",
        "cairn_order",
        "OrderRequest",
        "OrderResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "lint",
        "cairn_lint",
        "LintRequest",
        "LintResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "status",
        "cairn_status",
        "StatusRequest",
        "StatusResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "rationale",
        "cairn_rationale",
        "NodeRequest",
        "RationaleResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "todos",
        "cairn_todos",
        "ArtefactNodeRequest",
        "TodosResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "decisions",
        "cairn_decisions",
        "ArtefactNodeRequest",
        "DecisionsResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "research",
        "cairn_research",
        "NodeRequest",
        "ResearchResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "sources",
        "cairn_sources",
        "NodeRequest",
        "SourcesResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "changes",
        "cairn_changes",
        "ChangesRequest",
        "ChangesResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "show",
        "cairn_show_change",
        "ShowChangeRequest",
        "ShowChangeResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "hook",
        "cairn_hook",
        "HookRequest",
        "HookReport",
        SafetyClass::ReadOnly,
    ),
    tool(
        "ui",
        "cairn_ui",
        "UiRequest",
        "UiServerResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "scan",
        "cairn_scan",
        "ScanRequest",
        "ScanResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "archive",
        "cairn_archive",
        "ArchiveRequest",
        "ArchiveResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "rename",
        "cairn_rename",
        "RenameRequest",
        "RenameResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "init",
        "cairn_init",
        "InitRequest",
        "InitResponse",
        SafetyClass::Mutating,
    ),
];

const fn tool(
    cli_name: &'static str,
    mcp_name: &'static str,
    request_schema: &'static str,
    response_schema: &'static str,
    safety: SafetyClass,
) -> ToolMetadata {
    ToolMetadata {
        cli_name,
        mcp_name,
        request_schema,
        response_schema,
        safety,
    }
}

fn metadata_for_tool(name: &str) -> Option<ToolMetadata> {
    TOOL_REGISTRY
        .iter()
        .copied()
        .find(|tool| tool.cli_name == name || tool.mcp_name == name)
}

fn execute_data(
    root: &Path,
    blueprint_path: &Path,
    changes_dir: &Path,
    request: &QueryRequest,
    metadata: ToolMetadata,
) -> Result<Value, QueryError> {
    if metadata.cli_name == "archive" {
        let change = required(request.change.as_ref(), "change")?;
        let conflict_findings = hooks::detect_active_change_conflicts(changes_dir);
        if !conflict_findings.is_empty() {
            return Err(findings_error(&conflict_findings));
        }
        let report = changes::archive(root, blueprint_path, change).map_err(command_error)?;
        return Ok(json!({
            "archive_path": report.archive_path.to_string_lossy(),
            "summary": report.summary,
        }));
    }
    if metadata.cli_name == "rename" {
        let old_id = required(request.old_id.as_ref(), "old_id")?;
        let new_id = required(request.new_id.as_ref(), "new_id")?;
        let change = changes::create_rename_change(root, blueprint_path, old_id, new_id)
            .map_err(command_error)?;
        return Ok(change_json(&change));
    }
    if metadata.cli_name == "changes" {
        return discover_changes(root);
    }
    if metadata.cli_name == "show" {
        return show_change(root, request.change.as_ref());
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

fn load_for(
    command: &str,
    root: &Path,
    blueprint_path: &Path,
) -> Result<scanner::ScanResult, QueryError> {
    let result = if command == "scan" {
        scanner::scan(root, blueprint_path)
    } else {
        scanner::load_project(root, blueprint_path)
    };
    result.map_err(|message| QueryError {
        code: "CAIRN_COMMAND_FAILED".to_owned(),
        message,
        source_span: Some(blueprint_path.display().to_string()),
        remediation: None,
    })
}

fn required<'a>(value: Option<&'a String>, name: &str) -> Result<&'a str, QueryError> {
    value.map(String::as_str).ok_or_else(|| QueryError {
        code: format!("CAIRN_QUERY_MISSING_{}", name.to_ascii_uppercase()),
        message: format!("`{name}` is required"),
        source_span: None,
        remediation: None,
    })
}

fn discover_changes(root: &Path) -> Result<Value, QueryError> {
    let changes = changes::discover(root).map_err(|error| QueryError {
        code: "CAIRN_CHANGES_DISCOVERY_FAILED".to_owned(),
        message: error.to_string(),
        source_span: Some(root.join("meta/changes").display().to_string()),
        remediation: None,
    })?;
    Ok(json!({ "changes": changes.iter().map(change_json).collect::<Vec<_>>() }))
}

fn show_change(root: &Path, change: Option<&String>) -> Result<Value, QueryError> {
    let change_id = required(change, "change")?;
    let changes = changes::discover(root).map_err(|error| QueryError {
        code: "CAIRN_CHANGES_DISCOVERY_FAILED".to_owned(),
        message: error.to_string(),
        source_span: Some(root.join("meta/changes").display().to_string()),
        remediation: None,
    })?;
    changes
        .iter()
        .find(|candidate| candidate.id == change_id)
        .map(change_json)
        .ok_or_else(|| QueryError {
            code: "CAIRN_CHANGE_NOT_FOUND".to_owned(),
            message: format!("change `{change_id}` was not found"),
            source_span: Some(root.join("meta/changes").display().to_string()),
            remediation: None,
        })
}

fn change_json(change: &changes::Change) -> Value {
    json!({
        "id": change.id,
        "path": change.path.to_string_lossy(),
        "title": change.title,
        "proposal": change.proposal,
        "design": change.design,
        "summary": changes::operation_summary(change),
        "findings": change.findings,
        "delta": {
            "added_nodes": change.delta.added_nodes.len(),
            "modified_nodes": change.delta.modified_nodes.len(),
            "removed_nodes": change.delta.removed_nodes,
            "renamed_nodes": change.delta.renamed_nodes.iter().map(|rename| {
                json!({ "from": rename.from, "to": rename.to })
            }).collect::<Vec<_>>(),
            "added_edges": change.delta.added_edges.len(),
            "modified_edges": change.delta.modified_edges.len(),
            "removed_edges": change.delta.removed_edges.len(),
            "renamed_edges": change.delta.renamed_edges.len(),
        },
        "artefacts": change.artefacts.iter().map(|operation| {
            json!({
                "operation": format!("{:?}", operation.operation),
                "change_path": operation.change_path.to_string_lossy(),
                "target_path": operation.target_path.to_string_lossy(),
                "renamed_from": operation.renamed_from.as_ref().map(|path| path.to_string_lossy().to_string()),
            })
        }).collect::<Vec<_>>(),
    })
}

fn neighbourhood_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let response =
        query::neighbourhood(&scan_result.graph, required(request.node.as_ref(), "node")?)
            .map_err(finding_error)?;
    let node_ids = neighbourhood_ids(&scan_result.graph, &response.node.id);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.nodes.iter().any(|node| node_ids.contains(node))
                && (decision.status == DecisionStatus::Accepted
                    || request.has(QueryFlag::IncludeDeprecatedDecisions))
        })
        .cloned()
        .collect::<Vec<_>>();
    let todos = if request.has(QueryFlag::IncludeTodos) {
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
    let research = if request.has(QueryFlag::IncludeResearch) {
        research_for_nodes(scan_result, &node_ids)
    } else {
        Vec::new()
    };
    let reviews = if request.has(QueryFlag::IncludeReviews) {
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
    let mut data = json!({
        "node": node_json(&response.node),
        "inbound": response.inbound,
        "outbound": response.outbound,
        "contracts": response.node.contracts,
        "decisions": decisions.iter().map(decision_json).collect::<Vec<_>>(),
        "todos": todos.iter().map(todo_json).collect::<Vec<_>>(),
        "research": research.iter().map(research_json).collect::<Vec<_>>(),
        "reviews": reviews.iter().map(review_json).collect::<Vec<_>>(),
    });
    if request.has(QueryFlag::IncludeChanges) {
        data["active_changes"] = json!([]);
    }
    Ok(data)
}

fn contract_json(scan_result: &scanner::ScanResult, node: &str) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let contracts = node
        .contracts
        .iter()
        .filter_map(|path| scan_result.contracts.contracts.get(path))
        .filter(|contract| contract.node == node.id)
        .map(single_contract_json)
        .collect::<Vec<_>>();
    let body = contracts
        .first()
        .and_then(|contract| contract.get("body"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    Ok(json!({ "node": node.id, "contract": body, "contracts": contracts }))
}

fn single_contract_json(contract: &Contract) -> Value {
    json!({
        "path": contract.path,
        "node": contract.node,
        "declared_by": contract.declared_by,
        "body": contract.body,
    })
}

fn docstring_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(request.node.as_ref(), "node")?)
        .map_err(finding_error)?;
    let language = request.language.as_deref().unwrap_or("rust");
    let depends = query::depends(&scan_result.graph, &node.id, false)
        .map_err(finding_error)?
        .nodes;
    let prefix = match language {
        "python" => "#",
        "typescript" | "go" => "//",
        _ => "//!",
    };
    let lines = [
        format!("{prefix} {}", node.name),
        prefix.to_string(),
        format!("{prefix} Cairn-ID: {}", node.id),
        format!("{prefix} Cairn-Description: {}", node.description),
        format!("{prefix} Cairn-Depends: {}", depends.join(", ")),
        format!("{prefix} Cairn-Tags: {}", node.tags.join(", ")),
    ];
    Ok(json!({
        "node": node.id,
        "language": language,
        "docstring": lines.join("\n"),
    }))
}

fn files_json(scan_result: &scanner::ScanResult, node: &str) -> Result<Value, QueryError> {
    let node_record = scan_result.graph.resolve(node).map_err(finding_error)?;
    let targets = scan_result
        .target_reports
        .iter()
        .filter(|report| report.target_id.node_id == node_record.id)
        .map(|report| {
            json!({
                "path": report.target_id.path.to_string_lossy(),
                "language": report.language.as_str(),
                "reconciler_id": report.reconciler_id.0,
                "files": report.claimed_files,
                "hash": report.hash,
            })
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node_record.id,
        "files": node_record.files,
        "targets": targets,
    }))
}

fn dependency_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
    outbound: bool,
) -> Result<Value, QueryError> {
    let node = required(request.node.as_ref(), "node")?;
    let response = if outbound {
        query::depends(&scan_result.graph, node, request.has(QueryFlag::Transitive))
    } else {
        query::dependents(&scan_result.graph, node, request.has(QueryFlag::Transitive))
    }
    .map_err(finding_error)?;
    Ok(json!({ "node": response.node, "nodes": response.nodes }))
}

fn status_json(root: &Path, scan_result: &scanner::ScanResult) -> Value {
    let open = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open || todo.status == TodoStatus::InProgress)
        .map(todo_json)
        .collect::<Vec<_>>();
    let log_entries: Vec<String> = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            content
                .lines()
                .rev()
                .take(5)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();
    json!({
        "active_changes": [],
        "open_todos": open,
        "recent_log_entries": log_entries,
    })
}

fn rationale_json(scan_result: &scanner::ScanResult, node: &str) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let node_ids = neighbourhood_ids(&scan_result.graph, &node.id);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.status == DecisionStatus::Accepted
                && decision.nodes.iter().any(|id| node_ids.contains(id))
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
        .map(research_json)
        .collect::<Vec<_>>();
    let sources = scan_result
        .artefacts
        .sources
        .iter()
        .filter(|source| source_ids.contains(&source.id))
        .map(source_json)
        .collect::<Vec<_>>();
    Ok(json!({
        "node": node.id,
        "decisions": decisions.iter().map(decision_json).collect::<Vec<_>>(),
        "research": research,
        "sources": sources,
    }))
}

fn todos_response_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(request.node.as_ref(), "node")?)
        .map_err(finding_error)?;
    let status = request.status.as_deref().and_then(parse_todo_status_filter);
    let todos = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.node == node.id && status.is_none_or(|filter| todo.status == filter))
        .map(todo_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "todos": todos }))
}

fn decisions_response_json(
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let node = scan_result
        .graph
        .resolve(required(request.node.as_ref(), "node")?)
        .map_err(finding_error)?;
    let status = request
        .status
        .as_deref()
        .and_then(parse_decision_status_filter);
    let decisions = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            decision.nodes.contains(&node.id)
                && status.is_none_or(|filter| decision.status == filter)
        })
        .map(decision_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "decisions": decisions }))
}

fn research_response_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(research_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "research": research }))
}

fn sources_response_json(
    scan_result: &scanner::ScanResult,
    node: &str,
) -> Result<Value, QueryError> {
    let node = scan_result.graph.resolve(node).map_err(finding_error)?;
    let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]))
        .iter()
        .map(source_json)
        .collect::<Vec<_>>();
    Ok(json!({ "node": node.id, "sources": sources }))
}

fn hook_json(
    root: &Path,
    changes_dir: &Path,
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let kind = match request.status.as_deref().unwrap_or("all") {
        "structural" => HookKind::Structural,
        "interface" => HookKind::Interface,
        "tension" => HookKind::Tension,
        "all" => HookKind::All,
        other => {
            return Err(QueryError {
                code: "CAIRN_QUERY_INVALID_HOOK_KIND".to_owned(),
                message: format!("unknown hook kind `{other}`"),
                source_span: None,
                remediation: Some("Use structural, interface, tension, or all.".to_owned()),
            });
        }
    };
    let report = crate::hooks::run(kind, root, changes_dir, scan_result);
    Ok(json!({
        "kind": hook_kind_name(report.kind),
        "decision": hook_decision_name(report.decision),
        "findings": findings_json(&report.findings),
        "exit_code": report.exit_code(),
    }))
}

fn node_json(node: &NodeRecord) -> Value {
    json!({
        "id": node.id,
        "kind": format!("{:?}", node.kind),
        "name": node.name,
        "description": node.description,
        "tags": node.tags,
        "parent": node.parent,
        "children": node.children,
        "paths": node.paths,
        "owns_files": node.owns_files,
        "contracts": node.contracts,
        "state": format!("{:?}", node.state),
        "files": node.files,
        "span": {
            "file": node.span.file,
            "line": node.span.line,
            "column": node.span.column,
            "end_line": node.span.end_line,
            "end_column": node.span.end_column,
        },
    })
}

fn todo_json(todo: &Todo) -> Value {
    json!({
        "path": todo.path,
        "node": todo.node,
        "status": todo_status(todo.status),
        "created": todo.created,
        "satisfies": todo.satisfies,
    })
}

fn decision_json(decision: &Decision) -> Value {
    json!({
        "id": decision.id,
        "status": decision_status(decision.status),
        "nodes": decision.nodes,
        "informed_by": decision.informed_by,
        "supersedes": decision.supersedes,
        "refines": decision.refines,
        "related": decision.related,
    })
}

fn research_json(research: &Research) -> Value {
    json!({
        "id": research.id,
        "nodes": research.nodes,
        "sources": research.sources,
        "date": research.date,
    })
}

fn review_json(review: &Review) -> Value {
    json!({
        "path": review.path,
        "node": review.node,
        "review_type": format!("{:?}", review.review_type),
        "date": review.date,
        "reviewer": review.reviewer,
    })
}

fn source_json(source: &Source) -> Value {
    json!({
        "id": source.id,
        "file": source.file,
        "verification": source_verification(source.verification),
        "type": source.source_type,
        "date": source.date,
    })
}

fn findings_json(findings: &[Finding]) -> Vec<Value> {
    findings
        .iter()
        .map(|finding| {
            json!({
                "code": finding.code,
                "severity": format!("{:?}", finding.severity),
                "message": finding.message,
                "node": finding.node,
                "path": finding.path,
            })
        })
        .collect()
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

fn relevant_rules(rules: &BTreeMap<String, String>, tool: &str) -> BTreeMap<String, String> {
    let key = match tool.strip_prefix("cairn_").unwrap_or(tool) {
        "todos" => Some("todo"),
        "decisions" | "rationale" => Some("decision"),
        "research" => Some("research"),
        "sources" => Some("source"),
        "contract" => Some("contract"),
        "show_change" | "changes" => Some("change"),
        _ => None,
    };
    key.and_then(|key| rules.get(key).map(|value| (key.to_owned(), value.clone())))
        .into_iter()
        .collect()
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
            | "docstring"
            | "order"
            | "todos"
            | "decisions"
            | "research"
            | "sources"
            | "rationale"
            | "status"
    )
}

fn findings_error(findings: &[Finding]) -> QueryError {
    let message = findings
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ");
    QueryError {
        code: findings.first().map_or_else(
            || "CAIRN_QUERY_FINDINGS".to_owned(),
            |finding| finding.code.clone(),
        ),
        message,
        source_span: findings.first().and_then(|finding| finding.path.clone()),
        remediation: None,
    }
}

fn finding_error(finding: Finding) -> QueryError {
    QueryError {
        code: finding.code,
        message: finding.message,
        source_span: finding.path,
        remediation: None,
    }
}

fn command_error(message: String) -> QueryError {
    QueryError {
        code: "CAIRN_COMMAND_FAILED".to_owned(),
        message,
        source_span: None,
        remediation: None,
    }
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

const fn source_verification(verification: SourceVerification) -> &'static str {
    match verification {
        SourceVerification::Verified => "verified",
        SourceVerification::External => "external",
        SourceVerification::Unverified => "unverified",
    }
}

const fn hook_kind_name(kind: HookKind) -> &'static str {
    match kind {
        HookKind::Structural => "structural",
        HookKind::Interface => "interface",
        HookKind::Tension => "tension",
        HookKind::All => "all",
    }
}

const fn hook_decision_name(decision: ExitDecision) -> &'static str {
    match decision {
        ExitDecision::Pass => "pass",
        ExitDecision::Block => "block",
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
