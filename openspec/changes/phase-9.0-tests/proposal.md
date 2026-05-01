# Proposal: Phase 9.0 Brownfield Tests

**Change Type**: hybrid

## Dependencies

- `phase-8-summariser` (required dependency, not yet applied).
- `phase-9-brownfield` (Wave 4 rescope of `phase-9-brownfield/specs/brownfield/spec.md`; the test stub structure references the rescoped requirements). The rescoped scope is fixed; this pre-phase mirrors that scope as failing-state stubs.

Execution: MUST run after Phase 8 and MUST run before Phase 9 Brownfield Extraction.

## Problem/Context

Phase 9 Brownfield Extraction, after the Wave 4 rescope (Pattern 3 in `docs/strongholds/getcairn-cross-check-integrated.md`; trigger event Option B refined per `docs/strongholds/oq4-phase9-rescope-timing.md`), introduces 24 acceptance-criterion scenarios across 8 requirements:

1. Generate initial Cairn state from code (5 scenarios; existing).
2. Refine existing Cairn state from code changes (2 scenarios; existing).
3. Keep human review authoritative (1 scenario; existing).
4. Expose brownfield commands through MCP (2 scenarios; existing).
5. Suggest cross-cutting edges through the phase 7.6 queue (5 scenarios; new in Wave 4).
6. Run multi-round elicitation for brownfield onboarding (3 scenarios; new in Wave 4).
7. Resolve project-declared templates for stub authoring (3 scenarios; new in Wave 4).
8. Populate decision-attached obligations when the schema supports them (3 scenarios; conditional, new in Wave 4).

No failing test assertions exist today to confirm those criteria are met when Phase 9 lands. Without a pre-phase test wall, the apply agent implementing Phase 9 has no automated grading signal beyond build success. Sibling proposal `phase-9.0-tests` previously claimed "ten acceptance-criterion scenarios across four requirements"; this update syncs the count to the rescoped 24-across-8 shape.

## Proposed Solution

Write one failing `#[test]` assertion per acceptance criterion and per concrete design invariant in `tests/phase_9_brownfield.rs`. Every test is marked `#[ignore = "awaits phase-9"]` so this pre-phase archives on a green `cargo test`. Phase 9 removes each `#[ignore]` attribute as the corresponding feature code lands.

The test file also covers the seven numeric invariants stated in the Phase 9 design document (coupling score bands, candidate file-count thresholds, depth limit, edge threshold, sample byte/file limits, and the disabled-summariser fallback) because those invariants are precise enough to assert without the full runtime.

Tests for the conditional Requirement 8 (decision-attached obligations) compile against either the present-field branch or the absent-field branch. When the decision schema does not expose an `obligations` field, the matching tests assert the no-op rider (no obligations-related output). When the field exists, the same tests assert population. The `#[ignore]` attribute lifts in either case once Phase 9 ships the corresponding code path.

Test stubs follow the failing-state contract: each stub compiles cleanly under `cargo build`, sits behind `#[ignore = "awaits phase-9"]`, and either calls `unimplemented!()` (when no fixture is yet available) or asserts a property that only a not-yet-written feature satisfies. Running `cargo test -- --ignored` enumerates all stubs as failing red tests; running `cargo test` skips them all and the gate stays green.

## Acceptance Criteria

- `tests/phase_9_brownfield.rs` exists and contains one `#[ignore = "awaits phase-9"]` test per Phase 9 acceptance-criterion scenario across all 8 rescoped requirements (24 scenarios total).
- The same file contains one `#[ignore = "awaits phase-9"]` test per heuristic invariant in `phase-9-brownfield/design.md` (7 invariants).
- Total ignored test count is 33 (26 acceptance + 7 heuristic). The acceptance count exceeds the 24 scenarios because Req 5 scenario 1 splits into three stubs for failure isolation per the splitting principle in `design.md`.
- `cargo test` passes (all new tests are skipped).
- `cargo test -- --ignored` reports all new tests as failing (not erroring at compile time). Tests that cannot yet be exercised end-to-end without Phase 9 fixtures call `unimplemented!()` so the failure is a runtime panic, not a compile error.
- For Requirement 8 (conditional obligations), tests carry guard comments naming whether the decision schema currently exposes an `obligations` field; they compile and run identically in either branch.
- All strict Rust gates pass.

## Out of Scope

- Any brownfield feature implementation.
- Changes to `src/`.
- Changes to any existing test.
- Authoring the suggested-edges queue file format, the interview runner state machine, or the contract template resolver. Those land in Phase 9 itself; this pre-phase only stubs the failing assertions that Phase 9 must turn green.
