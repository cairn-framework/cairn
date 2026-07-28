//! Phase 8 Summariser: pluggable backends, draft store, and typed
//! request/response for `cairn draft create`. The cairn library provides
//! the framework; specific local_command and hosted backends land in
//! future commits.

mod accept;
mod backend;
mod baseline;
mod config;
mod generate;
mod prompt;
mod request;
mod store;

pub use accept::{AcceptError, accept};
pub use backend::{
    DisabledBackend, FakeBackend, HostedBackend, HostedConfig, LocalCommandBackend,
    SummariserBackend, SummariserBackendError, SummariserMode,
};
pub(crate) use baseline::{BaselineError, drop_baseline, record_baseline};
pub use config::SummariserSettings;
pub use generate::{GenerateError, generate};
pub use prompt::{PromptError, build_request};
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
/// Panics if `draft discard`, `draft edit`, `draft accept`, or `draft create` are not
/// present in the tool registry.
pub fn assert_draft_tools_registered() {
    let names: std::collections::HashSet<_> = crate::query_api::registry()
        .iter()
        .map(|entry| entry.cli_name)
        .collect();
    assert!(
        names.contains("draft discard"),
        "draft discard must be registered"
    );
    assert!(
        names.contains("draft edit"),
        "draft edit must be registered"
    );
    assert!(
        names.contains("draft accept"),
        "draft accept must be registered"
    );
    assert!(
        names.contains("draft create"),
        "draft create must be registered"
    );
}

/// Safety assertion: every registered draft tool has the correct
/// safety class. Read-only tools must never mutate; mutating tools
/// must be gated by the MCP mutating-tool flow.
///
/// # Panics
///
/// Panics if `draft list` or `draft show` are not read-only, or if
/// `draft discard`, `draft edit`, `draft accept`, or `draft create` are not
/// mutating.
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

    assert!(
        readonly.contains("draft list"),
        "draft list must be read-only"
    );
    assert!(
        readonly.contains("draft show"),
        "draft show must be read-only"
    );
    assert!(
        mutating.contains("draft discard"),
        "draft discard must be mutating"
    );
    assert!(
        mutating.contains("draft edit"),
        "draft edit must be mutating"
    );
    assert!(
        mutating.contains("draft accept"),
        "draft accept must be mutating"
    );
    assert!(
        mutating.contains("draft create"),
        "draft create must be mutating"
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
