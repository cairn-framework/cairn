//! Query handler submodules grouped by domain to keep files under the size gate.

mod artefacts;
mod bundle;
mod graph;
mod locate;
mod next_selection;
mod node;
mod project;
mod remediate;
pub(super) mod spine;

pub(super) use artefacts::{
    decisions_response_json, research_response_json, sources_response_json, todos_response_json,
};
pub(super) use bundle::bundle_json;
pub(super) use graph::{
    dependency_json, frontier_json, graph_response_json, islands_json, neighbourhood_json,
};
pub(super) use locate::locate_json;
pub(crate) use next_selection::{CleanItem, NextSelection, open_native_todos, select_next};
pub(super) use node::{contract_json, docstring_json, files_json, rationale_json};
pub(super) use project::{context_json, status_json};
pub(super) use remediate::hook_json;
pub(crate) use remediate::{health_json, remediate_json};
pub(super) use spine::{beads_json, blueprint_json, ui_meta_json};
