//! Rust language specification for the generic code reconciler.

use tree_sitter::Node;

use crate::reconcile::{
    generic::LanguageSpec,
    symbol::{SymbolKind, normalize_symbol},
    target::Language,
};

/// tree-sitter node kinds eligible for symbol extraction in Rust.
pub const PUBLIC_ITEM_KINDS: &[&str] = &[
    "function_item",
    "struct_item",
    "enum_item",
    "trait_item",
    "type_item",
    "const_item",
    "static_item",
    "mod_item",
    "union_item",
];

/// Rust language specification driving [`crate::reconcile::CodeReconciler`].
pub static RUST: LanguageSpec = LanguageSpec {
    language: Language::Rust,
    display_name: "Rust",
    grammar: || tree_sitter_rust::LANGUAGE.into(),
    extensions: &["rs"],
    exportable_kinds: PUBLIC_ITEM_KINDS,
    name_and_kind: rust_name_and_kind,
    interface_symbol: rust_interface_symbol,
    is_exportable: rust_is_exportable,
    fast_path: true,
    grammar_error_code: "CAIRN_RECONCILE_RUST_LANGUAGE",
    parse_error_code: "CAIRN_RECONCILE_PARSE_RUST",
};

fn rust_symbol_kind(ts_kind: &str) -> SymbolKind {
    match ts_kind {
        "function_item" => SymbolKind::Function,
        "struct_item" => SymbolKind::Struct,
        "enum_item" => SymbolKind::Enum,
        "trait_item" => SymbolKind::Trait,
        "type_item" => SymbolKind::Type,
        "const_item" => SymbolKind::Const,
        "static_item" => SymbolKind::Static,
        "mod_item" => SymbolKind::Module,
        "union_item" => SymbolKind::Union,
        _ => SymbolKind::Other,
    }
}

fn rust_name_and_kind(node: Node<'_>, source: &[u8]) -> (String, SymbolKind) {
    let name = node
        .child_by_field_name("name")
        .and_then(|n| n.utf8_text(source).ok())
        .unwrap_or_default()
        .to_owned();
    (name, rust_symbol_kind(node.kind()))
}

fn rust_is_exportable(node: Node<'_>, _source: &[u8]) -> bool {
    if !PUBLIC_ITEM_KINDS.contains(&node.kind()) {
        return false;
    }
    node.children(&mut node.walk())
        .any(|c| c.kind() == "visibility_modifier")
}

#[must_use]
fn rust_interface_symbol(node: Node<'_>, source: &[u8]) -> String {
    // Prefer the source text that precedes the body, which is the item's
    // precise signature (e.g. `pub fn alpha()`).
    let signature = node
        .child_by_field_name("body")
        .and_then(|body| source.get(node.start_byte()..body.start_byte()))
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .map(str::trim);
    if let Some(signature) = signature {
        return normalize_symbol(signature);
    }
    // Items without a body (e.g. `pub struct Config;`) are rebuilt from their
    // significant children in source order.
    let mut parts = Vec::with_capacity(8);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if matches!(
            kind,
            "const"
                | "enum"
                | "field_identifier"
                | "fn"
                | "identifier"
                | "mod"
                | "name"
                | "primitive_type"
                | "static"
                | "struct"
                | "trait"
                | "type"
                | "type_identifier"
                | "union"
                | "use"
                | "visibility_modifier"
        ) && let Ok(text) = child.utf8_text(source)
        {
            parts.push(text.trim());
        }
    }
    normalize_symbol(&parts.join(" "))
}
