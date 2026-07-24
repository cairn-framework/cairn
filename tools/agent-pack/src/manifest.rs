//! Manifest definition, TOML parsing, and validation for agent packs.

use crate::containment::validate_lexical_containment;
use crate::manifest_error::ManifestValidationError;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Top-level agent pack manifest definition matching `schema_version = 1`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PackManifest {
    /// Schema version integer, must be 1.
    pub(crate) schema_version: u32,
    /// Non-empty bundle release version.
    pub(crate) bundle_version: String,
    /// Canonical source rows.
    #[serde(default)]
    pub(crate) canonical: Vec<CanonicalRow>,
    /// Adapter output rows.
    #[serde(default)]
    pub(crate) adapters: Vec<AdapterRow>,
}

/// A canonical asset source row in the pack manifest.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CanonicalRow {
    /// Logical entry name.
    pub(crate) entry: String,
    /// Explicit opaque mode string.
    pub(crate) mode: String,
    /// Relative path to canonical source file.
    pub(crate) source: PathBuf,
}

/// An adapter output destination row in the pack manifest.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AdapterRow {
    /// Target harness name.
    pub(crate) harness: String,
    /// Logical entry name.
    pub(crate) entry: String,
    /// Explicit opaque mode string.
    pub(crate) mode: String,
    /// Relative path to destination output file.
    pub(crate) destination: PathBuf,
}

impl PackManifest {
    /// Parses a manifest TOML string and performs full schema and ownership validation.
    ///
    /// # Errors
    ///
    /// Returns [`ManifestValidationError`] if parse fails or any validation rule is violated.
    pub(crate) fn parse_str(content: &str) -> Result<Self, ManifestValidationError> {
        let manifest: Self = toml::from_str(content)
            .map_err(|e| ManifestValidationError::ParseError(e.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Performs schema, containment, canonical ownership, and producer uniqueness validation.
    ///
    /// # Errors
    ///
    /// Returns [`ManifestValidationError`] on any policy or constraint failure.
    fn validate(&self) -> Result<(), ManifestValidationError> {
        if self.schema_version != 1 {
            return Err(ManifestValidationError::InvalidSchemaVersion {
                found: self.schema_version,
            });
        }
        if self.bundle_version.trim().is_empty() {
            return Err(ManifestValidationError::EmptyBundleVersion);
        }

        let mut canonical_entries: HashMap<(String, String), usize> = HashMap::new();

        for (idx, c) in self.canonical.iter().enumerate() {
            if let Err(err) = validate_lexical_containment(&c.source) {
                return Err(ManifestValidationError::LexicalContainmentError {
                    row_type: "canonical",
                    index: idx,
                    path: c.source.clone(),
                    err,
                });
            }
            let key = (c.entry.clone(), c.mode.clone());
            if let Some(&first_idx) = canonical_entries.get(&key) {
                return Err(ManifestValidationError::DuplicateCanonicalEntryMode {
                    entry: c.entry.clone(),
                    mode: c.mode.clone(),
                    first_index: first_idx,
                    duplicate_index: idx,
                });
            }
            canonical_entries.insert(key, idx);
        }

        let mut seen_entry_modes: HashMap<(String, String, String), usize> = HashMap::new();
        let mut seen_destinations: HashMap<PathBuf, (usize, PathBuf)> = HashMap::new();
        let mut seen_destination_rows: Vec<(PathBuf, PathBuf, usize)> = Vec::new();

        for (idx, a) in self.adapters.iter().enumerate() {
            if let Err(err) = validate_lexical_containment(&a.destination) {
                return Err(ManifestValidationError::LexicalContainmentError {
                    row_type: "adapters",
                    index: idx,
                    path: a.destination.clone(),
                    err,
                });
            }

            let canonical_key = (a.entry.clone(), a.mode.clone());
            if !canonical_entries.contains_key(&canonical_key) {
                return Err(ManifestValidationError::UnownedAdapterEntry {
                    index: idx,
                    entry: a.entry.clone(),
                    mode: a.mode.clone(),
                });
            }

            let producer_key = (a.harness.clone(), a.entry.clone(), a.mode.clone());
            if let Some(&first_idx) = seen_entry_modes.get(&producer_key) {
                return Err(ManifestValidationError::DuplicateEntryModeProducer {
                    harness: a.harness.clone(),
                    entry: a.entry.clone(),
                    mode: a.mode.clone(),
                    first_index: first_idx,
                    duplicate_index: idx,
                });
            }
            seen_entry_modes.insert(producer_key, idx);

            let normalized_dest = normalize_path(&a.destination);
            let identity = filesystem_identity(&normalized_dest);
            if let Some((first_index, first)) = seen_destinations.get(&identity) {
                return Err(ManifestValidationError::DuplicateDestinationProducer {
                    first: first.clone(),
                    second: normalized_dest,
                    first_index: *first_index,
                    duplicate_index: idx,
                });
            }
            for (prior_identity, prior, prior_index) in &seen_destination_rows {
                if identity.starts_with(prior_identity) || prior_identity.starts_with(&identity) {
                    return Err(ManifestValidationError::DestinationHierarchyCollision {
                        first: prior.clone(),
                        second: normalized_dest,
                        first_index: *prior_index,
                        second_index: idx,
                    });
                }
            }
            seen_destination_rows.push((identity.clone(), normalized_dest.clone(), idx));
            seen_destinations.insert(identity, (idx, normalized_dest));
        }

        Ok(())
    }
}

/// Normalizes a path by removing `CurDir` components.
fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for comp in path.components() {
        if comp != std::path::Component::CurDir {
            normalized.push(comp);
        }
    }
    if normalized.as_os_str().is_empty() {
        path.to_path_buf()
    } else {
        normalized
    }
}

fn filesystem_identity(path: &Path) -> PathBuf {
    PathBuf::from(path.to_string_lossy().to_lowercase())
}
