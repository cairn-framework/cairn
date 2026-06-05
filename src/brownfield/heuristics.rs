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
///
/// `id` is path-derived by construction. Cycle 4 fix: previously a
/// public `String` field, now produced via `Candidate::new(directory,
/// file_count, confidence)` which derives the id deterministically
/// from the directory path. Consumers that need a different id source
/// can call `with_id` after construction; that path is explicit so
/// the path-derived invariant is the default.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    id: String,
    directory: String,
    file_count: usize,
    confidence: CandidateConfidence,
}

impl Candidate {
    /// Constructs a candidate from a directory path. The id is derived
    /// deterministically from the directory by replacing path
    /// separators with `.` and stripping leading `./`.
    #[must_use]
    pub fn new(
        directory: impl Into<String>,
        file_count: usize,
        confidence: CandidateConfidence,
    ) -> Self {
        let directory = directory.into();
        let id = path_derived_id(&directory);
        Self {
            id,
            directory,
            file_count,
            confidence,
        }
    }

    /// Overrides the path-derived id. Use only when a project config
    /// has a declared candidate name that differs from the directory.
    #[must_use]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Path-derived candidate identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Directory path from the repo root.
    #[must_use]
    pub fn directory(&self) -> &str {
        &self.directory
    }

    /// Number of source files contributing to the candidate.
    #[must_use]
    pub const fn file_count(&self) -> usize {
        self.file_count
    }

    /// Confidence bucket.
    #[must_use]
    pub const fn confidence(&self) -> CandidateConfidence {
        self.confidence
    }
}

pub(crate) fn path_derived_id(directory: &str) -> String {
    let trimmed = directory.trim_start_matches("./").trim_start_matches('/');
    if trimmed.is_empty() || trimmed == "." {
        return "root".to_owned();
    }
    trimmed.replace(['/', '\\'], ".")
}

/// Computes the coupling score for a candidate per design.md:
/// `(internal_imports + 1) / (external_imports + 1)`.
///
/// `internal_imports` counts imports that resolve inside the candidate
/// directory; `external_imports` counts imports that cross the candidate
/// boundary. The +1 offset avoids division by zero and rewards
/// candidates with internal cohesion even when import totals are small.
#[must_use]
// Reason: usize counts are always small (file/import counts) so f64
// precision loss is bounded and acceptable for a heuristic score.
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

    // Cycle 4: removed three tautological tests
    // (`min_candidate_file_count_is_three`,
    // `directory_depth_limit_is_four`,
    // `edge_observation_threshold_is_two`) that asserted constants
    // against literals (`assert_eq!(CONST, 3)`). Real fixture-driven
    // tests will land alongside the directory-traversal engine in
    // phase-9 task 1.1-1.3. The integration-test counterparts are
    // already `#[cairn_planned(phase = 900)]` per cycle 3 reasoning.

    #[test]
    fn candidate_new_derives_id_from_directory() {
        let c = Candidate::new("src/auth", 4, CandidateConfidence::High);
        assert_eq!(c.id(), "src.auth");
        assert_eq!(c.directory(), "src/auth");
        assert_eq!(c.file_count(), 4);
        assert_eq!(c.confidence(), CandidateConfidence::High);
    }

    #[test]
    fn candidate_new_strips_leading_dot_slash() {
        let c = Candidate::new("./src/api", 5, CandidateConfidence::Medium);
        assert_eq!(c.id(), "src.api");
    }

    #[test]
    fn candidate_with_id_overrides_path_derived() {
        let c = Candidate::new("src/auth", 4, CandidateConfidence::High).with_id("custom-name");
        assert_eq!(c.id(), "custom-name");
        assert_eq!(c.directory(), "src/auth");
    }

    #[test]
    fn path_derived_id_replaces_separators() {
        assert_eq!(path_derived_id("a/b/c"), "a.b.c");
        assert_eq!(path_derived_id("./a/b"), "a.b");
        assert_eq!(path_derived_id("a"), "a");
    }

    #[test]
    fn test_path_derived_id_bare_dot_returns_root() {
        // path_derived_id(".") returned "." because trim_start_matches("./")
        // does not strip a bare dot — only the two-char sequence "." + "/".
        // A bare-dot id renders as `id "."` in blueprint output which is
        // syntactically ambiguous and useless as a node identifier.
        assert_eq!(path_derived_id("."), "root");
    }

    #[test]
    fn test_path_derived_id_empty_string_returns_root() {
        assert_eq!(path_derived_id(""), "root");
    }
}
