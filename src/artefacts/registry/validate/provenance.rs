//! Decision provenance validation for computed reverse edges.
//!
//! Reverse edges are computed from loaded forward references. This module
//! reports the advisory that keeps accepted refinements visible to readers of
//! an accepted decision.

use super::super::{ArtefactSet, DecisionStatus};
use super::io::info;
use crate::artefacts::registry::dates::date_to_days;

/// Reports accepted decisions that have accepted refining decisions.
///
/// The reverse edge's refiner ID is the finding target so separate refiners
/// sharing one decision file remain distinct to scanner deduplication.
pub(super) fn validate_refined_authority(set: &mut ArtefactSet) {
    let decisions = set
        .decisions
        .iter()
        .map(|decision| (decision.id.as_str(), decision))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut findings = Vec::new();
    for decision in &set.decisions {
        if decision.status != DecisionStatus::Accepted {
            continue;
        }
        for refiner_id in &decision.refined_by {
            let Some(refiner) = decisions.get(refiner_id.as_str()) else {
                continue;
            };
            if refiner.status != DecisionStatus::Accepted
                || date_to_days(&refiner.date).is_none_or(|refiner_date| {
                    date_to_days(&decision.date)
                        .is_none_or(|decision_date| refiner_date <= decision_date)
                })
                || !decision
                    .nodes
                    .iter()
                    .any(|node| refiner.nodes.iter().any(|candidate| candidate == node))
            {
                continue;
            }
            let message =
                crate::copy::lookup("findings.codes.CAIRN_DECISION_REFINED_AUTHORITY.body")
                    .replace("{node}", &decision.id)
                    .replace("{target}", &refiner.id);
            let mut finding = info(
                "CAIRN_DECISION_REFINED_AUTHORITY",
                message,
                None,
                Some(decision.path.clone()),
            );
            finding.target = Some(refiner.id.clone());
            findings.push(finding);
        }
    }
    set.findings.extend(findings);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artefacts::registry::{Decision, RatificationTier};
    use crate::map::FindingSeverity;

    fn decision(id: &str, status: DecisionStatus) -> Decision {
        Decision {
            id: id.to_owned(),
            path: format!("meta/decisions/{id}.md"),
            nodes: vec!["app.core".to_owned()],
            status,
            ratification: RatificationTier::Binding,
            affects: Vec::new(),
            ratified_by_machine: false,
            receipts: Vec::new(),
            date: "2026-08-02".to_owned(),
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
        }
    }

    #[test]
    fn accepted_newer_overlapping_refinement_emits_info_targeted_to_refiner() {
        let mut base = decision("dec.base", DecisionStatus::Accepted);
        base.refined_by = vec!["dec.refiner".to_owned()];
        let mut refiner = decision("dec.refiner", DecisionStatus::Accepted);
        refiner.date = "2026-08-03".to_owned();
        let mut set = ArtefactSet {
            decisions: vec![base, refiner],
            ..ArtefactSet::default()
        };

        validate_refined_authority(&mut set);

        assert_eq!(set.findings.len(), 1);
        let finding = &set.findings[0];
        assert_eq!(finding.code, "CAIRN_DECISION_REFINED_AUTHORITY");
        assert_eq!(finding.severity, FindingSeverity::Info);
        assert_eq!(finding.target.as_deref(), Some("dec.refiner"));
        assert!(finding.message.contains("dec.base"));
        assert!(finding.message.contains("dec.refiner"));
    }

    #[test]
    fn proposed_refinement_is_silent() {
        let mut base = decision("dec.base", DecisionStatus::Accepted);
        base.refined_by = vec!["dec.refiner".to_owned()];
        let mut refiner = decision("dec.refiner", DecisionStatus::Proposed);
        refiner.date = "2026-08-03".to_owned();
        let mut set = ArtefactSet {
            decisions: vec![base, refiner],
            ..ArtefactSet::default()
        };

        validate_refined_authority(&mut set);

        assert!(set.findings.is_empty());
    }

    #[test]
    fn disjoint_refinement_is_silent() {
        let mut base = decision("dec.base", DecisionStatus::Accepted);
        base.refined_by = vec!["dec.refiner".to_owned()];
        let mut refiner = decision("dec.refiner", DecisionStatus::Accepted);
        refiner.date = "2026-08-03".to_owned();
        refiner.nodes = vec!["app.other".to_owned()];
        let mut set = ArtefactSet {
            decisions: vec![base, refiner],
            ..ArtefactSet::default()
        };

        validate_refined_authority(&mut set);

        assert!(set.findings.is_empty());
    }

    #[test]
    fn same_or_older_refinement_is_silent() {
        let mut base = decision("dec.base", DecisionStatus::Accepted);
        base.refined_by = vec!["dec.refiner".to_owned()];
        let refiner = decision("dec.refiner", DecisionStatus::Accepted);
        let mut set = ArtefactSet {
            decisions: vec![base, refiner],
            ..ArtefactSet::default()
        };

        validate_refined_authority(&mut set);

        assert!(set.findings.is_empty());
    }

    #[test]
    fn non_canonical_older_refinement_is_silent() {
        let mut base = decision("dec.base", DecisionStatus::Accepted);
        base.date = "2026-10-01".to_owned();
        base.refined_by = vec!["dec.refiner".to_owned()];
        let mut refiner = decision("dec.refiner", DecisionStatus::Accepted);
        refiner.date = "2026-9-30".to_owned();
        let mut set = ArtefactSet {
            decisions: vec![base, refiner],
            ..ArtefactSet::default()
        };

        validate_refined_authority(&mut set);

        assert!(set.findings.is_empty());
    }

    #[test]
    fn invalid_refinement_date_is_silent() {
        let mut base = decision("dec.base", DecisionStatus::Accepted);
        base.refined_by = vec!["dec.refiner".to_owned()];
        let mut refiner = decision("dec.refiner", DecisionStatus::Accepted);
        refiner.date = "not-a-date".to_owned();
        let mut set = ArtefactSet {
            decisions: vec![base, refiner],
            ..ArtefactSet::default()
        };

        validate_refined_authority(&mut set);

        assert!(set.findings.is_empty());
    }
}
