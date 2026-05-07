//! Draft store: pending/editable/accepted/discarded drafts under
//! `.cairn/state/summariser/`.
//!
//! Phase-8 reforge cycle 1: Draft is now an enum-of-payloads tagged by
//! `status`, so each lifecycle state carries only its valid fields. The
//! Pending variant has no `interface_hash` (recording that hash is a
//! postcondition of acceptance); Accepted variants carry a non-empty
//! hash by construction; Discarded is a terminal record. `DraftStore::write`
//! refuses to clobber an existing draft and returns
//! `DraftStoreError::Conflict`; the caller must invoke `overwrite` for
//! the rare legitimate replacement path.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

/// Wire schema version for serialised draft state.
pub const DRAFT_SCHEMA_VERSION: u32 = 1;

/// Lifecycle-tagged draft record. Each variant carries only the fields
/// valid for that lifecycle state, so illegal combinations are
/// unrepresentable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Draft {
    /// Awaiting human resolution. No interface hash recorded yet.
    Pending(PendingDraft),
    /// Editable copy was written but the original contract has not been
    /// replaced.
    Editable(EditableDraft),
    /// Draft was applied to the target contract; the interface hash at
    /// acceptance time is recorded.
    Accepted(AcceptedDraft),
    /// Terminal: the user discarded this draft.
    Discarded(DiscardedDraft),
}

/// Common fields all draft variants share.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DraftHeader {
    /// Stable identifier for the draft (file basename without extension).
    pub id: String,
    /// Target node for which this draft was generated.
    pub node_id: String,
    /// Artefact type this draft targets. Currently always "contract".
    pub artefact_type: String,
    /// Generated draft body.
    pub draft_text: String,
    /// RFC 3339 UTC timestamp when the draft was created.
    pub created_at: String,
}

/// A draft awaiting resolution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PendingDraft {
    /// Common header fields.
    #[serde(flatten)]
    pub header: DraftHeader,
}

/// A draft whose body was written to the editable directory.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EditableDraft {
    /// Common header fields.
    #[serde(flatten)]
    pub header: DraftHeader,
    /// Path to the editable file.
    pub editable_path: String,
}

/// A draft applied to a contract; carries the interface hash at the
/// moment of acceptance.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AcceptedDraft {
    /// Common header fields.
    #[serde(flatten)]
    pub header: DraftHeader,
    /// Interface hash recorded at acceptance time. Non-empty by
    /// construction.
    pub accepted_interface_hash: String,
}

/// A draft the user discarded.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscardedDraft {
    /// Common header fields.
    #[serde(flatten)]
    pub header: DraftHeader,
    /// Optional reason recorded at discard time.
    #[serde(default)]
    pub reason: Option<String>,
}

impl Draft {
    /// Returns the draft id regardless of variant.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Pending(d) => &d.header.id,
            Self::Editable(d) => &d.header.id,
            Self::Accepted(d) => &d.header.id,
            Self::Discarded(d) => &d.header.id,
        }
    }
}

/// Draft store error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DraftStoreError {
    /// I/O error while reading or writing draft state.
    Io(String),
    /// JSON parse error for an existing draft file.
    Parse(String),
    /// JSON serialisation error for a draft about to be written.
    Serialize(String),
    /// Draft with the given ID was not found.
    NotFound(String),
    /// A draft with the same ID already exists; the caller must use
    /// `overwrite` to replace it.
    Conflict(String),
}

impl std::fmt::Display for DraftStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "draft store io: {msg}"),
            Self::Parse(msg) => write!(f, "draft store parse: {msg}"),
            Self::Serialize(msg) => write!(f, "draft store serialise: {msg}"),
            Self::NotFound(id) => write!(f, "draft {id} not found"),
            Self::Conflict(id) => write!(
                f,
                "draft {id} already exists; use `cairn draft accept|edit|discard` first or call overwrite() explicitly"
            ),
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

    /// Writes a draft into the pending directory. Returns
    /// `DraftStoreError::Conflict` when a draft with the same ID is
    /// already present; the caller must use `overwrite` to replace it.
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::Io` when the directory cannot be created
    /// or the file cannot be written, `Serialize` when the draft cannot
    /// be encoded as JSON, and `Conflict` when a draft with the same id
    /// already exists.
    pub fn write(&self, draft: &Draft) -> Result<std::path::PathBuf, DraftStoreError> {
        let dir = self.pending_dir();
        fs::create_dir_all(&dir).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        let path = dir.join(format!("{}.json", draft.id()));
        if path.exists() {
            return Err(DraftStoreError::Conflict(draft.id().to_owned()));
        }
        Self::write_inner(&path, draft)
    }

    /// Writes a draft, replacing any existing entry with the same ID.
    /// Use this only for legitimate replacement (e.g., user explicitly
    /// regenerated).
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::Io` when the file cannot be written and
    /// `Serialize` when the draft cannot be encoded as JSON.
    pub fn overwrite(&self, draft: &Draft) -> Result<std::path::PathBuf, DraftStoreError> {
        let dir = self.pending_dir();
        fs::create_dir_all(&dir).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        let path = dir.join(format!("{}.json", draft.id()));
        Self::write_inner(&path, draft)
    }

    fn write_inner(path: &Path, draft: &Draft) -> Result<std::path::PathBuf, DraftStoreError> {
        let body = serde_json::to_string_pretty(draft)
            .map_err(|e| DraftStoreError::Serialize(e.to_string()))?;
        fs::write(path, body).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        Ok(path.to_path_buf())
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
        entries.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(entries)
    }

    /// Writes the draft body to the editable directory using the
    /// extension matched to the artefact type.
    ///
    /// # Errors
    ///
    /// Returns `DraftStoreError::Io` when the directory or file cannot be
    /// written.
    pub fn write_editable(&self, draft: &Draft) -> Result<std::path::PathBuf, DraftStoreError> {
        let dir = self.editable_dir();
        fs::create_dir_all(&dir).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        let header = match draft {
            Draft::Pending(d) => &d.header,
            Draft::Editable(d) => &d.header,
            Draft::Accepted(d) => &d.header,
            Draft::Discarded(d) => &d.header,
        };
        let extension = editable_extension(&header.artefact_type);
        let path = dir.join(format!("{}.{extension}", header.id));
        fs::write(&path, &header.draft_text).map_err(|e| DraftStoreError::Io(e.to_string()))?;
        Ok(path)
    }

    /// Returns the editable file path for a draft.
    #[must_use]
    pub fn editable_path(&self, draft_id: &str, artefact_type: &str) -> std::path::PathBuf {
        self.editable_dir()
            .join(format!("{draft_id}.{}", editable_extension(artefact_type)))
    }
}

fn editable_extension(artefact_type: &str) -> &'static str {
    match artefact_type {
        "contract" | "decision" | "research" | "review" | "todo" | "source" => "md",
        _ => "txt",
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

    fn sample_header() -> DraftHeader {
        DraftHeader {
            id: "draft-001".to_owned(),
            node_id: "node-a".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "# Auth\n\nReturns user.".to_owned(),
            created_at: "2026-05-07T12:00:00Z".to_owned(),
        }
    }

    fn pending() -> Draft {
        Draft::Pending(PendingDraft {
            header: sample_header(),
        })
    }

    #[test]
    fn pending_round_trips_through_serde() {
        let draft = pending();
        let json = serde_json::to_string(&draft).expect("serialise");
        let back: Draft = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, draft);
    }

    #[test]
    fn accepted_carries_non_empty_hash() {
        let draft = Draft::Accepted(AcceptedDraft {
            header: sample_header(),
            accepted_interface_hash: "sha256:abc".to_owned(),
        });
        let json = serde_json::to_string(&draft).expect("serialise");
        assert!(json.contains("\"accepted_interface_hash\":\"sha256:abc\""));
    }

    #[test]
    fn pending_serialised_form_omits_hash_field() {
        let draft = pending();
        let json = serde_json::to_string(&draft).expect("serialise");
        assert!(
            !json.contains("accepted_interface_hash"),
            "Pending must not carry accepted_interface_hash; got: {json}"
        );
    }

    #[test]
    fn store_write_then_read_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = pending();
        store.write(&draft).expect("write");
        let back = store.read("draft-001").expect("read");
        assert_eq!(back, draft);
    }

    #[test]
    fn store_write_refuses_conflict() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = pending();
        store.write(&draft).expect("first write");
        let err = store.write(&draft).expect_err("second write must conflict");
        assert!(matches!(err, DraftStoreError::Conflict(_)));
    }

    #[test]
    fn store_overwrite_replaces_existing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        store.write(&pending()).expect("first write");
        store.overwrite(&pending()).expect("overwrite");
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
            let draft = Draft::Pending(PendingDraft {
                header: DraftHeader {
                    id: (*id).to_owned(),
                    ..sample_header()
                },
            });
            store.write(&draft).expect("write");
        }
        let drafts = store.list().expect("list");
        let ids: Vec<&str> = drafts.iter().map(Draft::id).collect();
        assert_eq!(ids, vec!["a", "b", "c"]);
    }

    #[test]
    fn write_editable_uses_artefact_extension() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = pending();
        let path = store.write_editable(&draft).expect("write editable");
        assert!(path.exists());
        assert_eq!(path.extension().and_then(|s| s.to_str()), Some("md"));
    }

    #[test]
    fn editable_extension_falls_back_to_txt_for_unknown() {
        assert_eq!(editable_extension("contract"), "md");
        assert_eq!(editable_extension("custom-type"), "txt");
    }
}
