//! JSON renderer for the export envelope.

use std::fmt::Write as _;

use crate::{
    blueprint::NodeKind,
    map::graph::{NodeRecord, NodeState},
};

use super::{ArtefactEntry, ChangeEntry, EdgeEntry, ExportEnvelope, builder::artefact_type_label};

/// Renders the envelope as pretty-printed JSON.
///
/// `schema_version` always appears as the first key.
#[must_use]
pub fn render_json(envelope: &ExportEnvelope) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    let _ = writeln!(out, "  \"schema_version\": {},", envelope.schema_version);
    let _ = writeln!(
        out,
        "  \"generated_at\": \"{}\",",
        json_escape(&envelope.generated_at)
    );
    let _ = writeln!(
        out,
        "  \"blueprint_path\": \"{}\",",
        json_escape(&envelope.blueprint_path.to_string_lossy())
    );
    out.push_str("  \"nodes\": [");
    render_nodes(&mut out, &envelope.nodes);
    out.push_str("],\n");
    out.push_str("  \"edges\": [");
    render_edges(&mut out, &envelope.edges);
    out.push_str("],\n");
    out.push_str("  \"artefacts\": [");
    render_artefacts(&mut out, &envelope.artefacts);
    out.push_str("],\n");
    out.push_str("  \"changes\": [");
    render_changes(&mut out, &envelope.changes);
    out.push_str("]\n");
    out.push_str("}\n");
    out
}

fn render_nodes(out: &mut String, nodes: &[NodeRecord]) {
    if nodes.is_empty() {
        return;
    }
    out.push('\n');
    for (idx, node) in nodes.iter().enumerate() {
        let comma = if idx + 1 == nodes.len() { "" } else { "," };
        let _ = writeln!(
            out,
            "    {{\"id\": \"{}\", \"name\": \"{}\", \"kind\": \"{}\", \"state\": \"{}\", \"description\": \"{}\"}}{}",
            json_escape(&node.id),
            json_escape(&node.name),
            node_kind_label(node.kind),
            node_state_label(node.state),
            json_escape(&node.description),
            comma
        );
    }
    out.push_str("  ");
}

fn render_edges(out: &mut String, edges: &[EdgeEntry]) {
    if edges.is_empty() {
        return;
    }
    out.push('\n');
    for (idx, e) in edges.iter().enumerate() {
        let comma = if idx + 1 == edges.len() { "" } else { "," };
        let _ = writeln!(
            out,
            "    {{\"from\": \"{}\", \"to\": \"{}\", \"verb\": \"{}\"}}{}",
            json_escape(&e.from),
            json_escape(&e.to),
            json_escape(&e.verb),
            comma
        );
    }
    out.push_str("  ");
}

fn render_artefacts(out: &mut String, artefacts: &[ArtefactEntry]) {
    if artefacts.is_empty() {
        return;
    }
    out.push('\n');
    for (idx, a) in artefacts.iter().enumerate() {
        let comma = if idx + 1 == artefacts.len() { "" } else { "," };
        let node_field = a
            .node
            .as_ref()
            .map_or_else(|| "null".to_owned(), |n| format!("\"{}\"", json_escape(n)));
        let _ = writeln!(
            out,
            "    {{\"type\": \"{}\", \"id\": \"{}\", \"path\": \"{}\", \"node\": {}}}{}",
            artefact_type_label(a.artefact_type),
            json_escape(&a.id),
            json_escape(&a.path),
            node_field,
            comma
        );
    }
    out.push_str("  ");
}

fn render_changes(out: &mut String, changes_in: &[ChangeEntry]) {
    if changes_in.is_empty() {
        return;
    }
    out.push('\n');
    for (idx, c) in changes_in.iter().enumerate() {
        let comma = if idx + 1 == changes_in.len() { "" } else { "," };
        let _ = writeln!(
            out,
            "    {{\"id\": \"{}\", \"state\": \"{}\", \"title\": \"{}\"}}{}",
            json_escape(&c.id),
            json_escape(&c.state),
            json_escape(&c.title),
            comma
        );
    }
    out.push_str("  ");
}

pub(super) fn node_kind_label(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "system",
        NodeKind::Container => "container",
        NodeKind::Module => "module",
        NodeKind::Actor => "actor",
    }
}

fn node_state_label(state: NodeState) -> &'static str {
    match state {
        NodeState::Synced => "synced",
        NodeState::Ghost => "ghost",
        NodeState::Orphaned => "orphaned",
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_escape_handles_specials() {
        assert_eq!(json_escape("a\"b\\c\nd"), "a\\\"b\\\\c\\nd");
    }

    #[test]
    fn json_escape_handles_low_control_chars() {
        // U+0001 should render as 
        assert_eq!(json_escape("a\x01b"), "a\\u0001b");
    }
}
