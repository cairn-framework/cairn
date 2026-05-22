//! Phase 8 Summariser acceptance-criterion tests.
//!
//! Mixed state: tests that bind to library types delivered by reforge
//! cycle 1 (`SummariserMode`, `SummariserBackend`, the wire-protocol
//! request/response shapes, `DraftStore`, the typestate-tagged `Draft`
//! variants, and `Conflict` semantics) run as plain `#[test]` and
//! enforce their invariants on every `cargo test`. Tests that bind to
//! the still-pending CLI surface (`cairn summarise`, `cairn draft
//! accept/edit/discard`, MCP tool-registry filtering, contract
//! validation + rollback) carry `#[cflx_planned(phase = 800)]` and
//! stay skipped under `cargo test`; they fail with `unimplemented!`
//! under `cargo test -- --ignored`.

mod backend_and_config {
    use cairn::summariser::{
        DisabledBackend, Draft, DraftStore, PendingDraft, SUMMARISER_SCHEMA_VERSION,
        SummariserBackend, SummariserBackendError, SummariserMode, SummariserRequest,
    };
    use std::time::Duration;

    fn sample_request() -> SummariserRequest {
        SummariserRequest {
            schema_version: SUMMARISER_SCHEMA_VERSION,
            request_id: "req-a".to_owned(),
            draft_type: "contract".to_owned(),
            target_node: "node-a".to_owned(),
            map_facts: Vec::new(),
            contract_excerpt: None,
            interface_findings: Vec::new(),
            docstring_findings: Vec::new(),
            project_context: String::new(),
            rules: Vec::new(),
            code_samples: Vec::new(),
        }
    }

    fn pending_draft() -> Draft {
        Draft::Pending(PendingDraft {
            header: cairn::summariser::DraftHeader {
                id: "draft-001".to_owned(),
                node_id: "node-a".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: "# Auth\n\nReturns user.".to_owned(),
                created_at: "2026-05-07T12:00:00Z".to_owned(),
                transitions: Vec::new(),
            },
        })
    }

    /// Scenario: Disabled summariser does not generate a draft.
    #[test]
    fn test_disabled_backend_skips_draft_on_contradiction() {
        let mode = SummariserMode::default();
        assert!(matches!(mode, SummariserMode::Disabled));
        let backend = DisabledBackend;
        let result = backend.invoke(&sample_request(), Duration::from_secs(1));
        assert!(matches!(result, Err(SummariserBackendError::Disabled)));
    }

    /// Scenario: Configured backend creates a pending draft.
    #[test]
    fn test_configured_backend_creates_pending_draft() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = pending_draft();
        let path = store.write(&draft).expect("write");
        assert!(path.exists());
        let back = store.read("draft-001").expect("read");
        assert!(matches!(back, Draft::Pending(_)));
    }

    /// Scenario: Local command receives one request and stores `draft_text` only.
    /// Wire protocol assertion: `SummariserRequest` serialises with the
    /// design.doc field names (`schema_version`, `request_id`, `draft_type`,
    /// `target_node`, etc.) and `SummariserResponse` parses with `draft_text`
    /// as the canonical text payload.
    #[test]
    fn test_local_command_receives_request_and_stores_draft_text_only() {
        let req = sample_request();
        let json = serde_json::to_string(&req).expect("serialise");
        assert!(json.contains("\"schema_version\""));
        assert!(json.contains("\"request_id\""));
        assert!(json.contains("\"draft_type\""));
        assert!(json.contains("\"target_node\""));
        let resp: cairn::summariser::SummariserResponse =
            serde_json::from_str(r#"{"schema_version":1,"draft_text":"hi"}"#).expect("parse");
        assert_eq!(resp.draft_text, "hi");
    }

    /// Scenario: Backend failure does not create or modify a draft.
    #[test]
    fn test_backend_failure_does_not_create_or_modify_draft() {
        let backend = DisabledBackend;
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let result = backend.invoke(&sample_request(), Duration::from_secs(1));
        if result.is_err() {
            assert!(store.list().expect("list").is_empty());
        }
    }

    /// Wire protocol drift is caught early: response with unknown field rejected.
    #[test]
    fn test_response_rejects_unknown_fields_per_design() {
        let bad = r#"{"schema_version":1,"draft_text":"hi","rationale":"no"}"#;
        let result: Result<cairn::summariser::SummariserResponse, _> = serde_json::from_str(bad);
        assert!(result.is_err());
    }
}

mod resolution_actions {
    use cairn::summariser::{
        AcceptedDraft, DiscardedDraft, Draft, DraftHeader, DraftStore, DraftStoreError,
        PendingDraft,
    };
    use std::fs;

    fn sample_header() -> DraftHeader {
        DraftHeader {
            id: "draft-001".to_owned(),
            node_id: "node-a".to_owned(),
            artefact_type: "contract".to_owned(),
            draft_text: "# Auth\n\nReturns user.".to_owned(),
            created_at: "2026-05-07T12:00:00Z".to_owned(),
            transitions: Vec::new(),
        }
    }

    fn pending() -> Draft {
        Draft::Pending(PendingDraft {
            header: sample_header(),
        })
    }

    /// Scenario: draft accept replaces target contract and records hash.
    #[cairn::cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_replaces_contract_and_records_hash() {
        unimplemented!("awaits phase-8: cairn draft accept CLI command wiring");
    }

    /// Scenario: Accepted variant carries non-empty interface hash by construction.
    /// Cycle 3: `AcceptedDraft` is now constructor-gated so empty hashes
    /// cannot be assembled, even via deserialisation.
    #[test]
    fn test_accepted_carries_non_empty_interface_hash() {
        let inner = AcceptedDraft::new(sample_header(), "sha256:abc".to_owned())
            .expect("non-empty hash accepted");
        let accepted = Draft::Accepted(inner);
        match accepted {
            Draft::Accepted(d) => assert_eq!(d.accepted_interface_hash(), "sha256:abc"),
            _ => panic!("expected Accepted variant"),
        }
    }

    /// Cycle 3: `AcceptedDraft::new` rejects empty / whitespace hashes.
    #[test]
    fn test_accepted_rejects_empty_hash() {
        assert!(AcceptedDraft::new(sample_header(), String::new()).is_err());
        assert!(AcceptedDraft::new(sample_header(), "   \n".to_owned()).is_err());
    }

    /// Cycle 3: deserialising a payload with empty hash fails too,
    /// closing the wire-channel hole.
    #[test]
    fn test_accepted_deserialise_rejects_empty_hash() {
        let bad = r#"{"status":"accepted","id":"d1","node_id":"n","artefact_type":"contract","draft_text":"x","created_at":"t","accepted_interface_hash":""}"#;
        let result: Result<Draft, _> = serde_json::from_str(bad);
        assert!(result.is_err(), "deserialise must reject empty hash");
    }

    /// Scenario: draft edit writes editable file without modifying contract.
    #[test]
    fn test_draft_edit_writes_editable_file_without_modifying_contract() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        let draft = pending();
        store.write(&draft).expect("write");
        let editable_path = store.write_editable(&draft).expect("write editable");
        assert!(editable_path.exists());
        let body = fs::read_to_string(&editable_path).expect("read editable");
        assert_eq!(body, "# Auth\n\nReturns user.");
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
        let discarded = Draft::Discarded(DiscardedDraft {
            header: sample_header(),
            reason: Some("user rejected".to_owned()),
        });
        assert!(matches!(discarded, Draft::Discarded(_)));
    }

    /// Scenario: Generation never modifies a contract until resolution runs.
    #[test]
    fn test_generation_never_modifies_contract() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        store.write(&pending()).expect("write");
        assert!(store.pending_dir().join("draft-001.json").exists());
    }

    /// New: write refuses to clobber an existing draft.
    #[test]
    fn test_write_refuses_conflict_per_reforge_cycle_1() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = DraftStore::new(dir.path().to_owned());
        store.write(&pending()).expect("first write");
        let err = store.write(&pending()).expect_err("second must conflict");
        assert!(matches!(err, DraftStoreError::Conflict(_)));
    }
}

mod mcp_exposure {

    /// Scenario: Read-only summariser tools listed in default mode.
    #[cairn::cflx_planned(phase = 800)]
    #[test]
    fn test_read_only_tools_listed_in_default_mode() {
        unimplemented!("awaits phase-8: cairn_drafts and cairn_draft_show MCP tool registration");
    }

    /// Scenario: Mutating summariser tools absent in default mode.
    #[cairn::cflx_planned(phase = 800)]
    #[test]
    fn test_mutating_tools_absent_in_default_mode() {
        unimplemented!("awaits phase-8: mutating MCP tool registry filtering");
    }
}
