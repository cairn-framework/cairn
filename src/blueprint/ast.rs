//! Typed AST produced by the Cairn blueprint parser.

/// Source span with one-based line and column positions.
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ast {
    /// Top-level nodes.
    pub nodes: Vec<Node>,
    /// Top-level dependency edges.
    pub edges: Vec<Edge>,
}

/// Supported node declarations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    /// Field name.
    pub name: String,
    /// Field values.
    pub values: Vec<String>,
    /// Field span.
    pub span: Span,
}

/// Dependency edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Edge description.
    pub description: String,
    /// Edge span.
    pub span: Span,
}
