//! Phase 8 Summariser: pluggable backends, draft store, and typed
//! request/response for `cairn summarise`. The cairn library provides
//! the framework; specific local_command and hosted backends land in
//! future commits.

mod backend;
mod request;
mod store;

pub use backend::{DisabledBackend, SummariserBackend, SummariserBackendError, SummariserMode};
pub use request::{NodeContext, SummariserRequest, SummariserResponse};
pub use store::{
    Draft, DraftStatus, DraftStore, DraftStoreError, REQUEST_SCHEMA_VERSION, read_draft,
};
