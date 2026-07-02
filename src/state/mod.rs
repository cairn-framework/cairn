//! Read-only view over the beads issue backlog.
//!
//! `dec.change-format-only`: the pluggable `StateBackend` abstraction
//! (filesystem/beads record storage) is deleted; creating, claiming, and
//! sequencing work items is workflow, and cairn does not do workflow
//! (`dec.no-orchestrator`). The read-only backlog view below is a distinct,
//! ratified surface (`dec.beads-task-layer`) and is unaffected.

pub(crate) mod backlog;
