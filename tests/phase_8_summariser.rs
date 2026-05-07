//! Phase 8 Summariser acceptance-criterion tests.
//!
//! Test contract for `phase-8-summariser`. Each test corresponds to one
//! acceptance-criterion scenario across the three summariser requirements
//! (Backend & Configuration, Resolution Actions, MCP Exposure). Tests are
//! marked `#[cflx_planned(phase = 800)]` so `cargo test` skips them while
//! `cargo test -- --ignored` runs them and they fail with a clear
//! `unimplemented!` message naming the scenario. Phase 8 will remove
//! `#[cflx_planned]` group-by-group as code lands.

use cairn::cflx_planned;

mod backend_and_configuration {
    use super::cflx_planned;

    /// Scenario: Disabled summariser does not generate a draft.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_disabled_backend_skips_draft_on_contradiction() {
        unimplemented!("awaits phase-8: disabled summariser does not generate a draft");
    }

    /// Scenario: Configured backend creates a pending draft.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_configured_backend_creates_pending_draft() {
        unimplemented!("awaits phase-8: configured backend creates pending draft");
    }

    /// Scenario: Local command receives one request and stores `draft_text` only.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_local_command_receives_request_and_stores_draft_text() {
        unimplemented!("awaits phase-8: local command receives one request, stores draft_text");
    }

    /// Scenario: Backend failure does not create or modify a draft.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_backend_failure_does_not_create_or_modify_draft() {
        unimplemented!("awaits phase-8: backend failure does not create or modify draft");
    }
}

mod resolution_actions {
    use super::cflx_planned;

    /// Scenario: draft accept replaces target contract and records hash.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_replaces_contract_and_records_hash() {
        unimplemented!("awaits phase-8: draft accept replaces contract and records hash");
    }

    /// Scenario: draft edit writes editable file without modifying contract.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_draft_edit_writes_editable_file_without_modifying_contract() {
        unimplemented!("awaits phase-8: draft edit writes editable file");
    }

    /// Scenario: draft accept --edited applies the editable file.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_edited_applies_editable_file() {
        unimplemented!("awaits phase-8: draft accept --edited applies editable file");
    }

    /// Scenario: Accepting a draft with invalid frontmatter exits 1 and restores contract.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_draft_accept_invalid_frontmatter_exits_one_restores_contract() {
        unimplemented!("awaits phase-8: draft accept with invalid frontmatter exits 1");
    }

    /// Scenario: draft discard marks the draft discarded.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_draft_discard_marks_discarded() {
        unimplemented!("awaits phase-8: draft discard marks draft discarded");
    }

    /// Scenario: Generation never modifies a contract until resolution runs.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_generation_never_modifies_contract() {
        unimplemented!("awaits phase-8: generation never modifies contract");
    }
}

mod mcp_exposure {
    use super::cflx_planned;

    /// Scenario: Read-only summariser tools listed in default mode.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_read_only_tools_listed_in_default_mode() {
        unimplemented!("awaits phase-8: read-only tools listed in default mode");
    }

    /// Scenario: Mutating summariser tools absent in default mode.
    #[cflx_planned(phase = 800)]
    #[test]
    fn test_mutating_tools_absent_in_default_mode() {
        unimplemented!("awaits phase-8: mutating tools absent in default mode");
    }
}
