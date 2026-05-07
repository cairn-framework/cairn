//! Suggested-edges queue: per-change `suggested-edges.json`.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

/// Wire schema version for the suggested-edges queue.
pub const SUGGESTED_EDGES_QUEUE_VERSION: u32 = 1;

/// Triage state for a single suggested edge entry.
///
/// Newly-emitted entries default to `Pending`. A human reviewer
/// transitions the entry to `Accepted`, `Rejected`, or `Deferred`.
/// Pending entries block `cflx openspec validate <change> --strict` with
/// error code `CC002`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TriageState {
    /// Awaiting human triage. Default for new entries.
    #[default]
    Pending,
    /// Human approved this suggestion.
    Accepted,
    /// Human rejected this suggestion.
    Rejected,
    /// Human deferred this suggestion to a future change.
    Deferred,
}

/// Provenance pointer back into a trace sidecar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntryProvenance {
    /// Phase identifier that produced the entry.
    pub trace_phase: String,
    /// Stage within the run that produced the entry.
    pub stage: String,
}

/// One suggested edge between two declared nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedEdgeEntry {
    /// Source node ID.
    pub source: String,
    /// Target node ID.
    pub target: String,
    /// Verb describing the suggested relation.
    pub relation: String,
    /// Triage state. Defaults to `Pending` for new entries.
    #[serde(default)]
    pub triage_state: TriageState,
    /// Producer-computed confidence in [0.0, 1.0]. Optional.
    #[serde(default)]
    pub confidence: Option<f64>,
    /// Optional pointer back into the trace sidecar that produced the entry.
    #[serde(default)]
    pub provenance: Option<EntryProvenance>,
    /// Optional human note recorded during triage.
    #[serde(default)]
    pub triage_note: Option<String>,
}

/// Top-level queue payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SuggestedEdgesQueue {
    /// Schema version. Reader rejects higher values.
    pub version: u32,
    /// Queue entries.
    #[serde(default)]
    pub entries: Vec<SuggestedEdgeEntry>,
}

/// Queue reader error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueueError {
    /// File could not be read from disk.
    Io(String),
    /// File could not be parsed as JSON.
    Parse(String),
    /// Queue carries a higher schema version than the reader supports.
    UnsupportedVersion {
        /// Version found on disk.
        found: u32,
        /// Maximum version this reader supports.
        expected: u32,
    },
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "queue io: {msg}"),
            Self::Parse(msg) => write!(f, "queue parse: {msg}"),
            Self::UnsupportedVersion { found, expected } => write!(
                f,
                "queue version {found} is newer than reader version {expected}"
            ),
        }
    }
}

impl std::error::Error for QueueError {}

/// Reads the queue file at `path`. Returns `Ok(None)` when the file does
/// not exist; an absent queue is not an error per the phase-7.6 spec.
///
/// # Errors
///
/// Returns `QueueError` when the file is present but unreadable, malformed,
/// or carries a higher version than supported.
pub fn read_queue(path: &Path) -> Result<Option<SuggestedEdgesQueue>, QueueError> {
    if !path.exists() {
        return Ok(None);
    }
    let body = fs::read_to_string(path).map_err(|e| QueueError::Io(e.to_string()))?;
    let queue: SuggestedEdgesQueue =
        serde_json::from_str(&body).map_err(|e| QueueError::Parse(e.to_string()))?;
    if queue.version > SUGGESTED_EDGES_QUEUE_VERSION {
        return Err(QueueError::UnsupportedVersion {
            found: queue.version,
            expected: SUGGESTED_EDGES_QUEUE_VERSION,
        });
    }
    Ok(Some(queue))
}

/// Counts entries whose triage state is `Pending`. The `cflx openspec
/// validate --strict` gate uses this count to block archive with CC002
/// when non-zero.
#[must_use]
pub fn count_pending(queue: &SuggestedEdgesQueue) -> usize {
    queue
        .entries
        .iter()
        .filter(|e| e.triage_state == TriageState::Pending)
        .count()
}

/// Resolves the queue file for a change directory.
#[must_use]
pub fn queue_path_for_change(change_dir: &Path) -> std::path::PathBuf {
    change_dir.join("suggested-edges.json")
}

/// Reads the queue for a change directory. Returns `Ok(None)` when no
/// file exists.
///
/// # Errors
///
/// Returns `QueueError` for I/O, parse, or version errors.
pub fn read_from_change(change_dir: &Path) -> Result<Option<SuggestedEdgesQueue>, QueueError> {
    read_queue(&queue_path_for_change(change_dir))
}

/// Writes the queue for a change directory atomically (temp-file rename).
///
/// # Errors
///
/// Returns `QueueError::Io` when the directory cannot be created or the
/// temp-file rename fails.
pub fn write_to_change(change_dir: &Path, queue: &SuggestedEdgesQueue) -> Result<(), QueueError> {
    if !change_dir.exists() {
        fs::create_dir_all(change_dir).map_err(|e| QueueError::Io(e.to_string()))?;
    }
    let final_path = queue_path_for_change(change_dir);
    let temp_path = final_path.with_extension("json.tmp");
    let body = serde_json::to_string_pretty(queue).map_err(|e| QueueError::Io(e.to_string()))?;
    fs::write(&temp_path, body).map_err(|e| QueueError::Io(e.to_string()))?;
    fs::rename(&temp_path, &final_path).map_err(|e| QueueError::Io(e.to_string()))?;
    Ok(())
}

/// Returns `Ok(())` when the queue (if present) has zero pending
/// entries; returns `Err(CairnError::UntriagedSuggestedEdges)` (CC002)
/// otherwise. Used by `cflx openspec validate --strict`.
///
/// # Errors
///
/// Returns `CairnError::UntriagedSuggestedEdges` (code CC002) when one
/// or more entries are still in `Pending` triage state.
pub fn validate_strict(change_id: &str, change_dir: &Path) -> Result<(), crate::error::CairnError> {
    let Ok(Some(queue)) = read_from_change(change_dir) else {
        return Ok(());
    };
    let pending = count_pending(&queue);
    if pending == 0 {
        return Ok(());
    }
    Err(crate::error::CairnError::UntriagedSuggestedEdges {
        change_id: change_id.to_owned(),
        pending_count: pending,
        file_path: queue_path_for_change(change_dir)
            .to_string_lossy()
            .into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry(state: TriageState) -> SuggestedEdgeEntry {
        SuggestedEdgeEntry {
            source: "node-a".to_owned(),
            target: "node-b".to_owned(),
            relation: "calls".to_owned(),
            triage_state: state,
            confidence: Some(0.8),
            provenance: Some(EntryProvenance {
                trace_phase: "phase-9".to_owned(),
                stage: "propose".to_owned(),
            }),
            triage_note: None,
        }
    }

    #[test]
    fn queue_round_trips_through_serde() {
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![sample_entry(TriageState::Pending)],
        };
        let json = serde_json::to_string(&queue).expect("serialise");
        let back: SuggestedEdgesQueue = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, queue);
    }

    #[test]
    fn triage_state_defaults_to_pending() {
        let entry: SuggestedEdgeEntry =
            serde_json::from_str(r#"{"source":"a","target":"b","relation":"calls"}"#)
                .expect("parse minimal entry");
        assert_eq!(entry.triage_state, TriageState::Pending);
    }

    #[test]
    fn count_pending_counts_only_pending() {
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![
                sample_entry(TriageState::Pending),
                sample_entry(TriageState::Accepted),
                sample_entry(TriageState::Pending),
                sample_entry(TriageState::Rejected),
                sample_entry(TriageState::Deferred),
            ],
        };
        assert_eq!(count_pending(&queue), 2);
    }

    #[test]
    fn read_queue_returns_none_when_absent() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("missing.json");
        let result = read_queue(&path).expect("absent must not error");
        assert!(result.is_none());
    }

    #[test]
    fn read_queue_round_trips_against_disk() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("queue.json");
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![sample_entry(TriageState::Pending)],
        };
        let body = serde_json::to_string(&queue).expect("serialise");
        fs::write(&path, body).expect("write");
        let back = read_queue(&path).expect("read").expect("present");
        assert_eq!(back, queue);
    }

    #[test]
    fn read_queue_rejects_higher_version() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("queue.json");
        let body = serde_json::json!({
            "version": SUGGESTED_EDGES_QUEUE_VERSION + 1,
            "entries": [],
        })
        .to_string();
        fs::write(&path, body).expect("write");
        let err = read_queue(&path).expect_err("should reject");
        assert!(matches!(err, QueueError::UnsupportedVersion { .. }));
    }
}
