//! Phase 7.6 AI Provenance Foundation: typed schemas and readers for the
//! per-archived-change trace sidecar and the suggested-edges queue.
//!
//! The cairn library defines the schema and provides readers; cflx (or
//! another producer) is the writer of these files. These types are stable
//! and version-gated per `openspec/conventions.md` Section 3.

mod queue;
mod trace;

pub use queue::{
    QueueError, SUGGESTED_EDGES_QUEUE_VERSION, SuggestedEdgeEntry, SuggestedEdgesQueue,
    TriageState, count_pending, queue_path_for_change, read_from_change, read_queue,
    validate_strict, write_to_change,
};
pub use trace::{
    StageRecord, TRACE_SIDECAR_VERSION, TraceError, TraceSidecar, TraceStage, read_sidecar,
};
