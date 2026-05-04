# Tasks: Phase 7.6.0 Tests (AI Provenance Foundation Pre-Phase)

## 1. Trace Sidecar Tests (provenance-foundation spec)

- [ ] 1.1 Write `#[cflx_planned(phase = 706)]` test: `test_sidecar_is_state_versioned`.
- [ ] 1.2 Write `#[cflx_planned(phase = 706)]` test: `test_sidecar_covers_four_native_stages`.
- [ ] 1.3 Write `#[cflx_planned(phase = 706)]` test: `test_prompt_content_reserved_but_empty`.
- [ ] 1.4 Write `#[cflx_planned(phase = 706)]` test: `test_higher_version_fails_with_clear_error`.
- [ ] 1.5 Write `#[cflx_planned(phase = 706)]` test: `test_trace_human_output_labels_each_stage`.
- [ ] 1.6 Write `#[cflx_planned(phase = 706)]` test: `test_trace_json_output_is_schema_with_version`.
- [ ] 1.7 Write `#[cflx_planned(phase = 706)]` test: `test_trace_missing_sidecar_exits_cleanly`.
- [ ] 1.8 Write `#[cflx_planned(phase = 706)]` test: `test_trace_command_delegates_to_library_reader`.

## 2. Suggested-Edges Queue Tests (changes spec)

- [ ] 2.1 Write `#[cflx_planned(phase = 706)]` test: `test_queue_file_is_state_versioned`.
- [ ] 2.2 Write `#[cflx_planned(phase = 706)]` test: `test_entry_carries_source_target_relation_and_triage_state`.
- [ ] 2.3 Write `#[cflx_planned(phase = 706)]` test: `test_triage_state_defaults_to_pending`.
- [ ] 2.4 Write `#[cflx_planned(phase = 706)]` test: `test_queue_is_sibling_not_delta_operation`.
- [ ] 2.5 Write `#[cflx_planned(phase = 706)]` test: `test_validate_without_strict_surfaces_warning`.
- [ ] 2.6 Write `#[cflx_planned(phase = 706)]` test: `test_validate_strict_fails_cc002_on_pending`.
- [ ] 2.7 Write `#[cflx_planned(phase = 706)]` test: `test_validate_strict_passes_when_all_non_pending`.
- [ ] 2.8 Write `#[cflx_planned(phase = 706)]` test: `test_absent_queue_file_is_not_error`.

## 3. CLI Islands and Neighbourhood Tests (cli spec)

- [ ] 3.1 Write `#[cflx_planned(phase = 706)]` test: `test_islands_returns_component_breakdown`.
- [ ] 3.2 Write `#[cflx_planned(phase = 706)]` test: `test_islands_json_output_is_versioned`.
- [ ] 3.3 Write `#[cflx_planned(phase = 706)]` test: `test_neighbourhood_include_orphans_surfaces_reverse_only`.
- [ ] 3.4 Write `#[cflx_planned(phase = 706)]` test: `test_both_forms_delegate_to_library_query`.
- [ ] 3.5 Confirm CLI trace scenarios (human output, JSON output, missing sidecar) are covered by provenance-foundation tests 1.5–1.7. No additional test stubs required.

## 4. Query Islands and Neighbourhood Tests (query spec)

- [ ] 4.1 Write `#[cflx_planned(phase = 706)]` test: `test_query_islands_returns_one_entry_per_component`.
- [ ] 4.2 Write `#[cflx_planned(phase = 706)]` test: `test_query_islands_handles_single_component`.
- [ ] 4.3 Write `#[cflx_planned(phase = 706)]` test: `test_query_neighbourhood_include_orphans_surfaces_inbound_only`.
- [ ] 4.4 Write `#[cflx_planned(phase = 706)]` test: `test_query_islands_response_is_versioned`.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes (all 24 new tests are skipped as ignored).
- [ ] 5.5 `cargo test -- --ignored` shows all 24 new tests as FAILED (confirming bodies are `unimplemented!()`).
- [ ] 5.6 `bash scripts/pre-archive-rust-gates.sh` passes.
