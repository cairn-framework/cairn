# Proposal: Phase 9.0 Brownfield Tests

## Dependencies

- Requires: `phase-8-summariser` (not yet applied).
- Execution: MUST run after Phase 8 and MUST run before Phase 9 Brownfield Extraction.

## Problem/Context

Phase 9 Brownfield Extraction introduces ten acceptance-criterion scenarios across four requirements. No failing test assertions exist today to confirm those criteria are met when Phase 9 lands. Without a pre-phase test wall, the apply agent implementing Phase 9 has no automated grading signal beyond build success.

## Proposed Solution

Write one failing `#[test]` assertion per acceptance criterion and per concrete design invariant in `tests/phase_9_brownfield.rs`. Every test is marked `#[ignore = "awaits phase-9"]` so this pre-phase archives on a green `cargo test`. Phase 9 removes each `#[ignore]` attribute as the corresponding feature code lands.

The test file also covers the seven numeric invariants stated in the Phase 9 design document (coupling score bands, candidate file-count thresholds, depth limit, edge threshold, sample byte/file limits, and the disabled-summariser fallback) because those invariants are precise enough to assert without the full runtime.

## Acceptance Criteria

- `tests/phase_9_brownfield.rs` exists and contains one `#[ignore = "awaits phase-9"]` test per Phase 9 acceptance criterion scenario.
- `cargo test` passes (all new tests are skipped).
- `cargo test -- --ignored` reports all new tests as failing (not erroring at compile time).
- All strict Rust gates pass.

## Out of Scope

- Any brownfield feature implementation.
- Changes to `src/`.
- Changes to any existing test.
