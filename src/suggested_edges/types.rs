//! Types for the suggested-edges queue: triage states, entries, and
//! provenance pointers.

use serde::{Deserialize, Serialize};

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

/// Provenance linking a suggested edge to a trace sidecar entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeProvenance {
    /// Phase that produced the suggestion.
    pub trace_phase: String,
    /// Stage within the phase.
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
    /// Optional provenance pointer back into the trace sidecar that
    /// produced the entry.
    #[serde(default)]
    pub provenance: Option<EdgeProvenance>,
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
