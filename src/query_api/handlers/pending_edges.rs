//! Reverse provenance edge DTOs and projections for pending decisions.

use crate::artefacts::registry::Decision;
use serde::Serialize;
use std::collections::BTreeMap;

/// One reverse provenance edge carried by a pending decision briefing.
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct PendingDecisionEdge {
    /// Referenced decision ID.
    pub id: String,
    /// Referenced decision status.
    pub status: String,
    /// Referenced decision date.
    pub date: String,
}

/// Project reverse decision IDs into status and date-bearing wire edges.
pub(super) fn edge_rows(
    ids: &[String],
    decision_index: &BTreeMap<&str, &Decision>,
) -> Vec<PendingDecisionEdge> {
    ids.iter()
        .filter_map(|id| decision_index.get(id.as_str()))
        .map(|decision| PendingDecisionEdge {
            id: decision.id.clone(),
            status: crate::query_api::decision_status(decision.status).to_owned(),
            date: decision.date.clone(),
        })
        .collect()
}
