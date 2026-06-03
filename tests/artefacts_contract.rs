//! Integration tests for the contract artefact loader (`src/artefacts/contract.rs`).
//!
//! `load_contracts` is called on every `cairn scan`; it reads contract Markdown
//! files and validates their `node:` frontmatter against the blueprint AST.

use std::fs;

use cairn::{
    artefacts::contract::load_contracts,
    blueprint::{Ast, NodeKind, Span, ast::Node},
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_ast(node_id: &str, contracts: &[&str]) -> Ast {
    Ast {
        nodes: vec![Node {
            kind: NodeKind::Module,
            name: node_id.to_owned(),
            description: String::new(),
            id: node_id.to_owned(),
            tags: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: contracts.iter().map(ToString::to_string).collect(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("cairn.blueprint", 1, 1),
        }],
        edges: Vec::new(),
    }
}

fn contract_content(node_id: &str, body: &str) -> String {
    format!("---\nnode: {node_id}\n---\n{body}")
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_happy_path_contract_loads_correctly() {
    let dir = tempfile::tempdir().unwrap();
    let meta = dir.path().join("meta/contracts");
    fs::create_dir_all(&meta).unwrap();
    fs::write(
        meta.join("api.md"),
        contract_content("app.api", "## Interface\nFoo does bar."),
    )
    .unwrap();

    let ast = make_ast("app.api", &["meta/contracts/api.md"]);
    let set = load_contracts(dir.path(), &ast);

    assert!(
        set.findings.is_empty(),
        "valid contract must produce no findings; got: {:?}",
        set.findings
    );
    assert!(
        set.contracts.contains_key("meta/contracts/api.md"),
        "contract must be indexed by its declared path"
    );
    let contract = &set.contracts["meta/contracts/api.md"];
    assert_eq!(contract.node, "app.api");
    assert_eq!(contract.declared_by, "app.api");
    assert!(contract.body.contains("Foo does bar."));
}

#[test]
fn test_missing_contract_file_is_silently_skipped() {
    // NotFound is not an error — the contract is simply absent.
    let dir = tempfile::tempdir().unwrap();
    let ast = make_ast("app.api", &["meta/contracts/missing.md"]);
    let set = load_contracts(dir.path(), &ast);

    assert!(
        set.findings.is_empty(),
        "missing contract file must produce no findings"
    );
    assert!(
        set.contracts.is_empty(),
        "missing contract file must not produce a contract entry"
    );
}

#[test]
fn test_contract_missing_node_frontmatter_emits_error() {
    let dir = tempfile::tempdir().unwrap();
    let meta = dir.path().join("meta/contracts");
    fs::create_dir_all(&meta).unwrap();
    // No `node:` key in frontmatter.
    fs::write(meta.join("api.md"), "---\ntitle: orphan\n---\nbody").unwrap();

    let ast = make_ast("app.api", &["meta/contracts/api.md"]);
    let set = load_contracts(dir.path(), &ast);

    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_CONTRACT_MISSING_NODE"),
        "contract without node: field must emit CAIRN_CONTRACT_MISSING_NODE; got: {:?}",
        set.findings
    );
}

#[test]
fn test_contract_unknown_node_emits_error() {
    let dir = tempfile::tempdir().unwrap();
    let meta = dir.path().join("meta/contracts");
    fs::create_dir_all(&meta).unwrap();
    // References a node ID that doesn't exist in the AST.
    fs::write(meta.join("api.md"), contract_content("app.ghost", "")).unwrap();

    // AST only knows about app.api, not app.ghost.
    let ast = make_ast("app.api", &["meta/contracts/api.md"]);
    let set = load_contracts(dir.path(), &ast);

    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_CONTRACT_UNKNOWN_NODE"),
        "contract referencing unknown node must emit CAIRN_CONTRACT_UNKNOWN_NODE; got: {:?}",
        set.findings
    );
}

#[test]
fn test_contract_wrong_node_emits_error() {
    let dir = tempfile::tempdir().unwrap();
    let meta = dir.path().join("meta/contracts");
    fs::create_dir_all(&meta).unwrap();
    // app.api declares this contract but the contract says node: app.db.
    // Both app.api and app.db exist in the AST.
    let ast = Ast {
        nodes: vec![
            Node {
                kind: NodeKind::Module,
                name: "api".to_owned(),
                description: String::new(),
                id: "app.api".to_owned(),
                tags: Vec::new(),
                paths: Vec::new(),
                owns_files: false,
                contracts: vec!["meta/contracts/api.md".to_owned()],
                raw_fields: Vec::new(),
                children: Vec::new(),
                span: Span::point("cairn.blueprint", 1, 1),
            },
            Node {
                kind: NodeKind::Module,
                name: "db".to_owned(),
                description: String::new(),
                id: "app.db".to_owned(),
                tags: Vec::new(),
                paths: Vec::new(),
                owns_files: false,
                contracts: Vec::new(),
                raw_fields: Vec::new(),
                children: Vec::new(),
                span: Span::point("cairn.blueprint", 2, 1),
            },
        ],
        edges: Vec::new(),
    };
    // Contract says node: app.db but it was declared by app.api.
    fs::write(meta.join("api.md"), contract_content("app.db", "")).unwrap();

    let set = load_contracts(dir.path(), &ast);

    assert!(
        set.findings
            .iter()
            .any(|f| f.code == "CAIRN_CONTRACT_WRONG_NODE"),
        "contract declared by app.api but pointing to app.db must emit CAIRN_CONTRACT_WRONG_NODE; got: {:?}",
        set.findings
    );
}

#[test]
fn test_node_with_no_contracts_produces_no_findings() {
    let dir = tempfile::tempdir().unwrap();
    // Node declares no contracts.
    let ast = make_ast("app.api", &[]);
    let set = load_contracts(dir.path(), &ast);
    assert!(set.findings.is_empty());
    assert!(set.contracts.is_empty());
}

#[test]
fn test_contract_body_is_preserved() {
    let dir = tempfile::tempdir().unwrap();
    let meta = dir.path().join("meta/contracts");
    fs::create_dir_all(&meta).unwrap();
    let body = "## Interface\n\nExposes `fn process(input: &str) -> Result<Output>`.\n";
    fs::write(meta.join("api.md"), contract_content("app.api", body)).unwrap();

    let ast = make_ast("app.api", &["meta/contracts/api.md"]);
    let set = load_contracts(dir.path(), &ast);

    let contract = &set.contracts["meta/contracts/api.md"];
    assert!(
        contract.body.contains("fn process"),
        "contract body must be preserved verbatim; got: {:?}",
        contract.body
    );
}

#[test]
fn test_multiple_contracts_on_different_nodes() {
    let dir = tempfile::tempdir().unwrap();
    let meta = dir.path().join("meta/contracts");
    fs::create_dir_all(&meta).unwrap();
    fs::write(
        meta.join("api.md"),
        contract_content("app.api", "API contract"),
    )
    .unwrap();
    fs::write(
        meta.join("db.md"),
        contract_content("app.db", "DB contract"),
    )
    .unwrap();

    let ast = Ast {
        nodes: vec![
            Node {
                kind: NodeKind::Module,
                name: "api".to_owned(),
                description: String::new(),
                id: "app.api".to_owned(),
                tags: Vec::new(),
                paths: Vec::new(),
                owns_files: false,
                contracts: vec!["meta/contracts/api.md".to_owned()],
                raw_fields: Vec::new(),
                children: Vec::new(),
                span: Span::point("cairn.blueprint", 1, 1),
            },
            Node {
                kind: NodeKind::Module,
                name: "db".to_owned(),
                description: String::new(),
                id: "app.db".to_owned(),
                tags: Vec::new(),
                paths: Vec::new(),
                owns_files: false,
                contracts: vec!["meta/contracts/db.md".to_owned()],
                raw_fields: Vec::new(),
                children: Vec::new(),
                span: Span::point("cairn.blueprint", 2, 1),
            },
        ],
        edges: Vec::new(),
    };

    let set = load_contracts(dir.path(), &ast);

    assert!(
        set.findings.is_empty(),
        "valid contracts must produce no findings"
    );
    assert_eq!(set.contracts.len(), 2);
    assert!(set.contracts.contains_key("meta/contracts/api.md"));
    assert!(set.contracts.contains_key("meta/contracts/db.md"));
}
