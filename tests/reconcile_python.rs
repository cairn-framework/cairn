//! Integration tests for the Python code reconciler.
//!
//! Guards against silent regressions in tree-sitter-based symbol extraction:
//! Python public symbols are those whose names do not start with `_`.

use cairn::{
    blueprint::{Ast, NodeKind, Span, ast::Node},
    reconcile::{ReconcileRequest, Reconciler, python::PythonReconciler},
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

fn py_request<'a>(root: &'a std::path::Path, ignores: &'a [String]) -> ReconcileRequest<'a> {
    ReconcileRequest { root, ignores }
}

#[test]
fn test_py_public_function_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("api.py"),
        "def greet(name: str) -> str:\n    return name\n",
    )
    .unwrap();
    let ast = single_node_ast("app.api", ".");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("greet")),
        "public function 'greet' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_py_private_function_absent_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("helpers.py"),
        "def _internal():\n    pass\n",
    )
    .unwrap();
    let ast = single_node_ast("app.helpers", ".");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().all(|s| !s.contains("_internal")),
        "private function '_internal' must not appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_py_public_class_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("models.py"),
        "class UserRecord:\n    id: int = 0\n",
    )
    .unwrap();
    let ast = single_node_ast("app.models", ".");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("UserRecord")),
        "public class 'UserRecord' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_py_public_module_variable_appears_in_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("constants.py"), "VERSION = '1.0'\n").unwrap();
    let ast = single_node_ast("app.constants", ".");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().any(|s| s.contains("VERSION")),
        "public variable 'VERSION' must appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_py_private_module_variable_absent_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("secrets.py"), "_SECRET = 'hunter2'\n").unwrap();
    let ast = single_node_ast("app.secrets", ".");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.symbols.iter().all(|s| !s.contains("_SECRET")),
        "private variable '_SECRET' must not appear in symbols; got: {:?}",
        report.symbols
    );
}

#[test]
fn test_py_unowned_file_emits_orphaned_finding() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("orphan.py"),
        "def orphaned_fn():\n    pass\n",
    )
    .unwrap();
    let ast = single_node_ast("app.other", "other/");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.code == "CAIRN_RECONCILE_ORPHANED_FILE"),
        "unowned .py file must produce CAIRN_RECONCILE_ORPHANED_FILE; got: {:?}",
        report.findings
    );
}

#[test]
fn test_py_owned_file_in_claimed_files() {
    let dir = tempfile::tempdir().unwrap();
    let pkg = dir.path().join("service");
    fs::create_dir(&pkg).unwrap();
    fs::write(pkg.join("run.py"), "def run():\n    pass\n").unwrap();
    let ast = single_node_ast("app.service", "service");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap();
    assert!(
        report.claimed_files.contains_key("app.service"),
        "node 'app.service' must appear in claimed_files; got: {:?}",
        report.claimed_files.keys().collect::<Vec<_>>()
    );
}

#[test]
fn test_py_fingerprint_changes_on_new_public_symbol() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("svc.py");
    fs::write(&file, "def alpha():\n    pass\n").unwrap();
    let ast = single_node_ast("app.svc", ".");
    let fp1 = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap()
        .fingerprint;
    fs::write(&file, "def alpha():\n    pass\ndef beta():\n    pass\n").unwrap();
    let fp2 = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &[]))
        .unwrap()
        .fingerprint;
    assert_ne!(
        fp1, fp2,
        "adding a public function must change the fingerprint"
    );
}

#[test]
fn test_py_ignored_file_excluded_from_symbols() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("generated.py"),
        "def should_be_ignored():\n    pass\n",
    )
    .unwrap();
    let ast = single_node_ast("app.gen", ".");
    let report = PythonReconciler::new(&ast)
        .reconcile(py_request(dir.path(), &["generated.py".to_owned()]))
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
