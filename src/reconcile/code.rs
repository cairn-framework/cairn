//! Phase 1 Rust code reconciler.

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
    fingerprint::InterfaceFingerprint,
};

const PUBLIC_ITEM_KINDS: &[&str] = &[
    "const_item",
    "enum_item",
    "function_item",
    "mod_item",
    "static_item",
    "struct_item",
    "trait_item",
    "type_item",
    "union_item",
];

/// Rust source reconciler.
pub struct RustCodeReconciler<'a> {
    ast: &'a Ast,
}

impl<'a> RustCodeReconciler<'a> {
    /// Creates a Rust source reconciler.
    #[must_use]
    pub const fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }
}

impl Reconciler for RustCodeReconciler<'_> {
    fn id(&self) -> ReconcilerId {
        ReconcilerId("rust-code".to_owned())
    }

    #[allow(clippy::too_many_lines)] // Reason: sequential fast-path and parallel path kept together for clarity
    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let owners = eligible_owners(self.ast);
        let rust_files = discover_rust_files(request.root, request.ignores)?;
        // For small file counts, sequential parsing avoids thread spawn overhead.
        if rust_files.len() < 16 {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_rust::LANGUAGE.into())
                .map_err(|error| ReconcileError {
                    code: "CAIRN_RECONCILE_RUST_LANGUAGE".to_owned(),
                    message: error.to_string(),
                })?;
            let mut claimed_files = BTreeMap::<String, Vec<String>>::new();
            let mut findings = Vec::new();
            let mut symbols = Vec::new();
            let mut source = String::new();
            for file in rust_files {
                let rel = normalize(file.strip_prefix(request.root).unwrap_or(&file));
                if let Some(owner) = most_specific_owner(&owners, &rel) {
                    claimed_files
                        .entry(owner)
                        .or_default()
                        .push(rel.into_owned());
                    symbols.extend(public_symbols(&mut parser, &file, &mut source)?);
                } else {
                    findings.push(Finding {
                        code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                        severity: FindingSeverity::Info,
                        message: format!("Rust file `{rel}` is not owned by any eligible node"),
                        node: None,
                        target: None,
                        path: Some(rel.into_owned()),
                    });
                }
            }
            symbols.sort_unstable();
            return Ok(ReconcileReport {
                fingerprint: InterfaceFingerprint::from_sorted(&symbols),
                claimed_files,
                symbols: std::sync::Arc::new(symbols),
                findings,
            });
        }
        let thread_count = std::thread::available_parallelism().map_or(2, usize::from);
        let chunk_size = rust_files.len().div_ceil(thread_count).max(1);
        let chunks: Vec<_> = rust_files.chunks(chunk_size).collect();
        std::thread::scope(|s| {
            let owners_ref = &owners;
            let mut handles = Vec::with_capacity(chunks.len());
            for chunk in chunks {
                handles.push(s.spawn(move || {
                    let mut parser = tree_sitter::Parser::new();
                    parser
                        .set_language(&tree_sitter_rust::LANGUAGE.into())
                        .map_err(|error| ReconcileError {
                            code: "CAIRN_RECONCILE_RUST_LANGUAGE".to_owned(),
                            message: error.to_string(),
                        })?;
                    let mut claimed_files = BTreeMap::<String, Vec<String>>::new();
                    let mut findings = Vec::new();
                    let mut symbols = Vec::new();
                    let mut source = String::new();
                    for file in chunk {
                        let rel = normalize(file.strip_prefix(request.root).unwrap_or(file));
                        if let Some(owner) = most_specific_owner(owners_ref, &rel) {
                            claimed_files
                                .entry(owner)
                                .or_default()
                                .push(rel.into_owned());
                            symbols.extend(public_symbols(&mut parser, file, &mut source)?);
                        } else {
                            findings.push(Finding {
                                code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                                severity: FindingSeverity::Info,
                                message: format!(
                                    "Rust file `{rel}` is not owned by any eligible node"
                                ),
                                node: None,
                                target: None,
                                path: Some(rel.into_owned()),
                            });
                        }
                    }
                    Ok::<_, ReconcileError>((claimed_files, findings, symbols))
                }));
            }
            let mut all_claimed = BTreeMap::<String, Vec<String>>::new();
            let mut all_findings = Vec::new();
            let mut all_symbols = Vec::new();
            for handle in handles {
                let (claimed, findings, symbols) = handle.join().unwrap()?;
                for (owner, files) in claimed {
                    all_claimed.entry(owner).or_default().extend(files);
                }
                all_findings.extend(findings);
                all_symbols.extend(symbols);
            }
            all_symbols.sort_unstable();
            Ok(ReconcileReport {
                fingerprint: InterfaceFingerprint::from_sorted(&all_symbols),
                claimed_files: all_claimed,
                symbols: std::sync::Arc::new(all_symbols),
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

fn discover_rust_files(root: &Path, ignores: &[String]) -> Result<Vec<PathBuf>, ReconcileError> {
    let mut files = Vec::with_capacity(128);
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
        } else if file_type.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    Ok(())
}
fn public_symbols(
    parser: &mut tree_sitter::Parser,
    path: &Path,
    source: &mut String,
) -> Result<Vec<String>, ReconcileError> {
    source.clear();
    let mut file = fs::File::open(path).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    std::io::Read::read_to_string(&mut file, source).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    // Fast path: if the file contains no 'pub' keyword it has no public
    // symbols that our reconciler cares about (macro_rules! is already
    // excluded from PUBLIC_ITEM_KINDS).
    if !source.contains("pub ") {
        return Ok(Vec::new());
    }
    let tree = parser
        .parse(source.as_bytes(), None)
        .ok_or_else(|| ReconcileError {
            code: "CAIRN_RECONCILE_PARSE_RUST".to_owned(),
            message: format!("failed to parse `{}`", path.display()),
        })?;
    let mut symbols = Vec::new();
    collect_public_symbols(tree.root_node(), source.as_bytes(), &mut symbols)?;
    Ok(symbols)
}

fn collect_public_symbols(
    node: tree_sitter::Node<'_>,
    source: &[u8],
    symbols: &mut Vec<String>,
) -> Result<(), ReconcileError> {
    if node.child_count() == 0 {
        return Ok(());
    }
    let kind = node.kind();
    let is_target = PUBLIC_ITEM_KINDS.contains(&kind);
    let mut has_pub = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if is_target && child.kind() == "visibility_modifier" {
            has_pub = true;
        }
        collect_public_symbols(child, source, symbols)?;
    }
    if is_target && has_pub {
        symbols.push(interface_symbol(node, source));
    }
    Ok(())
}
fn interface_symbol(node: tree_sitter::Node<'_>, source: &[u8]) -> String {
    let signature = node
        .child_by_field_name("body")
        .and_then(|body| source.get(node.start_byte()..body.start_byte()))
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .map(str::trim);
    if let Some(signature) = signature {
        return normalize_symbol(signature);
    }
    let mut parts = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if matches!(
            kind,
            "visibility_modifier"
                | "struct"
                | "enum"
                | "trait"
                | "fn"
                | "mod"
                | "use"
                | "type"
                | "const"
                | "static"
                | "union"
                | "name"
                | "identifier"
                | "primitive_type"
                | "field_identifier"
                | "type_identifier"
        ) && let Ok(text) = child.utf8_text(source)
        {
            parts.push(text.trim());
        }
    }
    parts.join(" ")
}
fn normalize_symbol(text: &str) -> String {
    // Fast path: text is already normalized (no consecutive whitespace,
    // no leading/trailing whitespace).
    let bytes = text.as_bytes();
    if !bytes.is_empty()
        && !bytes[0].is_ascii_whitespace()
        && !bytes[bytes.len() - 1].is_ascii_whitespace()
        && !bytes
            .windows(2)
            .any(|w| w[0].is_ascii_whitespace() && w[1].is_ascii_whitespace())
    {
        return text.to_owned();
    }
    let mut result = String::with_capacity(text.len());
    let mut in_whitespace = true;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !in_whitespace && !result.is_empty() {
                result.push(' ');
            }
            in_whitespace = true;
        } else {
            result.push(ch);
            in_whitespace = false;
        }
    }
    if result.ends_with(' ') {
        result.pop();
    }
    result
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
