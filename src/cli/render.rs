// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::format::{
    decision_line, decisions_json, flag_value, lines, neighbourhood_ids, node_arg, node_json,
    parse_decision_status_filter, parse_todo_status_filter, render_node, research_for_nodes,
    research_json, research_line, review_line, reviews_json, source_line, sources_for_nodes,
    sources_json, string_array_json, todo_line, todos_json,
};
use super::*;
use crate::query_api;
fn scan_error_count(scan_result: &scanner::ScanResult) -> usize {
    scan_result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count()
}

fn scan_warning_count(scan_result: &scanner::ScanResult) -> usize {
    scan_result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Warning)
        .count()
}

fn scan_info_count(scan_result: &scanner::ScanResult) -> usize {
    scan_result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Info)
        .count()
}

fn scan_error_warning(error_count: usize, json: bool) -> String {
    if error_count == 0 {
        return String::new();
    }
    if json {
        format!(",\"warnings\":[\"scan has {error_count} error(s); graph may be incomplete\"]")
    } else {
        format!(
            "\nWarning: scan has {error_count} error(s); graph may be incomplete. \
             Run `cairn scan` for details."
        )
    }
}

pub(super) fn render_get(
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
pub(super) fn render_neighbourhood(
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
                research_for_nodes(scan_result, &node_ids)
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
                    decisions_json(&decisions),
                    todos_json(&todos),
                    research_json(&research),
                    reviews_json(&reviews)
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
                    lines(&decisions.iter().map(decision_line).collect::<Vec<_>>()),
                    lines(&todos.iter().map(todo_line).collect::<Vec<_>>()),
                    lines(&research.iter().map(research_line).collect::<Vec<_>>()),
                    lines(&reviews.iter().map(review_line).collect::<Vec<_>>())
                )
            }
        })
    })
}

pub(super) fn render_files(
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

pub(super) fn render_todos(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let status = flag_value(&parsed.command_args, "--status").and_then(parse_todo_status_filter);
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let todos = scan_result
            .artefacts
            .todos
            .iter()
            .filter(|todo| {
                todo.node == node.id && status.is_none_or(|filter| todo.status == filter)
            })
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"todos\":{}}}\n",
                esc(&node.id),
                todos_json(&todos)
            )
        } else {
            format!(
                "Todos for {}:\n{}\n",
                node.id,
                lines(&todos.iter().map(todo_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(super) fn render_decisions(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let status =
        flag_value(&parsed.command_args, "--status").and_then(parse_decision_status_filter);
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
                lines(&decisions.iter().map(decision_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(super) fn render_research(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let research = research_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]));
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"research\":{}}}\n",
                esc(&node.id),
                research_json(&research)
            )
        } else {
            format!(
                "Research for {}:\n{}\n",
                node.id,
                lines(&research.iter().map(research_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(super) fn render_sources(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let sources = sources_for_nodes(scan_result, &BTreeSet::from([node.id.clone()]));
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"sources\":{}}}\n",
                esc(&node.id),
                sources_json(&sources)
            )
        } else {
            format!(
                "Sources for {}:\n{}\n",
                node.id,
                lines(&sources.iter().map(source_line).collect::<Vec<_>>())
            )
        })
    })
}

pub(super) fn render_rationale(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;
        let node_ids = neighbourhood_ids(&scan_result.graph, &node.id);
        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.status == DecisionStatus::Accepted
                    && decision.nodes.iter().any(|node| node_ids.contains(node))
            })
            .cloned()
            .collect::<Vec<_>>();
        let research_ids = decisions
            .iter()
            .flat_map(|decision| decision.informed_by.iter())
            .cloned()
            .collect::<BTreeSet<_>>();
        let source_ids = decisions
            .iter()
            .flat_map(|decision| decision.informed_by.iter())
            .cloned()
            .chain(
                scan_result
                    .artefacts
                    .research
                    .iter()
                    .filter(|research| research_ids.contains(&research.id))
                    .flat_map(|research| research.sources.iter().cloned()),
            )
            .collect::<BTreeSet<_>>();
        let research = scan_result
            .artefacts
            .research
            .iter()
            .filter(|research| research_ids.contains(&research.id))
            .cloned()
            .collect::<Vec<_>>();
        let sources = scan_result
            .artefacts
            .sources
            .iter()
            .filter(|source| source_ids.contains(&source.id))
            .cloned()
            .collect::<Vec<_>>();
        Ok(if parsed.json {
            format!(
                "{{\"node\":\"{}\",\"decisions\":{},\"research\":{},\"sources\":{}}}\n",
                esc(&node.id),
                decisions_json(&decisions),
                research_json(&research),
                sources_json(&sources)
            )
        } else {
            format!(
                "Rationale for {}:\nDecisions:\n{}\nResearch:\n{}\nSources:\n{}\n",
                node.id,
                lines(&decisions.iter().map(decision_line).collect::<Vec<_>>()),
                lines(&research.iter().map(research_line).collect::<Vec<_>>()),
                lines(&sources.iter().map(source_line).collect::<Vec<_>>())
            )
        })
    })
}

// NOTE: render_context does not have access to Config, so it cannot show
// project_context. The JSON endpoint (context_json) includes it. Accept the
// divergence rather than threading Config through the CLI render layer.
pub(super) fn render_context(scan_result: &scanner::ScanResult) -> String {
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

pub(super) fn render_status(
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
            lines(&open.iter().map(todo_line).collect::<Vec<_>>()),
            lines(&log_entries)
        )
    }
}

pub(super) fn render_dependencies(
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
pub(super) fn render_health(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let health = query_api::health_json(root, &changes_dir, scan_result);
    if parsed.json {
        format!("{health}\n")
    } else {
        format_health_human(
            &health,
            scan_result
                .graph
                .findings
                .iter()
                .filter(|f| f.severity == FindingSeverity::Error)
                .count(),
            scan_result
                .graph
                .findings
                .iter()
                .filter(|f| f.severity == FindingSeverity::Warning)
                .count(),
        )
    }
}
fn format_health_human(
    health: &serde_json::Value,
    scan_errors: usize,
    scan_warnings: usize,
) -> String {
    let clean = health
        .get("clean")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let summary = health.get("summary").unwrap_or(&serde_json::Value::Null);
    let total_errors = summary
        .get("total_errors")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let total_warnings = summary
        .get("total_warnings")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let total_info = summary
        .get("total_info")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let modules = summary.get("modules").unwrap_or(&serde_json::Value::Null);
    let synced = modules
        .get("synced")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let ghost = modules
        .get("ghost")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let orphaned = modules
        .get("orphaned")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let mut lines = Vec::new();
    if clean {
        lines.push("Health: clean".to_owned());
    } else {
        lines.push("Health: needs attention".to_owned());
    }
    lines.push(format!(
        "  errors: {total_errors}, warnings: {total_warnings}, info: {total_info}"
    ));
    lines.push(format!(
        "  modules: {synced} synced, {ghost} ghost, {orphaned} orphaned"
    ));
    if scan_errors > 0 {
        lines.push(format!("  scan errors: {scan_errors}"));
    }
    if scan_warnings > 0 {
        lines.push(format!("  scan warnings: {scan_warnings}"));
    }
    lines.join("\n") + "\n"
}
pub(super) fn render_remediate(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
    if parsed.json {
        format!("{remediate}\n")
    } else {
        format_remediate_human(&remediate)
    }
}
fn format_remediate_human(remediate: &serde_json::Value) -> String {
    let empty: Vec<serde_json::Value> = Vec::new();
    let actions = remediate
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty);
    if actions.is_empty() {
        return "No actions required.\n".to_owned();
    }
    let mut lines = Vec::new();
    lines.push(format!("Actions ({}):", actions.len()));
    for action in actions {
        let priority = action
            .get("priority")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(99);
        let name = action
            .get("action")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown");
        let command = action
            .get("command")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let description = action
            .get("description")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let nodes = action
            .get("nodes")
            .and_then(serde_json::Value::as_array)
            .map(|arr: &Vec<serde_json::Value>| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        lines.push(format!("  [{priority}] {name}"));
        if !description.is_empty() {
            lines.push(format!("      {description}"));
        }
        if !command.is_empty() {
            lines.push(format!("      run: {command}"));
        }
        if !nodes.is_empty() {
            lines.push(format!("      nodes: {}", nodes.join(", ")));
        }
    }
    lines.join("\n") + "\n"
}
pub(super) fn render_next(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let health = query_api::health_json(root, &changes_dir, scan_result);
    let clean = health
        .get("clean")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if parsed.json {
        if clean {
            return "{\"next\":null,\"clean\":true}\n".to_owned();
        }
        let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
        let empty: Vec<serde_json::Value> = Vec::new();
        let actions = remediate
            .get("actions")
            .and_then(serde_json::Value::as_array)
            .unwrap_or(&empty);
        let first = actions.first().unwrap_or(&serde_json::Value::Null);
        return format!("{{\"next\":{first},\"clean\":false}}\n");
    }
    if clean {
        return "Next: nothing to do. Project is clean.\n".to_owned();
    }
    let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
    let empty: Vec<serde_json::Value> = Vec::new();
    let actions = remediate
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty);
    if let Some(first) = actions.first() {
        let name = first
            .get("action")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown");
        let command = first
            .get("command")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let description = first
            .get("description")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let nodes = first
            .get("nodes")
            .and_then(serde_json::Value::as_array)
            .map(|arr: &Vec<serde_json::Value>| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut lines = Vec::new();
        lines.push(format!("Next action: {name}"));
        if !description.is_empty() {
            lines.push(format!("  {description}"));
        }
        if !command.is_empty() {
            lines.push(format!("  run: {command}"));
        }
        if !nodes.is_empty() {
            lines.push(format!("  nodes: {}", nodes.join(", ")));
        }
        lines.join("\n") + "\n"
    } else {
        "Next: nothing to do.\n".to_owned()
    }
}
