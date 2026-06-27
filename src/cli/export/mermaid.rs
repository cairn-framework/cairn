//! Mermaid diagram renderer for the export envelope.
//!
//! Produces a `graph TD` diagram from the node and edge lists in the
//! envelope. Node IDs containing dots or hyphens are sanitized to
//! underscores so they are valid unquoted Mermaid identifiers; the
//! original ID is preserved in the node label.

use std::fmt::Write as _;

use crate::blueprint::NodeKind;

use super::ExportEnvelope;

/// Sanitizes a CAIRN node ID (e.g. `cairn.kernel.map`) into a valid
/// Mermaid identifier (e.g. `cairn_kernel_map`) by replacing `.` and
/// `-` with `_`. All other characters are left as-is.
pub(crate) fn mermaid_id(id: &str) -> String {
    id.replace(['.', '-'], "_")
}

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
/// Each node becomes a labelled rectangle using a sanitized Mermaid ID
/// with the original dotted ID in the label. Each edge becomes a `-->`
/// arrow between sanitized IDs. The output is valid Mermaid syntax
/// suitable for embedding in markdown fences or piping to mermaid-cli.
#[must_use]
pub fn render_mermaid(envelope: &ExportEnvelope) -> String {
    let mut out = String::from("graph TD\n");

    for node in &envelope.nodes {
        let mid = mermaid_id(&node.id);
        let label = if node.name.is_empty() || node.name == node.id {
            format!("{} ({})", node.id, kind_label(node.kind))
        } else {
            format!("{} ({}, {})", node.name, node.id, kind_label(node.kind))
        };
        writeln!(out, "    {mid}[\"{label}\"]").unwrap();
    }

    for edge in &envelope.edges {
        let from = mermaid_id(&edge.from);
        let to = mermaid_id(&edge.to);
        writeln!(out, "    {from} --> {to}").unwrap();
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
    fn mermaid_id_sanitizes_dots_and_hyphens() {
        assert_eq!(mermaid_id("cairn.kernel.map"), "cairn_kernel_map");
        assert_eq!(mermaid_id("my-node"), "my_node");
        assert_eq!(mermaid_id("plain"), "plain");
    }

    #[test]
    fn edge_produces_arrow_line_with_sanitized_ids() {
        let mut env = empty();
        env.edges.push(EdgeEntry {
            from: "a.b".to_owned(),
            to: "c.d".to_owned(),
            verb: "calls".to_owned(),
        });
        let out = render_mermaid(&env);
        assert!(out.contains("-->"), "must contain arrow");
        assert!(out.contains("a_b"), "from id must be sanitized");
        assert!(out.contains("c_d"), "to id must be sanitized");
    }
}
