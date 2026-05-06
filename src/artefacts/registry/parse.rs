//! Frontmatter parsers that turn artefact files into typed registry entries.
// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::io::error_finding;
use super::*;

pub(super) fn parse_todo_status(
    value: &str,
    path: &Path,
    set: &mut ArtefactSet,
) -> Option<TodoStatus> {
    match value {
        "open" => Some(TodoStatus::Open),
        "in_progress" => Some(TodoStatus::InProgress),
        "done" => Some(TodoStatus::Done),
        "blocked" => Some(TodoStatus::Blocked),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_TODO_STATUS_INVALID",
                format!("todo `{}` has invalid status `{value}`", path.display()),
                Some(path_string(path)),
            ));
            None
        }
    }
}

pub(super) fn parse_decision_status(
    value: &str,
    path: &Path,
    set: &mut ArtefactSet,
) -> Option<DecisionStatus> {
    match value {
        "proposed" => Some(DecisionStatus::Proposed),
        // `binding` is a legacy alias for `accepted`; deserialize-only, on-disk
        // fixtures that still carry it keep parsing.
        "accepted" | "binding" => Some(DecisionStatus::Accepted),
        "deprecated" => Some(DecisionStatus::Deprecated),
        "superseded" => Some(DecisionStatus::Superseded),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_DECISION_STATUS_INVALID",
                format!("decision `{}` has invalid status `{value}`", path.display()),
                Some(path_string(path)),
            ));
            None
        }
    }
}

pub(super) fn parse_review_type(
    value: &str,
    path: &Path,
    set: &mut ArtefactSet,
) -> Option<ReviewType> {
    match value {
        "human" => Some(ReviewType::Human),
        "agent_introspective" => Some(ReviewType::AgentIntrospective),
        "agent_cross_model" => Some(ReviewType::AgentCrossModel),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_REVIEW_TYPE_INVALID",
                format!(
                    "review `{}` has invalid review_type `{value}`",
                    path.display()
                ),
                Some(path_string(path)),
            ));
            None
        }
    }
}

pub(super) fn parse_source_verification(
    value: &str,
    path: &Path,
    set: &mut ArtefactSet,
) -> Option<SourceVerification> {
    match value {
        "verified" => Some(SourceVerification::Verified),
        "external" => Some(SourceVerification::External),
        "unverified" => Some(SourceVerification::Unverified),
        _ => {
            set.findings.push(error_finding(
                "CAIRN_SOURCE_VERIFICATION_INVALID",
                format!(
                    "source `{}` has invalid verification `{value}`",
                    path.display()
                ),
                Some(path_string(path)),
            ));
            None
        }
    }
}
