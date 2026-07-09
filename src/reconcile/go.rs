//! Go language specification for the generic code reconciler.

use tree_sitter::Node;

use crate::reconcile::{
    generic::LanguageSpec,
    symbol::{SymbolKind, normalize_symbol},
    target::Language,
};

/// tree-sitter node kinds eligible for symbol extraction in Go.
pub const GO_EXPORTABLE_KINDS: &[&str] = &[
    "function_declaration",
    "type_spec",
    "method_declaration",
    "const_spec",
    "var_spec",
];

/// Go language specification driving [`crate::reconcile::CodeReconciler`].
pub static GO: LanguageSpec = LanguageSpec {
    language: Language::Go,
    display_name: "Go",
    grammar: || tree_sitter_go::LANGUAGE.into(),
    extensions: &["go"],
    exportable_kinds: GO_EXPORTABLE_KINDS,
    name_and_kind: go_name_and_kind,
    interface_symbol: go_interface_symbol,
    is_exportable: go_is_exportable,
    fast_path: false,
    grammar_error_code: "CAIRN_RECONCILE_GO_LANGUAGE",
    parse_error_code: "CAIRN_RECONCILE_PARSE_GO",
};

fn go_symbol_kind(ts_kind: &str) -> SymbolKind {
    match ts_kind {
        "function_declaration" | "method_declaration" => SymbolKind::Function,
        "type_spec" => SymbolKind::Type,
        "const_spec" => SymbolKind::Const,
        "var_spec" => SymbolKind::Variable,
        _ => SymbolKind::Other,
    }
}

fn go_name_and_kind(node: Node<'_>, source: &[u8]) -> (String, SymbolKind) {
    let name = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(source).ok())
        .unwrap_or_default()
        .to_owned();
    (name, go_symbol_kind(node.kind()))
}

fn go_is_exportable(node: Node<'_>, source: &[u8]) -> bool {
    if !GO_EXPORTABLE_KINDS.contains(&node.kind()) {
        return false;
    }
    node.child_by_field_name("name").is_some_and(|n| {
        let text = n.utf8_text(source).unwrap_or("").to_owned();
        text.chars().next().is_some_and(char::is_uppercase)
    })
}

#[must_use]
fn go_interface_symbol(node: Node<'_>, source: &[u8]) -> String {
    let kind = node.kind();
    let name = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(source).ok())
        .map_or_else(
            || node.utf8_text(source).unwrap_or("").to_owned(),
            str::to_owned,
        );
    let signature = format!("{kind}:{name}");
    normalize_symbol(&signature)
}
