//! MCP query tool registry.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) const TOOL_REGISTRY: [ToolMetadata; 39] = [
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
        "symbols",
        "cairn_symbols",
        "NodeRequest",
        "SymbolsResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "bundle",
        "cairn_bundle",
        "NodeRequest",
        "BundleResponse",
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
        "islands",
        "cairn_islands",
        "IslandsRequest",
        "IslandsResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "frontier",
        "cairn_frontier",
        "FrontierRequest",
        "FrontierResponse",
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
        "health",
        "cairn_health",
        "HealthRequest",
        "HealthResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "remediate",
        "cairn_remediate",
        "RemediateRequest",
        "RemediateResponse",
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
    tool(
        "context",
        "cairn_context",
        "ContextRequest",
        "ContextResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "init_from_code",
        "cairn_init_from_code",
        "InitFromCodeRequest",
        "InitFromCodeResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "refine",
        "cairn_refine",
        "RefineRequest",
        "RefineResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "drafts",
        "cairn_drafts",
        "DraftsRequest",
        "DraftsResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "draft_show",
        "cairn_draft_show",
        "DraftShowRequest",
        "DraftShowResponse",
        SafetyClass::ReadOnly,
    ),
    tool(
        "draft_discard",
        "cairn_draft_discard",
        "DraftDiscardRequest",
        "DraftDiscardResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "draft_edit",
        "cairn_draft_edit",
        "DraftEditRequest",
        "DraftEditResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "draft_accept",
        "cairn_draft_accept",
        "DraftAcceptRequest",
        "DraftAcceptResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "summarise",
        "cairn_summarise",
        "SummariseRequest",
        "SummariseResponse",
        SafetyClass::Mutating,
    ),
    tool(
        "watch",
        "cairn_watch",
        "WatchRequest",
        "WatchResponse",
        SafetyClass::ReadOnly,
    ),
];

pub(super) const fn tool(
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
    fn test_registry_contains_drafts() {
        assert!(metadata_for_tool("drafts").is_some());
        assert!(metadata_for_tool("cairn_drafts").is_some());
    }

    #[test]
    fn test_registry_contains_draft_show() {
        assert!(metadata_for_tool("draft_show").is_some());
        assert!(metadata_for_tool("cairn_draft_show").is_some());
    }

    #[test]
    fn test_drafts_is_readonly() {
        assert!(is_readonly("drafts"));
        assert!(!is_mutating("drafts"));
    }

    #[test]
    fn test_draft_show_is_readonly() {
        assert!(is_readonly("draft_show"));
        assert!(!is_mutating("draft_show"));
    }

    #[test]
    fn test_registry_contains_draft_discard() {
        assert!(metadata_for_tool("draft_discard").is_some());
        assert!(metadata_for_tool("cairn_draft_discard").is_some());
    }

    #[test]
    fn test_draft_discard_is_mutating() {
        assert!(!is_readonly("draft_discard"));
        assert!(is_mutating("draft_discard"));
    }

    #[test]
    fn test_registry_contains_draft_edit() {
        assert!(metadata_for_tool("draft_edit").is_some());
        assert!(metadata_for_tool("cairn_draft_edit").is_some());
    }

    #[test]
    fn test_draft_edit_is_mutating() {
        assert!(!is_readonly("draft_edit"));
        assert!(is_mutating("draft_edit"));
    }

    #[test]
    fn test_registry_contains_draft_accept() {
        assert!(metadata_for_tool("draft_accept").is_some());
        assert!(metadata_for_tool("cairn_draft_accept").is_some());
    }

    #[test]
    fn test_draft_accept_is_mutating() {
        assert!(!is_readonly("draft_accept"));
        assert!(is_mutating("draft_accept"));
    }

    #[test]
    fn test_registry_size() {
        assert_eq!(TOOL_REGISTRY.len(), 39);
    }
}
