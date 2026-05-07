//! Brownfield candidate heuristics: directory traversal thresholds,
//! coupling score, and confidence buckets.

/// Minimum source-file count for a directory to become a candidate.
pub const MIN_CANDIDATE_FILE_COUNT: usize = 3;

/// Maximum directory depth below repo root that emits candidates without
/// explicit project config.
pub const DIRECTORY_DEPTH_LIMIT: usize = 4;

/// Minimum import observations to emit a cross-cutting edge between two
/// candidates.
pub const EDGE_OBSERVATION_THRESHOLD: usize = 2;

/// Confidence threshold at or above which the heuristic is deemed high.
/// Score = (`internal_imports` + 1) / (`external_imports` + 1).
/// (3+1)/(1+1) = 2.0 (high), (1+1)/(1+1) = 1.0 (medium), (0+1)/(2+1) = 0.33 (low).
pub const CONFIDENCE_HIGH: f64 = 2.0;

/// Confidence threshold at or above which the heuristic is deemed medium.
pub const CONFIDENCE_MEDIUM: f64 = 1.0;

/// Confidence bucket for a coupling score.
///
/// In-process classification only. The phase-7.6 suggested-edges
/// queue (`suggested-edges.json`) carries the raw `f64` score on
/// `SuggestedEdgeEntry::confidence`, not this enum, so no `serde`
/// derives are required.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandidateConfidence {
    /// High-confidence candidate.
    High,
    /// Medium-confidence candidate.
    Medium,
    /// Low-confidence candidate.
    Low,
}

/// Brownfield candidate emitted from directory scanning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    /// Path-derived candidate identifier.
    pub id: String,
    /// Directory path from the repo root.
    pub directory: String,
    /// Number of source files contributing to the candidate.
    pub file_count: usize,
    /// Confidence bucket.
    pub confidence: CandidateConfidence,
}

/// Computes the coupling score for a candidate per design.md:
/// `(internal_imports + 1) / (external_imports + 1)`.
///
/// `internal_imports` counts imports that resolve inside the candidate
/// directory; `external_imports` counts imports that cross the candidate
/// boundary. The +1 offset avoids division by zero and rewards
/// candidates with internal cohesion even when import totals are small.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn coupling_score(internal_imports: usize, external_imports: usize) -> f64 {
    let numerator = (internal_imports + 1) as f64;
    let denominator = (external_imports + 1) as f64;
    numerator / denominator
}

/// Maps a coupling score to a confidence bucket.
#[must_use]
pub fn classify_score(score: f64) -> CandidateConfidence {
    if score >= CONFIDENCE_HIGH {
        CandidateConfidence::High
    } else if score >= CONFIDENCE_MEDIUM {
        CandidateConfidence::Medium
    } else {
        CandidateConfidence::Low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coupling_score_high_confidence() {
        // (3+1)/(1+1) = 2.0
        let score = coupling_score(3, 1);
        assert!((score - 2.0).abs() < f64::EPSILON);
        assert_eq!(classify_score(score), CandidateConfidence::High);
    }

    #[test]
    fn coupling_score_medium_confidence() {
        // (1+1)/(1+1) = 1.0
        let score = coupling_score(1, 1);
        assert!((score - 1.0).abs() < f64::EPSILON);
        assert_eq!(classify_score(score), CandidateConfidence::Medium);
    }

    #[test]
    fn coupling_score_low_confidence() {
        // (0+1)/(2+1) = 0.333...
        let score = coupling_score(0, 2);
        assert!(score < CONFIDENCE_MEDIUM);
        assert_eq!(classify_score(score), CandidateConfidence::Low);
    }

    #[test]
    fn classify_boundary_cases() {
        // Score exactly at the High threshold buckets as High (>= 2.0).
        assert_eq!(classify_score(2.0), CandidateConfidence::High);
        // Score exactly at the Medium threshold buckets as Medium (>= 1.0).
        assert_eq!(classify_score(1.0), CandidateConfidence::Medium);
        // Just below Medium buckets as Low.
        assert_eq!(classify_score(0.99), CandidateConfidence::Low);
        // Just below High buckets as Medium.
        assert_eq!(classify_score(1.99), CandidateConfidence::Medium);
    }

    #[test]
    fn min_candidate_file_count_is_three() {
        assert_eq!(MIN_CANDIDATE_FILE_COUNT, 3);
    }

    #[test]
    fn directory_depth_limit_is_four() {
        assert_eq!(DIRECTORY_DEPTH_LIMIT, 4);
    }

    #[test]
    fn edge_observation_threshold_is_two() {
        assert_eq!(EDGE_OBSERVATION_THRESHOLD, 2);
    }
}
