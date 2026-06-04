//! Python code reconciler.

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

const PYTHON_ITEM_KINDS: &[&str] = &["function_definition", "class_definition", "assignment"];

/// Python source reconciler.
pub struct PythonReconciler<'a> {
    ast: &'a Ast,
}

impl<'a> PythonReconciler<'a> {
    /// Creates a new Python reconciler.
    #[must_use]
    pub const fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }
}

impl Reconciler for PythonReconciler<'_> {
    fn id(&self) -> ReconcilerId {
        ReconcilerId("python-code".to_owned())
    }

    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let owners = eligible_owners(self.ast);
        let py_files = discover_py_files(request.root, request.ignores)?;
        let thread_count = std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(2);
        let chunk_size = ((py_files.len() + thread_count - 1) / thread_count).max(1);
        let chunks: Vec<_> = py_files.chunks(chunk_size).collect();
        std::thread::scope(|s| {
            let owners_ref = &owners;
            let mut handles = Vec::with_capacity(chunks.len());
            for chunk in chunks {
                handles.push(s.spawn(move || {
                    let mut parser = tree_sitter::Parser::new();
                    parser
                        .set_language(&tree_sitter_python::LANGUAGE.into())
                        .map_err(|error| ReconcileError {
                            code: "CAIRN_RECONCILE_PYTHON_LANGUAGE".to_owned(),
                            message: error.to_string(),
                        })?;
                    let mut claimed_files = BTreeMap::<String, Vec<String>>::new();
                    let mut findings = Vec::new();
                    let mut symbols = Vec::new();
                    for file in chunk {
                        let rel = normalize(file.strip_prefix(request.root).unwrap_or(file));
                        if let Some(owner) = most_specific_owner(owners_ref, &rel) {
                            claimed_files
                                .entry(owner)
                                .or_default()
                                .push(rel.into_owned());
                            symbols.extend(public_symbols(&mut parser, file)?);
                        } else {
                            findings.push(Finding {
                                code: "CAIRN_RECONCILE_ORPHANED_FILE".to_owned(),
                                severity: FindingSeverity::Info,
                                message: format!(
                                    "Python file `{rel}` is not owned by any eligible node"
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
    owners.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
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

fn discover_py_files(root: &Path, ignores: &[String]) -> Result<Vec<PathBuf>, ReconcileError> {
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
        } else if file_type.is_file() && path.extension().is_some_and(|ext| ext == "py") {
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
        code: "CAIRN_RECONCILE_PARSE_PYTHON".to_owned(),
        message: format!("failed to parse `{}`", path.display()),
    })?;
    let mut symbols = Vec::new();
    let has_all = source.contains("__all__");
    collect_public_symbols(tree.root_node(), source.as_bytes(), &mut symbols, has_all)?;
    Ok(symbols)
}

fn collect_public_symbols(
    node: tree_sitter::Node<'_>,
    source: &[u8],
    symbols: &mut Vec<String>,
    has_all: bool,
) -> Result<(), ReconcileError> {
    if is_public_item(node, source, has_all) {
        symbols.push(interface_symbol(node, source));
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_public_symbols(child, source, symbols, has_all)?;
    }
    Ok(())
}

fn is_public_item(node: tree_sitter::Node<'_>, source: &[u8], has_all: bool) -> bool {
    if !PYTHON_ITEM_KINDS.contains(&node.kind()) {
        return false;
    }
    if has_all {
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
fn interface_symbol(node: tree_sitter::Node<'_>, source: &[u8]) -> String {
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

fn normalize_symbol(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
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
