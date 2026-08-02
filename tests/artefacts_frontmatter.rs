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

// ── Surgical field edits (upsert/remove) ──────────────────────────────────────

#[test]
fn test_upsert_field_inserts_before_closing_fence() {
    let source = "---\nnode: app\nstatus: open\n---\n\n# T\n\nBody.\n";
    let updated = frontmatter::upsert_field(source, "blocked_by", "[todo.a]").unwrap();
    assert_eq!(
        updated,
        "---\nnode: app\nstatus: open\nblocked_by: [todo.a]\n---\n\n# T\n\nBody.\n"
    );
}

#[test]
fn test_upsert_field_replaces_existing_value() {
    let source = "---\nnode: app\nparent: todo.old\nstatus: open\n---\nBody\n";
    let updated = frontmatter::upsert_field(source, "parent", "todo.new").unwrap();
    assert_eq!(
        updated,
        "---\nnode: app\nparent: todo.new\nstatus: open\n---\nBody\n"
    );
}

#[test]
fn test_upsert_field_collapses_block_list_without_orphans() {
    let source = "---\nnode: app\nblocked_by:\n  - todo.a\n  - todo.b\nstatus: open\n---\nBody\n";
    let updated = frontmatter::upsert_field(source, "blocked_by", "[todo.c]").unwrap();
    assert_eq!(
        updated, "---\nnode: app\nblocked_by: [todo.c]\nstatus: open\n---\nBody\n",
        "block-list items must not survive as orphans"
    );
}

#[test]
fn test_upsert_field_preserves_crlf_line_endings() {
    let source = "---\r\nnode: app\r\nstatus: open\r\n---\r\nBody\r\n";
    let updated = frontmatter::upsert_field(source, "parent", "todo.x").unwrap();
    assert_eq!(
        updated,
        "---\r\nnode: app\r\nstatus: open\r\nparent: todo.x\r\n---\r\nBody\r\n"
    );
}

#[test]
fn test_upsert_field_no_frontmatter_errors() {
    let result = frontmatter::upsert_field("# Just a body\n", "parent", "todo.x");
    assert_eq!(result, Err(frontmatter::SetFieldError::NoFrontmatter));
}

#[test]
fn test_remove_field_drops_scalar_line_only() {
    let source = "---\nnode: app\nparent: todo.x\nstatus: open\n---\nBody\n";
    let updated = frontmatter::remove_field(source, "parent").unwrap();
    assert_eq!(updated, "---\nnode: app\nstatus: open\n---\nBody\n");
}

#[test]
fn test_remove_field_drops_block_list_extent() {
    let source = "---\nnode: app\nblocked_by:\n  - todo.a\n  - todo.b\nstatus: open\n---\nBody\n";
    let updated = frontmatter::remove_field(source, "blocked_by").unwrap();
    assert_eq!(updated, "---\nnode: app\nstatus: open\n---\nBody\n");
}

#[test]
fn test_remove_field_missing_key_errors() {
    let source = "---\nnode: app\n---\nBody\n";
    assert_eq!(
        frontmatter::remove_field(source, "parent"),
        Err(frontmatter::SetFieldError::KeyNotFound)
    );
}

#[test]
fn test_remove_field_ignores_nested_key() {
    let source = "---\nnode: app\ndetails:\n  parent: nested\n---\nBody\n";
    assert_eq!(
        frontmatter::remove_field(source, "parent"),
        Err(frontmatter::SetFieldError::KeyNotFound),
        "an indented key is not a top-level match"
    );
}

#[test]
fn test_remove_field_keeps_trailing_blank_separator() {
    // A blank line after a block list separates it from the next key; it
    // is not a member and must survive the removal.
    let source = "---\nnode: app\nblocked_by:\n  - todo.a\n\nstatus: open\n---\nBody\n";
    let updated = frontmatter::remove_field(source, "blocked_by").unwrap();
    assert_eq!(updated, "---\nnode: app\n\nstatus: open\n---\nBody\n");
}

#[test]
fn test_remove_field_consumes_blank_inside_block() {
    // A blank line followed by another indented member belongs to the
    // block and goes with it.
    let source = "---\nnode: app\nblocked_by:\n  - todo.a\n\n  - todo.b\nstatus: open\n---\nBody\n";
    let updated = frontmatter::remove_field(source, "blocked_by").unwrap();
    assert_eq!(updated, "---\nnode: app\nstatus: open\n---\nBody\n");
}

#[test]
fn test_upsert_field_replacement_keeps_own_line_ending() {
    // Mixed-ending document: the replaced line keeps ITS ending, not the
    // closing fence's.
    let source = "---\nnode: app\r\nstatus: open\n---\nBody\n";
    let updated = frontmatter::upsert_field(source, "node", "other").unwrap();
    assert_eq!(updated, "---\nnode: other\r\nstatus: open\n---\nBody\n");
    let updated = frontmatter::upsert_field(source, "status", "done").unwrap();
    assert_eq!(updated, "---\nnode: app\r\nstatus: done\n---\nBody\n");
}

#[test]
fn test_upsert_field_insert_keeps_crlf_without_trailing_newline() {
    // The closing-fence element carries no \r when the document ends at
    // the fence; the inserted line must still follow the document style.
    let source = "---\r\nnode: app\r\n---";
    let updated = frontmatter::upsert_field(source, "parent", "todo.x").unwrap();
    assert_eq!(updated, "---\r\nnode: app\r\nparent: todo.x\r\n---");
}

#[test]
fn test_remove_field_keeps_whitespace_only_separator() {
    // A separator holding only spaces is a separator, not a block member.
    let source = "---\nnode: app\nblocked_by:\n  - todo.a\n   \nstatus: open\n---\nBody\n";
    let updated = frontmatter::remove_field(source, "blocked_by").unwrap();
    assert_eq!(updated, "---\nnode: app\n   \nstatus: open\n---\nBody\n");
}
