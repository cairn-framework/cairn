//! Phase 1 kernel integration tests.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use cairn::{
    artefacts::contract,
    blueprint::{lexer, parser},
    map::{build_graph, query},
    reconcile::{ReconcileRequest, Reconciler, code::RustCodeReconciler},
    scanner,
};

#[test]
fn test_lexer_tracks_spans_and_tags() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = lexer::tokenize("fixture", "System App \"desc\" id \"app\" @tag {}")?;

    assert!(
        tokens
            .iter()
            .any(|token| matches!(&token.kind, lexer::TokenKind::Tag(value) if value == "tag"))
    );
    assert_eq!(tokens[0].span.line, 1);
    assert_eq!(tokens[0].span.column, 1);

    Ok(())
}

#[test]
fn test_parser_handles_nested_nodes_edges_lists_and_owns_files()
-> Result<(), Box<dyn std::error::Error>> {
    let ast = parser::parse_str(
        "fixture",
        r#"System App "desc" id "app" @system {
    Container Api "api" id "app.api" {
        path "./src"
        owns-files: true
        Module Auth "auth" id "app.api.auth" {
            path ["./src/auth", "./src/auth/mod.rs"]
            contract "./meta/contracts/auth.md"
            todos "./meta/todos"
        }
    }
}
app.api.auth -> app.api "uses"
"#,
    )?;

    let api = &ast.nodes[0].children[0];
    let auth = &api.children[0];
    assert!(api.owns_files);
    assert_eq!(auth.paths.len(), 2);
    assert_eq!(auth.contracts, vec!["./meta/contracts/auth.md"]);
    assert_eq!(auth.raw_fields[0].name, "todos");
    assert_eq!(ast.edges[0].from, "app.api.auth");

    Ok(())
}

#[test]
fn test_parser_reports_location_for_malformed_blueprint() {
    let error = parser::parse_str("bad.blueprint", "System App \"desc\" id \"app\" {").unwrap_err();

    assert_eq!(error.span.file, "bad.blueprint");
    assert!(error.message.contains("expected `}`"));
}

#[test]
fn test_parser_rejects_legacy_blueprint_extension() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("legacy-extension")?;
    fs::write(
        root.join("cairn.dsl"),
        "System App \"desc\" id \"app\" {}\n",
    )?;

    let error = parser::parse_file(root.join("cairn.dsl")).unwrap_err();

    assert_eq!(error.code, "CAIRN_BLUEPRINT_LEGACY_EXTENSION");
    assert!(error.message.contains("cairn.blueprint"));

    Ok(())
}

#[test]
fn test_config_ignore_layers_and_protected_paths() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("config")?;
    fs::write(root.join(".gitignore"), "ignored-git\n")?;
    fs::write(root.join(".cairnignore"), "ignored-cairn\n")?;
    fs::write(
        root.join("cairn.config.yaml"),
        "reconcilers:\n  - id: rust-code\n    config:\n      ignore:\n        - ignored-config\ncontext: \"ctx\"\nrules:\n  one: \"two\"\nunknown: keep\n",
    )?;

    let config = scanner::config::load(&root)?;

    assert!(config.ignores.contains(&"ignored-git".to_owned()));
    assert!(config.ignores.contains(&"ignored-cairn".to_owned()));
    assert!(config.ignores.contains(&"ignored-config".to_owned()));
    assert!(!scanner::config::is_ignored(
        "meta/contracts/a.md",
        &config.ignores
    ));
    assert!(scanner::config::is_ignored(
        "ignored-config/file.rs",
        &config.ignores
    ));

    Ok(())
}

#[test]
fn test_graph_indexes_integrity_and_cycles() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("graph")?;
    let source = r#"System App "desc" id "app" {
    Container Api "api" id "app.api" {
        path "./src"
        owns-files: true
        Module Auth "auth" id "app.api.auth" {
            path "./src/auth"
        }
    }
}
app.api -> app.api.auth "contains"
app.api.auth -> app.api "cycle"
"#;
    let ast = parser::parse_str("fixture", source)?;
    let graph = build_graph(
        &ast,
        &root,
        &contract::ContractSet::default(),
        &BTreeMap::default(),
        Vec::new(),
    );

    assert!(graph.nodes.contains_key("app.api.auth"));
    assert_eq!(
        query::neighbourhood(&graph, "Auth")?.inbound,
        vec!["app.api".to_owned()]
    );
    let explorer_graph = query::graph(&graph);
    assert!(
        explorer_graph
            .nodes
            .iter()
            .any(|node| node.id == "app.api.auth")
    );
    assert!(explorer_graph.edges.iter().any(|edge| {
        edge.from == "app" && edge.to == "app.api" && edge.kind == query::GraphEdgeKind::Ownership
    }));
    assert!(explorer_graph.edges.iter().any(|edge| {
        edge.from == "app.api"
            && edge.to == "app.api.auth"
            && edge.kind == query::GraphEdgeKind::Dependency
            && edge.description == "contains"
    }));
    assert!(query::order(&graph).is_err());

    Ok(())
}

#[test]
fn test_contract_loader_and_cli_contract_query() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("contract")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
        contract "./meta/contracts/auth.md"
    }
}
"#,
    )?;
    fs::write(
        root.join("meta/contracts/auth.md"),
        "---\nnode: app.auth\n---\n# Auth Contract\n",
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    let node = result.graph.resolve("app.auth")?;
    assert_eq!(node.files, vec!["src/auth/lib.rs"]);
    assert_eq!(
        result.contracts.contracts["./meta/contracts/auth.md"].node,
        "app.auth"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["contract", "app.auth"])
        .output()?;

    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("Auth Contract"));

    Ok(())
}

#[test]
fn test_scan_writes_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("scan")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub struct Api;\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .arg("scan")
        .output()?;

    assert!(output.status.success());
    assert!(root.join("map.md").exists());
    assert!(!root.join("index.md").exists());
    assert!(root.join(".cairn/log.md").exists());
    assert!(root.join(".cairn/state/interface-hashes.json").exists());

    Ok(())
}

#[test]
fn test_scan_warns_when_legacy_blueprint_file_also_exists() -> Result<(), Box<dyn std::error::Error>>
{
    let root = temp_root("legacy-collision")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub struct Api;\n")?;
    fs::write(root.join("cairn.dsl"), "System Old \"old\" id \"old\" {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .arg("scan")
        .output()?;

    assert!(output.status.success());
    assert!(String::from_utf8(output.stderr)?.contains("cairn.dsl"));
    assert!(root.join("map.md").exists());

    Ok(())
}

#[test]
fn test_scan_with_only_legacy_blueprint_file_suggests_rename()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("legacy-only")?;
    fs::write(root.join("cairn.dsl"), "System Old \"old\" id \"old\" {}\n")?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .arg("scan")
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("no blueprint file was found"));
    assert!(stdout.contains("cairn.blueprint"));

    Ok(())
}

#[test]
fn test_internal_node_ownership_opt_in_controls_direct_files()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("ownership")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::write(root.join("src/direct.rs"), "pub fn direct() {}\n")?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn auth() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Container Api "api" id "app.api" {
        path "./src"
        Module Auth "auth" id "app.api.auth" {
            path "./src/auth"
        }
    }
}
"#,
    )?;

    let default_result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(
        default_result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_RECONCILE_ORPHANED_FILE")
    );

    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Container Api "api" id "app.api" {
        path "./src"
        owns-files: true
        Module Auth "auth" id "app.api.auth" {
            path "./src/auth"
        }
    }
}
"#,
    )?;
    let opt_in_result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(opt_in_result.graph.findings.is_empty());
    assert_eq!(
        opt_in_result.graph.resolve("app.api")?.files,
        vec!["src/direct.rs"]
    );
    assert_eq!(
        opt_in_result.graph.resolve("app.api.auth")?.files,
        vec!["src/auth/lib.rs"]
    );

    Ok(())
}

#[test]
fn test_rust_reconciler_extracts_public_symbols_with_tree_sitter()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("tree-sitter")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(
        root.join("src/lib.rs"),
        r"
const PRIVATE: u8 = 1;
pub const EXPORTED: u8 = 2;

fn hidden() {}

pub fn exported(
    value: u8,
) -> u8 {
    value + EXPORTED
}

pub(crate) struct Visible {
    pub field: u8,
}

macro_rules! generated {
    () => {
        pub fn generated_by_macro() {}
    };
}
",
    )?;
    let ast = parser::parse_str(
        "fixture",
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;
    let report = RustCodeReconciler::new(&ast).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;

    assert!(
        report
            .symbols
            .iter()
            .any(|symbol| { symbol == "pub fn exported( value: u8, ) -> u8" })
    );
    assert!(
        report
            .symbols
            .iter()
            .any(|symbol| { symbol.starts_with("pub(crate) struct Visible") })
    );
    assert!(
        !report
            .symbols
            .iter()
            .any(|symbol| symbol.contains("hidden"))
    );
    assert!(
        !report
            .symbols
            .iter()
            .any(|symbol| symbol.contains("generated_by_macro"))
    );

    Ok(())
}

#[test]
fn test_contract_wrong_node_and_missing_ghost_pointer_severity()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("contract-severity")?;
    fs::create_dir_all(root.join("src"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::write(root.join("src/lib.rs"), "pub fn lib() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
        contract "./meta/contracts/wrong.md"
    }
    Module Ghost "ghost" id "app.ghost" {
        path "./missing"
        contract "./meta/contracts/missing.md"
    }
}
"#,
    )?;
    fs::write(
        root.join("meta/contracts/wrong.md"),
        "---\nnode: app.missing\n---\n# Wrong\n",
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;

    assert!(
        result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_CONTRACT_UNKNOWN_NODE")
    );
    assert!(result.graph.findings.iter().any(|finding| {
        finding.code == "CAIRN_CONTRACT_MISSING"
            && finding.severity == cairn::map::FindingSeverity::Warning
    }));

    Ok(())
}

#[test]
fn test_cli_blocks_queries_on_structural_errors() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("structural-cli")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn lib() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src"
    }
    Module Two "two" id "app.two" {
        path "./src"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["get", "app.one"])
        .output()?;

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("CAIRN_INTEGRITY_PATH_TIE"));

    Ok(())
}

#[test]
fn test_contract_frontmatter_must_match_declaring_node() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("contract-wrong-existing")?;
    fs::create_dir_all(root.join("src/a"))?;
    fs::create_dir_all(root.join("src/b"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::write(root.join("src/a/lib.rs"), "pub fn a() {}\n")?;
    fs::write(root.join("src/b/lib.rs"), "pub fn b() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module A "a" id "app.a" {
        path "./src/a"
        contract "./meta/contracts/a.md"
    }
    Module B "b" id "app.b" {
        path "./src/b"
    }
}
"#,
    )?;
    fs::write(
        root.join("meta/contracts/a.md"),
        "---\nnode: app.b\n---\n# Wrong Contract\n",
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(result.graph.findings.iter().any(|finding| {
        finding.code == "CAIRN_CONTRACT_WRONG_NODE"
            && finding.node.as_deref() == Some("app.a")
            && finding.severity == cairn::map::FindingSeverity::Error
    }));

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["contract", "app.a"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("CAIRN_CONTRACT_WRONG_NODE"));

    Ok(())
}

#[test]
fn test_phase_2_loads_artefacts_and_query_commands() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("artefacts")?;
    write_phase_2_fixture(&root)?;
    fs::create_dir_all(root.join(".cairn"))?;
    fs::write(
        root.join(".cairn/log.md"),
        "- scan: nodes=2, findings=0, errors=0\n",
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;

    assert!(result.graph.findings.is_empty());
    assert_eq!(result.artefacts.todos.len(), 1);
    assert_eq!(result.artefacts.decisions.len(), 2);
    assert_eq!(result.artefacts.reviews[0].reviewer, "george");
    assert_eq!(result.artefacts.research[0].sources, vec!["src.auth"]);
    assert_eq!(result.artefacts.sources[0].id, "src.auth");

    let rationale = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "rationale", "app.auth"])
        .output()?;
    assert!(rationale.status.success());
    let rationale = String::from_utf8(rationale.stdout)?;
    assert!(rationale.contains("\"decisions\""));
    assert!(rationale.contains("dec.auth"));
    assert!(rationale.contains("res.auth"));
    assert!(rationale.contains("src.auth"));

    let todos = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["todos", "app.auth", "--status", "open"])
        .output()?;
    assert!(todos.status.success());
    assert!(String::from_utf8(todos.stdout)?.contains("Todo"));

    let neighbourhood = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "neighbourhood", "app.auth"])
        .output()?;
    assert!(neighbourhood.status.success());
    let neighbourhood = String::from_utf8(neighbourhood.stdout)?;
    assert!(neighbourhood.contains("dec.auth"));
    assert!(!neighbourhood.contains("todo.md"));

    let status = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "status"])
        .output()?;
    assert!(status.status.success());
    let status = String::from_utf8(status.stdout)?;
    assert!(status.contains("\"active_changes\":[]"));
    assert!(status.contains("todo.md"));
    assert!(status.contains("scan: nodes=2"));

    Ok(())
}

#[test]
fn test_phase_2_integrity_findings() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("artefact-integrity")?;
    write_phase_2_fixture(&root)?;
    fs::write(
        root.join("meta/sources/src.auth.md"),
        "---\nid: src.auth\nfile: ./meta/sources/auth.txt\nsha256: bad\nverification: verified\ntype: document\ndate: 2026-04-17\ndescription: Auth evidence.\n---\n# Source\n",
    )?;
    fs::write(
        root.join("meta/reviews/bad.md"),
        "---\nnode: app.auth\nreview_type: robot\ndate: 2026-04-17\nreviewer: bot\n---\n# Bad\n",
    )?;
    fs::write(
        root.join("meta/decisions/missing-source.md"),
        "---\nid: dec.missing\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-04-17\ninformed_by:\n  - type: source\n    id: src.missing\n---\n# Missing provenance\n",
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;

    assert!(result.graph.findings.iter().any(|finding| {
        finding.code == "CAIRN_SOURCE_SHA256_MISMATCH"
            && finding.severity == cairn::map::FindingSeverity::Error
    }));
    assert!(
        result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_REVIEW_TYPE_INVALID")
    );
    assert!(
        result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_DECISION_UNKNOWN_PROVENANCE")
    );

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["sources", "app.auth"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("CAIRN_SOURCE_SHA256_MISMATCH"));

    Ok(())
}

#[test]
fn test_hook_structural_blocks_structural_errors() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-structural")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn lib() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module One "one" id "app.one" {
        path "./src"
    }
    Module Two "two" id "app.two" {
        path "./src"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "structural"])
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Decision: block"));
    assert!(stdout.contains("CAIRN_INTEGRITY_PATH_TIE"));

    Ok(())
}

#[test]
fn test_hook_tension_reports_warnings_without_blocking() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-tension")?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
        contract "./meta/contracts/missing.md"
    }
}
"#,
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "hook", "tension"])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("\"kind\":\"tension\""));
    assert!(stdout.contains("\"decision\":\"pass\""));
    assert!(stdout.contains("CAIRN_CONTRACT_MISSING"));

    Ok(())
}

#[test]
fn test_hook_interface_blocks_changed_interface_hash() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-interface")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn one() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;
    scanner::scan(&root, &root.join("cairn.blueprint"))?;
    fs::write(root.join("src/lib.rs"), "pub fn two() {}\n")?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "interface"])
        .output()?;

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("CAIRN_INTERFACE_HASH_CHANGED"));

    Ok(())
}

#[allow(clippy::needless_raw_string_hashes)]
#[test]
fn test_hash_stable_across_comments_and_formatting() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hash-stable-comments")?;
    fs::create_dir_all(root.join("src"))?;
    let source1 = r#"
// This is a comment
pub fn foo() {}
pub struct Bar;
"#;
    let source2 = r#"
/* Block comment */ pub   fn   foo(){}
/**
 * Multi-line
 * comment
 */
pub struct Bar/*trailing*/;
"#;
    fs::write(root.join("src/lib.rs"), source1)?;
    let ast1 = parser::parse_str(
        "fixture",
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;
    let report1 = RustCodeReconciler::new(&ast1).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;
    let hash1 = report1.fingerprint.hash.clone();

    fs::write(root.join("src/lib.rs"), source2)?;
    let report2 = RustCodeReconciler::new(&ast1).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;
    let hash2 = report2.fingerprint.hash.clone();

    assert_eq!(
        hash1, hash2,
        "hash should be stable across comments and formatting"
    );
    Ok(())
}

#[test]
fn test_hash_stable_across_private_symbols() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hash-stable-private")?;
    fs::create_dir_all(root.join("src"))?;
    let source1 = "
pub fn public_api() {}
fn private_helper() {}
struct PrivateStruct {}
";
    let source2 = "
pub fn public_api() {}
fn different_private() {}
struct another_private {}
const PRIVATE_CONST: u8 = 1;
";
    fs::write(root.join("src/lib.rs"), source1)?;
    let ast1 = parser::parse_str(
        "fixture",
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;
    let report1 = RustCodeReconciler::new(&ast1).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;
    let hash1 = report1.fingerprint.hash.clone();

    fs::write(root.join("src/lib.rs"), source2)?;
    let report2 = RustCodeReconciler::new(&ast1).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;
    let hash2 = report2.fingerprint.hash.clone();

    assert_eq!(hash1, hash2, "hash should be stable across private symbols");
    Ok(())
}

#[test]
fn test_hash_stable_across_source_order() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hash-stable-order")?;
    fs::create_dir_all(root.join("src"))?;
    let source1 = "
pub fn first() {}
pub struct Second;
pub enum Third { A, B }
";
    let source2 = "
pub enum Third { A, B }
pub struct Second;
pub fn first() {}
";
    fs::write(root.join("src/lib.rs"), source1)?;
    let ast1 = parser::parse_str(
        "fixture",
        r#"System App "desc" id "app" {
    Module Lib "lib" id "app.lib" {
        path "./src"
    }
}
"#,
    )?;
    let report1 = RustCodeReconciler::new(&ast1).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;
    let hash1 = report1.fingerprint.hash.clone();

    fs::write(root.join("src/lib.rs"), source2)?;
    let report2 = RustCodeReconciler::new(&ast1).reconcile(ReconcileRequest {
        root: &root,
        ignores: &[],
    })?;
    let hash2 = report2.fingerprint.hash.clone();

    assert_eq!(hash1, hash2, "hash should be stable across source order");
    Ok(())
}

#[test]
fn test_divergence_contradiction_ct001() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("divergence-contradiction")?;
    fs::create_dir_all(root.join("src/rust"))?;
    fs::create_dir_all(root.join("src/ts"))?;
    fs::write(root.join("src/rust/lib.rs"), "pub fn api() {}\n")?;
    fs::write(root.join("src/ts/index.ts"), "export function api() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Multi "multi" id "app.multi" {
        path ["./src/rust/lib.rs", "./src/ts/index.ts"]
    }
}
"#,
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;

    let ct001 = result.graph.findings.iter().find(|f| f.code == "CT001");
    assert!(
        ct001.is_some(),
        "Expected CT001 divergence contradiction finding, got: {:?}",
        result.graph.findings
    );
    assert!(
        ct001.unwrap().severity == cairn::map::FindingSeverity::Error,
        "CT001 should be an Error severity"
    );

    Ok(())
}

#[test]
fn test_hook_all_reports_active_change_conflicts() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-conflicts")?;
    write_clean_hook_fixture(&root)?;
    write_change(
        &root,
        "first",
        "## MODIFIED Nodes\nModule Auth \"auth\" id \"app.auth\" {}\n",
        &[],
    )?;
    write_change(&root, "second", "## REMOVED Nodes\napp.auth\n", &[])?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "hook", "all"])
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("CAIRN_CHANGE_BLUEPRINT_CONFLICT"));
    assert!(stdout.contains("first"));
    assert!(stdout.contains("second"));

    Ok(())
}

#[test]
fn test_active_change_conflicts_cover_artefacts_and_renames()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-artifact-renames")?;
    write_clean_hook_fixture(&root)?;
    write_change(
        &root,
        "first",
        "## RENAMED Nodes\napp.auth -> app.identity\n",
        &[(
            "meta/contracts/auth.md",
            "---\noperation: modified\nnode: app.auth\n---\n# Contract\n",
        )],
    )?;
    write_change(
        &root,
        "second",
        "## RENAMED Nodes\napp.auth -> app.account\n",
        &[(
            "meta/contracts/auth.md",
            "---\noperation: modified\nnode: app.auth\n---\n# Contract\n",
        )],
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["hook", "structural"])
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("CAIRN_CHANGE_ARTEFACT_CONFLICT"));
    assert!(stdout.contains("CAIRN_CHANGE_RENAME_CONFLICT"));

    Ok(())
}

#[test]
fn test_archive_guard_runs_conflict_detector_before_mutation()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("archive-conflict")?;
    write_clean_hook_fixture(&root)?;
    write_change(
        &root,
        "first",
        "## MODIFIED Nodes\nModule Auth \"auth\" id \"app.auth\" {}\n",
        &[],
    )?;
    write_change(
        &root,
        "second",
        "## MODIFIED Nodes\nModule Auth \"auth\" id \"app.auth\" {}\n",
        &[],
    )?;
    let before = fs::read_to_string(root.join("cairn.blueprint"))?;

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["archive", "first"])
        .output()?;

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("CAIRN_CHANGE_BLUEPRINT_CONFLICT"));
    assert_eq!(fs::read_to_string(root.join("cairn.blueprint"))?, before);
    assert!(root.join("meta/changes/first").exists());
    assert!(root.join("meta/changes/second").exists());

    Ok(())
}

#[test]
fn test_committed_hook_runner_invokes_hook_all() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("hook-script")?;
    write_clean_hook_fixture(&root)?;
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_cairn"));
    let bin_dir = bin
        .parent()
        .ok_or("binary path has no parent directory")?
        .to_owned();
    let old_path = std::env::var_os("PATH").unwrap_or_default();
    let path = format!("{}:{}", bin_dir.display(), old_path.to_string_lossy());
    let hook_script = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/cairn-hook-all.sh");

    let output = Command::new("sh")
        .current_dir(&root)
        .env("PATH", path)
        .arg(&hook_script)
        .args(["--file", "cairn.blueprint"])
        .output()?;

    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)?.contains("Hook: all"));

    Ok(())
}

#[test]
fn test_divergence_intentional_asymmetry_ct002() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("divergence-asymmetry")?;
    fs::create_dir_all(root.join("src/rust"))?;
    fs::create_dir_all(root.join("src/ts"))?;
    fs::write(root.join("src/rust/lib.rs"), "pub fn api() {}\n")?;
    fs::write(root.join("src/ts/index.ts"), "export function api() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Multi "multi" id "app.multi" {
        path ["./src/rust/lib.rs", "./src/ts/index.ts"]
    }
}
"#,
    )?;
    fs::write(
        root.join("cairn.config.yaml"),
        r#"context: ""
rules: {}
multi_target:
  intentional_asymmetry:
    - node: app.multi
      contract_role: public_api
      targets:
        - src/rust/lib.rs
        - src/ts/index.ts
      reason: "The client intentionally exposes a narrowed interface."
"#,
    )?;

    let _config = scanner::config::load(&root)?;
    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;

    let ct002 = result.graph.findings.iter().find(|f| f.code == "CT002");
    assert!(
        ct002.is_some(),
        "Expected CT002 rationale tension finding, got: {:?}",
        result.graph.findings
    );
    assert!(
        ct002.unwrap().severity == cairn::map::FindingSeverity::Warning,
        "CT002 should be a Warning severity"
    );
    assert!(
        ct002.unwrap().message.contains("intentionally exposes"),
        "CT002 message should contain the reason"
    );

    let ct001 = result.graph.findings.iter().find(|f| f.code == "CT001");
    assert!(
        ct001.is_none(),
        "Should NOT have CT001 when asymmetry is intentional"
    );

    Ok(())
}

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn write_clean_hook_fixture(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src/auth"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
    }
}
"#,
    )?;
    Ok(())
}

fn write_change(
    root: &Path,
    id: &str,
    blueprint_delta: &str,
    artefacts: &[(&str, &str)],
) -> Result<(), Box<dyn std::error::Error>> {
    let change = root.join("meta/changes").join(id);
    fs::create_dir_all(&change)?;
    fs::write(change.join("proposal.md"), format!("# Proposal: {id}\n"))?;
    fs::write(change.join("blueprint.delta"), blueprint_delta)?;
    for (relative, content) in artefacts {
        let path = change.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    Ok(())
}

fn write_phase_2_fixture(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("src/auth"))?;
    fs::create_dir_all(root.join("src/store"))?;
    fs::create_dir_all(root.join("meta/contracts"))?;
    fs::create_dir_all(root.join("meta/todos"))?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::create_dir_all(root.join("meta/reviews"))?;
    fs::create_dir_all(root.join("meta/research"))?;
    fs::create_dir_all(root.join("meta/sources"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(root.join("src/store/lib.rs"), "pub fn save() {}\n")?;
    fs::write(root.join("meta/sources/auth.txt"), "evidence\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
        contract "./meta/contracts/auth.md"
        todos "./meta/todos"
        decisions "./meta/decisions"
        reviews "./meta/reviews"
        research "./meta/research"
        sources "./meta/sources"
    }
    Module Store "store" id "app.store" {
        path "./src/store"
    }
}
app.auth -> app.store "persists"
"#,
    )?;
    fs::write(
        root.join("meta/contracts/auth.md"),
        "---\nnode: app.auth\n---\n# Auth Contract\n",
    )?;
    fs::write(
        root.join("meta/todos/todo.md"),
        "---\nnode: app.auth\nstatus: open\ncreated: 2026-04-17\nsatisfies: login\n---\n# Todo\n",
    )?;
    fs::write(
        root.join("meta/decisions/dec.auth.md"),
        "---\nid: dec.auth\nnodes: [app.auth, app.store]\nstatus: accepted\ndate: 2026-04-17\nrevisited: 2026-04-17\nrevisit_triggers:\n  - \"Auth persistence changes\"\ninformed_by:\n  - type: research\n    id: res.auth\n  - type: source\n    id: src.auth\nsupersedes: [dec.old]\nrefines: []\nrelated: []\n---\n# Decision\n",
    )?;
    fs::write(
        root.join("meta/decisions/dec.old.md"),
        "---\nid: dec.old\nnodes: [app.auth]\nstatus: superseded\ndate: 2026-04-16\n---\n# Old Decision\n",
    )?;
    fs::write(
        root.join("meta/reviews/review.md"),
        "---\nnode: app.auth\ndate: 2026-04-17\nreviewer: george\nrelated_change: commit:abc\n---\n# Review\n",
    )?;
    fs::write(
        root.join("meta/research/res.auth.md"),
        "---\nid: res.auth\nnodes: [app.auth]\ndate: 2026-04-17\nsources:\n  - src.auth\ntags: [auth]\n---\n# Research\n",
    )?;
    fs::write(
        root.join("meta/sources/src.auth.md"),
        "---\nid: src.auth\nfile: ./meta/sources/auth.txt\nsha256: bdcf4c994585af6dd6cb1cfbff78bcc73ab27dc30a299db5bb83766ca05b5de4\nverification: verified\ntype: document\ndate: 2026-04-17\ntags: [auth]\ndescription: Auth evidence.\n---\n# Source\n",
    )?;
    Ok(())
}

#[test]
fn test_scan_exclusions_suppress_orphan_findings() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("scan-exclude")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::create_dir_all(root.join("generated/cache"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(
        root.join("generated/cache/output.rs"),
        "pub fn cached() {}\n",
    )?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
    }
}
"#,
    )?;

    let without_exclude = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(
        without_exclude
            .graph
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE"
                && f.message.contains("generated/cache/output.rs")),
        "orphan finding expected without exclusion"
    );

    fs::write(root.join("cairn.config.yaml"), "ignore:\n  - generated\n")?;

    fs::write(root.join("src/stray.rs"), "pub fn stray() {}\n")?;

    let with_exclude = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(
        !with_exclude
            .graph
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE"
                && f.message.contains("generated/cache/output.rs")),
        "orphan finding should be suppressed by ignore"
    );
    assert!(
        with_exclude
            .graph
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE" && f.message.contains("stray.rs")),
        "non-excluded orphan should still produce a finding"
    );

    Ok(())
}

#[test]
fn test_neighbourhood_renders_with_warning_despite_scan_errors()
-> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("neighbourhood-errors")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(root.join("src/orphan.rs"), "pub fn stray() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
    }
}
"#,
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(
        result.graph.has_errors(),
        "fixture should have orphan errors"
    );

    let text = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["neighbourhood", "app.auth"])
        .output()?;
    let stdout = String::from_utf8(text.stdout)?;
    assert!(
        text.status.success(),
        "neighbourhood should succeed with warnings"
    );
    assert!(stdout.contains("Node: app.auth"), "should render the node");
    assert!(
        stdout.contains("Warning: scan has"),
        "should include warning"
    );

    let json = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "neighbourhood", "app.auth"])
        .output()?;
    let json_stdout = String::from_utf8(json.stdout)?;
    assert!(
        json.status.success(),
        "JSON neighbourhood should succeed with warnings"
    );
    assert!(json_stdout.contains("\"node\""), "should render node JSON");
    assert!(
        json_stdout.contains("\"warnings\""),
        "JSON output should include warnings field"
    );

    Ok(())
}

#[test]
fn test_onboard_groups_orphans_and_classifies() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("onboard")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::create_dir_all(root.join("src/db"))?;
    fs::create_dir_all(root.join("generated/cache"))?;
    fs::write(root.join("src/auth/login.rs"), "pub fn login() {}\n")?;
    fs::write(root.join("src/auth/session.rs"), "pub fn session() {}\n")?;
    fs::write(root.join("src/db/pool.rs"), "pub fn pool() {}\n")?;
    fs::write(
        root.join("generated/cache/output.rs"),
        "pub fn cached() {}\n",
    )?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Core "core" id "app.core" {
        path "./src/core"
    }
}
"#,
    )?;

    let text = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["onboard"])
        .output()?;
    let stdout = String::from_utf8(text.stdout)?;
    assert!(text.status.success(), "onboard should succeed");
    assert!(
        stdout.contains("Suggested ignores"),
        "should suggest ignores for generated dir"
    );
    assert!(
        stdout.contains("Suggested blueprint nodes"),
        "should suggest nodes for source dirs"
    );
    assert!(
        stdout.contains("generated"),
        "should mention generated in ignore suggestions"
    );
    assert!(
        stdout.contains("src/auth") || stdout.contains("src.auth"),
        "should mention auth in node suggestions"
    );

    let json = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&root)
        .args(["--json", "onboard"])
        .output()?;
    let json_stdout = String::from_utf8(json.stdout)?;
    assert!(json.status.success(), "JSON onboard should succeed");
    let parsed: serde_json::Value = serde_json::from_str(&json_stdout)?;
    assert!(
        parsed["total_orphaned_files"].as_u64().unwrap() >= 4,
        "should find at least 4 orphaned files"
    );
    assert!(
        !parsed["ignore_suggestions"].as_array().unwrap().is_empty(),
        "should have ignore suggestions"
    );
    assert!(
        !parsed["node_suggestions"].as_array().unwrap().is_empty(),
        "should have node suggestions"
    );

    Ok(())
}

#[test]
fn test_provenance_lint_warns_for_uncovered_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("provenance-lint")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::create_dir_all(root.join("src/store"))?;
    fs::create_dir_all(root.join("meta/decisions"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(root.join("src/store/lib.rs"), "pub fn save() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
        decisions "./meta/decisions"
    }
    Module Store "store" id "app.store" {
        path "./src/store"
    }
}
"#,
    )?;
    fs::write(
        root.join("meta/decisions/dec.auth.md"),
        "---\nid: dec.auth\nnodes: [app.auth]\nstatus: accepted\ndate: 2026-05-11\n---\n# Auth decision\n",
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    let provenance_warnings: Vec<_> = result
        .graph
        .findings
        .iter()
        .filter(|f| f.code == "CAIRN_PROVENANCE_NO_DECISION")
        .collect();

    assert!(
        provenance_warnings
            .iter()
            .any(|f| f.node.as_deref() == Some("app.store")),
        "uncovered leaf node should get provenance warning"
    );
    assert!(
        !provenance_warnings
            .iter()
            .any(|f| f.node.as_deref() == Some("app.auth")),
        "covered node should NOT get provenance warning"
    );
    assert!(
        !provenance_warnings
            .iter()
            .any(|f| f.node.as_deref() == Some("app")),
        "parent node should NOT get provenance warning"
    );

    Ok(())
}

#[test]
fn test_provenance_lint_skips_when_no_decisions_exist() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("provenance-skip")?;
    fs::create_dir_all(root.join("src/auth"))?;
    fs::write(root.join("src/auth/lib.rs"), "pub fn login() {}\n")?;
    fs::write(
        root.join("cairn.blueprint"),
        r#"System App "desc" id "app" {
    Module Auth "auth" id "app.auth" {
        path "./src/auth"
    }
}
"#,
    )?;

    let result = scanner::load_project(&root, &root.join("cairn.blueprint"))?;
    assert!(
        !result
            .graph
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_PROVENANCE_NO_DECISION"),
        "should not warn when no decisions exist at all"
    );

    Ok(())
}
