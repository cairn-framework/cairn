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
    fingerprint::InterfaceFingerprint,
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

    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let owners = eligible_owners(self.ast);
        let ts_files = discover_ts_files(request.root, request.ignores)?;
        let thread_count = std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(2);
        let chunk_size = (ts_files.len() / thread_count).max(1);
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
                    let mut findings = Vec::new();
                    let mut symbols = Vec::new();
                    for file in chunk {
                        let rel = normalize(file.strip_prefix(request.root).unwrap_or(file));
                        if let Some(owner) = most_specific_owner(owners_ref, &rel) {
                            claimed_files.entry(owner).or_default().push(rel);
                            symbols.extend(public_symbols(&mut parser, file)?);
                        } else {
                            findings.push(Finding {
                                code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                                severity: FindingSeverity::Info,
                                message: format!(
                                    "TypeScript file `{rel}` is not owned by any eligible node"
                                ),
                                node: None,
                                target: None,
                                path: Some(rel),
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
            for files in all_claimed.values_mut() {
                files.sort();
            }
            Ok(ReconcileReport {
                fingerprint: InterfaceFingerprint::from_symbols(&all_symbols),
                claimed_files: all_claimed,
                symbols: all_symbols,
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
    owners
        .iter()
        .filter(|(_, path)| {
            // An empty path or "." is a root-level claim that matches any file.
            path.is_empty()
                || path == "."
                || file == path
                || (file.starts_with(path) && file.as_bytes().get(path.len()) == Some(&b'/'))
        })
        .max_by_key(|(_, path)| path.len())
        .map(|(id, _)| id.clone())
}

fn discover_ts_files(root: &Path, ignores: &[String]) -> Result<Vec<PathBuf>, ReconcileError> {
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
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str())
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
) -> Result<Vec<String>, ReconcileError> {
    let source = fs::read_to_string(path).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    let tree = parser.parse(&source, None).ok_or_else(|| ReconcileError {
        code: "CAIRN_RECONCILE_PARSE_TS".to_owned(),
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
    if is_exportable(node, source) {
        symbols.push(interface_symbol(node, source));
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_public_symbols(child, source, symbols)?;
    }
    Ok(())
}

fn is_exportable(node: tree_sitter::Node<'_>, _source: &[u8]) -> bool {
    if !EXPORTABLE_KINDS.contains(&node.kind()) {
        return false;
    }
    if node.kind() == "export_statement" {
        return true;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|child| child.kind() == "visibility_modifier" || child.kind() == "export")
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

fn normalize_symbol(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn trim_dot(path: &str) -> String {
    path.trim_start_matches("./").to_owned()
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
