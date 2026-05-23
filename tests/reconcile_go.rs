//! Integration tests for the Go code reconciler.
//!
//! Guards against silent regressions in tree-sitter-based symbol extraction:
//! Go exported symbols are identified by an uppercase first character.

use cairn::{
    blueprint::{Ast, NodeKind, Span, ast::Node},
    reconcile::{ReconcileRequest, Reconciler, go::GoReconciler},
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

fn go_request<'a>(root: &'a std::path::Path, ignores: &'a [String]) -> ReconcileRequest<'a> {
    ReconcileRequest { root, ignores }
}

#[test]
fn test_go_exported_function_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("handler.go"),
        "package api\n\nfunc Greet(name string) string { return name }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("Greet")),
        "exported func 'Greet' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_go_unexported_function_absent_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("helpers.go"),
        "package api\n\nfunc helper() {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().all(|s| !s.contains("helper")),
        "unexported func 'helper' must not appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_go_exported_type_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("types.go"),
        "package types\n\ntype UserRecord struct { ID int }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.types", ".");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("UserRecord")),
        "exported type 'UserRecord' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_go_exported_const_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("version.go"),
        "package version\n\nconst Version = \"1.0\"\n",
    )
    .unwrap();
    let ast = single_node_ast("app.version", ".");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("Version")),
        "exported const 'Version' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_go_unowned_file_emits_orphaned_finding() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("orphan.go"),
        "package orphan\n\nfunc Orphaned() {}\n",
    )
    .unwrap();
    // AST node claims a path that doesn't match the file.
    let ast = single_node_ast("app.other", "other/");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE"),
        "unowned .go file must produce CAIRN_RECONCILE_ORPHANED_FILE; got: {:?}",
        report.findings
    );
}

#[test]
fn test_go_owned_file_in_claimed_files() {
    let dir = tempfile::tempdir().unwrap();
    let pkg = dir.path().join("server");
    fs::create_dir(&pkg).unwrap();
    fs::write(pkg.join("main.go"), "package server\n\nfunc Run() {}\n").unwrap();
    let ast = single_node_ast("app.server", "server");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.claimed_files.contains_key("app.server"),
        "node 'app.server' must appear in claimed_files; got: {:?}",
        report.claimed_files.keys().collect::<Vec<_>>()
    );
}

#[test]
fn test_go_fingerprint_changes_on_new_export() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("api.go");
    fs::write(&file, "package api\n\nfunc Alpha() {}\n").unwrap();
    let ast = single_node_ast("app.api", ".");
    let fp1 = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap()
        .fingerprint;
    fs::write(&file, "package api\n\nfunc Alpha() {}\nfunc Beta() {}\n").unwrap();
    let fp2 = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &[]))
        .unwrap()
        .fingerprint;
    assert_ne!(fp1, fp2, "adding exported func must change the fingerprint");
}

#[test]
fn test_go_ignored_file_excluded_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("generated.go"),
        "package gen\n\nfunc ShouldBeIgnored() {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.gen", ".");
    let report = GoReconciler::new(&ast)
        .reconcile(go_request(dir.path(), &["generated.go".to_owned()]))
        .unwrap();
    assert!(
        report
            .symbols
            .iter()
            .all(|s| !s.contains("ShouldBeIgnored")),
        "ignored file symbols must not appear; got: {:?}",
        report.symbols
    );
}
