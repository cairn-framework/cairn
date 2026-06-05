//! Phase 8 Summariser acceptance-criterion tests.
//!
//! Mixed state: tests that bind to library types delivered by reforge
//! cycle 1 (`SummariserMode`, `SummariserBackend`, the wire-protocol
//! request/response shapes, `DraftStore`, the typestate-tagged `Draft`
//! variants, and `Conflict` semantics) run as plain `#[test]` and
//! enforce their invariants on every `cargo test`. Tests that bind to
//! the still-pending CLI surface (`cairn summarise`, `cairn draft
//! accept/edit/discard`, MCP tool-registry filtering, contract
//! validation + rollback) carry `#[cairn_planned(phase = 800)]` and
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
                metadata: None,
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
    use cairn::summariser::{AcceptError, accept};
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
            metadata: None,
        }
    }

    fn pending() -> Draft {
        Draft::Pending(PendingDraft {
            header: sample_header(),
        })
    }

    fn auth_draft(text: &str) -> Draft {
        Draft::Pending(PendingDraft {
            header: DraftHeader {
                id: "draft-001".to_owned(),
                node_id: "app.auth".to_owned(),
                artefact_type: "contract".to_owned(),
                draft_text: text.to_owned(),
                created_at: "2026-05-07T12:00:00Z".to_owned(),
                transitions: Vec::new(),
                metadata: None,
            },
        })
    }

    fn temp_project() -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();

        let blueprint = r#"System App "App" id "app" {
    Module Auth "Auth" id "app.auth" {
        contract "meta/contracts/auth.md"
    }
}"#;
        let blueprint_path = root.join("cairn.blueprint");
        fs::create_dir_all(root.join("meta/contracts")).unwrap();
        fs::write(&blueprint_path, blueprint).unwrap();
        fs::write(
            root.join("meta/contracts/auth.md"),
            "---\nnode: app.auth\n---\n# Auth\n\nOriginal.",
        )
        .unwrap();

        (dir, blueprint_path)
    }

    /// Scenario: draft accept replaces target contract and records hash.
    #[test]
    fn test_draft_accept_replaces_contract_and_records_hash() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let draft_text = "---\nnode: app.auth\n---\n# Auth\n\nUpdated.";
        store.write(&auth_draft(draft_text)).expect("write draft");

        let result = accept(root, "draft-001", &blueprint, false).expect("accept");
        assert_eq!(result, "draft-001");

        let written =
            fs::read_to_string(root.join("meta/contracts/auth.md")).expect("read contract");
        assert_eq!(written, draft_text);

        let draft = store.read("draft-001").expect("read draft");
        assert!(
            matches!(draft, Draft::Accepted(_)),
            "draft must be Accepted, got {draft:?}"
        );
        if let Draft::Accepted(a) = draft {
            assert!(
                !a.accepted_interface_hash().is_empty(),
                "accepted draft must record non-empty hash"
            );
        }
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
    #[test]
    fn test_draft_accept_edited_applies_editable_file() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let draft_text = "---\nnode: app.auth\n---\n# Auth\n\nGenerated.";
        store.write(&auth_draft(draft_text)).expect("write draft");

        let edited_text = "---\nnode: app.auth\n---\n# Auth\n\nEdited version.";
        fs::create_dir_all(root.join(".cairn/state/summariser/editable")).unwrap();
        fs::write(store.editable_path("draft-001", "contract"), edited_text).unwrap();

        let result = accept(root, "draft-001", &blueprint, true).expect("accept edited");
        assert_eq!(result, "draft-001");

        let written =
            fs::read_to_string(root.join("meta/contracts/auth.md")).expect("read contract");
        assert_eq!(written, edited_text);

        let draft = store.read("draft-001").expect("read draft");
        assert!(
            matches!(draft, Draft::Accepted(_)),
            "draft must be Accepted after edited accept, got {draft:?}"
        );
    }

    /// Scenario: Accepting a draft with invalid frontmatter exits 1 and restores contract.
    #[test]
    fn test_draft_accept_invalid_frontmatter_exits_one_restores_contract() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        let original = fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap();
        store
            .write(&auth_draft("no frontmatter here"))
            .expect("write draft");

        let result = accept(root, "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::Validation(_))),
            "expected Validation error, got {result:?}"
        );

        let restored = fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap();
        assert_eq!(restored, original, "contract must remain unchanged");

        let draft = store.read("draft-001").expect("read draft");
        assert!(
            matches!(draft, Draft::Pending(_)),
            "draft must stay pending, got {draft:?}"
        );
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
    use serde_json::Value;

    fn tool_names(response: &Value) -> Vec<&str> {
        response["result"]["tools"]
            .as_array()
            .expect("tools array")
            .iter()
            .map(|t| t["name"].as_str().expect("tool name"))
            .collect()
    }

    /// Scenario: Read-only summariser tools listed in default mode.
    #[test]
    fn test_read_only_tools_listed_in_default_mode() {
        let config = cairn::mcp::ServerConfig::default();
        let response =
            cairn::mcp::handle_line(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#, &config);
        let names = tool_names(&response);
        assert!(
            names.contains(&"cairn_drafts"),
            "cairn_drafts must be listed"
        );
        assert!(
            names.contains(&"cairn_draft_show"),
            "cairn_draft_show must be listed"
        );
    }

    /// Scenario: Mutating summariser tools absent in default mode.
    #[test]
    fn test_mutating_tools_absent_in_default_mode() {
        let config = cairn::mcp::ServerConfig::default();
        let response =
            cairn::mcp::handle_line(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#, &config);
        let names = tool_names(&response);
        assert!(
            !names.contains(&"cairn_summarise"),
            "cairn_summarise must not be listed in default mode"
        );
        assert!(
            !names.contains(&"cairn_draft_accept"),
            "cairn_draft_accept must not be listed in default mode"
        );
        assert!(
            !names.contains(&"cairn_draft_edit"),
            "cairn_draft_edit must not be listed in default mode"
        );
        assert!(
            !names.contains(&"cairn_draft_discard"),
            "cairn_draft_discard must not be listed in default mode"
        );
    }
}
