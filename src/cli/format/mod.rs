//! CLI formatting helpers: JSON serialization, human rendering, and shared utilities.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]

mod json;
mod render;
mod util;

pub(crate) use json::{
    decisions_json, finding_json, node_json, research_json, reviews_json, sources_json,
    string_array_json, todos_json,
};
pub(crate) use render::{
    decision_line, render_findings, render_node, research_line, review_line, source_line, todo_line,
};
pub(crate) use util::{
    err, error_output, esc, finding_output, findings_output, flag_value, lines, neighbourhood_ids,
    node_arg, ok, parse_decision_status_filter, parse_todo_status_filter, research_for_nodes,
    sources_for_nodes,
};
