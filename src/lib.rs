//! Foundation metadata for the Cairn crate.
//!
//! Phase 0 intentionally exposes only package metadata. Domain modules for the
//! DSL, ontology, scanner, and archive flows are introduced by later phases.

/// Returns the Cargo package name compiled into this crate.
///
/// The value is static package metadata and cannot fail.
#[must_use]
pub const fn package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Returns the Cargo package version compiled into this crate.
///
/// The value is static package metadata and cannot fail.
#[must_use]
pub const fn package_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the deterministic version label printed by the foundation CLI.
///
/// The label is built from Cargo package metadata and cannot fail.
#[must_use]
pub fn version_label() -> String {
    format!("{} {}", package_name(), package_version())
}

#[cfg(test)]
mod tests {
    use super::{package_name, package_version, version_label};

    #[test]
    fn test_package_name_returns_cairn() {
        assert_eq!(package_name(), "cairn");
    }

    #[test]
    fn test_package_version_is_not_empty() {
        assert!(!package_version().is_empty());
    }

    #[test]
    fn test_version_label_includes_name_and_version() {
        assert_eq!(
            version_label(),
            format!("{} {}", package_name(), package_version())
        );
    }
}
