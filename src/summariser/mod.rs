//! Phase 8 Summariser: pluggable backends, draft store, and typed
//! request/response for `cairn summarise`. The cairn library provides
//! the framework; specific local_command and hosted backends land in
//! future commits.

mod accept;
mod backend;
mod generate;
mod request;
mod store;

pub use accept::{AcceptError, accept};
pub use backend::{
    DisabledBackend, FakeBackend, HostedBackend, HostedConfig, LocalCommandBackend,
    SummariserBackend, SummariserBackendError, SummariserMode,
};
pub use generate::{GenerateError, generate};
pub use request::{CodeSample, SUMMARISER_SCHEMA_VERSION, SummariserRequest, SummariserResponse};
pub use store::{
    AcceptedDraft, DRAFT_SCHEMA_VERSION, DiscardedDraft, Draft, DraftHeader, DraftStatus,
    DraftStore, DraftStoreError, DraftTransitionError, EditableDraft, EmptyInterfaceHash,
    PendingDraft, TransitionRecord, read_draft, validate_transition,
};

/// Safety assertion: every mutating draft tool has a corresponding
/// registry entry. If this panics, a resolution action was added
/// without registering it in `query_api::registry`.
///
/// # Panics
///
/// Panics if `draft_discard`, `draft_edit`, or `draft_accept` are not
/// present in the tool registry.
pub fn assert_draft_tools_registered() {
    let names: std::collections::HashSet<_> = crate::query_api::registry()
        .iter()
        .map(|entry| entry.cli_name)
        .collect();
    assert!(
        names.contains("draft_discard"),
        "draft_discard must be registered"
    );
    assert!(
        names.contains("draft_edit"),
        "draft_edit must be registered"
    );
    assert!(
        names.contains("draft_accept"),
        "draft_accept must be registered"
    );
}

/// Safety assertion: every registered draft tool has the correct
/// safety class. Read-only tools must never mutate; mutating tools
/// must be gated by the MCP mutating-tool flow.
///
/// # Panics
///
/// Panics if `drafts` or `draft_show` are not read-only, or if
/// `draft_discard`, `draft_edit`, or `draft_accept` are not mutating.
pub fn assert_draft_tool_safety_classes() {
    use crate::query_api::SafetyClass;
    let readonly: std::collections::HashSet<_> = crate::query_api::visible_tools(false)
        .iter()
        .map(|tool| tool.cli_name)
        .collect();
    let mutating: std::collections::HashSet<_> = crate::query_api::visible_tools(true)
        .iter()
        .filter(|tool| tool.safety == SafetyClass::Mutating)
        .map(|tool| tool.cli_name)
        .collect();

    assert!(readonly.contains("drafts"), "drafts must be read-only");
    assert!(
        readonly.contains("draft_show"),
        "draft_show must be read-only"
    );
    assert!(
        mutating.contains("draft_discard"),
        "draft_discard must be mutating"
    );
    assert!(
        mutating.contains("draft_edit"),
        "draft_edit must be mutating"
    );
    assert!(
        mutating.contains("draft_accept"),
        "draft_accept must be mutating"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_one_registry_safety_classes() {
        assert_draft_tools_registered();
        assert_draft_tool_safety_classes();
    }
}

#[test]
fn test_package_name_returns_cairn() {
    assert_eq!(env!("CARGO_PKG_NAME"), "cairn");
}

#[test]
fn test_package_version_is_not_empty() {
    assert!(!env!("CARGO_PKG_VERSION").is_empty());
}

#[test]
fn test_version_label_includes_name_and_version() {
    let label = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    assert!(label.starts_with("cairn v"));
}
