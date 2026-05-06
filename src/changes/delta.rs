//! Parser for `blueprint.delta` documents into structured node and edge operations.
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
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix("- ").unwrap_or(trimmed);
    trimmed.trim().to_owned()
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
