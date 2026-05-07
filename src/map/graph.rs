//! In-memory map graph structures.

use std::{collections::BTreeMap, error::Error, fmt};

use crate::blueprint::{NodeKind, Span};

/// Runtime state assigned during reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeState {
    /// Declared path exists and has claimed files.
    Synced,
    /// Declared path or contract target is currently absent.
    Ghost,
    /// Source reality exists but no eligible node owns it.
    Orphaned,
}

/// Integrity or reconciliation finding severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingSeverity {
    /// Blocks successful lint/map validation.
    Error,
    /// Advisory finding.
    Warning,
    /// Informational nudge that does not block hooks or gates.
    Info,
}

/// Finding with stable code.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Finding {
    /// Stable finding code.
    pub code: String,
    /// Severity.
    pub severity: FindingSeverity,
    /// Human-readable message.
    pub message: String,
    /// Optional node ID.
    pub node: Option<String>,
    /// Optional file path.
    pub path: Option<String>,
}

/// Flattened node record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeRecord {
    /// Node kind.
    pub kind: NodeKind,
    /// Stable ID.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Tags.
    pub tags: Vec<String>,
    /// Optional parent ID.
    pub parent: Option<String>,
    /// Child node IDs.
    pub children: Vec<String>,
    /// Declared paths.
    pub paths: Vec<String>,
    /// Effective file ownership flag.
    pub owns_files: bool,
    /// Contract pointers.
    pub contracts: Vec<String>,
    /// Reconciled state.
    pub state: NodeState,
    /// Claimed files.
    pub files: Vec<String>,
    /// Declaration span.
    pub span: Span,
}

/// Dependency edge reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeRef {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Description.
    pub description: String,
}

/// Queryable map graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph {
    /// Nodes keyed by ID.
    pub nodes: BTreeMap<String, NodeRecord>,
    /// Exact name index.
    pub names: BTreeMap<String, Vec<String>>,
    /// Outbound dependency edges.
    pub outbound: BTreeMap<String, Vec<EdgeRef>>,
    /// Inbound dependency edges.
    pub inbound: BTreeMap<String, Vec<EdgeRef>>,
    /// Integrity findings.
    pub findings: Vec<Finding>,
}

impl Graph {
    /// Resolves an exact node ID or unambiguous name.
    ///
    /// # Errors
    ///
    /// Returns a query finding when the value is not an ID or unambiguous name.
    pub fn resolve(&self, value: &str) -> Result<&NodeRecord, Finding> {
        if let Some(node) = self.nodes.get(value) {
            return Ok(node);
        }
        if let Some(ids) = self.names.get(value)
            && let [id] = ids.as_slice()
            && let Some(node) = self.nodes.get(id)
        {
            return Ok(node);
        }
        let suggestion = self
            .nodes
            .keys()
            .filter(|id| id.contains(value) || value.contains(id.as_str()))
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let suffix = if suggestion.is_empty() {
            String::new()
        } else {
            format!("; suggestions: {suggestion}")
        };
        Err(Finding {
            code: "CAIRN_QUERY_NODE_NOT_FOUND".to_owned(),
            severity: FindingSeverity::Error,
            message: format!("node `{value}` was not found{suffix}"),
            node: None,
            path: None,
        })
    }

    /// True when any error finding exists.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for Finding {}
