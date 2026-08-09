//! The record schema.
//!
//! A run that completes produces exactly one record per prompt, whatever the
//! model did, so a live run's records can be counted against its corpus. An
//! instrument fault (an unreadable prompt, a missing fixture, a production
//! surface that will not run, a failed write) produces no record at all: it
//! fails the run. A record can never be read without knowing which backend and
//! which model produced it.

use serde::{Deserialize, Serialize};

use super::backend::{BackendErrorClass, BackendIdentity};
use super::taxonomy::{FailureClass, FailureSubclass};

/// Wire schema version for records.
pub const RECORD_SCHEMA_VERSION: u32 = 1;

/// How a run ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    /// The first response scanned clean.
    CleanFirstShot,
    /// A later response scanned clean, after repair feedback.
    CleanAfterRepair,
    /// The repair bound was reached with the scan still dirty.
    RepairBoundExhausted,
    /// The backend did not deliver a usable answer: it failed to respond, timed
    /// out, or answered outside the protocol (an unusable path, or leaving the
    /// prompt's expected paths unauthored).
    BackendFailure,
}

/// Tokens spent across every response a run received.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenTotals {
    /// Prompt tokens.
    pub prompt: u64,
    /// Completion tokens.
    pub completion: u64,
    /// Prompt plus completion.
    pub total: u64,
}

impl TokenTotals {
    /// Adds one response's usage.
    pub(crate) const fn add(&mut self, prompt: u64, completion: u64) {
        self.prompt = self.prompt.saturating_add(prompt);
        self.completion = self.completion.saturating_add(completion);
        self.total = self.prompt.saturating_add(self.completion);
    }
}

/// One finding code the last failed scan produced, with its attribution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Hotspot {
    /// Syntax, generated guidance, or a missing repair affordance.
    pub class: FailureClass,
    /// Where the failure originates.
    pub subclass: FailureSubclass,
    /// The finding code.
    pub code: String,
    /// Severity as published on the lint wire.
    pub severity: String,
    /// How many findings carried this code.
    pub count: u32,
    /// First node the code named, when it named one.
    pub node: Option<String>,
    /// First path the code named, when it named one.
    pub path: Option<String>,
}

/// A classified backend failure, as carried in a record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordError {
    /// Failure class.
    pub class: BackendErrorClass,
    /// Human-readable detail.
    pub detail: String,
}

/// The result of running one prompt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Wire schema version.
    pub schema_version: u32,
    /// The prompt this record measures.
    pub prompt_id: String,
    /// How the run ended.
    pub outcome: Outcome,
    /// Which backend and model produced the responses.
    pub backend: BackendIdentity,
    /// Model invocations made, including a failed one.
    pub iterations: u32,
    /// Tokens spent across every response received.
    pub tokens: TokenTotals,
    /// Whether the first response scanned clean.
    pub first_shot_valid: bool,
    /// Hotspots from the last failed scan. Empty on a first-shot-clean run, and
    /// on a backend failure even when an earlier attempt did scan: an
    /// infrastructure or protocol failure is not evidence about authoring.
    pub hotspots: Vec<Hotspot>,
    /// The classified error, when the backend did not deliver a usable answer:
    /// it failed to respond, timed out, or answered outside the protocol.
    pub error: Option<RecordError>,
}
