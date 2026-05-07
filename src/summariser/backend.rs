//! Summariser backend trait and the default disabled implementation.

use serde::{Deserialize, Serialize};

use super::request::{SummariserRequest, SummariserResponse};

/// Configured summariser mode. Disabled by default per phase-8 spec.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum SummariserMode {
    /// Summariser is disabled; no backend invocation occurs.
    #[default]
    Disabled,
    /// Spawn a local command per request, communicating via JSON over
    /// stdin/stdout.
    LocalCommand {
        /// Path to the executable.
        command: String,
        /// Optional command arguments.
        #[serde(default)]
        args: Vec<String>,
        /// Per-invocation timeout in milliseconds.
        timeout_ms: u64,
    },
    /// Reserved for a hosted API backend in a future phase.
    Hosted {
        /// Adapter identifier.
        adapter: String,
    },
}

/// Backend invocation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SummariserBackendError {
    /// Backend is disabled in the active configuration.
    Disabled,
    /// Local command exited non-zero.
    NonZeroExit {
        /// Exit code reported by the child.
        code: i32,
        /// Captured stderr.
        stderr: String,
    },
    /// Local command exceeded the configured timeout.
    Timeout {
        /// Configured timeout in milliseconds.
        timeout_ms: u64,
    },
    /// I/O error while spawning or communicating with the child.
    Io(String),
    /// Response could not be parsed as a `SummariserResponse`.
    Parse(String),
}

impl std::fmt::Display for SummariserBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => write!(f, "summariser backend is disabled"),
            Self::NonZeroExit { code, stderr } => {
                write!(f, "summariser backend exited {code}: {stderr}")
            }
            Self::Timeout { timeout_ms } => {
                write!(f, "summariser backend exceeded timeout of {timeout_ms} ms")
            }
            Self::Io(msg) => write!(f, "summariser io: {msg}"),
            Self::Parse(msg) => write!(f, "summariser response parse: {msg}"),
        }
    }
}

impl std::error::Error for SummariserBackendError {}

/// Pluggable summariser interface.
pub trait SummariserBackend {
    /// Sends one `SummariserRequest` and returns the parsed response.
    ///
    /// # Errors
    ///
    /// Returns `SummariserBackendError::Disabled` for the disabled
    /// backend, `NonZeroExit` or `Timeout` for failed local-command
    /// invocations, `Io` for spawn or pipe errors, and `Parse` when the
    /// response cannot be parsed.
    fn invoke(
        &self,
        request: &SummariserRequest,
    ) -> Result<SummariserResponse, SummariserBackendError>;
}

/// Default backend that always refuses. Used when summariser mode is
/// `Disabled` or when no backend is configured.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisabledBackend;

impl SummariserBackend for DisabledBackend {
    fn invoke(
        &self,
        _request: &SummariserRequest,
    ) -> Result<SummariserResponse, SummariserBackendError> {
        Err(SummariserBackendError::Disabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summariser::request::NodeContext;

    fn sample_request() -> SummariserRequest {
        SummariserRequest {
            version: 1,
            artefact_type: "contract".to_owned(),
            node: NodeContext {
                node_id: "node-a".to_owned(),
                name: "Auth".to_owned(),
                description: String::new(),
                contract: None,
                contradiction: None,
            },
        }
    }

    #[test]
    fn mode_default_is_disabled() {
        let mode = SummariserMode::default();
        assert!(matches!(mode, SummariserMode::Disabled));
    }

    #[test]
    fn disabled_backend_returns_disabled_error() {
        let backend = DisabledBackend;
        let request = sample_request();
        let err = backend.invoke(&request).expect_err("should error");
        assert!(matches!(err, SummariserBackendError::Disabled));
    }

    #[test]
    fn mode_round_trips_through_serde() {
        let mode = SummariserMode::LocalCommand {
            command: "/usr/local/bin/summariser".to_owned(),
            args: vec!["--quiet".to_owned()],
            timeout_ms: 30_000,
        };
        let json = serde_json::to_string(&mode).expect("serialise");
        let back: SummariserMode = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, mode);
    }
}
