//! Deterministic interface fingerprints.
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
/// Interface hash value.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InterfaceFingerprint {
    /// Stable hexadecimal hash.
    pub hash: String,
}
impl InterfaceFingerprint {
    /// Computes a deterministic hash for sorted symbols.
    #[must_use]
    pub fn from_symbols(symbols: &[String]) -> Self {
        let mut sorted = symbols.to_vec();
        sorted.sort_unstable();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_symbols_is_deterministic() {
        let fp1 = InterfaceFingerprint::from_symbols(&["b".to_owned(), "a".to_owned()]);
        let fp2 = InterfaceFingerprint::from_symbols(&["b".to_owned(), "a".to_owned()]);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn from_symbols_sorts_before_hashing() {
        let fp1 = InterfaceFingerprint::from_symbols(&["b".to_owned(), "a".to_owned()]);
        let fp2 = InterfaceFingerprint::from_symbols(&["a".to_owned(), "b".to_owned()]);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn from_symbols_matches_from_sorted_with_pre_sorted_input() {
        let mut symbols = vec!["b".to_owned(), "a".to_owned(), "c".to_owned()];
        symbols.sort_unstable();
        let fp1 =
            InterfaceFingerprint::from_symbols(&["b".to_owned(), "a".to_owned(), "c".to_owned()]);
        let fp2 = InterfaceFingerprint::from_sorted(&symbols);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn different_symbols_produce_different_hashes() {
        let fp1 = InterfaceFingerprint::from_symbols(&["a".to_owned()]);
        let fp2 = InterfaceFingerprint::from_symbols(&["b".to_owned()]);
        assert_ne!(fp1.hash, fp2.hash);
    }

    #[test]
    fn empty_symbols_produces_sixteen_char_hash() {
        let fp = InterfaceFingerprint::from_symbols(&[]);
        assert_eq!(fp.hash.len(), 16);
        assert!(fp.hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_format_is_lower_hex() {
        let fp = InterfaceFingerprint::from_symbols(&["x".to_owned()]);
        assert!(fp.hash.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(fp.hash.chars().all(|c| !c.is_ascii_uppercase()));
    }
}
