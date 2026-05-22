//! Pluggable state persistence backend.
//!
//! The `StateBackend` enum abstracts artefact state storage (status, claim,
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
    Beads(BeadsStateBackend),
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
/// Beads-backed state store. Records are stored as JSON strings in the beads
/// key-value store, keyed with a `cairn:state:` prefix.
#[derive(Clone, Debug)]
pub struct BeadsStateBackend {
    root: PathBuf,
    prefix: String,
}

impl BeadsStateBackend {
    /// Create a new beads backend rooted at the given project directory.
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            prefix: "cairn:state:".to_owned(),
        }
    }

    fn full_key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }

    fn bd_kv_output(&self, args: &[&str]) -> Result<std::process::Output, StateError> {
        std::process::Command::new("bd")
            .arg("-C")
            .arg(&self.root)
            .arg("kv")
            .args(args)
            .output()
            .map_err(StateError::Io)
    }

    /// Load a record by key. Returns `Ok(None)` when the record does not exist.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed JSON or bd errors.
    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StateError> {
        let output = self.bd_kv_output(&["get", &self.full_key(key)])?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !output.status.success() && stdout.contains("(not set)") {
            return Ok(None);
        }
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(StateError::Serialization(format!(
                "bd kv get failed: {stderr}"
            )));
        }
        let trimmed = stdout.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        let value =
            serde_json::from_str(trimmed).map_err(|e| StateError::Serialization(e.to_string()))?;
        Ok(Some(value))
    }

    /// Save a record by key. Overwrites existing records.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on serialization or bd errors.
    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StateError> {
        let json =
            serde_json::to_string(value).map_err(|e| StateError::Serialization(e.to_string()))?;
        let output = self.bd_kv_output(&["set", &self.full_key(key), &json])?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(StateError::Serialization(format!(
                "bd kv set failed: {stderr}"
            )));
        }
        Ok(())
    }

    /// List all record keys.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed JSON or bd errors.
    pub fn list(&self) -> Result<Vec<String>, StateError> {
        let output = self.bd_kv_output(&["list", "--json"])?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(StateError::Serialization(format!(
                "bd kv list failed: {stderr}"
            )));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }
        let map: std::collections::BTreeMap<String, serde_json::Value> =
            serde_json::from_str(trimmed).map_err(|e| StateError::Serialization(e.to_string()))?;
        let mut keys = Vec::new();
        for key in map.keys() {
            if let Some(stem) = key.strip_prefix(&self.prefix) {
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
    /// Returns `StateError::Io` on subprocess failures.
    pub fn remove(&self, key: &str) -> Result<(), StateError> {
        let _ = self.bd_kv_output(&["clear", &self.full_key(key)])?;
        Ok(())
    }

    /// Query records by type. Returns matching (key, record) pairs.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed JSON or bd errors.
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
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed JSON or bd errors.
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
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed JSON or bd errors.
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
    /// Create an epic bead for a change and return its bead ID.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed output or bd errors.
    pub fn create_change_epic(&self, change_id: &str) -> Result<String, StateError> {
        let title = format!("Change: {change_id}");
        let output = std::process::Command::new("bd")
            .arg("-C")
            .arg(&self.root)
            .arg("create")
            .arg(&title)
            .arg("--type")
            .arg("epic")
            .arg("--silent")
            .output()
            .map_err(StateError::Io)?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(StateError::Serialization(format!(
                "bd create epic failed: {stderr}"
            )));
        }
        let bead_id = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if bead_id.is_empty() {
            return Err(StateError::Serialization(
                "bd create epic returned empty bead ID".to_owned(),
            ));
        }
        Ok(bead_id)
    }
    /// Create task beads as children of an epic with sequential needs edges.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed output or bd errors.
    pub fn create_task_beads(
        &self,
        epic_id: &str,
        tasks: &[&str],
    ) -> Result<Vec<String>, StateError> {
        let mut task_ids = Vec::new();
        for (idx, task) in tasks.iter().enumerate() {
            let title = format!("Task {idx}: {task}");
            let output = std::process::Command::new("bd")
                .arg("-C")
                .arg(&self.root)
                .arg("create")
                .arg(&title)
                .arg("--type")
                .arg("task")
                .arg("--parent")
                .arg(epic_id)
                .arg("--silent")
                .output()
                .map_err(StateError::Io)?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(StateError::Serialization(format!(
                    "bd create task failed: {stderr}"
                )));
            }
            let bead_id = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if bead_id.is_empty() {
                return Err(StateError::Serialization(
                    "bd create task returned empty bead ID".to_owned(),
                ));
            }
            if let Some(prev) = task_ids.last() {
                let dep_output = std::process::Command::new("bd")
                    .arg("-C")
                    .arg(&self.root)
                    .arg("dep")
                    .arg("add")
                    .arg(&bead_id)
                    .arg(prev)
                    .output()
                    .map_err(StateError::Io)?;
                if !dep_output.status.success() {
                    let stderr = String::from_utf8_lossy(&dep_output.stderr);
                    return Err(StateError::Serialization(format!(
                        "bd dep add failed: {stderr}"
                    )));
                }
            }
            task_ids.push(bead_id);
        }
        Ok(task_ids)
    }
    /// List child task beads of an epic.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed output or bd errors.
    pub fn list_child_tasks(&self, epic_id: &str) -> Result<Vec<(String, String)>, StateError> {
        let output = std::process::Command::new("bd")
            .arg("-C")
            .arg(&self.root)
            .arg("children")
            .arg(epic_id)
            .arg("--json")
            .output()
            .map_err(StateError::Io)?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(StateError::Serialization(format!(
                "bd children failed: {stderr}"
            )));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();
        if trimmed.is_empty() || trimmed == "[]" {
            return Ok(Vec::new());
        }
        let children: Vec<serde_json::Value> =
            serde_json::from_str(trimmed).map_err(|e| StateError::Serialization(e.to_string()))?;
        let mut tasks = Vec::new();
        for child in children {
            if let (Some(id), Some(title)) = (
                child.get("id").and_then(|v| v.as_str()),
                child.get("title").and_then(|v| v.as_str()),
            ) {
                tasks.push((id.to_owned(), title.to_owned()));
            }
        }
        Ok(tasks)
    }

    /// Claim an epic and all its open child tasks.
    ///
    /// # Errors
    ///
    /// Returns `StateError::Io` on subprocess failures and `StateError::Serialization`
    /// on malformed output or bd errors.
    pub fn claim_change(&self, epic_id: &str) -> Result<(), StateError> {
        let epic_output = std::process::Command::new("bd")
            .arg("-C")
            .arg(&self.root)
            .arg("update")
            .arg(epic_id)
            .arg("--claim")
            .output()
            .map_err(StateError::Io)?;
        if !epic_output.status.success() {
            let stderr = String::from_utf8_lossy(&epic_output.stderr);
            return Err(StateError::Serialization(format!(
                "bd claim epic failed: {stderr}"
            )));
        }
        let children = self.list_child_tasks(epic_id)?;
        for (child_id, _) in children {
            let child_output = std::process::Command::new("bd")
                .arg("-C")
                .arg(&self.root)
                .arg("update")
                .arg(&child_id)
                .arg("--claim")
                .output()
                .map_err(StateError::Io)?;
            if !child_output.status.success() {
                let stderr = String::from_utf8_lossy(&child_output.stderr);
                return Err(StateError::Serialization(format!(
                    "bd claim task {child_id} failed: {stderr}"
                )));
            }
        }
        Ok(())
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
        "beads" => Ok(StateBackend::Beads(BeadsStateBackend::new(root))),
        _ => Err(StateError::Serialization(format!(
            "unknown state backend: {name}"
        ))),
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

    #[test]
    fn storage_backend_filesystem_returns_working_backend() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir =
            std::env::temp_dir().join(format!("cairn-state-be-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let backend = storage_backend("filesystem", dir.clone()).unwrap();
        backend
            .save(
                "key",
                &TestRecord {
                    id: "x".to_owned(),
                    count: 1,
                },
            )
            .unwrap();
        let loaded: Option<TestRecord> = backend.load("key").unwrap();
        assert_eq!(loaded.unwrap().id, "x");
    }

    #[test]
    fn storage_backend_unknown_returns_error() {
        let dir = std::env::temp_dir().join("cairn-state-be-unknown");
        let result = storage_backend("s3", dir);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("unknown state backend"));
        assert!(err.contains("s3"));
    }
    #[test]
    fn storage_backend_beads_returns_working_backend() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let backend = storage_backend("beads", root).unwrap();
        let key = format!("test-{}", std::process::id());
        backend
            .save(
                &key,
                &TestRecord {
                    id: "x".to_owned(),
                    count: 1,
                },
            )
            .unwrap();
        let loaded: Option<TestRecord> = backend.load(&key).unwrap();
        assert_eq!(loaded.unwrap().id, "x");
        backend.remove(&key).unwrap();
    }
    #[test]
    fn beads_create_change_epic_returns_bead_id() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let backend = BeadsStateBackend::new(root);
        let change_id = format!("test-epic-{}", std::process::id());
        let bead_id = backend.create_change_epic(&change_id).unwrap();
        assert!(!bead_id.is_empty(), "bead ID must not be empty");
        assert!(
            bead_id.starts_with("cairn-"),
            "bead ID should start with cairn- prefix, got: {bead_id}"
        );
        let _ = std::process::Command::new("bd")
            .arg("delete")
            .arg(&bead_id)
            .arg("--force")
            .output();
    }
    #[test]
    fn beads_create_task_beads_returns_bead_ids() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let backend = BeadsStateBackend::new(root);
        let change_id = format!("test-tasks-{}", std::process::id());
        let epic_id = backend.create_change_epic(&change_id).unwrap();
        let task_ids = backend
            .create_task_beads(&epic_id, &["First task", "Second task"])
            .unwrap();
        assert_eq!(task_ids.len(), 2, "expected 2 task beads");
        for id in &task_ids {
            assert!(
                id.starts_with("cairn-"),
                "task bead ID should start with cairn-"
            );
        }
        for id in task_ids {
            let _ = std::process::Command::new("bd")
                .arg("delete")
                .arg(&id)
                .arg("--force")
                .output();
        }
        let _ = std::process::Command::new("bd")
            .arg("delete")
            .arg(&epic_id)
            .arg("--force")
            .output();
    }

    #[test]
    fn beads_list_child_tasks_returns_task_titles() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let backend = BeadsStateBackend::new(root);
        let change_id = format!("test-list-{}", std::process::id());
        let epic_id = backend.create_change_epic(&change_id).unwrap();
        let task_ids = backend
            .create_task_beads(&epic_id, &["Alpha", "Beta"])
            .unwrap();
        let tasks = backend.list_child_tasks(&epic_id).unwrap();
        assert_eq!(tasks.len(), 2, "expected 2 child tasks");
        for id in task_ids {
            let _ = std::process::Command::new("bd")
                .arg("delete")
                .arg(&id)
                .arg("--force")
                .output();
        }
        let _ = std::process::Command::new("bd")
            .arg("delete")
            .arg(&epic_id)
            .arg("--force")
            .output();
    }

    #[test]
    fn beads_claim_change_succeeds() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let backend = BeadsStateBackend::new(root);
        let change_id = format!("test-claim-{}", std::process::id());
        let epic_id = backend.create_change_epic(&change_id).unwrap();
        let task_ids = backend
            .create_task_beads(&epic_id, &["One", "Two"])
            .unwrap();
        backend.claim_change(&epic_id).unwrap();
        for id in task_ids {
            let _ = std::process::Command::new("bd")
                .arg("delete")
                .arg(&id)
                .arg("--force")
                .output();
        }
        let _ = std::process::Command::new("bd")
            .arg("delete")
            .arg(&epic_id)
            .arg("--force")
            .output();
    }
}
