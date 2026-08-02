//! Core artefact registry types.

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

impl TodoStatus {
    /// Parses a CLI status token into a [`TodoStatus`].
    #[must_use]
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "open" => Some(Self::Open),
            "in_progress" => Some(Self::InProgress),
            "done" => Some(Self::Done),
            "blocked" => Some(Self::Blocked),
            _ => None,
        }
    }

    /// Canonical on-disk / CLI token for this status.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in_progress",
            Self::Done => "done",
            Self::Blocked => "blocked",
        }
    }
}

/// One parsed `defers:` reference from a todo: a finding code plus the path
/// or node it was raised against (`todo.lint-selection-folding` item 1a).
/// Both halves must match an emitted finding for the reference to bind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefersRef {
    /// Finding code the reference points at.
    pub code: String,
    /// Path or node id the finding was raised against.
    pub location: String,
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
    /// Todo stems this todo is blocked by (`dec.todo-relationship-model`
    /// ruling 1: directed dependency, authored on the downstream todo).
    pub blocked_by: Vec<String>,
    /// Containing todo stem (`dec.todo-relationship-model` ruling 1: the
    /// milestone/epic edge; grouping only, never order).
    pub parent: Option<String>,
    /// Weak, non-directional references: `dec.`/`res.`/`src.` ids or todo
    /// stems (`dec.todo-relationship-model` rulings 1 to 3).
    pub related: Vec<String>,
    /// Parsed `defers:` references. While the todo is `blocked`, each
    /// reference parks the matching Info finding out of loop selection.
    pub defers: Vec<DefersRef>,
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
/// Decision ratification tier (`todo.decision-ratification-tiers`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RatificationTier {
    /// A decision whose scope permits machine ratification.
    Local,
    /// A maintainer-only decision, and the default for existing artefacts.
    Binding,
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
    /// Ratification tier (`todo.decision-ratification-tiers`).
    pub ratification: RatificationTier,
    /// Repository-relative paths governed by this decision
    /// (`todo.decision-ratification-tiers`).
    pub affects: Vec<String>,
    /// Whether a machine ratified this decision
    /// (`todo.decision-ratification-tiers`).
    pub ratified_by_machine: bool,
    /// Review artefact stems evidencing ratification
    /// (`todo.decision-ratification-tiers`).
    pub receipts: Vec<String>,

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
    /// Whether this decision records an unresolved generative gap
    /// (`cairn gap`). Cleared by editing `gap: false` or deleting the file.
    pub gap: bool,

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
    /// Canonical manifest hash of the reviewed subject
    /// (`todo.decision-ratification-tiers`).
    pub subject_hash: Option<String>,
    /// Committed lens prompt hash for a receipt-grade review
    /// (`todo.decision-ratification-tiers`).
    pub lens_prompt_hash: Option<String>,
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
    /// Research method: primary observation or secondary (cites sources).
    pub method: ResearchMethod,
    /// Tags.
    pub tags: Vec<String>,
    /// Markdown body.
    pub body: String,
}

/// Whether the research evidence is original or derived from cited sources.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ResearchMethod {
    /// Original observation or experiment; the methodology is the evidence.
    Primary,
    /// Derived from cited sources (default); requires `sources`.
    #[default]
    Secondary,
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
    /// Live in-repo path read as it stands: resolution and containment
    /// checked, no hash (`dec.source-tracked-verification`).
    Tracked,
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

/// Active change directory loaded into the scan substrate.
///
/// A lightweight, text-only view of a `meta/changes/<id>/` directory: enough for
/// scan checks (e.g. revisit-trigger relevance) and queries to consume changes
/// uniformly from the [`ArtefactSet`], without pulling in the change-application
/// machinery (deltas, artefact operations) owned by the `changes` module.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChangeRecord {
    /// Change ID, derived from the directory name.
    pub id: String,
    /// Change directory path, formed by joining the scan root with
    /// `meta/changes/<id>` (root-relative when the scan root is relative).
    pub path: String,
    /// Proposal title (first heading), falling back to the ID.
    pub title: String,
    /// Proposal markdown body.
    pub proposal: String,
    /// Optional design markdown body.
    pub design: Option<String>,
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
    /// Active changes loaded from `meta/changes/`.
    pub changes: Vec<ChangeRecord>,
    /// Loading and validation findings.
    pub findings: Vec<Finding>,
}
