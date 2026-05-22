//! Fixture non-code reconciler demonstrating the extension API.
//!
//! This reconciler is test-only and produces observations (findings)
//! without claiming files or proposing new nodes. It demonstrates how
//! a domain outside source code (e.g., infrastructure, compliance) can
//! contribute findings to the map through the reconciler trait.

use super::{ReconcileError, ReconcileReport, ReconcileRequest, Reconciler, ReconcilerId};
use crate::map::graph::{Finding, FindingSeverity};

/// A fixture reconciler that produces deterministic observations.
///
/// This is a demonstration reconciler for testing the extension API.
/// It does not claim files or produce symbols; it only emits findings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureReconciler {
    id: String,
}

impl FixtureReconciler {
    /// Create a new fixture reconciler with the given ID.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl Reconciler for FixtureReconciler {
    fn id(&self) -> ReconcilerId {
        ReconcilerId(format!("fixture/{}", self.id))
    }

    fn reconcile(&self, _request: ReconcileRequest<'_>) -> Result<ReconcileReport, ReconcileError> {
        let findings = vec![Finding {
            code: "FIXTURE-001".to_owned(),
            severity: FindingSeverity::Info,
            message: format!("Fixture reconciler '{}' observed state", self.id),
            node: Some("fixture.observation".to_owned()),
            path: None,
        }];

        Ok(ReconcileReport {
            claimed_files: std::collections::BTreeMap::new(),
            symbols: Vec::new(),
            fingerprint: super::fingerprint::InterfaceFingerprint::from_symbols(&[]),
            findings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_reconciler_produces_findings() {
        let reconciler = FixtureReconciler::new("test");
        let request = ReconcileRequest {
            root: std::path::Path::new("."),
            ignores: &[],
        };

        let report = reconciler.reconcile(request).unwrap();
        assert!(!report.findings.is_empty());
        assert!(report.claimed_files.is_empty());
        assert!(report.symbols.is_empty());
    }

    #[test]
    fn test_fixture_reconciler_id_format() {
        let reconciler = FixtureReconciler::new("demo");
        assert_eq!(reconciler.id().0, "fixture/demo");
    }
}
