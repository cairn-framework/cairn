//! Byte-identical equivalence between the generic `CodeReconciler` and the
//! committed per-language baselines captured from the pre-refactor reconcilers.
//!
//! Each baseline under `tests/fixtures/reconcile_baseline/<lang>/<lang>.baseline.json`
//! was serialised by the ORIGINAL reconcilers. The generic reconciler must
//! reproduce that exact `ReconcileReport` so the refactor is behaviour-neutral.
//!
//! The final test guards the spec-only pipeline boundary: an alternate
//! `LanguageSpec` drives the shared pipeline with no dedicated module.

use std::{
    fs,
    path::{Path, PathBuf},
};

use cairn::{
    blueprint::{Ast, NodeKind, Span, ast::Node},
    reconcile::{
        CodeReconciler, LanguageSpec, ReconcileReport, ReconcileRequest, Reconciler, SymbolKind,
        spec_for, target::Language,
    },
};

fn fixture_root(lang: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/reconcile_baseline")
        .join(lang)
}

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

fn reconcile(dir: &str, lang: Language, node_id: &str, node_path: &str) -> ReconcileReport {
    let root = fixture_root(dir);
    let ast = single_node_ast(node_id, node_path);
    CodeReconciler::new(&ast, spec_for(lang).unwrap())
        .reconcile(ReconcileRequest {
            root: &root,
            ignores: &[],
        })
        .expect("reconcile")
}

fn assert_matches_baseline(dir: &str, report: &ReconcileReport) {
    let expected = fs::read_to_string(fixture_root(dir).join(format!("{dir}.baseline.json")))
        .expect("read baseline");
    let actual = serde_json::to_string_pretty(report).expect("serialize report");
    assert_eq!(
        actual, expected,
        "generic reconciler diverged from baseline for {dir}"
    );
}

/// The committed Rust fixtures carry a `.fixture` suffix so the repo's own
/// self-host scan (which claims every `.rs` under the blueprint-owned
/// `./tests` tree) cannot pick them up. Materialise them as real `.rs`
/// files in a tempdir before reconciling.
fn materialise_rust_fixtures(dest: &Path) {
    let src_root = fixture_root("rust");
    for rel in ["src/lib.rs", "orphan.rs"] {
        let from = src_root.join(format!("{rel}.fixture"));
        let to = dest.join(rel);
        fs::create_dir_all(to.parent().expect("parent")).expect("mkdir");
        fs::copy(&from, &to).expect("copy fixture");
    }
}

#[test]
fn generic_matches_rust_baseline() {
    let dir = std::env::temp_dir().join(format!(
        "cairn-reconcile-baseline-rust-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    materialise_rust_fixtures(&dir);
    let ast = single_node_ast("app.lib", "src");
    let report = CodeReconciler::new(&ast, spec_for(Language::Rust).unwrap())
        .reconcile(ReconcileRequest {
            root: &dir,
            ignores: &[],
        })
        .expect("reconcile");
    fs::remove_dir_all(&dir).ok();
    assert_matches_baseline("rust", &report);
}

#[test]
fn generic_matches_typescript_baseline() {
    let report = reconcile("typescript", Language::TypeScript, "app.api", "src");
    assert_matches_baseline("typescript", &report);
}

#[test]
fn generic_matches_python_baseline() {
    let report = reconcile("python", Language::Python, "app.api", ".");
    assert_matches_baseline("python", &report);
}

#[test]
fn generic_matches_go_baseline() {
    let report = reconcile("go", Language::Go, "app.api", ".");
    assert_matches_baseline("go", &report);
}

// --- Cheap-extension guarantee: an ALTERNATE spec for an existing language needs
// no dedicated module. (A brand-new language still requires a `Language` variant
// and registry entry; this guards the spec-only pipeline boundary.) ---

fn demo_name_and_kind(node: tree_sitter::Node<'_>, source: &[u8]) -> (String, SymbolKind) {
    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("left"))
        .and_then(|n| n.utf8_text(source).ok())
        .unwrap_or_default()
        .to_owned();
    (name, SymbolKind::Other)
}

fn demo_is_exportable(node: tree_sitter::Node<'_>, _source: &[u8]) -> bool {
    matches!(
        node.kind(),
        "function_definition" | "class_definition" | "assignment"
    )
}

#[must_use]
fn demo_interface_symbol(node: tree_sitter::Node<'_>, source: &[u8]) -> String {
    let kind = node.kind();
    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("left"))
        .and_then(|n| n.utf8_text(source).ok())
        .map_or_else(
            || node.utf8_text(source).unwrap_or("").to_owned(),
            str::to_owned,
        );
    format!("{kind}:{name}")
}

static DEMO: LanguageSpec = LanguageSpec {
    language: Language::Python,
    display_name: "Demo",
    grammar: || tree_sitter_python::LANGUAGE.into(),
    extensions: &["py"],
    exportable_kinds: &["function_definition", "class_definition", "assignment"],
    name_and_kind: demo_name_and_kind,
    interface_symbol: demo_interface_symbol,
    is_exportable: demo_is_exportable,
    fast_path: true,
    grammar_error_code: "CAIRN_RECONCILE_DEMO_LANGUAGE",
    parse_error_code: "CAIRN_RECONCILE_PARSE_DEMO",
};

#[test]
fn alternate_spec_for_existing_language_needs_no_new_module() {
    let root = fixture_root("python");
    let ast = single_node_ast("app.api", ".");
    let report = CodeReconciler::new(&ast, &DEMO)
        .reconcile(ReconcileRequest {
            root: &root,
            ignores: &[],
        })
        .expect("reconcile demo spec");

    assert!(
        !report.symbols.is_empty(),
        "a LanguageSpec alone (no dedicated module) must extract symbols"
    );
}
