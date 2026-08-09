//! Family-local coordination store: append-only facts and epoch tokens.
//!
//! The store roots at `<git-common-dir>/cairn/coord/`, the only location
//! shared by every worktree of a checkout family, per
//! `dec.rung-three-coordination-substrate` clause 2. One file per fact means
//! concurrent writers never contend: no lock, no log, no git object writes.

/// The single write path and the console barrier.
pub mod append;
/// The fact envelope and evidence classes.
pub mod envelope;
/// Epoch-succession exclusion tokens.
pub mod epoch;
/// Git subprocess helpers: resolving the family's shared git directory.
pub mod git;
/// The full-listing fold over immutable coordination facts.
pub mod read;
/// Store root resolution and lazy initialisation.
pub mod store;
/// UTC timestamp formatting without a time crate.
pub(crate) mod time;
/// Integrity verification and archival compaction.
pub mod verify;
