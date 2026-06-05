# Proposal: Phase 7.8.0 Tests (Cairn Export Pre-Phase)

**Change Type**: hybrid

## Dependencies

No hard dependencies; this pre-phase is lifecycle-orthogonal.

Execution: MUST run BEFORE `phase-7.8-cairn-export`. Archives when `cargo test` passes with all new tests ignored.

## Problem/Context

Phase 7.8 Cairn Export introduces 8 acceptance-criterion scenarios for the export command. Without a committed test contract, the apply agent implementing phase-7.8 has no machine-checkable grading signal per scenario. This pre-phase writes that contract so phase-7.8 can remove `#[cflx_planned]` attributes one at a time and use `cargo test` as its correctness signal.

This follows the test-first pre-phase convention defined in `openspec/specs/testing-baseline/spec.md` (Requirement: Test-first pre-phase convention) and `openspec/conventions.md`.

## Proposed Solution

Add `tests/phase_7_8_cairn_export.rs` containing one `#[test]` per phase-7.8 acceptance-criterion scenario, each annotated `#[cflx_planned(phase = 78)]`. The tests use `unimplemented!()` bodies so they compile but fail when run. The file compiles cleanly under all strict Rust gates. This pre-phase archives on a green `cargo test` because all new tests are skipped.

## Acceptance Criteria

- `tests/phase_7_8_cairn_export.rs` exists and compiles.
- The file contains exactly one `#[test]` per phase-7.8 scenario, each marked `#[cflx_planned(phase = 78)]`.
- `cargo test` passes (planned tests are skipped).
- `cargo test -- --ignored` reports all 8 new tests as failed (they are not yet implemented).
- All strict Rust gates pass.

## Out of Scope

- Any export command implementation.
- Changes to `src/`.
- Snapshot file creation.
