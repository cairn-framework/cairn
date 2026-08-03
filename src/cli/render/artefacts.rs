//! Artefact query renderers (todos, decisions, research, sources, rationale).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{
    decision_index, decision_line_with_index, decisions_json, flag_value, lines, node_arg,
    positional_node, reverse_provenance_lines,
};
use super::super::*;
use crate::query_api::{QueryRequest, parse_decision_status_filter};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn render_todos(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    // The node is optional: bare `cairn todos` (with or without leading
    // flags such as `--status`) lists todos project-wide.
    let node = positional_node(&parsed.command_args).cloned();
    let status = flag_value(&parsed.command_args, "--status").map(ToOwned::to_owned);
    let request = QueryRequest {
        tool: "todos".to_owned(),
        node,
        symbol: None,
        change: None,
        old_id: None,
        new_id: None,
        status,
        language: None,
        flags: BTreeSet::new(),
        mutating: false,
    };
    let data = crate::query_api::execute(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    Ok(todos_text(&data))
}
/// Renders the canonical `todos_response_json` data as human text. A null
/// `node` marks a project-wide listing. Each todo's relationship edges
/// render one per line beneath it, naming edge kind and target with the
/// target's current status when the response resolves one.
fn todos_text(data: &Value) -> String {
    let heading = data["node"].as_str().map_or_else(
        || "Todos (project-wide):".to_owned(),
        |node_id| format!("Todos for {node_id}:"),
    );
    let statuses = &data["relation_statuses"];
    let mut todo_lines: Vec<String> = Vec::new();
    for value in data["todos"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
    {
        // Edges join their todo's element so the bullet renderer treats
        // them as indented continuation lines, not peer todos.
        let mut block = format!(
            "{} [{}] {}",
            value["node"].as_str().unwrap_or_default(),
            value["status"].as_str().unwrap_or_default(),
            value["path"].as_str().unwrap_or_default(),
        );
        for (kind, target) in edge_entries(value) {
            let _ = match statuses.get(&target).and_then(Value::as_str) {
                Some(status) => write!(block, "\n  {kind}: {target} ({status})"),
                None => write!(block, "\n  {kind}: {target}"),
            };
        }
        todo_lines.push(block);
    }
    format!("{heading}\n{}\n", lines(&todo_lines))
}

/// Ordered relationship edges on one todo wire object: `blocked_by`
/// entries, then `parent`, then `related`.
fn edge_entries(todo: &Value) -> Vec<(&'static str, String)> {
    let mut edges = Vec::new();
    let list = |value: &Value| -> Vec<String> {
        value
            .as_array()
            .map_or(&[][..], std::ops::Deref::deref)
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect()
    };
    for target in list(&todo["blocked_by"]) {
        edges.push(("blocked_by", target));
    }
    if let Some(parent) = todo["parent"].as_str() {
        edges.push(("parent", parent.to_owned()));
    }
    for target in list(&todo["related"]) {
        edges.push(("related", target));
    }
    edges
}

pub(crate) fn render_decisions(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let status =
        flag_value(&parsed.command_args, "--status").and_then(parse_decision_status_filter);
    if let Some(query) = flag_value(&parsed.command_args, "--grep") {
        return Ok(render_decisions_grep(parsed, scan_result, query));
    }
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.nodes.contains(&node.id)
                    && status.is_none_or(|filter| decision.status == filter)
            })
            .cloned()
            .collect::<Vec<_>>();
        let index = decision_index(&scan_result.artefacts.decisions);
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"decisions\":{}}}\n",
                esc(&node.id),
                decisions_json(&decisions)
            )
        } else {
            format!(
                "Decisions for {}:\n{}\n",
                node.id,
                lines(
                    &decisions
                        .iter()
                        .map(|decision| decision_line_with_index(decision, &index))
                        .collect::<Vec<_>>(),
                )
            )
        })
    })
}

fn render_decisions_grep(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
    query: &str,
) -> String {
    let status =
        flag_value(&parsed.command_args, "--status").and_then(parse_decision_status_filter);
    let needle = query.to_lowercase();
    let matches = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| {
            status.is_none_or(|filter| decision.status == filter)
                && (decision.id.to_lowercase().contains(&needle)
                    || decision.body.to_lowercase().contains(&needle)
                    || decision
                        .nodes
                        .iter()
                        .any(|node| node.to_lowercase().contains(&needle)))
        })
        .cloned()
        .collect::<Vec<_>>();
    let index = decision_index(&scan_result.artefacts.decisions);
    if parsed.json {
        format!(
            "{{\"query\":\"{}\",\"decisions\":{}}}\n",
            esc(query),
            decisions_json(&matches)
        )
    } else {
        format!(
            "Decisions matching \"{}\":\n{}\n",
            query,
            lines(
                &matches
                    .iter()
                    .map(|decision| decision_line_with_index(decision, &index))
                    .collect::<Vec<_>>(),
            )
        )
    }
}

pub(crate) fn render_research(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    let node = node_arg(&parsed.command_args)?;
    let request = QueryRequest {
        tool: "research".to_owned(),
        node: Some(node.to_owned()),
        symbol: None,
        change: None,
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags: BTreeSet::new(),
        mutating: false,
    };
    let data = crate::query_api::execute(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    Ok(research_text(&data))
}
/// Renders the canonical `research_response_json` data as human text.
fn research_text(data: &Value) -> String {
    let node_id = data["node"].as_str().unwrap_or_default();
    let research_lines: Vec<String> = data["research"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
        .iter()
        .map(|value| {
            let sources: Vec<String> = value["sources"]
                .as_array()
                .map_or(&[][..], std::ops::Deref::deref)
                .iter()
                .map(|source| source.as_str().unwrap_or_default().to_owned())
                .collect();
            format!(
                "{} sources: {}",
                value["id"].as_str().unwrap_or_default(),
                sources.join(", ")
            )
        })
        .collect();
    format!("Research for {node_id}:\n{}\n", lines(&research_lines))
}
pub(crate) fn render_sources(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    let node = node_arg(&parsed.command_args)?;
    let request = QueryRequest {
        tool: "sources".to_owned(),
        node: Some(node.to_owned()),
        symbol: None,
        change: None,
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags: BTreeSet::new(),
        mutating: false,
    };
    let data = crate::query_api::execute(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    Ok(sources_text(&data))
}
/// Renders the canonical `sources_response_json` data as human text.
fn sources_text(data: &Value) -> String {
    let node_id = data["node"].as_str().unwrap_or_default();
    let sources_lines: Vec<String> = data["sources"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
        .iter()
        .map(|value| {
            format!(
                "{} [{}] {}",
                value["id"].as_str().unwrap_or_default(),
                value["verification"].as_str().unwrap_or_default(),
                value["file"].as_str().unwrap_or_default()
            )
        })
        .collect();
    format!("Sources for {node_id}:\n{}\n", lines(&sources_lines))
}

pub(crate) fn render_rationale(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    let node = node_arg(&parsed.command_args)?;
    let request = QueryRequest {
        tool: "rationale".to_owned(),
        node: Some(node.to_owned()),
        symbol: None,
        change: None,
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags: BTreeSet::new(),
        mutating: false,
    };
    let data = crate::query_api::execute(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    Ok(rationale_text(&data))
}
/// Renders the canonical `rationale_json` data as human text.
fn rationale_text(data: &Value) -> String {
    let node_id = data["node"].as_str().unwrap_or_default();

    let index = data["decision_index"]
        .as_object()
        .map_or_else(BTreeMap::new, |entries| {
            entries
                .iter()
                .filter_map(|(id, value)| {
                    Some((
                        id.clone(),
                        (
                            value["status"].as_str()?.to_owned(),
                            value["date"].as_str()?.to_owned(),
                        ),
                    ))
                })
                .collect::<BTreeMap<_, _>>()
        });
    let decisions_lines: Vec<String> = data["decisions"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
        .iter()
        .map(|value| {
            let via = value["via"]
                .as_array()
                .map_or(&[][..], std::ops::Deref::deref)
                .iter()
                .map(|node| node.as_str().unwrap_or_default().to_owned())
                .collect::<Vec<_>>();
            let via_suffix = if via.is_empty() {
                String::new()
            } else {
                format!(" (via {})", via.join(", "))
            };
            let mut line = format!(
                "{} [{}] {}{via_suffix}",
                value["id"].as_str().unwrap_or_default(),
                value["status"].as_str().unwrap_or_default(),
                value["path"].as_str().unwrap_or_default()
            );
            let refined_by = value["refined_by"]
                .as_array()
                .map_or(&[][..], std::ops::Deref::deref)
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            let superseded_by = value["superseded_by"]
                .as_array()
                .map_or(&[][..], std::ops::Deref::deref)
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            line.push_str(&reverse_provenance_lines(
                &refined_by,
                &superseded_by,
                &index,
            ));
            line
        })
        .collect();
    let research_lines: Vec<String> = data["research"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
        .iter()
        .map(|value| {
            let sources: Vec<String> = value["sources"]
                .as_array()
                .map_or(&[][..], std::ops::Deref::deref)
                .iter()
                .map(|source| source.as_str().unwrap_or_default().to_owned())
                .collect();
            format!(
                "{} sources: {}",
                value["id"].as_str().unwrap_or_default(),
                sources.join(", ")
            )
        })
        .collect();

    let sources_lines: Vec<String> = data["sources"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
        .iter()
        .map(|value| {
            format!(
                "{} [{}] {}",
                value["id"].as_str().unwrap_or_default(),
                value["verification"].as_str().unwrap_or_default(),
                value["file"].as_str().unwrap_or_default()
            )
        })
        .collect();

    format!(
        "Rationale for {node_id}:\nDecisions:\n{}\nResearch:\n{}\nSources:\n{}\n",
        lines(&decisions_lines),
        lines(&research_lines),
        lines(&sources_lines)
    )
}

#[cfg(test)]
mod tests;
