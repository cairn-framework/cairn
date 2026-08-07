//! Family-local coordination store: append-only facts and epoch tokens.
//!
//! The store roots at `<git-common-dir>/cairn/coord/`, the only location
//! shared by every worktree of a checkout family, per
//! `dec.rung-three-coordination-substrate` clause 2.

/// Git subprocess helpers: resolving the family's shared git directory.
pub mod git;
