//! CLI renderers for query responses.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::*;

mod artefacts;
mod health;
mod node;
mod project;
mod remediate;

pub(crate) use artefacts::{
    render_decisions, render_rationale, render_research, render_sources, render_todos,
};
pub(crate) use health::render_health;
pub(crate) use node::{render_files, render_get, render_neighbourhood};
pub(crate) use project::{render_context, render_dependencies, render_status};
pub(crate) use remediate::{render_next, render_remediate};

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
