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
#[test]
fn test_lsp_binary_is_workspace_member_with_workspace_lints() {
    let manifest = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .expect("Cargo.toml should exist");

    // All three binaries should be declared.
    assert!(
        manifest.contains("name = \"cairn\""),
        "CLI binary 'cairn' should be declared"
    );
    assert!(
        manifest.contains("name = \"cairn-mcp\""),
        "MCP binary 'cairn-mcp' should be declared"
    );
    assert!(
        manifest.contains("name = \"cairn-lsp\""),
        "LSP binary 'cairn-lsp' should be declared"
    );

    // Workspace lints should apply.
    assert!(
        manifest.contains("workspace = true"),
        "workspace lints should be enabled"
    );
}

/// Scenario: All binaries report version on --version.
#[test]
fn test_all_binaries_report_version() {
    use std::process::Command;

    let binaries = ["cairn", "cairn-mcp", "cairn-lsp"];
    for bin in binaries {
        let output = Command::new("cargo")
            .args(["run", "--bin", bin, "--", "--version"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("cargo run should succeed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success(),
            "{bin} --version should exit 0, got stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            stdout.contains("cairn"),
            "{bin} --version should mention 'cairn', got: {stdout}"
        );
    }
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
#[test]
fn test_fixture_reconciler_observations_enter_map_without_new_nodes() {
    use cairn::reconcile::fixture::FixtureReconciler;
    use cairn::reconcile::{ReconcileRequest, Reconciler};

    let reconciler = FixtureReconciler::new("fixture-test");
    let request = ReconcileRequest {
        root: std::path::Path::new("."),
        ignores: &[],
    };

    let report = reconciler
        .reconcile(request)
        .expect("reconcile should succeed");

    // Fixture reconciler should produce findings (observations) but no new nodes.
    assert!(
        !report.findings.is_empty(),
        "fixture reconciler should produce observations"
    );
    assert!(
        report.claimed_files.is_empty(),
        "fixture reconciler should not claim files (no new nodes)"
    );
    assert!(
        report.symbols.is_empty(),
        "fixture reconciler should not produce symbols"
    );
}
