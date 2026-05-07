//! Phase 9 Brownfield extraction: typed candidate, confidence, and
//! coupling-score helpers shared across `cairn init --from-code`,
//! `cairn refine`, and the suggest engine. The full extraction pipeline
//! and CLI wiring land in subsequent commits.

mod heuristics;

pub use heuristics::{
    CONFIDENCE_HIGH, CONFIDENCE_MEDIUM, Candidate, CandidateConfidence, DIRECTORY_DEPTH_LIMIT,
    EDGE_OBSERVATION_THRESHOLD, MIN_CANDIDATE_FILE_COUNT, classify_score, coupling_score,
};
