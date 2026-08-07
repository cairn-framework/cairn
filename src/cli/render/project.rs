//! Project-wide query renderers: status, backlog, and dependencies.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::format::{flag_value, lines, node_arg, string_array_json, todos_json};
use super::super::*;
use super::{scan_error_count, scan_info_count, scan_warning_count};
use crate::query_api::{QueryFlag, QueryRequest};
use std::collections::BTreeSet;

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
        let next_json = crate::query_api::work_item_for_selection(&crate::query_api::select_next(
            root,
            &root.join(&parsed.changes_dir),
            scan_result,
        ))
        .map_or(serde_json::Value::Null, |item| {
            serde_json::to_value(item).expect("WorkItem serialises")
        });
        format!(
            "{{\"active_changes\":[],\"open_todos\":{},\"recent_log_entries\":{},\"next_recommended\":{next_json}}}\n",
            todos_json(&open),
            string_array_json(&log_entries),
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
        at: None,
        since: None,
        tool: "deps".to_owned(),
        node: Some(node.to_owned()),
        symbol: None,
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
pub(in crate::cli::render) mod tests;

/// Renders the wave preview (`cairn wave`) or its stats projection
/// (`cairn wave stats`): both run the shared composer tool against the
/// already computed scan, so the CLI, the console, and the driver see one
/// composition. Clause 5 sentences hold: a held unit queues behind a
/// *unit*; the word claim needs a lease fact.
pub(crate) fn render_wave(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let stats = parsed.command_args.get(1).map(String::as_str) == Some("stats");
    let request = QueryRequest {
        tool: if stats { "wave stats" } else { "wave" }.to_owned(),
        at: flag_value(&parsed.command_args, "--at").map(ToOwned::to_owned),
        since: flag_value(&parsed.command_args, "--since").map(ToOwned::to_owned),
        ..QueryRequest::default()
    };
    let changes_dir = root.join(&parsed.changes_dir);
    let response = crate::query_api::execute_with_scan(
        root,
        &parsed.file,
        &changes_dir,
        &request,
        scan_result,
    )
    .map_err(|error| Finding {
        code: error.code,
        severity: FindingSeverity::Error,
        message: error.message,
        node: None,
        target: None,
        path: None,
        deferred_by: None,
        parked_by: None,
    })?;
    if parsed.json {
        return Ok(format!("{}\n", response.data));
    }
    let data = &response.data;
    if stats {
        return Ok(format!(
            "false-overlap rate: {} over {} of {} exclusions with merge evidence; threshold unset\n",
            data["false_overlap_rate"]
                .as_f64()
                .map_or_else(|| "n/a".to_owned(), |rate| format!("{rate:.2}")),
            data["window"]["size"],
            data["window"]["cap"],
        ));
    }
    Ok(render_wave_body(data))
}

/// Renders the human wave preview from the tool's data payload.
fn render_wave_body(data: &serde_json::Value) -> String {
    use std::fmt::Write as _;
    let units = data["units"].as_array().cloned().unwrap_or_default();
    let mut out = format!(
        "Next wave: {} unit(s)  {}\n",
        units.len(),
        data["plan"].as_str().unwrap_or("-")
    );
    let _ = writeln!(
        out,
        "rule {}  write-sets disjoint  hotspot permission: one unit per wave",
        data["rule"].as_str().unwrap_or("-")
    );
    for unit in &units {
        let includes: Vec<&str> = unit["write_set"]["includes"]
            .as_array()
            .map(|prefixes| {
                prefixes
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        let _ = writeln!(
            out,
            "  {}  writes {}  completeness: {}{}",
            unit["id"].as_str().unwrap_or("-"),
            includes.join(", "),
            unit["write_set"]["completeness"].as_str().unwrap_or("-"),
            if unit["hotspot_permission"] == true {
                "  holds the hotspot permission"
            } else {
                ""
            },
        );
        if unit["write_set"]["resolution"] == "unresolved" {
            let _ = writeln!(
                out,
                "    runs alone: {}, so cairn treats it as touching every file",
                unit["write_set"]["unresolved_reason"]
                    .as_str()
                    .unwrap_or("-"),
            );
        }
    }
    render_held(&mut out, data);
    out
}

/// Renders the held list with clause-5 sentences: a unit queues behind a
/// unit; the word claim appears only with a lease fact.
fn render_held(out: &mut String, data: &serde_json::Value) {
    use std::fmt::Write as _;
    let held = data["held"].as_array().cloned().unwrap_or_default();
    let _ = writeln!(out, "Held: {} unit(s)", held.len());
    for entry in &held {
        let id = entry["id"].as_str().unwrap_or("-");
        match entry["reason"].as_str().unwrap_or("-") {
            "write-sets-overlap" => {
                let _ = writeln!(
                    out,
                    "  {id} waits for this wave: same files as {}, one at a time. It queues behind that unit and joins the next wave.",
                    entry["behind"].as_str().unwrap_or("-"),
                );
            }
            "lease-held" => {
                let _ = writeln!(
                    out,
                    "  {id} queues behind a claim: lease fact {}.",
                    entry["blocking_fact_id"].as_str().unwrap_or("-"),
                );
            }
            "parked" => {
                let _ = writeln!(
                    out,
                    "  {id} is parked by ruling fact {}.",
                    entry["blocking_fact_id"].as_str().unwrap_or("-")
                );
            }
            _ => {
                let _ = writeln!(out, "  {id} runs alone in a later wave.");
            }
        }
    }
}

#[cfg(test)]
mod wave_render_tests {
    use super::render_wave_body;

    #[test]
    fn rendered_digest_is_pasteable_into_ruling_run() {
        let data = serde_json::json!({
            "plan": "plan-0123456789abcdef",
            "rule": "wf.default:1",
            "units": [],
            "held": [],
        });
        let out = render_wave_body(&data);
        let first = out.lines().next().expect("first line");
        // The digest must appear exactly as `cairn ruling run` accepts it:
        // bare, no JSON quoting.
        assert!(first.ends_with("plan-0123456789abcdef"), "{first}");
        assert!(!out.contains('"'), "{out}");
    }
}
