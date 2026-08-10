//! Deterministic decision-evidence index behind `cairn onboard decisions`.
//!
//! Indexes the closed evidence set named by `dec.brownfield-extraction-mechanism`
//! clause 1: files under `docs/adr/` and `docs/decisions/`, README sections
//! headed Decision, Rationale, or Invariant, source comments carrying the
//! literal `// invariant:` or `# invariant:` marker, and the code targets
//! brownfield discovery reports.
//!
//! Binding is path-to-blueprint, never candidate-id-to-blueprint: a path-derived
//! discovery id is evidence, never a node id. Evidence that no declared path
//! claims is reported unbound; a binding is never invented. Nothing here scans
//! arbitrary prose, calls a model, drafts narrative, or mutates the blueprint.

use std::path::Path;

use crate::{error::CairnError, map::graph::Graph};

use super::{discovery, walk};

mod collect;
mod render;

use collect::{
    collect_code_targets, collect_documents, collect_invariant_comments, collect_readme_sections,
};
pub use render::{render_human, render_json};

/// Wire schema version for the evidence index.
pub const SCHEMA_VERSION: u32 = 1;

/// Directories whose files are decision evidence by location.
const DOCUMENT_ROOTS: &[&str] = &["docs/adr", "docs/decisions"];

/// README filename inspected for evidence sections.
const README_FILE: &str = "README.md";

/// README headings that mark a decision-evidence section, compared
/// case-insensitively against the whole heading text.
const SECTION_HEADINGS: &[&str] = &["decision", "rationale", "invariant"];

/// Comment markers that make a source line decision evidence.
const INVARIANT_MARKERS: &[&str] = &["// invariant:", "# invariant:"];

/// Recursion ceiling below each document root.
const MAX_DOCUMENT_DEPTH: usize = 8;

/// Which closed-set source produced one piece of evidence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceKind {
    /// A file under `docs/adr/` or `docs/decisions/`.
    Document,
    /// A README section headed Decision, Rationale, or Invariant.
    ReadmeSection,
    /// A source comment carrying `// invariant:` or `# invariant:`.
    InvariantComment,
    /// A directory brownfield discovery reports as a code target.
    CodeTarget,
}

impl EvidenceKind {
    /// Stable wire label for this kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::ReadmeSection => "readme-section",
            Self::InvariantComment => "invariant-comment",
            Self::CodeTarget => "code-target",
        }
    }
}

/// One indexed piece of decision evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    /// Which closed-set source produced it.
    pub kind: EvidenceKind,
    /// Path relative to the project root, forward-slashed.
    pub path: String,
    /// One-based line, for evidence located inside a file.
    pub line: Option<usize>,
    /// What was observed: a document title, the heading text, the invariant
    /// text, or the path-derived discovery candidate id.
    pub detail: String,
}

/// Evidence a declared blueprint path claims.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundEvidence {
    /// Id of the most-specific blueprint node declaring the path.
    pub node: String,
    /// The evidence itself.
    pub evidence: Evidence,
}

/// The deterministic decision-evidence index for one project root.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EvidenceIndex {
    /// Evidence bound to the blueprint node that owns its path.
    pub bound: Vec<BoundEvidence>,
    /// Evidence no declared path claims.
    pub unbound: Vec<Evidence>,
}

/// Resolves which blueprint node declares ownership of a path.
///
/// Mirrors the reconciler's private `eligible_owners` and `most_specific_owner`
/// (`src/reconcile/generic.rs`), which this module cannot call: eligible leaf or
/// `owns-files` nodes contribute normalised declared paths, most-specific first,
/// and `map::paths::is_component_prefix` selects the owner.
/// `tests/onboard_owner_parity.rs` pins both to the same answers.
pub struct OwnerResolver<'graph> {
    owners: Vec<(&'graph str, String)>,
}

impl<'graph> OwnerResolver<'graph> {
    /// Collects declared ownership from `graph`, most-specific path first.
    #[must_use]
    pub fn new(graph: &'graph Graph) -> Self {
        let mut owners: Vec<(&'graph str, String)> = Vec::new();
        for node in graph.nodes.values() {
            let is_internal = !node.children.is_empty();
            if !is_internal || node.owns_files {
                for path in &node.paths {
                    owners.push((node.id.as_str(), crate::map::paths::trim_dot(path)));
                }
            }
        }
        owners.sort_by_key(|(_, path)| std::cmp::Reverse(path.len()));
        Self { owners }
    }

    /// The most-specific declared owner of `path`, or `None` when nothing
    /// claims it.
    ///
    /// Two different nodes declaring an equally specific matching path is a
    /// blueprint defect, not a binding. The reconciler breaks that tie in
    /// declaration order, which the graph does not preserve, so rather than
    /// pick a winner it would not pick, this reports the evidence unbound.
    #[must_use]
    pub fn owner_of(&self, path: &str) -> Option<&'graph str> {
        let mut matches = self
            .owners
            .iter()
            .filter(|(_, declared)| crate::map::paths::is_component_prefix(declared, path));
        let (owner, declared) = matches.next()?;
        // Sorted most-specific first, so only an equally long declared path can
        // still contend, and an equal length that matches the same path means
        // the same declared path under a second node.
        for (other, other_declared) in matches {
            if other_declared.len() < declared.len() {
                break;
            }
            if other != owner {
                return None;
            }
        }
        Some(owner)
    }
}

/// Builds the decision-evidence index for `root` against the loaded `graph`.
///
/// One survey serves both questions: the code targets are the discovery
/// candidates, while the invariant scan needs every source file the survey
/// observed, which is a wider set.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when traversal fails.
pub fn index(root: &Path, graph: &Graph) -> Result<EvidenceIndex, CairnError> {
    let survey = walk::survey(root)?;
    let extraction = discovery::from_survey(root, &survey);
    let mut evidence = Vec::new();
    collect_documents(root, &mut evidence);
    collect_readme_sections(root, &survey, &mut evidence);
    collect_invariant_comments(root, &survey, &mut evidence);
    collect_code_targets(&extraction, &mut evidence);
    evidence.sort_by(|a, b| (a.kind, &a.path, a.line).cmp(&(b.kind, &b.path, b.line)));

    let resolver = OwnerResolver::new(graph);
    let mut report = EvidenceIndex::default();
    for item in evidence {
        // Ownership is path-derived, so confirm the resolved id is a real node
        // in the loaded graph before publishing a binding
        // (`dec.brownfield-extraction-mechanism` clause 1).
        match resolver
            .owner_of(&item.path)
            .filter(|id| graph.nodes.contains_key(*id))
        {
            Some(node) => report.bound.push(BoundEvidence {
                node: node.to_owned(),
                evidence: item,
            }),
            None => report.unbound.push(item),
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests;
