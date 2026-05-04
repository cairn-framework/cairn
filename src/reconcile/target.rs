//! Multi-target capability types.

use std::path::Path;

use crate::reconcile::ReconcilerId;

/// Default contract role assigned to targets without explicit configuration.
pub const DEFAULT_CONTRACT_ROLE: &str = "public_api";

/// Identifies a specific target within a node by node ID and path.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TargetId {
    /// Node that owns this target.
    pub node_id: String,
    /// Path for this target within the node.
    pub path: std::path::PathBuf,
}

impl TargetId {
    /// Creates a new target ID.
    #[must_use]
    pub const fn new(node_id: String, path: std::path::PathBuf) -> Self {
        Self { node_id, path }
    }

    /// Returns the target ID as a colon-separated string.
    #[must_use]
    pub fn as_str(&self) -> String {
        format!("{}:{}", self.node_id, self.path.display())
    }
}

/// Supported programming language for reconciliation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Language {
    /// Rust source files.
    Rust,
    /// TypeScript and TSX source files.
    TypeScript,
    /// Python source files.
    Python,
    /// Go source files.
    Go,
}

impl Language {
    /// Detects language from file extension.
    #[must_use]
    pub fn from_extension(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?;
        match ext {
            "rs" => Some(Self::Rust),
            "ts" | "tsx" => Some(Self::TypeScript),
            "py" => Some(Self::Python),
            "go" => Some(Self::Go),
            _ => None,
        }
    }

    /// Parses language from string representation.
    #[must_use]
    pub fn from_language_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rust" => Some(Self::Rust),
            "typescript" | "ts" => Some(Self::TypeScript),
            "python" | "py" => Some(Self::Python),
            "go" => Some(Self::Go),
            _ => None,
        }
    }

    /// Returns the language as a string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::TypeScript => "typescript",
            Self::Python => "python",
            Self::Go => "go",
        }
    }

    /// Returns the reconciler ID for this language.
    #[must_use]
    pub fn reconciler_id(&self) -> ReconcilerId {
        match self {
            Self::Rust => ReconcilerId("rust-code".to_owned()),
            Self::TypeScript => ReconcilerId("typescript-code".to_owned()),
            Self::Python => ReconcilerId("python-code".to_owned()),
            Self::Go => ReconcilerId("go-code".to_owned()),
        }
    }
}

/// A reconciled target representing a single path in a node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Target {
    /// Unique target identifier.
    pub id: TargetId,
    /// Language for this target.
    pub language: Language,
    /// Reconciler identifier.
    pub reconciler_id: ReconcilerId,
    /// Contract role this target satisfies.
    pub contract_role: String,
}

impl Target {
    /// Creates a new target with language detected from path extension.
    #[must_use]
    pub fn new(node_id: String, path: std::path::PathBuf, language: Language) -> Self {
        let reconciler_id = language.reconciler_id();
        Self {
            id: TargetId::new(node_id, path),
            language,
            reconciler_id,
            contract_role: DEFAULT_CONTRACT_ROLE.to_owned(),
        }
    }

    /// Sets the contract role and returns self for chaining.
    #[must_use]
    pub fn with_contract_role(mut self, role: String) -> Self {
        self.contract_role = role;
        self
    }
}

/// List of supported language identifiers as strings.
pub const SUPPORTED_LANGUAGES: &[&str] = &["rust", "typescript", "python", "go"];

/// Returns the error message for unsupported languages.
#[must_use]
pub fn language_error_message() -> String {
    format!(
        "supported languages are: {}",
        SUPPORTED_LANGUAGES.join(", ")
    )
}
