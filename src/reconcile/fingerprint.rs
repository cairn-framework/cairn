//! Deterministic interface fingerprints.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Interface hash value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceFingerprint {
    /// Stable hexadecimal hash.
    pub hash: String,
}

impl InterfaceFingerprint {
    /// Computes a deterministic hash for sorted symbols.
    #[must_use]
    pub fn from_symbols(symbols: &[String]) -> Self {
        let mut sorted = symbols.to_vec();
        sorted.sort();
        Self::from_sorted(&sorted)
    }
    /// Computes a deterministic hash for an already-sorted symbol slice.
    #[must_use]
    pub fn from_sorted(symbols: &[String]) -> Self {
        let mut hasher = DefaultHasher::new();
        symbols.hash(&mut hasher);
        Self {
            hash: format!("{:016x}", hasher.finish()),
        }
    }
}
