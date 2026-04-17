//! Recursive descent parser for the Phase 1 Cairn DSL grammar.

use std::{fs, path::Path};

use super::{
    ast::{Ast, Edge, Field, Node, NodeKind, Span},
    error::{ParseError, ParseErrorKind},
    lexer::{Token, TokenKind, tokenize},
};

/// Parses a DSL file from disk.
///
/// # Errors
///
/// Returns an I/O-backed parse error when the file cannot be read, or a
/// source-positioned parse error when the DSL is malformed.
pub fn parse_file(path: impl AsRef<Path>) -> Result<Ast, ParseError> {
    let path = path.as_ref();
    let source = fs::read_to_string(path).map_err(|error| {
        let span = Span::point(path.display().to_string(), 1, 1);
        ParseError {
            code: "CAIRN_IO_READ_DSL",
            message: format!("failed to read DSL: {error}").into_boxed_str(),
            span: Box::new(span),
            kind: Box::new(ParseErrorKind::UnexpectedToken {
                expected: "readable DSL file".to_owned(),
                encountered: "I/O error".to_owned(),
            }),
        }
    })?;
    parse_str(&path.display().to_string(), &source)
}

/// Parses DSL source.
///
/// # Errors
///
/// Returns a source-positioned parse error when the DSL is malformed.
pub fn parse_str(file: &str, source: &str) -> Result<Ast, ParseError> {
    Parser {
        tokens: tokenize(file, source)?,
        index: 0,
    }
    .parse()
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn parse(&mut self) -> Result<Ast, ParseError> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        while !self.at_eof() {
            if self.peek_node_kind().is_some() {
                nodes.push(self.node()?);
            } else {
                edges.push(self.edge()?);
            }
        }
        Ok(Ast { nodes, edges })
    }

    fn node(&mut self) -> Result<Node, ParseError> {
        let kind_token = self.next().clone();
        let kind = node_kind(&kind_token).ok_or_else(|| {
            ParseError::unexpected(
                kind_token.span.clone(),
                "node declaration",
                describe(&kind_token.kind),
            )
        })?;
        let name = self.expect_word("node name")?;
        let description = self.expect_string("node description")?;
        self.expect_word_exact("id")?;
        let id = self.expect_string("node id")?;
        let mut tags = Vec::new();
        while let TokenKind::Tag(tag) = self.peek().kind.clone() {
            self.next();
            tags.push(tag);
        }
        self.expect_kind(&TokenKind::OpenBrace, "`{`")?;
        let mut paths = Vec::new();
        let mut owns_files = false;
        let mut contracts = Vec::new();
        let mut raw_fields = Vec::new();
        let mut children = Vec::new();
        while !self.consume_kind(&TokenKind::CloseBrace) {
            if self.at_eof() {
                return Err(ParseError::unexpected(
                    self.peek().span.clone(),
                    "`}`",
                    "end of input",
                ));
            }
            if self.peek_node_kind().is_some() {
                children.push(self.node()?);
                continue;
            }
            let field_span = self.peek().span.clone();
            let field_name = self.expect_word("field name")?;
            match field_name.as_str() {
                "path" => paths.extend(self.field_values()?),
                "contract" => contracts.extend(self.field_values()?),
                "owns-files" => {
                    self.expect_kind(&TokenKind::Colon, "`:`")?;
                    let value = self.expect_word("boolean")?;
                    owns_files = value == "true";
                }
                _ => {
                    let values = self.field_values()?;
                    raw_fields.push(Field {
                        name: field_name,
                        values,
                        span: field_span,
                    });
                }
            }
        }
        Ok(Node {
            kind,
            name,
            description,
            id,
            tags,
            paths,
            owns_files,
            contracts,
            raw_fields,
            children,
            span: kind_token.span,
        })
    }

    fn edge(&mut self) -> Result<Edge, ParseError> {
        let span = self.peek().span.clone();
        let from = self.expect_word("edge source id")?;
        self.expect_kind(&TokenKind::Arrow, "`->`")?;
        let to = self.expect_word("edge target id")?;
        let description = self.expect_string("edge description")?;
        Ok(Edge {
            from,
            to,
            description,
            span,
        })
    }

    fn field_values(&mut self) -> Result<Vec<String>, ParseError> {
        if self.consume_kind(&TokenKind::OpenBracket) {
            let mut values = Vec::new();
            while !self.consume_kind(&TokenKind::CloseBracket) {
                values.push(self.expect_string("list item string")?);
                let _ = self.consume_kind(&TokenKind::Comma);
            }
            Ok(values)
        } else {
            Ok(vec![self.expect_string("field value string")?])
        }
    }

    fn expect_word_exact(&mut self, word: &str) -> Result<(), ParseError> {
        let token = self.next().clone();
        match token.kind {
            TokenKind::Word(value) if value == word => Ok(()),
            kind => Err(ParseError::unexpected(token.span, word, describe(&kind))),
        }
    }

    fn expect_word(&mut self, expected: &str) -> Result<String, ParseError> {
        let token = self.next().clone();
        match token.kind {
            TokenKind::Word(value) => Ok(value),
            kind => Err(ParseError::unexpected(
                token.span,
                expected,
                describe(&kind),
            )),
        }
    }

    fn expect_string(&mut self, expected: &str) -> Result<String, ParseError> {
        let token = self.next().clone();
        match token.kind {
            TokenKind::String(value) => Ok(value),
            kind => Err(ParseError::unexpected(
                token.span,
                expected,
                describe(&kind),
            )),
        }
    }

    fn expect_kind(&mut self, kind: &TokenKind, expected: &str) -> Result<(), ParseError> {
        let token = self.next().clone();
        if &token.kind == kind {
            Ok(())
        } else {
            Err(ParseError::unexpected(
                token.span,
                expected,
                describe(&token.kind),
            ))
        }
    }

    fn consume_kind(&mut self, kind: &TokenKind) -> bool {
        if &self.peek().kind == kind {
            self.next();
            true
        } else {
            false
        }
    }

    fn peek_node_kind(&self) -> Option<NodeKind> {
        node_kind(self.peek())
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn next(&mut self) -> &Token {
        let token = &self.tokens[self.index];
        if token.kind != TokenKind::Eof {
            self.index += 1;
        }
        token
    }
}

fn node_kind(token: &Token) -> Option<NodeKind> {
    match &token.kind {
        TokenKind::Word(value) if value == "System" => Some(NodeKind::System),
        TokenKind::Word(value) if value == "Container" => Some(NodeKind::Container),
        TokenKind::Word(value) if value == "Module" => Some(NodeKind::Module),
        TokenKind::Word(value) if value == "Actor" => Some(NodeKind::Actor),
        _ => None,
    }
}

fn describe(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Word(value) => format!("word `{value}`"),
        TokenKind::String(value) => format!("string `{value}`"),
        TokenKind::Tag(value) => format!("tag `@{value}`"),
        TokenKind::OpenBrace => "`{`".to_owned(),
        TokenKind::CloseBrace => "`}`".to_owned(),
        TokenKind::OpenBracket => "`[`".to_owned(),
        TokenKind::CloseBracket => "`]`".to_owned(),
        TokenKind::Comma => "`,`".to_owned(),
        TokenKind::Colon => "`:`".to_owned(),
        TokenKind::Arrow => "`->`".to_owned(),
        TokenKind::Eof => "end of input".to_owned(),
    }
}
