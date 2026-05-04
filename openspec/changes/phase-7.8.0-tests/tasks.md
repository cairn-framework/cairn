# Tasks: Phase 7.8.0 Tests (Cairn Export Pre-Phase)

## 1. Export Command Tests

- [ ] 1.1 Write `#[cflx_planned(phase = 708)]` test: `test_default_format_is_json`.
- [ ] 1.2 Write `#[cflx_planned(phase = 708)]` test: `test_markdown_format_selected_via_flag`.
- [ ] 1.3 Write `#[cflx_planned(phase = 708)]` test: `test_json_envelope_carries_schema_version`.
- [ ] 1.4 Write `#[cflx_planned(phase = 708)]` test: `test_markdown_payload_contains_no_em_dashes`.
- [ ] 1.5 Write `#[cflx_planned(phase = 708)]` test: `test_output_flag_is_required`.
- [ ] 1.6 Write `#[cflx_planned(phase = 708)]` test: `test_invalid_format_value_is_rejected`.
- [ ] 1.7 Write `#[cflx_planned(phase = 708)]` test: `test_export_is_lifecycle_orthogonal`.
- [ ] 1.8 Write `#[cflx_planned(phase = 708)]` test: `test_render_delegates_to_shared_library_service`.

## 2. Required Verification

- [ ] 2.1 `cargo build` passes with zero warnings.
- [ ] 2.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 2.3 `cargo fmt --check` passes.
- [ ] 2.4 `cargo test` passes (all 8 new tests are skipped as planned).
- [ ] 2.5 `cargo test -- --ignored` shows all 8 new tests as FAILED (confirming bodies are `unimplemented!()`).
- [ ] 2.6 `bash scripts/pre-archive-rust-gates.sh` passes.
