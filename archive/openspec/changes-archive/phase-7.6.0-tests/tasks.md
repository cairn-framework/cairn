# Tasks: Phase 7.6.0 Tests (AI Provenance Foundation Pre-Phase)

## 1. Trace Sidecar Tests (provenance-foundation spec)

- [x] 1.1 Write `#[cflx_planned(phase = 706)]` test: `test_sidecar_is_state_versioned`.
- [x] 1.2 Write `#[cflx_planned(phase = 706)]` test: `test_sidecar_covers_four_native_stages`.
- [x] 1.3 Write `#[cflx_planned(phase = 706)]` test: `test_prompt_content_reserved_but_empty`.
- [x] 1.4 Write `#[cflx_planned(phase = 706)]` test: `test_higher_version_fails_with_clear_error`.
- [x] 1.5 Write `#[cflx_planned(phase = 706)]` test: `test_trace_human_output_labels_each_stage`.
- [x] 1.6 Write `#[cflx_planned(phase = 706)]` test: `test_trace_json_output_is_schema_with_version`.
- [x] 1.7 Write `#[cflx_planned(phase = 706)]` test: `test_trace_missing_sidecar_exits_cleanly`.
- [x] 1.8 Write `#[cflx_planned(phase = 706)]` test: `test_trace_command_delegates_to_library_reader`.

## 2. Suggested-Edges Queue Tests (changes spec)

- [x] 2.1 Write `#[cflx_planned(phase = 706)]` test: `test_queue_file_is_state_versioned`.
- [x] 2.2 Write `#[cflx_planned(phase = 706)]` test: `test_entry_carries_source_target_relation_and_triage_state`.
- [x] 2.3 Write `#[cflx_planned(phase = 706)]` test: `test_triage_state_defaults_to_pending`.
- [x] 2.4 Write `#[cflx_planned(phase = 706)]` test: `test_queue_is_sibling_not_delta_operation`.
- [x] 2.5 Write `#[cflx_planned(phase = 706)]` test: `test_validate_without_strict_surfaces_warning`.
- [x] 2.6 Write `#[cflx_planned(phase = 706)]` test: `test_validate_strict_fails_cc002_on_pending`.
- [x] 2.7 Write `#[cflx_planned(phase = 706)]` test: `test_validate_strict_passes_when_all_non_pending`.
- [x] 2.8 Write `#[cflx_planned(phase = 706)]` test: `test_absent_queue_file_is_not_error`.

## 3. CLI Islands and Neighbourhood Tests (cli spec)

- [x] 3.1 Write `#[cflx_planned(phase = 706)]` test: `test_islands_returns_component_breakdown`.
- [x] 3.2 Write `#[cflx_planned(phase = 706)]` test: `test_islands_json_output_is_versioned`.
- [x] 3.3 Write `#[cflx_planned(phase = 706)]` test: `test_neighbourhood_include_orphans_surfaces_reverse_only`.
- [x] 3.4 Write `#[cflx_planned(phase = 706)]` test: `test_both_forms_delegate_to_library_query`.
- [x] 3.5 Confirm CLI trace scenarios (human output, JSON output, missing sidecar) are covered by provenance-foundation tests 1.5–1.7. No additional test stubs required.

## 4. Query Islands and Neighbourhood Tests (query spec)

- [x] 4.1 Write `#[cflx_planned(phase = 706)]` test: `test_query_islands_returns_one_entry_per_component`.
- [x] 4.2 Write `#[cflx_planned(phase = 706)]` test: `test_query_islands_handles_single_component`.
- [x] 4.3 Write `#[cflx_planned(phase = 706)]` test: `test_query_neighbourhood_include_orphans_surfaces_inbound_only`.
- [x] 4.4 Write `#[cflx_planned(phase = 706)]` test: `test_query_islands_response_is_versioned`.

## 5. Required Verification

- [x] 5.1 `cargo build` passes with zero warnings.
- [x] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 5.3 `cargo fmt --check` passes.
- [x] 5.4 `cargo test` passes (all 24 new tests are skipped as ignored).
- [x] 5.5 `cargo test -- --ignored` shows all 24 new tests as FAILED (confirming bodies are `unimplemented!()`).
- [x] 5.6 `bash scripts/pre-archive-rust-gates.sh` passes.
