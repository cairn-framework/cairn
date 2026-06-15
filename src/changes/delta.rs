//! Blueprint delta document parser.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::*;

/// Parses a `blueprint.delta` document.
///
/// # Errors
///
/// Returns a human-readable parse error when a section contains invalid syntax.
pub fn parse_blueprint_delta(file: &str, source: &str) -> Result<BlueprintDelta, String> {
    let sections = delta_sections(source);
    Ok(BlueprintDelta {
        added_nodes: parse_node_section(file, sections.get("ADDED Nodes").map(String::as_str))?,
        modified_nodes: parse_node_section(
            file,
            sections.get("MODIFIED Nodes").map(String::as_str),
        )?,
        removed_nodes: parse_id_lines(sections.get("REMOVED Nodes").map(String::as_str)),
        renamed_nodes: parse_rename_lines(sections.get("RENAMED Nodes").map(String::as_str))?,
        added_edges: parse_edge_section(file, sections.get("ADDED Edges").map(String::as_str))?,
        modified_edges: parse_edge_section(
            file,
            sections.get("MODIFIED Edges").map(String::as_str),
        )?,
        removed_edges: parse_edge_section(file, sections.get("REMOVED Edges").map(String::as_str))?,
        renamed_edges: parse_edge_renames(file, sections.get("RENAMED Edges").map(String::as_str))?,
    })
}

pub(super) fn delta_sections(source: &str) -> BTreeMap<String, String> {
    let mut sections = BTreeMap::<String, String>::new();
    let mut current: Option<String> = None;
    for line in source.lines() {
        if let Some(title) = line.trim().strip_prefix("## ") {
            current = Some(title.trim().to_owned());
            sections.entry(title.trim().to_owned()).or_default();
            continue;
        }
        if let Some(title) = &current {
            sections.entry(title.clone()).or_default().push_str(line);
            sections.entry(title.clone()).or_default().push('\n');
        }
    }
    sections
}

pub(super) fn parse_node_section(file: &str, section: Option<&str>) -> Result<Vec<Node>, String> {
    let Some(section) = section else {
        return Ok(Vec::new());
    };
    let source = uncomment_lines(section);
    if source.trim().is_empty() {
        return Ok(Vec::new());
    }
    let ast = parse_str(file, &source).map_err(|error| error.to_string())?;
    if !ast.edges.is_empty() {
        return Err(format!(
            "{file}: node delta section contains edge operations"
        ));
    }
    Ok(flatten_nodes(ast.nodes))
}

pub(super) fn parse_edge_section(file: &str, section: Option<&str>) -> Result<Vec<Edge>, String> {
    let Some(section) = section else {
        return Ok(Vec::new());
    };
    let source = uncomment_lines(section);
    if source.trim().is_empty() {
        return Ok(Vec::new());
    }
    let ast = parse_str(file, &source).map_err(|error| error.to_string())?;
    if !ast.nodes.is_empty() {
        return Err(format!(
            "{file}: edge delta section contains node operations"
        ));
    }
    Ok(ast.edges)
}

pub(super) fn parse_id_lines(section: Option<&str>) -> Vec<String> {
    section
        .into_iter()
        .flat_map(str::lines)
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .collect()
}

pub(super) fn parse_rename_lines(section: Option<&str>) -> Result<Vec<Rename>, String> {
    section
        .into_iter()
        .flat_map(str::lines)
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let Some((from, to)) = line.split_once("->") else {
                return Err(format!("malformed rename operation `{line}`"));
            };
            Ok(Rename {
                from: clean_scalar(from),
                to: clean_scalar(to),
            })
        })
        .collect()
}

pub(super) fn parse_edge_renames(
    file: &str,
    section: Option<&str>,
) -> Result<Vec<EdgeRename>, String> {
    section
        .into_iter()
        .flat_map(str::lines)
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let Some((from, to)) = line.split_once("=>") else {
                return Err(format!("malformed edge rename operation `{line}`"));
            };
            let from_edges = parse_edge_section(file, Some(from))?;
            let to_edges = parse_edge_section(file, Some(to))?;
            let [from_edge] = from_edges.as_slice() else {
                return Err(format!(
                    "edge rename source must contain one edge: `{line}`"
                ));
            };
            let [to_edge] = to_edges.as_slice() else {
                return Err(format!(
                    "edge rename target must contain one edge: `{line}`"
                ));
            };
            Ok(EdgeRename {
                from: from_edge.clone(),
                to: to_edge.clone(),
            })
        })
        .collect()
}

pub(super) fn uncomment_lines(source: &str) -> String {
    source
        .lines()
        .map(clean_list_line)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn clean_list_line(line: &str) -> String {
    // trim_start() only: preserves the space that is part of the "- " prefix,
    // so "- " (bullet with no content) strips to "" rather than leaving "-".
    let leading_stripped = line.trim_start();
    let content = leading_stripped
        .strip_prefix("- ")
        .unwrap_or(leading_stripped);
    content.trim_end().to_owned()
}

pub(super) fn clean_scalar(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches('\'')
        .to_owned()
}

pub(super) fn flatten_nodes(nodes: Vec<Node>) -> Vec<Node> {
    let mut flattened = Vec::new();
    for node in nodes {
        flattened.push(node.clone());
        flattened.extend(flatten_nodes(node.children));
    }
    flattened
}
#[cfg(test)]
mod tests {
    use super::*;

    // ── delta_sections ────────────────────────────────────────────────────────

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

    // ── clean_list_line ───────────────────────────────────────────────────────

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

    // ── clean_scalar ──────────────────────────────────────────────────────────

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

    // ── parse_id_lines ────────────────────────────────────────────────────────

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

    // ── parse_rename_lines ────────────────────────────────────────────────────

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

    // ── flatten_nodes ─────────────────────────────────────────────────────────

    #[test]
    fn test_flatten_nodes_empty() {
        assert!(flatten_nodes(vec![]).is_empty());
    }

    #[test]
    fn test_flatten_nodes_no_children() {
        use crate::blueprint::{NodeKind, Span};
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
        use crate::blueprint::{NodeKind, Span};
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

    // ── parse_blueprint_delta (end-to-end) ────────────────────────────────────

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
}
