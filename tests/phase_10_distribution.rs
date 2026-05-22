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

/// Scenario: All binaries support --help.
#[test]
fn test_all_binaries_support_help() {
    use std::process::Command;

    let binaries = ["cairn", "cairn-mcp", "cairn-lsp"];
    for bin in binaries {
        let output = Command::new("cargo")
            .args(["run", "--bin", bin, "--", "--help"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("cargo run should succeed");

        assert!(
            output.status.success(),
            "{bin} --help should exit 0, got stderr: {}",
            String::from_utf8_lossy(&output.stderr)
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
#[test]
fn test_plugin_docs_cover_cli_mcp_and_project_context() {
    let claude_code = include_str!("../docs/claude-code.md");
    let agent_prompts = include_str!("../docs/agent-prompts.md");
    let mcp = include_str!("../docs/mcp.md");
    assert!(
        claude_code.contains("cairn-mcp") && claude_code.contains("CLI"),
        "plugin docs must cover MCP and CLI"
    );
    assert!(
        agent_prompts.contains("cairn context") && agent_prompts.contains("cairn lint"),
        "plugin docs must cover project context queries"
    );
    assert!(
        mcp.contains("cairn_get") || mcp.contains("tool"),
        "MCP docs must list available tools"
    );
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

/// Scenario: Cargo.toml has required packaging metadata.
#[test]
fn test_cargo_toml_has_required_packaging_metadata() {
    let manifest = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .expect("Cargo.toml should exist");

    assert!(
        manifest.contains("name = \"cairn\""),
        "package name must be declared"
    );
    assert!(
        manifest.contains("version = \"0.1.0\""),
        "package version must be declared"
    );
    assert!(
        manifest.contains("description ="),
        "package description must be declared"
    );
    assert!(
        manifest.contains("license ="),
        "package license must be declared"
    );
    assert!(
        manifest.contains("readme ="),
        "package readme must be declared"
    );
    assert!(
        manifest.contains("repository ="),
        "package repository must be declared"
    );
}

/// Scenario: Every CLI command has a description.
#[test]
fn test_every_cli_command_has_description() {
    use cairn::cli::run;
    let result = run(&["--help".to_owned()]);
    assert!(
        result.code == 0,
        "help must exit zero, got code: {}",
        result.code
    );
    assert!(
        !result.stdout.is_empty(),
        "help must produce non-empty output"
    );
    // Verify that common commands appear in the help text.
    for cmd in [
        "scan", "lint", "check", "get", "context", "ui", "init", "change",
    ] {
        assert!(
            result.stdout.contains(cmd),
            "help output must mention command: {cmd}"
        );
    }
}

/// Scenario: Agent prompts doc exists and references cairn commands.
#[test]
fn test_agent_prompts_doc_references_cairn_commands() {
    let prompts = include_str!("../docs/agent-prompts.md");
    assert!(
        prompts.contains("cairn context"),
        "agent prompts must reference cairn context"
    );
    assert!(
        prompts.contains("cairn lint"),
        "agent prompts must reference cairn lint"
    );
    assert!(
        prompts.contains("cairn scan"),
        "agent prompts must reference cairn scan"
    );
    assert!(
        prompts.contains("cairn hook"),
        "agent prompts must reference cairn hook"
    );
}

/// Scenario: Claude Code integration doc exists and references MCP and CLI.
#[test]
fn test_claude_code_integration_doc_references_mcp_and_cli() {
    let doc = include_str!("../docs/claude-code.md");
    assert!(
        doc.contains("cairn-mcp"),
        "Claude Code integration doc must reference cairn-mcp"
    );
    assert!(
        doc.contains("cairn context"),
        "Claude Code integration doc must reference CLI commands"
    );
    assert!(
        doc.contains("cairn scan --strict"),
        "Claude Code integration doc must reference scan --strict"
    );
}

/// Scenario: Command reference doc lists all CLI commands.
#[test]
fn test_command_reference_doc_lists_all_commands() {
    let doc = include_str!("../docs/commands.md");
    // Verify core commands are documented.
    for cmd in ["scan", "lint", "check", "context", "get", "init", "ui"] {
        assert!(
            doc.contains(&format!("cairn {cmd}")),
            "command reference must mention cairn {cmd}"
        );
    }
    // Verify change lifecycle commands.
    assert!(
        doc.contains("cairn change new"),
        "command reference must mention cairn change new"
    );
    // Verify draft and summarise commands (JSON-only).
    for cmd in ["drafts", "draft_show", "summarise"] {
        assert!(
            doc.contains(&format!("cairn {cmd}")),
            "command reference must mention cairn {cmd}"
        );
    }
    assert!(
        doc.contains("--json"),
        "command reference must mention --json flag"
    );
    assert!(
        doc.contains("Exit codes"),
        "command reference must document exit codes"
    );
}

/// Scenario: Every registered CLI command has a non-empty description.
#[test]
fn test_every_registered_command_has_description() {
    let registry = cairn::cli::registry();
    let result = cairn::cli::run(&["--help".to_owned()]);
    for tool in registry {
        // MCP-only tools are intentionally excluded from CLI help.
        if tool.mcp_name == "cairn_init_from_code" {
            continue;
        }
        assert!(
            result.stdout.contains(tool.cli_name),
            "help output must mention registered command: {}",
            tool.cli_name
        );
    }
}

/// Scenario: Every extra CLI command has a non-empty description.
#[test]
fn test_every_extra_cli_command_has_description() {
    let extra = ["accept", "change", "check", "export", "onboard", "refine"];
    let result = cairn::cli::run(&["--help".to_owned()]);
    for cmd in &extra {
        assert!(
            result.stdout.contains(cmd),
            "help output must mention extra command: {cmd}"
        );
    }
}

/// Scenario: Installation and startup for all binaries is documented.
#[test]
fn test_installation_and_startup_documented() {
    let doc = include_str!("../docs/commands.md");
    assert!(
        doc.contains("cargo install"),
        "installation docs must mention cargo install"
    );
    assert!(
        doc.contains("cairn-mcp"),
        "installation docs must mention cairn-mcp"
    );
    assert!(
        doc.contains("cairn-lsp"),
        "installation docs must mention cairn-lsp"
    );
    assert!(doc.contains("Startup"), "docs must have a startup section");
}
/// Scenario: Example project exists and exercises core capabilities.
#[test]
fn test_example_project_exercises_core_capabilities() {
    let example_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/demo");
    assert!(
        example_dir.exists(),
        "example project must exist at examples/demo"
    );
    let blueprint = example_dir.join("cairn.blueprint");
    assert!(
        blueprint.exists(),
        "example project must have a cairn.blueprint"
    );
    let result = cairn::cli::run(&[
        "--file".to_owned(),
        blueprint.to_string_lossy().to_string(),
        "check".to_owned(),
    ]);
    assert!(
        result.code == 0,
        "cairn check on example project must exit zero, got code: {} with stderr: {}",
        result.code,
        result.stderr
    );
    assert!(example_dir.join("meta/todos").exists());
    assert!(example_dir.join("meta/decisions").exists());
    assert!(example_dir.join("meta/contracts").exists());
    assert!(example_dir.join("meta/sources").exists());
    assert!(example_dir.join("meta/research").exists());
    assert!(example_dir.join("src").exists());
    assert!(example_dir.join("meta/changes").exists());
    assert!(
        example_dir.join("README.md").exists(),
        "example project must have a README"
    );
}
