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

#[test]
fn test_cli_blocks_queries_on_structural_errors() -> Result<(), Box<dyn std::error::Error>> {
    let root = temp_root("structural-cli")?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn lib() {}\n")?;
    fs::write(
        root.join("cairn.dsl"),
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
        root.join("cairn.dsl"),
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

    let result = scanner::load_project(&root, &root.join("cairn.dsl"))?;
    assert!(result.graph.findings.iter().any(|finding| {
        finding.code == "CAIRN_CONTRACT_WRONG_NODE"
            && finding.node.as_deref() == Some("app.a")
            && finding.severity == cairn::ontology::FindingSeverity::Error
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

    let result = scanner::load_project(&root, &root.join("cairn.dsl"))?;

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

    let result = scanner::load_project(&root, &root.join("cairn.dsl"))?;

    assert!(result.graph.findings.iter().any(|finding| {
        finding.code == "CAIRN_SOURCE_SHA256_MISMATCH"
            && finding.severity == cairn::ontology::FindingSeverity::Error
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

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
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
        root.join("cairn.dsl"),
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
