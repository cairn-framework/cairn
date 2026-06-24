//! Tests for blueprint delta application and AST serialization helpers.

use super::*;
use crate::blueprint::{NodeKind, Span};

fn span() -> Span {
    Span::point("test", 1, 1)
}

fn leaf(id: &str) -> Node {
    Node {
        kind: NodeKind::Module,
        id: id.to_owned(),
        name: id.to_owned(),
        description: String::new(),
        tags: Vec::new(),
        paths: Vec::new(),
        owns_files: false,
        contracts: Vec::new(),
        raw_fields: Vec::new(),
        children: Vec::new(),
        span: span(),
    }
}

fn mk_edge(from: &str, to: &str, desc: &str) -> Edge {
    Edge {
        from: from.to_owned(),
        to: to.to_owned(),
        description: desc.to_owned(),
        span: span(),
    }
}

// ── same_edge ─────────────────────────────────────────────────────────────

#[test]
fn test_same_edge_equal() {
    let e = mk_edge("a", "b", "calls");
    assert!(same_edge(&e, &e.clone()));
}

#[test]
fn test_same_edge_different_description_is_not_equal() {
    let e1 = mk_edge("a", "b", "calls");
    let e2 = mk_edge("a", "b", "queries");
    assert!(!same_edge(&e1, &e2));
}

#[test]
fn test_same_edge_different_from_is_not_equal() {
    assert!(!same_edge(&mk_edge("a", "b", "c"), &mk_edge("x", "b", "c")));
}

// ── replace_exact_id ──────────────────────────────────────────────────────

#[test]
fn test_replace_exact_id_exact_match() {
    assert_eq!(replace_exact_id("app.api", "app.api", "app.new"), "app.new");
}

#[test]
fn test_replace_exact_id_no_substring_replacement() {
    // "app" must not be replaced inside "app.api".
    assert_eq!(replace_exact_id("app.api", "app", "x"), "app.api");
}

#[test]
fn test_replace_exact_id_no_match() {
    assert_eq!(replace_exact_id("app.api", "other", "x"), "app.api");
}

// ── node_kind_name ────────────────────────────────────────────────────────

#[test]
fn test_node_kind_name_all_variants() {
    assert_eq!(node_kind_name(NodeKind::System), "System");
    assert_eq!(node_kind_name(NodeKind::Container), "Container");
    assert_eq!(node_kind_name(NodeKind::Module), "Module");
    assert_eq!(node_kind_name(NodeKind::Actor), "Actor");
}

// ── serialize_field_values ────────────────────────────────────────────────

#[test]
fn test_serialize_field_values_empty_produces_brackets() {
    assert_eq!(serialize_field_values(&[]), "[]");
}

#[test]
fn test_serialize_field_values_single_produces_quoted_string() {
    let v = vec!["src/api".to_owned()];
    assert_eq!(serialize_field_values(&v), r#""src/api""#);
}

#[test]
fn test_serialize_field_values_multiple_produces_bracketed_list() {
    let v = vec!["a".to_owned(), "b".to_owned()];
    assert_eq!(serialize_field_values(&v), r#"["a", "b"]"#);
}

// ── serialize_node ────────────────────────────────────────────────────────

#[test]
fn test_serialize_node_basic_structure() {
    let n = Node {
        kind: NodeKind::Module,
        id: "app.api".to_owned(),
        name: "Api".to_owned(),
        description: "The API".to_owned(),
        tags: vec!["public".to_owned()],
        ..leaf("app.api")
    };
    let mut out = String::new();
    serialize_node(&n, 0, &mut out);
    assert!(out.contains("Module Api"), "kind and name: {out:?}");
    assert!(out.contains(r#""The API""#), "description quoted: {out:?}");
    assert!(out.contains(r#"id "app.api""#), "id quoted: {out:?}");
    assert!(out.contains("@public"), "tag present: {out:?}");
    assert!(out.starts_with("Module"), "no indent at level 0: {out:?}");
}

#[test]
fn test_serialize_node_with_path_emits_path_line() {
    let n = Node {
        paths: vec!["src/api".to_owned()],
        ..leaf("app.api")
    };
    let mut out = String::new();
    serialize_node(&n, 0, &mut out);
    assert!(out.contains(r#"path "src/api""#), "path line: {out:?}");
}

#[test]
fn test_serialize_node_owns_files_true_emits_flag() {
    let n = Node {
        owns_files: true,
        ..leaf("app.api")
    };
    let mut out = String::new();
    serialize_node(&n, 0, &mut out);
    assert!(out.contains("owns-files: true"), "owns-files flag: {out:?}");
}

#[test]
fn test_serialize_node_owns_files_false_does_not_emit_flag() {
    let mut out = String::new();
    serialize_node(&leaf("app.api"), 0, &mut out);
    assert!(
        !out.contains("owns-files"),
        "no owns-files when false: {out:?}"
    );
}

#[test]
fn test_serialize_node_indent_applied_at_nonzero_level() {
    let mut out = String::new();
    serialize_node(&leaf("a"), 4, &mut out);
    assert!(
        out.starts_with("    "),
        "4-space indent at level 4: {out:?}"
    );
}

// ── rename_node_id ────────────────────────────────────────────────────────

#[test]
fn test_rename_node_id_renames_top_level_node() {
    let mut nodes = vec![leaf("old")];
    rename_node_id(&mut nodes, "old", "new");
    assert_eq!(nodes[0].id, "new");
}

#[test]
fn test_rename_node_id_renames_nested_child() {
    let child = leaf("child");
    let mut parent = leaf("parent");
    parent.children = vec![child];
    let mut nodes = vec![parent];
    rename_node_id(&mut nodes, "child", "renamed");
    assert_eq!(nodes[0].children[0].id, "renamed");
}

#[test]
fn test_rename_node_id_no_match_is_noop() {
    let mut nodes = vec![leaf("a")];
    rename_node_id(&mut nodes, "missing", "x");
    assert_eq!(nodes[0].id, "a");
}

// ── remove_node ───────────────────────────────────────────────────────────

#[test]
fn test_remove_node_removes_top_level() {
    let mut nodes = vec![leaf("a"), leaf("b")];
    remove_node(&mut nodes, "a");
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].id, "b");
}

#[test]
fn test_remove_node_removes_nested_child() {
    let mut parent = leaf("parent");
    parent.children = vec![leaf("child")];
    let mut nodes = vec![parent];
    remove_node(&mut nodes, "child");
    assert!(nodes[0].children.is_empty(), "child must be removed");
}

#[test]
fn test_remove_node_no_match_is_noop() {
    let mut nodes = vec![leaf("a")];
    remove_node(&mut nodes, "missing");
    assert_eq!(nodes.len(), 1);
}

// ── replace_node ──────────────────────────────────────────────────────────

#[test]
fn test_replace_node_replaces_top_level() {
    let replacement = Node {
        description: "replaced".to_owned(),
        ..leaf("a")
    };
    let mut nodes = vec![leaf("a")];
    replace_node(&mut nodes, &replacement).unwrap();
    assert_eq!(nodes[0].description, "replaced");
}

#[test]
fn test_replace_node_replaces_nested_child() {
    let replacement = Node {
        description: "new desc".to_owned(),
        ..leaf("child")
    };
    let mut parent = leaf("parent");
    parent.children = vec![leaf("child")];
    let mut nodes = vec![parent];
    replace_node(&mut nodes, &replacement).unwrap();
    assert_eq!(nodes[0].children[0].description, "new desc");
}

#[test]
fn test_replace_node_not_found_returns_err() {
    let mut nodes = vec![leaf("a")];
    let result = replace_node(&mut nodes, &leaf("missing"));
    assert!(result.is_err(), "not-found must return Err");
    let msg = result.unwrap_err();
    assert!(msg.contains("missing"), "error message names the id: {msg}");
}

// ── strip_change_frontmatter ──────────────────────────────────────────────

#[test]
fn test_strip_frontmatter_no_frontmatter_passes_through() {
    let src = "# Title\n\nbody text\n";
    let result = strip_change_frontmatter(src);
    assert!(result.contains("# Title"));
    assert!(result.contains("body text"));
}

#[test]
fn test_strip_frontmatter_removes_operation_field() {
    let src = "---\noperation: add\ntitle: foo\n---\nbody\n";
    let result = strip_change_frontmatter(src);
    assert!(
        !result.contains("operation: add"),
        "operation line stripped: {result:?}"
    );
    assert!(
        result.contains("title: foo"),
        "other fields kept: {result:?}"
    );
    assert!(result.contains("body"), "body kept: {result:?}");
}

#[test]
fn test_strip_frontmatter_removes_renamed_from_field() {
    let src = "---\noperation: rename\nrenamed_from: old.md\ntitle: bar\n---\nbody\n";
    let result = strip_change_frontmatter(src);
    assert!(
        !result.contains("renamed_from:"),
        "renamed_from stripped: {result:?}"
    );
    assert!(result.contains("title: bar"), "title kept: {result:?}");
}

#[test]
fn test_strip_frontmatter_operation_in_body_not_stripped() {
    // A body line starting with "operation:" must not be removed.
    // Only lines inside the frontmatter block are candidates for stripping.
    let src = "---\ntitle: foo\n---\noperation: something\n";
    let result = strip_change_frontmatter(src);
    assert!(
        result.contains("operation: something"),
        "body line must not be stripped: {result:?}"
    );
}

#[test]
fn test_strip_frontmatter_always_ends_with_newline() {
    // Input with no trailing newline must still produce one.
    let result = strip_change_frontmatter("no newline at end");
    assert!(result.ends_with('\n'), "must end with newline: {result:?}");
}

// ── apply_archive blueprint preservation ──────────────────────────────────

fn change_with_delta(delta: BlueprintDelta) -> Change {
    Change {
        id: "noop".to_owned(),
        path: PathBuf::from("meta/changes/noop"),
        title: "noop".to_owned(),
        proposal: String::new(),
        design: None,
        delta,
        artefacts: Vec::new(),
        findings: Vec::new(),
    }
}

const COMMENTED_BLUEPRINT: &str = "# header comment\n#\n# second line\n\nSystem App \"desc\" id \"app\" {\n    # inner comment\n    Module Core \"core\" id \"app.core\" {\n        path \"./src/core\"\n    }\n}\n";

#[test]
fn test_apply_archive_empty_delta_preserves_blueprint_verbatim() {
    let dir = tempfile::tempdir().unwrap();
    let blueprint = dir.path().join("cairn.blueprint");
    fs::write(&blueprint, COMMENTED_BLUEPRINT).unwrap();

    apply_archive(
        dir.path(),
        Path::new("cairn.blueprint"),
        &change_with_delta(BlueprintDelta::default()),
    )
    .expect("empty-delta archive must succeed");

    let after = fs::read_to_string(&blueprint).unwrap();
    assert_eq!(
        after, COMMENTED_BLUEPRINT,
        "empty-delta archive must leave the blueprint byte-identical, comments included"
    );
}

#[test]
fn test_apply_archive_nonempty_delta_preserves_comments() {
    let dir = tempfile::tempdir().unwrap();
    let blueprint = dir.path().join("cairn.blueprint");
    fs::write(&blueprint, COMMENTED_BLUEPRINT).unwrap();

    let mut delta = BlueprintDelta::default();
    delta.removed_nodes.push("app.core".to_owned());
    apply_archive(
        dir.path(),
        Path::new("cairn.blueprint"),
        &change_with_delta(delta),
    )
    .expect("non-empty-delta archive must succeed");

    let after = fs::read_to_string(&blueprint).unwrap();
    assert!(
        !after.contains("app.core"),
        "removed node must be absent: {after:?}"
    );
    assert!(
        after.starts_with("# header comment\n#\n# second line\n\n"),
        "header comments and blank line must survive a structural delta: {after:?}"
    );
    assert!(
        after.contains("# inner comment"),
        "comments outside the removed declaration must be preserved: {after:?}"
    );
}

#[test]
fn test_apply_blueprint_delta_modify_preserves_siblings() {
    let base = "# top comment\nSystem App \"d\" id \"app\" {\n    Module A \"a\" id \"app.a\" {\n        path \"./a\"\n    }\n\n    # section B\n    Module B \"b\" id \"app.b\" {\n        path \"./b\"\n    }\n}\n";
    let mut modified = leaf("app.a");
    modified.name = "A".to_owned();
    modified.description = "modified".to_owned();
    let mut delta = BlueprintDelta::default();
    delta.modified_nodes.push(modified);

    let result = apply_blueprint_delta(base, &delta).expect("modify delta must apply");

    assert!(
        result.contains("# top comment"),
        "leading comment preserved: {result:?}"
    );
    assert!(
        result.contains("\n\n    # section B"),
        "blank line and inter-sibling comment preserved: {result:?}"
    );
    assert!(
        result.contains("Module A \"modified\" id \"app.a\""),
        "modified node reflects the change: {result:?}"
    );
    assert!(
        result.contains("    Module B \"b\" id \"app.b\" {\n        path \"./b\"\n    }"),
        "untouched sibling kept verbatim: {result:?}"
    );
}

#[test]
fn test_apply_blueprint_delta_remove_edge_preserves_section_comment() {
    let base = "System App \"d\" id \"app\" {\n    Module A \"a\" id \"app.a\" {}\n    Module B \"b\" id \"app.b\" {}\n}\n\n# wiring\napp.a -> app.b \"calls\"\napp.b -> app.a \"replies\"\n";
    let mut delta = BlueprintDelta::default();
    delta.removed_edges.push(mk_edge("app.a", "app.b", "calls"));

    let result = apply_blueprint_delta(base, &delta).expect("edge-removal delta must apply");

    assert!(
        result.contains("# wiring"),
        "edge-section comment preserved: {result:?}"
    );
    assert!(
        result.contains("app.b -> app.a \"replies\""),
        "surviving edge kept verbatim: {result:?}"
    );
    assert!(
        !result.contains("app.a -> app.b"),
        "removed edge absent: {result:?}"
    );
    assert!(
        result.contains("    Module A \"a\" id \"app.a\" {}"),
        "untouched nodes kept verbatim: {result:?}"
    );
}

#[test]
fn test_apply_blueprint_delta_modify_preserves_trivia_at_depth_three() {
    // Mirrors the real cairn.blueprint shape: System > Container > Module.
    let base = "# header\nSystem S \"s\" id \"s\" {\n    Container K \"k\" id \"s.k\" {\n        Module A \"a\" id \"s.k.a\" {\n            path \"./a\"\n        }\n\n        # inner section\n        Module B \"b\" id \"s.k.b\" {\n            path \"./b\"\n        }\n    }\n}\n";
    let mut modified = leaf("s.k.a");
    modified.name = "A".to_owned();
    modified.description = "modified".to_owned();
    let mut delta = BlueprintDelta::default();
    delta.modified_nodes.push(modified);

    let result = apply_blueprint_delta(base, &delta).expect("nested modify must apply");

    assert!(
        result.contains("# header"),
        "top-level comment preserved: {result:?}"
    );
    assert!(
        result.contains("\n\n        # inner section"),
        "blank line and depth-2 section comment preserved: {result:?}"
    );
    assert!(
        result.contains("Module A \"modified\" id \"s.k.a\""),
        "deeply nested modified node reflects the change: {result:?}"
    );
    assert!(
        result
            .contains("        Module B \"b\" id \"s.k.b\" {\n            path \"./b\"\n        }"),
        "untouched depth-3 sibling kept verbatim: {result:?}"
    );
}

#[test]
fn test_apply_blueprint_delta_two_edges_one_line_falls_back_correctly() {
    // Two edges share a source line (legal but unconventional). The line-wise
    // splice cannot apply a per-edge op here, so the verified fallback must
    // still produce structurally correct output.
    let base = "System App \"d\" id \"app\" {}\nx -> y \"a\" p -> q \"b\"\n";
    let mut delta = BlueprintDelta::default();
    delta.removed_edges.push(mk_edge("x", "y", "a"));

    let result = apply_blueprint_delta(base, &delta).expect("delta must apply");

    assert!(
        !result.contains("x -> y"),
        "removed edge must be gone: {result:?}"
    );
    assert!(
        result.contains("p -> q \"b\""),
        "surviving edge must remain: {result:?}"
    );
}

#[test]
fn test_apply_blueprint_delta_two_nodes_one_line_falls_back_correctly() {
    // Two declarations share a source line; the splice would skip a sibling, so
    // the verified fallback must still drop only the removed node.
    let base = "System App \"d\" id \"app\" { Module A \"a\" id \"app.a\" {} Module B \"b\" id \"app.b\" {} }\n";
    let mut delta = BlueprintDelta::default();
    delta.removed_nodes.push("app.a".to_owned());

    let result = apply_blueprint_delta(base, &delta).expect("delta must apply");

    assert!(
        !result.contains("app.a"),
        "removed node must be gone: {result:?}"
    );
    assert!(
        result.contains("id \"app.b\""),
        "surviving sibling must remain: {result:?}"
    );
}

#[test]
fn test_blueprint_delta_is_empty() {
    assert!(BlueprintDelta::default().is_empty());
    let mut delta = BlueprintDelta::default();
    delta.added_nodes.push(leaf("app.new"));
    assert!(!delta.is_empty());
}
