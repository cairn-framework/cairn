//! Human-readable decision rendering and reverse provenance lines.

use crate::artefacts::registry::Decision;
use crate::query_api::decision_status;
use std::collections::BTreeMap;

/// Build a status/date lookup for decisions rendered together.
pub(crate) fn decision_index(decisions: &[Decision]) -> BTreeMap<String, (String, String)> {
    decisions
        .iter()
        .map(|decision| {
            (
                decision.id.clone(),
                (
                    decision_status(decision.status).to_owned(),
                    decision.date.clone(),
                ),
            )
        })
        .collect()
}

/// Render a decision and its reverse provenance edges.
pub(crate) fn decision_line_with_index(
    decision: &Decision,
    index: &BTreeMap<String, (String, String)>,
) -> String {
    let mut line = format!(
        "{} [{}] {}",
        decision.id,
        decision_status(decision.status),
        decision.nodes.join(", ")
    );
    line.push_str(&reverse_provenance_lines(
        &decision.refined_by,
        &decision.superseded_by,
        index,
    ));
    line
}

/// Render reverse provenance edges as continuation lines with no trailing
/// newline, so callers can compose the result without blank separators.
pub(crate) fn reverse_provenance_lines(
    refined_by: &[String],
    superseded_by: &[String],
    index: &BTreeMap<String, (String, String)>,
) -> String {
    let mut lines = String::new();
    for (key, ids) in [
        ("decision.refined-by", refined_by),
        ("decision.superseded-by", superseded_by),
    ] {
        for id in ids {
            let (status, date) = index
                .get(id)
                .map_or(("unknown", "unknown"), |(status, date)| {
                    (status.as_str(), date.as_str())
                });
            lines.push('\n');
            lines.push_str(
                &crate::copy::lookup(key)
                    .replace("{id}", id)
                    .replace("{status}", status)
                    .replace("{date}", date),
            );
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artefacts::registry::{DecisionStatus, RatificationTier};

    #[test]
    fn decision_line_format() {
        let decision = Decision {
            path: "./decision.md".to_owned(),
            id: "adopt-rust".to_owned(),
            status: DecisionStatus::Accepted,
            nodes: vec!["app".to_owned(), "lib".to_owned()],
            date: String::new(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            refined_by: Vec::new(),
            superseded_by: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
            ratification: RatificationTier::Binding,
            affects: Vec::new(),
            ratified_by_machine: false,
            receipts: Vec::new(),
        };
        assert_eq!(
            decision_line_with_index(&decision, &BTreeMap::new()),
            "adopt-rust [accepted] app, lib"
        );
    }

    #[test]
    fn reverse_lines_include_proposed_status_and_date() {
        let refined_by = vec!["dec.proposed-refiner".to_owned()];
        let index = BTreeMap::from([(
            "dec.proposed-refiner".to_owned(),
            ("proposed".to_owned(), "2026-08-04".to_owned()),
        )]);

        assert_eq!(
            reverse_provenance_lines(&refined_by, &[], &index),
            "\n  refined by dec.proposed-refiner, proposed 2026-08-04"
        );
    }
}
