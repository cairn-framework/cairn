//! Authorability eval instrument: one prompt in, one scored record out.
//!
//! Cairn's blueprint grammar and artefact frontmatter are increasingly authored
//! by models. This instrument measures whether a model produces them validly,
//! graded by the production validators rather than by a copy of the finding
//! logic.
//!
//! One run copies `tests/fixtures/cairn-bootstrap` into a scratch workspace,
//! invokes a backend through the model-execution seam, applies the response,
//! scores it with `cairn scan --strict` and `cairn lint --json`, feeds any
//! findings back through a bounded repair loop, and emits exactly one
//! [`Record`](crate::authoreval::Record).
//!
//! The harness owns model execution. Everything else is here.

mod backend;
mod cli;
mod prompt;
mod record;
mod runner;
mod scorer;
mod taxonomy;
mod workspace;

// Reason: every test in this module drives a shell stub, so the whole module,
// helpers included, is unix-only. Gating the tests individually leaves those
// helpers unused on Windows, which `-D warnings` rejects.
#[cfg(all(test, unix))]
mod loop_tests;
#[cfg(test)]
mod tests;

pub use backend::{BackendErrorClass, BackendIdentity};
pub use cli::{Invocation, help_text, sibling_cairn_bin};
pub use record::{Hotspot, Outcome, RECORD_SCHEMA_VERSION, Record, RecordError, TokenTotals};
pub use runner::{BackendSpec, RunConfig, run_prompt_file};
pub use taxonomy::{FailureClass, FailureSubclass};
