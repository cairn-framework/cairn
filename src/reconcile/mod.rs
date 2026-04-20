//! Reconciler trait and report types.

pub mod code;
pub mod fingerprint;
mod rust_semantics;
mod validation;

pub(crate) use validation::semantic_findings;

use std::{error::Error, fmt, path::Path};

use crate::map::graph::Finding;

/// Reconciler identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReconcilerId(pub String);

/// Reconcile request.
pub struct ReconcileRequest<'a> {
    /// Project root.
    pub root: &'a Path,
    /// Ignore patterns.
    pub ignores: &'a [String],
}

/// Reconcile report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconcileReport {
    /// Files claimed by node ID.
    pub claimed_files: std::collections::BTreeMap<String, Vec<String>>,
    /// Public symbols.
    pub symbols: Vec<String>,
    /// Interface fingerprint.
    pub fingerprint: fingerprint::InterfaceFingerprint,
    /// Source-level dependency observations.
    pub dependencies: Vec<DependencyObservation>,
    /// Parsed Cairn fact lines from source docstrings.
    pub docstrings: Vec<DocstringFacts>,
    /// Reconciliation findings.
    pub findings: Vec<Finding>,
}

/// Confidence assigned to a source dependency observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationConfidence {
    /// The observation resolved to exactly one owning node.
    High,
    /// The observation was plausible but not uniquely resolvable.
    Low,
}

/// Dependency observed in source code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObservation {
    /// Source Cairn node that owns the observed file.
    pub from: String,
    /// Resolved target node when unique.
    pub to: Option<String>,
    /// Candidate target nodes when ambiguous.
    pub candidates: Vec<String>,
    /// Original dependency text.
    pub reference: String,
    /// File containing the observation.
    pub path: String,
    /// One-based source line.
    pub line: usize,
    /// One-based source column.
    pub column: usize,
    /// Resolution confidence.
    pub confidence: ObservationConfidence,
}

/// Parsed Cairn fact lines attached to a Rust module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocstringFacts {
    /// Node that owns the documented module source.
    pub owner: String,
    /// File containing the docstring.
    pub path: String,
    /// Parsed fact lines in source order.
    pub facts: Vec<DocstringFact>,
}

/// One parsed Cairn docstring fact line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocstringFact {
    /// Fact key without the `Cairn-` prefix.
    pub key: String,
    /// Raw fact value after trimming ASCII whitespace.
    pub value: String,
    /// One-based source line.
    pub line: usize,
    /// One-based source column.
    pub column: usize,
}

/// Reconciler error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconcileError {
    /// Stable code.
    pub code: String,
    /// Message.
    pub message: String,
}

impl fmt::Display for ReconcileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for ReconcileError {}

/// Domain-agnostic reconciler interface.
pub trait Reconciler {
    /// Reconciler ID.
    fn id(&self) -> ReconcilerId;

    /// Reconciles project reality.
    ///
    /// # Errors
    ///
    /// Returns a reconciler error when source discovery or analysis fails.
    fn reconcile(&self, request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError>;
}
