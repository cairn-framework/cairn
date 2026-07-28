//! Accept-time contract-baseline recording and its rollback transaction.
//!
//! `accept()` installs a contract, then records the accepted node's shape into
//! `.cairn/state/contract-baselines.json`. Every step after the post-write scan
//! is fallible and collectively atomic, so these tests pin the committed state
//! and each rollback path.

mod accept_baseline {
    use cairn::summariser::{AcceptError, Draft, DraftHeader, DraftStore, PendingDraft, accept};
    use std::fs;

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

    fn baseline_path(root: &std::path::Path) -> std::path::PathBuf {
        root.join(".cairn/state/contract-baselines.json")
    }

    /// Scenario: a successful accept records the accepted node's baseline.
    #[test]
    fn test_draft_accept_records_contract_baseline() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store
            .write(&auth_draft("---\nnode: app.auth\n---\n# Auth\n\nUpdated."))
            .expect("write draft");

        accept(root, "draft-001", &blueprint, false).expect("accept");

        let raw = fs::read_to_string(baseline_path(root)).expect("baseline file");
        let value: serde_json::Value = serde_json::from_str(&raw).expect("baseline json");
        assert_eq!(value["version"], 1);
        assert_eq!(value["nodes"]["app.auth"]["kind"], "Module");
        assert_eq!(value["nodes"]["app.auth"]["parent"], "app");
        assert!(
            value["nodes"]["app.auth"]["edges"]
                .as_array()
                .expect("edges array")
                .is_empty()
        );
        assert!(
            value["nodes"]["app.auth"].get("paths").is_none(),
            "reduced record must not carry paths: {raw}"
        );
    }

    /// Scenario: re-accepting overwrites only its own node's entry.
    #[test]
    fn test_draft_accept_overwrites_only_its_own_baseline_entry() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        fs::create_dir_all(root.join(".cairn/state")).unwrap();
        fs::write(
            baseline_path(root),
            r#"{"version":1,"nodes":{"app":{"kind":"System","parent":null,"edges":[]},"app.auth":{"kind":"Container","parent":null,"edges":["app.other"]}}}"#,
        )
        .unwrap();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store
            .write(&auth_draft("---\nnode: app.auth\n---\n# Auth\n\nUpdated."))
            .expect("write draft");

        accept(root, "draft-001", &blueprint, false).expect("accept");

        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(baseline_path(root)).expect("baseline file"))
                .expect("baseline json");
        assert_eq!(value["nodes"]["app.auth"]["kind"], "Module");
        assert_eq!(value["nodes"]["app.auth"]["parent"], "app");
        assert_eq!(
            value["nodes"]["app"],
            serde_json::json!({"kind":"System","parent":null,"edges":[]}),
            "an unrelated entry must round-trip unchanged"
        );
    }

    /// Scenario: an accept rolled back by its post-write scan records nothing.
    #[test]
    fn test_rolled_back_accept_records_no_baseline() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        // A duplicate node id makes every scan of this project fail with an
        // Error finding, so the post-write scan rolls the accept back.
        fs::write(
            &blueprint,
            r#"System App "App" id "app" {
    Module Auth "Auth" id "app.auth" {
        contract "meta/contracts/auth.md"
    }
    Module Dup "Dup" id "app.auth" {
    }
}"#,
        )
        .unwrap();
        let original = fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store
            .write(&auth_draft("---\nnode: app.auth\n---\n# Auth\n\nUpdated."))
            .expect("write draft");

        let result = accept(root, "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::ScanFailed(_))),
            "expected ScanFailed, got {result:?}"
        );
        assert!(
            !baseline_path(root).exists(),
            "a rolled-back accept must not create the baseline file"
        );
        assert_eq!(
            fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap(),
            original
        );
    }

    /// Scenario: an unreadable baseline state file rolls the whole accept back.
    /// The post-write scan reads the same file, so an unsupported schema
    /// version surfaces there before the baseline is ever written.
    #[test]
    fn test_unreadable_baseline_state_rolls_the_whole_accept_back() {
        let (dir, blueprint) = temp_project();
        let root = dir.path();
        fs::create_dir_all(root.join(".cairn/state")).unwrap();
        // A schema version the reader refuses.
        let state = r#"{"version":2,"nodes":{}}"#;
        fs::write(baseline_path(root), state).unwrap();
        let original = fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store
            .write(&auth_draft("---\nnode: app.auth\n---\n# Auth\n\nUpdated."))
            .expect("write draft");

        let result = accept(root, "draft-001", &blueprint, false);
        assert!(
            matches!(result, Err(AcceptError::ScanFailed(_))),
            "expected ScanFailed, got {result:?}"
        );
        assert_eq!(
            fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap(),
            original,
            "the installed contract text must be restored"
        );
        assert_eq!(
            fs::read_to_string(baseline_path(root)).unwrap(),
            state,
            "the baseline file must be byte-identical to its prior state"
        );
        let draft = store.read("draft-001").expect("read draft");
        assert!(
            matches!(draft, Draft::Pending(_)),
            "draft must stay pending, got {draft:?}"
        );
    }

    /// Scenario: the baseline write itself failing after the post-write scan
    /// restores the contract and leaves no baseline behind.
    #[cfg(unix)]
    #[test]
    fn test_unwritable_baseline_state_rolls_the_whole_accept_back() {
        use std::os::unix::fs::PermissionsExt;

        let (dir, blueprint) = temp_project();
        let root = dir.path();
        let state_dir = root.join(".cairn/state");
        fs::create_dir_all(&state_dir).unwrap();
        let original = fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store
            .write(&auth_draft("---\nnode: app.auth\n---\n# Auth\n\nUpdated."))
            .expect("write draft");

        fs::set_permissions(&state_dir, fs::Permissions::from_mode(0o555)).unwrap();
        if fs::write(state_dir.join("probe"), b"x").is_ok() {
            // Privileges that ignore the mode bits: nothing to inject.
            fs::set_permissions(&state_dir, fs::Permissions::from_mode(0o755)).unwrap();
            fs::remove_file(state_dir.join("probe")).unwrap();
            return;
        }

        let result = accept(root, "draft-001", &blueprint, false);
        fs::set_permissions(&state_dir, fs::Permissions::from_mode(0o755)).unwrap();

        assert!(
            matches!(result, Err(AcceptError::Baseline(_))),
            "expected Baseline error, got {result:?}"
        );
        assert_eq!(
            fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap(),
            original,
            "the installed contract text must be restored"
        );
        assert!(!baseline_path(root).exists());
    }

    /// Scenario: the draft-store overwrite failing after the baseline was
    /// written rolls the contract, the baseline file, and the draft back.
    #[cfg(unix)]
    #[test]
    fn test_failed_draft_store_write_rolls_the_whole_accept_back() {
        use std::os::unix::fs::PermissionsExt;

        let (dir, blueprint) = temp_project();
        let root = dir.path();
        let original = fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap();
        let store = DraftStore::new(root.join(".cairn/state/summariser"));
        store
            .write(&auth_draft("---\nnode: app.auth\n---\n# Auth\n\nUpdated."))
            .expect("write draft");

        // A read-only pending directory blocks the atomic rewrite of the draft
        // file while leaving it readable, so the accept fails at its last step.
        let pending = store.pending_dir();
        fs::set_permissions(&pending, fs::Permissions::from_mode(0o555)).unwrap();
        let bypasses_permissions = fs::write(pending.join("probe"), b"x").is_ok();
        if bypasses_permissions {
            // Running with privileges that ignore the mode bits: the failure
            // cannot be injected, so there is nothing to assert.
            fs::set_permissions(&pending, fs::Permissions::from_mode(0o755)).unwrap();
            fs::remove_file(pending.join("probe")).unwrap();
            return;
        }

        let result = accept(root, "draft-001", &blueprint, false);
        fs::set_permissions(&pending, fs::Permissions::from_mode(0o755)).unwrap();

        assert!(
            matches!(result, Err(AcceptError::DraftStore(_))),
            "expected DraftStore error, got {result:?}"
        );
        assert_eq!(
            fs::read_to_string(root.join("meta/contracts/auth.md")).unwrap(),
            original,
            "the installed contract text must be restored"
        );
        assert!(
            !baseline_path(root).exists(),
            "a baseline this call created must be removed again"
        );
        let draft = store.read("draft-001").expect("read draft");
        assert!(
            matches!(draft, Draft::Pending(_)),
            "draft must stay pending, got {draft:?}"
        );
    }
}
