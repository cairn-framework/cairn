//! JSON serialization helpers for CLI output.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::render::{decision_status, review_type, source_verification, todo_status};
use super::util::esc;

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
                    "{{\"id\":\"{}\",\"status\":\"{}\",\"nodes\":{},\"informed_by\":{},\"supersedes\":{},\"refines\":{},\"related\":{}}}",
                    esc(&decision.id),
                    decision_status(decision.status),
                    string_array_json(&decision.nodes),
                    string_array_json(&decision.informed_by),
                    string_array_json(&decision.supersedes),
                    string_array_json(&decision.refines),
                    string_array_json(&decision.related)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn research_json(research: &[Research]) -> String {
    format!(
        "[{}]",
        research
            .iter()
            .map(|item| {
                format!(
                    "{{\"id\":\"{}\",\"nodes\":{},\"sources\":{},\"date\":\"{}\"}}",
                    esc(&item.id),
                    string_array_json(&item.nodes),
                    string_array_json(&item.sources),
                    esc(&item.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn reviews_json(reviews: &[Review]) -> String {
    format!(
        "[{}]",
        reviews
            .iter()
            .map(|review| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"review_type\":\"{}\",\"date\":\"{}\",\"reviewer\":\"{}\"}}",
                    esc(&review.path),
                    esc(&review.node),
                    review_type(review.review_type),
                    esc(&review.date),
                    esc(&review.reviewer)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn sources_json(sources: &[Source]) -> String {
    format!(
        "[{}]",
        sources
            .iter()
            .map(|source| {
                format!(
                    "{{\"id\":\"{}\",\"file\":\"{}\",\"verification\":\"{}\",\"type\":\"{}\",\"date\":\"{}\"}}",
                    esc(&source.id),
                    esc(&source.file),
                    source_verification(source.verification),
                    esc(&source.source_type),
                    esc(&source.date)
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
        artefacts::registry::{DecisionStatus, SourceVerification, TodoStatus},
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

    // ── todos_json ───────────────────────────────────────────────────────────

    fn todo(status: TodoStatus, satisfies: Option<&str>) -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: "app".to_owned(),
            status,
            created: "2026-01-01".to_owned(),
            satisfies: satisfies.map(ToOwned::to_owned),
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
            claims: None,
            body: String::new(),
        };
        let json = decisions_json(&[decision]);
        assert!(json.contains("\"id\":\"adopt-rust\""));
        assert!(json.contains("\"status\":\"accepted\""));
        assert!(json.contains("\"nodes\":[\"app\"]"));
        assert!(json.contains("\"supersedes\":[\"old\"]"));
    }

    // ── reviews_json ─────────────────────────────────────────────────────────

    #[test]
    fn test_reviews_json_serializes_review_type_and_node() {
        let review = Review {
            path: "./review.md".to_owned(),
            node: "app".to_owned(),
            review_type: ReviewType::Human,
            date: "2026-01-01".to_owned(),
            reviewer: "alice".to_owned(),
            related_change: None,
            body: String::new(),
        };
        let json = reviews_json(&[review]);
        assert!(json.contains("\"node\":\"app\""));
        assert!(json.contains("\"review_type\":\"human\""));
        assert!(json.contains("\"reviewer\":\"alice\""));
    }

    // ── sources_json ─────────────────────────────────────────────────────────

    #[test]
    fn test_sources_json_serializes_verification_and_type() {
        let source = Source {
            id: "src-1".to_owned(),
            path: "./source.md".to_owned(),
            file: "./source.md".to_owned(),
            sha256: None,
            verification: SourceVerification::External,
            source_type: "article".to_owned(),
            date: "2026-01-01".to_owned(),
            tags: Vec::new(),
            description: String::new(),
            body: String::new(),
        };
        let json = sources_json(&[source]);
        assert!(json.contains("\"id\":\"src-1\""));
        assert!(json.contains("\"verification\":\"external\""));
        assert!(json.contains("\"type\":\"article\""));
    }

    // ── research_json ────────────────────────────────────────────────────────

    #[test]
    fn test_research_json_serializes_nodes_and_sources() {
        let research = Research {
            path: "./research.md".to_owned(),
            id: "r-1".to_owned(),
            nodes: vec!["app".to_owned()],
            date: "2026-01-01".to_owned(),
            sources: vec!["src-1".to_owned()],
            tags: Vec::new(),
            body: String::new(),
        };
        let json = research_json(&[research]);
        assert!(json.contains("\"id\":\"r-1\""));
        assert!(json.contains("\"nodes\":[\"app\"]"));
        assert!(json.contains("\"sources\":[\"src-1\"]"));
    }
}
