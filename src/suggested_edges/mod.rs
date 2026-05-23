//! Suggested-edges queue: per-change `suggested-edges.json`.
//!
//! Mutable triage workflows for AI-suggested graph edges. Traces
//! (immutable records) remain in the `provenance` module; this module
//! owns the queue lifecycle: read, write, validate, and count.

mod types;

use std::{fs, path::Path};

pub use types::{EdgeProvenance, QueueError, SuggestedEdgeEntry, SuggestedEdgesQueue, TriageState};

/// Wire schema version for the suggested-edges queue.
pub const SUGGESTED_EDGES_QUEUE_VERSION: u32 = 1;

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

/// Writes the queue for a change directory atomically. The temp-file
/// path carries pid + nanos + counter suffix so concurrent writers
/// do not race on the same `.json.tmp` filename.
///
/// # Errors
///
/// Returns `QueueError::Io` when the directory cannot be created or
/// either filesystem operation fails.
pub fn write_to_change(change_dir: &Path, queue: &SuggestedEdgesQueue) -> Result<(), QueueError> {
    if !change_dir.exists() {
        fs::create_dir_all(change_dir).map_err(|e| QueueError::Io(e.to_string()))?;
    }
    let final_path = queue_path_for_change(change_dir);
    let temp_path = unique_temp_path(&final_path);
    let body = serde_json::to_string_pretty(queue).map_err(|e| QueueError::Io(e.to_string()))?;
    fs::write(&temp_path, body).map_err(|e| QueueError::Io(e.to_string()))?;
    fs::rename(&temp_path, &final_path).map_err(|e| QueueError::Io(e.to_string()))?;
    Ok(())
}

fn unique_temp_path(final_path: &Path) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    // pid alone is insufficient for cross-thread uniqueness, and
    // SystemTime::now() may report identical nanos on consecutive
    // sub-microsecond calls. The atomic counter guarantees uniqueness
    // across threads in the same process; pid + nanos guards against
    // cross-process collisions.
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let pid = std::process::id();
    let stem = final_path
        .file_name()
        .map(std::ffi::OsStr::to_string_lossy)
        .unwrap_or_default();
    let parent = final_path.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!("{stem}.{pid}.{nanos}.{seq}.tmp"))
}

/// Returns `Ok(())` when the queue (if present) has zero pending
/// entries; returns `Err(CairnError::UntriagedSuggestedEdges)` (CC002)
/// when entries are pending; returns
/// `Err(CairnError::ChangeDiscovery)` (CC003) when the queue file
/// exists but cannot be read, parsed, or carries a version newer than
/// supported.
///
/// # Errors
///
/// Returns `UntriagedSuggestedEdges` (CC002) for pending entries and
/// `ChangeDiscovery` (CC003) for queue I/O, parse, or version errors.
pub fn validate_strict(change_id: &str, change_dir: &Path) -> Result<(), crate::error::CairnError> {
    let queue = match read_from_change(change_dir) {
        Ok(None) => return Ok(()),
        Ok(Some(q)) => q,
        Err(e) => {
            return Err(crate::error::CairnError::ChangeDiscovery {
                path: queue_path_for_change(change_dir)
                    .to_string_lossy()
                    .into_owned(),
                detail: e.to_string(),
            });
        }
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
            provenance: Some(EdgeProvenance {
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

    #[test]
    fn edge_provenance_round_trips() {
        let prov = EdgeProvenance {
            trace_phase: "phase-10".to_owned(),
            stage: "apply".to_owned(),
        };
        let json = serde_json::to_string(&prov).expect("serialise");
        let back: EdgeProvenance = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, prov);
    }

    // ── write_to_change ───────────────────────────────────────────────────────

    #[test]
    fn write_to_change_creates_directory_and_file() {
        let dir = tempfile::tempdir().expect("temp dir");
        let change_dir = dir.path().join("change-xyz");
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![sample_entry(TriageState::Pending)],
        };
        write_to_change(&change_dir, &queue).expect("write must succeed");
        let path = queue_path_for_change(&change_dir);
        assert!(path.exists(), "queue file must exist after write");
    }

    #[test]
    fn write_to_change_round_trips_content() {
        let dir = tempfile::tempdir().expect("temp dir");
        let change_dir = dir.path().join("change-abc");
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![
                sample_entry(TriageState::Pending),
                sample_entry(TriageState::Accepted),
            ],
        };
        write_to_change(&change_dir, &queue).expect("write");
        let back = read_from_change(&change_dir)
            .expect("read")
            .expect("present");
        assert_eq!(back, queue);
    }

    #[test]
    fn write_to_change_is_idempotent_on_overwrite() {
        let dir = tempfile::tempdir().expect("temp dir");
        let change_dir = dir.path().join("change-ow");
        let queue_v1 = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![sample_entry(TriageState::Pending)],
        };
        let queue_v2 = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![sample_entry(TriageState::Accepted)],
        };
        write_to_change(&change_dir, &queue_v1).expect("first write");
        write_to_change(&change_dir, &queue_v2).expect("second write");
        let back = read_from_change(&change_dir)
            .expect("read")
            .expect("present");
        assert_eq!(back, queue_v2, "second write must overwrite the first");
    }

    // ── validate_strict ───────────────────────────────────────────────────────

    #[test]
    fn validate_strict_absent_queue_returns_ok() {
        let dir = tempfile::tempdir().expect("temp dir");
        let result = validate_strict("change-1", dir.path());
        assert!(result.is_ok(), "absent queue must not block archive");
    }

    #[test]
    fn validate_strict_zero_pending_returns_ok() {
        let dir = tempfile::tempdir().expect("temp dir");
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![
                sample_entry(TriageState::Accepted),
                sample_entry(TriageState::Rejected),
            ],
        };
        write_to_change(dir.path(), &queue).expect("write");
        let result = validate_strict("change-1", dir.path());
        assert!(
            result.is_ok(),
            "all-triaged queue must not block archive: {result:?}"
        );
    }

    #[test]
    fn validate_strict_pending_entries_returns_cc002() {
        let dir = tempfile::tempdir().expect("temp dir");
        let queue = SuggestedEdgesQueue {
            version: SUGGESTED_EDGES_QUEUE_VERSION,
            entries: vec![
                sample_entry(TriageState::Accepted),
                sample_entry(TriageState::Pending),
                sample_entry(TriageState::Pending),
            ],
        };
        write_to_change(dir.path(), &queue).expect("write");
        let err = validate_strict("my-change", dir.path()).unwrap_err();
        // Must be the CC002 error variant with the correct change id and count.
        match &err {
            crate::error::CairnError::UntriagedSuggestedEdges {
                change_id,
                pending_count,
                ..
            } => {
                assert_eq!(change_id, "my-change");
                assert_eq!(*pending_count, 2);
            }
            other => panic!("expected UntriagedSuggestedEdges, got {other:?}"),
        }
    }

    #[test]
    fn validate_strict_malformed_file_returns_cc003() {
        let dir = tempfile::tempdir().expect("temp dir");
        // Write invalid JSON to the queue path so the reader returns a parse error.
        let path = queue_path_for_change(dir.path());
        fs::write(&path, "not valid json").expect("write");
        let err = validate_strict("change-err", dir.path()).unwrap_err();
        assert!(
            matches!(err, crate::error::CairnError::ChangeDiscovery { .. }),
            "parse failure must produce CC003 ChangeDiscovery, got {err:?}"
        );
    }
}
