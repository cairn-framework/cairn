//! MCP query tool registry.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) const TOOL_REGISTRY: [ToolMetadata; 41] = [
    tool(
        "get",
        "cairn_get",
        "NodeRequest",
        "NodeResponse",
        SafetyClass::ReadOnly,
        "Inspect a node by ID",
    ),
    tool(
        "neighbourhood",
        "cairn_neighbourhood",
        "NeighbourhoodRequest",
        "NeighbourhoodResponse",
        SafetyClass::ReadOnly,
        "Show a node and its neighbours",
    ),
    tool(
        "contract",
        "cairn_contract",
        "NodeRequest",
        "ContractResponse",
        SafetyClass::ReadOnly,
        "Show the contract for a node",
    ),
    tool(
        "docstring",
        "cairn_docstring",
        "DocstringRequest",
        "DocstringResponse",
        SafetyClass::ReadOnly,
        "Generate a docstring for a node",
    ),
    tool(
        "files",
        "cairn_files",
        "NodeRequest",
        "FilesResponse",
        SafetyClass::ReadOnly,
        "List files owned by a node",
    ),
    tool(
        "bundle",
        "cairn_bundle",
        "NodeRequest",
        "BundleResponse",
        SafetyClass::ReadOnly,
        "Generate bundle: contract, decisions, dependency interfaces, and gates for a node",
    ),
    tool(
        "deps",
        "cairn_dependents",
        "DependencyRequest",
        "DependencyResponse",
        SafetyClass::ReadOnly,
        "List nodes a given node depends on (out) or that depend on it (in)",
    ),
    tool(
        "deps",
        "cairn_depends",
        "DependencyRequest",
        "DependencyResponse",
        SafetyClass::ReadOnly,
        "List nodes a given node depends on (out) or that depend on it (in)",
    ),
    tool(
        "order",
        "cairn_order",
        "OrderRequest",
        "OrderResponse",
        SafetyClass::ReadOnly,
        "Topological order of all nodes",
    ),
    tool(
        "islands",
        "cairn_islands",
        "IslandsRequest",
        "IslandsResponse",
        SafetyClass::ReadOnly,
        "Show connected components of the map graph",
    ),
    tool(
        "frontier",
        "cairn_frontier",
        "FrontierRequest",
        "FrontierResponse",
        SafetyClass::ReadOnly,
        "Show buildable-now and blocked ghost nodes",
    ),
    tool(
        "lint",
        "cairn_lint",
        "LintRequest",
        "LintResponse",
        SafetyClass::ReadOnly,
        "Lint the blueprint and report findings",
    ),
    tool(
        "status",
        "cairn_status",
        "StatusRequest",
        "StatusResponse",
        SafetyClass::ReadOnly,
        "Show project status summary",
    ),
    tool(
        "rationale",
        "cairn_rationale",
        "NodeRequest",
        "RationaleResponse",
        SafetyClass::ReadOnly,
        "Show rationale chain for a node",
    ),
    tool(
        "todos",
        "cairn_todos",
        "ArtefactNodeRequest",
        "TodosResponse",
        SafetyClass::ReadOnly,
        "List todos linked to a node",
    ),
    tool(
        "decisions",
        "cairn_decisions",
        "ArtefactNodeRequest",
        "DecisionsResponse",
        SafetyClass::ReadOnly,
        "List decisions linked to a node",
    ),
    tool(
        "research",
        "cairn_research",
        "NodeRequest",
        "ResearchResponse",
        SafetyClass::ReadOnly,
        "List research linked to a node",
    ),
    tool(
        "sources",
        "cairn_sources",
        "NodeRequest",
        "SourcesResponse",
        SafetyClass::ReadOnly,
        "List sources linked to a node",
    ),
    tool(
        "changes",
        "cairn_changes",
        "ChangesRequest",
        "ChangesResponse",
        SafetyClass::ReadOnly,
        "List active and archived changes",
    ),
    tool(
        "show",
        "cairn_show_change",
        "ShowChangeRequest",
        "ShowChangeResponse",
        SafetyClass::ReadOnly,
        "Show details of a change",
    ),
    tool(
        "hook",
        "cairn_hook",
        "HookRequest",
        "HookReport",
        SafetyClass::ReadOnly,
        "Run reconciliation hooks",
    ),
    tool(
        "health",
        "cairn_health",
        "HealthRequest",
        "HealthResponse",
        SafetyClass::ReadOnly,
        "Comprehensive health check: lint, hooks, and module state",
    ),
    tool(
        "remediate",
        "cairn_remediate",
        "RemediateRequest",
        "RemediateResponse",
        SafetyClass::ReadOnly,
        "Generate an ordered action plan from current findings",
    ),
    tool(
        "ui",
        "cairn_ui",
        "UiRequest",
        "UiServerResponse",
        SafetyClass::ReadOnly,
        "Launch the web UI",
    ),
    tool(
        "scan",
        "cairn_scan",
        "ScanRequest",
        "ScanResponse",
        SafetyClass::Mutating,
        "Scan the project and report findings",
    ),
    tool(
        "archive",
        "cairn_archive",
        "ArchiveRequest",
        "ArchiveResponse",
        SafetyClass::Mutating,
        "Archive a completed change",
    ),
    tool(
        "rename",
        "cairn_rename",
        "RenameRequest",
        "RenameResponse",
        SafetyClass::Mutating,
        "Rename a node ID across the project",
    ),
    tool(
        "init",
        "cairn_init",
        "InitRequest",
        "InitResponse",
        SafetyClass::Mutating,
        "Scaffold a new cairn project",
    ),
    tool(
        "context",
        "cairn_context",
        "ContextRequest",
        "ContextResponse",
        SafetyClass::ReadOnly,
        "Structured project overview for agents",
    ),
    tool(
        "init_from_code",
        "cairn_init_from_code",
        "InitFromCodeRequest",
        "InitFromCodeResponse",
        SafetyClass::Mutating,
        "Scaffold a project from existing code",
    ),
    tool(
        "refine",
        "cairn_refine",
        "RefineRequest",
        "RefineResponse",
        SafetyClass::Mutating,
        "Re-run brownfield discovery and write a timestamped change",
    ),
    tool(
        "draft list",
        "cairn_drafts",
        "DraftsRequest",
        "DraftsResponse",
        SafetyClass::ReadOnly,
        "List pending draft proposals",
    ),
    tool(
        "draft show",
        "cairn_draft_show",
        "DraftShowRequest",
        "DraftShowResponse",
        SafetyClass::ReadOnly,
        "Show a draft proposal",
    ),
    tool(
        "draft discard",
        "cairn_draft_discard",
        "DraftDiscardRequest",
        "DraftDiscardResponse",
        SafetyClass::Mutating,
        "Discard a draft proposal",
    ),
    tool(
        "draft edit",
        "cairn_draft_edit",
        "DraftEditRequest",
        "DraftEditResponse",
        SafetyClass::Mutating,
        "Edit a draft proposal in your editor",
    ),
    tool(
        "draft accept",
        "cairn_draft_accept",
        "DraftAcceptRequest",
        "DraftAcceptResponse",
        SafetyClass::Mutating,
        "Accept a draft proposal and apply it",
    ),
    tool(
        "draft create",
        "cairn_summarise",
        "SummariseRequest",
        "SummariseResponse",
        SafetyClass::Mutating,
        "Generate a contract summary for a node",
    ),
    tool(
        "watch",
        "cairn_watch",
        "WatchRequest",
        "WatchResponse",
        SafetyClass::ReadOnly,
        "Watch for finding changes and emit events",
    ),
    tool(
        "ui_meta",
        "cairn_ui_meta",
        "UiMetaRequest",
        "UiMetaResponse",
        SafetyClass::ReadOnly,
        "List available query commands and their request/response schemas",
    ),
    tool(
        "blueprint",
        "cairn_blueprint",
        "BlueprintRequest",
        "BlueprintResponse",
        SafetyClass::ReadOnly,
        "Show the raw blueprint file",
    ),
    tool(
        "beads",
        "cairn_beads",
        "NodeRequest",
        "BeadsResponse",
        SafetyClass::ReadOnly,
        "List backlog beads linked to a node",
    ),
];

pub(super) const fn tool(
    cli_name: &'static str,
    mcp_name: &'static str,
    request_schema: &'static str,
    response_schema: &'static str,
    safety: SafetyClass,
    description: &'static str,
) -> ToolMetadata {
    ToolMetadata {
        cli_name,
        mcp_name,
        request_schema,
        response_schema,
        safety,
        description,
    }
}

pub(super) fn metadata_for_tool(name: &str) -> Option<ToolMetadata> {
    TOOL_REGISTRY
        .iter()
        .copied()
        .find(|tool| tool.cli_name == name || tool.mcp_name == name)
}

pub(super) const fn registry_slice() -> &'static [ToolMetadata] {
    &TOOL_REGISTRY
}

#[allow(dead_code)] // Reason: used by tests; will be used by CLI/MCP wiring in task 4.1
pub(super) fn is_readonly(name: &str) -> bool {
    metadata_for_tool(name).is_some_and(|m| matches!(m.safety, SafetyClass::ReadOnly))
}

#[allow(dead_code)] // Reason: used by tests; will be used by CLI/MCP wiring in task 4.1
pub(super) fn is_mutating(name: &str) -> bool {
    metadata_for_tool(name).is_some_and(|m| matches!(m.safety, SafetyClass::Mutating))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_contains_draft_list() {
        assert!(metadata_for_tool("draft list").is_some());
        assert!(metadata_for_tool("cairn_drafts").is_some());
    }

    #[test]
    fn test_registry_contains_draft_show() {
        assert!(metadata_for_tool("draft show").is_some());
        assert!(metadata_for_tool("cairn_draft_show").is_some());
    }

    #[test]
    fn test_draft_list_is_readonly() {
        assert!(is_readonly("draft list"));
        assert!(!is_mutating("draft list"));
    }

    #[test]
    fn test_draft_show_is_readonly() {
        assert!(is_readonly("draft show"));
        assert!(!is_mutating("draft show"));
    }

    #[test]
    fn test_registry_contains_draft_discard() {
        assert!(metadata_for_tool("draft discard").is_some());
        assert!(metadata_for_tool("cairn_draft_discard").is_some());
    }

    #[test]
    fn test_draft_discard_is_mutating() {
        assert!(!is_readonly("draft discard"));
        assert!(is_mutating("draft discard"));
    }

    #[test]
    fn test_registry_contains_draft_edit() {
        assert!(metadata_for_tool("draft edit").is_some());
        assert!(metadata_for_tool("cairn_draft_edit").is_some());
    }

    #[test]
    fn test_draft_edit_is_mutating() {
        assert!(!is_readonly("draft edit"));
        assert!(is_mutating("draft edit"));
    }

    #[test]
    fn test_registry_contains_draft_accept() {
        assert!(metadata_for_tool("draft accept").is_some());
        assert!(metadata_for_tool("cairn_draft_accept").is_some());
    }

    #[test]
    fn test_draft_accept_is_mutating() {
        assert!(!is_readonly("draft accept"));
        assert!(is_mutating("draft accept"));
    }

    #[test]
    fn test_registry_contains_draft_create() {
        assert!(metadata_for_tool("draft create").is_some());
        assert!(metadata_for_tool("cairn_summarise").is_some());
    }

    #[test]
    fn test_draft_create_is_mutating() {
        assert!(!is_readonly("draft create"));
        assert!(is_mutating("draft create"));
    }

    #[test]
    fn test_registry_size() {
        assert_eq!(TOOL_REGISTRY.len(), 41);
    }

    #[test]
    fn test_every_registry_tool_has_a_description() {
        for tool in &TOOL_REGISTRY {
            assert!(
                !tool.description.is_empty(),
                "registry tool `{}` is missing a description",
                tool.cli_name
            );
        }
    }
}
