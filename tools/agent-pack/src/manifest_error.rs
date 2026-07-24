//! Validation error types for agent pack manifests.

use crate::containment::ContainmentError;
use std::fmt;
use std::path::PathBuf;

/// Errors occurring during manifest parsing or schema/ownership validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ManifestValidationError {
    /// TOML parsing failed.
    ParseError(String),
    /// Invalid schema version.
    InvalidSchemaVersion {
        /// Found schema version.
        found: u32,
    },
    /// Bundle version is empty.
    EmptyBundleVersion,
    /// Adapter specifies an entry/mode that has no canonical producer.
    UnownedAdapterEntry {
        /// Row index in manifest.
        index: usize,
        /// Entry name.
        entry: String,
        /// Mode name.
        mode: String,
    },
    /// Duplicate canonical owner for logical entry-mode pair.
    DuplicateCanonicalEntryMode {
        /// Entry name.
        entry: String,
        /// Mode name.
        mode: String,
        /// First canonical row index.
        first_index: usize,
        /// Duplicate canonical row index.
        duplicate_index: usize,
    },
    /// Duplicate producer for a harness and logical entry-mode pair.
    DuplicateEntryModeProducer {
        /// Harness name.
        harness: String,
        /// Entry name.
        entry: String,
        /// Mode name.
        mode: String,
        /// First adapter row index.
        first_index: usize,
        /// Duplicate adapter row index.
        duplicate_index: usize,
    },
    /// Duplicate producer for normalized destination path.
    DuplicateDestinationProducer {
        /// First declared destination.
        first: PathBuf,
        /// Duplicate declared destination.
        second: PathBuf,
        /// First adapter row index.
        first_index: usize,
        /// Duplicate adapter row index.
        duplicate_index: usize,
    },
    /// One destination is a strict ancestor of another.
    DestinationHierarchyCollision {
        /// Earlier normalized destination.
        first: PathBuf,
        /// Later normalized destination.
        second: PathBuf,
        /// Earlier adapter row index.
        first_index: usize,
        /// Later adapter row index.
        second_index: usize,
    },
    /// Path failed lexical containment check.
    LexicalContainmentError {
        /// Row table name ("canonical" or "adapters").
        row_type: &'static str,
        /// Row index in manifest.
        index: usize,
        /// Path that failed.
        path: PathBuf,
        /// Containment error.
        err: ContainmentError,
    },
}

impl fmt::Display for ManifestValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(message) => {
                write!(
                    f,
                    "manifest parse error: {message}. Correct the TOML syntax"
                )
            }
            Self::InvalidSchemaVersion { found } => write!(
                f,
                "unsupported schema_version {found}; set schema_version = 1"
            ),
            Self::EmptyBundleVersion => {
                write!(
                    f,
                    "bundle_version is empty; set it to the pack release version"
                )
            }
            Self::UnownedAdapterEntry { index, entry, mode } => write!(
                f,
                "row [[adapters]] index {index}: entry '{entry}' mode '{mode}' has no canonical owner; add the matching [[canonical]] row or correct this row"
            ),
            Self::DuplicateCanonicalEntryMode {
                entry,
                mode,
                first_index,
                duplicate_index,
            } => write!(
                f,
                "duplicate canonical owner for entry '{entry}' mode '{mode}' at [[canonical]] indexes {first_index} and {duplicate_index}; remove one row or give it a distinct entry-mode key"
            ),
            Self::DuplicateEntryModeProducer {
                harness,
                entry,
                mode,
                first_index,
                duplicate_index,
            } => write!(
                f,
                "duplicate producer for harness '{harness}' entry '{entry}' mode '{mode}' at [[adapters]] indexes {first_index} and {duplicate_index}; remove one row or give it a distinct harness entry-mode key"
            ),
            Self::DuplicateDestinationProducer {
                first,
                second,
                first_index,
                duplicate_index,
            } => write!(
                f,
                "duplicate producers for filesystem-equivalent destinations '{}' and '{}' at [[adapters]] indexes {first_index} and {duplicate_index}; change one destination or remove the duplicate row",
                first.display(),
                second.display()
            ),
            Self::DestinationHierarchyCollision {
                first,
                second,
                first_index,
                second_index,
            } => write!(
                f,
                "destination hierarchy collision between '{}' and '{}' at [[adapters]] indexes {first_index} and {second_index}; choose destinations that are not ancestors of one another",
                first.display(),
                second.display()
            ),
            Self::LexicalContainmentError {
                row_type,
                index,
                path,
                err,
            } => write!(
                f,
                "row [[{row_type}]] index {index} path '{}': {err}; replace it with a non-empty project-relative path containing no parent traversal",
                path.display()
            ),
        }
    }
}

impl std::error::Error for ManifestValidationError {}
