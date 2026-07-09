//! Multi-target capability types.

use std::{fs, path::Path};

use crate::reconcile::ReconcilerId;
use crate::scanner::config;

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
    pub fn new(node_id: String, path: std::path::PathBuf) -> Self {
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
    /// Unknown language: no reconciler available.
    Unknown,
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::TypeScript => "typescript",
            Self::Python => "python",
            Self::Go => "go",
            Self::Unknown => "unknown",
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
            Self::Unknown => ReconcilerId("none".to_owned()),
        }
    }
    /// Infers the dominant language by walking a directory.
    ///
    /// Counts files with supported extensions, applies the same ignore rules
    /// used by reconcilers, and returns the language with the most files.
    /// Ties are broken by the order of [`SUPPORTED_LANGUAGES`].
    #[must_use]
    pub fn infer_from_directory(root: &Path, path: &Path, ignores: &[String]) -> Option<Self> {
        let abs_dir = root.join(path);
        if !abs_dir.is_dir() {
            return None;
        }

        let mut counts = std::collections::BTreeMap::<Self, usize>::new();
        infer_walk(root, &abs_dir, ignores, &mut counts);

        counts
            .into_iter()
            .max_by(|(lang_a, count_a), (lang_b, count_b)| {
                count_a.cmp(count_b).then_with(|| {
                    let order_a = SUPPORTED_LANGUAGES
                        .iter()
                        .position(|&s| s == lang_a.as_str())
                        .unwrap_or(usize::MAX);
                    let order_b = SUPPORTED_LANGUAGES
                        .iter()
                        .position(|&s| s == lang_b.as_str())
                        .unwrap_or(usize::MAX);
                    order_b.cmp(&order_a)
                })
            })
            .map(|(lang, _)| lang)
    }
}

#[allow(clippy::collapsible_if)] // Reason: keeping the extension check inside the file branch mirrors the reconciler walk idiom and avoids a double from_extension call.
fn infer_walk(
    root: &Path,
    dir: &Path,
    ignores: &[String],
    counts: &mut std::collections::BTreeMap<Language, usize>,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if config::is_ignored(&rel, ignores) {
            continue;
        }
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            infer_walk(root, &path, ignores, counts);
        } else if file_type.is_file() {
            if let Some(lang) = Language::from_extension(&path) {
                *counts.entry(lang).or_insert(0) += 1;
            }
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
