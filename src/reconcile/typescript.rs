//! TypeScript language specification for the generic code reconciler.

use tree_sitter::Node;

use crate::reconcile::{
    generic::LanguageSpec,
    symbol::{SymbolKind, normalize_symbol},
    target::Language,
};

/// tree-sitter node kinds eligible for symbol extraction in TypeScript.
pub const EXPORTABLE_KINDS: &[&str] = &[
    "export_statement",
    "class_declaration",
    "function_declaration",
    "interface_declaration",
    "type_alias_declaration",
    "enum_declaration",
    "variable_declaration",
];

/// TypeScript language specification driving [`crate::reconcile::CodeReconciler`].
pub static TYPESCRIPT: LanguageSpec = LanguageSpec {
    language: Language::TypeScript,
    display_name: "TypeScript",
    grammar: || tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
    extensions: &["ts", "tsx"],
    exportable_kinds: EXPORTABLE_KINDS,
    name_and_kind: ts_name_and_kind,
    interface_symbol: ts_interface_symbol,
    is_exportable: ts_is_exportable,
    fast_path: false,
    grammar_error_code: "CAIRN_RECONCILE_TYPESCRIPT_LANGUAGE",
    parse_error_code: "CAIRN_RECONCILE_PARSE_TYPESCRIPT",
};

fn ts_symbol_kind(ts_kind: &str) -> SymbolKind {
    match ts_kind {
        "function_declaration" => SymbolKind::Function,
        "class_declaration" => SymbolKind::Class,
        "interface_declaration" => SymbolKind::Interface,
        "type_alias_declaration" => SymbolKind::Type,
        "enum_declaration" => SymbolKind::Enum,
        "variable_declaration" => SymbolKind::Variable,
        _ => SymbolKind::Other,
    }
}

/// Resolves the (name, kind) pair for a TypeScript exportable node. For an
/// `export_statement`, both are derived from the wrapped declaration child
/// rather than the wrapper itself.
fn ts_name_and_kind(node: Node<'_>, source: &[u8]) -> (String, SymbolKind) {
    let declared = if node.kind() == "export_statement" {
        node.children(&mut node.walk())
            .find(|c| {
                matches!(
                    c.kind(),
                    "class_declaration"
                        | "function_declaration"
                        | "interface_declaration"
                        | "type_alias_declaration"
                        | "enum_declaration"
                        | "variable_declaration"
                        | "lexical_declaration"
                )
            })
            .unwrap_or(node)
    } else {
        node
    };
    let name = declared
        .child_by_field_name("name")
        .or_else(|| {
            if node.kind() == "export_statement" {
                node.children(&mut node.walk())
                    .find(|c| c.kind() == "identifier" || c.kind() == "string_literal")
            } else {
                None
            }
        })
        .and_then(|n| n.utf8_text(source).ok())
        .map_or_else(
            || node.utf8_text(source).unwrap_or("").to_owned(),
            str::to_owned,
        );
    (name, ts_symbol_kind(declared.kind()))
}

fn ts_is_exportable(node: Node<'_>, _source: &[u8]) -> bool {
    if node.kind() == "export_statement" {
        return true;
    }
    node.children(&mut node.walk())
        .any(|c| c.kind() == "visibility_modifier" || c.kind() == "export")
}

#[must_use]
fn ts_interface_symbol(node: Node<'_>, source: &[u8]) -> String {
    let kind = node.kind();
    let name = node
        .child_by_field_name("name")
        .or_else(|| {
            if kind == "export_statement" {
                node.children(&mut node.walk())
                    .find(|c| c.kind() == "identifier" || c.kind() == "string_literal")
            } else {
                None
            }
        })
        .and_then(|n| n.utf8_text(source).ok())
        .map_or_else(
            || node.utf8_text(source).unwrap_or("").to_owned(),
            str::to_owned,
        );

    let signature = format!("{kind}:{name}");
    normalize_symbol(&signature)
}
