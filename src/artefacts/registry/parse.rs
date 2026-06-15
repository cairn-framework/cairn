//! Frontmatter value parsing for artefacts.

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
#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn path() -> &'static Path {
        Path::new("meta/artefacts/test.md")
    }

    // ── parse_todo_status ─────────────────────────────────────────────────────

    #[test]
    fn test_todo_status_all_variants() {
        let cases = [
            ("open", Some(TodoStatus::Open)),
            ("in_progress", Some(TodoStatus::InProgress)),
            ("done", Some(TodoStatus::Done)),
            ("blocked", Some(TodoStatus::Blocked)),
        ];
        for (value, expected) in cases {
            let mut set = ArtefactSet::default();
            assert_eq!(
                parse_todo_status(value, path(), &mut set),
                expected,
                "'{value}' must parse to {expected:?}"
            );
            assert!(
                set.findings.is_empty(),
                "valid status must produce no findings"
            );
        }
    }

    #[test]
    fn test_todo_status_invalid_emits_finding() {
        let mut set = ArtefactSet::default();
        let result = parse_todo_status("unknown", path(), &mut set);
        assert!(result.is_none(), "invalid status must return None");
        assert_eq!(set.findings.len(), 1);
        assert_eq!(set.findings[0].code, "CAIRN_TODO_STATUS_INVALID");
    }

    // ── parse_decision_status ─────────────────────────────────────────────────

    #[test]
    fn test_decision_status_all_variants() {
        let cases = [
            ("proposed", Some(DecisionStatus::Proposed)),
            ("accepted", Some(DecisionStatus::Accepted)),
            ("deprecated", Some(DecisionStatus::Deprecated)),
            ("superseded", Some(DecisionStatus::Superseded)),
        ];
        for (value, expected) in cases {
            let mut set = ArtefactSet::default();
            assert_eq!(
                parse_decision_status(value, path(), &mut set),
                expected,
                "'{value}' must parse to {expected:?}"
            );
            assert!(
                set.findings.is_empty(),
                "valid status must produce no findings"
            );
        }
    }

    /// "binding" is a legacy alias for "accepted". On-disk fixtures that still
    /// carry it must continue to load correctly; removing this alias is a
    /// breaking change that requires a migration sweep of all decision files.
    #[test]
    fn test_decision_status_binding_alias_is_accepted() {
        let mut set = ArtefactSet::default();
        assert_eq!(
            parse_decision_status("binding", path(), &mut set),
            Some(DecisionStatus::Accepted),
            "'binding' legacy alias must map to Accepted"
        );
        assert!(set.findings.is_empty());
    }

    #[test]
    fn test_decision_status_invalid_emits_finding() {
        let mut set = ArtefactSet::default();
        let result = parse_decision_status("approved", path(), &mut set);
        assert!(result.is_none());
        assert_eq!(set.findings[0].code, "CAIRN_DECISION_STATUS_INVALID");
    }

    // ── parse_review_type ─────────────────────────────────────────────────────

    #[test]
    fn test_review_type_all_variants() {
        let cases = [
            ("human", Some(ReviewType::Human)),
            ("agent_introspective", Some(ReviewType::AgentIntrospective)),
            ("agent_cross_model", Some(ReviewType::AgentCrossModel)),
        ];
        for (value, expected) in cases {
            let mut set = ArtefactSet::default();
            assert_eq!(
                parse_review_type(value, path(), &mut set),
                expected,
                "'{value}' must parse to {expected:?}"
            );
            assert!(set.findings.is_empty());
        }
    }

    #[test]
    fn test_review_type_invalid_emits_finding() {
        let mut set = ArtefactSet::default();
        let result = parse_review_type("peer", path(), &mut set);
        assert!(result.is_none());
        assert_eq!(set.findings[0].code, "CAIRN_REVIEW_TYPE_INVALID");
    }

    // ── parse_source_verification ─────────────────────────────────────────────

    #[test]
    fn test_source_verification_all_variants() {
        let cases = [
            ("verified", Some(SourceVerification::Verified)),
            ("external", Some(SourceVerification::External)),
            ("unverified", Some(SourceVerification::Unverified)),
        ];
        for (value, expected) in cases {
            let mut set = ArtefactSet::default();
            assert_eq!(
                parse_source_verification(value, path(), &mut set),
                expected,
                "'{value}' must parse to {expected:?}"
            );
            assert!(set.findings.is_empty());
        }
    }

    #[test]
    fn test_source_verification_invalid_emits_finding() {
        let mut set = ArtefactSet::default();
        let result = parse_source_verification("trusted", path(), &mut set);
        assert!(result.is_none());
        assert_eq!(set.findings[0].code, "CAIRN_SOURCE_VERIFICATION_INVALID");
    }
}
