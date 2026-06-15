//! Human-readable rendering and status display helpers for CLI output.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::json::node_json;
use super::util::esc;

pub(crate) fn render_node(node: &NodeRecord, json: bool) -> String {
    if json {
        format!("{}\n", node_json(node))
    } else {
        format!(
            "ID: {}\nName: {}\nDescription: {}\nState: {:?}\n",
            node.id, node.name, node.description, node.state
        )
    }
}

pub(crate) fn render_findings(findings: &[Finding], json: bool) -> String {
    if json {
        if findings.is_empty() {
            return "{\"findings\":[]}\n".to_owned();
        }
        let mut out = String::from("{\"findings\":[");
        for (i, finding) in findings.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str("{\"code\":\"");
            out.push_str(&esc(&finding.code));
            out.push_str("\",\"severity\":\"");
            out.push_str(finding.severity.name());
            out.push_str("\",\"message\":\"");
            out.push_str(&esc(&finding.message));
            out.push_str("\"}");
        }
        out.push_str("]}\n");
        out
    } else if findings.is_empty() {
        format!(
            "Findings:\n{}\n",
            super::super::copy::lookup("empty-states.cli-clean-map.body")
        )
    } else {
        let mut out = String::from("Findings:\n");
        for finding in findings {
            let _ = writeln!(
                out,
                "{:?}: {} {}",
                finding.severity, finding.code, finding.message
            );
        }
        out
    }
}

pub(crate) fn todo_line(todo: &Todo) -> String {
    format!("{} [{}] {}", todo.node, todo_status(todo.status), todo.path)
}

pub(crate) fn decision_line(decision: &Decision) -> String {
    format!(
        "{} [{}] {}",
        decision.id,
        decision_status(decision.status),
        decision.nodes.join(", ")
    )
}

pub(crate) fn research_line(research: &Research) -> String {
    format!("{} sources: {}", research.id, research.sources.join(", "))
}

pub(crate) fn review_line(review: &Review) -> String {
    format!(
        "{} [{}] {}",
        review.node,
        review_type(review.review_type),
        review.path
    )
}

pub(crate) fn source_line(source: &Source) -> String {
    format!(
        "{} [{}] {}",
        source.id,
        source_verification(source.verification),
        source.file
    )
}

pub(crate) const fn todo_status(status: TodoStatus) -> &'static str {
    match status {
        TodoStatus::Open => "open",
        TodoStatus::InProgress => "in_progress",
        TodoStatus::Done => "done",
        TodoStatus::Blocked => "blocked",
    }
}

pub(crate) const fn decision_status(status: DecisionStatus) -> &'static str {
    match status {
        DecisionStatus::Proposed => "proposed",
        DecisionStatus::Accepted => "accepted",
        DecisionStatus::Deprecated => "deprecated",
        DecisionStatus::Superseded => "superseded",
    }
}

pub(crate) const fn review_type(review_type: ReviewType) -> &'static str {
    match review_type {
        ReviewType::Human => "human",
        ReviewType::AgentIntrospective => "agent_introspective",
        ReviewType::AgentCrossModel => "agent_cross_model",
    }
}

pub(crate) const fn source_verification(verification: SourceVerification) -> &'static str {
    match verification {
        SourceVerification::Verified => "verified",
        SourceVerification::External => "external",
        SourceVerification::Unverified => "unverified",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        artefacts::registry::{DecisionStatus, SourceVerification, TodoStatus},
        blueprint::{NodeKind, Span},
        map::{FindingSeverity, NodeRecord, NodeState},
    };

    #[test]
    fn test_todo_status_roundtrip() {
        use super::super::util::parse_todo_status_filter;
        for (status, name) in [
            (TodoStatus::Open, "open"),
            (TodoStatus::InProgress, "in_progress"),
            (TodoStatus::Done, "done"),
            (TodoStatus::Blocked, "blocked"),
        ] {
            assert_eq!(todo_status(status), name);
            assert_eq!(parse_todo_status_filter(name), Some(status));
        }
    }

    #[test]
    fn test_decision_status_roundtrip() {
        use super::super::util::parse_decision_status_filter;
        for (status, name) in [
            (DecisionStatus::Proposed, "proposed"),
            (DecisionStatus::Accepted, "accepted"),
            (DecisionStatus::Deprecated, "deprecated"),
            (DecisionStatus::Superseded, "superseded"),
        ] {
            assert_eq!(decision_status(status), name);
            assert_eq!(parse_decision_status_filter(name), Some(status));
        }
    }

    #[test]
    fn test_review_type_display_strings() {
        assert_eq!(review_type(ReviewType::Human), "human");
        assert_eq!(
            review_type(ReviewType::AgentIntrospective),
            "agent_introspective"
        );
        assert_eq!(
            review_type(ReviewType::AgentCrossModel),
            "agent_cross_model"
        );
    }

    #[test]
    fn test_source_verification_display_strings() {
        assert_eq!(
            source_verification(SourceVerification::Verified),
            "verified"
        );
        assert_eq!(
            source_verification(SourceVerification::External),
            "external"
        );
        assert_eq!(
            source_verification(SourceVerification::Unverified),
            "unverified"
        );
    }

    // ── line-formatting helpers ──────────────────────────────────────────────

    fn todo(status: TodoStatus) -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: "app".to_owned(),
            status,
            created: "2026-01-01".to_owned(),
            satisfies: None,
            body: String::new(),
        }
    }

    #[test]
    fn test_todo_line_format() {
        assert_eq!(todo_line(&todo(TodoStatus::Open)), "app [open] ./todo.md");
        assert_eq!(
            todo_line(&todo(TodoStatus::InProgress)),
            "app [in_progress] ./todo.md"
        );
    }

    #[test]
    fn test_decision_line_format() {
        let decision = Decision {
            path: "./decision.md".to_owned(),
            id: "adopt-rust".to_owned(),
            status: DecisionStatus::Accepted,
            nodes: vec!["app".to_owned(), "lib".to_owned()],
            date: String::new(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            claims: None,
            body: String::new(),
        };
        assert_eq!(decision_line(&decision), "adopt-rust [accepted] app, lib");
    }

    #[test]
    fn test_research_line_format() {
        let research = Research {
            path: "./research.md".to_owned(),
            id: "r-1".to_owned(),
            nodes: Vec::new(),
            date: String::new(),
            sources: vec!["src-1".to_owned(), "src-2".to_owned()],
            tags: Vec::new(),
            body: String::new(),
        };
        assert_eq!(research_line(&research), "r-1 sources: src-1, src-2");
    }

    #[test]
    fn test_review_line_format() {
        let review = Review {
            path: "./review.md".to_owned(),
            node: "app".to_owned(),
            review_type: ReviewType::Human,
            date: String::new(),
            reviewer: String::new(),
            related_change: None,
            body: String::new(),
        };
        assert_eq!(review_line(&review), "app [human] ./review.md");
    }

    #[test]
    fn test_source_line_format() {
        let source = Source {
            id: "src-1".to_owned(),
            path: "./source.md".to_owned(),
            file: "./source.md".to_owned(),
            sha256: None,
            verification: SourceVerification::External,
            source_type: String::new(),
            date: String::new(),
            tags: Vec::new(),
            description: String::new(),
            body: String::new(),
        };
        assert_eq!(source_line(&source), "src-1 [external] ./source.md");
    }

    // ── render_node / render_findings ─────────────────────────────────────────

    fn sample_node() -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: "app".to_owned(),
            name: "app".to_owned(),
            description: "The app".to_owned(),
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    #[test]
    fn test_render_node_human_format() {
        let rendered = render_node(&sample_node(), false);
        assert!(rendered.contains("ID: app"));
        assert!(rendered.contains("Name: app"));
        assert!(rendered.contains("Description: The app"));
        assert!(rendered.contains("State: Synced"));
    }

    #[test]
    fn test_render_node_json_format() {
        let rendered = render_node(&sample_node(), true);
        assert!(rendered.contains("\"id\":\"app\""));
        assert!(rendered.contains("\"name\":\"app\""));
    }

    #[test]
    fn test_render_findings_empty_json() {
        assert_eq!(render_findings(&[], true), "{\"findings\":[]}\n");
    }

    #[test]
    fn test_render_findings_non_empty_json() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Error,
            message: "bad".to_owned(),
            node: None,
            target: None,
            path: None,
        };
        let rendered = render_findings(&[finding], true);
        assert!(rendered.contains("\"code\":\"CAIRN_TEST\""));
        assert!(rendered.contains("\"severity\":\"error\""));
        assert!(rendered.contains("\"message\":\"bad\""));
    }

    #[test]
    fn test_render_findings_non_empty_human() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Warning,
            message: "look out".to_owned(),
            node: None,
            target: None,
            path: None,
        };
        let rendered = render_findings(&[finding], false);
        assert!(rendered.starts_with("Findings:\n"));
        assert!(rendered.contains("Warning: CAIRN_TEST look out"));
    }
}
