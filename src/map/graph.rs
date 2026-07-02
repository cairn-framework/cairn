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

impl FindingSeverity {
    /// Canonical lowercase wire-format label.
    ///
    /// Use this everywhere a severity is rendered into JSON. Cycle 4
    /// fix: previously, four manual emitters (`cli/format.rs`,
    /// `query_api/serialise.rs`, `hooks/render.rs`, `cli/hooks.rs`)
    /// printed the severity via Debug (`PascalCase`) while
    /// `ui/serialise.rs` and the serde derive emitted lowercase.
    /// Consumers parsing `severity` saw `"Error"` from one path and
    /// `"error"` from another. This method is the single source of
    /// truth.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
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
    /// Optional target node ID or contract role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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
    /// Extracted public symbols for this node.
    pub symbols: Vec<crate::reconcile::SymbolRecord>,
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
            target: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::{NodeKind, Span};
    use std::collections::BTreeMap;

    fn bare_node(id: &str) -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: id.to_owned(),
            name: id.to_owned(),
            description: String::new(),
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            symbols: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    fn one_node_graph(id: &str) -> Graph {
        let mut nodes = BTreeMap::new();
        nodes.insert(id.to_owned(), bare_node(id));
        Graph {
            nodes,
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        }
    }

    // ── FindingSeverity::name ──────────────────────────────────────────────────

    #[test]
    fn test_severity_name_error() {
        assert_eq!(FindingSeverity::Error.name(), "error");
    }

    #[test]
    fn test_severity_name_warning() {
        assert_eq!(FindingSeverity::Warning.name(), "warning");
    }

    #[test]
    fn test_severity_name_info() {
        assert_eq!(FindingSeverity::Info.name(), "info");
    }

    // ── Finding: Display ──────────────────────────────────────────────────────

    #[test]
    fn test_finding_display_format() {
        let f = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Warning,
            message: "something went wrong".to_owned(),
            node: None,
            target: None,
            path: None,
        };
        assert_eq!(f.to_string(), "CAIRN_TEST: something went wrong");
    }

    // ── Graph::has_errors ─────────────────────────────────────────────────────

    #[test]
    fn test_has_errors_empty_findings_returns_false() {
        let g = one_node_graph("a");
        assert!(!g.has_errors());
    }

    #[test]
    fn test_has_errors_warning_only_returns_false() {
        let mut g = one_node_graph("a");
        g.findings.push(Finding {
            code: "W".to_owned(),
            severity: FindingSeverity::Warning,
            message: "warn".to_owned(),
            node: None,
            target: None,
            path: None,
        });
        assert!(!g.has_errors(), "warning alone must not count as error");
    }

    #[test]
    fn test_has_errors_error_finding_returns_true() {
        let mut g = one_node_graph("a");
        g.findings.push(Finding {
            code: "E".to_owned(),
            severity: FindingSeverity::Error,
            message: "err".to_owned(),
            node: None,
            target: None,
            path: None,
        });
        assert!(g.has_errors());
    }

    // ── Graph::resolve — exact ID ─────────────────────────────────────────────

    #[test]
    fn test_resolve_exact_id_returns_node() {
        let g = one_node_graph("app.api");
        let node = g.resolve("app.api").expect("should resolve");
        assert_eq!(node.id, "app.api");
    }

    #[test]
    fn test_resolve_unknown_id_returns_not_found_finding() {
        let g = one_node_graph("app.api");
        let err = g.resolve("missing").unwrap_err();
        assert_eq!(err.code, "CAIRN_QUERY_NODE_NOT_FOUND");
    }

    #[test]
    fn test_resolve_unknown_id_error_is_error_severity() {
        let g = one_node_graph("app.api");
        let err = g.resolve("missing").unwrap_err();
        assert_eq!(err.severity, FindingSeverity::Error);
    }

    // ── Graph::resolve — name lookup ──────────────────────────────────────────

    #[test]
    fn test_resolve_by_unique_name_returns_node() {
        let mut g = one_node_graph("app.api");
        // "Api" is an alias for the node with id "app.api".
        g.names.insert("Api".to_owned(), vec!["app.api".to_owned()]);
        let node = g.resolve("Api").expect("should resolve by name");
        assert_eq!(node.id, "app.api");
    }

    #[test]
    fn test_resolve_ambiguous_name_falls_through_to_error() {
        // Two nodes share the name "Api". The [id] destructure fails for 2 elements,
        // so resolve must return an error, not silently pick one.
        let mut g = one_node_graph("app.api");
        g.nodes.insert("test.api".to_owned(), bare_node("test.api"));
        g.names.insert(
            "Api".to_owned(),
            vec!["app.api".to_owned(), "test.api".to_owned()],
        );
        let err = g.resolve("Api").unwrap_err();
        assert_eq!(
            err.code, "CAIRN_QUERY_NODE_NOT_FOUND",
            "ambiguous name must produce not-found error, not silently pick one node"
        );
    }

    // ── Graph::resolve — suggestions ──────────────────────────────────────────

    #[test]
    fn test_resolve_unknown_with_partial_match_includes_suggestion() {
        // Searching for "api" when "app.api" exists — "app.api" contains "api".
        let g = one_node_graph("app.api");
        let err = g.resolve("api").unwrap_err();
        assert!(
            err.message.contains("suggestion"),
            "message must include 'suggestion' when a partial match exists: {}",
            err.message
        );
        assert!(
            err.message.contains("app.api"),
            "message must name the matching node: {}",
            err.message
        );
    }

    #[test]
    fn test_resolve_unknown_with_no_partial_match_has_no_suggestion() {
        let g = one_node_graph("app.api");
        let err = g.resolve("zzz").unwrap_err();
        assert!(
            !err.message.contains("suggestion"),
            "no partial match must produce no suggestion: {}",
            err.message
        );
    }

    #[test]
    fn test_resolve_suggestions_capped_at_three() {
        // Four nodes all matching the search term; suggestions must be <= 3.
        let mut g = Graph {
            nodes: BTreeMap::new(),
            names: BTreeMap::new(),
            outbound: BTreeMap::new(),
            inbound: BTreeMap::new(),
            findings: Vec::new(),
        };
        for id in &["x.a", "x.b", "x.c", "x.d"] {
            g.nodes.insert((*id).to_owned(), bare_node(id));
        }
        let err = g.resolve("x").unwrap_err();
        // All four nodes contain "x", but take(3) limits to 3 suggestions.
        // Count commas in the suggestions section: 2 commas means 3 items.
        let suggestions_part = err.message.split("suggestions: ").nth(1).unwrap_or("");
        let suggestion_count = suggestions_part.split(", ").count();
        assert!(
            suggestion_count <= 3,
            "suggestions must be capped at 3, got {suggestion_count}: {}",
            err.message
        );
    }
}
