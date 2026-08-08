//! Artefact loading and I/O helpers.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::Component,
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

pub(super) fn markdown_paths(
    root: &Path,
    raw_pointer: &str,
    set: &mut ArtefactSet,
) -> Vec<PathBuf> {
    let Some(pointer) = super::manifest::normalise_repo_pointer(raw_pointer) else {
        set.findings.push(error_finding(
            "CAIRN_ARTEFACT_READ_FAILED",
            format!("artefact pointer `{raw_pointer}` is not a safe repository-relative path"),
            Some(raw_pointer.to_owned()),
        ));
        return Vec::new();
    };
    match pointer_contains_symlink(root, &pointer) {
        Ok(true) => {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_READ_FAILED",
                format!("artefact pointer `{raw_pointer}` resolves through a symlink"),
                Some(raw_pointer.to_owned()),
            ));
            return Vec::new();
        }
        Ok(false) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            set.findings.push(error_finding(
                "CAIRN_ARTEFACT_READ_FAILED",
                format!("failed to inspect artefact pointer `{raw_pointer}`: {error}"),
                Some(raw_pointer.to_owned()),
            ));
            return Vec::new();
        }
    }
    let path = root.join(&pointer);
    if path.is_dir() {
        return match read_dir_markdown(&path) {
            Ok(paths) => paths,
            Err(error) if error.kind() == io::ErrorKind::InvalidData => {
                set.findings.push(error_finding(
                    "CAIRN_ARTEFACT_READ_FAILED",
                    format!("failed to read artefact directory `{pointer}`: {error}"),
                    Some(pointer.clone()),
                ));
                Vec::new()
            }
            Err(error) => {
                set.findings.push(error_finding(
                    "CAIRN_ARTEFACT_DIR_READ_FAILED",
                    format!("failed to read artefact directory `{pointer}`: {error}"),
                    Some(pointer.clone()),
                ));
                Vec::new()
            }
        };
    }
    if path.exists() {
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_file() => vec![path],
            Ok(_) => {
                set.findings.push(error_finding(
                    "CAIRN_ARTEFACT_READ_FAILED",
                    format!("artefact pointer `{pointer}` is not a regular file"),
                    Some(pointer),
                ));
                Vec::new()
            }
            Err(error) => {
                set.findings.push(error_finding(
                    "CAIRN_ARTEFACT_READ_FAILED",
                    format!("failed to inspect artefact pointer `{pointer}`: {error}"),
                    Some(pointer),
                ));
                Vec::new()
            }
        }
    } else {
        set.findings.push(warning(
            "CAIRN_ARTEFACT_POINTER_MISSING",
            format!("artefact pointer `{pointer}` is missing"),
            None,
            Some(pointer),
        ));
        Vec::new()
    }
}

fn pointer_contains_symlink(root: &Path, pointer: &str) -> io::Result<bool> {
    let mut current = root.to_owned();
    for component in Path::new(pointer).components() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);
        if fs::symlink_metadata(&current)?.file_type().is_symlink() {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn read_dir_markdown(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        let entry = entry.path();
        if entry.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let metadata = fs::symlink_metadata(&entry)?;
        if metadata.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("directory contains symlink `{}`", entry.display()),
            ));
        }
        if !metadata.file_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("directory contains non-regular file `{}`", entry.display()),
            ));
        }
        paths.push(entry);
    }
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
        deferred_by: None,
        parked_by: None,
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
        deferred_by: None,
        parked_by: None,
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
        deferred_by: None,
        parked_by: None,
    }
}

/// Returns the lexical path spelling used on artefact records and findings.
///
/// `Path::components` removes redundant current-directory components and
/// separators without touching the filesystem, so a walked path such as
/// `././meta/todos/todo.example.md` is stored as `meta/todos/todo.example.md`.
pub(super) fn path_string(path: &Path) -> String {
    let mut normalized = PathBuf::new();
    let mut saw_component = false;
    for component in path.components() {
        saw_component = true;
        if component == Component::CurDir {
            continue;
        }
        normalized.push(component.as_os_str());
    }
    if normalized.as_os_str().is_empty() && saw_component {
        normalized.push(".");
    }
    normalized.to_string_lossy().into_owned()
}

pub(super) fn is_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}
