//! Component-boundary path containment, shared by the reconciler's owner
//! attribution and the wave composer's disjointness test, so path
//! containment has exactly one implementation in the codebase
//! (`res.parallel-dispatch-rung-3` Part 3).

/// Strips a leading `./` from a declared path.
pub(crate) fn trim_dot(path: &str) -> String {
    path.trim_start_matches("./").to_owned()
}

/// True when `prefix` contains `path` at a component boundary: equality, a
/// `prefix/`-delimited ancestor, or the universal prefix (empty or `.`).
/// Prefixes carry no trailing slash: a stored `docs/registries/` would
/// inspect the first byte of the filename and always fail.
pub(crate) fn is_component_prefix(prefix: &str, path: &str) -> bool {
    prefix.is_empty()
        || prefix == "."
        || path == prefix
        || (path.starts_with(prefix) && path.as_bytes().get(prefix.len()) == Some(&b'/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn containment_stops_at_component_boundaries() {
        assert!(is_component_prefix("src/ui", "src/ui"));
        assert!(is_component_prefix("src/ui", "src/ui/app.js"));
        assert!(
            !is_component_prefix("src/ui", "src/ui_assets"),
            "src/ui must not match src/ui_assets"
        );
        assert!(!is_component_prefix("src/ui", "src/ui_assets/app.js"));
        assert!(is_component_prefix(".", "anything"));
        assert!(is_component_prefix("", "anything"));
    }

    #[test]
    fn a_trailing_slash_prefix_would_false_negative_and_trim_dot_normalises() {
        // The stored form must carry no trailing slash: with one, the
        // check inspects the first byte of the filename and fails even on
        // a genuine child.
        assert!(!is_component_prefix(
            "docs/registries/",
            "docs/registries/error-codes.md"
        ));
        assert!(is_component_prefix(
            "docs/registries",
            "docs/registries/error-codes.md"
        ));
        assert_eq!(trim_dot("./docs/registries"), "docs/registries");
        assert_eq!(trim_dot("src/ui"), "src/ui");
    }
}
