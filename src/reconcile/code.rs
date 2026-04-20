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
    rust_semantics::{OwnerIndex, dependency_observations, docstring_facts},
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

    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let owners = eligible_owners(self.ast);
        let rust_files = discover_rust_files(request.root, request.ignores)?;
        let mut claimed_files = BTreeMap::<String, Vec<String>>::new();
        let mut findings = Vec::new();
        let mut owned_files = Vec::new();
        for file in rust_files {
            let rel = normalize(file.strip_prefix(request.root).unwrap_or(&file));
            if let Some(owner) = most_specific_owner(&owners, &rel) {
                claimed_files
                    .entry(owner.clone())
                    .or_default()
                    .push(rel.clone());
                owned_files.push((file, rel, owner));
            } else {
                findings.push(Finding {
                    code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                    severity: FindingSeverity::Error,
                    message: format!("Rust file `{rel}` is not owned by any eligible node"),
                    node: None,
                    path: Some(rel),
                });
            }
        }
        for files in claimed_files.values_mut() {
            files.sort();
        }
        let owner_index = OwnerIndex::new(owners, &claimed_files);
        let mut symbols = Vec::new();
        let mut dependencies = Vec::new();
        let mut docstrings = Vec::new();
        for (file, rel, owner) in owned_files {
            let analysis = analyze_rust_file(&file)?;
            symbols.extend(analysis.symbols);
            dependencies.extend(dependency_observations(
                &analysis.tree,
                &analysis.source,
                &rel,
                &owner,
                &owner_index,
            )?);
            if let Some(facts) = docstring_facts(&analysis.source, &rel, &owner) {
                docstrings.push(facts);
            }
        }
        Ok(ReconcileReport {
            fingerprint: InterfaceFingerprint::from_symbols(&symbols),
            claimed_files,
            symbols,
            dependencies,
            docstrings,
            findings,
        })
    }
}

fn eligible_owners(ast: &Ast) -> Vec<(String, String)> {
    let mut owners = Vec::new();
    for node in &ast.nodes {
        collect_owner(node, &mut owners);
    }
    owners
}

fn collect_owner(node: &Node, owners: &mut Vec<(String, String)>) {
    super::rust_semantics::collect_owner(node, owners);
}

fn most_specific_owner(owners: &[(String, String)], file: &str) -> Option<String> {
    owners
        .iter()
        .filter(|(_, path)| file == path || file.starts_with(&format!("{path}/")))
        .max_by_key(|(_, path)| path.len())
        .map(|(id, _)| id.clone())
}

fn discover_rust_files(root: &Path, ignores: &[String]) -> Result<Vec<PathBuf>, ReconcileError> {
    let mut files = Vec::new();
    walk(root, root, ignores, &mut files)?;
    files.sort();
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
        if path.is_dir() {
            walk(root, &path, ignores, files)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

struct RustFileAnalysis {
    source: String,
    tree: tree_sitter::Tree,
    symbols: Vec<String>,
}

fn analyze_rust_file(path: &Path) -> Result<RustFileAnalysis, ReconcileError> {
    let source = fs::read_to_string(path).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .map_err(|error| ReconcileError {
            code: "CAIRN_RECONCILE_RUST_LANGUAGE".to_owned(),
            message: error.to_string(),
        })?;
    let tree = parser.parse(&source, None).ok_or_else(|| ReconcileError {
        code: "CAIRN_RECONCILE_PARSE_RUST".to_owned(),
        message: format!("failed to parse `{}`", path.display()),
    })?;
    let mut symbols = Vec::new();
    collect_public_symbols(tree.root_node(), source.as_bytes(), &mut symbols)?;
    symbols.sort();
    Ok(RustFileAnalysis {
        source,
        tree,
        symbols,
    })
}

fn collect_public_symbols(
    node: tree_sitter::Node<'_>,
    source: &[u8],
    symbols: &mut Vec<String>,
) -> Result<(), ReconcileError> {
    if is_public_item(node) {
        symbols.push(interface_symbol(node, source)?);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_public_symbols(child, source, symbols)?;
    }
    Ok(())
}

fn is_public_item(node: tree_sitter::Node<'_>) -> bool {
    if !PUBLIC_ITEM_KINDS.contains(&node.kind()) {
        return false;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|child| child.kind() == "visibility_modifier")
}

fn interface_symbol(node: tree_sitter::Node<'_>, source: &[u8]) -> Result<String, ReconcileError> {
    let signature = node
        .child_by_field_name("body")
        .and_then(|body| source.get(node.start_byte()..body.start_byte()))
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .map(str::trim)
        .map(ToOwned::to_owned);
    if let Some(signature) = signature {
        return Ok(normalize_symbol(&signature));
    }
    let text = node.utf8_text(source).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_SYMBOL_TEXT".to_owned(),
        message: error.to_string(),
    })?;
    Ok(normalize_symbol(text))
}

fn normalize_symbol(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
