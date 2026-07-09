//! Python language specification for the generic code reconciler.

use tree_sitter::Node;

use crate::reconcile::{
    generic::LanguageSpec,
    symbol::{SymbolKind, normalize_symbol},
    target::Language,
};

/// tree-sitter node kinds eligible for symbol extraction in Python.
pub const PYTHON_ITEM_KINDS: &[&str] = &["function_definition", "class_definition", "assignment"];

/// Python language specification driving [`crate::reconcile::CodeReconciler`].
pub static PYTHON: LanguageSpec = LanguageSpec {
    language: Language::Python,
    display_name: "Python",
    grammar: || tree_sitter_python::LANGUAGE.into(),
    extensions: &["py"],
    exportable_kinds: PYTHON_ITEM_KINDS,
    name_and_kind: py_name_and_kind,
    interface_symbol: py_interface_symbol,
    is_exportable: py_is_exportable,
    fast_path: false,
    grammar_error_code: "CAIRN_RECONCILE_PYTHON_LANGUAGE",
    parse_error_code: "CAIRN_RECONCILE_PARSE_PYTHON",
};

fn py_symbol_kind(ts_kind: &str) -> SymbolKind {
    match ts_kind {
        "function_definition" => SymbolKind::Function,
        "class_definition" => SymbolKind::Class,
        "assignment" => SymbolKind::Variable,
        _ => SymbolKind::Other,
    }
}

fn py_name_and_kind(node: Node<'_>, source: &[u8]) -> (String, SymbolKind) {
    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("left"))
        .and_then(|n| n.utf8_text(source).ok())
        .unwrap_or_default()
        .to_owned();
    (name, py_symbol_kind(node.kind()))
}

fn py_is_exportable(node: Node<'_>, source: &[u8]) -> bool {
    if !PYTHON_ITEM_KINDS.contains(&node.kind()) {
        return false;
    }
    // `__all__` makes every top-level definition public regardless of naming.
    if std::str::from_utf8(source).map_or(false, |s| s.contains("__all__")) {
        return true;
    }
    // function_definition and class_definition use `name`; assignment uses `left`.
    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("left"))
        .and_then(|n| n.utf8_text(source).ok());
    name.is_some_and(|n| !n.starts_with('_'))
}

#[must_use]
fn py_interface_symbol(node: Node<'_>, source: &[u8]) -> String {
    let kind = node.kind();
    // assignment nodes have no `name` field; fall back to `left` (the target).
    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("left"))
        .and_then(|n| n.utf8_text(source).ok())
        .map_or_else(
            || node.utf8_text(source).unwrap_or("").to_owned(),
            str::to_owned,
        );
    let signature = format!("{kind}:{name}");
    normalize_symbol(&signature)
}
