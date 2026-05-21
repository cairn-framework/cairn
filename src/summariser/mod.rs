//! Phase 8 Summariser: pluggable backends, draft store, and typed
//! request/response for `cairn summarise`. The cairn library provides
//! the framework; specific local_command and hosted backends land in
//! future commits.

mod backend;
mod request;
mod store;

pub use backend::{
    DisabledBackend, FakeBackend, LocalCommandBackend, SummariserBackend, SummariserBackendError,
    SummariserMode,
};
pub use request::{NodeContext, SUMMARISER_SCHEMA_VERSION, SummariserRequest, SummariserResponse};
pub use store::{
    AcceptedDraft, DRAFT_SCHEMA_VERSION, DiscardedDraft, Draft, DraftHeader, DraftStore,
    DraftStoreError, EditableDraft, EmptyInterfaceHash, PendingDraft, read_draft,
};
