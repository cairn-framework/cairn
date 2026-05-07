# Proposal: Phase 7.6.0 Tests (AI Provenance Foundation Pre-Phase)

**Change Type**: hybrid

## Dependencies

- `phase-7.5c-verification-states` (required dependency, archived).

Execution: MUST run BEFORE `phase-7.6-ai-provenance-foundation`. Archives when `cargo test` passes with all new tests ignored.

## Problem/Context

Phase 7.6 AI Provenance Foundation introduces 27 acceptance-criterion scenarios across four specs (provenance-foundation, changes, cli, and query), mapping to 24 unique test functions (three trace scenarios are asserted by both the provenance-foundation and CLI requirements). Without a committed test contract, the apply agent implementing phase-7.6 has no machine-checkable grading signal per scenario. This pre-phase writes that contract so phase-7.6 can remove `#[cflx_planned]` attributes one group at a time and use `cargo test` as its correctness signal.

This follows the test-first pre-phase convention defined in `openspec/specs/testing-baseline/spec.md` (Requirement: Test-first pre-phase convention) and `openspec/conventions.md`.

## Proposed Solution

Add `tests/phase_7_6_ai_provenance.rs` containing one `#[test]` per unique phase-7.6 acceptance-criterion scenario, each annotated `#[cflx_planned(phase = 76)]`. The tests use `unimplemented!()` bodies so they compile but fail when run. The file compiles cleanly under all strict Rust gates. This pre-phase archives on a green `cargo test` because all new tests are skipped.

## Acceptance Criteria

- `tests/phase_7_6_ai_provenance.rs` exists and compiles.
- The file contains exactly one `#[test]` per unique phase-7.6 scenario, each marked `#[cflx_planned(phase = 76)]`.
- `cargo test` passes (planned tests are skipped).
- `cargo test -- --ignored` reports all 24 new tests as failed (they are not yet implemented).
- All strict Rust gates pass.

## Out of Scope

- Any AI provenance foundation implementation.
- Changes to `src/`.
- Snapshot file creation.
