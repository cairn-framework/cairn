# Tasks: Phase 9.0 Brownfield Tests

## 1. Test File

- [ ] 1.1 Create `tests/phase_9_brownfield.rs` with fixture helpers (`fixture_repo_without_blueprint`, `fixture_repo_with_blueprint`, `fixture_repo_with_brownfield_change`).

## 2. Init Tests (spec Req 1)

- [ ] 2.1 Write `init__discovery_does_not_require_existing_blueprint`: asserts `init_from_code` does not error when no `cairn.blueprint` exists in fixture repo.
- [ ] 2.2 Write `init__candidate_heuristics_are_deterministic`: asserts that running `init_from_code` twice on the same fixture produces identical candidate sets.
- [ ] 2.3 Write `init__creates_brownfield_change_directory`: asserts `meta/changes/brownfield-init/`, `proposal.md`, `blueprint.delta`, and stub contracts are created; asserts `cairn.blueprint` is not created.
- [ ] 2.4 Write `init__existing_change_protected_without_force`: asserts `init_from_code` returns error code 1 when `brownfield-init/` already exists and `--force` is not set.
- [ ] 2.5 Write `init__force_replaces_existing_change`: asserts `init_from_code --force` replaces the existing change directory; asserts main `cairn.blueprint` and main `meta/` artefacts are untouched.

## 3. Refine Tests (spec Req 2)

- [ ] 3.1 Write `refine__proposes_additions_for_new_directories`: asserts `refine` creates a change directory and its `blueprint.delta` contains only ADDED operations for directories not in the existing blueprint.
- [ ] 3.2 Write `refine__does_not_replace_current_truth`: asserts `refine` does not overwrite `cairn.blueprint` on disk. Mark with `// NOTE: needs sharpening in phase-9 once change-aware query API exists`.

## 4. Human Review Test (spec Req 3)

- [ ] 4.1 Write `review__false_positive_deletion_respected`: asserts that deleting a node entry from the generated `blueprint.delta` before archive leaves the delta valid and does not re-add the entry. Mark with `// NOTE: needs sharpening in phase-9 with archive-mock`.

## 5. MCP Tests (spec Req 4)

- [ ] 5.1 Write `mcp__brownfield_tools_absent_in_default_mode`: asserts `cairn_init_from_code` and `cairn_refine` are not present in the tool registry when mutating tools are disabled.
- [ ] 5.2 Write `mcp__brownfield_tools_present_in_mutating_mode`: asserts both tools appear in the registry when mutating tools are enabled.

## 6. Heuristic Invariant Tests (design doc)

- [ ] 6.1 Write `heuristics__coupling_score_high_confidence`: asserts score `(3+1)/(1+1) = 2.0` maps to high confidence.
- [ ] 6.2 Write `heuristics__coupling_score_medium_confidence`: asserts score `(1+1)/(1+1) = 1.0` maps to medium confidence.
- [ ] 6.3 Write `heuristics__coupling_score_low_confidence`: asserts score `(0+1)/(2+1) = 0.33` maps to low confidence.
- [ ] 6.4 Write `heuristics__directory_candidate_min_three_files`: asserts a directory with exactly 3 source files becomes a candidate; a directory with 2 files and no import edges does not.
- [ ] 6.5 Write `heuristics__directory_depth_limit_four`: asserts a directory at depth 5 below repo root is not emitted as a candidate without explicit config.
- [ ] 6.6 Write `heuristics__edge_threshold_two_import_observations`: asserts an edge is emitted between two candidates when there are at least 2 import observations; asserts no edge is emitted for 1 observation without a high-confidence public API reference.
- [ ] 6.7 Write `heuristics__summariser_disabled_uses_path_derived_names`: asserts that when the summariser is disabled, candidate names derive from directory paths and contain no AI-generated content.

## 7. Required Verification

- [ ] 7.1 `cargo build` passes with zero warnings.
- [ ] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 7.3 `cargo fmt --check` passes.
- [ ] 7.4 `cargo test` passes (all phase-9 tests are skipped via `#[ignore]`).
- [ ] 7.5 `cargo test -- --ignored` shows all 17 phase-9 tests as failing (not compile errors).
- [ ] 7.6 `bash scripts/pre-archive-rust-gates.sh` passes end-to-end.
