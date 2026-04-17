//! Phase 1 Rust code reconciler.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    dsl::{Ast, Node},
    ontology::graph::{Finding, FindingSeverity},
    scanner::config::is_ignored,
};

use super::{
    ReconcileError, ReconcileReport, ReconcileRequest, Reconciler, ReconcilerId,
    fingerprint::InterfaceFingerprint,
};

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
        let mut symbols = Vec::new();
        for file in rust_files {
            let rel = normalize(file.strip_prefix(request.root).unwrap_or(&file));
            if let Some(owner) = most_specific_owner(&owners, &rel) {
                claimed_files
                    .entry(owner.clone())
                    .or_default()
                    .push(rel.clone());
                symbols.extend(public_symbols(&file)?);
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
        Ok(ReconcileReport {
            fingerprint: InterfaceFingerprint::from_symbols(&symbols),
            claimed_files,
            symbols,
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

fn public_symbols(path: &Path) -> Result<Vec<String>, ReconcileError> {
    let source = fs::read_to_string(path).map_err(|error| ReconcileError {
        code: "CAIRN_RECONCILE_READ_SOURCE".to_owned(),
        message: format!("failed to read `{}`: {error}", path.display()),
    })?;
    Ok(source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("pub ") || trimmed.starts_with("pub(") {
                Some(trimmed.to_owned())
            } else {
                None
            }
        })
        .collect())
}

fn trim_dot(path: &str) -> String {
    path.trim_start_matches("./").to_owned()
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
