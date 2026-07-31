//! Allowlist parsing and governed-member expansion.

// Reason: the parent module owns the shared path plumbing these helpers build on.
#![allow(clippy::wildcard_imports)]
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use super::*;

/// Parses the binding-surface allowlist rows from its committed text.
///
/// The file is the single parsed authority for the binding surface, so both
/// the scanner (current tree) and the commit hook (merge-base tree) classify
/// against the same grammar without a second copy of the rules in code.
///
/// # Errors
///
/// Returns a message when a row is malformed or the file declares no rule.
pub(crate) fn parse_allowlist(source: &str) -> Result<Vec<RepoPathRule>, String> {
    let mut rules = Vec::new();
    for line in source.lines() {
        let Some(row) = line.strip_prefix("- ") else {
            continue;
        };
        let row = row.trim();
        let Some(rule) = normalise_repo_entry(row) else {
            return Err(format!("malformed rule: `{row}`"));
        };
        rules.push(rule);
    }
    if rules.is_empty() {
        return Err("no valid rules".to_owned());
    }
    Ok(rules)
}

/// Canonical repository-relative paths of every file a rule governs.
///
/// Directory rules expand recursively and follow symlinks exactly as the
/// manifest hasher does, and every member is reported in canonical form, so a
/// caller classifies the bytes a decision really governs rather than the
/// spelling its entry happens to use.
///
/// # Errors
///
/// Returns a [`ManifestError`] when the path is unreadable, cycles, or
/// resolves outside the repository. A path that does not exist yet governs
/// nothing and yields an empty set, so a candidate may name files its change
/// will create. Callers must treat that as a refusal,
/// never as an empty governed set.
pub(crate) fn governed_canonical_files(
    root: &Path,
    rule: &RepoPathRule,
) -> Result<Vec<String>, ManifestError> {
    let canonical_root = fs::canonicalize(root).map_err(|error| ManifestError {
        message: format!(
            "cannot canonicalise repository root {}: {error}",
            root.display()
        ),
    })?;
    let mut out = Vec::new();
    match rule {
        // Receipts are evidence, never subject: excluded by identity here for
        // the same reason the hasher excludes them, so a decision may name its
        // future receipt paths before they exist.
        RepoPathRule::File(path)
            if is_review_path(path) || resolves_into_reviews(&canonical_root, &root.join(path)) => {
        }
        RepoPathRule::File(path) if root.join(path).symlink_metadata().is_err() => {}
        RepoPathRule::File(path) => {
            let canonical = canonicalise_contained(&root.join(path), &canonical_root, path)?;
            if let Some(relative) = canonical_relative(&canonical_root, &canonical) {
                out.push(relative);
            }
        }
        RepoPathRule::Dir(path)
            if is_review_path(path)
                || resolves_into_reviews(&canonical_root, &root.join(path))
                || root.join(path).symlink_metadata().is_err() => {}
        RepoPathRule::Dir(path) => {
            let mut ancestors = BTreeSet::new();
            collect_canonical_files(
                &canonical_root,
                &root.join(path),
                path,
                &mut ancestors,
                &mut out,
            )?;
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

fn collect_canonical_files(
    canonical_root: &Path,
    filesystem_path: &Path,
    label: &str,
    ancestors: &mut BTreeSet<PathBuf>,
    out: &mut Vec<String>,
) -> Result<(), ManifestError> {
    let canonical = canonicalise_contained(filesystem_path, canonical_root, label)?;
    if !ancestors.insert(canonical.clone()) {
        return Err(ManifestError {
            message: format!("governed directory path {label} forms a symlink cycle"),
        });
    }
    let mut children = fs::read_dir(filesystem_path)
        .map_err(|error| read_error(label, &error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| read_error(label, &error))?;
    children.sort_by_key(std::fs::DirEntry::file_name);
    for child in children {
        let name = child.file_name();
        let Some(name) = name.to_str() else {
            return Err(ManifestError {
                message: format!("governed path under {label} is not valid UTF-8"),
            });
        };
        let child_label = format!("{label}/{name}");
        let child_path = child.path();
        if is_review_path(&child_label) || resolves_into_reviews(canonical_root, &child_path) {
            continue;
        }
        let child_canonical = canonicalise_contained(&child_path, canonical_root, &child_label)?;
        let metadata =
            fs::metadata(&child_canonical).map_err(|error| read_error(&child_label, &error))?;
        if metadata.is_dir() {
            collect_canonical_files(canonical_root, &child_path, &child_label, ancestors, out)?;
        } else if metadata.is_file()
            && let Some(relative) = canonical_relative(canonical_root, &child_canonical)
        {
            out.push(relative);
        }
    }
    ancestors.remove(&canonical);
    Ok(())
}

fn canonical_relative(canonical_root: &Path, canonical: &Path) -> Option<String> {
    let relative = canonical.strip_prefix(canonical_root).ok()?;
    let mut text = String::new();
    for part in relative.components() {
        if !text.is_empty() {
            text.push('/');
        }
        text.push_str(part.as_os_str().to_str()?);
    }
    (!text.is_empty()).then_some(text)
}
