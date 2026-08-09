//! The real backend: a subprocess speaking the JSON contract.
//!
//! The plumbing deliberately mirrors `summariser::backend::LocalCommandBackend`
//! rather than sharing it, which would couple a development instrument to a
//! stable product module. Three details differ, and each is why the deadline is
//! real: it starts at entry, the output readers start before the request is
//! written, and every error raised before the child is seen to exit kills and
//! reaps it. Errors after that point (a drain timeout, a non-zero status, an
//! unparseable answer) see an already-reaped child.

use std::io::{Read as _, Write as _};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

use super::{
    AuthorRequest, AuthorResponse, AuthorevalBackend, BackendError, BackendIdentity, DRAIN_GRACE,
};

/// Poll interval while waiting for the child, never allowed to outlast the
/// deadline.
const POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Real backend: spawns a command, writes the request JSON to stdin, reads the
/// response JSON from stdout, and abandons the child when the deadline elapses.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandBackend {
    command: String,
    args: Vec<String>,
    model: String,
}

impl CommandBackend {
    /// Creates a command backend from a program, its arguments, and the model
    /// identity it stands for.
    pub(crate) const fn new(command: String, args: Vec<String>, model: String) -> Self {
        Self {
            command,
            args,
            model,
        }
    }

    /// Spawns the child with all three streams piped.
    fn spawn(&self) -> Result<Child, BackendError> {
        Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| BackendError::Io(e.to_string()))
    }
}

/// The three off-thread stream workers, each reporting through a channel.
struct Streams {
    stdout: Receiver<String>,
    stderr: Receiver<String>,
    writer: Receiver<Option<String>>,
}

/// Starts the readers, then the writer.
///
/// Order matters. A backend that emits a pipe buffer's worth of output before
/// draining stdin would deadlock against a parent blocked in `write_all`, so
/// nothing is written until both readers are running, and the write itself runs
/// off-thread so a backend that never reads cannot hold the call open.
fn start_streams(child: &mut Child, request: String) -> Result<Streams, BackendError> {
    let pipes = (child.stdin.take(), child.stdout.take(), child.stderr.take());
    let (Some(mut stdin), Some(mut out_pipe), Some(mut err_pipe)) = pipes else {
        return Err(BackendError::Io(
            "failed to open the backend's pipes".to_owned(),
        ));
    };

    let (stdout_tx, stdout) = channel();
    let (stderr_tx, stderr) = channel();
    let (writer_tx, writer) = channel();

    std::thread::spawn(move || {
        let mut buf = String::new();
        let _ = out_pipe.read_to_string(&mut buf);
        let _ = stdout_tx.send(buf);
    });
    std::thread::spawn(move || {
        let mut buf = String::new();
        let _ = err_pipe.read_to_string(&mut buf);
        let _ = stderr_tx.send(buf);
    });
    std::thread::spawn(move || {
        // A backend may exit before reading stdin. That broken pipe is not an
        // error; the exit status and stdout still carry the answer. Dropping
        // stdin here closes it, signalling end of input.
        let outcome = match stdin.write_all(request.as_bytes()) {
            Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => Some(e.to_string()),
            _ => None,
        };
        let _ = writer_tx.send(outcome);
    });

    Ok(Streams {
        stdout,
        stderr,
        writer,
    })
}

/// Waits for the child, abandoning it once the deadline passes.
fn wait_for_exit(
    child: &mut Child,
    deadline: Instant,
    timeout_ms: u64,
) -> Result<ExitStatus, BackendError> {
    loop {
        match child.try_wait() {
            // The clock is checked even on exit: a child that finished after
            // the deadline finished too late, and accepting it would make the
            // deadline advisory.
            Ok(Some(status)) => {
                if Instant::now() >= deadline {
                    return Err(finish(child, BackendError::Timeout { timeout_ms }));
                }
                return Ok(status);
            }
            Ok(None) => {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return Err(finish(child, BackendError::Timeout { timeout_ms }));
                }
                // Never sleep past the deadline: a 1 ms budget must not buy the
                // child a 10 ms poll interval.
                std::thread::sleep(remaining.min(POLL_INTERVAL));
            }
            Err(e) => return Err(finish(child, BackendError::Io(e.to_string()))),
        }
    }
}

/// Collects the streams under one shared drain deadline.
///
/// The deadline is fixed at [`DRAIN_GRACE`] from the moment the child was seen
/// to exit. Not one budget per pipe, which would triple it, and not whatever
/// remains of the execution deadline, which would let a child that exits early
/// buy a held pipe almost the whole timeout.
fn collect(streams: &Streams, timeout_ms: u64) -> Result<(String, String), BackendError> {
    let drain_deadline = Instant::now() + DRAIN_GRACE;
    let remaining = || drain_deadline.saturating_duration_since(Instant::now());

    let stdout = streams
        .stdout
        .recv_timeout(remaining())
        .map_err(|_| BackendError::Timeout { timeout_ms })?;

    // stderr is diagnostics, not the answer. If a descendant is holding it
    // open, an empty string beats failing a call whose stdout already arrived.
    let stderr = streams.stderr.recv_timeout(remaining()).unwrap_or_default();

    // A timeout here is not "the write succeeded": a descendant may still hold
    // stdin and the request may never have been delivered in full.
    match streams.writer.recv_timeout(remaining()) {
        Ok(None) => Ok((stdout, stderr)),
        Ok(Some(message)) => Err(BackendError::Io(message)),
        Err(_) => Err(BackendError::Timeout { timeout_ms }),
    }
}

/// Kills and reaps `child`, then returns `error`.
///
/// Every error raised while the child may still be running funnels through
/// here, so a failed invocation never leaves a running or zombie backend
/// behind. Errors raised after the child was seen to exit skip it: it is
/// already reaped. Only the direct child is handled; a backend that forks a
/// descendant inheriting its streams is out of contract.
fn finish(child: &mut Child, error: BackendError) -> BackendError {
    let _ = child.kill();
    let _ = child.wait();
    error
}

impl AuthorevalBackend for CommandBackend {
    fn identity(&self) -> BackendIdentity {
        BackendIdentity {
            kind: "command".to_owned(),
            model: self.model.clone(),
        }
    }

    fn invoke(
        &self,
        request: &AuthorRequest<'_>,
        timeout: Duration,
    ) -> Result<AuthorResponse, BackendError> {
        let timeout_ms = u64::try_from(timeout.as_millis()).unwrap_or(u64::MAX);
        // A caller-supplied budget can be large enough to overflow `Instant`;
        // saturating to "effectively never" beats panicking inside a call whose
        // whole job is to fail politely.
        let now = Instant::now();
        let deadline = now
            .checked_add(timeout)
            .unwrap_or_else(|| now + Duration::from_hours(24));

        let json = serde_json::to_string(request)
            .map_err(|e| BackendError::Io(format!("failed to serialise request: {e}")))?;

        // Nothing is spawned once the deadline has already passed; a zero or
        // exhausted budget must not start a backend it cannot wait for.
        if Instant::now() >= deadline {
            return Err(BackendError::Timeout { timeout_ms });
        }

        let mut child = self.spawn()?;
        let streams = match start_streams(&mut child, json) {
            Ok(streams) => streams,
            Err(error) => return Err(finish(&mut child, error)),
        };

        let status = wait_for_exit(&mut child, deadline, timeout_ms)?;
        let (stdout, stderr) = collect(&streams, timeout_ms)?;

        if !status.success() {
            return Err(BackendError::NonZeroExit {
                code: status.code().unwrap_or(-1),
                stderr,
            });
        }

        serde_json::from_str(&stdout).map_err(|e| BackendError::Parse(e.to_string()))
    }
}
