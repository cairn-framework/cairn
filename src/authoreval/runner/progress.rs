//! The state one run carries between attempts, and the records it builds.

use std::collections::{BTreeMap, BTreeSet};

use super::super::backend::{BackendErrorClass, BackendIdentity};
use super::super::prompt::Prompt;
use super::super::record::{
    Hotspot, Outcome, RECORD_SCHEMA_VERSION, Record, RecordError, TokenTotals,
};
use super::super::scorer::{Finding, ScanVerdict};
use super::super::taxonomy::classify;

/// What the loop carries between attempts.
#[derive(Default)]
pub(super) struct Progress {
    pub(super) tokens: TokenTotals,
    pub(super) last_failed: Vec<Finding>,
    previous_codes: BTreeSet<String>,
    persisted: BTreeSet<String>,
}

impl Progress {
    /// Records a failed scan.
    ///
    /// `persisted` is intersected against the immediately preceding scan only,
    /// never the union of every scan, so a code that disappears and comes back
    /// did not survive the feedback in between.
    pub(super) fn observe(&mut self, verdict: ScanVerdict) {
        let codes: BTreeSet<String> = verdict
            .findings
            .iter()
            .map(|finding| finding.code.clone())
            .collect();
        self.persisted = codes.intersection(&self.previous_codes).cloned().collect();
        self.previous_codes = codes;
        self.last_failed = verdict.findings;
    }

    /// The hotspots of the last failed scan, empty before one has happened.
    fn hotspots(&self) -> Vec<Hotspot> {
        hotspots(&self.last_failed, &self.persisted)
    }

    /// A record for an attempt that scanned clean.
    pub(super) fn clean(
        &self,
        prompt: &Prompt,
        backend: BackendIdentity,
        iterations: u32,
    ) -> Record {
        Record {
            schema_version: RECORD_SCHEMA_VERSION,
            prompt_id: prompt.id.clone(),
            outcome: if iterations == 1 {
                Outcome::CleanFirstShot
            } else {
                Outcome::CleanAfterRepair
            },
            backend,
            iterations,
            tokens: self.tokens,
            first_shot_valid: iterations == 1,
            hotspots: self.hotspots(),
            error: None,
        }
    }

    /// A record for a run that reached its repair bound still dirty.
    pub(super) fn exhausted(
        &self,
        prompt: &Prompt,
        backend: BackendIdentity,
        iterations: u32,
    ) -> Record {
        Record {
            schema_version: RECORD_SCHEMA_VERSION,
            prompt_id: prompt.id.clone(),
            outcome: Outcome::RepairBoundExhausted,
            backend,
            iterations,
            tokens: self.tokens,
            first_shot_valid: false,
            hotspots: self.hotspots(),
            error: None,
        }
    }

    /// A `backend_failure` record: the classified error, and never hotspots.
    ///
    /// An infrastructure or protocol failure is not evidence about authoring
    /// quality, so carrying an earlier scan's hotspots would misattribute it.
    pub(super) fn failed(
        &self,
        prompt: &Prompt,
        backend: BackendIdentity,
        iterations: u32,
        class: BackendErrorClass,
        detail: String,
    ) -> Record {
        Record {
            schema_version: RECORD_SCHEMA_VERSION,
            prompt_id: prompt.id.clone(),
            outcome: Outcome::BackendFailure,
            backend,
            iterations,
            tokens: self.tokens,
            first_shot_valid: false,
            hotspots: Vec::new(),
            error: Some(RecordError { class, detail }),
        }
    }
}

/// Aggregates the last failed scan's findings into per-code hotspots, sorted
/// by code.
fn hotspots(findings: &[Finding], persisted: &BTreeSet<String>) -> Vec<Hotspot> {
    let mut by_code: BTreeMap<&str, Hotspot> = BTreeMap::new();

    for finding in findings {
        by_code
            .entry(finding.code.as_str())
            .and_modify(|hotspot| {
                hotspot.count = hotspot.count.saturating_add(1);
                // Findings sort with `None` first, so the first one seen may
                // carry no location while a later one does. The hotspot
                // documents the first location the code named, not the first
                // finding's absence of one.
                if hotspot.node.is_none() {
                    hotspot.node.clone_from(&finding.node);
                }
                if hotspot.path.is_none() {
                    hotspot.path.clone_from(&finding.path);
                }
            })
            .or_insert_with(|| {
                let (class, subclass) = classify(
                    &finding.code,
                    finding.envelope_parse_span(),
                    persisted.contains(&finding.code),
                );
                Hotspot {
                    class,
                    subclass,
                    code: finding.code.clone(),
                    severity: finding.severity.clone(),
                    count: 1,
                    node: finding.node.clone(),
                    path: finding.path.clone(),
                }
            });
    }

    by_code.into_values().collect()
}
