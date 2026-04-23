# Design: Phase 9.0 Brownfield Tests

## References

- `openspec/changes/phase-9-brownfield/specs/brownfield/spec.md`: acceptance criteria being tested.
- `openspec/changes/phase-9-brownfield/design.md`: numeric invariants being tested.
- `openspec/specs/testing-baseline/spec.md`: test-first pre-phase convention.
- `openspec/changes/archive/phase-7.5a-test-fortification/`: prior-art pre-phase structure.

## Test File Placement

All tests go in `tests/phase_9_brownfield.rs` as a top-level integration test file. This matches the existing `tests/` layout and keeps brownfield assertions isolated from the god-module unit tests added in Phase 7.5a.

Each test function name encodes the requirement and scenario it covers, following the pattern `<requirement_slug>__<scenario_slug>`.

## Ignored-Test Convention

Every test carries `#[ignore = "awaits phase-9"]`. Phase 9's first task in each group removes the attribute from the relevant tests and makes the implementation pass.

Running `cargo test -- --ignored` at any point between this pre-phase and Phase 9 landing will show the full list of failing red tests and serve as the design contract.

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
| `heuristics__coupling_score_high_confidence` | design doc coupling score >= 2.0 |
| `heuristics__coupling_score_medium_confidence` | design doc coupling score >= 1.0 |
| `heuristics__coupling_score_low_confidence` | design doc coupling score < 1.0 |
| `heuristics__directory_candidate_min_three_files` | design doc min file count = 3 |
| `heuristics__directory_depth_limit_four` | design doc max depth = 4 |
| `heuristics__edge_threshold_two_import_observations` | design doc edge threshold >= 2 |
| `heuristics__summariser_disabled_uses_path_derived_names` | design doc disabled-mode fallback |

## Flagged Vague Criteria

Two scenarios are difficult to isolate in a unit or integration test because they require the full archive pipeline, which does not exist yet:

- **"False positive can be removed" (Req 3, scenario 1):** The assertion depends on archive applying only remaining operations after a human edit. The pre-phase test asserts the delta structure is mutable before archive, but cannot exercise the archive pipeline itself. Phase 9 should sharpen this criterion with a concrete archive-mock or fixture.
- **"Refine does not replace current truth" (Req 2, scenario 2):** The assertion depends on change-aware query semantics that Phase 9 implements. The pre-phase test asserts that `cairn refine` does not overwrite `cairn.blueprint` on disk, which is a necessary but not sufficient condition. Phase 9 should add a query-layer assertion once the change-aware query API exists.

These tests are still written and ignored; they carry a comment marking them as needing sharpening in Phase 9.

## Sample Test Sketch

```rust
#[test]
#[ignore = "awaits phase-9"]
fn init__creates_brownfield_change_directory() {
    let repo = fixture_repo_without_blueprint();
    cairn::init_from_code(&repo, false).expect("init should succeed");
    assert!(repo.path().join("meta/changes/brownfield-init").exists());
    assert!(repo.path().join("meta/changes/brownfield-init/proposal.md").exists());
    assert!(repo.path().join("meta/changes/brownfield-init/blueprint.delta").exists());
    assert!(!repo.path().join("cairn.blueprint").exists());
}
```

Fixtures use `tempdir`-backed helper functions defined at the top of `tests/phase_9_brownfield.rs`. No new test dependencies are required; `tempfile` is already a dev-dependency.
