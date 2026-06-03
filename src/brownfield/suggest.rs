//! Cross-cutting edge suggester for brownfield extraction.
//!
//! Produces `SuggestedEdgeEntry` values that the deterministic discovery
//! extractor cannot infer (e.g., semantic relationships from shared tags
//! or name similarity).  Every emitted entry carries `triage_state:
//! Pending` so the phase-7.6 `CC002` gate blocks archive until a human
//! reviewer accepts, rejects, or defers the suggestion.

use crate::suggested_edges::{
    EdgeProvenance, SuggestedEdgeEntry, SuggestedEdgesQueue, TriageState,
};

use super::discovery::Extraction;

/// Generate cross-cutting edge suggestions from an extraction result.
///
/// The current heuristic looks for candidate pairs that share at least one
/// tag but do not already have a deterministic sibling edge between them.
/// Each shared tag produces a bidirectional `related_to` suggestion with
/// confidence derived from the lower of the two candidate confidence scores.
///
/// All emitted entries set `triage_state` to `Pending` and populate
/// `provenance` with the supplied phase and stage.
///
/// # Panics
///
/// Never panics.
#[must_use]
pub fn suggest_edges(
    extraction: &Extraction,
    trace_phase: &str,
    stage: &str,
) -> Vec<SuggestedEdgeEntry> {
    let mut entries = Vec::new();
    let _candidate_ids: Vec<&str> = extraction
        .candidates
        .iter()
        .map(|c| c.id.as_str())
        .collect();

    for (i, a) in extraction.candidates.iter().enumerate() {
        for (j, b) in extraction.candidates.iter().enumerate() {
            if i >= j {
                continue;
            }
            // Skip if a deterministic edge already exists.
            if has_deterministic_edge(a, b) {
                continue;
            }
            let shared_tags: Vec<&str> = a
                .tags
                .iter()
                .filter(|t| b.tags.contains(t))
                .map(std::string::String::as_str)
                .collect();
            if shared_tags.is_empty() {
                continue;
            }
            let confidence = a.confidence.min(b.confidence);
            let provenance = EdgeProvenance {
                trace_phase: trace_phase.to_owned(),
                stage: stage.to_owned(),
            };
            entries.push(SuggestedEdgeEntry {
                source: a.id.clone(),
                target: b.id.clone(),
                relation: "related_to".to_owned(),
                triage_state: TriageState::Pending,
                confidence: Some(confidence),
                provenance: Some(provenance.clone()),
                triage_note: None,
            });
            entries.push(SuggestedEdgeEntry {
                source: b.id.clone(),
                target: a.id.clone(),
                relation: "related_to".to_owned(),
                triage_state: TriageState::Pending,
                confidence: Some(confidence),
                provenance: Some(provenance),
                triage_note: None,
            });
        }
    }

    entries
}

/// Write suggested edges for an extraction to the change directory.
///
/// Generates cross-cutting edges and appends them to any existing queue.
/// If no edges are generated, the queue file is left untouched.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when the existing queue cannot be
/// read, and `CairnError::WriteOutput` when the queue cannot be written.
pub fn write_suggested_edges(
    change_dir: &std::path::Path,
    extraction: &Extraction,
    trace_phase: &str,
    stage: &str,
) -> Result<(), crate::error::CairnError> {
    let entries = suggest_edges(extraction, trace_phase, stage);
    if entries.is_empty() {
        return Ok(());
    }

    let path = crate::suggested_edges::queue_path_for_change(change_dir);
    let mut queue = crate::suggested_edges::read_queue(&path)
        .map_err(|e| crate::error::CairnError::ChangeDiscovery {
            path: path.to_string_lossy().into_owned(),
            detail: e.to_string(),
        })?
        .unwrap_or_else(|| SuggestedEdgesQueue {
            version: crate::suggested_edges::SUGGESTED_EDGES_QUEUE_VERSION,
            entries: Vec::new(),
        });

    queue.entries.extend(entries);

    crate::suggested_edges::write_to_change(change_dir, &queue).map_err(|e| {
        crate::error::CairnError::WriteOutput {
            path: change_dir.to_string_lossy().into_owned(),
            detail: e.to_string(),
        }
    })
}

/// Check whether two candidates already have a deterministic edge.
fn has_deterministic_edge(
    a: &super::discovery::DiscoveredCandidate,
    b: &super::discovery::DiscoveredCandidate,
) -> bool {
    a.edges.iter().any(|e| e.target == b.id) || b.edges.iter().any(|e| e.target == a.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brownfield::discovery::{DiscoveredCandidate, DiscoveredEdge, Extraction};

    #[test]
    fn test_suggest_edges_empty_extraction() {
        let extraction = Extraction {
            candidates: vec![],
            schema_version: 1,
        };
        let entries = suggest_edges(&extraction, "phase-9", "propose");
        assert!(entries.is_empty());
    }

    #[test]
    fn test_suggest_edges_skips_existing_edges() {
        let a = DiscoveredCandidate {
            id: "src.a".to_owned(),
            name: "a".to_owned(),
            description: "A".to_owned(),
            path: "src/a".to_owned(),
            tags: vec!["shared".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![DiscoveredEdge {
                target: "src.b".to_owned(),
                description: "sibling".to_owned(),
                confidence: 0.5,
            }],
        };
        let b = DiscoveredCandidate {
            id: "src.b".to_owned(),
            name: "b".to_owned(),
            description: "B".to_owned(),
            path: "src/b".to_owned(),
            tags: vec!["shared".to_owned()],
            confidence: 0.7,
            evidence: vec![],
            edges: vec![],
        };
        let extraction = Extraction {
            candidates: vec![a, b],
            schema_version: 1,
        };
        let entries = suggest_edges(&extraction, "phase-9", "propose");
        assert!(
            entries.is_empty(),
            "should skip candidates with existing deterministic edge"
        );
    }

    #[test]
    fn test_suggest_edges_produces_bidirectional_entries() {
        let a = DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let b = DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let extraction = Extraction {
            candidates: vec![a, b],
            schema_version: 1,
        };
        let entries = suggest_edges(&extraction, "phase-9", "propose");
        assert_eq!(entries.len(), 2);
        assert!(
            entries
                .iter()
                .any(|e| e.source == "src.auth" && e.target == "src.identity")
        );
        assert!(
            entries
                .iter()
                .any(|e| e.source == "src.identity" && e.target == "src.auth")
        );
    }

    fn make_candidate(id: &str, tags: &[&str], edges_to: &[&str]) -> DiscoveredCandidate {
        DiscoveredCandidate {
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            path: id.replace('.', "/"),
            tags: tags.iter().map(ToString::to_string).collect(),
            confidence: 0.9,
            evidence: Vec::new(),
            edges: edges_to
                .iter()
                .map(|t| DiscoveredEdge {
                    target: t.to_string(),
                    description: "dep".to_owned(),
                    confidence: 0.5,
                })
                .collect(),
        }
    }

    fn extraction(candidates: Vec<DiscoveredCandidate>) -> Extraction {
        Extraction {
            candidates,
            schema_version: 1,
        }
    }

    #[test]
    fn test_suggest_edges_skips_when_reverse_edge_exists() {
        // has_deterministic_edge only checked a.edges; it missed b → a.
        // A pair where b already points to a must not generate suggestions.
        let a = make_candidate("src.a", &["shared"], &[]); // no edge from a
        let b = make_candidate("src.b", &["shared"], &["src.a"]); // b → a exists
        let entries = suggest_edges(&extraction(vec![a, b]), "p9", "propose");
        assert!(
            entries.is_empty(),
            "reverse deterministic edge must suppress suggestions, got: {entries:?}"
        );
    }

    #[test]
    fn test_suggest_edges_confidence_uses_minimum_of_pair() {
        let mut a = make_candidate("x.auth", &["sec"], &[]);
        let mut b = make_candidate("x.identity", &["sec"], &[]);
        a.confidence = 0.9;
        b.confidence = 0.6;
        let entries = suggest_edges(&extraction(vec![a, b]), "p9", "propose");
        for e in &entries {
            assert!(
                (e.confidence.unwrap() - 0.6).abs() < 1e-9,
                "confidence must be min(0.9, 0.6) = 0.6, got {:?}",
                e.confidence
            );
        }
    }

    #[test]
    fn test_suggest_edges_multiple_shared_tags_one_entry_per_direction() {
        // Two shared tags must still produce exactly one entry per direction,
        // not one per tag. The suggestion carries the relationship, not each tag.
        let a = make_candidate("x.a", &["alpha", "beta"], &[]);
        let b = make_candidate("x.b", &["alpha", "beta"], &[]);
        let entries = suggest_edges(&extraction(vec![a, b]), "p9", "propose");
        assert_eq!(
            entries.len(),
            2,
            "two shared tags must produce 2 entries (one per direction), got {entries:?}"
        );
    }

    #[test]
    fn test_suggest_edges_provenance_fields_are_set() {
        let a = make_candidate("x.a", &["tag"], &[]);
        let b = make_candidate("x.b", &["tag"], &[]);
        let entries = suggest_edges(&extraction(vec![a, b]), "phase-9", "review");
        for e in &entries {
            let prov = e.provenance.as_ref().expect("provenance must be set");
            assert_eq!(prov.trace_phase, "phase-9");
            assert_eq!(prov.stage, "review");
        }
    }

    #[test]
    fn test_suggest_edges_single_candidate_is_empty() {
        let a = make_candidate("x.a", &["shared"], &[]);
        let entries = suggest_edges(&extraction(vec![a]), "p9", "propose");
        assert!(entries.is_empty(), "single candidate cannot form a pair");
    }

    #[test]
    fn test_suggest_edges_no_shared_tags_is_empty() {
        let a = make_candidate("x.a", &["foo"], &[]);
        let b = make_candidate("x.b", &["bar"], &[]);
        let entries = suggest_edges(&extraction(vec![a, b]), "p9", "propose");
        assert!(
            entries.is_empty(),
            "no shared tags must produce no suggestions"
        );
    }
}
