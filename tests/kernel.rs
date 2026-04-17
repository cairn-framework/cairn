//! Phase 1 kernel integration tests.

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use cairn::{
    artefacts::contract,
    dsl::{lexer, parser},
    ontology::{build_graph, query},
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
fn test_parser_reports_location_for_malformed_dsl() {
    let error = parser::parse_str("bad.dsl", "System App \"desc\" id \"app\" {").unwrap_err();

    assert_eq!(error.span.file, "bad.dsl");
    assert!(error.message.contains("expected `}`"));
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
        root.join("cairn.dsl"),
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

    let result = scanner::load_project(&root, &root.join("cairn.dsl"))?;
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
        root.join("cairn.dsl"),
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
    assert!(root.join("index.md").exists());
    assert!(root.join(".cairn/log.md").exists());
    assert!(root.join(".cairn/state/interface-hashes.json").exists());

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
        root.join("cairn.dsl"),
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

    let default_result = scanner::load_project(&root, &root.join("cairn.dsl"))?;
    assert!(
        default_result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_RECONCILE_ORPHANED_FILE")
    );

    fs::write(
        root.join("cairn.dsl"),
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
    let opt_in_result = scanner::load_project(&root, &root.join("cairn.dsl"))?;
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
        root.join("cairn.dsl"),
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

    let result = scanner::load_project(&root, &root.join("cairn.dsl"))?;

    assert!(
        result
            .graph
            .findings
            .iter()
            .any(|finding| finding.code == "CAIRN_CONTRACT_UNKNOWN_NODE")
    );
    assert!(result.graph.findings.iter().any(|finding| {
        finding.code == "CAIRN_CONTRACT_MISSING"
            && finding.severity == cairn::ontology::FindingSeverity::Warning
    }));

    Ok(())
}

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
