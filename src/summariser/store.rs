//! Draft store: pending/accepted/edited/discarded drafts under
//! `.cairn/state/summariser/`.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

/// Wire schema version for serialised request/draft state.
pub const REQUEST_SCHEMA_VERSION: u32 = 1;

/// Lifecycle status of a draft.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DraftStatus {
    /// Draft is awaiting human resolution.
    #[default]
    Pending,
    /// Draft has been applied to the target contract.
    Accepted,
    /// Draft has been written to an editable file but not yet applied.
    Editable,
    /// Draft has been explicitly discarded.
    Discarded,
}

/// Stored draft record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Draft {
    /// Stable identifier for the draft (file basename without extension).
    pub id: String,
    /// Target node for which this draft was generated.
    pub node_id: String,
    /// Artefact type this draft targets. Currently always "contract".
    pub artefact_type: String,
    /// Generated draft body.
    pub draft_text: String,
    /// Lifecycle status.
    #[serde(default)]
    pub status: DraftStatus,
    /// Interface hash recorded at acceptance time. Empty until accepted.
    #[serde(default)]
    pub accepted_interface_hash: String,
    /// RFC 3339 UTC timestamp when the draft was created.
    pub created_at: String,
}

/// Draft store error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DraftStoreError {
    /// I/O error while reading or writing draft state.
    Io(String),
    /// JSON parse error for an existing draft file.
    Parse(String),
    /// Draft with the given ID was not found.
    NotFound(String),
}

impl std::fmt::Display for DraftStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "draft store io: {msg}"),
            Self::Parse(msg) => write!(f, "draft store parse: {msg}"),
            Self::NotFound(id) => write!(f, "draft {id} not found"),
        }
    }
}

impl std::error::Error for DraftStoreError {}

/// Filesystem-backed draft store rooted at `.cairn/state/summariser/`.
#[derive(Clone, Debug)]
pub struct DraftStore {
    root: std::path::PathBuf,
}

impl DraftStore {
    /// Constructs a draft store rooted at `root` (typically
    /// `.cairn/state/summariser/`).
    #[must_use]
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the directory storing pending drafts.
    #[must_use]
    pub fn pending_dir(&self) -> std::path::PathBuf {
        self.root.join("pending")
    }

    /// Returns the directory storing editable copies of drafts.
    #[must_use]
    pub fn editable_dir(&self) -> std::path::PathBuf {
        self.root.join("editable")
    }

    /// Writes a draft into the pending directory.
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::Io` when the directory cannot be created
    /// or the file cannot be written.
    pub fn write(&self, draft: &Draft) -> Result<std::path::PathBuf, DraftStoreError> {
        let dir = self.pending_dir();
        fs::create_dir_all(&dir).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        let path = dir.join(format!("{}.json", draft.id));
        let body =
            serde_json::to_string_pretty(draft).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        fs::write(&path, body).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        Ok(path)
    }

    /// Reads a draft by ID.
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::NotFound` when the draft file is absent,
    /// `Io` for read failures, and `Parse` when the file is malformed.
    pub fn read(&self, draft_id: &str) -> Result<Draft, DraftStoreError> {
        let path = self.pending_dir().join(format!("{draft_id}.json"));
        if !path.exists() {
            return Err(DraftStoreError::NotFound(draft_id.to_owned()));
        }
        let body = fs::read_to_string(&path).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        let draft: Draft =
            serde_json::from_str(&body).map_err(|e| DraftStoreError::Parse(e.to_string()))?;
        Ok(draft)
    }

    /// Lists all drafts in the pending directory, sorted by ID.
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::Io` when the directory cannot be read.
    pub fn list(&self) -> Result<Vec<Draft>, DraftStoreError> {
        let dir = self.pending_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut entries: Vec<Draft> = Vec::new();
        for entry in fs::read_dir(&dir).map_err(|e| DraftStoreError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| DraftStoreError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let body = fs::read_to_string(&path).map_err(|e| DraftStoreError::Io(e.to_string()))?;
            let draft: Draft =
                serde_json::from_str(&body).map_err(|e| DraftStoreError::Parse(e.to_string()))?;
            entries.push(draft);
        }
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(entries)
    }

    /// Writes the draft body to the editable directory under `<id>.md`
    /// without changing the pending status.
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::Io` when the directory or file cannot be
    /// written.
    pub fn write_editable(&self, draft: &Draft) -> Result<std::path::PathBuf, DraftStoreError> {
        let dir = self.editable_dir();
        fs::create_dir_all(&dir).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        let path = dir.join(format!("{}.md", draft.id));
        fs::write(&path, &draft.draft_text).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        Ok(path)
    }

    /// Returns the editable file path for a draft.
    #[must_use]
    pub fn editable_path(&self, draft_id: &str) -> std::path::PathBuf {
        self.editable_dir().join(format!("{draft_id}.md"))
    }
}

/// Thin wrapper to read a draft from any path.
///
/// # Errors
///
/// Returns `DraftStoreError::Io` for read failures and `Parse` for
/// malformed JSON.
pub fn read_draft(path: &Path) -> Result<Draft, DraftStoreError> {
    let body = fs::read_to_string(path).map_err(|e| DraftStoreError::Io(e.to_string()))?;
    let draft: Draft =
        serde_json::from_str(&body).map_err(|e| DraftStoreError::Parse(e.to_string()))?;
    Ok(draft)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_draft() -> Draft {
        Draft {
            id: "draft-001".to_owned(),
            node_id: "node-a".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "# Auth\n\nReturns user.".to_owned(),
            status: DraftStatus::Pending,
            accepted_interface_hash: String::new(),
            created_at: "2026-05-07T12:00:00Z".to_owned(),
        }
    }

    #[test]
    fn draft_round_trips_through_serde() {
        let draft = sample_draft();
        let json = serde_json::to_string(&draft).expect("serialise");
        let back: Draft = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, draft);
    }

    #[test]
    fn draft_status_default_is_pending() {
        let status = DraftStatus::default();
        assert!(matches!(status, DraftStatus::Pending));
    }

    #[test]
    fn store_write_and_read_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = sample_draft();
        store.write(&draft).expect("write");
        let back = store.read("draft-001").expect("read");
        assert_eq!(back, draft);
    }

    #[test]
    fn store_read_missing_returns_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let err = store.read("missing").expect_err("should error");
        assert!(matches!(err, DraftStoreError::NotFound(_)));
    }

    #[test]
    fn store_list_returns_sorted_drafts() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        for id in &["b", "a", "c"] {
            let mut draft = sample_draft();
            draft.id = (*id).to_owned();
            store.write(&draft).expect("write");
        }
        let drafts = store.list().expect("list");
        let ids: Vec<&str> = drafts.iter().map(|d| d.id.as_str()).collect();
        assert_eq!(ids, vec!["a", "b", "c"]);
    }

    #[test]
    fn write_editable_creates_md_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = sample_draft();
        let path = store.write_editable(&draft).expect("write editable");
        assert!(path.exists());
        assert_eq!(path.extension().and_then(|s| s.to_str()), Some("md"));
        let body = fs::read_to_string(&path).expect("read");
        assert_eq!(body, draft.draft_text);
    }
}
