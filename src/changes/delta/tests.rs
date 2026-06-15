//! Tests for the blueprint delta parser.

use crate::blueprint::{Node, NodeKind, Span};

use super::*;

// ── delta_sections ──────────────────────────────────────────────────────────

#[test]
fn test_delta_sections_empty_source() {
    let sections = delta_sections("");
    assert!(sections.is_empty());
}

#[test]
fn test_delta_sections_extracts_named_sections() {
    let src = "## ADDED Nodes\nmodule foo\n\n## REMOVED Nodes\n- bar\n";
    let sections = delta_sections(src);
    assert!(
        sections.contains_key("ADDED Nodes"),
        "ADDED Nodes section must be present"
    );
    assert!(
        sections.contains_key("REMOVED Nodes"),
        "REMOVED Nodes section must be present"
    );
}

#[test]
fn test_delta_sections_content_before_first_heading_is_ignored() {
    let src = "preamble text\n## ADDED Nodes\nmodule foo\n";
    let sections = delta_sections(src);
    assert_eq!(sections.len(), 1);
    // preamble must not appear in section content
    assert!(!sections["ADDED Nodes"].contains("preamble"));
}

#[test]
fn test_delta_sections_captures_section_lines() {
    let src = "## REMOVED Nodes\n- app.old\n- app.gone\n";
    let sections = delta_sections(src);
    let content = &sections["REMOVED Nodes"];
    assert!(content.contains("app.old"));
    assert!(content.contains("app.gone"));
}

#[test]
fn test_delta_sections_empty_section_present_as_empty_string() {
    let src = "## ADDED Nodes\n## REMOVED Nodes\n- x\n";
    let sections = delta_sections(src);
    assert_eq!(sections["ADDED Nodes"].trim(), "");
    assert!(sections["REMOVED Nodes"].contains('x'));
}

// ── clean_list_line ─────────────────────────────────────────────────────────

#[test]
fn test_clean_list_line_strips_bullet_prefix() {
    assert_eq!(clean_list_line("- item"), "item");
}

#[test]
fn test_clean_list_line_trims_surrounding_whitespace() {
    assert_eq!(clean_list_line("  - item  "), "item");
}

#[test]
fn test_clean_list_line_no_prefix_kept_as_is() {
    assert_eq!(clean_list_line("app.foo"), "app.foo");
}

#[test]
fn test_clean_list_line_empty_bullet_yields_empty() {
    assert_eq!(clean_list_line("- "), "");
}

#[test]
fn test_clean_list_line_empty_string_yields_empty() {
    assert_eq!(clean_list_line(""), "");
}

// ── clean_scalar ────────────────────────────────────────────────────────────

#[test]
fn test_clean_scalar_strips_backticks() {
    assert_eq!(clean_scalar("`app.foo`"), "app.foo");
}

#[test]
fn test_clean_scalar_strips_double_quotes() {
    assert_eq!(clean_scalar("\"app.foo\""), "app.foo");
}

#[test]
fn test_clean_scalar_strips_single_quotes() {
    assert_eq!(clean_scalar("'app.foo'"), "app.foo");
}

#[test]
fn test_clean_scalar_trims_whitespace() {
    assert_eq!(clean_scalar("  app.foo  "), "app.foo");
}

#[test]
fn test_clean_scalar_plain_value_unchanged() {
    assert_eq!(clean_scalar("app.foo"), "app.foo");
}

#[test]
fn test_clean_scalar_strips_all_quote_styles() {
    assert_eq!(clean_scalar("`id`"), "id");
    assert_eq!(clean_scalar("\"id\""), "id");
    assert_eq!(clean_scalar("'id'"), "id");
}

// ── uncomment_lines ─────────────────────────────────────────────────────────

#[test]
fn test_uncomment_lines_filters_blank_lines() {
    let section = "- a\n\n- b\n";
    let result = uncomment_lines(section);
    assert_eq!(result, "a\nb");
}

// ── parse_id_lines ──────────────────────────────────────────────────────────

#[test]
fn test_parse_id_lines_none_gives_empty() {
    assert!(parse_id_lines(None).is_empty());
}

#[test]
fn test_parse_id_lines_strips_bullets_and_filters_empty() {
    let ids = parse_id_lines(Some("- app.old\n\n- app.gone\n"));
    assert_eq!(ids, vec!["app.old", "app.gone"]);
}

#[test]
fn test_parse_id_lines_plain_ids_without_bullets() {
    let ids = parse_id_lines(Some("app.a\napp.b\n"));
    assert_eq!(ids, vec!["app.a", "app.b"]);
}

// ── parse_rename_lines ──────────────────────────────────────────────────────

#[test]
fn test_parse_rename_lines_none_gives_empty() {
    assert!(parse_rename_lines(None).unwrap().is_empty());
}

#[test]
fn test_parse_rename_lines_happy_path() {
    let renames = parse_rename_lines(Some("- app.auth -> app.authentication\n"))
        .expect("valid rename must parse");
    assert_eq!(renames.len(), 1);
    assert_eq!(renames[0].from, "app.auth");
    assert_eq!(renames[0].to, "app.authentication");
}

#[test]
fn test_parse_rename_lines_backtick_quoted_ids() {
    let renames = parse_rename_lines(Some("- `app.old` -> `app.new`\n"))
        .expect("backtick-quoted rename must parse");
    assert_eq!(renames[0].from, "app.old");
    assert_eq!(renames[0].to, "app.new");
}

#[test]
fn test_parse_rename_lines_missing_arrow_is_error() {
    let result = parse_rename_lines(Some("- app.old app.new\n"));
    assert!(result.is_err(), "missing '->' must produce error");
    assert!(result.unwrap_err().contains("malformed rename"));
}

#[test]
fn test_parse_rename_lines_rejects_missing_arrow() {
    let err = parse_rename_lines(Some("app.a app.b")).unwrap_err();
    assert!(err.contains("malformed rename operation"));
}

// ── flatten_nodes ───────────────────────────────────────────────────────────

#[test]
fn test_flatten_nodes_empty() {
    assert!(flatten_nodes(vec![]).is_empty());
}

#[test]
fn test_flatten_nodes_no_children() {
    let node = Node {
        kind: NodeKind::Module,
        name: "foo".to_owned(),
        description: String::new(),
        id: "app.foo".to_owned(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: Vec::new(),
        span: Span::point("delta.md", 1, 1),
    };
    let flat = flatten_nodes(vec![node]);
    assert_eq!(flat.len(), 1);
    assert_eq!(flat[0].id, "app.foo");
}

#[test]
fn test_flatten_nodes_nested_children_included() {
    let child = Node {
        kind: NodeKind::Module,
        name: "child".to_owned(),
        description: String::new(),
        id: "app.child".to_owned(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: Vec::new(),
        span: Span::point("delta.md", 2, 1),
    };
    let parent = Node {
        kind: NodeKind::Container,
        name: "parent".to_owned(),
        description: String::new(),
        id: "app.parent".to_owned(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: vec![child],
        span: Span::point("delta.md", 1, 1),
    };
    let flat = flatten_nodes(vec![parent]);
    let ids: Vec<&str> = flat.iter().map(|n| n.id.as_str()).collect();
    assert_eq!(
        ids,
        vec!["app.parent", "app.child"],
        "parent must precede its children in the flattened list"
    );
}

// ── parse_blueprint_delta (end-to-end) ──────────────────────────────────────

#[test]
fn test_parse_delta_empty_document() {
    let delta = parse_blueprint_delta("test.md", "").expect("empty doc must parse");
    assert!(delta.added_nodes.is_empty());
    assert!(delta.removed_nodes.is_empty());
    assert!(delta.added_edges.is_empty());
}

#[test]
fn test_parse_delta_removed_nodes_section() {
    let src = "## REMOVED Nodes\n- app.old\n- app.gone\n";
    let delta = parse_blueprint_delta("test.md", src).expect("must parse");
    assert_eq!(delta.removed_nodes, vec!["app.old", "app.gone"]);
}

#[test]
fn test_parse_delta_renamed_nodes_section() {
    let src = "## RENAMED Nodes\n- app.auth -> app.authentication\n";
    let delta = parse_blueprint_delta("test.md", src).expect("must parse");
    assert_eq!(delta.renamed_nodes.len(), 1);
    assert_eq!(delta.renamed_nodes[0].from, "app.auth");
    assert_eq!(delta.renamed_nodes[0].to, "app.authentication");
}

#[test]
fn test_parse_delta_added_nodes_section() {
    // Node declarations use explicit `id` syntax: Kind Name "desc" id "dot.id" {}
    let src = "## ADDED Nodes\nModule New \"New module\" id \"app.new\" {}\n";
    let delta = parse_blueprint_delta("test.md", src).expect("must parse");
    assert_eq!(delta.added_nodes.len(), 1);
    assert_eq!(delta.added_nodes[0].id, "app.new");
}

#[test]
fn test_parse_delta_node_section_with_edges_is_error() {
    let src = "## ADDED Nodes\napp.a -> app.b \"uses\"\n";
    let result = parse_blueprint_delta("test.md", src);
    assert!(result.is_err(), "edge syntax in node section must error");
    assert!(result.unwrap_err().contains("edge operations"));
}

#[test]
fn test_parse_delta_added_edges_section() {
    let src = "## ADDED Edges\napp.api -> app.db \"queries\"\n";
    let delta = parse_blueprint_delta("test.md", src).expect("must parse");
    assert_eq!(delta.added_edges.len(), 1);
    assert_eq!(delta.added_edges[0].from, "app.api");
    assert_eq!(delta.added_edges[0].to, "app.db");
    assert_eq!(delta.added_edges[0].description, "queries");
}

#[test]
fn test_parse_delta_multiple_sections() {
    let src = concat!(
        "## ADDED Nodes\nModule New \"New\" id \"app.new\" {}\n",
        "## REMOVED Nodes\n- app.old\n",
        "## ADDED Edges\napp.new -> app.existing \"uses\"\n",
    );
    let delta = parse_blueprint_delta("test.md", src).expect("must parse");
    assert_eq!(delta.added_nodes.len(), 1);
    assert_eq!(delta.removed_nodes, vec!["app.old"]);
    assert_eq!(delta.added_edges.len(), 1);
}

#[test]
fn test_parse_delta_malformed_rename_is_error() {
    let src = "## RENAMED Nodes\n- app.old app.new\n";
    let result = parse_blueprint_delta("test.md", src);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("malformed rename"));
}

// ── edge cases ──────────────────────────────────────────────────────────────

#[test]
fn test_parse_node_section_rejects_edge_operations() {
    let section = "app.a -> app.b \"desc\"";
    let err = parse_node_section("delta.md", Some(section)).unwrap_err();
    assert!(err.contains("node delta section contains edge operations"));
}

#[test]
fn test_parse_edge_section_rejects_node_operations() {
    let section = "Module m \"M\" id \"m\" {}";
    let err = parse_edge_section("delta.md", Some(section)).unwrap_err();
    assert!(err.contains("edge delta section contains node operations"));
}

#[test]
fn test_parse_edge_renames_with_multiple_source_edges_fails() {
    let section = "app.a -> app.b => app.x -> app.y";
    assert!(parse_edge_renames("delta.md", Some(section)).is_err());
}

#[test]
fn test_parse_edge_renames_with_multiple_target_edges_fails() {
    let section = "app.a -> app.b => app.x -> app.y, app.z -> app.w";
    assert!(parse_edge_renames("delta.md", Some(section)).is_err());
}

#[test]
fn test_parse_edge_renames_rejects_missing_rename_arrow() {
    let section = "app.a -> app.b app.x -> app.y";
    let err = parse_edge_renames("delta.md", Some(section)).unwrap_err();
    assert!(err.contains("malformed edge rename operation"));
}

#[test]
fn test_parse_blueprint_delta_multiple_sections_round_trip() {
    let src = "## ADDED Nodes\nModule m \"M\" id \"m\" {}\n## REMOVED Nodes\n- old\n## ADDED Edges\nm -> old \"x\"\n";
    let delta = parse_blueprint_delta("delta.md", src).unwrap();
    assert_eq!(delta.added_nodes.len(), 1);
    assert_eq!(delta.removed_nodes, vec!["old"]);
    assert_eq!(delta.added_edges.len(), 1);
}
