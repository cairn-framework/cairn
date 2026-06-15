//! Query handler submodules grouped by domain to keep files under the size gate.

mod artefacts;
mod graph;
mod node;
mod project;
mod remediate;

pub(super) use artefacts::{
    decisions_response_json, research_response_json, sources_response_json, todos_response_json,
};
pub(super) use graph::{dependency_json, islands_json, neighbourhood_json};
pub(super) use node::{contract_json, docstring_json, files_json, rationale_json};
pub(super) use project::{context_json, status_json};
pub(super) use remediate::hook_json;
pub(crate) use remediate::{health_json, remediate_json};
