//! Change-domain data types: `Change`, `BlueprintDelta`, artefact operations, and snapshots.
use super::*;

pub struct Change {
    /// Change ID, derived from the directory name.
    pub id: String,
    /// Change directory path.
    pub path: PathBuf,
    /// Proposal title.
    pub title: String,
    /// Proposal markdown.
    pub proposal: String,
    /// Optional design markdown.
    pub design: Option<String>,
    /// Parsed blueprint delta.
    pub delta: BlueprintDelta,
    /// Parsed artefact operations.
    pub artefacts: Vec<ArtefactOperation>,
    /// Validation messages collected while loading the change.
    pub findings: Vec<String>,
}

/// Blueprint delta operations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BlueprintDelta {
    /// Added nodes.
    pub added_nodes: Vec<Node>,
    /// Modified node declarations.
    pub modified_nodes: Vec<Node>,
    /// Removed node IDs.
    pub removed_nodes: Vec<String>,
    /// Renamed node IDs.
    pub renamed_nodes: Vec<Rename>,
    /// Added edges.
    pub added_edges: Vec<Edge>,
    /// Modified edges.
    pub modified_edges: Vec<Edge>,
    /// Removed edges.
    pub removed_edges: Vec<Edge>,
    /// Renamed edge endpoints.
    pub renamed_edges: Vec<EdgeRename>,
}

/// Old and new ID pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rename {
    /// Existing ID.
    pub from: String,
    /// Proposed ID.
    pub to: String,
}

/// Edge replacement pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeRename {
    /// Existing edge.
    pub from: Edge,
    /// Proposed edge.
    pub to: Edge,
}

/// Artefact operation parsed from mirrored change directories.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtefactOperation {
    /// Operation type.
    pub operation: ChangeOperation,
    /// Path inside the change directory.
    pub change_path: PathBuf,
    /// Target path in the main tree.
    pub target_path: PathBuf,
    /// Source path for rename operations.
    pub renamed_from: Option<PathBuf>,
    /// File content.
    pub content: String,
}

/// Supported operation kinds.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ChangeOperation {
    /// Add a new item.
    Added,
    /// Modify an existing item.
    Modified,
    /// Remove an existing item.
    Removed,
    /// Rename an existing item.
    Renamed,
}

/// Archive outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveReport {
    /// Archive directory path.
    pub archive_path: PathBuf,
    /// Human-readable operation summary.
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Snapshot {
    pub(super) path: PathBuf,
    pub(super) content: Option<Vec<u8>>,
}
