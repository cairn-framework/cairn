//! Typed AST produced by the Cairn blueprint parser.

/// Source span with one-based line and column positions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Span {
    /// Source path used for diagnostics.
    pub file: String,
    /// Start line.
    pub line: usize,
    /// Start column.
    pub column: usize,
    /// End line.
    pub end_line: usize,
    /// End column.
    pub end_column: usize,
}

impl Span {
    /// Creates a zero-width span at a source position.
    #[must_use]
    pub fn point(file: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            end_line: line,
            end_column: column,
        }
    }
}

/// Parsed blueprint root.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ast {
    /// Top-level nodes.
    pub nodes: Vec<Node>,
    /// Top-level dependency edges.
    pub edges: Vec<Edge>,
}

/// Supported node declarations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NodeKind {
    /// System declaration.
    System,
    /// Container declaration.
    Container,
    /// Module declaration.
    Module,
    /// Actor declaration.
    Actor,
}

/// Parsed node declaration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node {
    /// Declaration kind.
    pub kind: NodeKind,
    /// Human-readable name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Stable ID.
    pub id: String,
    /// Tags declared as `@tag`.
    pub tags: Vec<String>,
    /// Optional path field.
    pub paths: Vec<String>,
    /// Internal file ownership opt-in.
    pub owns_files: bool,
    /// Contract artefact pointers.
    pub contracts: Vec<String>,
    /// Retained non-contract artefact pointer fields.
    pub raw_fields: Vec<Field>,
    /// Nested child nodes.
    pub children: Vec<Self>,
    /// Declaration source span.
    pub span: Span,
}

/// Retained field metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// Field name.
    pub name: String,
    /// Field values.
    pub values: Vec<String>,
    /// Field span.
    pub span: Span,
}

/// Edge provenance recorded in the blueprint.
///
/// An absent edge marker is intentionally represented as [`Self::HandDeclared`]
/// so existing blueprints retain their meaning without a migration.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EdgeProvenance {
    /// Edge declared by a human without a provenance marker.
    #[default]
    HandDeclared,
    /// Edge inferred from observed brownfield dependencies.
    Inferred,
}

impl EdgeProvenance {
    /// Marker text used by the canonical blueprint grammar.
    pub(crate) const MARKER: &'static str = "inferred";

    /// Returns the marker text for this provenance, if one is required.
    #[must_use]
    pub(crate) const fn marker(self) -> Option<&'static str> {
        match self {
            Self::HandDeclared => None,
            Self::Inferred => Some(Self::MARKER),
        }
    }
}

/// Dependency edge.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Edge {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Edge description.
    pub description: String,
    /// Edge provenance marker.
    pub provenance: EdgeProvenance,
    /// Edge span.
    pub span: Span,
}
