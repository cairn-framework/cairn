# Proposal: Phase 7.7.0 Tests (UX Foundation Pre-Phase)

**Change Type**: hybrid

## Dependencies

- `phase-7.5c-verification-states` (recommended ordering only; not a code dependency).

Execution: MUST run BEFORE `phase-7.7-ux-foundation`. Archives when `cargo test` passes with all new tests ignored.

## Problem/Context

Phase 7.7 UX Foundation introduces 29 acceptance-criterion scenarios across three specs (CLI, Graph Explorer, Reconciliation). Without a committed test contract, the apply agent implementing phase-7.7 has no machine-checkable grading signal per scenario. This pre-phase writes that contract so phase-7.7 can remove `#[cflx_planned]` attributes one group at a time and use `cargo test` as its correctness signal.

This follows the test-first pre-phase convention defined in `openspec/conventions.md`.

## Proposed Solution

Add `tests/phase_7_7_ux_foundation.rs` containing one `#[test]` per phase-7.7 acceptance-criterion scenario, each annotated `#[cflx_planned(phase = 77)]`. The tests use `unimplemented!()` bodies so they compile but fail when run. The file compiles cleanly under all strict Rust gates. This pre-phase archives on a green `cargo test` because all new tests are skipped.

Scenarios 7 (CLI empty-state copy free of em-dashes) and 11 (webui empty-state copy free of em-dashes) assert the same property on the same centralised copy file; they map to a single shared test `empty_state__copy_has_no_em_dashes`.

## Acceptance Criteria

- `tests/phase_7_7_ux_foundation.rs` exists and compiles.
- The file contains exactly one `#[test]` per phase-7.7 scenario, each marked `#[cflx_planned(phase = 77)]` (28 unique test functions covering 29 scenarios).
- `cargo test` passes (planned tests are skipped).
- `cargo test -- --ignored` reports all 28 new tests as failed (they are not yet implemented).
- All strict Rust gates pass.

## Out of Scope

- Any UX Foundation feature implementation.
- Changes to `src/`.
- Snapshot file creation.
