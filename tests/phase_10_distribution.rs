//! Phase 10 Distribution acceptance-criterion tests.
//!
//! Test contract for `phase-10-distribution`. Each test corresponds to one
//! acceptance-criterion scenario for the LSP server, plugin packaging,
//! reconciler extension points, and release checks. Tests are marked
//! `#[cflx_planned(phase = 1000)]` so `cargo test` skips them while
//! `cargo test -- --ignored` runs them and they fail with a clear
//! `unimplemented!` message naming the scenario. Phase 10 will remove
//! `#[cflx_planned]` group-by-group as code lands.
//!
//! See `openspec/specs/testing-baseline/spec.md` for the test-first pre-phase
//! convention.

use cairn::cflx_planned;

/// Scenario: LSP binary is a workspace member with workspace lints.
#[cflx_planned(phase = 1000)]
#[test]
fn test_lsp_binary_is_workspace_member_with_workspace_lints() {
    unimplemented!("awaits phase-10: cairn-lsp is workspace member with workspace lints");
}

/// Scenario: LSP diagnostics match `cairn lint --json`.
#[cflx_planned(phase = 1000)]
#[test]
fn test_lsp_diagnostics_match_cairn_lint_json() {
    unimplemented!("awaits phase-10: LSP diagnostics match cairn lint --json");
}

/// Scenario: LSP hover returns node metadata.
#[cflx_planned(phase = 1000)]
#[test]
fn test_lsp_hover_returns_node_metadata() {
    unimplemented!("awaits phase-10: LSP hover returns node metadata");
}

/// Scenario: LSP definition resolves to node declaration span.
#[cflx_planned(phase = 1000)]
#[test]
fn test_lsp_definition_resolves_to_node_declaration_span() {
    unimplemented!("awaits phase-10: LSP definition resolves to node declaration span");
}

/// Scenario: Plugin docs cover CLI, MCP, and project context.
#[cflx_planned(phase = 1000)]
#[test]
fn test_plugin_docs_cover_cli_mcp_and_project_context() {
    unimplemented!("awaits phase-10: plugin docs cover CLI, MCP, and project context");
}

/// Scenario: Example project exercises all listed capabilities.
#[cflx_planned(phase = 1000)]
#[test]
fn test_example_project_exercises_all_listed_capabilities() {
    unimplemented!("awaits phase-10: example project exercises all listed capabilities");
}

/// Scenario: Fixture reconciler observations enter the map without new nodes.
#[cflx_planned(phase = 1000)]
#[test]
fn test_fixture_reconciler_observations_enter_map_without_new_nodes() {
    unimplemented!("awaits phase-10: fixture reconciler observations enter map without new nodes");
}
