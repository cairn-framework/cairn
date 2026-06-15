//! Beads-backed state store. Records are stored as JSON strings in the beads
//! key-value store, keyed with a `cairn:state:` prefix.

use serde::{Serialize, de::DeserializeOwned};
use std::path::PathBuf;

use super::{StateError, StateRecord};

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
