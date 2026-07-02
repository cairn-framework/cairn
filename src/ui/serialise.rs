//! UI response serialisation helpers.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

pub(super) const fn kind_name(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "system",
        NodeKind::Container => "container",
        NodeKind::Module => "module",
        NodeKind::Actor => "actor",
    }
}

/// Stable lowercase wire name for a reconciliation state.
pub(super) const fn state_name(state: crate::map::NodeState) -> &'static str {
    use crate::map::NodeState;
    match state {
        NodeState::Synced => "synced",
        NodeState::Ghost => "ghost",
        NodeState::Orphaned => "orphaned",
    }
}

pub(super) const fn severity_name(severity: FindingSeverity) -> &'static str {
    match severity {
        FindingSeverity::Error => "error",
        FindingSeverity::Warning => "warning",
        FindingSeverity::Info => "info",
    }
}

/// Stable lowercase wire name for a symbol kind, matching `SymbolKind`'s
/// `#[serde(rename_all = "lowercase")]` representation.
pub(super) const fn symbol_kind_name(kind: crate::reconcile::SymbolKind) -> &'static str {
    use crate::reconcile::SymbolKind;
    match kind {
        SymbolKind::Function => "function",
        SymbolKind::Struct => "struct",
        SymbolKind::Class => "class",
        SymbolKind::Enum => "enum",
        SymbolKind::Trait => "trait",
        SymbolKind::Interface => "interface",
        SymbolKind::Type => "type",
        SymbolKind::Const => "const",
        SymbolKind::Static => "static",
        SymbolKind::Module => "module",
        SymbolKind::Union => "union",
        SymbolKind::Variable => "variable",
        SymbolKind::Other => "other",
    }
}

pub(super) const fn graph_edge_kind_name(kind: GraphEdgeKind) -> &'static str {
    match kind {
        GraphEdgeKind::Ownership => "ownership",
        GraphEdgeKind::Dependency => "dependency",
    }
}

pub(super) fn map_json(values: &BTreeMap<String, String>) -> String {
    let fields = values
        .iter()
        .map(|(key, value)| format!("\"{}\":\"{}\"", esc(key), esc(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{fields}}}")
}

pub(super) fn string_array_json(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", esc(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(super) fn optional_json(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_owned(), |text| format!("\"{}\"", esc(text)))
}

pub(super) fn percent_decode(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hi = chars.next();
            let lo = chars.next();
            let decoded = hi.zip(lo).and_then(|(h, l)| {
                h.to_digit(16)
                    .zip(l.to_digit(16))
                    .map(|(hv, lv)| (hv, lv, h, l))
            });
            if let Some((hi_val, lo_val, _, _)) = decoded
                && let Ok(byte) = u8::try_from(hi_val * 16 + lo_val)
            {
                output.push(char::from(byte));
                continue;
            }
            // Fallback: preserve `%` and any consumed characters as-is.
            output.push('%');
            if let Some((_, _, h, l)) = decoded {
                output.push(h);
                output.push(l);
            } else {
                // One or both hex chars missing (truncated input).
                if let Some(h) = hi {
                    output.push(h);
                }
                if let Some(l) = lo {
                    output.push(l);
                }
            }
            continue;
        }
        output.push(ch);
    }
    output.replace('+', " ")
}

pub(super) fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
#[cfg(test)]
mod tests {
    use super::*;

    // ── percent_decode ────────────────────────────────────────────────────────

    #[test]
    fn test_percent_decode_valid_sequence_decoded() {
        assert_eq!(percent_decode("%20"), " ");
    }

    #[test]
    fn test_percent_decode_plus_becomes_space() {
        assert_eq!(percent_decode("hello+world"), "hello world");
    }

    #[test]
    fn test_percent_decode_empty_string() {
        assert_eq!(percent_decode(""), "");
    }

    #[test]
    fn test_percent_decode_no_encoding_unchanged() {
        assert_eq!(percent_decode("app.api"), "app.api");
    }

    #[test]
    fn test_percent_decode_mixed_valid_and_plain() {
        assert_eq!(percent_decode("%41pp%2eapi"), "App.api");
    }

    /// An invalid hex pair must NOT silently drop the characters after `%`.
    /// Browser / RFC 3986 §2.1 behaviour: treat `%` followed by non-hex as
    /// literal `%` + the two characters unchanged.
    #[test]
    fn test_percent_decode_invalid_hex_pair_preserves_both_chars() {
        // Bug: current impl eats 'G' and 'H' because the if-let chain consumes
        // them from the iterator then falls through without pushing them.
        assert_eq!(percent_decode("%GHtest"), "%GHtest");
    }

    #[test]
    fn test_percent_decode_truncated_percent_at_end_preserved() {
        assert_eq!(percent_decode("end%"), "end%");
    }

    #[test]
    fn test_percent_decode_truncated_percent_one_char_preserved() {
        // `%G` at end — the `G` must not be silently dropped.
        assert_eq!(percent_decode("end%G"), "end%G");
    }
}
