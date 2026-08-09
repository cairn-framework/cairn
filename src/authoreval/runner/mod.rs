//! The bounded deterministic repair loop.
//!
//! One prompt, one scratch workspace, at most `1 + max_repairs` model
//! invocations, and exactly one record for every run that completes. An
//! instrument fault fails the run instead, producing no record. What is fed back after a failed attempt is
//! the previous scan's findings verbatim: the instrument never rewrites,
//! summarises, or ranks them, because what is under measurement is whether
//! cairn's own output is enough to repair from.

use std::time::Duration;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use super::backend::{
    AuthorRequest, AuthorResponse, AuthorevalBackend, BackendErrorClass, CommandBackend,
    ReplayBackend, SCHEMA_VERSION,
};
use super::prompt::Prompt;
use super::record::Record;
use super::scorer::{absolute_bin, score};
use super::workspace::Workspace;
use crate::error::CairnError;

/// Repair attempts allowed after the first response, by default.
pub(crate) const DEFAULT_MAX_REPAIRS: u32 = 3;

/// Per-call backend deadline in milliseconds, by default.
pub(crate) const DEFAULT_TIMEOUT_MS: u64 = 120_000;

/// Which backend a run drives.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BackendSpec {
    /// Serve the prompt file's own replay script. Offline and deterministic.
    Replay,
    /// Spawn a command speaking the JSON contract on stdin and stdout.
    Command {
        /// Program to spawn.
        program: String,
        /// Fixed arguments passed to it.
        args: Vec<String>,
        /// Model identity the command stands for.
        model: String,
    },
}

/// One run's configuration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RunConfig {
    /// Fixture copied into the scratch workspace. Never mutated.
    pub fixture: Utf8PathBuf,
    /// The `cairn` binary used for scoring.
    pub cairn_bin: Utf8PathBuf,
    /// Backend driving the run.
    pub backend: BackendSpec,
    /// Repair attempts allowed after the first response.
    pub max_repairs: u32,
    /// Per-call backend deadline, in milliseconds.
    pub timeout_ms: u64,
}

/// Runs one prompt file end to end and returns its record.
///
/// # Errors
///
/// Returns [`CairnError::AuthorEval`] for instrument faults only: an unreadable
/// or invalid prompt, a missing fixture, a production surface that will not
/// run, or an I/O failure while writing an already-validated response.
///
/// Anything that is the backend's doing is a record, not an error: failing to
/// answer, answering outside the response shape, writing an unusable or
/// conflicting path, and leaving an expected path unauthored all produce an
/// [`Outcome::BackendFailure`](super::record::Outcome::BackendFailure) record.
pub fn run_prompt_file(config: &RunConfig, prompt_path: &Utf8Path) -> Result<Record, CairnError> {
    let prompt = Prompt::load(prompt_path)?;

    match &config.backend {
        BackendSpec::Replay => {
            let Some(script) = prompt.replay.clone() else {
                return Err(CairnError::AuthorEval {
                    message: format!(
                        "prompt `{}` declares no replay script, so it cannot run offline",
                        prompt.id
                    ),
                });
            };
            run(config, &prompt, &ReplayBackend::new(script))
        }
        BackendSpec::Command {
            program,
            args,
            model,
        } => run(
            config,
            &prompt,
            &CommandBackend::new(program.clone(), args.clone(), model.clone()),
        ),
    }
}

/// Drives one prompt against one backend.
pub(crate) fn run(
    config: &RunConfig,
    prompt: &Prompt,
    backend: &dyn AuthorevalBackend,
) -> Result<Record, CairnError> {
    let workspace = Workspace::from_fixture(&config.fixture)?;
    let cairn_bin = absolute_bin(&config.cairn_bin)?;
    let identity = backend.identity();
    let max_attempts = config.max_repairs.saturating_add(1);
    let mut progress = Progress::default();

    for attempt in 1..=max_attempts {
        let request = AuthorRequest {
            schema_version: SCHEMA_VERSION,
            prompt_id: &prompt.id,
            attempt,
            instruction: &prompt.instruction,
            findings: &progress.last_failed,
        };

        let response = match backend.invoke(&request, Duration::from_millis(config.timeout_ms)) {
            Ok(response) => response,
            Err(error) => {
                let class = error.class();
                return Ok(progress.failed(prompt, identity, attempt, class, error.to_string()));
            }
        };
        progress
            .tokens
            .add(response.tokens.prompt, response.tokens.completion);

        let targets = match applicable(&workspace, prompt, &response) {
            Ok(targets) => targets,
            Err(detail) => {
                let class = BackendErrorClass::Protocol;
                return Ok(progress.failed(prompt, identity, attempt, class, detail));
            }
        };

        Workspace::write(&response.files, &targets)?;
        let verdict = score(&cairn_bin, workspace.root())?;
        if verdict.clean {
            return Ok(progress.clean(prompt, identity, attempt));
        }
        progress.observe(verdict);
    }

    Ok(progress.exhausted(prompt, identity, max_attempts))
}

mod progress;

use progress::Progress;

/// Checks a response can be applied and answers the prompt, resolving targets.
///
/// Two rules, both facts about the response rather than the instrument, so the
/// caller records a rejection as a protocol violation. The workspace judges the
/// whole batch in one place. And the fixture already scans clean, so an answer
/// that authors nothing, or something unrelated, would otherwise score
/// `clean_first_shot` and report perfect authorability for no authoring.
///
/// # Errors
///
/// Returns the detail to carry in the record.
fn applicable(
    workspace: &Workspace,
    prompt: &Prompt,
    response: &AuthorResponse,
) -> Result<Vec<(String, Utf8PathBuf)>, String> {
    let targets = workspace
        .validate(&response.files)
        .map_err(|reason| format!("response is not applicable: {reason}"))?;

    let written: Vec<String> = targets
        .iter()
        .map(|(relative, _)| relative.clone())
        .collect();
    let unmet = prompt.unmet(&written);
    if unmet.is_empty() {
        Ok(targets)
    } else {
        Err(format!(
            "response left the prompt's expected paths unauthored: {}",
            unmet.join(", ")
        ))
    }
}
