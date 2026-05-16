//! Compile-time copy lookup from design-system/copy.toml.
//!
//! All user-facing CLI strings live in `docs/design-system/copy.toml` so that
//! copy review, translation, and consistency checks can happen in one place.
//! The file is baked into the binary via `include_str!` and parsed once on
//! first access.

use std::sync::LazyLock;

/// Parsed copy table, initialised on first access.
static COPY: LazyLock<toml::Table> = LazyLock::new(|| {
    include_str!("../../docs/design-system/copy.toml")
        .parse::<toml::Table>()
        .expect("copy.toml must be valid TOML")
});

/// Look up a dotted key path (e.g. `"errors.usage"`).
///
/// Returns the string value if the key resolves to a TOML string.
/// Returns `key` itself as a fallback when the path is missing or
/// points to a non-string value.
pub fn lookup(key: &str) -> &str {
    let mut segments = key.split('.');

    let Some(first) = segments.next() else {
        return key;
    };

    let Some(mut current) = COPY.get(first) else {
        return key;
    };

    for segment in segments {
        match current.get(segment) {
            Some(next) => current = next,
            None => return key,
        }
    }

    current.as_str().unwrap_or(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_existing_key() {
        let value = lookup("errors.usage");
        assert_eq!(value, "usage: cairn <command> [--file path] [--json]");
    }

    #[test]
    fn test_lookup_missing_key_fallback() {
        let key = "this.key.does.not.exist";
        assert_eq!(lookup(key), key);
    }

    #[test]
    fn test_lookup_nested_key() {
        let value = lookup("errors.unknown-command");
        assert_eq!(value, "unknown command");
    }

    #[test]
    fn test_lookup_empty_states() {
        let value = lookup("empty-states.no-findings");
        assert_eq!(value, "No findings to report.");
    }

    #[test]
    fn test_lookup_cli_no_blueprint() {
        let value = lookup("empty-states.cli-no-blueprint");
        assert!(value.starts_with("No cairn.blueprint"));
        assert!(
            value.contains('\n'),
            "multiline: embedded newline preserved"
        );
        assert!(value.contains("cairn init"), "should mention cairn init");
    }

    #[test]
    fn test_lookup_finding_code_table() {
        let value = lookup("findings.codes.CAIRN_RECONCILE_ORPHANED_FILE.heading");
        assert_eq!(value, "Orphaned file");
    }
}
