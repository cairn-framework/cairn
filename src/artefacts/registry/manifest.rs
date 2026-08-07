//! Canonical subject manifests for decision ratification receipts.

use std::{
    collections::BTreeSet,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use super::sha256::sha256_hex;

mod governed;
mod review_coverage;

pub(crate) use review_coverage::review_path_covered;

pub(crate) use governed::{governed_canonical_files, parse_allowlist};

/// Frontmatter keys excluded from a decision's reviewed content.
pub const RATIFICATION_KEYS: &[&str] = &["status", "ratification", "ratified_by", "receipts"];

/// Review artefact directory, as declared by the registry kind table.
pub(crate) const REVIEWS_DIR: &str = "meta/reviews";

/// A normalised repository entry used as either an exact file or directory rule.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RepoPathRule {
    /// An exact file path.
    File(String),
    /// A directory prefix, stored without its trailing slash.
    Dir(String),
}

/// Removes ratification metadata from a leading frontmatter block.
///
/// Only complete, leading `---` frontmatter is considered. The removal is
/// line-based so all retained bytes, including line endings, remain unchanged.
#[must_use]
pub fn governed_content(raw: &str) -> String {
    let Some(frontmatter_end) = leading_frontmatter_end(raw) else {
        return raw.to_owned();
    };

    let mut output = String::with_capacity(raw.len());
    let mut cursor = 0;
    let mut position = first_line_end(raw);

    while position < frontmatter_end {
        let line_end = next_line_end(raw, position);
        let line = line_without_ending(&raw[position..line_end]);
        if let Some(opens_list) = ratification_key_shape(line.trim()) {
            output.push_str(&raw[cursor..position]);
            position = line_end;
            if opens_list {
                while position < frontmatter_end {
                    let continuation_end = next_line_end(raw, position);
                    let continuation = line_without_ending(&raw[position..continuation_end]);
                    if continues_open_list(continuation) {
                        position = continuation_end;
                    } else {
                        break;
                    }
                }
            }
            cursor = position;
        } else {
            position = line_end;
        }
    }

    output.push_str(&raw[cursor..]);
    output
}

/// Normalises a plain repository-relative file path.
///
/// Trailing slashes are stripped so callers that receive changed-path lists use
/// one canonical spelling. Empty paths, absolute paths, dot segments, parent
/// segments, and backslashes are rejected.
#[must_use]
pub fn normalise_repo_path(raw: &str) -> Option<String> {
    normalise_segments(raw)
}

/// Normalises a repository entry, retaining whether its source was a directory rule.
#[must_use]
pub fn normalise_repo_entry(raw: &str) -> Option<RepoPathRule> {
    let is_directory = raw.ends_with('/');
    let path = normalise_segments(raw)?;
    Some(if is_directory {
        RepoPathRule::Dir(path)
    } else {
        RepoPathRule::File(path)
    })
}

/// Returns whether a normalised rule covers a plain file path.
#[must_use]
pub fn rule_matches(rule: &RepoPathRule, file: &str) -> bool {
    let Some(file) = valid_repo_path(file) else {
        return false;
    };

    match rule {
        RepoPathRule::File(path) => path == file,
        RepoPathRule::Dir(directory) => file
            .strip_prefix(directory.as_str())
            .is_some_and(|rest| rest.starts_with('/')),
    }
}

/// Failure while constructing a canonical subject manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManifestError {
    /// Human-readable failure detail, including the offending path.
    pub message: String,
}

impl fmt::Display for ManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ManifestError {}

/// Computes the canonical hash covering a decision and its governed files.
///
/// The decision is hashed from [`governed_content`]. Every other included path
/// is read as raw bytes beneath `repo_root`. Directory entries expand to their
/// recursively contained files. Review artefacts are excluded by identity,
/// preventing the evidence from becoming part of its subject.
///
/// # Errors
///
/// Returns a [`ManifestError`] when the repository root cannot be
/// canonicalised, an included path is malformed, missing, non-UTF-8, cycles
/// through symlinks, or resolves outside the repository.
pub fn compute_subject_hash(
    repo_root: &Path,
    decision_repo_path: &str,
    decision_raw: &str,
    affects: &[String],
) -> Result<String, ManifestError> {
    let canonical_root = fs::canonicalize(repo_root).map_err(|error| ManifestError {
        message: format!(
            "cannot canonicalise repository root {}: {error}",
            repo_root.display()
        ),
    })?;
    let decision_path = normalise_or_error(decision_repo_path)?;
    let mut entries = BTreeSet::new();
    entries.insert(decision_path.clone());

    for affects_path in affects {
        match normalise_repo_entry(affects_path).ok_or_else(|| ManifestError {
            message: format!("invalid manifest path {affects_path}"),
        })? {
            RepoPathRule::File(path)
                if is_review_path(&path)
                    || resolves_into_reviews(&canonical_root, &repo_root.join(&path)) => {}
            RepoPathRule::File(path) => {
                entries.insert(path);
            }
            RepoPathRule::Dir(path)
                if is_review_path(&path)
                    || resolves_into_reviews(&canonical_root, &repo_root.join(&path)) => {}
            RepoPathRule::Dir(path) => {
                expand_directory(repo_root, &canonical_root, &path, &mut entries)?;
            }
        }
    }

    let mut manifest = String::new();
    for path in entries {
        if is_review_path(&path) {
            continue;
        }

        let hex = if path == decision_path {
            sha256_hex(governed_content(decision_raw).as_bytes())
        } else {
            sha256_hex(&read_contained_file(repo_root, &canonical_root, &path)?)
        };
        manifest.push_str(&hex);
        manifest.push_str("  ");
        manifest.push_str(&path);
        manifest.push('\n');
    }

    Ok(format!("sha256:{}", sha256_hex(manifest.as_bytes())))
}

/// Reads a decision artefact from disk and computes its subject hash.
///
/// Accepts the stored artefact path in absolute form or the root-prefixed
/// relative form the registry records for nested roots, so the scanner, the
/// pending queue, and the commit hook share one resolution rule and cannot
/// drift.
///
/// # Errors
///
/// Returns a [`ManifestError`] when the stored path is not UTF-8, the file
/// cannot be read, or [`compute_subject_hash`] fails.
pub(crate) fn compute_decision_subject_hash(
    root: &Path,
    decision: &super::Decision,
) -> Result<String, ManifestError> {
    let stored = Path::new(&decision.path);
    let relative = stored.strip_prefix(root).unwrap_or(stored);
    let repo_path = relative.to_str().ok_or_else(|| ManifestError {
        message: format!("decision path {} is not valid UTF-8", stored.display()),
    })?;
    let raw = fs::read_to_string(root.join(repo_path)).map_err(|error| ManifestError {
        message: format!("cannot read decision `{repo_path}`: {error}"),
    })?;
    compute_subject_hash(root, repo_path, &raw, &decision.affects)
}

fn normalise_segments(raw: &str) -> Option<String> {
    valid_repo_path(raw).map(str::to_owned)
}

/// Borrowed validation twin of [`normalise_segments`], for match loops that
/// never need ownership.
fn valid_repo_path(raw: &str) -> Option<&str> {
    let bytes = raw.as_bytes();
    let has_drive_prefix =
        bytes.first().is_some_and(u8::is_ascii_alphabetic) && bytes.get(1) == Some(&b':');
    if raw.is_empty() || raw.starts_with('/') || raw.contains('\\') || has_drive_prefix {
        return None;
    }

    let normalised = raw.trim_end_matches('/');
    if normalised.is_empty()
        || normalised
            .split('/')
            .any(|segment| matches!(segment, "" | "." | ".."))
    {
        return None;
    }

    Some(normalised)
}

fn leading_frontmatter_end(raw: &str) -> Option<usize> {
    let first_end = first_line_end(raw);
    if line_without_ending(&raw[..first_end]) != "---" || first_end == raw.len() {
        return None;
    }

    let mut position = first_end;
    while position < raw.len() {
        let line_end = next_line_end(raw, position);
        if line_without_ending(&raw[position..line_end]).trim() == "---" {
            return Some(position);
        }
        position = line_end;
    }
    None
}

fn first_line_end(raw: &str) -> usize {
    next_line_end(raw, 0)
}

fn next_line_end(raw: &str, start: usize) -> usize {
    raw[start..]
        .find('\n')
        .map_or(raw.len(), |offset| start + offset + 1)
}

fn line_without_ending(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

/// Mirrors `frontmatter::parse` list state: while a block list is open, a row
/// stays inside it unless it is the trimmed closing delimiter or a new
/// top-level `key:` line. Comments, blanks, and junk without a colon are
/// inert to the parser and keep the list open; `- ` rows (with or without
/// colons) and the indented nested `id:` form are items.
fn continues_open_list(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed == "---" {
        return false;
    }
    if trimmed == "-" || trimmed.starts_with("- ") {
        return true;
    }
    match trimmed.split_once(':') {
        None => true,
        Some((key, _)) => key.trim() == "id" && line.starts_with(char::is_whitespace),
    }
}

/// True when a filesystem path canonically resolves inside the review
/// artefact home, so a symlink alias can never pull receipt bytes into a
/// subject manifest.
fn resolves_into_reviews(canonical_root: &Path, filesystem_path: &Path) -> bool {
    let reviews_root = canonical_root.join(REVIEWS_DIR);
    let reviews_canonical = fs::canonicalize(&reviews_root).unwrap_or(reviews_root);
    fs::canonicalize(filesystem_path)
        .is_ok_and(|canonical| canonical.starts_with(&reviews_canonical))
}

/// Classifies a trimmed frontmatter line: `Some(opens_list)` when its key is a
/// ratification key under the same trimmed-key rule `frontmatter::parse`
/// applies (any indentation counts as a top-level key), `None` otherwise.
/// `opens_list` mirrors the parser: an empty value opens a block list whose
/// items are `- ` rows at any indentation, with blank lines tolerated between
/// items.
fn ratification_key_shape(trimmed: &str) -> Option<bool> {
    let (key, value) = trimmed.split_once(':')?;
    if !RATIFICATION_KEYS.contains(&key.trim()) {
        return None;
    }
    Some(value.trim().is_empty())
}

fn normalise_or_error(raw: &str) -> Result<String, ManifestError> {
    normalise_repo_path(raw).ok_or_else(|| ManifestError {
        message: format!("invalid manifest path {raw}"),
    })
}

fn is_review_path(path: &str) -> bool {
    path == REVIEWS_DIR
        || path
            .strip_prefix(REVIEWS_DIR)
            .is_some_and(|remainder| remainder.starts_with('/'))
}

fn expand_directory(
    repo_root: &Path,
    canonical_root: &Path,
    directory: &str,
    entries: &mut BTreeSet<String>,
) -> Result<(), ManifestError> {
    if is_review_path(directory) {
        return Ok(());
    }
    let mut ancestors = BTreeSet::new();
    walk_directory(
        canonical_root,
        &repo_root.join(directory),
        directory,
        &mut ancestors,
        entries,
    )
}

fn walk_directory(
    canonical_root: &Path,
    filesystem_path: &Path,
    repo_path: &str,
    ancestors: &mut BTreeSet<PathBuf>,
    entries: &mut BTreeSet<String>,
) -> Result<(), ManifestError> {
    let canonical = canonicalise_contained(filesystem_path, canonical_root, repo_path)?;
    if !fs::metadata(&canonical)
        .map_err(|error| read_error(repo_path, &error))?
        .is_dir()
    {
        return Err(ManifestError {
            message: format!("manifest directory path {repo_path} is not a directory"),
        });
    }
    if !ancestors.insert(canonical.clone()) {
        return Err(ManifestError {
            message: format!("manifest directory path {repo_path} forms a symlink cycle"),
        });
    }

    let mut children = fs::read_dir(filesystem_path)
        .map_err(|error| read_error(repo_path, &error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| read_error(repo_path, &error))?;
    children.sort_by_key(std::fs::DirEntry::file_name);

    for child in children {
        let name = child.file_name();
        let Some(name) = name.to_str() else {
            return Err(ManifestError {
                message: format!("manifest path under {repo_path} is not valid UTF-8"),
            });
        };
        // Cairn's own state directory is derived output, never governed
        // content: scans write it freely, so hashing it would let a scan
        // drift a subject manifest away from its receipts.
        if name == ".cairn" {
            continue;
        }
        let child_label = format!("{repo_path}/{name}");
        let child_filesystem_path = child.path();
        if is_review_path(&child_label)
            || resolves_into_reviews(canonical_root, &child_filesystem_path)
        {
            continue;
        }
        let canonical_child =
            canonicalise_contained(&child_filesystem_path, canonical_root, &child_label)?;
        let metadata =
            fs::metadata(&canonical_child).map_err(|error| read_error(&child_label, &error))?;
        if metadata.is_dir() {
            walk_directory(
                canonical_root,
                &child_filesystem_path,
                &child_label,
                ancestors,
                entries,
            )?;
        } else if metadata.is_file() {
            entries.insert(child_label);
        }
    }

    ancestors.remove(&canonical);
    Ok(())
}

fn read_contained_file(
    repo_root: &Path,
    canonical_root: &Path,
    path: &str,
) -> Result<Vec<u8>, ManifestError> {
    let filesystem_path = repo_root.join(path);
    let canonical = canonicalise_contained(&filesystem_path, canonical_root, path)?;
    if !fs::metadata(&canonical)
        .map_err(|error| read_error(path, &error))?
        .is_file()
    {
        return Err(ManifestError {
            message: format!("manifest path {path} is not a file"),
        });
    }
    fs::read(canonical).map_err(|error| read_error(path, &error))
}

/// Hashes a repository file after containment checks, for receipt evidence
/// bound to committed files such as lens prompts.
///
/// # Errors
///
/// Returns a [`ManifestError`] when the path is malformed, missing, escapes
/// the repository after symlink resolution, or cannot be read.
pub(crate) fn contained_file_sha256(root: &Path, repo_path: &str) -> Result<String, ManifestError> {
    let canonical_root = fs::canonicalize(root).map_err(|error| ManifestError {
        message: format!(
            "cannot canonicalise repository root {}: {error}",
            root.display()
        ),
    })?;
    let normalised = normalise_or_error(repo_path)?;
    let bytes = read_contained_file(root, &canonical_root, &normalised)?;
    Ok(format!("sha256:{}", sha256_hex(&bytes)))
}

fn canonicalise_contained(
    filesystem_path: &Path,
    canonical_root: &Path,
    label: &str,
) -> Result<PathBuf, ManifestError> {
    let canonical = fs::canonicalize(filesystem_path).map_err(|error| read_error(label, &error))?;
    if canonical.strip_prefix(canonical_root).is_err() {
        return Err(ManifestError {
            message: format!("manifest path {label} resolves outside the repository"),
        });
    }
    Ok(canonical)
}

fn read_error(path: &str, error: &std::io::Error) -> ManifestError {
    ManifestError {
        message: format!("cannot read manifest path {path}: {error}"),
    }
}

#[cfg(test)]
mod tests;
