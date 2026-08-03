//! CLI renderers for query responses.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::*;
use crate::query_api::QueryError;

mod artefacts;
mod bundle;
mod changes_view;
mod context_view;
mod health;
mod locate;
mod node;
mod pending;
mod project;
mod remediate;
mod session;

pub(crate) use artefacts::{
    render_decisions, render_rationale, render_research, render_sources, render_todos,
};
pub(crate) use bundle::render_bundle;
pub(crate) use changes_view::{render_changes, render_show};
pub(crate) use health::render_health;
pub(crate) use locate::render_locate;
pub(crate) use node::{render_files, render_get, render_neighbourhood};
pub(crate) use pending::render_pending_detail;
pub(crate) use project::{render_backlog, render_dependencies, render_status};
pub(crate) use remediate::{render_brief, render_next, render_remediate};
pub(crate) use session::render_context;

pub(crate) fn scan_error_count(scan_result: &scanner::ScanResult) -> usize {
    scan_result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Error)
        .count()
}

pub(crate) fn scan_warning_count(scan_result: &scanner::ScanResult) -> usize {
    scan_result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Warning)
        .count()
}

pub(crate) fn scan_info_count(scan_result: &scanner::ScanResult) -> usize {
    scan_result
        .graph
        .findings
        .iter()
        .filter(|finding| finding.severity == FindingSeverity::Info)
        .count()
}

pub(crate) fn scan_error_warning(error_count: usize, json: bool) -> String {
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
/// Converts a `query_api` error back into the `Finding` shape the CLI
/// render dispatch expects, so a migrated human renderer can keep the
/// identical `finding_output` error path (same code/message) it had when
/// it called the engine directly.
pub(crate) fn query_error_to_finding(error: QueryError) -> Finding {
    Finding {
        code: error.code,
        severity: FindingSeverity::Error,
        message: error.message,
        node: None,
        target: None,
        path: error.source_span,
        deferred_by: None,
        parked_by: None,
    }
}
