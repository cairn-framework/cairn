//! Markdown renderer for the export envelope.

use std::fmt::Write as _;

use crate::map::graph::NodeRecord;

use super::{
    ArtefactEntry, ChangeEntry, EdgeEntry, ExportEnvelope,
    builder::{artefact_type_label, artefact_type_order},
    json::node_kind_label,
};

/// Renders the envelope as a flattened Markdown document.
///
/// Sections are H1 `# Cairn Export` followed by `## Nodes`, `## Edges`,
/// `## Artefacts`, and `## Active Changes` in that order. The document
/// contains no U+2014 character.
#[must_use]
pub fn render_markdown(envelope: &ExportEnvelope) -> String {
    let mut out = String::new();
    out.push_str("# Cairn Export\n\n");
    let _ = writeln!(out, "Generated: {}", envelope.generated_at);
    let _ = writeln!(
        out,
        "Blueprint: {}\n",
        envelope.blueprint_path.to_string_lossy()
    );
    render_nodes(&mut out, &envelope.nodes);
    render_edges(&mut out, &envelope.edges);
    render_artefacts(&mut out, &envelope.artefacts);
    render_changes(&mut out, &envelope.changes);
    out
}

fn render_nodes(out: &mut String, nodes: &[NodeRecord]) {
    out.push_str("## Nodes\n\n");
    if nodes.is_empty() {
        out.push_str("(none)\n\n");
        return;
    }
    let mut groups: std::collections::BTreeMap<String, Vec<&NodeRecord>> =
        std::collections::BTreeMap::new();
    for node in nodes {
        let key = node
            .parent
            .clone()
            .unwrap_or_else(|| "(top level)".to_owned());
        groups.entry(key).or_default().push(node);
    }
    for (parent, group) in groups {
        let _ = writeln!(out, "### {parent}\n");
        for node in group {
            let _ = writeln!(
                out,
                "- `{}` ({}): {}",
                node.id,
                node_kind_label(node.kind),
                strip_em_dashes(&node.name)
            );
        }
        out.push('\n');
    }
}

fn render_edges(out: &mut String, edges: &[EdgeEntry]) {
    out.push_str("## Edges\n\n");
    if edges.is_empty() {
        out.push_str("(none)\n\n");
        return;
    }
    for e in edges {
        let _ = writeln!(
            out,
            "- `{}` {} `{}`",
            e.from,
            strip_em_dashes(&e.verb),
            e.to
        );
    }
    out.push('\n');
}

fn render_artefacts(out: &mut String, artefacts: &[ArtefactEntry]) {
    out.push_str("## Artefacts\n\n");
    if artefacts.is_empty() {
        out.push_str("(none)\n\n");
        return;
    }
    let mut groups: std::collections::BTreeMap<u8, Vec<&ArtefactEntry>> =
        std::collections::BTreeMap::new();
    for a in artefacts {
        groups
            .entry(artefact_type_order(a.artefact_type))
            .or_default()
            .push(a);
    }
    for (_, group) in groups {
        if group.is_empty() {
            continue;
        }
        let label = artefact_type_label(group[0].artefact_type);
        let _ = writeln!(out, "### {label}\n");
        for a in group {
            let _ = writeln!(out, "- `{}` ({})", a.id, a.path);
        }
        out.push('\n');
    }
}

fn render_changes(out: &mut String, changes_in: &[ChangeEntry]) {
    out.push_str("## Active Changes\n\n");
    if changes_in.is_empty() {
        out.push_str("(none)\n\n");
        return;
    }
    for c in changes_in {
        let _ = writeln!(
            out,
            "- `{}` ({}): {}",
            c.id,
            c.state,
            strip_em_dashes(&c.title)
        );
    }
    out.push('\n');
}

fn strip_em_dashes(s: &str) -> String {
    s.replace('\u{2014}', ", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_em_dashes_replaces_with_comma_space() {
        assert_eq!(
            strip_em_dashes("foo \u{2014} bar"),
            "foo , space-bar".replace("space-bar", " bar")
        );
    }

    #[test]
    fn strip_em_dashes_passes_through_no_em() {
        assert_eq!(strip_em_dashes("plain text"), "plain text");
    }
}
