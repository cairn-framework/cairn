//! Source-positioned parser errors.

use std::{error::Error, fmt};

use super::ast::Span;

/// Parser error category.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    /// A token did not match the grammar.
    UnexpectedToken {
        /// Expected token description.
        expected: String,
        /// Encountered token description.
        encountered: String,
    },
    /// A string literal was not terminated.
    UnterminatedString,
    /// A required node field was missing.
    MissingField(&'static str),
}

/// Parser error with stable code and source location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    /// Stable error code.
    pub code: &'static str,
    /// Human-readable message.
    pub message: Box<str>,
    /// Source span.
    pub span: Box<Span>,
    /// Specific parser failure.
    pub kind: Box<ParseErrorKind>,
}

impl ParseError {
    /// Creates an unexpected-token parser error.
    #[must_use]
    pub fn unexpected(
        span: Span,
        expected: impl Into<String>,
        encountered: impl Into<String>,
    ) -> Self {
        let expected = expected.into();
        let encountered = encountered.into();
        Self {
            code: "CAIRN_PARSE_UNEXPECTED_TOKEN",
            message: format!("expected {expected}, encountered {encountered}").into_boxed_str(),
            span: Box::new(span),
            kind: Box::new(ParseErrorKind::UnexpectedToken {
                expected,
                encountered,
            }),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{}: {}",
            self.span.file, self.span.line, self.span.column, self.message
        )
    }
}

impl Error for ParseError {}
