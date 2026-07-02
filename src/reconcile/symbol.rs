//! Structured symbol records extracted by the reconcilers.

/// One extracted public symbol with its source location.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SymbolRecord {
    /// Identifier name.
    pub name: String,
    /// Symbol kind.
    pub kind: SymbolKind,
    /// Normalised signature (same string that feeds the interface fingerprint).
    pub signature: String,
    /// Repo-relative file path.
    pub file: String,
    /// 1-based start line.
    pub line: u32,
    /// 1-based end line.
    pub end_line: u32,
}

/// Language-agnostic symbol kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
    /// Function or method.
    Function,
    /// Struct.
    Struct,
    /// Class.
    Class,
    /// Enum.
    Enum,
    /// Trait.
    Trait,
    /// Interface.
    Interface,
    /// Type alias.
    Type,
    /// Constant.
    Const,
    /// Static.
    Static,
    /// Module.
    Module,
    /// Union.
    Union,
    /// Variable.
    Variable,
    /// Any kind not covered above.
    Other,
}

/// Collapses all whitespace runs to single spaces and trims the ends.
///
/// Shared by all four reconcilers: the normalised text is the exact string
/// that feeds both `SymbolRecord.signature` and the interface fingerprint
/// hash, so this computation must never diverge per language.
#[must_use]
pub fn normalize_symbol(text: &str) -> String {
    // Fast path: text is already normalized (no consecutive whitespace,
    // no leading/trailing whitespace).
    let bytes = text.as_bytes();
    if !bytes.is_empty()
        && !bytes[0].is_ascii_whitespace()
        && !bytes[bytes.len() - 1].is_ascii_whitespace()
        && !bytes
            .windows(2)
            .any(|w| w[0].is_ascii_whitespace() && w[1].is_ascii_whitespace())
    {
        return text.to_owned();
    }
    let mut result = String::with_capacity(text.len());
    let mut in_whitespace = true;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !in_whitespace && !result.is_empty() {
                result.push(' ');
            }
            in_whitespace = true;
        } else {
            result.push(ch);
            in_whitespace = false;
        }
    }
    if result.ends_with(' ') {
        result.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::normalize_symbol;

    #[test]
    fn normalize_symbol_collapses_consecutive_whitespace() {
        assert_eq!(normalize_symbol("fn   foo"), "fn foo");
    }

    #[test]
    fn normalize_symbol_trims_leading_and_trailing_whitespace() {
        assert_eq!(normalize_symbol("  fn foo  "), "fn foo");
    }

    #[test]
    fn normalize_symbol_already_normalized_returns_input_owned() {
        assert_eq!(normalize_symbol("fn foo"), "fn foo");
    }

    #[test]
    fn normalize_symbol_collapses_newlines_like_whitespace() {
        assert_eq!(normalize_symbol("fn\n  foo"), "fn foo");
    }
}
