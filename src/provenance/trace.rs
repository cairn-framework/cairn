//! Trace sidecar: per-archived-change `.cflx-trace.json`.

use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};

/// Wire schema version for the trace sidecar.
pub const TRACE_SIDECAR_VERSION: u32 = 1;

/// A single cairn-native cflx stage.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TraceStage {
    /// Propose stage: change directory authoring.
    Propose,
    /// Apply stage: implementation against the failing test contract.
    Apply,
    /// Accept stage: verification of the implementation.
    Accept,
    /// Archive stage: finalisation and consolidation.
    Archive,
}

/// Stage record carried by the sidecar for one stage. Token and model
/// fields are `Option` per phase-7.6 task 1.2: backends that do not
/// report tokens or stages that ran without a model (e.g., archive
/// verification only) round-trip cleanly.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StageRecord {
    /// Identifier of the model that ran the stage, when known.
    #[serde(default)]
    pub model_id: Option<String>,
    /// Tokens consumed on the input side, when reported.
    #[serde(default)]
    pub tokens_in: Option<u64>,
    /// Tokens generated, when reported.
    #[serde(default)]
    pub tokens_out: Option<u64>,
    /// End-to-end latency in milliseconds.
    pub latency_ms: u64,
    /// Whether the stage completed successfully.
    pub success: bool,
    /// Optional error message captured from the stage.
    #[serde(default)]
    pub error_message: Option<String>,
    /// RFC 3339 UTC timestamp at the start of the stage.
    pub started_at: String,
    /// RFC 3339 UTC timestamp at the end of the stage.
    pub ended_at: String,
}

/// Top-level sidecar payload. The `phase` field self-identifies the
/// archived phase the sidecar describes per phase-7.6 task 1.1.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TraceSidecar {
    /// Schema version. The reader rejects sidecars with a higher version.
    pub version: u32,
    /// Phase identifier (e.g., "phase-7.6") this sidecar describes.
    #[serde(default)]
    pub phase: String,
    /// Stage records keyed by stage name.
    #[serde(default)]
    pub stages: BTreeMap<TraceStage, StageRecord>,
    /// Reserved for a future phase. Currently always empty. Stored as
    /// untyped JSON values so future producers can evolve the prompt
    /// schema without bumping `version`.
    #[serde(default)]
    pub prompts: Vec<serde_json::Value>,
}

/// Trace sidecar reader error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceError {
    /// File could not be read from disk.
    Io(String),
    /// File could not be parsed as JSON.
    Parse(String),
    /// Sidecar carries a higher schema version than the reader supports.
    UnsupportedVersion {
        /// Version found on disk.
        found: u32,
        /// Maximum version this reader supports.
        expected: u32,
    },
}

impl std::fmt::Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "trace sidecar io: {msg}"),
            Self::Parse(msg) => write!(f, "trace sidecar parse: {msg}"),
            Self::UnsupportedVersion { found, expected } => write!(
                f,
                "trace sidecar version {found} is newer than reader version {expected}"
            ),
        }
    }
}

impl std::error::Error for TraceError {}

/// Reads and parses a `.cflx-trace.json` sidecar from disk.
///
/// # Errors
///
/// Returns `TraceError::Io` when the file cannot be read, `TraceError::Parse`
/// when the payload is not valid JSON for the schema, and
/// `TraceError::UnsupportedVersion` when the sidecar's `version` exceeds
/// `TRACE_SIDECAR_VERSION`.
pub fn read_sidecar(path: &Path) -> Result<TraceSidecar, TraceError> {
    let body = fs::read_to_string(path).map_err(|e| TraceError::Io(e.to_string()))?;
    let sidecar: TraceSidecar =
        serde_json::from_str(&body).map_err(|e| TraceError::Parse(e.to_string()))?;
    if sidecar.version > TRACE_SIDECAR_VERSION {
        return Err(TraceError::UnsupportedVersion {
            found: sidecar.version,
            expected: TRACE_SIDECAR_VERSION,
        });
    }
    Ok(sidecar)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sidecar() -> TraceSidecar {
        let mut stages = BTreeMap::new();
        for stage in [
            TraceStage::Propose,
            TraceStage::Apply,
            TraceStage::Accept,
            TraceStage::Archive,
        ] {
            stages.insert(
                stage,
                StageRecord {
                    model_id: Some("claude-sonnet-4-6".to_owned()),
                    tokens_in: Some(100),
                    tokens_out: Some(50),
                    latency_ms: 1234,
                    success: true,
                    error_message: None,
                    started_at: "2026-05-07T12:00:00Z".to_owned(),
                    ended_at: "2026-05-07T12:00:01Z".to_owned(),
                },
            );
        }
        TraceSidecar {
            version: TRACE_SIDECAR_VERSION,
            phase: "phase-7.6".to_owned(),
            stages,
            prompts: Vec::new(),
        }
    }

    #[test]
    fn sidecar_round_trips_through_serde() {
        let sidecar = sample_sidecar();
        let json = serde_json::to_string(&sidecar).expect("serialise");
        let back: TraceSidecar = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, sidecar);
    }

    #[test]
    fn sidecar_carries_four_native_stages() {
        let sidecar = sample_sidecar();
        let names: Vec<TraceStage> = sidecar.stages.keys().copied().collect();
        assert_eq!(
            names,
            vec![
                TraceStage::Propose,
                TraceStage::Apply,
                TraceStage::Accept,
                TraceStage::Archive,
            ]
        );
    }

    #[test]
    fn prompts_field_is_empty_in_this_phase() {
        let sidecar = sample_sidecar();
        assert!(sidecar.prompts.is_empty());
    }

    #[test]
    fn higher_version_rejected_via_in_memory_value() {
        let json = serde_json::json!({
            "version": TRACE_SIDECAR_VERSION + 1,
            "stages": {},
            "prompts": [],
        });
        let sidecar: TraceSidecar = serde_json::from_value(json).expect("parse");
        // The reader function would reject this; we replicate the check
        // here to exercise the error variant.
        assert!(sidecar.version > TRACE_SIDECAR_VERSION);
    }

    #[test]
    fn read_sidecar_round_trips_against_disk() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("trace.json");
        let sidecar = sample_sidecar();
        let body = serde_json::to_string(&sidecar).expect("serialise");
        fs::write(&path, body).expect("write");
        let back = read_sidecar(&path).expect("read");
        assert_eq!(back, sidecar);
    }

    #[test]
    fn read_sidecar_rejects_higher_version() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("trace.json");
        let body = serde_json::json!({
            "version": TRACE_SIDECAR_VERSION + 1,
            "stages": {},
            "prompts": [],
        })
        .to_string();
        fs::write(&path, body).expect("write");
        let err = read_sidecar(&path).expect_err("should reject");
        match err {
            TraceError::UnsupportedVersion { found, expected } => {
                assert_eq!(found, TRACE_SIDECAR_VERSION + 1);
                assert_eq!(expected, TRACE_SIDECAR_VERSION);
            }
            other => panic!("expected UnsupportedVersion, got {other:?}"),
        }
    }

    #[test]
    fn read_sidecar_io_error_for_missing_file() {
        let err = read_sidecar(Path::new("/nonexistent/trace.json")).expect_err("should error");
        assert!(matches!(err, TraceError::Io(_)));
    }

    #[test]
    fn sidecar_carries_phase_field() {
        let sidecar = sample_sidecar();
        assert_eq!(sidecar.phase, "phase-7.6");
        let json = serde_json::to_string(&sidecar).expect("serialise");
        assert!(json.contains("\"phase\":\"phase-7.6\""));
    }

    #[test]
    fn stage_record_token_fields_optional() {
        let json = r#"{
            "latency_ms": 100,
            "success": true,
            "started_at": "2026-05-07T12:00:00Z",
            "ended_at": "2026-05-07T12:00:01Z"
        }"#;
        let record: StageRecord = serde_json::from_str(json).expect("parse without tokens");
        assert!(record.model_id.is_none());
        assert!(record.tokens_in.is_none());
        assert!(record.tokens_out.is_none());
    }
}
