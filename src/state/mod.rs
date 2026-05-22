//! Pluggable state persistence backend.
//!
//! The `StateBackend` trait abstracts artefact state storage (status, claim,
//! ready-queries) from the filesystem default. Content (markdown bodies,
//! blueprint text) stays as files unconditionally.

use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, io, path::PathBuf};

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
pub trait StateBackend {
    /// Load a record by key. Returns `Ok(None)` when the record does not exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on read failures and `StateError::Serialization`
    /// on malformed JSON.
    fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StateError>;

    /// Save a record by key. Overwrites existing records.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on write failures and `StateError::Serialization`
    /// on serialization failures.
    fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StateError>;

    /// List all record keys.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures.
    fn list(&self) -> Result<Vec<String>, StateError>;

    /// Remove a record by key. Idempotent: succeeds when the record does not
    /// exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on deletion failures.
    fn remove(&self, key: &str) -> Result<(), StateError>;

    /// Query records by type. Returns matching (key, record) pairs.
    ///
    /// Default implementation iterates over all records; backends MAY override
    /// with indexed queries.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on directory read failures and
    /// `StateError::Serialization` on malformed JSON.
    fn query_by_type<T: StateRecord>(
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
    fn query_by_label<T: StateRecord>(&self, label: &str) -> Result<Vec<(String, T)>, StateError> {
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
    fn query_by_dependency<T: StateRecord>(
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
}

impl StateBackend for FilesystemStateBackend {
    fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StateError> {
        let path = self.path_for(key);
        if !path.exists() {
            return Ok(None);
        }
        let json = std::fs::read_to_string(&path)?;
        let value =
            serde_json::from_str(&json).map_err(|e| StateError::Serialization(e.to_string()))?;
        Ok(Some(value))
    }

    fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StateError> {
        std::fs::create_dir_all(&self.root)?;
        let path = self.path_for(key);
        let json = serde_json::to_string_pretty(value)
            .map_err(|e| StateError::Serialization(e.to_string()))?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<String>, StateError> {
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

    fn remove(&self, key: &str) -> Result<(), StateError> {
        let path = self.path_for(key);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestRecord {
        id: String,
        count: u32,
    }

    fn temp_backend() -> FilesystemStateBackend {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("cairn-state-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        FilesystemStateBackend::new(dir)
    }

    #[test]
    fn round_trip_save_and_load() {
        let backend = temp_backend();
        let record = TestRecord {
            id: "test-1".to_owned(),
            count: 42,
        };
        backend.save("rec1", &record).unwrap();
        let loaded: Option<TestRecord> = backend.load("rec1").unwrap();
        assert_eq!(loaded, Some(record));
    }

    #[test]
    fn load_missing_returns_none() {
        let backend = temp_backend();
        let loaded: Option<TestRecord> = backend.load("missing").unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn list_returns_saved_keys() {
        let backend = temp_backend();
        backend
            .save(
                "a",
                &TestRecord {
                    id: "a".to_owned(),
                    count: 1,
                },
            )
            .unwrap();
        backend
            .save(
                "b",
                &TestRecord {
                    id: "b".to_owned(),
                    count: 2,
                },
            )
            .unwrap();
        let mut keys = backend.list().unwrap();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);
    }

    #[test]
    fn list_empty_when_root_missing() {
        let backend = temp_backend();
        let keys = backend.list().unwrap();
        assert!(keys.is_empty());
    }

    #[test]
    fn remove_deletes_record() {
        let backend = temp_backend();
        backend
            .save(
                "del",
                &TestRecord {
                    id: "x".to_owned(),
                    count: 0,
                },
            )
            .unwrap();
        assert!(backend.load::<TestRecord>("del").unwrap().is_some());
        backend.remove("del").unwrap();
        assert!(backend.load::<TestRecord>("del").unwrap().is_none());
    }

    #[test]
    fn remove_missing_is_idempotent() {
        let backend = temp_backend();
        backend.remove("never-existed").unwrap();
    }

    #[test]
    fn save_overwrites_existing() {
        let backend = temp_backend();
        backend
            .save(
                "key",
                &TestRecord {
                    id: "first".to_owned(),
                    count: 1,
                },
            )
            .unwrap();
        backend
            .save(
                "key",
                &TestRecord {
                    id: "second".to_owned(),
                    count: 2,
                },
            )
            .unwrap();
        let loaded: TestRecord = backend.load("key").unwrap().unwrap();
        assert_eq!(loaded.id, "second");
        assert_eq!(loaded.count, 2);
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct QueryableRecord {
        id: String,
        record_type: String,
        labels: Vec<String>,
        dependencies: Vec<String>,
    }

    impl StateRecord for QueryableRecord {
        fn record_type(&self) -> &str {
            &self.record_type
        }

        fn labels(&self) -> &[String] {
            &self.labels
        }

        fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
    }

    #[test]
    fn query_by_type_filters_records() {
        let backend = temp_backend();
        backend
            .save(
                "draft1",
                &QueryableRecord {
                    id: "d1".to_owned(),
                    record_type: "draft".to_owned(),
                    labels: vec![],
                    dependencies: vec![],
                },
            )
            .unwrap();
        backend
            .save(
                "snapshot1",
                &QueryableRecord {
                    id: "s1".to_owned(),
                    record_type: "snapshot".to_owned(),
                    labels: vec![],
                    dependencies: vec![],
                },
            )
            .unwrap();
        let results: Vec<(String, QueryableRecord)> = backend.query_by_type("draft").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "draft1");
        assert_eq!(results[0].1.id, "d1");
    }

    #[test]
    fn query_by_label_filters_records() {
        let backend = temp_backend();
        backend
            .save(
                "rec1",
                &QueryableRecord {
                    id: "r1".to_owned(),
                    record_type: "x".to_owned(),
                    labels: vec!["urgent".to_owned()],
                    dependencies: vec![],
                },
            )
            .unwrap();
        backend
            .save(
                "rec2",
                &QueryableRecord {
                    id: "r2".to_owned(),
                    record_type: "x".to_owned(),
                    labels: vec!["low".to_owned()],
                    dependencies: vec![],
                },
            )
            .unwrap();
        let results: Vec<(String, QueryableRecord)> = backend.query_by_label("urgent").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "rec1");
    }

    #[test]
    fn query_by_dependency_filters_records() {
        let backend = temp_backend();
        backend
            .save(
                "rec1",
                &QueryableRecord {
                    id: "r1".to_owned(),
                    record_type: "x".to_owned(),
                    labels: vec![],
                    dependencies: vec!["db".to_owned()],
                },
            )
            .unwrap();
        backend
            .save(
                "rec2",
                &QueryableRecord {
                    id: "r2".to_owned(),
                    record_type: "x".to_owned(),
                    labels: vec![],
                    dependencies: vec!["api".to_owned()],
                },
            )
            .unwrap();
        let results: Vec<(String, QueryableRecord)> = backend.query_by_dependency("db").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "rec1");
    }
}
