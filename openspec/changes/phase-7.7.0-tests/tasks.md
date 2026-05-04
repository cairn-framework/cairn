# Tasks: Phase 7.7.0 Tests (UX Foundation Pre-Phase)

## 1. CLI Tests (spec CLI)

- [ ] 1.1 Write `#[cflx_planned(phase = 77)]` test: `check__whole_map_inspection_without_arguments`
- [ ] 1.2 Write `#[cflx_planned(phase = 77)]` test: `check__node_scoped_inspection_with_positional_argument`
- [ ] 1.3 Write `#[cflx_planned(phase = 77)]` test: `check__inspection_delegates_to_same_library_service_as_lint`
- [ ] 1.4 Write `#[cflx_planned(phase = 77)]` test: `check__inspection_has_no_json_mode`

## 2. Empty-State Tests (spec CLI and Graph Explorer)

- [ ] 2.1 Write `#[cflx_planned(phase = 77)]` test: `empty_state__no_blueprint_invocation_renders_cta`
- [ ] 2.2 Write `#[cflx_planned(phase = 77)]` test: `empty_state__clean_map_result_renders_cta`
- [ ] 2.3 Write `#[cflx_planned(phase = 77)]` test: `empty_state__copy_has_no_em_dashes` (covers both CLI scenario 7 and Graph Explorer scenario 11)

## 3. Graph Explorer Tests (spec Graph Explorer)

- [ ] 3.1 Write `#[cflx_planned(phase = 77)]` test: `explorer__empty_state_component_uses_token_only_styling`
- [ ] 3.2 Write `#[cflx_planned(phase = 77)]` test: `explorer__ten_inline_empty_state_strings_replaced`
- [ ] 3.3 Write `#[cflx_planned(phase = 77)]` test: `explorer__missing_copy_keys_surface_console_warning`
- [ ] 3.4 Write `#[cflx_planned(phase = 77)]` test: `explorer__three_severity_buckets_render_with_count_badges`
- [ ] 3.5 Write `#[cflx_planned(phase = 77)]` test: `explorer__scope_toggle_filters_to_selected_node`
- [ ] 3.6 Write `#[cflx_planned(phase = 77)]` test: `explorer__scope_toggle_disabled_when_no_node_selected`
- [ ] 3.7 Write `#[cflx_planned(phase = 77)]` test: `explorer__category_filter_chips_derive_from_finding_stream`
- [ ] 3.8 Write `#[cflx_planned(phase = 77)]` test: `explorer__panel_reads_only_from_query_consumer_api`
- [ ] 3.9 Write `#[cflx_planned(phase = 77)]` test: `explorer__banner_renders_highest_severity_finding_nudge`
- [ ] 3.10 Write `#[cflx_planned(phase = 77)]` test: `explorer__banner_tie_break_by_lowest_numbered_code`
- [ ] 3.11 Write `#[cflx_planned(phase = 77)]` test: `explorer__banner_cta_is_copy_pasteable_cli_snippet`
- [ ] 3.12 Write `#[cflx_planned(phase = 77)]` test: `explorer__banner_hidden_when_node_has_no_findings`
- [ ] 3.13 Write `#[cflx_planned(phase = 77)]` test: `explorer__structural_error_indicator`
- [ ] 3.14 Write `#[cflx_planned(phase = 77)]` test: `explorer__interface_contradiction_indicator`
- [ ] 3.15 Write `#[cflx_planned(phase = 77)]` test: `explorer__rationale_tension_indicator`
- [ ] 3.16 Write `#[cflx_planned(phase = 77)]` test: `explorer__info_severity_findings_appear_in_overlay`

## 4. Reconciliation Tests (spec Reconciliation)

- [ ] 4.1 Write `#[cflx_planned(phase = 77)]` test: `reconciliation__info_variant_defined_on_kernel_enum`
- [ ] 4.2 Write `#[cflx_planned(phase = 77)]` test: `reconciliation__orphaned_file_emits_info_finding`
- [ ] 4.3 Write `#[cflx_planned(phase = 77)]` test: `reconciliation__unverified_contract_emits_info_finding`
- [ ] 4.4 Write `#[cflx_planned(phase = 77)]` test: `reconciliation__info_findings_do_not_block_hooks_or_gates`
- [ ] 4.5 Write `#[cflx_planned(phase = 77)]` test: `reconciliation__info_findings_round_trip_through_serde_json`

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes (all 28 new tests are skipped as planned).
- [ ] 5.5 `cargo test -- --ignored` shows all 28 new tests as FAILED (confirming bodies are `unimplemented!()`).
- [ ] 5.6 `bash scripts/pre-archive-rust-gates.sh` passes.
