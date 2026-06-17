//! Phase 7.8 cairn export command. Renders a single envelope over current
//! map state, edges, artefacts, and active changes, in JSON, Markdown,
//! or Mermaid form.

use std::path::{Path, PathBuf};

use crate::{artefacts::registry::ArtefactType, map::graph::NodeRecord};

mod builder;
mod json;
mod markdown;
mod mermaid;
mod runner;

pub use builder::build_export;
pub(crate) use builder::current_timestamp_rfc3339;
pub use json::render_json;
pub use markdown::render_markdown;
pub use mermaid::render_mermaid;
pub use runner::run;

/// Wire schema version for the export envelope.
pub const SCHEMA_VERSION: u32 = 1;

/// Full project state envelope rendered by `cairn export`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportEnvelope {
    /// Schema version of the envelope.
    pub schema_version: u32,
    /// UTC RFC 3339 timestamp captured at the start of `build_export`.
    pub generated_at: String,
    /// Path to the blueprint file used to build the export.
    pub blueprint_path: PathBuf,
    /// Flattened node records from the current scan.
    pub nodes: Vec<NodeRecord>,
    /// Flat dependency edge list across the graph.
    pub edges: Vec<EdgeEntry>,
    /// Flat artefact list (one entry per direct-typed artefact).
    pub artefacts: Vec<ArtefactEntry>,
    /// Active change directory entries.
    pub changes: Vec<ChangeEntry>,
}

/// One dependency edge (source, target, verb).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeEntry {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Verb describing the relation.
    pub verb: String,
}

/// One artefact in the export, identified by direct type, ID, and path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtefactEntry {
    /// Direct artefact type.
    pub artefact_type: ArtefactType,
    /// Stable artefact ID, or path for path-keyed records.
    pub id: String,
    /// Declared path on disk.
    pub path: String,
    /// Optional node ID the artefact attaches to.
    pub node: Option<String>,
}

/// One active-change summary in the export.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeEntry {
    /// Change ID (directory name).
    pub id: String,
    /// Lifecycle state, currently always `active` in this phase.
    pub state: String,
    /// Proposal title.
    pub title: String,
}

pub(crate) fn blueprint_root(file: &Path) -> &Path {
    file.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_envelope() -> ExportEnvelope {
        ExportEnvelope {
            schema_version: SCHEMA_VERSION,
            generated_at: "2026-05-07T12:00:00Z".to_owned(),
            blueprint_path: PathBuf::from("cairn.blueprint"),
            nodes: Vec::new(),
            edges: Vec::new(),
            artefacts: Vec::new(),
            changes: Vec::new(),
        }
    }

    #[test]
    fn schema_version_is_one() {
        assert_eq!(SCHEMA_VERSION, 1);
    }

    #[test]
    fn json_first_key_is_schema_version() {
        let env = empty_envelope();
        let out = render_json(&env);
        let trimmed = out.trim_start_matches('{').trim_start();
        assert!(trimmed.starts_with("\"schema_version\""));
    }

    #[test]
    fn markdown_has_required_sections_in_order() {
        let env = empty_envelope();
        let out = render_markdown(&env);
        assert!(out.starts_with("# Cairn Export"));
        let nodes_idx = out.find("## Nodes").expect("missing Nodes header");
        let edges_idx = out.find("## Edges").expect("missing Edges header");
        let artefacts_idx = out.find("## Artefacts").expect("missing Artefacts header");
        let changes_idx = out
            .find("## Active Changes")
            .expect("missing Active Changes header");
        assert!(nodes_idx < edges_idx);
        assert!(edges_idx < artefacts_idx);
        assert!(artefacts_idx < changes_idx);
    }

    #[test]
    fn markdown_has_no_em_dashes() {
        let env = empty_envelope();
        let out = render_markdown(&env);
        assert!(!out.contains('\u{2014}'));
    }
}
