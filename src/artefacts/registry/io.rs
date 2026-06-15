//! Artefact loading and I/O helpers.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

use crate::{
    blueprint::{Field, Node},
    map::graph::{Finding, FindingSeverity},
};

pub(super) fn pointers(ast: &Ast, field_name: &str) -> Vec<String> {
    let mut result = Vec::new();
    for node in &ast.nodes {
        collect_pointers(node, field_name, &mut result);
    }
    result.sort();
    result.dedup();
    result
}

pub(super) fn collect_pointers(node: &Node, field_name: &str, result: &mut Vec<String>) {
    for Field { name, values, .. } in &node.raw_fields {
        if name == field_name {
            result.extend(values.iter().cloned());
        }
    }
    for child in &node.children {
        collect_pointers(child, field_name, result);
    }
}

pub(super) fn collect_ids(ast: &Ast) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for node in &ast.nodes {
        collect_node_id(node, &mut ids);
    }
    ids
}

pub(super) fn collect_node_id(node: &Node, ids: &mut BTreeSet<String>) {
    ids.insert(node.id.clone());
    for child in &node.children {
        collect_node_id(child, ids);
    }
}

pub(super) fn markdown_paths(root: &Path, pointer: &str, set: &mut ArtefactSet) -> Vec<PathBuf> {
    let path = root.join(pointer);
    if path.is_dir() {
        return read_dir_markdown(&path).unwrap_or_else(|error| {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_DIR_READ_FAILED",
                format!("failed to read artefact directory `{pointer}`: {error}"),
                Some(pointer.to_owned()),
            ));
            Vec::new()
        });
    }
    if path.exists() {
        vec![path]
    } else {
        set.findings.push(warning(
            "CAIRN_ARTEFACT_POINTER_MISSING",
            format!("artefact pointer `{pointer}` is missing"),
            None,
            Some(pointer.to_owned()),
        ));
        Vec::new()
    }
}

pub(super) fn read_dir_markdown(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|entry| entry.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths)
}

pub(super) fn parse_file(
    path: &Path,
    pointer: &str,
    set: &mut ArtefactSet,
) -> Option<frontmatter::Frontmatter> {
    fs::read_to_string(path)
        .map(|source| frontmatter::parse(&source))
        .map_err(|error| {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_READ_FAILED",
                format!(
                    "failed to read artefact `{}` from `{pointer}`: {error}",
                    path.display()
                ),
                Some(path_string(path)),
            ));
        })
        .ok()
}

pub(super) fn required(
    values: &BTreeMap<String, String>,
    key: &str,
    path: String,
    set: &mut ArtefactSet,
) -> Option<String> {
    values
        .get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .or_else(|| {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_MISSING_FIELD",
                format!("artefact `{path}` lacks required `{key}` frontmatter"),
                Some(path),
            ));
            None
        })
}

pub(super) fn optional(values: &BTreeMap<String, String>, key: &str) -> Option<String> {
    values.get(key).filter(|value| !value.is_empty()).cloned()
}

pub(super) fn list(parsed: &frontmatter::Frontmatter, key: &str) -> Vec<String> {
    parsed.lists.get(key).cloned().unwrap_or_default()
}

pub(super) fn error(
    code: &str,
    message: String,
    node: Option<String>,
    path: Option<String>,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Error,
        message,
        node,
        target: None,
        path,
    }
}

pub(super) fn warning(
    code: &str,
    message: String,
    node: Option<String>,
    path: Option<String>,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Warning,
        message,
        node,
        target: None,
        path,
    }
}

pub(super) fn error_finding(code: &str, message: String, path: Option<String>) -> Finding {
    error(code, message, None, path)
}

pub(super) fn info(
    code: &str,
    message: String,
    node: Option<String>,
    path: Option<String>,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Info,
        message,
        node,
        target: None,
        path,
    }
}

pub(super) fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

pub(super) fn is_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}
