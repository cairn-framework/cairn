//! Query handler submodules grouped by domain to keep files under the size gate.

mod artefacts;
mod bundle;
mod context;
mod coordination;
mod graph;
mod locate;
mod next_selection;
mod node;
mod pending;
mod pending_brief;
mod pending_edges;
mod pending_evidence;
mod pending_rubric;
mod project;
mod remediate;
mod roadmap;
pub(super) mod spine;
mod work_item;

pub(super) use artefacts::{
    decisions_response_json, research_response_json, sources_response_json, todos_response_json,
};
pub(super) use bundle::bundle_json;
pub(super) use context::context_json;
pub(crate) use context::where_left;
pub(super) use coordination::{coordination_leases_json, coordination_rulings_json};
pub(super) use graph::{
    dependency_json, frontier_json, graph_response_json, islands_json, neighbourhood_json,
};
pub(super) use locate::locate_json;
pub(crate) use next_selection::{
    CleanItem, NextSelection, decision_summary, open_native_todos, select_next,
    work_item_for_selection,
};
pub(super) use node::{contract_json, docstring_json, files_json, rationale_json};
pub(super) use pending::pending_json;
pub(crate) use pending::pending_rows;
pub use pending::{PendingDecision, PendingDecisionEdge, PendingResponse, PendingTier};
pub use pending_evidence::{PendingEvidence, PendingReceipt};
pub use pending_rubric::PendingRubric;
pub(super) use project::status_json;
pub use project::{StatusActiveChange, StatusResponse, StatusTodo};
pub use remediate::RemediateResponse;
pub(super) use remediate::hook_json;
pub(crate) use remediate::{health_json, remediate_actions_raw, remediate_json};
pub(super) use roadmap::roadmap_json;
pub(crate) use roadmap::roadmap_response;
pub use roadmap::{RoadmapItem, RoadmapResponse, RoadmapTier};
pub(super) use spine::{beads_json, blueprint_json, ui_meta_json};
pub(crate) use work_item::from_finding_action;
pub use work_item::{WorkItem, WorkItemSource};
