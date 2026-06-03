//! Integration tests for the Rust code reconciler.
//!
//! Guards against silent regressions in tree-sitter-based symbol extraction.
//! Rust public items are identified by the presence of a `visibility_modifier`
//! (`pub`, `pub(crate)`, etc.) child node.

use cairn::{
    blueprint::{Ast, NodeKind, Span, ast::Node},
    reconcile::{ReconcileRequest, Reconciler, code::RustCodeReconciler},
};
use std::fs;

fn single_node_ast(node_id: &str, path: &str) -> Ast {
    Ast {
        nodes: vec![Node {
            kind: NodeKind::Module,
            name: node_id.to_owned(),
            description: String::new(),
            id: node_id.to_owned(),
            tags: Vec::new(),
            paths: vec![path.to_owned()],
            owns_files: false,
            contracts: Vec::new(),
            raw_fields: Vec::new(),
            children: Vec::new(),
            span: Span::point("cairn.blueprint", 1, 1),
        }],
        edges: Vec::new(),
    }
}

fn rs_request<'a>(root: &'a std::path::Path, ignores: &'a [String]) -> ReconcileRequest<'a> {
    ReconcileRequest { root, ignores }
}

#[test]
fn test_rust_pub_fn_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("lib.rs"),
        "pub fn greet(name: &str) -> String { name.to_owned() }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.lib", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("greet")),
        "pub fn 'greet' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_rust_private_fn_absent_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), "fn internal() {}\n").unwrap();
    let ast = single_node_ast("app.lib", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().all(|s| !s.contains("internal")),
        "private fn 'internal' must not appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_rust_pub_struct_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("types.rs"),
        "pub struct Config { pub timeout: u64 }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.types", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("Config")),
        "pub struct 'Config' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_rust_pub_enum_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("status.rs"),
        "pub enum Status { Active, Inactive }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.status", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("Status")),
        "pub enum 'Status' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_rust_pub_trait_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("processor.rs"),
        "pub trait Processor { fn process(&self); }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.processor", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("Processor")),
        "pub trait 'Processor' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_rust_pub_const_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("limits.rs"),
        "pub const MAX_RETRIES: usize = 5;\n",
    )
    .unwrap();
    let ast = single_node_ast("app.limits", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("MAX_RETRIES")),
        "pub const 'MAX_RETRIES' must appear in symbols; got: {:?}",
        report.symbols
    );
}

/// The `const` keyword must appear in the symbol, not just the identifier,
/// so that `pub const X` and `pub static X` can be distinguished in the
/// interface fingerprint.
#[test]
fn test_rust_pub_const_symbol_contains_const_keyword() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("limits.rs"),
        "pub const VERSION: &str = \"1\";\n",
    )
    .unwrap();
    let ast = single_node_ast("app.limits", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    let sym = report
        .symbols
        .iter()
        .find(|s| s.contains("VERSION"))
        .cloned()
        .unwrap_or_default();
    assert!(
        sym.contains("const"),
        "pub const symbol must contain 'const' keyword; got: {sym:?}"
    );
}

/// Same as above for `static`.
#[test]
fn test_rust_pub_static_symbol_contains_static_keyword() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("globals.rs"),
        "pub static COUNTER: u32 = 0;\n",
    )
    .unwrap();
    let ast = single_node_ast("app.globals", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    let sym = report
        .symbols
        .iter()
        .find(|s| s.contains("COUNTER"))
        .cloned()
        .unwrap_or_default();
    assert!(
        sym.contains("static"),
        "pub static symbol must contain 'static' keyword; got: {sym:?}"
    );
}

#[test]
fn test_rust_unowned_file_emits_orphaned_finding() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("orphan.rs"), "pub fn orphaned() {}\n").unwrap();
    let ast = single_node_ast("app.other", "other/");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE"),
        "unowned .rs file must produce CAIRN_RECONCILE_ORPHANED_FILE; got: {:?}",
        report.findings
    );
}

#[test]
fn test_rust_owned_file_in_claimed_files() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn run() {}\n").unwrap();
    let ast = single_node_ast("app.core", "src");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.claimed_files.contains_key("app.core"),
        "node 'app.core' must appear in claimed_files; got: {:?}",
        report.claimed_files.keys().collect::<Vec<_>>()
    );
}

#[test]
fn test_rust_fingerprint_changes_on_new_pub_fn() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("api.rs");
    fs::write(&file, "pub fn alpha() {}\n").unwrap();
    let ast = single_node_ast("app.api", ".");
    let fp1 = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap()
        .fingerprint;
    fs::write(&file, "pub fn alpha() {}\npub fn beta() {}\n").unwrap();
    let fp2 = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &[]))
        .unwrap()
        .fingerprint;
    assert_ne!(fp1, fp2, "adding pub fn must change the fingerprint");
}

#[test]
fn test_rust_ignored_file_excluded_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("generated.rs"),
        "pub fn should_be_ignored() {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.gen", ".");
    let report = RustCodeReconciler::new(&ast)
        .reconcile(rs_request(dir.path(), &["generated.rs".to_owned()]))
        .unwrap();
    assert!(
        report
            .symbols
            .iter()
            .all(|s| !s.contains("should_be_ignored")),
        "ignored file symbols must not appear; got: {:?}",
        report.symbols
    );
}
