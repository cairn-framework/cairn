# Proposal: Phase 10.0 Distribution Tests

**Change Type**: hybrid

## Dependencies

- `phase-9-brownfield` (required dependency, not yet applied).

Execution: MUST run before `phase-10-distribution`.

## Problem/Context

Phase 10 introduces an LSP server, plugin packaging, reconciler extensions, and release checks. Without a prior test-first pass, the implementation agent has no failing assertions to target and no gate that turns green on correct delivery.

This pre-phase authors every failing test assertion against the acceptance criteria defined in `openspec/changes/phase-10-distribution/specs/distribution/spec.md`. All tests are marked `#[ignore = "awaits phase-10"]` so this pre-phase archives on a green `cargo test`. Phase 10 removes the `#[ignore]` attributes as each criterion is satisfied.

## Proposed Solution

Add `tests/phase_10_distribution.rs` containing one `#[test]` per acceptance-criterion scenario. Each test carries `#[ignore = "awaits phase-10"]` and encodes a concrete assertion that passes only once the corresponding phase-10 code lands.

No production code is added or modified. No `#[ignore]` attributes are removed. The archive gate passes because ignored tests are skipped.

## Acceptance Criteria

- `tests/phase_10_distribution.rs` exists and compiles.
- Every test in the file carries `#[ignore = "awaits phase-10"]`.
- `cargo test` passes (ignored tests are skipped).
- `cargo test -- --ignored` runs all tests and each fails for the correct reason (missing implementation, not a compile error or assertion logic mistake in the test itself).
- All strict Rust gates pass.

## Out of Scope

- Any phase-10 implementation code.
- Removing `#[ignore]` attributes. That is phase-10's first task per group.
- Tests for acceptance criteria that are under-specified in the phase-10 spec (see flagged items in `design.md`).
