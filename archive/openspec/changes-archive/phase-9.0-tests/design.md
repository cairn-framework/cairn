# Design: Phase 9.0 Brownfield Tests

## References

- `openspec/changes/phase-9-brownfield/specs/brownfield/spec.md`: acceptance criteria being tested. After Wave 4 rescope, the spec carries 8 requirements with 24 scenarios.
- `openspec/changes/phase-9-brownfield/design.md`: numeric invariants being tested.
- `openspec/specs/testing-baseline/spec.md`: test-first pre-phase convention.
- `openspec/changes/archive/phase-7.5a-test-fortification/`: prior-art pre-phase structure.
- `docs/strongholds/getcairn-cross-check-integrated.md`: Wave 4 rescope rationale (Pattern 3, Integrator decision 3).
- `docs/strongholds/oq4-phase9-rescope-timing.md`: Option B refined verdict; this pre-phase authors after rescope merges and before its own apply.
- `openspec/conventions.md` Section 9: genesis transcript convention referenced by Requirement 6 scenarios.

## Test File Placement

All tests go in `tests/phase_9_brownfield.rs` as a top-level integration test file. This matches the existing `tests/` layout and keeps brownfield assertions isolated from the god-module unit tests added in Phase 7.5a.

Each test function name encodes the requirement and scenario it covers, following the pattern `<requirement_slug>__<scenario_slug>`.

## Ignored-Test Convention

Every test carries `#[cflx_planned(phase = 900)]`. Phase 9's first task in each group removes the attribute from the relevant tests and makes the implementation pass.

Running `cargo test -- --ignored` at any point between this pre-phase and Phase 9 landing will show the full list of failing red tests and serve as the design contract.

## Failing-State Stub Contract

Each stub MUST satisfy four properties:

1. Compile cleanly under `cargo build` with zero warnings (the stub respects the project's clippy-as-deny-warnings posture).
2. Sit behind `#[cflx_planned(phase = 900)]` so `cargo test` skips it and the gate stays green.
3. Either call `unimplemented!()` (when no fixture is yet available) or assert a property only a not-yet-written feature satisfies (when the assertion compiles against today's tree). Either form yields a runtime panic, not a compile error, when run with `--ignored`.
4. Carry a `///` doc-comment naming the property the implementation must satisfy. The doc-comment is intentional redundancy with the corresponding `tasks.md` task line (the canonical apply-stage authority): it lets a reader of `tests/phase_9_brownfield.rs` see the asserted property without context-switching to `tasks.md`. The doc-comment is not a second source of truth; if the two drift, `tasks.md` wins.

Stubs for the conditional Requirement 8 (decision-attached obligations) carry a guard comment naming the current schema state. The two field-present stubs panic via `unimplemented!()` until the schema grows the field; the field-absent stub asserts the no-op rider directly and is the always-callable branch.

## Acceptance Criterion Coverage

| Test function | Criterion source |
|---|---|
| `init__discovery_does_not_require_existing_blueprint` | spec Req 1, scenario 1 |
| `init__candidate_heuristics_are_deterministic` | spec Req 1, scenario 2 |
| `init__creates_brownfield_change_directory` | spec Req 1, scenario 3 |
| `init__existing_change_protected_without_force` | spec Req 1, scenario 4 |
| `init__force_replaces_existing_change` | spec Req 1, scenario 5 |
| `refine__proposes_additions_for_new_directories` | spec Req 2, scenario 1 |
| `refine__does_not_replace_current_truth` | spec Req 2, scenario 2 |
| `review__false_positive_deletion_respected` | spec Req 3, scenario 1 |
| `mcp__brownfield_tools_absent_in_default_mode` | spec Req 4, scenario 1 |
| `mcp__brownfield_tools_present_in_mutating_mode` | spec Req 4, scenario 2 |
| `suggest__engine_writes_to_queue_file` | spec Req 5, scenario 1 |
| `suggest__entry_triage_state_is_pending` | spec Req 5, scenario 1 (assertion split) |
| `suggest__entry_provenance_carries_trace_phase` | spec Req 5, scenario 1 (assertion split) |
| `suggest__pending_entries_block_archive_with_cc002` | spec Req 5, scenario 3 |
| `suggest__no_auto_accept_on_high_confidence` | spec Req 5, scenario 2 |
| `suggest__refine_emits_to_queue_file_with_propose_stage` | spec Req 5, scenario 4 |
| `suggest__force_init_aborts_on_pending_entries` | spec Req 5, scenario 5 |
| `interview__session_persists_across_invocations` | spec Req 6, scenario 1 |
| `interview__final_transcript_lands_at_genesis_path` | spec Req 6, scenario 2 |
| `interview__session_state_never_leaks_outside_change_dir` | spec Req 6, scenario 3 |
| `templates__matching_template_guides_stub_authoring` | spec Req 7, scenario 1 |
| `templates__non_matching_candidates_fall_back_to_builtin` | spec Req 7, scenario 2 |
| `templates__ill_formed_template_does_not_block_authoring` | spec Req 7, scenario 3 |
| `obligations__populated_when_field_exists` | spec Req 8, scenario 1 |
| `obligations__reviewable_before_archive` | spec Req 8, scenario 2 |
| `obligations__no_op_when_field_absent` | spec Req 8, scenario 3 |
| `heuristics__coupling_score_high_confidence` | design doc coupling score >= 2.0 |
| `heuristics__coupling_score_medium_confidence` | design doc coupling score >= 1.0 |
| `heuristics__coupling_score_low_confidence` | design doc coupling score < 1.0 |
| `heuristics__directory_candidate_min_three_files` | design doc min file count = 3 |
| `heuristics__directory_depth_limit_four` | design doc max depth = 4 |
| `heuristics__edge_threshold_two_import_observations` | design doc edge threshold >= 2 |
| `heuristics__summariser_disabled_uses_path_derived_names` | design doc disabled-mode fallback |

Total: 26 acceptance-criterion stubs plus 7 heuristic-invariant stubs. The acceptance count exceeds the spec's 24-scenario count because Req 5 scenario 1 carries three load-bearing assertions (queue write, pending state, trace phase) and is split across three stubs for clear failure isolation; the other 23 scenarios map one-to-one. Total stub count is 33 (26 acceptance + 7 heuristic). Regenerating the count is straightforward: `grep -c "#\[ignore" tests/phase_9_brownfield.rs`.

**Splitting principle.** Multi-THEN scenarios split into separate stubs when each clause exercises a distinct subsystem (for example schema serialisation versus gate registration versus trace context resolver); they stay unified when all clauses hinge on one syscall (for example one filesystem write producing four sibling artefacts). Req 5 scenario 1 splits because its three THENs touch the queue file writer, the triage-state policy, and the trace-phase resolver. Req 1 scenario 3 stays unified because its five THENs all hinge on one init invocation's filesystem fan-out.

## Flagged Vague Criteria

Two scenarios are difficult to isolate in a unit or integration test because they require the full archive pipeline, which does not exist yet:

- **"False positive can be removed" (Req 3, scenario 1):** The assertion depends on archive applying only remaining operations after a human edit. The pre-phase test asserts the delta structure is mutable before archive, but cannot exercise the archive pipeline itself. Phase 9 should sharpen this criterion with a concrete archive-mock or fixture.
- **"Refine does not replace current truth" (Req 2, scenario 2):** The assertion depends on change-aware query semantics that Phase 9 implements. The pre-phase test asserts that `cairn refine` does not overwrite `cairn.blueprint` on disk, which is a necessary but not sufficient condition. Phase 9 should add a query-layer assertion once the change-aware query API exists.

Wave 4 stubs that depend on cross-component fixtures (suggest engine queue file, interview session state, project-config templates, decision schema) carry `unimplemented!()` markers until Phase 9 supplies the fixtures. The marker is intentional: it preserves the failing-state contract without requiring this pre-phase to author the fixture surface, which belongs to Phase 9.

## Sample Test Sketch

```rust
/// Asserts brownfield init creates `openspec/changes/brownfield-init/` with proposal,
/// blueprint.delta, and stub contracts, and does not touch the main `cairn.blueprint`.
#[test]
#[cflx_planned(phase = 900)]
fn init__creates_brownfield_change_directory() {
    let repo = fixture_repo_without_blueprint();
    cairn::init_from_code(&repo, false).expect("init should succeed");
    assert!(repo.path().join("openspec/changes/brownfield-init").exists());
    assert!(repo.path().join("openspec/changes/brownfield-init/proposal.md").exists());
    assert!(repo.path().join("openspec/changes/brownfield-init/blueprint.delta").exists());
    assert!(!repo.path().join("cairn.blueprint").exists());
}

/// Asserts every entry written by the suggest engine carries `triage_state == "pending"`
/// regardless of computed confidence (no auto-accept policy promotes it).
#[test]
#[cflx_planned(phase = 900)]
fn suggest__entry_triage_state_is_pending() {
    // Wave 4 stub: fixture not yet available; runtime panic is the failing-state signal.
    unimplemented!("phase-9 supplies the suggest engine fixture");
}
```

Fixtures use `tempdir`-backed helper functions defined at the top of `tests/phase_9_brownfield.rs`. No new test dependencies are required; `tempfile` is already a dev-dependency. Wave 4 fixtures are added as helper-function shells that Phase 9 fleshes out.
