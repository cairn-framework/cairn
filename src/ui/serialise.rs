//! JSON escaping, percent-decoding, and stable wire names for UI responses.
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
            if let (Some(hi), Some(lo)) = (hi, lo)
                && let (Some(hi), Some(lo)) = (hi.to_digit(16), lo.to_digit(16))
            {
                let Ok(byte) = u8::try_from(hi * 16 + lo) else {
                    output.push(ch);
                    continue;
                };
                output.push(char::from(byte));
                continue;
            }
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
