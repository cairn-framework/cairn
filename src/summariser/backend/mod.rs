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
        if let Err(e) = stdin.write_all(json.as_bytes()) {
            // Reason: the command may exit before reading stdin (e.g. echo
            // that never reads).  We still need stdout, stderr and exit
            // status; only propagate non-broken-pipe errors.
            if e.kind() != std::io::ErrorKind::BrokenPipe {
                return Err(SummariserBackendError::Io(e.to_string()));
            }
        }
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
mod tests;
