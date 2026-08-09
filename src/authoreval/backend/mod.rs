//! The model-execution seam.
//!
//! The harness owns model execution; cairn owns the prompt, the fixture, the
//! production validation, and the scoring. Everything crossing that boundary
//! goes through [`AuthorevalBackend`], so a deterministic offline backend and
//! a real harness-driven one are interchangeable.
//!
//! The subprocess plumbing in [`CommandBackend`] deliberately mirrors
//! `summariser::backend::LocalCommandBackend` rather than sharing it: sharing
//! would couple a development instrument to a stable product module.

use std::cell::Cell;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::scorer::Finding;

mod command;

pub(crate) use command::CommandBackend;

/// Wire schema version for the request envelope.
pub(crate) const SCHEMA_VERSION: u32 = 1;

/// Window allowed for draining pipes once the child has exited.
///
/// The deadline bounds how long the backend gets to run. It does not bound
/// this: the child is already gone, and refusing to spend a moment collecting
/// bytes it has already written would throw away a complete answer whenever a
/// backend finishes near its budget. A descendant holding a pipe open is the
/// case this bounds, and two seconds is enough to read what is buffered.
const DRAIN_GRACE: Duration = Duration::from_secs(2);

/// Which backend, and which model, produced a response.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BackendIdentity {
    /// Backend kind, `replay` or `command`.
    pub kind: String,
    /// Model identity the backend speaks for.
    pub model: String,
}

/// Classified backend failure. Total over every backend error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendErrorClass {
    /// The per-call deadline elapsed and the backend was abandoned.
    Timeout,
    /// The backend could not be run, or ran and failed.
    Invocation,
    /// The backend answered, but outside the response shape.
    Protocol,
}

/// One authoring request sent to a backend.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct AuthorRequest<'a> {
    /// Wire schema version.
    pub(crate) schema_version: u32,
    /// Prompt this request belongs to.
    pub(crate) prompt_id: &'a str,
    /// 1 on the first invocation, incremented once per repair.
    pub(crate) attempt: u32,
    /// The authoring instruction.
    pub(crate) instruction: &'a str,
    /// The previous failed scan's findings, verbatim. Empty on attempt 1.
    pub(crate) findings: &'a [Finding],
}

/// One file the model wrote, as complete post-edit content.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FileEdit {
    /// Path relative to the scratch workspace root.
    pub(crate) path: String,
    /// Full contents to write at that path.
    pub(crate) contents: String,
}

/// Tokens a backend reports for one response.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct TokenUsage {
    /// Prompt tokens.
    #[serde(default)]
    pub(crate) prompt: u64,
    /// Completion tokens.
    #[serde(default)]
    pub(crate) completion: u64,
}

/// One authoring response received from a backend.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AuthorResponse {
    /// Files to write, relative to the scratch workspace root.
    #[serde(default)]
    pub(crate) files: Vec<FileEdit>,
    /// Tokens spent producing this response.
    #[serde(default)]
    pub(crate) tokens: TokenUsage,
}

/// Backend invocation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum BackendError {
    /// The per-call deadline elapsed.
    Timeout {
        /// The deadline that elapsed, in milliseconds.
        timeout_ms: u64,
    },
    /// The backend command exited non-zero.
    NonZeroExit {
        /// Exit code reported by the child.
        code: i32,
        /// Captured stderr.
        stderr: String,
    },
    /// The backend could not be spawned or spoken to.
    Io(String),
    /// A replay script ran out of turns.
    ScriptExhausted {
        /// Attempt number that found the script empty.
        attempt: u32,
    },
    /// The response could not be parsed.
    Parse(String),
}

impl BackendError {
    /// The class this error is reported as in a record.
    pub(crate) const fn class(&self) -> BackendErrorClass {
        match self {
            Self::Timeout { .. } => BackendErrorClass::Timeout,
            Self::NonZeroExit { .. } | Self::Io(_) | Self::ScriptExhausted { .. } => {
                BackendErrorClass::Invocation
            }
            Self::Parse(_) => BackendErrorClass::Protocol,
        }
    }
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout { timeout_ms } => {
                write!(f, "backend exceeded timeout of {timeout_ms} ms")
            }
            Self::NonZeroExit { code, stderr } => write!(f, "backend exited {code}: {stderr}"),
            Self::Io(msg) => write!(f, "backend io: {msg}"),
            Self::ScriptExhausted { attempt } => {
                write!(f, "replay script has no turn for attempt {attempt}")
            }
            Self::Parse(msg) => write!(f, "backend response parse: {msg}"),
        }
    }
}

impl std::error::Error for BackendError {}

/// Pluggable authoring backend.
///
/// `timeout` is a per-call obligation rather than backend state, so every
/// implementation honours the same deadline contract. Implementations MUST
/// abandon the call and return [`BackendError::Timeout`] once it elapses.
///
/// The deadline bounds the backend's execution. Collecting output it has
/// already written may take a short bounded moment beyond it; see
/// `DRAIN_GRACE`.
///
/// A backend that cannot block is exempt: [`ReplayBackend`] answers from memory
/// and has no deadline to exceed, so it never reports a timeout of its own.
pub(crate) trait AuthorevalBackend {
    /// Which backend and model this is, for the record.
    fn identity(&self) -> BackendIdentity;

    /// Sends one request and returns the parsed response.
    fn invoke(
        &self,
        request: &AuthorRequest<'_>,
        timeout: Duration,
    ) -> Result<AuthorResponse, BackendError>;
}

/// One scripted turn a [`ReplayBackend`] serves.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum ReplayTurn {
    /// Serve this response.
    Response(AuthorResponse),
    /// Fail with this classified error.
    Failure {
        /// Class to report the failure as.
        class: BackendErrorClass,
        /// Detail carried into the record for an invocation or protocol
        /// failure. A timeout's detail is its elapsed budget, so this is
        /// ignored for `timeout`.
        detail: String,
    },
}

/// A fixed script of turns, served in order, one per invocation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct ReplayScript {
    /// Model identity this script stands in for.
    pub(crate) model: String,
    /// Turns served in order.
    pub(crate) turns: Vec<ReplayTurn>,
}

/// Deterministic offline backend: no network, no API key, no harness.
///
/// It answers from memory, so `timeout` is only used to shape a scripted
/// timeout failure; it can never exceed a deadline on its own.
#[derive(Clone, Debug)]
pub(crate) struct ReplayBackend {
    script: ReplayScript,
    cursor: Cell<usize>,
}

impl ReplayBackend {
    /// Creates a replay backend from a script.
    pub(crate) const fn new(script: ReplayScript) -> Self {
        Self {
            script,
            cursor: Cell::new(0),
        }
    }
}

impl AuthorevalBackend for ReplayBackend {
    fn identity(&self) -> BackendIdentity {
        BackendIdentity {
            kind: "replay".to_owned(),
            model: self.script.model.clone(),
        }
    }

    fn invoke(
        &self,
        request: &AuthorRequest<'_>,
        timeout: Duration,
    ) -> Result<AuthorResponse, BackendError> {
        let index = self.cursor.get();
        let Some(turn) = self.script.turns.get(index) else {
            return Err(BackendError::ScriptExhausted {
                attempt: request.attempt,
            });
        };
        self.cursor.set(index + 1);

        match turn {
            ReplayTurn::Response(response) => Ok(response.clone()),
            ReplayTurn::Failure { class, detail } => Err(match *class {
                BackendErrorClass::Timeout => BackendError::Timeout {
                    timeout_ms: u64::try_from(timeout.as_millis()).unwrap_or(u64::MAX),
                },
                BackendErrorClass::Invocation => BackendError::Io(detail.clone()),
                BackendErrorClass::Protocol => BackendError::Parse(detail.clone()),
            }),
        }
    }
}
