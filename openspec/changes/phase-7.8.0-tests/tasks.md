# Tasks: Phase 7.8.0 Tests (Cairn Export Pre-Phase)

## 1. Export Command Tests

- [ ] 1.1 Write `#[ignore = "awaits phase-7.8"]` test: `default_format_is_json`.
- [ ] 1.2 Write `#[ignore = "awaits phase-7.8"]` test: `markdown_format_selected_via_flag`.
- [ ] 1.3 Write `#[ignore = "awaits phase-7.8"]` test: `json_envelope_carries_schema_version`.
- [ ] 1.4 Write `#[ignore = "awaits phase-7.8"]` test: `markdown_payload_contains_no_em_dashes`.
- [ ] 1.5 Write `#[ignore = "awaits phase-7.8"]` test: `output_flag_is_required`.
- [ ] 1.6 Write `#[ignore = "awaits phase-7.8"]` test: `invalid_format_value_is_rejected`.
- [ ] 1.7 Write `#[ignore = "awaits phase-7.8"]` test: `export_is_lifecycle_orthogonal`.
- [ ] 1.8 Write `#[ignore = "awaits phase-7.8"]` test: `render_delegates_to_shared_library_service`.

## 2. Required Verification

- [ ] 2.1 `cargo build` passes with zero warnings.
- [ ] 2.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 2.3 `cargo fmt --check` passes.
- [ ] 2.4 `cargo test` passes (all 8 new tests are skipped as ignored).
- [ ] 2.5 `cargo test -- --ignored` shows all 8 new tests as FAILED (confirming bodies are `todo!()`).
- [ ] 2.6 `bash scripts/pre-archive-rust-gates.sh` passes.
