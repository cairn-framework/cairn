//! Pluggable state persistence backend.
//!
//! The `StateBackend` enum abstracts artefact state storage (status, claim,
//! ready-queries) from the filesystem default. Content (markdown bodies,
//! blueprint text) stays as files unconditionally.

use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, io, path::PathBuf};
pub(crate) mod beads;
pub use beads::BeadsStateBackend;
#[cfg(test)]
mod tests;

/// Error type for state backend operations.
#[derive(Debug)]
pub enum StateError {
    /// I/O failure.
    Io(io::Error),
    /// Serialization or deserialization failure.
    Serialization(String),
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::Io(e) => write!(f, "I/O error: {e}"),
            StateError::Serialization(msg) => write!(f, "serialization error: {msg}"),
        }
    }
}

impl std::error::Error for StateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StateError::Io(e) => Some(e),
            StateError::Serialization(_) => None,
        }
    }
}

impl From<io::Error> for StateError {
    fn from(e: io::Error) -> Self {
        StateError::Io(e)
    }
}

impl From<serde_json::Error> for StateError {
    fn from(e: serde_json::Error) -> Self {
        StateError::Serialization(e.to_string())
    }
}

/// Record metadata required for querying.
pub trait StateRecord: DeserializeOwned {
    /// The logical type of this record (e.g., "draft", "snapshot").
    fn record_type(&self) -> &str;

    /// Labels attached to this record for filtering.
    fn labels(&self) -> &[String] {
        &[]
    }

    /// Dependencies referenced by this record.
    fn dependencies(&self) -> &[String] {
        &[]
    }
}

/// Pluggable backend for artefact state persistence.
#[derive(Clone, Debug)]
pub enum StateBackend {
    /// Filesystem-backed JSON store.
    Filesystem(FilesystemStateBackend),
    /// Beads key-value store backend.
    Beads(beads::BeadsStateBackend),
}

impl StateBackend {
    /// Load a record by key. Returns `Ok(None)` when the record does not exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on read failures and `StateError::Serialization`
    /// on malformed JSON.
    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StateError> {
        match self {
            Self::Filesystem(fs) => fs.load(key),
            Self::Beads(b) => b.load(key),
        }
    }

    /// Save a record by key. Overwrites existing records.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on write failures and `StateError::Serialization`
    /// on serialization failures.
    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StateError> {
        match self {
            Self::Filesystem(fs) => fs.save(key, value),
            Self::Beads(b) => b.save(key, value),
        }
    }

    /// List all record keys.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures.
    pub fn list(&self) -> Result<Vec<String>, StateError> {
        match self {
            Self::Filesystem(fs) => fs.list(),
            Self::Beads(b) => b.list(),
        }
    }

    /// Remove a record by key. Idempotent: succeeds when the record does not
    /// exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on deletion failures.
    pub fn remove(&self, key: &str) -> Result<(), StateError> {
        match self {
            Self::Filesystem(fs) => fs.remove(key),
            Self::Beads(b) => b.remove(key),
        }
    }

    /// Query records by type. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    pub fn query_by_type<T: StateRecord>(
        &self,
        record_type: &str,
    ) -> Result<Vec<(String, T)>, StateError> {
        match self {
            Self::Filesystem(fs) => fs.query_by_type(record_type),
            Self::Beads(b) => b.query_by_type(record_type),
        }
    }

    /// Query records by label. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    pub fn query_by_label<T: StateRecord>(
        &self,
        label: &str,
    ) -> Result<Vec<(String, T)>, StateError> {
        match self {
            Self::Filesystem(fs) => fs.query_by_label(label),
            Self::Beads(b) => b.query_by_label(label),
        }
    }

    /// Query records by dependency. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    pub fn query_by_dependency<T: StateRecord>(
        &self,
        dependency: &str,
    ) -> Result<Vec<(String, T)>, StateError> {
        match self {
            Self::Filesystem(fs) => fs.query_by_dependency(dependency),
            Self::Beads(b) => b.query_by_dependency(dependency),
        }
    }
}

/// Filesystem-backed state store. Records are stored as individual JSON files
/// under the root directory, keyed by filename (`.json` suffix appended).
#[derive(Clone, Debug)]
pub struct FilesystemStateBackend {
    root: PathBuf,
}

impl FilesystemStateBackend {
    /// Create a new filesystem backend rooted at the given directory.
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn path_for(&self, key: &str) -> PathBuf {
        self.root.join(format!("{key}.json"))
    }

    /// Load a record by key. Returns `Ok(None)` when the record does not exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on read failures and `StateError::Serialization`
    /// on malformed JSON.
    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StateError> {
        let path = self.path_for(key);
        if !path.exists() {
            return Ok(None);
        }
        let json = std::fs::read_to_string(&path)?;
        let value =
            serde_json::from_str(&json).map_err(|e| StateError::Serialization(e.to_string()))?;
        Ok(Some(value))
    }

    /// Save a record by key. Overwrites existing records.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on write failures and `StateError::Serialization`
    /// on serialization failures.
    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StateError> {
        std::fs::create_dir_all(&self.root)?;
        let path = self.path_for(key);
        let json = serde_json::to_string_pretty(value)
            .map_err(|e| StateError::Serialization(e.to_string()))?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    /// List all record keys.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures.
    pub fn list(&self) -> Result<Vec<String>, StateError> {
        if !self.root.exists() {
            return Ok(Vec::new());
        }
        let mut keys = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(stem) = name.strip_suffix(".json") {
                keys.push(stem.to_owned());
            }
        }
        Ok(keys)
    }

    /// Remove a record by key. Idempotent: succeeds when the record does not
    /// exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on deletion failures.
    pub fn remove(&self, key: &str) -> Result<(), StateError> {
        let path = self.path_for(key);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// Query records by type. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    pub fn query_by_type<T: StateRecord>(
        &self,
        record_type: &str,
    ) -> Result<Vec<(String, T)>, StateError> {
        let mut results = Vec::new();
        for key in self.list()? {
            if let Some(record) = self.load::<T>(&key)?
                && record.record_type() == record_type
            {
                results.push((key, record));
            }
        }
        Ok(results)
    }

    /// Query records by label. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    pub fn query_by_label<T: StateRecord>(
        &self,
        label: &str,
    ) -> Result<Vec<(String, T)>, StateError> {
        let mut results = Vec::new();
        for key in self.list()? {
            if let Some(record) = self.load::<T>(&key)?
                && record.labels().contains(&label.to_owned())
            {
                results.push((key, record));
            }
        }
        Ok(results)
    }

    /// Query records by dependency. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    pub fn query_by_dependency<T: StateRecord>(
        &self,
        dependency: &str,
    ) -> Result<Vec<(String, T)>, StateError> {
        let mut results = Vec::new();
        for key in self.list()? {
            if let Some(record) = self.load::<T>(&key)?
                && record.dependencies().contains(&dependency.to_owned())
            {
                results.push((key, record));
            }
        }
        Ok(results)
    }
}

/// Create a state backend from a backend name and root path.
///
/// Supported backends:
/// - `filesystem` (default): JSON files under the root directory.
/// - `beads`: Beads key-value store.
///
/// # Errors
///
/// Returns `StateError::Serialization` for unknown backend names.
pub fn storage_backend(name: &str, root: PathBuf) -> Result<StateBackend, StateError> {
    match name {
        "filesystem" => Ok(StateBackend::Filesystem(FilesystemStateBackend::new(root))),
        "beads" => Ok(StateBackend::Beads(beads::BeadsStateBackend::new(root))),
        _ => Err(StateError::Serialization(format!(
            "unknown state backend: {name}"
        ))),
    }
}
