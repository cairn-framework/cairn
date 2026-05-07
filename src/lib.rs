//! Cairn kernel library.
//!
//! The library exposes the typed parser, map, scanner, and query services
//! used by the CLI. Command wrappers render these responses but do not own the
//! query semantics.

/// Contract artefact loading.
pub mod artefacts;
/// Cairn blueprint parsing.
pub mod blueprint;
/// Change discovery, delta parsing, validation, and archive support.
pub mod changes;
/// CLI command registry and renderer helpers.
pub mod cli;
/// Shared error type.
pub mod error;
/// Hook engine and active-change conflict detection.
pub mod hooks;
/// Map graph construction and queries.
pub mod map;
/// MCP stdio transport.
pub mod mcp;
/// AI provenance: trace sidecar and suggested-edges queue schemas.
pub mod provenance;
/// Shared structured query API and MCP tool registry.
pub mod query_api;
/// Code reconciliation interfaces.
pub mod reconcile;
/// Project scanner orchestration and generated outputs.
pub mod scanner;
/// Phase 8 summariser: pluggable backends and draft store.
pub mod summariser;
/// Embedded graph explorer server and query bridge.
pub mod ui;

/// Verification state types.
pub mod verification;

/// Re-export the `cflx_planned` attribute macro.
pub use cairn_macros::cflx_planned;

/// Returns the Cargo package name compiled into this crate.
#[must_use]
pub const fn package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Returns the Cargo package version compiled into this crate.
#[must_use]
pub const fn package_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the deterministic version label printed by the CLI.
#[must_use]
pub fn version_label() -> String {
    format!("{} {}", package_name(), package_version())
}

#[cfg(test)]
mod tests {
    use super::{cli, package_name, package_version, version_label};

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

    #[test]
    fn test_phase_one_registry_safety_classes() {
        assert_eq!(
            cli::registry()
                .iter()
                .find(|command| command.cli_name == "scan")
                .map(|command| command.safety),
            Some(cli::SafetyClass::Mutating)
        );
        assert!(
            cli::registry()
                .iter()
                .filter(|command| {
                    command.cli_name != "scan"
                        && command.cli_name != "init"
                        && command.cli_name != "archive"
                        && command.cli_name != "rename"
                })
                .all(|command| command.safety == cli::SafetyClass::ReadOnly)
        );
    }
}
