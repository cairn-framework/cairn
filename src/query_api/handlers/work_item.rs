//! Shared wire projection for actionable findings, todos, and beads.

use schemars::{r#gen::SchemaGenerator, schema::Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::artefacts::registry::Todo;
use crate::{query_api::handlers::next_selection::decision_summary, state::backlog::BacklogItem};

/// The source category of a projected work item.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WorkItemSource {
    /// An ephemeral finding remediation action.
    Finding,
    /// A durable native todo artefact.
    Todo,
    /// A read-only beads backlog item.
    Bead,
}

/// Stable presentation shape shared by next-action JSON surfaces.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, schemars::JsonSchema)]
pub struct WorkItem {
    /// Provenance category.
    pub source: WorkItemSource,
    /// Human-readable title.
    pub title: String,
    /// Related graph node, when available.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub node: Option<String>,
    /// Suggested command, when available.
    #[schemars(required, schema_with = "nullable_string_schema")]
    pub command: Option<String>,
    /// Cross-source queue rank; lower ranks are more urgent.
    pub rank: u32,
}

fn nullable_string_schema(generator: &mut SchemaGenerator) -> Schema {
    generator.subschema_for::<Option<String>>()
}

impl WorkItem {
    /// Queue rank shared by every native-todo work item.
    pub(crate) const TODO_RANK: u32 = 100;

    pub(crate) fn from_todo(todo: &Todo) -> Self {
        Self {
            source: WorkItemSource::Todo,
            title: decision_summary(&todo.body),
            node: Some(todo.node.clone()),
            command: Some(format!("cairn todos {}", todo.node)),
            rank: Self::TODO_RANK,
        }
    }

    pub(crate) fn from_bead(bead: &BacklogItem) -> Self {
        Self {
            source: WorkItemSource::Bead,
            title: bead.title.clone(),
            node: bead.linked_node().map(ToOwned::to_owned),
            command: Some(format!("bd show {}", bead.id)),
            rank: 200,
        }
    }
}

/// Converts an existing remediation action into the shared finding shape.
/// Synthetic `action: none` placeholders are not findings and are omitted.
pub(crate) fn from_finding_action(action: &Value) -> Option<WorkItem> {
    if action.get("action").and_then(Value::as_str) == Some("none") {
        return None;
    }
    let rank = u32::try_from(action.get("priority").and_then(Value::as_u64)?).ok()?;
    let title = action
        .get("description")
        .and_then(Value::as_str)?
        .to_owned();
    let node = action
        .get("nodes")
        .and_then(Value::as_array)
        .and_then(|nodes| nodes.first())
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let command = action
        .get("command")
        .and_then(Value::as_str)
        .filter(|command| !command.is_empty())
        .map(ToOwned::to_owned);
    Some(WorkItem {
        source: WorkItemSource::Finding,
        title,
        node,
        command,
        rank,
    })
}
