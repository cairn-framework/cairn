//! Shared error type for the Cairn crate.

use std::fmt;

/// Top-level error type for Cairn operations.
#[derive(Debug)]
pub enum CairnError {
    /// Verification blocked by an upstream dependency.
    BlockedVerification {
        /// Description of what upstream dependency is missing.
        upstream_cause: String,
    },
    /// `cflx openspec validate --strict` found pending suggested-edge
    /// entries that block archive.
    UntriagedSuggestedEdges {
        /// Change ID whose queue carries pending entries.
        change_id: String,
        /// Number of entries with `triage_state == Pending`.
        pending_count: usize,
        /// Path to the queue file.
        file_path: String,
    },
}

impl fmt::Display for CairnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockedVerification { upstream_cause } => {
                write!(
                    f,
                    "verification blocked by upstream dependency: {upstream_cause}"
                )
            }
            Self::UntriagedSuggestedEdges {
                change_id,
                pending_count,
                file_path,
            } => write!(
                f,
                "change `{change_id}` has {pending_count} untriaged suggested-edge entries in {file_path}; resolve them before --strict validate passes"
            ),
        }
    }
}

impl std::error::Error for CairnError {}

impl CairnError {
    /// Returns the error code for this error.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::BlockedVerification { .. } => "CC001",
            Self::UntriagedSuggestedEdges { .. } => "CC002",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked_verification_code_is_cc001() {
        let err = CairnError::BlockedVerification {
            upstream_cause: "missing upstream phase".to_string(),
        };
        assert_eq!(err.code(), "CC001");
    }

    #[test]
    fn test_untriaged_suggested_edges_code_is_cc002() {
        let err = CairnError::UntriagedSuggestedEdges {
            change_id: "phase-x".to_owned(),
            pending_count: 3,
            file_path: "openspec/changes/phase-x/suggested-edges.json".to_owned(),
        };
        assert_eq!(err.code(), "CC002");
        let msg = format!("{err}");
        assert!(msg.contains("phase-x"));
        assert!(msg.contains('3'));
        assert!(msg.contains("suggested-edges.json"));
    }
}
