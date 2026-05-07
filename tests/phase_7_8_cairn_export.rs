//! Phase 7.8 Cairn Export acceptance-criterion tests.
//!
//! Test contract for `phase-7.8-cairn-export`. Each test corresponds to one
//! acceptance-criterion scenario for the `cairn export` command. Tests are
//! marked `#[cflx_planned(phase = 708)]` so `cargo test` skips them while
//! `cargo test -- --ignored` runs them and they fail with a clear
//! `unimplemented!` message naming the scenario. Phase 7.8 will remove
//! `#[cflx_planned]` group-by-group as code lands.

use cairn::cflx_planned;

/// Scenario: Default format is JSON.
#[cflx_planned(phase = 708)]
#[test]
fn test_default_format_is_json() {
    unimplemented!("awaits phase-7.8: default format is JSON");
}

/// Scenario: Markdown format selected via flag.
#[cflx_planned(phase = 708)]
#[test]
fn test_markdown_format_selected_via_flag() {
    unimplemented!("awaits phase-7.8: markdown format selected via flag");
}

/// Scenario: JSON envelope carries `schema_version`.
#[cflx_planned(phase = 708)]
#[test]
fn test_json_envelope_carries_schema_version() {
    unimplemented!("awaits phase-7.8: JSON envelope carries schema_version");
}

/// Scenario: Markdown payload contains no em-dashes.
#[cflx_planned(phase = 708)]
#[test]
fn test_markdown_payload_contains_no_em_dashes() {
    unimplemented!("awaits phase-7.8: markdown payload contains no em-dashes");
}

/// Scenario: --output flag is required.
#[cflx_planned(phase = 708)]
#[test]
fn test_output_flag_is_required() {
    unimplemented!("awaits phase-7.8: --output flag is required");
}

/// Scenario: Invalid --format value is rejected.
#[cflx_planned(phase = 708)]
#[test]
fn test_invalid_format_value_is_rejected() {
    unimplemented!("awaits phase-7.8: invalid --format value is rejected");
}

/// Scenario: Export is lifecycle-orthogonal.
#[cflx_planned(phase = 708)]
#[test]
fn test_export_is_lifecycle_orthogonal() {
    unimplemented!("awaits phase-7.8: export is lifecycle-orthogonal");
}

/// Scenario: Render delegates to shared library service.
#[cflx_planned(phase = 708)]
#[test]
fn test_render_delegates_to_shared_library_service() {
    unimplemented!("awaits phase-7.8: render delegates to shared library service");
}
