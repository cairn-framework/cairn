//! Integration tests for the blueprint hand-written lexer (`src/blueprint/lexer.rs`).
//!
//! The lexer is the character-level foundation for all blueprint parsing.
//! Bugs here propagate silently through the parser into every feature that
//! reads a cairn.blueprint file.

use cairn::blueprint::lexer::{TokenKind, tokenize};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Extract token kinds from a source string, panicking on lex error.
fn kinds(src: &str) -> Vec<TokenKind> {
    tokenize("test", src)
        .expect("tokenize must succeed")
        .into_iter()
        .map(|t| t.kind)
        .collect()
}

/// Assert exact token sequence including the terminal Eof.
macro_rules! assert_kinds {
    ($src:expr, [$($kind:expr),* $(,)?]) => {{
        let got = kinds($src);
        let expected: Vec<TokenKind> = vec![$($kind,)* TokenKind::Eof];
        assert_eq!(got, expected, "unexpected token sequence for {:?}", $src);
    }};
}

// ── punctuation and delimiters ────────────────────────────────────────────────

#[test]
fn test_empty_source_produces_only_eof() {
    assert_kinds!("", []);
}

#[test]
fn test_open_brace() {
    assert_kinds!("{", [TokenKind::OpenBrace]);
}

#[test]
fn test_close_brace() {
    assert_kinds!("}", [TokenKind::CloseBrace]);
}

#[test]
fn test_open_bracket() {
    assert_kinds!("[", [TokenKind::OpenBracket]);
}

#[test]
fn test_close_bracket() {
    assert_kinds!("]", [TokenKind::CloseBracket]);
}

#[test]
fn test_comma() {
    assert_kinds!(",", [TokenKind::Comma]);
}

#[test]
fn test_colon() {
    assert_kinds!(":", [TokenKind::Colon]);
}

#[test]
fn test_arrow() {
    assert_kinds!("->", [TokenKind::Arrow]);
}

// ── words ─────────────────────────────────────────────────────────────────────

#[test]
fn test_simple_word() {
    assert_kinds!("Module", [TokenKind::Word("Module".to_owned())]);
}

#[test]
fn test_word_with_dot() {
    assert_kinds!("app.api", [TokenKind::Word("app.api".to_owned())]);
}

#[test]
fn test_word_with_hyphen() {
    // owns-files is a field name containing a hyphen.
    assert_kinds!("owns-files", [TokenKind::Word("owns-files".to_owned())]);
}

#[test]
fn test_word_stops_before_arrow() {
    // "foo->bar" must lex as three tokens, not one word.
    assert_kinds!(
        "foo->bar",
        [
            TokenKind::Word("foo".to_owned()),
            TokenKind::Arrow,
            TokenKind::Word("bar".to_owned()),
        ]
    );
}

#[test]
fn test_word_with_trailing_hyphen_not_confused_with_arrow() {
    // "foo-" — hyphen at end with no `>` after it stays in the word.
    assert_kinds!("foo-", [TokenKind::Word("foo-".to_owned())]);
}

#[test]
fn test_word_stops_at_open_brace() {
    // No space: the brace terminates the word token.
    assert_kinds!(
        "foo{",
        [TokenKind::Word("foo".to_owned()), TokenKind::OpenBrace]
    );
}

#[test]
fn test_word_stops_at_colon() {
    assert_kinds!(
        "key:",
        [TokenKind::Word("key".to_owned()), TokenKind::Colon]
    );
}

// ── strings ───────────────────────────────────────────────────────────────────

#[test]
fn test_simple_string() {
    assert_kinds!(
        r#""hello world""#,
        [TokenKind::String("hello world".to_owned())]
    );
}

#[test]
fn test_empty_string() {
    assert_kinds!(r#""""#, [TokenKind::String(String::new())]);
}

#[test]
fn test_string_with_escaped_quote() {
    // \" inside a string produces a literal double-quote character.
    assert_kinds!(
        r#""say \"hi\"""#,
        [TokenKind::String(r#"say "hi""#.to_owned())]
    );
}

#[test]
fn test_string_with_backslash_n_produces_literal_n() {
    // The lexer does not interpret \n as newline — it pushes the char after \.
    // This is by design: blueprint strings are descriptions, not code literals.
    assert_kinds!(
        r#""line\nbreak""#,
        [TokenKind::String("linenbreak".to_owned())]
    );
}

#[test]
fn test_unterminated_string_is_error() {
    let result = tokenize("test", r#""unterminated"#);
    assert!(result.is_err(), "unterminated string must produce an error");
    let err = result.unwrap_err();
    assert_eq!(err.code, "CAIRN_PARSE_UNTERMINATED_STRING");
}

// ── tags ──────────────────────────────────────────────────────────────────────

#[test]
fn test_simple_tag() {
    assert_kinds!("@public", [TokenKind::Tag("public".to_owned())]);
}

#[test]
fn test_tag_with_hyphen() {
    assert_kinds!("@my-tag", [TokenKind::Tag("my-tag".to_owned())]);
}

#[test]
fn test_multiple_tags() {
    assert_kinds!(
        "@a @b",
        [
            TokenKind::Tag("a".to_owned()),
            TokenKind::Tag("b".to_owned()),
        ]
    );
}

// ── whitespace and comments ───────────────────────────────────────────────────

#[test]
fn test_whitespace_between_tokens_is_skipped() {
    assert_kinds!(
        "  foo   bar  ",
        [
            TokenKind::Word("foo".to_owned()),
            TokenKind::Word("bar".to_owned()),
        ]
    );
}

#[test]
fn test_comment_to_end_of_line_is_skipped() {
    // Everything from # to \n is stripped; "bar" on the next line is kept.
    assert_kinds!(
        "foo # this is a comment\nbar",
        [
            TokenKind::Word("foo".to_owned()),
            TokenKind::Word("bar".to_owned()),
        ]
    );
}

#[test]
fn test_comment_only_source_produces_only_eof() {
    assert_kinds!("# full line comment\n# another", []);
}

// ── span tracking ─────────────────────────────────────────────────────────────

#[test]
fn test_token_span_file_is_propagated() {
    let tokens = tokenize("my.blueprint", "foo").unwrap();
    assert_eq!(tokens[0].span.file, "my.blueprint");
}

#[test]
fn test_token_span_line_starts_at_one() {
    let tokens = tokenize("test", "foo").unwrap();
    assert_eq!(tokens[0].span.line, 1);
}

#[test]
fn test_token_span_line_increments_after_newline() {
    let tokens = tokenize("test", "foo\nbar").unwrap();
    // "foo" is on line 1, "bar" on line 2.
    assert_eq!(tokens[0].span.line, 1);
    assert_eq!(tokens[1].span.line, 2);
}

#[test]
fn test_token_span_column_starts_at_one() {
    let tokens = tokenize("test", "foo").unwrap();
    assert_eq!(tokens[0].span.column, 1);
}

// ── full blueprint fragment ───────────────────────────────────────────────────

#[test]
fn test_full_node_declaration_token_sequence() {
    let src = r#"Module Api "API module" id "app.api" {}"#;
    let k = kinds(src);
    // Expected: Module, Api, "API module", id, "app.api", {, }, Eof
    assert_eq!(
        k,
        vec![
            TokenKind::Word("Module".to_owned()),
            TokenKind::Word("Api".to_owned()),
            TokenKind::String("API module".to_owned()),
            TokenKind::Word("id".to_owned()),
            TokenKind::String("app.api".to_owned()),
            TokenKind::OpenBrace,
            TokenKind::CloseBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_edge_token_sequence() {
    let src = r#"app.api -> app.db "queries""#;
    let k = kinds(src);
    assert_eq!(
        k,
        vec![
            TokenKind::Word("app.api".to_owned()),
            TokenKind::Arrow,
            TokenKind::Word("app.db".to_owned()),
            TokenKind::String("queries".to_owned()),
            TokenKind::Eof,
        ]
    );
}
