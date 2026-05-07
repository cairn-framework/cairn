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

/// Confidence threshold above which the heuristic is deemed high.
/// Score = (`high_confidence_refs` + 1) / (`total_refs` + 1).
/// (3+1)/(1+1) = 2.0 (high), (1+1)/(1+1) = 1.0 (medium), (0+1)/(2+1) = 0.33 (low).
pub const CONFIDENCE_HIGH: f64 = 1.5;

/// Confidence threshold above which the heuristic is deemed medium.
pub const CONFIDENCE_MEDIUM: f64 = 0.5;

/// Confidence threshold below which the heuristic is deemed low.
pub const CONFIDENCE_LOW: f64 = 0.5;

/// Confidence bucket for a coupling score.
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

/// Computes the coupling score for a candidate.
///
/// `high_confidence_refs` counts public-API or import references that the
/// scanner classified as high confidence; `total_refs` counts all
/// references including unresolved or test-only ones. The +1 offset
/// avoids division by zero and rewards candidates with corroborating
/// signal even when reference totals are small.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn coupling_score(high_confidence_refs: usize, total_refs: usize) -> f64 {
    let numerator = (high_confidence_refs + 1) as f64;
    let denominator = (total_refs + 1) as f64;
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
