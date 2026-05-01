//! Verification state types for test classification.

use serde::{Deserialize, Serialize};

/// The lifecycle state of a verification test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationState {
    /// Verification authored but not yet wired to the battery.
    Draft,
    /// Verification scoped to a future phase, deliberately skipped now.
    Planned,
    /// Test ran and asserted true.
    Passed,
    /// Test ran and asserted false or panicked.
    Failed,
    /// Test could not execute because of an upstream missing piece.
    Blocked,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_state_serde_roundtrip() {
        let variants = vec![
            VerificationState::Draft,
            VerificationState::Planned,
            VerificationState::Passed,
            VerificationState::Failed,
            VerificationState::Blocked,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).expect("serialization failed");
            let deserialized: VerificationState =
                serde_json::from_str(&json).expect("deserialization failed");
            assert_eq!(variant, deserialized, "round-trip failed for {variant:?}");
        }
    }
}
