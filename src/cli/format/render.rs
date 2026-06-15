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
}
