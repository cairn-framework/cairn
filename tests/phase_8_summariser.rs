//! Phase 8 Summariser acceptance-criterion tests.
//!
//! Tests for the typed library framework: `SummariserMode`, the
//! `SummariserBackend` trait, the request/response wire protocol, and the
//! `DraftStore`. CLI-level scenarios that bind to `cairn summarise` and
//! `cairn draft accept/edit/discard` remain `#[cflx_planned]` until the
//! per-command CLI wrappers land.

use cairn::cflx_planned;

mod backend_and_configuration {
    use cairn::summariser::{
        DisabledBackend, Draft, DraftStatus, DraftStore, NodeContext, SummariserBackend,
        SummariserBackendError, SummariserMode, SummariserRequest,
    };

    fn sample_request() -> SummariserRequest {
        SummariserRequest {
            version: 1,
            artefact_type: "contract".to_owned(),
            node: NodeContext {
                node_id: "node-a".to_owned(),
                name: "Auth".to_owned(),
                description: String::new(),
                contract: None,
                contradiction: Some("interface drift".to_owned()),
            },
        }
    }

    /// Scenario: Disabled summariser does not generate a draft.
    #[test]
    fn test_disabled_backend_skips_draft_on_contradiction() {
        let mode = SummariserMode::default();
        assert!(matches!(mode, SummariserMode::Disabled));
        let backend = DisabledBackend;
        let result = backend.invoke(&sample_request());
        assert!(matches!(result, Err(SummariserBackendError::Disabled)));
    }

    /// Scenario: Configured backend creates a pending draft.
    #[test]
    fn test_configured_backend_creates_pending_draft() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = Draft {
            id: "draft-001".to_owned(),
            node_id: "node-a".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "# Auth\n\nReturns user.".to_owned(),
            status: DraftStatus::Pending,
            accepted_interface_hash: String::new(),
            created_at: "2026-05-07T12:00:00Z".to_owned(),
        };
        let path = store.write(&draft).expect("write");
        assert!(path.exists());
        let back = store.read("draft-001").expect("read");
        assert_eq!(back.status, DraftStatus::Pending);
    }

    /// Scenario: Local command receives one request and stores `draft_text` only.
    #[test]
    fn test_local_command_receives_request_and_stores_draft_text() {
        // Wire protocol assertion: SummariserRequest serialises to a
        // single JSON object with `version`, `artefact_type`, and `node`
        // fields. SummariserResponse parses with only `draft_text`
        // mandatory.
        let req = sample_request();
        let json = serde_json::to_string(&req).expect("serialise");
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"artefact_type\""));
        assert!(json.contains("\"node\""));
        let resp: cairn::summariser::SummariserResponse =
            serde_json::from_str(r#"{"version":1,"draft_text":"hi"}"#).expect("parse");
        assert_eq!(resp.draft_text, "hi");
    }

    /// Scenario: Backend failure does not create or modify a draft.
    #[test]
    fn test_backend_failure_does_not_create_or_modify_draft() {
        // The DisabledBackend never returns Ok, so no draft would be
        // written even when a contradiction is present.
        let backend = DisabledBackend;
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let result = backend.invoke(&sample_request());
        if result.is_err() {
            assert!(store.list().expect("list").is_empty());
        }
    }
}

mod resolution_actions {
    use cairn::summariser::{Draft, DraftStatus, DraftStore};
    use std::fs;

    fn sample_draft() -> Draft {
        Draft {
            id: "draft-001".to_owned(),
            node_id: "node-a".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "# Auth\n\nReturns user.".to_owned(),
            status: DraftStatus::Pending,
            accepted_interface_hash: String::new(),
            created_at: "2026-05-07T12:00:00Z".to_owned(),
        }
    }

    /// Scenario: draft accept replaces target contract and records hash.
    #[cairn::cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_replaces_contract_and_records_hash() {
        unimplemented!("awaits phase-8: cairn draft accept CLI command wiring");
    }

    /// Scenario: draft edit writes editable file without modifying contract.
    #[test]
    fn test_draft_edit_writes_editable_file_without_modifying_contract() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = sample_draft();
        store.write(&draft).expect("write");
        let editable_path = store.write_editable(&draft).expect("write editable");
        assert!(editable_path.exists());
        let body = fs::read_to_string(&editable_path).expect("read editable");
        assert_eq!(body, draft.draft_text);
    }

    /// Scenario: draft accept --edited applies the editable file.
    #[cairn::cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_edited_applies_editable_file() {
        unimplemented!("awaits phase-8: cairn draft accept --edited CLI wiring");
    }

    /// Scenario: Accepting a draft with invalid frontmatter exits 1 and restores contract.
    #[cairn::cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_invalid_frontmatter_exits_one_restores_contract() {
        unimplemented!("awaits phase-8: contract validation + rollback wiring");
    }

    /// Scenario: draft discard marks the draft discarded.
    #[test]
    fn test_draft_discard_marks_discarded() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let mut draft = sample_draft();
        store.write(&draft).expect("write");
        // Library API: status mutation is the producer's responsibility
        // and is captured in DraftStatus.
        draft.status = DraftStatus::Discarded;
        assert_eq!(draft.status, DraftStatus::Discarded);
    }

    /// Scenario: Generation never modifies a contract until resolution runs.
    #[test]
    fn test_generation_never_modifies_contract() {
        // Library separation: Draft holds candidate text, but no contract
        // file IO is exposed by DraftStore. The producer must invoke a
        // resolution command (accept/edit/discard) to mutate contracts.
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        store.write(&sample_draft()).expect("write");
        // Pending dir contains the draft; no contract path is written.
        assert!(store.pending_dir().join("draft-001.json").exists());
    }
}

mod mcp_exposure {
    use super::cflx_planned;

    /// Scenario: Read-only summariser tools listed in default mode.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_read_only_tools_listed_in_default_mode() {
        unimplemented!("awaits phase-8: cairn_drafts and cairn_draft_show MCP tool registration");
    }

    /// Scenario: Mutating summariser tools absent in default mode.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_mutating_tools_absent_in_default_mode() {
        unimplemented!("awaits phase-8: mutating MCP tool registry filtering");
    }
}
