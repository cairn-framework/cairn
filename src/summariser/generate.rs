//! Draft generation orchestration.
//!
//! Bridges the backend trait and the draft store: builds a request,
//! invokes the backend, and persists the response as a pending draft.

use std::time::Duration;

use crate::summariser::{
    backend::{SummariserBackend, SummariserBackendError},
    request::SummariserRequest,
    store::{Draft, DraftHeader, DraftStore, DraftStoreError, PendingDraft},
};

/// Error during draft generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerateError {
    /// Backend invocation failed.
    Backend(SummariserBackendError),
    /// Draft store operation failed.
    Store(DraftStoreError),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(e) => write!(f, "backend failed: {e}"),
            Self::Store(e) => write!(f, "store failed: {e}"),
        }
    }
}

impl std::error::Error for GenerateError {}

impl From<SummariserBackendError> for GenerateError {
    fn from(e: SummariserBackendError) -> Self {
        Self::Backend(e)
    }
}

impl From<DraftStoreError> for GenerateError {
    fn from(e: DraftStoreError) -> Self {
        Self::Store(e)
    }
}

/// Generates a draft by invoking `backend` with `request` and persisting
/// the response as a `PendingDraft` in `store`.
///
/// Returns the draft id on success.
///
/// # Errors
///
/// Returns `GenerateError::Backend` when the backend invocation fails,
/// and `GenerateError::Store` when the draft cannot be written.
pub fn generate(
    backend: &dyn SummariserBackend,
    request: &SummariserRequest,
    timeout: Duration,
    store: &DraftStore,
    draft_id: &str,
    created_at: &str,
) -> Result<String, GenerateError> {
    let response = backend.invoke(request, timeout)?;
    let draft = Draft::Pending(PendingDraft {
        header: DraftHeader {
            id: draft_id.to_owned(),
            node_id: request.node.node_id.clone(),
            artefact_type: request.artefact_type.clone(),
            draft_text: response.summary,
            created_at: created_at.to_owned(),
        },
    });
    store.write(&draft)?;
    Ok(draft_id.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summariser::{
        backend::FakeBackend,
        request::{NodeContext, SUMMARISER_SCHEMA_VERSION, SummariserRequest, SummariserResponse},
        store::read_draft,
    };

    fn sample_request() -> SummariserRequest {
        SummariserRequest {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            artefact_type: "contract".to_owned(),
            node: NodeContext {
                node_id: "app.auth".to_owned(),
                name: "Auth".to_owned(),
                description: String::new(),
                contract: None,
                contradiction: None,
            },
        }
    }

    fn temp_store() -> DraftStore {
        let dir = tempfile::tempdir().unwrap();
        DraftStore::new(dir.path())
    }

    #[test]
    fn test_generate_creates_pending_draft() {
        let response = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "generated contract".to_owned(),
            metadata: None,
        };
        let backend = FakeBackend::ok(response);
        let store = temp_store();
        let draft_id = generate(
            &backend,
            &sample_request(),
            Duration::from_secs(1),
            &store,
            "draft-001",
            "2024-01-15T10:30:00Z",
        )
        .expect("should succeed");
        assert_eq!(draft_id, "draft-001");

        let path = store.pending_dir().join("draft-001.json");
        assert!(path.exists(), "draft file should exist");
    }

    #[test]
    fn test_generate_returns_backend_error() {
        let backend = FakeBackend::err(SummariserBackendError::Io("network down".to_owned()));
        let store = temp_store();
        let result = generate(
            &backend,
            &sample_request(),
            Duration::from_secs(1),
            &store,
            "draft-002",
            "2024-01-15T10:30:00Z",
        );
        match result {
            Err(GenerateError::Backend(SummariserBackendError::Io(msg))) => {
                assert_eq!(msg, "network down");
            }
            other => panic!("expected Backend Io error, got {other:?}"),
        }
    }

    #[test]
    fn test_generate_returns_store_conflict() {
        let response = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "first".to_owned(),
            metadata: None,
        };
        let backend = FakeBackend::ok(response);
        let store = temp_store();
        generate(
            &backend,
            &sample_request(),
            Duration::from_secs(1),
            &store,
            "draft-003",
            "2024-01-15T10:30:00Z",
        )
        .expect("first write should succeed");

        let response2 = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "second".to_owned(),
            metadata: None,
        };
        let backend2 = FakeBackend::ok(response2);
        let result = generate(
            &backend2,
            &sample_request(),
            Duration::from_secs(1),
            &store,
            "draft-003",
            "2024-01-15T10:31:00Z",
        );
        assert!(
            matches!(
                result,
                Err(GenerateError::Store(DraftStoreError::Conflict(_)))
            ),
            "expected Store Conflict error, got {result:?}"
        );
    }

    #[test]
    fn test_generate_draft_contains_node_id() {
        let response = SummariserResponse {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            summary: "draft body".to_owned(),
            metadata: None,
        };
        let backend = FakeBackend::ok(response);
        let store = temp_store();
        generate(
            &backend,
            &sample_request(),
            Duration::from_secs(1),
            &store,
            "draft-004",
            "2024-01-15T10:30:00Z",
        )
        .unwrap();

        let draft = read_draft(&store.pending_dir().join("draft-004.json")).unwrap();
        assert_eq!(draft.id(), "draft-004");
        match draft {
            Draft::Pending(d) => {
                assert_eq!(d.header.node_id, "app.auth");
                assert_eq!(d.header.draft_text, "draft body");
            }
            other => panic!("expected Pending draft, got {other:?}"),
        }
    }
}
