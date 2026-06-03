// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;
use crate::map::graph::Finding;

/// Supported v1 artefact types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtefactType {
    /// Contract artefact.
    Contract,
    /// Todo artefact.
    Todo,
    /// Decision artefact.
    Decision,
    /// Review artefact.
    Review,
    /// Research artefact.
    Research,
    /// Source artefact.
    Source,
}

/// Generic artefact loader request.
#[derive(Clone, Copy, Debug)]
pub struct ArtefactLoadRequest<'a> {
    /// Project root.
    pub root: &'a Path,
    /// Parsed blueprint.
    pub ast: &'a Ast,
}

/// Generic loaded artefact record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtefactRecord {
    /// Artefact type.
    pub artefact_type: ArtefactType,
    /// Stable artefact ID, or path for path-keyed records.
    pub id: String,
    /// Declared path.
    pub path: String,
}

/// Artefact loader error.
pub type ArtefactError = String;

/// Common interface for typed artefact loaders.
pub trait ArtefactLoader {
    /// Artefact type handled by the loader.
    fn artefact_type(&self) -> ArtefactType;
    /// Loads records for the request.
    ///
    /// # Errors
    ///
    /// Returns a loader-level error when the filesystem cannot be traversed.
    fn load(&self, request: ArtefactLoadRequest<'_>) -> Result<Vec<ArtefactRecord>, ArtefactError>;
}

/// Todo status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TodoStatus {
    /// Open todo.
    Open,
    /// In progress todo.
    InProgress,
    /// Completed todo.
    Done,
    /// Blocked todo.
    Blocked,
}

/// Parsed todo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
    /// Source path.
    pub path: String,
    /// Referenced node.
    pub node: String,
    /// Status.
    pub status: TodoStatus,
    /// Creation date.
    pub created: String,
    /// Optional satisfied contract clause.
    pub satisfies: Option<String>,
    /// Markdown body.
    pub body: String,
}

/// Decision status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionStatus {
    /// Proposed decision.
    Proposed,
    /// Accepted decision.
    Accepted,
    /// Deprecated decision.
    Deprecated,
    /// Superseded decision.
    Superseded,
}

/// Claims mode for folder enumeration in decision artefacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimsMode {
    /// Every file in the folder must match the claimed list exactly.
    Exhaustive,
    /// The claimed list is illustrative only; no drift check is performed.
    Illustrative,
}

/// Parsed claims block from a decision artefact frontmatter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claims {
    /// Folder path the claim refers to, relative to project root.
    pub folder: String,
    /// Claim mode.
    pub mode: ClaimsMode,
    /// Claimed file names (not paths).
    pub items: Vec<String>,
}

/// Parsed decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decision {
    /// Stable decision ID.
    pub id: String,
    /// Source path.
    pub path: String,
    /// Referenced nodes.
    pub nodes: Vec<String>,
    /// Status.
    pub status: DecisionStatus,
    /// Decision date.
    pub date: String,
    /// Last revisited date.
    pub revisited: Option<String>,
    /// Revisit triggers.
    pub revisit_triggers: Vec<String>,
    /// Referenced research/source IDs.
    pub informed_by: Vec<String>,
    /// Superseded decision IDs.
    pub supersedes: Vec<String>,
    /// Refined decision IDs.
    pub refines: Vec<String>,
    /// Related decision IDs.
    pub related: Vec<String>,
    /// Whether all node references are intentionally orphaned.
    pub orphaned: bool,
    /// Orphan reason.
    pub orphan_reason: Option<String>,

    /// Optional folder-enumeration claims.
    pub claims: Option<Claims>,
    /// Markdown body.
    pub body: String,
}

/// Review subtype.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewType {
    /// Human-authored review.
    Human,
    /// Implementing agent self-review.
    AgentIntrospective,
    /// Cross-model agent review.
    AgentCrossModel,
}

/// Parsed review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    /// Source path.
    pub path: String,
    /// Referenced node.
    pub node: String,
    /// Review subtype.
    pub review_type: ReviewType,
    /// Review date.
    pub date: String,
    /// Reviewer identifier.
    pub reviewer: String,
    /// Optional related change.
    pub related_change: Option<String>,
    /// Markdown body.
    pub body: String,
}

/// Parsed research.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Research {
    /// Stable research ID.
    pub id: String,
    /// Source path.
    pub path: String,
    /// Referenced nodes.
    pub nodes: Vec<String>,
    /// Research date.
    pub date: String,
    /// Referenced source IDs.
    pub sources: Vec<String>,
    /// Tags.
    pub tags: Vec<String>,
    /// Markdown body.
    pub body: String,
}

/// Source verification state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceVerification {
    /// Local file hash is verified.
    Verified,
    /// External URL reference.
    External,
    /// Unverified source.
    Unverified,
}

/// Parsed source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    /// Stable source ID.
    pub id: String,
    /// Source manifest path.
    pub path: String,
    /// Local file path or URL.
    pub file: String,
    /// Optional expected SHA-256.
    pub sha256: Option<String>,
    /// Verification state.
    pub verification: SourceVerification,
    /// Source type.
    pub source_type: String,
    /// Source date.
    pub date: String,
    /// Tags.
    pub tags: Vec<String>,
    /// Description.
    pub description: String,
    /// Markdown body.
    pub body: String,
}

/// Loaded Phase 2 artefacts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtefactSet {
    /// Contract set.
    pub contracts: ContractSet,
    /// Todos.
    pub todos: Vec<Todo>,
    /// Decisions.
    pub decisions: Vec<Decision>,
    /// Reviews.
    pub reviews: Vec<Review>,
    /// Research records.
    pub research: Vec<Research>,
    /// Sources.
    pub sources: Vec<Source>,
    /// Loading and validation findings.
    pub findings: Vec<Finding>,
}
