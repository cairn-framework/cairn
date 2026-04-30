# Proposal: Phase 8.0 Tests (Summariser Pre-Phase)

**Change Type**: hybrid

## Dependencies

- `phase-7.5b-cleansing-splits` (required dependency, archived).

Execution: MUST run BEFORE `phase-8-summariser`. Archives when `cargo test` passes with all new tests ignored.

## Problem/Context

Phase 8 Summariser introduces 12 acceptance-criterion scenarios across three requirements. Without a committed test contract, the apply agent implementing phase-8 has no machine-checkable grading signal per scenario. This pre-phase writes that contract so phase-8 can remove `#[ignore]` attributes one group at a time and use `cargo test` as its correctness signal.

This follows the test-first pre-phase convention defined in `openspec/specs/testing-baseline/spec.md` (Requirement: Test-first pre-phase convention) and `openspec/conventions.md`.

## Proposed Solution

Add `tests/phase_8_summariser.rs` containing one `#[test]` per phase-8 acceptance-criterion scenario, each annotated `#[ignore = "awaits phase-8"]`. The tests use `todo!()` bodies so they compile but fail when run. The file compiles cleanly under all strict Rust gates. This pre-phase archives on a green `cargo test` because all new tests are skipped.

## Acceptance Criteria

- `tests/phase_8_summariser.rs` exists and compiles.
- The file contains exactly one `#[test]` per phase-8 scenario, each marked `#[ignore = "awaits phase-8"]`.
- `cargo test` passes (ignored tests are skipped).
- `cargo test -- --ignored` reports all 12 new tests as failed (they are not yet implemented).
- All strict Rust gates pass.

## Out of Scope

- Any summariser implementation.
- Changes to `src/`.
- Snapshot file creation.
- MCP server changes.
