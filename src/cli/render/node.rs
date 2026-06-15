//! Node-level query renderers.
#![allow(clippy::wildcard_imports)]
use super::super::format::{
    lines, neighbourhood_ids, node_arg, node_json, render_node, string_array_json,
};
use super::super::*;
use super::{scan_error_count, scan_error_warning};

pub(crate) fn render_get(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        query::get(&scan_result.graph, node)
            .map(|response| render_node(&response.node, parsed.json))
    })
}

// Reason: neighbourhood rendering spans human + JSON branches for many node
// fields; extracting each branch would fragment the output logic.
#[allow(clippy::too_many_lines)]
pub(crate) fn render_neighbourhood(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let include_orphans = parsed
            .command_args
            .iter()
            .any(|arg| arg == "--include-orphans");
        let response =
            query::neighbourhood_with_options(&scan_result.graph, node, include_orphans)?;
        Ok({
            let include_todos = parsed.command_args.iter().any(|arg| arg == "--include-todos");
            let include_research = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-research");
            let include_reviews = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-reviews");
            let include_deprecated = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-deprecated-decisions");
            let include_changes = parsed
                .command_args
                .iter()
                .any(|arg| arg == "--include-changes");
            let node_ids = neighbourhood_ids(&scan_result.graph, &response.node.id);
            let decisions = scan_result
                .artefacts
                .decisions
                .iter()
                .filter(|decision| {
                    decision.nodes.iter().any(|node| node_ids.contains(node))
                        && (decision.status == DecisionStatus::Accepted || include_deprecated)
                })
                .cloned()
                .collect::<Vec<_>>();
            let todos = if include_todos {
                scan_result
                    .artefacts
                    .todos
                    .iter()
                    .filter(|todo| node_ids.contains(&todo.node))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let research = if include_research {
                super::super::format::research_for_nodes(scan_result, &node_ids)
            } else {
                Vec::new()
            };
            let reviews = if include_reviews {
                scan_result
                    .artefacts
                    .reviews
                    .iter()
                    .filter(|review| node_ids.contains(&review.node))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let error_count = scan_error_count(scan_result);
            let warnings = scan_error_warning(error_count, parsed.json);
            if parsed.json {
                let active_changes = if include_changes {
                    ",\"active_changes\":[]"
                } else {
                    ""
                };
                format!(
                    "{{\"node\":{},\"inbound\":{},\"outbound\":{},\"contracts\":{},\"decisions\":{},\"todos\":{},\"research\":{},\"reviews\":{}{active_changes}{warnings}}}\n",
                    node_json(&response.node),
                    string_array_json(&response.inbound),
                    string_array_json(&response.outbound),
                    string_array_json(&response.node.contracts),
                    super::super::format::decisions_json(&decisions),
                    super::super::format::todos_json(&todos),
                    super::super::format::research_json(&research),
                    super::super::format::reviews_json(&reviews)
                )
            } else {
                let active_changes = if include_changes {
                    "\nActive changes:\nNone"
                } else {
                    ""
                };
                format!(
                    "Node: {}\nInbound:\n{}\nOutbound:\n{}\nContracts:\n{}\nAccepted decisions:\n{}\nTodos:\n{}\nResearch:\n{}\nReviews:\n{}{active_changes}{warnings}\n",
                    response.node.id,
                    lines(&response.inbound),
                    lines(&response.outbound),
                    lines(&response.node.contracts),
                    lines(&decisions.iter().map(super::super::format::decision_line).collect::<Vec<_>>()),
                    lines(&todos.iter().map(super::super::format::todo_line).collect::<Vec<_>>()),
                    lines(&research.iter().map(super::super::format::research_line).collect::<Vec<_>>()),
                    lines(&reviews.iter().map(super::super::format::review_line).collect::<Vec<_>>())
                )
            }
        })
    })
}

pub(crate) fn render_files(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node_record = scan_result.graph.resolve(node)?;
        let target_reports_for_node: Vec<_> = scan_result
            .target_reports
            .iter()
            .filter(|r| r.target_id.node_id == node_record.id)
            .collect();
        let has_multi_target = target_reports_for_node.len() > 1;
        if parsed.json {
            let targets_json = if target_reports_for_node.is_empty() {
                "[]".to_string()
            } else {
                let items: Vec<String> = target_reports_for_node
                    .iter()
                    .map(|r| {
                        format!(
                            "{{\"path\":\"{}\",\"language\":\"{}\",\"reconciler_id\":\"{}\",\"files\":{},\"hash\":\"{}\"}}",
                            esc(&r.target_id.path.to_string_lossy()),
                            r.language.as_str(),
                            r.reconciler_id.0,
                            string_array_json(&r.claimed_files),
                            esc(&r.hash)
                        )
                    })
                    .collect();
                    format!("[{}]", items.join(","))
            };
            if has_multi_target {
                Ok(format!(
                    "{{\"node\":\"{}\",\"targets\":{}}}\n",
                    esc(&node_record.id),
                    targets_json
                ))
            } else {
                Ok(format!(
                    "{{\"node\":\"{}\",\"files\":{},\"targets\":{}}}\n",
                    esc(&node_record.id),
                    string_array_json(&node_record.files),
                    targets_json
                ))
            }
        } else {
            let mut output = format!("Files for {}:\n", node_record.id);
            if has_multi_target {
                for r in &target_reports_for_node {
                    use std::fmt::Write;
                    writeln!(
                        output,
                        "  {} ({}): {}",
                        r.target_id.path.display(),
                        r.language.as_str(),
                        r.claimed_files.join(", ")
                    ).unwrap();
                    writeln!(output, "    reconciler: {}", r.reconciler_id.0).unwrap();
                    writeln!(output, "    hash: {}", r.hash).unwrap();
                }
            } else if let Some(r) = target_reports_for_node.first() {
                use std::fmt::Write;
                writeln!(
                    output,
                    "  {}: {}",
                    r.target_id.path.display(),
                    r.claimed_files.join(", ")
                ).unwrap();
                writeln!(output, "  language: {}", r.language.as_str()).unwrap();
                writeln!(output, "  reconciler: {}", r.reconciler_id.0).unwrap();
                writeln!(output, "  hash: {}", r.hash).unwrap();
            } else {
                use std::fmt::Write;
                writeln!(output, "  {}", lines(&node_record.files)).unwrap();
            }
            output.push('\n');
            Ok(output)
        }
    })
}
