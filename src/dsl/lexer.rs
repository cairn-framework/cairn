//! Hand-written Cairn DSL lexer.

use super::{
    ast::Span,
    error::{ParseError, ParseErrorKind},
};

/// Token kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// Identifier or keyword-like word.
    Word(String),
    /// Quoted string.
    String(String),
    /// `@tag`.
    Tag(String),
    /// `{`.
    OpenBrace,
    /// `}`.
    CloseBrace,
    /// `[`.
    OpenBracket,
    /// `]`.
    CloseBracket,
    /// `,`.
    Comma,
    /// `:`.
    Colon,
    /// `->`.
    Arrow,
    /// End of input.
    Eof,
}

/// Lexer token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    /// Token kind.
    pub kind: TokenKind,
    /// Source span.
    pub span: Span,
}

/// Tokenizes DSL source.
///
/// # Errors
///
/// Returns a parse error when a string literal is unterminated.
pub fn tokenize(file: &str, source: &str) -> Result<Vec<Token>, ParseError> {
    let mut lexer = Lexer {
        file,
        chars: source.chars().collect(),
        index: 0,
        line: 1,
        column: 1,
    };
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        let is_eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if is_eof {
            return Ok(tokens);
        }
    }
}

struct Lexer<'a> {
    file: &'a str,
    chars: Vec<char>,
    index: usize,
    line: usize,
    column: usize,
}

impl Lexer<'_> {
    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_space_and_comments();
        let start = self.span_start();
        let Some(ch) = self.peek() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: start,
            });
        };
        match ch {
            '{' => Ok(self.single(TokenKind::OpenBrace)),
            '}' => Ok(self.single(TokenKind::CloseBrace)),
            '[' => Ok(self.single(TokenKind::OpenBracket)),
            ']' => Ok(self.single(TokenKind::CloseBracket)),
            ',' => Ok(self.single(TokenKind::Comma)),
            ':' => Ok(self.single(TokenKind::Colon)),
            '"' => self.string(),
            '@' => Ok(self.tag()),
            '-' if self.peek_next() == Some('>') => {
                self.bump();
                self.bump();
                Ok(Token {
                    kind: TokenKind::Arrow,
                    span: self.finish(start),
                })
            }
            _ => Ok(self.word()),
        }
    }

    fn skip_space_and_comments(&mut self) {
        loop {
            while self.peek().is_some_and(char::is_whitespace) {
                self.bump();
            }
            if self.peek() == Some('#') {
                while self.peek().is_some_and(|ch| ch != '\n') {
                    self.bump();
                }
                continue;
            }
            break;
        }
    }

    fn string(&mut self) -> Result<Token, ParseError> {
        let start = self.span_start();
        self.bump();
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                return Ok(Token {
                    kind: TokenKind::String(value),
                    span: self.finish(start),
                });
            }
            if ch == '\\' {
                self.bump();
                if let Some(escaped) = self.peek() {
                    value.push(escaped);
                    self.bump();
                }
            } else {
                value.push(ch);
                self.bump();
            }
        }
        Err(ParseError {
            code: "CAIRN_PARSE_UNTERMINATED_STRING",
            message: "unterminated string literal".into(),
            span: Box::new(start),
            kind: Box::new(ParseErrorKind::UnterminatedString),
        })
    }

    fn tag(&mut self) -> Token {
        let start = self.span_start();
        self.bump();
        let mut value = String::new();
        while self.peek().is_some_and(is_word_char) {
            value.push(self.bump().expect("peeked char exists"));
        }
        Token {
            kind: TokenKind::Tag(value),
            span: self.finish(start),
        }
    }

    fn word(&mut self) -> Token {
        let start = self.span_start();
        let mut value = String::new();
        while self.peek().is_some_and(|ch| {
            !ch.is_whitespace() && !matches!(ch, '{' | '}' | '[' | ']' | ',' | ':' | '"')
        }) {
            if self.peek() == Some('-') && self.peek_next() == Some('>') {
                break;
            }
            value.push(self.bump().expect("peeked char exists"));
        }
        Token {
            kind: TokenKind::Word(value),
            span: self.finish(start),
        }
    }

    fn single(&mut self, kind: TokenKind) -> Token {
        let start = self.span_start();
        self.bump();
        Token {
            kind,
            span: self.finish(start),
        }
    }

    fn span_start(&self) -> Span {
        Span::point(self.file, self.line, self.column)
    }

    fn finish(&self, mut span: Span) -> Span {
        span.end_line = self.line;
        span.end_column = self.column;
        span
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.index += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }
}

fn is_word_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.')
}
