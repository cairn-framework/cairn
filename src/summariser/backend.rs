//! Summariser backend trait and the default disabled implementation.

use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    time::Duration,
};

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

impl SummariserMode {
    /// Returns the configured per-invocation timeout, when applicable.
    #[must_use]
    pub const fn timeout(&self) -> Option<Duration> {
        match self {
            Self::Disabled | Self::Hosted { .. } => None,
            Self::LocalCommand { timeout_ms, .. } => Some(Duration::from_millis(*timeout_ms)),
        }
    }
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
///
/// `timeout` is a per-call obligation (not pulled from `&self`) so that
/// hosted backends can honour the same contract without re-deriving the
/// configured timeout from a mode struct. Implementations MUST kill the
/// child or cancel the request when the deadline elapses and return
/// `SummariserBackendError::Timeout`.
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
        timeout: Duration,
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
        _timeout: Duration,
    ) -> Result<SummariserResponse, SummariserBackendError> {
        Err(SummariserBackendError::Disabled)
    }
}

/// Deterministic fake backend for tests.
///
/// Returns a pre-configured response on every invocation,
/// ignoring the request payload and timeout. This makes tests
/// deterministic and removes the need for real LLM backends
/// during development.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FakeBackend {
    response: Result<SummariserResponse, SummariserBackendError>,
}

impl FakeBackend {
    /// Creates a fake that always returns `response`.
    #[must_use]
    pub const fn ok(response: SummariserResponse) -> Self {
        Self {
            response: Ok(response),
        }
    }

    /// Creates a fake that always returns `error`.
    #[must_use]
    pub const fn err(error: SummariserBackendError) -> Self {
        Self {
            response: Err(error),
        }
    }
}

impl SummariserBackend for FakeBackend {
    fn invoke(
        &self,
        _request: &SummariserRequest,
        _timeout: Duration,
    ) -> Result<SummariserResponse, SummariserBackendError> {
        self.response.clone()
    }
}

/// Local command backend: spawns a subprocess, sends JSON on stdin,
/// reads JSON from stdout, and enforces a per-call timeout.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)] // Reason: will be constructed by CLI wiring in upcoming task 4.1
pub struct LocalCommandBackend {
    command: String,
    args: Vec<String>,
}

#[allow(dead_code)] // Reason: same as struct; used by CLI wiring in task 4.1
impl LocalCommandBackend {
    /// Creates a backend from a command path and argument list.
    #[must_use]
    pub const fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

impl SummariserBackend for LocalCommandBackend {
    fn invoke(
        &self,
        request: &SummariserRequest,
        timeout: Duration,
    ) -> Result<SummariserResponse, SummariserBackendError> {
        let json = serde_json::to_string(request)
            .map_err(|e| SummariserBackendError::Io(format!("failed to serialise request: {e}")))?;

        let mut child = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| SummariserBackendError::Io(e.to_string()))?;

        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| SummariserBackendError::Io("failed to open stdin".to_owned()))?;
        stdin
            .write_all(json.as_bytes())
            .map_err(|e| SummariserBackendError::Io(e.to_string()))?;
        drop(stdin);

        let mut stdout_pipe = child
            .stdout
            .take()
            .ok_or_else(|| SummariserBackendError::Io("failed to open stdout".to_owned()))?;
        let mut stderr_pipe = child
            .stderr
            .take()
            .ok_or_else(|| SummariserBackendError::Io("failed to open stderr".to_owned()))?;

        let stdout_thread = std::thread::spawn(move || {
            let mut buf = String::new();
            let _ = stdout_pipe.read_to_string(&mut buf);
            buf
        });
        let stderr_thread = std::thread::spawn(move || {
            let mut buf = String::new();
            let _ = stderr_pipe.read_to_string(&mut buf);
            buf
        });

        let start = std::time::Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(st)) => break st,
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        let _ = child.kill();
                        let _ = child.wait();
                        let _ = stdout_thread.join();
                        let _ = stderr_thread.join();
                        return Err(SummariserBackendError::Timeout {
                            timeout_ms: u64::try_from(timeout.as_millis()).unwrap_or(u64::MAX),
                        });
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    let _ = child.kill();
                    let _ = stdout_thread.join();
                    let _ = stderr_thread.join();
                    return Err(SummariserBackendError::Io(e.to_string()));
                }
            }
        };

        let stdout = stdout_thread
            .join()
            .map_err(|e| SummariserBackendError::Io(format!("stdout thread panicked: {e:?}")))?;
        let stderr = stderr_thread
            .join()
            .map_err(|e| SummariserBackendError::Io(format!("stderr thread panicked: {e:?}")))?;

        if !status.success() {
            return Err(SummariserBackendError::NonZeroExit {
                code: status.code().unwrap_or(-1),
                stderr,
            });
        }

        serde_json::from_str(&stdout).map_err(|e| SummariserBackendError::Parse(e.to_string()))
    }
}

/// Configuration for a hosted API backend.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HostedConfig {
    /// Adapter identifier (e.g. "openai", "anthropic").
    pub adapter: String,
    /// Base URL for the hosted API.
    #[serde(default)]
    pub base_url: Option<String>,
    /// Per-invocation timeout in milliseconds.
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

/// Hosted API backend placeholder.
///
/// Parses and stores configuration but returns an unsupported-backend
/// error on every invocation. A concrete provider adapter will replace
/// this behaviour in a future phase.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)] // Reason: placeholder until concrete provider adapter lands
pub struct HostedBackend {
    config: HostedConfig,
}

#[allow(dead_code)] // Reason: same as struct; placeholder until adapter lands
impl HostedBackend {
    /// Creates a hosted backend from parsed configuration.
    #[must_use]
    pub const fn new(config: HostedConfig) -> Self {
        Self { config }
    }
}

impl SummariserBackend for HostedBackend {
    fn invoke(
        &self,
        _request: &SummariserRequest,
        _timeout: Duration,
    ) -> Result<SummariserResponse, SummariserBackendError> {
        Err(SummariserBackendError::Io(format!(
            "unsupported hosted backend: {}",
            self.config.adapter
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summariser::request::{NodeContext, SUMMARISER_SCHEMA_VERSION};

    fn sample_request() -> SummariserRequest {
        SummariserRequest {
            schema_version: SUMMARISER_SCHEMA_VERSION,
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
        assert!(mode.timeout().is_none());
    }

    #[test]
    fn local_command_mode_exposes_timeout() {
        let mode = SummariserMode::LocalCommand {
            command: "/bin/cat".to_owned(),
            args: Vec::new(),
            timeout_ms: 5000,
        };
        assert_eq!(mode.timeout(), Some(Duration::from_secs(5)));
    }

    #[test]
    fn disabled_backend_returns_disabled_error() {
        let backend = DisabledBackend;
        let request = sample_request();
        let err = backend
            .invoke(&request, Duration::from_secs(1))
            .expect_err("should error");
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

    #[test]
    fn test_fake_backend_ok_returns_configured_response() {
        let response = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "fake summary".to_owned(),
            metadata: None,
        };
        let backend = FakeBackend::ok(response.clone());
        let result = backend.invoke(&sample_request(), Duration::from_secs(1));
        assert_eq!(result.unwrap(), response);
    }

    #[test]
    fn test_fake_backend_err_returns_configured_error() {
        let backend = FakeBackend::err(SummariserBackendError::Disabled);
        let result = backend.invoke(&sample_request(), Duration::from_secs(1));
        assert!(matches!(result, Err(SummariserBackendError::Disabled)));
    }

    #[test]
    fn test_fake_backend_different_requests_returns_same_response() {
        let response = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "deterministic".to_owned(),
            metadata: None,
        };
        let backend = FakeBackend::ok(response);
        let req1 = sample_request();
        let req2 = SummariserRequest {
            node: NodeContext {
                node_id: "different".to_owned(),
                ..req1.node.clone()
            },
            ..req1.clone()
        };
        let out1 = backend.invoke(&req1, Duration::from_millis(100)).unwrap();
        let out2 = backend.invoke(&req2, Duration::from_secs(10)).unwrap();
        assert_eq!(out1, out2);
    }

    #[test]
    fn test_local_command_backend_echoes_valid_response() {
        let expected = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "from shell".to_owned(),
            metadata: None,
        };
        let json = serde_json::to_string(&expected).unwrap();
        let script = format!("printf '%s\\n' '{json}'");
        let backend = LocalCommandBackend::new("/bin/sh".to_owned(), vec!["-c".to_owned(), script]);
        let result = backend
            .invoke(&sample_request(), Duration::from_secs(5))
            .expect("should succeed");
        assert_eq!(result.summary, expected.summary);
    }

    #[test]
    fn test_local_command_backend_non_zero_exit_returns_error() {
        let backend = LocalCommandBackend::new(
            "/bin/sh".to_owned(),
            vec!["-c".to_owned(), "echo 'bad' >&2; exit 1".to_owned()],
        );
        let result = backend.invoke(&sample_request(), Duration::from_secs(5));
        match result {
            Err(SummariserBackendError::NonZeroExit { code, stderr }) => {
                assert_eq!(code, 1);
                assert!(
                    stderr.contains("bad"),
                    "stderr should contain 'bad', got: {stderr}"
                );
            }
            other => panic!("expected NonZeroExit, got {other:?}"),
        }
    }

    #[test]
    fn test_local_command_backend_invalid_json_returns_parse_error() {
        let backend = LocalCommandBackend::new(
            "/bin/sh".to_owned(),
            vec!["-c".to_owned(), "echo 'not json'".to_owned()],
        );
        let result = backend.invoke(&sample_request(), Duration::from_secs(5));
        assert!(
            matches!(result, Err(SummariserBackendError::Parse(_))),
            "expected Parse error for invalid JSON, got {result:?}"
        );
    }

    #[test]
    fn test_local_command_backend_timeout_returns_timeout_error() {
        let backend = LocalCommandBackend::new(
            "/bin/sh".to_owned(),
            vec!["-c".to_owned(), "sleep 10".to_owned()],
        );
        let result = backend.invoke(&sample_request(), Duration::from_millis(100));
        assert!(
            matches!(result, Err(SummariserBackendError::Timeout { .. })),
            "expected Timeout error, got {result:?}"
        );
    }

    #[test]
    fn test_hosted_backend_returns_unsupported_error() {
        let config = HostedConfig {
            adapter: "openai".to_owned(),
            base_url: Some("https://api.openai.com".to_owned()),
            timeout_ms: Some(30_000),
        };
        let backend = HostedBackend::new(config);
        let result = backend.invoke(&sample_request(), Duration::from_secs(5));
        match result {
            Err(SummariserBackendError::Io(msg)) => {
                assert_eq!(msg, "unsupported hosted backend: openai");
            }
            other => panic!("expected Io error, got {other:?}"),
        }
    }

    #[test]
    fn test_hosted_config_round_trips_through_serde() {
        let config = HostedConfig {
            adapter: "anthropic".to_owned(),
            base_url: Some("https://api.anthropic.com".to_owned()),
            timeout_ms: Some(60_000),
        };
        let json = serde_json::to_string(&config).expect("serialise");
        let back: HostedConfig = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, config);
    }

    #[test]
    fn test_hosted_config_with_defaults_parses() {
        let json = r#"{"adapter":"test"}"#;
        let config: HostedConfig = serde_json::from_str(json).expect("parse");
        assert_eq!(config.adapter, "test");
        assert!(config.base_url.is_none());
        assert!(config.timeout_ms.is_none());
    }
}
