//! Lexical and resolved path containment validation functions.
//!
//! These functions serve as safety checks to prevent path traversal attacks,
//! lexical escapes, symlink redirection, and writes outside intended directories.

use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Error conditions encountered during containment validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainmentError {
    /// Path was empty.
    EmptyPath,
    /// Path is absolute or contains a root/prefix component.
    NotRelative {
        /// The path that failed validation.
        path: PathBuf,
    },
    /// Path syntax would resolve differently across supported platforms.
    NonPortable {
        /// The path that failed validation.
        path: PathBuf,
    },
    /// Path contained illegal components such as parent directory `..`.
    ParentTraversal {
        /// The path that contained a parent directory component.
        path: PathBuf,
    },
    /// Root directory canonicalization failed.
    RootNotFound {
        /// Root path that could not be resolved.
        root: PathBuf,
        /// Underlying error message.
        details: String,
    },
    /// Resolved target path escaped root directory boundaries.
    ResolvedEscape {
        /// Target path that escaped the root.
        target: PathBuf,
        /// Root path expected to contain target.
        root: PathBuf,
    },
    /// Path contained a symlink component before write.
    SymlinkDetected {
        /// Path component identified as a symlink.
        symlink_path: PathBuf,
    },
}

impl fmt::Display for ContainmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => write!(f, "path is empty"),
            Self::NotRelative { path } => {
                write!(f, "path '{}' is not relative", path.display())
            }
            Self::NonPortable { path } => write!(
                f,
                "path '{}' is not portable; use ASCII forward-slash project-relative syntax without drive or UNC prefixes",
                path.display()
            ),
            Self::ParentTraversal { path } => {
                write!(
                    f,
                    "path '{}' contains parent traversal component ('..')",
                    path.display()
                )
            }
            Self::RootNotFound { root, details } => {
                write!(
                    f,
                    "root directory '{}' could not be resolved: {}",
                    root.display(),
                    details
                )
            }
            Self::ResolvedEscape { target, root } => {
                write!(
                    f,
                    "resolved path '{}' escapes root directory '{}'",
                    target.display(),
                    root.display()
                )
            }
            Self::SymlinkDetected { symlink_path } => {
                write!(f, "symlink detected at path '{}'", symlink_path.display())
            }
        }
    }
}

impl std::error::Error for ContainmentError {}

/// Validates that a path is relative and lexically contained.
///
/// Accepts only non-empty relative paths composed entirely of [`Component::Normal`]
/// and [`Component::CurDir`] components. Rejects parent traversal (`..`),
/// prefixes, and root directories.
///
/// # Errors
///
/// Returns [`ContainmentError`] if the path is empty, non-relative, or contains `..`.
pub fn validate_lexical_containment(path: &Path) -> Result<(), ContainmentError> {
    if path.as_os_str().is_empty() {
        return Err(ContainmentError::EmptyPath);
    }
    let portable = path.to_str().is_some_and(|value| {
        let bytes = value.as_bytes();
        !(value.contains('\\')
            || !value.is_ascii()
            || bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':')
    });
    if !portable {
        return Err(ContainmentError::NonPortable {
            path: path.to_path_buf(),
        });
    }

    for comp in path.components() {
        match comp {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => {
                return Err(ContainmentError::ParentTraversal {
                    path: path.to_path_buf(),
                });
            }
            Component::Prefix(_) | Component::RootDir => {
                return Err(ContainmentError::NotRelative {
                    path: path.to_path_buf(),
                });
            }
        }
    }

    Ok(())
}

/// Validates that the resolved location of `candidate` stays within `root`.
///
/// The candidate may be absolute or relative to `root`. The deepest existing
/// ancestor is canonicalised so the check also works for a not-yet-created leaf.
///
/// # Errors
///
/// Returns [`ContainmentError`] when the root cannot be resolved or the
/// candidate's existing ancestry resolves outside it.
pub fn validate_resolved_path(root: &Path, candidate: &Path) -> Result<PathBuf, ContainmentError> {
    if candidate
        .components()
        .any(|component| component == Component::ParentDir)
    {
        return Err(ContainmentError::ParentTraversal {
            path: candidate.to_path_buf(),
        });
    }
    let canonical_root = canonical_root(root)?;
    let target = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };
    let existing =
        deepest_existing_ancestor(&target).ok_or_else(|| ContainmentError::RootNotFound {
            root: target.clone(),
            details: "path has no existing ancestor".to_string(),
        })?;
    let resolved = fs::canonicalize(existing).map_err(|error| ContainmentError::RootNotFound {
        root: existing.to_path_buf(),
        details: error.to_string(),
    })?;
    if !resolved.starts_with(&canonical_root) {
        return Err(ContainmentError::ResolvedEscape {
            target: resolved,
            root: canonical_root,
        });
    }
    Ok(target)
}

fn canonical_root(root: &Path) -> Result<PathBuf, ContainmentError> {
    fs::canonicalize(root).map_err(|error| ContainmentError::RootNotFound {
        root: root.to_path_buf(),
        details: error.to_string(),
    })
}

fn deepest_existing_ancestor(path: &Path) -> Option<&Path> {
    path.ancestors()
        .find(|ancestor| fs::symlink_metadata(ancestor).is_ok())
}

/// Validates that `rel_path` when resolved under `root` stays strictly contained within `root`.
///
/// Canonicalizes `root` and verifies each existing path component down to the deepest existing ancestor.
/// Rejects any intermediate symlinks or resolutions that escape `root`.
///
/// # Errors
///
/// Returns [`ContainmentError`] if lexical containment fails, `root` is invalid, a symlink is hit,
/// or resolved canonical path escapes `root`.
pub fn validate_resolved_containment(
    root: &Path,
    rel_path: &Path,
) -> Result<PathBuf, ContainmentError> {
    validate_lexical_containment(rel_path)?;
    let mut current = canonical_root(root)?;
    for component in rel_path.components() {
        if let Component::Normal(name) = component {
            current.push(name);
            if let Ok(metadata) = fs::symlink_metadata(&current)
                && metadata.file_type().is_symlink()
            {
                return Err(ContainmentError::SymlinkDetected {
                    symlink_path: current,
                });
            }
        }
    }
    validate_resolved_path(root, rel_path)
}
