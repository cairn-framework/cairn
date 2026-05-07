//! Phase 7.6 AI Provenance Foundation acceptance-criterion tests.
//!
//! Test contract for `phase-7.6-ai-provenance-foundation`. Each test corresponds
//! to one acceptance-criterion scenario across the four spec deltas
//! (`provenance-foundation`, `changes`, `cli`, `query`). Tests are marked
//! `#[cflx_planned(phase = 706)]` so `cargo test` skips them while
//! `cargo test -- --ignored` runs them and they fail with a clear
//! `unimplemented!` message naming the scenario. Phase 7.6 will remove
//! `#[cflx_planned]` group-by-group as code lands.

use cairn::cflx_planned;

mod provenance_foundation {
    use super::cflx_planned;

    /// Scenario: Sidecar is state-versioned.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_sidecar_is_state_versioned() {
        unimplemented!("awaits phase-7.6: sidecar is state-versioned");
    }

    /// Scenario: Sidecar covers the four cairn-native stages.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_sidecar_covers_four_native_stages() {
        unimplemented!("awaits phase-7.6: sidecar covers four cairn-native stages");
    }

    /// Scenario: Prompt content is reserved but empty in this phase.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_prompt_content_reserved_but_empty() {
        unimplemented!("awaits phase-7.6: prompt content reserved but empty");
    }

    /// Scenario: Higher version than understood fails with a clear error.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_higher_version_fails_with_clear_error() {
        unimplemented!("awaits phase-7.6: higher version fails with clear error");
    }

    /// Scenario: Default human output is labelled per stage.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_trace_human_output_labels_each_stage() {
        unimplemented!("awaits phase-7.6: trace human output labels each stage");
    }

    /// Scenario: JSON output is the schema with promoted version.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_trace_json_output_is_schema_with_version() {
        unimplemented!("awaits phase-7.6: trace JSON output is schema with version");
    }

    /// Scenario: Missing sidecar exits cleanly.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_trace_missing_sidecar_exits_cleanly() {
        unimplemented!("awaits phase-7.6: trace missing sidecar exits cleanly");
    }

    /// Scenario: Trace command does not own semantics; delegates to library reader.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_trace_command_delegates_to_library_reader() {
        unimplemented!("awaits phase-7.6: trace command delegates to library reader");
    }
}

mod changes {
    use super::cflx_planned;

    /// Scenario: Queue file is state-versioned.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_queue_file_is_state_versioned() {
        unimplemented!("awaits phase-7.6: queue file is state-versioned");
    }

    /// Scenario: Each entry carries source, target, relation, and triage state.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_entry_carries_source_target_relation_and_triage_state() {
        unimplemented!("awaits phase-7.6: entry carries source, target, relation, triage state");
    }

    /// Scenario: Triage state defaults to pending for newly-emitted entries.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_triage_state_defaults_to_pending() {
        unimplemented!("awaits phase-7.6: triage state defaults to pending");
    }

    /// Scenario: Queue is a sibling, not a delta operation.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_queue_is_sibling_not_delta_operation() {
        unimplemented!("awaits phase-7.6: queue is sibling, not delta operation");
    }

    /// Scenario: Validate without --strict surfaces count as warning.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_validate_without_strict_surfaces_warning() {
        unimplemented!("awaits phase-7.6: validate without strict surfaces warning");
    }

    /// Scenario: Validate --strict fails with CC002 on pending entries.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_validate_strict_fails_cc002_on_pending() {
        unimplemented!("awaits phase-7.6: validate strict fails CC002 on pending");
    }

    /// Scenario: Validate --strict passes when all entries are non-pending.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_validate_strict_passes_when_all_non_pending() {
        unimplemented!("awaits phase-7.6: validate strict passes when all non-pending");
    }

    /// Scenario: Absent queue file is not an error.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_absent_queue_file_is_not_error() {
        unimplemented!("awaits phase-7.6: absent queue file is not error");
    }
}

mod cli {
    use super::cflx_planned;

    /// Scenario: Islands command returns whole-graph component breakdown.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_islands_returns_component_breakdown() {
        unimplemented!("awaits phase-7.6: islands returns component breakdown");
    }

    /// Scenario: Islands JSON output is versioned.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_islands_json_output_is_versioned() {
        unimplemented!("awaits phase-7.6: islands JSON output is versioned");
    }

    /// Scenario: Neighbourhood with --include-orphans surfaces reverse-only nodes.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_neighbourhood_include_orphans_surfaces_reverse_only() {
        unimplemented!("awaits phase-7.6: neighbourhood --include-orphans surfaces reverse-only");
    }

    /// Scenario: Both forms (CLI and MCP) delegate to the library query.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_both_forms_delegate_to_library_query() {
        unimplemented!("awaits phase-7.6: both forms delegate to library query");
    }
}

mod query {
    use super::cflx_planned;

    /// Scenario: Islands returns one entry per connected component.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_query_islands_returns_one_entry_per_component() {
        unimplemented!("awaits phase-7.6: query islands returns one entry per component");
    }

    /// Scenario: Islands handles the trivial single-component case.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_query_islands_handles_single_component() {
        unimplemented!("awaits phase-7.6: query islands handles single component");
    }

    /// Scenario: Neighbourhood with `include_orphans` surfaces inbound-only neighbours.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_query_neighbourhood_include_orphans_surfaces_inbound_only() {
        unimplemented!(
            "awaits phase-7.6: query neighbourhood include_orphans surfaces inbound-only"
        );
    }

    /// Scenario: Islands query response is versioned.
    #[cflx_planned(phase = 706)]
    #[test]
    fn test_query_islands_response_is_versioned() {
        unimplemented!("awaits phase-7.6: query islands response is versioned");
    }
}
