//! Project-wide query renderers (context, status, dependencies).
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{flag_value, lines, node_arg, string_array_json, todos_json};
use super::super::*;
use super::{scan_error_count, scan_info_count, scan_warning_count};
use crate::query_api::{QueryFlag, QueryRequest};
use std::collections::BTreeSet;

// NOTE: render_context has no Config access, so it cannot show project_context
// (the context_json endpoint includes it). The backlog summary is text-only too.
pub(crate) fn render_context(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    use std::fmt::Write as _;

    let system = scan_result
        .graph
        .nodes
        .values()
        .find(|n| n.kind == crate::blueprint::ast::NodeKind::System);
    let system_name = system.map_or("unknown", |n| n.name.as_str());
    let system_desc = system.map_or("", |n| n.description.as_str());

    let edge_count: usize = scan_result.graph.outbound.values().map(Vec::len).sum();
    let errors = scan_error_count(scan_result);
    let warnings = scan_warning_count(scan_result);
    let infos = scan_info_count(scan_result);

    let mut out = format!(
        "{} ({} nodes, {} edges)\n{}\n\nFindings: {} errors, {} warnings, {} info\n\nStructure:\n",
        system_name,
        scan_result.graph.nodes.len(),
        edge_count,
        system_desc,
        errors,
        warnings,
        infos,
    );

    let prefix = system.map(|s| format!("{}.", s.id)).unwrap_or_default();
    let opts = super::context_view::ContextOpts::parse(&parsed.command_args);
    let structure = if opts.mermaid {
        super::context_view::render_mermaid(&scan_result.graph, &opts, &prefix)
    } else {
        super::context_view::render_structure(&scan_result.graph, &opts, &prefix)
    };
    out.push_str(&structure);

    let ac = &scan_result.artefacts;
    write!(
        out,
        "\nArtefacts: {} contracts, {} decisions, {} todos, {} research, {} reviews, {} sources\n",
        ac.contracts.contracts.len(),
        ac.decisions.len(),
        ac.todos.len(),
        ac.research.len(),
        ac.reviews.len(),
        ac.sources.len(),
    )
    .unwrap();

    let backlog = crate::state::backlog::read(root);
    let ready = crate::state::backlog::ready(&backlog);
    let _ = write!(out, "\nBacklog: {} ready\n", ready.len());
    for item in ready.iter().take(5) {
        let _ = writeln!(out, "  {} [P{}] {}", item.id, item.priority, item.title);
    }

    out
}

/// Renders the beads (issues) linked to a node via their `cairn-node:<id>`
/// label, the CLI counterpart of the webui inspector's beads panel.
pub(crate) fn render_backlog(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    use std::fmt::Write as _;
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let items = crate::state::backlog::read(root);
        let beads = crate::state::backlog::for_node(&items, &node.id);
        Ok(if parsed.json {
            let arr = beads
                .iter()
                .map(|b| b.to_json().to_string())
                .collect::<Vec<_>>()
                .join(",");
            format!("{{\"node\":\"{}\",\"beads\":[{arr}]}}\n", esc(&node.id))
        } else if beads.is_empty() {
            format!(
                "{}\n",
                crate::copy::lookup("empty-states.node-no-beads.body")
            )
        } else {
            let mut out = format!("Beads for {}:\n", node.id);
            for b in &beads {
                let _ = writeln!(
                    out,
                    "  {} [P{}] [{}] {}",
                    b.id, b.priority, b.status, b.title
                );
            }
            out
        })
    })
}

fn render_status_brief(
    scan_result: &scanner::ScanResult,
    root: &Path,
    open: &[Todo],
    next_recommended: Option<&str>,
) -> String {
    let total = scan_error_count(scan_result)
        + scan_warning_count(scan_result)
        + scan_info_count(scan_result);
    let findings_line = crate::copy::lookup("status.brief.findings")
        .replace("{total}", &total.to_string())
        .replace("{errors}", &scan_error_count(scan_result).to_string())
        .replace("{warnings}", &scan_warning_count(scan_result).to_string())
        .replace("{info}", &scan_info_count(scan_result).to_string());
    let mut out = String::from("Status:\n");
    out.push_str(&findings_line);
    out.push('\n');
    out.push_str(
        &crate::copy::lookup("status.brief.open-todos").replace("{count}", &open.len().to_string()),
    );
    out.push('\n');
    for todo in open.iter().take(5) {
        out.push_str(&super::super::format::todo_line(todo));
        out.push('\n');
    }
    let remaining = open.len().saturating_sub(5);
    out.push_str(
        &crate::copy::lookup("status.brief.todo-overflow")
            .replace("{remaining}", &remaining.to_string()),
    );
    out.push('\n');
    out.push_str("Recent log entries:\n");
    let brief_log = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            let mut seen = std::collections::HashSet::new();
            content
                .lines()
                .rev()
                .filter(|line| !line.trim().is_empty())
                .filter(|line| seen.insert(line.to_string()))
                .take(5)
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();
    if brief_log.is_empty() {
        out.push_str("None\n");
    } else {
        out.push_str(&brief_log.join("\n"));
        out.push('\n');
    }
    out.push_str("Next recommended:\n");
    out.push_str(next_recommended.unwrap_or("None"));
    out
}

pub(crate) fn render_status(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
    root: &Path,
) -> String {
    let open = scan_result
        .artefacts
        .todos
        .iter()
        .filter(|todo| todo.status == TodoStatus::Open || todo.status == TodoStatus::InProgress)
        .cloned()
        .collect::<Vec<_>>();
    let log_entries = fs::read_to_string(root.join(".cairn/log.md"))
        .map(|content| {
            content
                .lines()
                .rev()
                .take(5)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let backlog = crate::state::backlog::read(root);
    let ready = crate::state::backlog::ready(&backlog);
    let native_todos = crate::query_api::open_native_todos(scan_result);
    let next_recommended = native_todos.first().map_or_else(
        || {
            ready
                .first()
                .map(|top| format!("{} [P{}] {}", top.id, top.priority, top.title))
        },
        |top| {
            Some(format!(
                "{} (native todo, node: {})",
                super::remediate::decision_summary(&top.body),
                top.node
            ))
        },
    );
    if parsed.json {
        format!(
            "{{\"active_changes\":[],\"open_todos\":{},\"recent_log_entries\":{},\"next_recommended\":{}}}\n",
            todos_json(&open),
            string_array_json(&log_entries),
            next_recommended
                .as_deref()
                .map_or_else(|| "null".to_owned(), |value| format!("\"{}\"", esc(value)))
        )
    } else if parsed.brief {
        render_status_brief(scan_result, root, &open, next_recommended.as_deref())
    } else {
        let active_changes =
            crate::changes::discover(root, &root.join(&parsed.changes_dir)).unwrap_or_default();
        let active_lines = lines(&crate::changes::active_changes_lines(&active_changes));
        format!(
            "Status:\nActive changes:\n{}\nOpen todos:\n{}\nRecent log entries:\n{}\nNext recommended:\n{}\n",
            active_lines,
            lines(
                &open
                    .iter()
                    .map(super::super::format::todo_line)
                    .collect::<Vec<_>>()
            ),
            lines(&log_entries),
            next_recommended.unwrap_or_else(|| "None".to_owned())
        )
    }
}
pub(crate) fn render_dependencies(parsed: &ParsedArgs, root: &Path) -> Result<String, Finding> {
    let node = node_arg(&parsed.command_args)?;
    let mut flags = BTreeSet::new();
    if parsed.command_args.iter().any(|arg| arg == "--transitive") {
        flags.insert(QueryFlag::Transitive);
    }
    if flag_value(&parsed.command_args, "--direction") == Some("in") {
        flags.insert(QueryFlag::Inbound);
    }
    let request = QueryRequest {
        tool: "deps".to_owned(),
        node: Some(node.to_owned()),
        change: None,
        old_id: None,
        new_id: None,
        status: None,
        language: None,
        flags,
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
    let node_id = data["node"].as_str().unwrap_or_default();
    let nodes: Vec<String> = data["nodes"]
        .as_array()
        .map_or(&[][..], std::ops::Deref::deref)
        .iter()
        .filter_map(|value| value.as_str().map(ToOwned::to_owned))
        .collect();
    Ok(format!("{node_id}:\n{}\n", lines(&nodes)))
}

#[cfg(test)]
mod tests;
