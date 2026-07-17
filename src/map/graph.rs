// cairn:allow-large-module reason: JsonSchema helper and graph wire types remain cohesive
//! In-memory map graph structures.

use std::{collections::BTreeMap, error::Error, fmt};

use crate::blueprint::{NodeKind, Span};
use schemars::{r#gen::SchemaGenerator, schema::Schema};

fn nullable_string_schema(generator: &mut SchemaGenerator) -> Schema {
    generator.subschema_for::<Option<String>>()
}

/// Runtime state assigned during reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeState {
    /// Declared path exists and owns claimed source files (or no paths).
    Synced,
    /// Declared structure with no owned source files yet (missing path or
    /// empty scaffolding).
    Ghost,
    /// Source reality exists but no eligible node owns it.
    Orphaned,
}

/// Integrity or reconciliation finding severity.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
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
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Finding {
    /// Stable finding code.
    pub code: String,
    /// Severity.
    pub severity: FindingSeverity,
    /// Human-readable message.
    pub message: String,
    /// Optional node ID.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub node: Option<String>,
    /// Optional target node ID or contract role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Optional file path.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub path: Option<String>,
    /// Decision id this finding is deferred by, if any (set at the emission site).
    /// Skipped in `--json` so the wire format is unchanged.
    #[serde(skip)]
    pub deferred_by: Option<String>,
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
    /// Resolves an exact node ID, unambiguous name, or unambiguous dotted
    /// suffix alias (e.g. `kernel.scanner` for `cairn.kernel.scanner`).
    ///
    /// # Errors
    ///
    /// Returns a query finding when the value is not an ID, unambiguous name,
    /// or unambiguous suffix alias. An ambiguous suffix lists the candidate
    /// IDs in the finding message.
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
        if !value.is_empty() {
            let dotted = format!(".{value}");
            let candidates = self
                .nodes
                .keys()
                .filter(|id| id.ends_with(&dotted))
                .cloned()
                .collect::<Vec<_>>();
            if let [id] = candidates.as_slice() {
                return Ok(&self.nodes[id]);
            }
            if candidates.len() > 1 {
                return Err(Finding {
                    code: "CAIRN_QUERY_NODE_NOT_FOUND".to_owned(),
                    severity: FindingSeverity::Error,
                    message: format!(
                        "node `{value}` is ambiguous; candidates: {}",
                        candidates.join(", ")
                    ),
                    node: None,
                    target: None,
                    path: None,
                    deferred_by: None,
                });
            }
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
            deferred_by: None,
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
            deferred_by: None,
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
            deferred_by: None,
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
            deferred_by: None,
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

    // ── Graph::resolve: suffix aliases ────────────────────────────────────────

    #[test]
    fn test_resolve_unique_dotted_suffix_returns_node() {
        let mut g = one_node_graph("cairn.kernel.scanner");
        g.nodes
            .insert("cairn.kernel.map".to_owned(), bare_node("cairn.kernel.map"));
        assert_eq!(
            g.resolve("scanner").expect("suffix must resolve").id,
            "cairn.kernel.scanner"
        );
        assert_eq!(
            g.resolve("kernel.scanner")
                .expect("multi-segment suffix must resolve")
                .id,
            "cairn.kernel.scanner"
        );
    }

    #[test]
    fn test_resolve_ambiguous_suffix_lists_candidates() {
        let mut g = one_node_graph("app.api");
        g.nodes.insert("test.api".to_owned(), bare_node("test.api"));
        let err = g.resolve("api").unwrap_err();
        assert_eq!(err.code, "CAIRN_QUERY_NODE_NOT_FOUND");
        assert!(
            err.message.contains("ambiguous"),
            "ambiguous suffix must be reported as ambiguous: {}",
            err.message
        );
        assert!(
            err.message.contains("app.api") && err.message.contains("test.api"),
            "ambiguous suffix must list all candidate IDs: {}",
            err.message
        );
    }

    #[test]
    fn test_resolve_suffix_requires_segment_boundary() {
        // "pi" is a trailing substring of "app.api" but not a dotted suffix.
        let g = one_node_graph("app.api");
        assert!(
            g.resolve("pi").is_err(),
            "non-segment suffix must not resolve"
        );
    }

    #[test]
    fn test_resolve_exact_id_wins_over_suffix_alias() {
        // "kernel.map" exists as a full ID and as a suffix of another node;
        // the exact ID must win without an ambiguity error.
        let mut g = one_node_graph("kernel.map");
        g.nodes
            .insert("cairn.kernel.map".to_owned(), bare_node("cairn.kernel.map"));
        assert_eq!(
            g.resolve("kernel.map").expect("exact ID wins").id,
            "kernel.map"
        );
    }

    // ── Graph::resolve — suggestions ──────────────────────────────────────────

    #[test]
    fn test_resolve_unknown_with_partial_match_includes_suggestion() {
        // "app.a" is a partial match for "app.api" but not a dotted suffix,
        // so resolve still fails with a suggestion.
        let g = one_node_graph("app.api");
        let err = g.resolve("app.a").unwrap_err();
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
