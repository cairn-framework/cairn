//! Integration tests for the TypeScript code reconciler.
//!
//! Guards against silent regressions in tree-sitter-based symbol extraction
//! and file-to-node ownership mapping (issue: zero prior coverage).

use cairn::{
    blueprint::{Ast, NodeKind, Span, ast::Node},
    reconcile::{ReconcileRequest, Reconciler, SymbolKind, typescript::TypeScriptReconciler},
};
use std::fs;

/// Minimal blueprint AST with one node that owns the given path prefix.
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

#[test]
fn test_exported_function_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("api.ts"),
        "export function greet(name: string): string { return name; }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("greet")),
        "exported function 'greet' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_unexported_function_absent_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("helpers.ts"),
        "function internal(): void {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.helpers", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    assert!(
        report.symbols.iter().all(|s| !s.contains("internal")),
        "unexported function 'internal' must not appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_exported_interface_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("types.ts"),
        "export interface UserRecord { id: number; name: string; }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.types", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("UserRecord")),
        "exported interface 'UserRecord' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_exported_type_alias_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("aliases.ts"),
        "export type UserId = string;\n",
    )
    .unwrap();
    let ast = single_node_ast("app.aliases", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("UserId")),
        "exported type alias 'UserId' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_unowned_ts_file_emits_orphaned_file_finding() {
    let dir = tempfile::tempdir().unwrap();
    // Write a file that no node claims.
    fs::write(dir.path().join("unowned.ts"), "export const x = 1;\n").unwrap();
    // AST with a node that claims a different path.
    let ast = single_node_ast("app.other", "other/");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE"),
        "unowned .ts file must produce CAIRN_RECONCILE_ORPHANED_FILE; got: {:?}",
        report.findings
    );
}

#[test]
fn test_owned_file_is_in_claimed_files() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir(&src).unwrap();
    fs::write(src.join("index.ts"), "export const VERSION = '1';\n").unwrap();
    let ast = single_node_ast("app.core", "src");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    assert!(
        report.claimed_files.contains_key("app.core"),
        "node 'app.core' must appear in claimed_files; got: {:?}",
        report.claimed_files.keys().collect::<Vec<_>>()
    );
}

#[test]
fn test_fingerprint_changes_when_symbols_change() {
    let dir = tempfile::tempdir().unwrap();

    fs::write(
        dir.path().join("v1.ts"),
        "export function alpha(): void {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.svc", ".");
    let fp1 = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap()
        .fingerprint;

    fs::write(
        dir.path().join("v1.ts"),
        "export function alpha(): void {}\nexport function beta(): void {}\n",
    )
    .unwrap();
    let fp2 = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap()
        .fingerprint;

    assert_ne!(
        fp1, fp2,
        "adding a new export must change the interface fingerprint"
    );
}

#[test]
fn test_ignored_ts_file_is_not_reconciled() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("dist.ts"),
        "export function shouldBeIgnored(): void {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.dist", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &["dist.ts".to_owned()],
        })
        .unwrap();
    assert!(
        report
            .symbols
            .iter()
            .all(|s| !s.contains("shouldBeIgnored")),
        "ignored file symbols must not appear; got: {:?}",
        report.symbols
    );
}

/// ── Symbol record tests (Phase 1) ──────────────────────────────────────────

#[test]
fn test_ts_fn_symbol_record_has_correct_fields() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("api.ts"),
        "export function greet(name: string): string { return name; }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    let records = report
        .node_symbol_records
        .get("app.api")
        .expect("node_symbol_records must contain app.api");
    assert_eq!(records.len(), 1, "exactly one record expected");
    let rec = &records[0];
    assert_eq!(rec.name, "greet", "name must be 'greet'");
    assert_eq!(rec.kind, SymbolKind::Function, "kind must be Function");
    assert_eq!(rec.line, 1, "start line must be 1");
    assert!(
        rec.signature.contains("greet"),
        "signature must contain 'greet'; got: {:?}",
        rec.signature
    );
}

#[test]
fn test_ts_interface_record_has_interface_kind() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("api.ts"),
        "export interface Config { timeout: number; }\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    let records = report.node_symbol_records.get("app.api").unwrap();
    let rec = records
        .iter()
        .find(|r| r.name == "Config")
        .expect("record for Config must exist");
    assert_eq!(rec.kind, SymbolKind::Interface);
}

#[test]
fn test_ts_symbol_record_signature_matches_fingerprint_string() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("api.ts"),
        "export function alpha(): void {}\nexport function beta(): void {}\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = TypeScriptReconciler::new(&ast)
        .reconcile(ReconcileRequest {
            root: dir.path(),
            ignores: &[],
        })
        .unwrap();
    let mut sig_from_records: Vec<String> = report
        .node_symbol_records
        .get("app.api")
        .unwrap()
        .iter()
        .map(|r| r.signature.clone())
        .collect();
    sig_from_records.sort_unstable();
    let mut sig_from_flat: Vec<String> = report
        .node_symbols
        .get("app.api")
        .unwrap()
        .iter()
        .cloned()
        .collect();
    sig_from_flat.sort_unstable();
    assert_eq!(
        sig_from_records, sig_from_flat,
        "record signatures must match the flat symbol strings fed to the fingerprint"
    );
}
