//! Project-wide query renderers (context, status, dependencies).
#![allow(clippy::wildcard_imports)]
use super::super::format::{lines, node_arg, string_array_json, todos_json};
use super::super::*;
use super::{scan_error_count, scan_info_count, scan_warning_count};

// NOTE: render_context does not have access to Config, so it cannot show
// project_context. The JSON endpoint (context_json) includes it. Accept the
// divergence rather than threading Config through the CLI render layer.
pub(crate) fn render_context(scan_result: &scanner::ScanResult) -> String {
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
        "{} ({} nodes, {} edges)\n{}\n\nFindings: {} errors, {} warnings, {} info\n\nModules:\n",
        system_name,
        scan_result.graph.nodes.len(),
        edge_count,
        system_desc,
        errors,
        warnings,
        infos,
    );

    for node in scan_result.graph.nodes.values() {
        let paths = node.paths.join(", ");
        writeln!(
            out,
            "  {} ({}) [{:?}] {}",
            node.id, node.name, node.state, paths
        )
        .unwrap();
    }

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
    if parsed.json {
        format!(
            "{{\"active_changes\":[],\"open_todos\":{},\"recent_log_entries\":{}}}\n",
            todos_json(&open),
            string_array_json(&log_entries)
        )
    } else {
        format!(
            "Status:\nActive changes:\nNone\nOpen todos:\n{}\nRecent log entries:\n{}\n",
            lines(
                &open
                    .iter()
                    .map(super::super::format::todo_line)
                    .collect::<Vec<_>>()
            ),
            lines(&log_entries)
        )
    }
}

pub(crate) fn render_dependencies(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let transitive = parsed.command_args.iter().any(|arg| arg == "--transitive");
    node_arg(&parsed.command_args).and_then(|node| {
        let response = if parsed.command == "depends" {
            query::depends(&scan_result.graph, node, transitive)
        } else {
            query::dependents(&scan_result.graph, node, transitive)
        }?;
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"nodes\":{}}}\n",
                esc(&response.node),
                string_array_json(&response.nodes)
            )
        } else {
            format!("{}:\n{}\n", response.node, lines(&response.nodes))
        })
    })
}
