//! JSON serialization helpers for CLI output.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::util::esc;
use crate::query_api::{decision_status, ratification_tier, ratified_by_wire, todo_status};

pub(crate) fn node_json(node: &NodeRecord) -> String {
    format!(
        "{{\"id\":\"{}\",\"name\":\"{}\",\"description\":\"{}\",\"state\":\"{:?}\",\"children\":{},\"files\":{}}}",
        esc(&node.id),
        esc(&node.name),
        esc(&node.description),
        node.state,
        string_array_json(&node.children),
        string_array_json(&node.files)
    )
}

pub(crate) fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{}\",\"message\":\"{}\"}}",
        esc(&finding.code),
        finding.severity.name(),
        esc(&finding.message)
    )
}

pub(crate) fn todos_json(todos: &[Todo]) -> String {
    format!(
        "[{}]",
        todos
            .iter()
            .map(|todo| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"status\":\"{}\",\"created\":\"{}\",\"satisfies\":\"{}\"}}",
                    esc(&todo.path),
                    esc(&todo.node),
                    todo_status(todo.status),
                    esc(&todo.created),
                    esc(todo.satisfies.as_deref().unwrap_or(""))
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn decisions_json(decisions: &[Decision]) -> String {
    format!(
        "[{}]",
        decisions
            .iter()
            .map(|decision| {
                format!(
                    "{{\"id\":\"{}\",\"status\":\"{}\",\"nodes\":{},\"informed_by\":{},\"supersedes\":{},\"refines\":{},\"related\":{},\"revisited\":{},\"ratification\":\"{}\",\"ratified_by\":{}}}",
                    esc(&decision.id),
                    decision_status(decision.status),
                    string_array_json(&decision.nodes),
                    string_array_json(&decision.informed_by),
                    string_array_json(&decision.supersedes),
                    string_array_json(&decision.refines),
                    string_array_json(&decision.related),
                    decision
                        .revisited
                        .as_deref()
                        .map_or_else(|| "null".to_owned(), |value| format!("\"{}\"", esc(value))),
                    ratification_tier(decision.ratification),
                    ratified_by_wire(decision)
                        .map_or_else(|| "null".to_owned(), |value| format!("\"{value}\""))
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn string_array_json(values: &[String]) -> String {
    let mut out = String::from('[');
    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&esc(value));
        out.push('"');
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        artefacts::registry::{DecisionStatus, TodoStatus},
        blueprint::{NodeKind, Span},
        map::{FindingSeverity, NodeRecord, NodeState},
    };

    // ── string_array_json ────────────────────────────────────────────────────

    #[test]
    fn test_string_array_json_empty() {
        assert_eq!(string_array_json(&[]), "[]");
    }

    #[test]
    fn test_string_array_json_single() {
        assert_eq!(string_array_json(&["a".to_owned()]), "[\"a\"]");
    }

    #[test]
    fn test_string_array_json_multiple() {
        assert_eq!(
            string_array_json(&["a".to_owned(), "b".to_owned()]),
            "[\"a\",\"b\"]"
        );
    }

    #[test]
    fn test_string_array_json_value_with_quote_is_escaped() {
        assert_eq!(string_array_json(&["a\"b".to_owned()]), "[\"a\\\"b\"]");
    }

    // ── finding_json ─────────────────────────────────────────────────────────

    #[test]
    fn test_finding_json_escapes_message_and_code() {
        let finding = Finding {
            code: "CAIRN_TEST\"".to_owned(),
            severity: FindingSeverity::Error,
            message: "bad \"input\"".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        };
        let json = finding_json(&finding);
        assert!(json.contains("\"code\":\"CAIRN_TEST\\\"\""));
        assert!(json.contains("\"message\":\"bad \\\"input\\\"\""));
        assert!(json.contains("\"severity\":\"error\""));
    }

    // ── node_json ────────────────────────────────────────────────────────────

    #[test]
    fn test_node_json_includes_id_children_and_files() {
        let node = NodeRecord {
            kind: NodeKind::Module,
            id: "app".to_owned(),
            name: "app".to_owned(),
            description: String::new(),
            tags: Vec::new(),
            parent: None,
            children: vec!["child".to_owned()],
            paths: Vec::new(),
            owns_files: false,
            symbols: Vec::new(),
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: vec!["src/lib.rs".to_owned()],
            span: Span::point("test", 1, 1),
        };
        let json = node_json(&node);
        assert!(json.contains("\"id\":\"app\""));
        assert!(json.contains("\"children\":[\"child\"]"));
        assert!(json.contains("\"files\":[\"src/lib.rs\"]"));
    }

    #[test]
    fn test_node_json_reports_ghost_state() {
        // `cairn get --json` must expose Ghost so empty scaffolding is not
        // mistaken for Synced (gh:#238).
        let node = NodeRecord {
            kind: NodeKind::Module,
            id: "app.empty".to_owned(),
            name: "empty".to_owned(),
            description: String::new(),
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            paths: vec!["./src/empty".to_owned()],
            owns_files: true,
            symbols: Vec::new(),
            contracts: Vec::new(),
            state: NodeState::Ghost,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        };
        let json = node_json(&node);
        assert!(
            json.contains("\"state\":\"Ghost\""),
            "get json must report Ghost state: {json}"
        );
        assert!(json.contains("\"files\":[]"));
    }

    // ── todos_json ───────────────────────────────────────────────────────────

    fn todo(status: TodoStatus, satisfies: Option<&str>) -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: "app".to_owned(),
            status,
            created: "2026-01-01".to_owned(),
            satisfies: satisfies.map(ToOwned::to_owned),
            blocked_by: Vec::new(),
            parent: None,
            related: Vec::new(),
            defers: Vec::new(),
            body: String::new(),
        }
    }

    #[test]
    fn test_todos_json_empty_list() {
        assert_eq!(todos_json(&[]), "[]");
    }

    #[test]
    fn test_todos_json_includes_status_and_satisfies() {
        let todos = vec![
            todo(TodoStatus::Open, Some("decision-1")),
            todo(TodoStatus::Done, None),
        ];
        let json = todos_json(&todos);
        assert!(json.contains("\"status\":\"open\""));
        assert!(json.contains("\"satisfies\":\"decision-1\""));
        assert!(json.contains("\"satisfies\":\"\""));
    }

    // ── decisions_json ───────────────────────────────────────────────────────

    #[test]
    fn test_decisions_json_serializes_status_and_node_refs() {
        let decision = Decision {
            path: "./decision.md".to_owned(),
            id: "adopt-rust".to_owned(),
            status: DecisionStatus::Accepted,
            nodes: vec!["app".to_owned()],
            date: String::new(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: vec!["old".to_owned()],
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
            ratification: crate::artefacts::registry::RatificationTier::Binding,
            affects: Vec::new(),
            ratified_by_machine: false,
            receipts: Vec::new(),
        };
        let json = decisions_json(&[decision]);
        assert!(json.contains("\"id\":\"adopt-rust\""));
        assert!(json.contains("\"status\":\"accepted\""));
        assert!(json.contains("\"nodes\":[\"app\"]"));
        assert!(json.contains("\"supersedes\":[\"old\"]"));
        assert!(
            json.contains("\"revisited\":null"),
            "absent revisited serializes as null: {json}"
        );
        assert!(json.contains("\"ratification\":\"binding\""));
        assert!(json.contains("\"ratified_by\":\"maintainer\""));
    }

    #[test]
    fn test_decisions_json_serializes_populated_revisited() {
        let decision = Decision {
            path: "./decision.md".to_owned(),
            id: "adopt-rust".to_owned(),
            status: DecisionStatus::Accepted,
            nodes: vec!["app".to_owned()],
            date: "2026-01-01".to_owned(),
            revisited: Some("2026-06-29".to_owned()),
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
            ratification: crate::artefacts::registry::RatificationTier::Binding,
            affects: Vec::new(),
            ratified_by_machine: false,
            receipts: Vec::new(),
        };
        let json = decisions_json(&[decision]);
        assert!(
            json.contains("\"revisited\":\"2026-06-29\""),
            "populated revisited serializes as the date string: {json}"
        );
    }
}
