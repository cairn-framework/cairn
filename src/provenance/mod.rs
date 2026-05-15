//! Phase 7.6 AI Provenance Foundation: typed schemas and readers for the
//! per-archived-change trace sidecar.
//!
//! The cairn library defines the schema and provides readers; cflx (or
//! another producer) is the writer of these files. These types are stable
//! and version-gated per `openspec/conventions.md` Section 3.
//!
//! Suggested-edges queue types live in the `suggested_edges` module.

mod trace;

pub use trace::{
    StageRecord, TRACE_SIDECAR_VERSION, TraceError, TraceSidecar, TraceStage, read_sidecar,
};
