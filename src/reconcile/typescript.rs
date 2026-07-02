//! TypeScript code reconciler.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    blueprint::{Ast, Node},
    map::graph::{Finding, FindingSeverity},
    scanner::config::is_ignored,
};

use super::{
    ReconcileError, ReconcileReport, ReconcileRequest, Reconciler, ReconcilerId,
    fingerprint::InterfaceFingerprint, symbol::normalize_symbol,
};

const EXPORTABLE_KINDS: &[&str] = &[
    "export_statement",
    "class_declaration",
    "function_declaration",
    "interface_declaration",
    "type_alias_declaration",
    "enum_declaration",
    "variable_declaration",
];

/// TypeScript source reconciler.
pub struct TypeScriptReconciler<'a> {
    ast: &'a Ast,
}

impl<'a> TypeScriptReconciler<'a> {
    /// Creates a new TypeScript reconciler.
    #[must_use]
    pub const fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }
}

impl Reconciler for TypeScriptReconciler<'_> {
    fn id(&self) -> ReconcilerId {
        ReconcilerId("typescript-code".to_owned())
    }

    #[allow(clippy::too_many_lines)] // Reason: parallel per-chunk processing and merge kept together for clarity
    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let owners = eligible_owners(self.ast);
        let ts_files = discover_ts_files(request.root, request.ignores)?;
        let thread_count = std::thread::available_parallelism().map_or(2, usize::from);
        let chunk_size = ts_files.len().div_ceil(thread_count).max(1);
        let chunks: Vec<_> = ts_files.chunks(chunk_size).collect();
        std::thread::scope(|s| {
            let owners_ref = &owners;
            let mut handles = Vec::with_capacity(chunks.len());
            for chunk in chunks {
                handles.push(s.spawn(move || {
                    let mut parser = tree_sitter::Parser::new();
                    parser
                        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
                        .map_err(|error| ReconcileError {
                            code: "CAIRN_RECONCILE_TS_LANGUAGE".to_owned(),
                            message: error.to_string(),
                        })?;
                    let mut claimed_files = BTreeMap::<String, Vec<String>>::new();
                    let mut node_symbols = BTreeMap::<String, Vec<String>>::new();
                    let mut node_symbol_records =
                        BTreeMap::<String, Vec<super::SymbolRecord>>::new();
                    let mut findings = Vec::new();
                    let mut symbols = Vec::new();
                    for file in chunk {
                        let rel = normalize(file.strip_prefix(request.root).unwrap_or(file));
                        if let Some(owner) = most_specific_owner(owners_ref, &rel) {
                            let (file_symbols, file_records) =
                                public_symbols(&mut parser, file, &rel)?;
                            claimed_files
                                .entry(owner.clone())
                                .or_default()
                                .push(rel.into_owned());
                            node_symbols
                                .entry(owner.clone())
                                .or_default()
                                .extend(file_symbols.clone());
                            node_symbol_records
                                .entry(owner)
                                .or_default()
                                .extend(file_records);
                            symbols.extend(file_symbols);
                        } else {
                            findings.push(Finding {
                                code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                                severity: FindingSeverity::Info,
                                message: format!(
                                    "TypeScript file `{rel}` is not owned by any eligible node"
                                ),
                                node: None,
                                target: None,
                                path: Some(rel.into_owned()),
                            });
                        }
                    }
                    Ok::<_, ReconcileError>((
                        claimed_files,
                        findings,
                        symbols,
                        node_symbols,
                        node_symbol_records,
                    ))
                }));
            }
            let mut all_claimed = BTreeMap::<String, Vec<String>>::new();
            let mut all_findings = Vec::new();
            let mut all_symbols = Vec::new();
            let mut all_node_symbols = BTreeMap::<String, Vec<String>>::new();
            let mut all_node_symbol_records = BTreeMap::<String, Vec<super::SymbolRecord>>::new();
            for handle in handles {
                let (claimed, findings, symbols, node_symbols, node_symbol_records) =
                    handle.join().unwrap()?;
                for (owner, files) in claimed {
                    all_claimed.entry(owner).or_default().extend(files);
                }
                for (owner, syms) in node_symbols {
                    all_node_symbols.entry(owner).or_default().extend(syms);
                }
                for (owner, records) in node_symbol_records {
                    all_node_symbol_records
                        .entry(owner)
                        .or_default()
                        .extend(records);
                }
                all_findings.extend(findings);
                all_symbols.extend(symbols);
            }
            all_symbols.sort_unstable();
            for node_syms in all_node_symbols.values_mut() {
                node_syms.sort_unstable();
            }
            for records in all_node_symbol_records.values_mut() {
                records.sort_by(|a, b| a.signature.cmp(&b.signature));
            }
            Ok(ReconcileReport {
                fingerprint: InterfaceFingerprint::from_sorted(&all_symbols),
                claimed_files: all_claimed,
                symbols: std::sync::Arc::new(all_symbols),
                node_symbols: all_node_symbols,
                node_symbol_records: all_node_symbol_records,
                findings: all_findings,
            })
        })
    }
}

fn eligible_owners(ast: &Ast) -> Vec<(String, String)> {
    let mut owners = Vec::new();
    for node in &ast.nodes {
        collect_owner(node, &mut owners);
    }
    owners.sort_by_key(|b| std::cmp::Reverse(b.1.len()));
    owners
}

fn collect_owner(node: &Node, owners: &mut Vec<(String, String)>) {
    let is_internal = !node.children.is_empty();
    if !is_internal || node.owns_files {
        for path in &node.paths {
            owners.push((node.id.clone(), trim_dot(path)));
        }
    }
    for child in &node.children {
        collect_owner(child, owners);
    }
}
fn most_specific_owner(owners: &[(String, String)], file: &str) -> Option<String> {
    for (id, path) in owners {
        // An empty path or "." is a root-level claim that matches any file.
        if path.is_empty()
            || path == "."
            || file == path
            || (file.starts_with(path) && file.as_bytes().get(path.len()) == Some(&b'/'))
        {
            return Some(id.clone());
        }
    }
    None
}

fn discover_ts_files(root: &Path, ignores: &[String]) -> Result<Vec<PathBuf>, ReconcileError> {
    let mut files = Vec::new();
    walk(root, root, ignores, &mut files)?;
    files.sort_unstable();
    Ok(files)
}

fn walk(
    root: &Path,
    dir: &Path,
    ignores: &[String],
    files: &mut Vec<PathBuf>,
) -> Result<(), ReconcileError> {
    for entry in fs::read_dir(dir).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_DIR".to_owned(),
        message: format!("failed to read `{}`: {error}", dir.display()),
    })? {
        let entry = entry.map_err(|error| ReconcileError {
            code: "CAIRN_RECONCILE_READ_DIR_ENTRY".to_owned(),
            message: error.to_string(),
        })?;
        let path = entry.path();
        let rel = normalize(path.strip_prefix(root).unwrap_or(&path));
        if is_ignored(&rel, ignores) {
            continue;
        }
        let file_type = entry.file_type().map_err(|error| ReconcileError {
            code: "CAIRN_RECONCILE_READ_DIR_ENTRY".to_owned(),
            message: error.to_string(),
        })?;
        if file_type.is_dir() {
            walk(root, &path, ignores, files)?;
        } else if file_type.is_file()
            && let Some(ext) = path.extension().and_then(|e| e.to_str())
            && (ext == "ts" || ext == "tsx")
        {
            files.push(path);
        }
    }
    Ok(())
}
fn public_symbols(
    parser: &mut tree_sitter::Parser,
    path: &Path,
    file_rel: &str,
) -> Result<(Vec<String>, Vec<super::SymbolRecord>), ReconcileError> {
    let source = fs::read_to_string(path).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    let tree = parser.parse(&source, None).ok_or_else(|| ReconcileError {
        code: "CAIRN_RECONCILE_PARSE_TS".to_owned(),
        message: format!("failed to parse `{}`", path.display()),
    })?;
    let mut symbols = Vec::new();
    let mut records = Vec::new();
    collect_public_symbols(
        tree.root_node(),
        source.as_bytes(),
        &mut symbols,
        &mut records,
        file_rel,
    )?;
    Ok((symbols, records))
}

fn collect_public_symbols(
    node: tree_sitter::Node<'_>,
    source: &[u8],
    symbols: &mut Vec<String>,
    records: &mut Vec<super::SymbolRecord>,
    file_rel: &str,
) -> Result<(), ReconcileError> {
    if node.child_count() == 0 {
        return Ok(());
    }
    let kind = node.kind();
    let is_target = EXPORTABLE_KINDS.contains(&kind);
    let mut is_exportable = kind == "export_statement";
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if is_target && (child.kind() == "visibility_modifier" || child.kind() == "export") {
            is_exportable = true;
        }
        collect_public_symbols(child, source, symbols, records, file_rel)?;
    }
    if is_target && is_exportable {
        let signature = interface_symbol(node, source);
        let (name, symbol_kind) = symbol_name_and_kind(node, source);
        records.push(super::SymbolRecord {
            name,
            kind: symbol_kind,
            signature: signature.clone(),
            file: file_rel.to_owned(),
            line: u32::try_from(node.start_position().row).unwrap_or(u32::MAX) + 1,
            end_line: u32::try_from(node.end_position().row).unwrap_or(u32::MAX) + 1,
        });
        symbols.push(signature);
    }
    Ok(())
}

/// Resolves the (name, kind) pair for a TypeScript exportable node. For
/// `export_statement`, both are derived from the wrapped declaration child
/// rather than the wrapper itself.
fn symbol_name_and_kind(node: tree_sitter::Node<'_>, source: &[u8]) -> (String, super::SymbolKind) {
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
    (name, symbol_kind(declared.kind()))
}

/// Maps a TypeScript tree-sitter node kind to a language-agnostic [`super::SymbolKind`].
fn symbol_kind(ts_kind: &str) -> super::SymbolKind {
    match ts_kind {
        "function_declaration" => super::SymbolKind::Function,
        "class_declaration" => super::SymbolKind::Class,
        "interface_declaration" => super::SymbolKind::Interface,
        "type_alias_declaration" => super::SymbolKind::Type,
        "enum_declaration" => super::SymbolKind::Enum,
        "variable_declaration" | "lexical_declaration" => super::SymbolKind::Variable,
        _ => super::SymbolKind::Other,
    }
}

#[must_use]
fn interface_symbol(node: tree_sitter::Node<'_>, source: &[u8]) -> String {
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

fn trim_dot(path: &str) -> String {
    path.trim_start_matches("./").to_owned()
}

fn normalize(path: &Path) -> std::borrow::Cow<'_, str> {
    let s = path.to_string_lossy();
    if s.contains('\\') {
        std::borrow::Cow::Owned(s.replace('\\', "/"))
    } else {
        s
    }
}
