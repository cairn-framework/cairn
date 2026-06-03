//! Integration tests for the blueprint recursive-descent parser.
//!
//! `parse_str` is called on every project load, delta application, and
//! architecture hook check. It had zero dedicated tests.

use cairn::blueprint::{NodeKind, ast::Ast, parser::parse_str};

// ── helpers ───────────────────────────────────────────────────────────────────

fn ok(src: &str) -> Ast {
    parse_str("test.blueprint", src).expect("valid blueprint must parse without error")
}

fn err(src: &str) -> String {
    parse_str("test.blueprint", src)
        .expect_err("invalid blueprint must return error")
        .message
        .to_string()
}

// ── empty source ──────────────────────────────────────────────────────────────

#[test]
fn test_empty_source_produces_empty_ast() {
    let ast = ok("");
    assert!(ast.nodes.is_empty());
    assert!(ast.edges.is_empty());
}

// ── node kinds ────────────────────────────────────────────────────────────────

#[test]
fn test_module_node_parsed() {
    let ast = ok(r#"Module Api "API module" id "app.api" {}"#);
    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0].kind, NodeKind::Module);
    assert_eq!(ast.nodes[0].name, "Api");
    assert_eq!(ast.nodes[0].description, "API module");
    assert_eq!(ast.nodes[0].id, "app.api");
}

#[test]
fn test_system_node_parsed() {
    let ast = ok(r#"System Cairn "Architecture tool" id "cairn" {}"#);
    assert_eq!(ast.nodes[0].kind, NodeKind::System);
    assert_eq!(ast.nodes[0].id, "cairn");
}

#[test]
fn test_container_node_parsed() {
    let ast = ok(r#"Container Backend "Server processes" id "backend" {}"#);
    assert_eq!(ast.nodes[0].kind, NodeKind::Container);
}

#[test]
fn test_actor_node_parsed() {
    let ast = ok(r#"Actor User "End user" id "actor.user" {}"#);
    assert_eq!(ast.nodes[0].kind, NodeKind::Actor);
}

// ── node fields ───────────────────────────────────────────────────────────────

#[test]
fn test_node_with_single_path() {
    let ast = ok(r#"Module Api "desc" id "app.api" { path "./src/api" }"#);
    assert_eq!(ast.nodes[0].paths, vec!["./src/api"]);
}

#[test]
fn test_node_with_multiple_paths_list_syntax() {
    let ast = ok(r#"Module Api "desc" id "app.api" { path ["./src/api", "./lib/api"] }"#);
    assert_eq!(ast.nodes[0].paths, vec!["./src/api", "./lib/api"]);
}

#[test]
fn test_node_with_single_tag() {
    let ast = ok(r#"Module Api "desc" id "app.api" @public {}"#);
    assert_eq!(ast.nodes[0].tags, vec!["public"]);
}

#[test]
fn test_node_with_multiple_tags() {
    let ast = ok(r#"Module Api "desc" id "app.api" @public @versioned {}"#);
    assert_eq!(ast.nodes[0].tags, vec!["public", "versioned"]);
}

#[test]
fn test_node_with_contract() {
    let ast = ok(r#"Module Api "desc" id "app.api" { contract "meta/contracts/api.md" }"#);
    assert_eq!(ast.nodes[0].contracts, vec!["meta/contracts/api.md"]);
}

#[test]
fn test_node_with_multiple_separate_path_declarations() {
    // Two `path` declarations in one node body must accumulate into one vec.
    let ast = ok(r#"Module Api "desc" id "app.api" {
    path "./src/api"
    path "./lib/api"
}"#);
    assert_eq!(
        ast.nodes[0].paths,
        vec!["./src/api", "./lib/api"],
        "multiple path declarations must be additive"
    );
}

#[test]
fn test_empty_path_list_produces_no_paths() {
    // `path []` is syntactically valid but contributes nothing.
    let ast = ok(r#"Module Api "desc" id "app.api" { path [] }"#);
    assert!(
        ast.nodes[0].paths.is_empty(),
        "empty path list must produce no path entries"
    );
}

#[test]
fn test_node_with_owns_files_true() {
    let ast = ok(r#"Container Backend "desc" id "backend" { owns-files: true }"#);
    assert!(ast.nodes[0].owns_files);
}

#[test]
fn test_node_with_owns_files_false() {
    let ast = ok(r#"Container Backend "desc" id "backend" { owns-files: false }"#);
    assert!(!ast.nodes[0].owns_files);
}

#[test]
fn test_node_with_raw_field() {
    let ast = ok(r#"Module Api "desc" id "app.api" { status "stable" }"#);
    assert_eq!(ast.nodes[0].raw_fields.len(), 1);
    assert_eq!(ast.nodes[0].raw_fields[0].name, "status");
    assert_eq!(ast.nodes[0].raw_fields[0].values, vec!["stable"]);
}

#[test]
fn test_node_with_multiple_raw_fields() {
    let ast = ok(r#"Module Api "desc" id "app.api" { status "stable" tier "1" }"#);
    assert_eq!(ast.nodes[0].raw_fields.len(), 2);
}

// ── nested nodes ──────────────────────────────────────────────────────────────

#[test]
fn test_nested_child_node() {
    let ast = ok(r#"Container Backend "desc" id "backend" {
    Module Api "API" id "backend.api" {}
}"#);
    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0].children.len(), 1);
    assert_eq!(ast.nodes[0].children[0].id, "backend.api");
    assert_eq!(ast.nodes[0].children[0].kind, NodeKind::Module);
}

#[test]
fn test_deeply_nested_nodes() {
    let ast = ok(r#"System App "desc" id "app" {
    Container Api "API Container" id "app.api" {
        Module Handler "Handler" id "app.api.handler" {}
    }
}"#);
    assert_eq!(ast.nodes[0].children.len(), 1);
    assert_eq!(ast.nodes[0].children[0].children.len(), 1);
    assert_eq!(ast.nodes[0].children[0].children[0].id, "app.api.handler");
}

// ── edges ─────────────────────────────────────────────────────────────────────

#[test]
fn test_single_edge_parsed() {
    let ast = ok(r#"app.api -> app.db "queries database""#);
    assert_eq!(ast.edges.len(), 1);
    assert_eq!(ast.edges[0].from, "app.api");
    assert_eq!(ast.edges[0].to, "app.db");
    assert_eq!(ast.edges[0].description, "queries database");
}

#[test]
fn test_multiple_edges_parsed() {
    let ast = ok(r#"app.api -> app.db "queries"
app.api -> app.cache "reads"
app.db -> app.log "writes""#);
    assert_eq!(ast.edges.len(), 3);
}

// ── mixed nodes and edges ─────────────────────────────────────────────────────

#[test]
fn test_nodes_and_edges_in_one_source() {
    let ast = ok(r#"Module Api "desc" id "app.api" {}
Module Db "desc" id "app.db" {}
app.api -> app.db "queries"
"#);
    assert_eq!(ast.nodes.len(), 2);
    assert_eq!(ast.edges.len(), 1);
}

// ── span information ──────────────────────────────────────────────────────────

#[test]
fn test_node_span_file_matches_input() {
    let ast = parse_str("my.blueprint", r#"Module Api "desc" id "app.api" {}"#).unwrap();
    assert_eq!(ast.nodes[0].span.file, "my.blueprint");
}

#[test]
fn test_edge_span_line_is_correct() {
    let src = "Module Api \"desc\" id \"app.api\" {}\napp.api -> app.db \"queries\"\n";
    let ast = ok(src);
    // Edge is on line 2.
    assert_eq!(ast.edges[0].span.line, 2);
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_unclosed_brace_is_error() {
    let msg = err(r#"Module Api "desc" id "app.api" {"#);
    assert!(
        !msg.is_empty(),
        "unclosed brace must produce a non-empty error message"
    );
}

#[test]
fn test_missing_id_keyword_is_error() {
    // "app.api" where `id` keyword is expected.
    let msg = err(r#"Module Api "desc" "app.api" {}"#);
    assert!(
        msg.contains("id") || msg.contains("word"),
        "missing `id` keyword must mention the expectation; got: {msg}"
    );
}

#[test]
fn test_unknown_node_kind_is_edge_not_node() {
    // "Unknown" is not a node keyword so the parser attempts to parse an edge.
    let result = parse_str("test.blueprint", r#"Unknown Foo "desc" id "x" {}"#);
    assert!(
        result.is_err(),
        "unrecognised node kind must not parse as a valid node or edge"
    );
}

#[test]
fn test_edge_missing_arrow_is_error() {
    let msg = err(r#"app.api app.db "queries""#);
    assert!(
        msg.contains("->") || msg.contains("arrow") || msg.contains("->"),
        "missing -> must produce error mentioning arrow; got: {msg}"
    );
}
