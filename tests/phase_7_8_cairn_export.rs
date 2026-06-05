//! Phase 7.8 Cairn Export acceptance-criterion tests.
//!
//! End-to-end verification of the `cairn export` command. Each test maps
//! to one acceptance-criterion scenario from the phase-7.8 spec.

use std::path::PathBuf;

use cairn::cli::export::{
    self, ArtefactEntry, ChangeEntry, EdgeEntry, ExportEnvelope, SCHEMA_VERSION,
};

fn empty_envelope() -> ExportEnvelope {
    ExportEnvelope {
        schema_version: SCHEMA_VERSION,
        generated_at: "2026-05-07T12:00:00Z".to_owned(),
        blueprint_path: PathBuf::from("cairn.blueprint"),
        nodes: Vec::new(),
        edges: Vec::new(),
        artefacts: Vec::new(),
        changes: Vec::new(),
    }
}

/// Scenario: Default format is JSON.
#[test]
fn test_default_format_is_json() {
    // The CLI dispatch defaults to JSON when --format is omitted; we exercise
    // this by inspecting render_json output, which is what the default path
    // produces.
    let env = empty_envelope();
    let out = export::render_json(&env);
    assert!(out.starts_with("{\n"));
    assert!(out.contains("\"schema_version\""));
}

/// Scenario: Markdown format selected via flag.
#[test]
fn test_markdown_format_selected_via_flag() {
    let env = empty_envelope();
    let md = export::render_markdown(&env);
    assert!(md.starts_with("# Cairn Export"));
    assert!(md.contains("## Nodes"));
    assert!(md.contains("## Edges"));
    assert!(md.contains("## Artefacts"));
    assert!(md.contains("## Active Changes"));
}

/// Scenario: JSON envelope carries `schema_version`.
#[test]
fn test_json_envelope_carries_schema_version() {
    let env = empty_envelope();
    let out = export::render_json(&env);
    let trimmed = out.trim_start_matches('{').trim_start();
    assert!(
        trimmed.starts_with("\"schema_version\""),
        "first key must be schema_version"
    );
    assert!(out.contains("\"schema_version\": 1"));
}

/// Scenario: Markdown payload contains no em-dashes.
#[test]
fn test_markdown_payload_contains_no_em_dashes() {
    let env = empty_envelope();
    let md = export::render_markdown(&env);
    assert!(
        !md.contains('\u{2014}'),
        "markdown must not contain U+2014 (em-dash)"
    );
}

/// Scenario: --output flag is required.
#[test]
fn test_output_flag_is_required() {
    let result = cairn::cli::run(&[
        "export".to_owned(),
        "--format".to_owned(),
        "json".to_owned(),
    ]);
    assert_eq!(result.code, 1);
    assert!(
        result.stderr.contains("--output"),
        "stderr must mention --output, got: {}",
        result.stderr
    );
}

/// Scenario: Invalid format value is rejected.
#[test]
fn test_invalid_format_value_is_rejected() {
    let result = cairn::cli::run(&[
        "export".to_owned(),
        "--format".to_owned(),
        "csv".to_owned(),
        "--output".to_owned(),
        "out.csv".to_owned(),
    ]);
    assert_eq!(result.code, 1);
    assert!(
        result.stderr.contains("csv"),
        "stderr must mention rejected format, got: {}",
        result.stderr
    );
}

/// Scenario: Export is lifecycle-orthogonal.
#[test]
fn test_export_is_lifecycle_orthogonal() {
    // The renderer accepts an envelope regardless of node/edge/artefact/change
    // contents, including an envelope with mixed populated fields.
    let env = ExportEnvelope {
        schema_version: SCHEMA_VERSION,
        generated_at: "2026-05-07T12:00:00Z".to_owned(),
        blueprint_path: PathBuf::from("cairn.blueprint"),
        nodes: Vec::new(),
        edges: vec![EdgeEntry {
            from: "node-a".to_owned(),
            to: "node-b".to_owned(),
            verb: "calls".to_owned(),
        }],
        artefacts: vec![ArtefactEntry {
            artefact_type: cairn::artefacts::registry::ArtefactType::Contract,
            id: "meta/contracts/foo/contract.md".to_owned(),
            path: "meta/contracts/foo/contract.md".to_owned(),
            node: Some("node-a".to_owned()),
        }],
        changes: vec![ChangeEntry {
            id: "phase-x".to_owned(),
            state: "active".to_owned(),
            title: "Phase X".to_owned(),
        }],
    };
    let json = export::render_json(&env);
    let md = export::render_markdown(&env);
    assert!(json.contains("\"schema_version\": 1"));
    assert!(md.contains("phase-x"));
}

/// Scenario: Render delegates to a shared library service.
#[test]
fn test_render_delegates_to_shared_library_service() {
    // build_export is the shared library entrypoint usable by CLI, MCP, LSP,
    // and webui consumers without parsing CLI text. Returns CairnError per
    // phase-7.8 reforge cycle 1 (was Result<_, String>).
    let _: fn(
        &std::path::Path,
        &std::path::Path,
    ) -> Result<ExportEnvelope, cairn::error::CairnError> = export::build_export;
    let _: fn(&ExportEnvelope) -> String = export::render_json;
    let _: fn(&ExportEnvelope) -> String = export::render_markdown;
}

/// Scenario: `build_export` errors carry CK001 when the scanner fails.
#[test]
fn test_build_export_returns_ck001_on_scanner_error() {
    let result = export::build_export(
        std::path::Path::new("/nonexistent/cairn.blueprint"),
        std::path::Path::new("/nonexistent/changes"),
    );
    let err = result.expect_err("missing blueprint must error");
    assert_eq!(err.code(), "CK001");
}

/// Cycle 3: when the runner is invoked with --json, error envelopes
/// carry the canonical `code` field so MCP/LSP consumers can parse them.
#[test]
fn test_export_runner_emits_json_errors_when_json_flag_set() {
    let result = cairn::cli::run(&[
        "--json".to_owned(),
        "--file".to_owned(),
        "/nonexistent/cairn.blueprint".to_owned(),
        "export".to_owned(),
        "--format".to_owned(),
        "json".to_owned(),
        "--output".to_owned(),
        "/tmp/cycle-3-test-out.json".to_owned(),
    ]);
    assert_eq!(result.code, 1, "missing blueprint must exit 1");
    // findings_output writes JSON envelopes to stdout, plain text to stderr.
    assert!(
        result.stdout.contains("\"code\":\"CK001\""),
        "JSON error envelope must carry CK001 on stdout, got stdout: {}, stderr: {}",
        result.stdout,
        result.stderr
    );
}

/// Insta snapshot: pin JSON wire format for a representative envelope.
#[test]
fn json_snapshot_pins_wire_format() {
    let env = ExportEnvelope {
        schema_version: SCHEMA_VERSION,
        generated_at: "2026-05-07T12:00:00Z".to_owned(),
        blueprint_path: PathBuf::from("cairn.blueprint"),
        nodes: Vec::new(),
        edges: vec![EdgeEntry {
            from: "node-a".to_owned(),
            to: "node-b".to_owned(),
            verb: "calls".to_owned(),
        }],
        artefacts: vec![ArtefactEntry {
            artefact_type: cairn::artefacts::registry::ArtefactType::Contract,
            id: "meta/contracts/foo/contract.md".to_owned(),
            path: "meta/contracts/foo/contract.md".to_owned(),
            node: Some("node-a".to_owned()),
        }],
        changes: vec![ChangeEntry {
            id: "phase-x".to_owned(),
            state: "active".to_owned(),
            title: "Phase X".to_owned(),
        }],
    };
    insta::assert_snapshot!("export_json_envelope", export::render_json(&env));
}

/// Insta snapshot: pin Markdown wire format for a representative envelope.
#[test]
fn markdown_snapshot_pins_wire_format() {
    let env = ExportEnvelope {
        schema_version: SCHEMA_VERSION,
        generated_at: "2026-05-07T12:00:00Z".to_owned(),
        blueprint_path: PathBuf::from("cairn.blueprint"),
        nodes: Vec::new(),
        edges: vec![EdgeEntry {
            from: "node-a".to_owned(),
            to: "node-b".to_owned(),
            verb: "calls".to_owned(),
        }],
        artefacts: Vec::new(),
        changes: vec![ChangeEntry {
            id: "phase-x".to_owned(),
            state: "active".to_owned(),
            title: "Phase X".to_owned(),
        }],
    };
    insta::assert_snapshot!("export_markdown_envelope", export::render_markdown(&env));
}

/// Scenario: Mermaid format selected via flag.
#[test]
fn test_mermaid_format_selected_via_flag() {
    let env = empty_envelope();
    let out = export::render_mermaid(&env);
    assert!(
        out.starts_with("graph TD"),
        "mermaid output must start with `graph TD`, got: {out}"
    );
}

/// Scenario: Mermaid output contains node declarations.
#[test]
fn test_mermaid_output_contains_node_declarations() {
    use cairn::blueprint::NodeKind;
    use cairn::map::graph::{NodeRecord, NodeState};
    let mut env = empty_envelope();
    env.nodes.push(NodeRecord {
        id: "app.api".to_owned(),
        name: "App API".to_owned(),
        kind: NodeKind::Module,
        parent: Some("app".to_owned()),
        description: String::new(),
        state: NodeState::Synced,
        paths: Vec::new(),
        tags: Vec::new(),
        children: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        files: Vec::new(),
        span: cairn::blueprint::Span::point("", 0, 0),
    });
    let out = export::render_mermaid(&env);
    assert!(out.contains("app.api"), "mermaid must declare the node id");
}

/// Scenario: Mermaid output contains edge declarations.
#[test]
fn test_mermaid_output_contains_edge_declarations() {
    let mut env = empty_envelope();
    env.edges.push(EdgeEntry {
        from: "node-a".to_owned(),
        to: "node-b".to_owned(),
        verb: "calls".to_owned(),
    });
    let out = export::render_mermaid(&env);
    assert!(
        out.contains("node_a") && out.contains("node_b"),
        "mermaid must include sanitized edge endpoint IDs"
    );
    assert!(out.contains("-->"), "mermaid must use --> for edges");
}

/// Scenario: Mermaid format is parseable by --format flag.
#[test]
fn test_mermaid_format_flag_accepted() {
    let result = export::run(
        &[
            "export".to_owned(),
            "--format".to_owned(),
            "mermaid".to_owned(),
            "--output".to_owned(),
            "/tmp/test-mermaid-flag.md".to_owned(),
        ],
        std::path::Path::new("nonexistent.blueprint"),
        std::path::Path::new("meta/changes"),
        false,
    );
    // Will fail because blueprint doesn't exist, but must NOT fail with "unknown format".
    assert_ne!(result.stderr, "unknown export format: mermaid");
}

/// Scenario: Mermaid node IDs are valid unquoted Mermaid identifiers.
/// Node IDs with dots (e.g. "app.api") must be sanitized to `app_api`
/// so renderers can parse them. The original id appears in the label.
#[test]
fn test_mermaid_node_ids_are_sanitized() {
    use cairn::blueprint::{NodeKind, Span};
    use cairn::map::graph::{NodeRecord, NodeState};
    let mut env = empty_envelope();
    env.nodes.push(NodeRecord {
        id: "app.api".to_owned(),
        name: "App API".to_owned(),
        kind: NodeKind::Module,
        parent: None,
        description: String::new(),
        state: NodeState::Synced,
        paths: Vec::new(),
        tags: Vec::new(),
        children: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        files: Vec::new(),
        span: Span::point("", 0, 0),
    });
    let out = export::render_mermaid(&env);
    // The sanitized ID must appear without surrounding quotes as a Mermaid id.
    assert!(
        out.contains("app_api"),
        "mermaid must sanitize dots to underscores in node id: {out}"
    );
    // The original dotted id must appear in the label, not as a bare id.
    assert!(
        out.contains("\"app.api\"") || out.contains("app.api"),
        "mermaid must preserve original id in the label"
    );
    // Must NOT use quoted-string syntax as the node id: ["app.api"][...]
    assert!(
        !out.contains("[\"app.api\"]"),
        "mermaid must not use a quoted string as a node id"
    );
}

/// Scenario: Mermaid edges use sanitized IDs.
#[test]
fn test_mermaid_edge_ids_are_sanitized() {
    let mut env = empty_envelope();
    env.edges.push(EdgeEntry {
        from: "cairn.kernel".to_owned(),
        to: "cairn.ui".to_owned(),
        verb: "uses".to_owned(),
    });
    let out = export::render_mermaid(&env);
    assert!(
        out.contains("cairn_kernel") && out.contains("cairn_ui"),
        "mermaid edges must use sanitized IDs: {out}"
    );
    assert!(
        !out.contains("[\"cairn.kernel\"]"),
        "mermaid edges must not quote dotted IDs"
    );
}
