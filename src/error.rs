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
        }
    }
}

impl std::error::Error for CairnError {}

impl CairnError {
    /// Returns the error code for this error.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::BlockedVerification { .. } => "CC001",
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
}
