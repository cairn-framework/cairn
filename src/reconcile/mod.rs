//! Reconciler trait and report types.

pub mod code;
pub mod fingerprint;
pub mod go;
pub mod python;
pub mod target;
pub mod typescript;

/// Fixture reconciler demonstrating the extension API.
pub mod fixture;

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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReconcileReport {
    /// Files claimed by node ID.
    pub claimed_files: std::collections::BTreeMap<String, Vec<String>>,
    /// Public symbols.
    #[serde(with = "serde_arc_vec")]
    pub symbols: std::sync::Arc<Vec<String>>,
    /// Interface fingerprint.
    pub fingerprint: fingerprint::InterfaceFingerprint,
    /// Reconciliation findings.
    pub findings: Vec<Finding>,
}

/// Serde helpers for `Arc<Vec<String>>`.
mod serde_arc_vec {
    use std::sync::Arc;

    /// Serializes the inner `Vec<String>` through the `Arc`.
    pub(crate) fn serialize<S: serde::Serializer>(
        value: &Arc<Vec<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(value.as_ref(), serializer)
    }

    /// Deserializes a `Vec<String>` and wraps it in an `Arc`.
    pub(crate) fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Arc<Vec<String>>, D::Error> {
        let v: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Arc::new(v))
    }
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
