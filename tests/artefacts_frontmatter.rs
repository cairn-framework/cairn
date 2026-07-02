//! Tests for the Markdown frontmatter parser (`src/artefacts/frontmatter.rs`).
//!
//! The parser is called on every decision, research, contract, and artefact
//! file. Regressions here silently corrupt scanner findings and UI metadata.

use cairn::artefacts::frontmatter::{self, Frontmatter};

/// Converts `Option<&Vec<String>>` to `Option<Vec<&str>>` for concise assertions.
fn list<'a>(fm: &'a Frontmatter, key: &str) -> Option<Vec<&'a str>> {
    fm.lists
        .get(key)
        .map(|v| v.iter().map(String::as_str).collect())
}

// ── No frontmatter ────────────────────────────────────────────────────────────

#[test]
fn test_no_frontmatter_body_is_full_source() {
    let src = "# Title\n\nSome body text.\n";
    let fm = frontmatter::parse(src);
    assert!(
        fm.values.is_empty(),
        "no frontmatter → values must be empty"
    );
    assert!(fm.lists.is_empty(), "no frontmatter → lists must be empty");
    assert_eq!(fm.body, src, "no frontmatter → body is full source");
}

#[test]
fn test_no_frontmatter_when_dashes_not_first_line() {
    let src = "\n---\nkey: val\n---\nbody";
    let fm = frontmatter::parse(src);
    // Leading blank line means it's not a frontmatter block.
    assert!(fm.values.is_empty());
    assert_eq!(fm.body, src);
}

// ── Empty frontmatter ─────────────────────────────────────────────────────────

#[test]
fn test_empty_frontmatter_block() {
    let src = "---\n---\nBody text.";
    let fm = frontmatter::parse(src);
    assert!(fm.values.is_empty());
    assert!(fm.lists.is_empty());
    assert_eq!(fm.body, "Body text.");
}

// ── Simple key-value pairs ────────────────────────────────────────────────────

#[test]
fn test_simple_key_value() {
    let src = "---\ntitle: My Decision\nstatus: accepted\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.values.get("title").map(String::as_str),
        Some("My Decision")
    );
    assert_eq!(
        fm.values.get("status").map(String::as_str),
        Some("accepted")
    );
}

#[test]
fn test_key_with_colon_in_value() {
    // split_once(':') must stop at the first colon — not at subsequent ones.
    let src = "---\nurl: http://example.com/path\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.values.get("url").map(String::as_str),
        Some("http://example.com/path"),
        "value containing ':' must be preserved in full"
    );
}

#[test]
fn test_quoted_value_strips_double_quotes() {
    let src = "---\ntitle: \"My Decision\"\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.values.get("title").map(String::as_str),
        Some("My Decision")
    );
}

#[test]
fn test_quoted_value_strips_single_quotes() {
    let src = "---\ntitle: 'My Decision'\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.values.get("title").map(String::as_str),
        Some("My Decision")
    );
}

#[test]
fn test_inline_comment_stripped_from_value() {
    let src = "---\nstatus: proposed # pending review\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.values.get("status").map(String::as_str),
        Some("proposed"),
        "inline comment must be stripped from value"
    );
}

// ── Block lists ───────────────────────────────────────────────────────────────

#[test]
fn test_block_list_simple_items() {
    let src = "---\ntags:\n- architecture\n- backend\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        list(&fm, "tags"),
        Some(vec!["architecture", "backend"]),
        "block list items must be collected under their key"
    );
}

#[test]
fn test_block_list_id_field_items() {
    // items with `id:` sub-field are extracted by their ID value
    let src = "---\ndependencies:\n- id: db\n- id: cache\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        list(&fm, "dependencies"),
        Some(vec!["db", "cache"]),
        "block list `- id: X` items must be collected by their id value"
    );
}

#[test]
fn test_block_list_quoted_item_with_colon_is_plain_scalar() {
    // A quoted item that happens to contain a colon (e.g. a Rust function
    // signature) must be kept whole, not misparsed as an `id:` sub-field pair.
    let src = "---\ninterface:\n  - \"fn handle(a: &str) -> String\"\n  - \"fn other()\"\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        list(&fm, "interface"),
        Some(vec!["fn handle(a: &str) -> String", "fn other()"]),
        "quoted list items containing a colon must not be dropped or split"
    );
}

#[test]
fn test_block_list_inline_comment_stripped() {
    let src = "---\ntags:\n- arch # main\n- db\n---\n";
    let fm = frontmatter::parse(src);
    let tags = fm.lists.get("tags").expect("tags list must exist");
    assert_eq!(
        tags[0], "arch",
        "inline comment in list item must be stripped"
    );
    assert_eq!(tags[1], "db");
}

#[test]
fn test_block_list_new_key_resets_active_list() {
    // After a block-list key, starting a new scalar key must not add to the list.
    let src = "---\ntags:\n- foo\nstatus: accepted\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(list(&fm, "tags"), Some(vec!["foo"]));
    assert_eq!(
        fm.values.get("status").map(String::as_str),
        Some("accepted")
    );
}

// ── Inline lists ──────────────────────────────────────────────────────────────

#[test]
fn test_inline_list_parsed_into_lists() {
    let src = "---\nphases: [discovery, design, implement]\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        list(&fm, "phases"),
        Some(vec!["discovery", "design", "implement"]),
        "inline `[a, b, c]` must be parsed into the lists map"
    );
}

#[test]
fn test_inline_list_also_stored_in_values() {
    // The raw bracketed string is preserved in values alongside the parsed list.
    let src = "---\nphases: [a, b]\n---\n";
    let fm = frontmatter::parse(src);
    assert!(
        fm.values.contains_key("phases"),
        "inline list key must also appear in values"
    );
}

#[test]
fn test_inline_list_empty_brackets() {
    let src = "---\ntags: []\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        list(&fm, "tags"),
        Some(vec![] as Vec<&str>),
        "empty inline list must produce an empty vec"
    );
}

#[test]
fn test_inline_list_single_item() {
    let src = "---\ntags: [only]\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(list(&fm, "tags"), Some(vec!["only"]));
}

// ── Body preservation ─────────────────────────────────────────────────────────

#[test]
fn test_body_after_closing_dashes() {
    let src = "---\ntitle: T\n---\n# Heading\n\nParagraph.\n";
    let fm = frontmatter::parse(src);
    // body.join("\n") does not re-add the trailing newline that lines() strips.
    assert_eq!(fm.body, "# Heading\n\nParagraph.");
}

#[test]
fn test_body_is_empty_when_only_frontmatter() {
    let src = "---\ntitle: T\n---\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.body, "",
        "no content after closing dashes → body is empty"
    );
}

#[test]
fn test_body_with_dashes_is_not_re_parsed() {
    // A `---` inside the body must not be treated as another frontmatter boundary.
    let src = "---\ntitle: T\n---\nBody line 1\n---\nBody line 2";
    let fm = frontmatter::parse(src);
    assert!(
        fm.body.contains("---"),
        "literal `---` in body must be preserved, not consumed as frontmatter"
    );
}

// ── Unclosed frontmatter ──────────────────────────────────────────────────────

#[test]
fn test_unclosed_frontmatter_parses_keys() {
    // No closing `---`: parser treats everything as frontmatter, body is empty.
    let src = "---\ntitle: T\nstatus: open\n";
    let fm = frontmatter::parse(src);
    assert_eq!(
        fm.values.get("title").map(String::as_str),
        Some("T"),
        "unclosed frontmatter: keys before EOF must still be parsed"
    );
    assert_eq!(fm.values.get("status").map(String::as_str), Some("open"),);
    assert_eq!(fm.body, "", "unclosed frontmatter produces empty body");
}

// ── Equality and clone ────────────────────────────────────────────────────────

#[test]
fn test_frontmatter_equality_and_clone() {
    let src = "---\nkey: val\n---\nbody";
    let a = frontmatter::parse(src);
    let b = a.clone();
    assert_eq!(a, b);
}
