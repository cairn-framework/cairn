//! Mermaid diagram renderer for the export envelope.
//!
//! Produces a `graph TD` diagram from the node and edge lists in the
//! envelope. Node IDs are quoted because they contain dots.

use std::fmt::Write as _;

use crate::blueprint::NodeKind;

use super::ExportEnvelope;

fn kind_label(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::System => "system",
        NodeKind::Container => "container",
        NodeKind::Module => "module",
        NodeKind::Actor => "actor",
    }
}

/// Renders the envelope as a Mermaid `graph TD` diagram.
///
/// Each node becomes a labelled rectangle. Each edge becomes a `-->` arrow.
/// The output is valid Mermaid syntax suitable for embedding in markdown
/// fences or piping to a Mermaid CLI renderer.
#[must_use]
pub fn render_mermaid(envelope: &ExportEnvelope) -> String {
    let mut out = String::from("graph TD\n");

    for node in &envelope.nodes {
        // Quote the ID (dots are not valid unquoted Mermaid identifiers).
        let label = if node.name.is_empty() || node.name == node.id {
            node.id.clone()
        } else {
            format!("{} ({})", node.name, kind_label(node.kind))
        };
        writeln!(out, "    [\"{}\"][\"{label}\"]", node.id).unwrap();
    }

    for edge in &envelope.edges {
        writeln!(out, "    [\"{}\"] --> [\"{}\"]", edge.from, edge.to).unwrap();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::export::{EdgeEntry, ExportEnvelope, SCHEMA_VERSION};
    use std::path::PathBuf;

    fn empty() -> ExportEnvelope {
        ExportEnvelope {
            schema_version: SCHEMA_VERSION,
            generated_at: "2026-01-01T00:00:00Z".to_owned(),
            blueprint_path: PathBuf::from("cairn.blueprint"),
            nodes: Vec::new(),
            edges: Vec::new(),
            artefacts: Vec::new(),
            changes: Vec::new(),
        }
    }

    #[test]
    fn empty_envelope_produces_valid_header() {
        let out = render_mermaid(&empty());
        assert_eq!(out.trim_end(), "graph TD");
    }

    #[test]
    fn edge_produces_arrow_line() {
        let mut env = empty();
        env.edges.push(EdgeEntry {
            from: "a".to_owned(),
            to: "b".to_owned(),
            verb: "calls".to_owned(),
        });
        let out = render_mermaid(&env);
        assert!(out.contains("-->"), "must contain arrow");
        assert!(out.contains("\"a\""), "must quote from");
        assert!(out.contains("\"b\""), "must quote to");
    }
}
