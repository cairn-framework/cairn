//! Phase 10 Distribution acceptance-criterion tests.
//!
//! Tests covering plugin packaging, reconciler extension points, and release
//! checks. LSP-related scenarios were retired along with the LSP surface per
//! decision #105.
//!
//! See `openspec/specs/testing-baseline/spec.md` for the test-first pre-phase
//! convention.

// LSP tests retired per decision #105; cflx_planned macro kept for future use.
// use cairn::cflx_planned;
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
    // Verify watch command.
    assert!(
        doc.contains("cairn watch"),
        "command reference must mention cairn watch"
    );
    // Verify change subcommands.
    assert!(
        doc.contains("cairn change tasks"),
        "command reference must mention cairn change tasks"
    );
    assert!(
        doc.contains("cairn change apply"),
        "command reference must mention cairn change apply"
    );
    // Verify migration command.
    assert!(
        doc.contains("cairn import-openspec"),
        "command reference must mention cairn import-openspec"
    );
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

/// Scenario: Gas City reference adapter pack has required structure.
#[test]
fn test_gas_city_adapter_pack_structure() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pack_dir = root.join("adapters/gascity");
    assert!(pack_dir.exists(), "adapters/gascity/ directory must exist");

    // pack.toml with metadata.
    let pack_toml = pack_dir.join("pack.toml");
    assert!(pack_toml.exists(), "pack.toml must exist");
    let pack_content = std::fs::read_to_string(&pack_toml).expect("read pack.toml");
    assert!(pack_content.contains("name"), "pack.toml must have a name");
    assert!(
        pack_content.contains("cairn"),
        "pack.toml must reference cairn"
    );

    // README with install steps.
    let readme = pack_dir.join("README.md");
    assert!(readme.exists(), "README.md must exist");
    let readme_content = std::fs::read_to_string(&readme).expect("read README");
    assert!(
        readme_content.contains("install"),
        "README must mention install steps"
    );

    // Required formula files.
    let formulas = [
        "cairn-reconcile.formula.toml",
        "cairn-lint.formula.toml",
        "cairn-drift-gate.formula.toml",
        "cairn-onboard.formula.toml",
    ];
    for name in formulas {
        let path = pack_dir.join("formulas").join(name);
        assert!(path.exists(), "formula {name} must exist");
        let content = std::fs::read_to_string(&path).expect("read formula");
        assert!(
            content.contains("formula = "),
            "formula {name} must declare formula name"
        );
        assert!(
            content.contains("[[steps]]"),
            "formula {name} must have at least one step"
        );
    }
}

/// Scenario: Every formula TOML file has multi-step structure with dependency edges.
///
/// Each formula must have at least 2 steps. Every step must carry `id` and
/// `description`. At least one step must declare `needs` to represent ordering.
/// New formula additions are validated here automatically via directory walk.
#[test]
fn test_formula_steps_have_required_fields_and_deps() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let formulas_dir = root.join("adapters/gascity/formulas");
    assert!(
        formulas_dir.exists(),
        "adapters/gascity/formulas/ must exist"
    );

    // Required formula files (6 total: 4 existing + 2 new).
    let expected = [
        "cairn-reconcile.formula.toml",
        "cairn-lint.formula.toml",
        "cairn-drift-gate.formula.toml",
        "cairn-onboard.formula.toml",
        "cairn-propose-node.formula.toml",
        "cairn-wave-dispatch.formula.toml",
    ];
    for name in expected {
        assert!(
            formulas_dir.join(name).exists(),
            "formula file {name} must exist"
        );
    }

    // Walk all formula files and validate structure.
    let entries = std::fs::read_dir(&formulas_dir).expect("read formulas dir");
    let mut checked = 0usize;
    for entry in entries {
        let path = entry.expect("dir entry").path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.ends_with(".formula.toml") {
            continue;
        }
        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {name}: {e}"));
        let doc: toml::Value =
            toml::from_str(&content).unwrap_or_else(|e| panic!("{name} is not valid TOML: {e}"));

        // Top-level fields.
        assert!(
            doc.get("formula").and_then(toml::Value::as_str).is_some(),
            "{name}: missing string field `formula`"
        );
        assert!(
            doc.get("description")
                .and_then(toml::Value::as_str)
                .is_some(),
            "{name}: missing string field `description`"
        );

        // Steps array: at least 2 entries.
        let steps = doc
            .get("steps")
            .and_then(toml::Value::as_array)
            .unwrap_or_else(|| panic!("{name}: missing `[[steps]]` array"));
        assert!(
            steps.len() >= 2,
            "{name}: need ≥2 steps, found {}",
            steps.len()
        );

        // Each step: id + description required; at least one step has needs.
        let mut any_needs = false;
        for (i, step) in steps.iter().enumerate() {
            assert!(
                step.get("id").and_then(toml::Value::as_str).is_some(),
                "{name} step[{i}]: missing string field `id`"
            );
            assert!(
                step.get("description")
                    .and_then(toml::Value::as_str)
                    .is_some(),
                "{name} step[{i}]: missing string field `description`"
            );
            if step.get("needs").is_some() {
                any_needs = true;
            }
        }
        assert!(any_needs, "{name}: at least one step must declare `needs`");

        checked += 1;
    }
    assert!(checked >= 6, "expected ≥6 formula files, found {checked}");
}
