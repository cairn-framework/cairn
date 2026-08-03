//! Session-continuity renderer for `cairn context`: the human overview,
//! the waiting-on-you queue, and the where-work-was-left block.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::pending::render_pending_detail;
use super::{scan_error_count, scan_info_count, scan_warning_count};

// NOTE: render_context has no Config access, so it cannot show project_context
// (the context_json endpoint includes it). The backlog summary is text-only too.
pub(crate) fn render_context(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
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

    let pending_signatures = scan_result
        .artefacts
        .decisions
        .iter()
        .filter(|decision| decision.status == DecisionStatus::Proposed)
        .count();

    let mut out = format!(
        "{} ({} nodes, {} edges)\n{}\n\nFindings: {} errors, {} warnings, {} info\n{}\n\n",
        system_name,
        scan_result.graph.nodes.len(),
        edge_count,
        system_desc,
        errors,
        warnings,
        infos,
        copy::lookup("context.pending-signatures")
            .replace("{count}", &pending_signatures.to_string()),
    );
    let pending_rows =
        crate::query_api::pending_rows(root, scan_result).map_err(super::query_error_to_finding)?;
    let _ = writeln!(out, "\n{}", copy::lookup("context.waiting-on-you"));
    if pending_rows.is_empty() {
        let _ = writeln!(out, "{}", copy::lookup("context.waiting-none"));
    } else {
        for row in &pending_rows {
            out.push_str(&render_pending_detail(row));
            out.push('\n');
        }
    }
    render_where_left(&mut out, parsed, root, scan_result);
    let next = crate::query_api::work_item_for_selection(&crate::query_api::select_next(
        root,
        &root.join(&parsed.changes_dir),
        scan_result,
    ));
    if let Some(item) = next {
        let command = item
            .command
            .as_deref()
            .unwrap_or_else(|| copy::lookup("context.no-command"));
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("context.recommended")
                .replace("{title}", &item.title)
                .replace("{command}", command)
        );
    } else {
        let _ = writeln!(out, "{}", copy::lookup("context.recommended-none"));
    }

    out.push_str("\nStructure:\n");
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

    Ok(out)
}

/// Renders the "where work was left" continuity block from the shared
/// projection: in-progress todos, active changes, then in-progress beads,
/// or an honest empty.
fn render_where_left(
    out: &mut String,
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) {
    use std::fmt::Write as _;
    let left = crate::query_api::where_left(root, &root.join(&parsed.changes_dir), scan_result);
    let _ = writeln!(out, "{}", copy::lookup("context.where-left"));
    let in_progress = left["in_progress"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    let active_changes = left["active_changes"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    let in_progress_backlog = left["in_progress_backlog"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    if in_progress.is_empty() && active_changes.is_empty() && in_progress_backlog.is_empty() {
        let _ = writeln!(out, "{}", copy::lookup("context.where-left-none"));
        return;
    }
    for todo in in_progress {
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("context.where-left-todo")
                .replace("{stem}", todo["stem"].as_str().unwrap_or(""))
                .replace("{node}", todo["node"].as_str().unwrap_or(""))
        );
    }
    for change in active_changes {
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("context.where-left-change")
                .replace("{id}", change["id"].as_str().unwrap_or(""))
                .replace("{title}", change["title"].as_str().unwrap_or(""))
        );
    }
    for bead in in_progress_backlog {
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("context.where-left-bead")
                .replace("{id}", bead["id"].as_str().unwrap_or(""))
                .replace("{title}", bead["title"].as_str().unwrap_or(""))
        );
    }
}

#[cfg(test)]
#[path = "session/tests.rs"]
mod tests;
