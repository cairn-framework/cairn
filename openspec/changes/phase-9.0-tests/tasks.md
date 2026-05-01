# Tasks: Phase 9.0 Brownfield Tests

## 1. Test File

- [ ] 1.1 Create `tests/phase_9_brownfield.rs` with fixture helpers (`fixture_repo_without_blueprint`, `fixture_repo_with_blueprint`, `fixture_repo_with_brownfield_change`).
- [ ] 1.2 Add additional fixture helpers needed by Wave 4 stubs: `fixture_repo_with_pending_suggested_edges`, `fixture_change_with_partial_interview_state`, `fixture_project_config_with_contract_template`, and `fixture_decision_with_obligations_field`. Helpers may return placeholder values until Phase 9 supplies them; tests calling these helpers carry `unimplemented!()` markers so the runtime failure is informative.

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

## 7. Suggest Engine Tests (spec Req 5)

- [ ] 7.1 Write `suggest__engine_writes_to_queue_file`: asserts the suggest engine emits a cross-cutting edge into `openspec/changes/<change>/suggested-edges.json` during a brownfield init run that triggers the engine. Stub calls `unimplemented!()` until the engine fixture exists.
- [ ] 7.2 Write `suggest__entry_triage_state_is_pending`: asserts every entry written by the engine has `triage_state == "pending"` regardless of computed confidence.
- [ ] 7.3 Write `suggest__entry_provenance_carries_trace_phase`: asserts the `provenance.trace_phase` field on a written entry names the phase that produced it.
- [ ] 7.4 Write `suggest__pending_entries_block_archive_with_cc002`: asserts `cflx openspec validate <change> --strict` exits non-zero with error code `CC002` when pending entries exist; the failure message names the pending count and the queue file path.
- [ ] 7.5 Write `suggest__no_auto_accept_on_high_confidence`: asserts a synthetic entry with high computed confidence still lands as `pending`; no auto-accept policy promotes it.
- [ ] 7.6 Write `suggest__manual_test_entries_accept_empty_provenance`: asserts a manually authored entry with no producing trace context loads cleanly with an empty `provenance` object and the schema-version check still passes.

## 8. Interview Runner Tests (spec Req 6)

- [ ] 8.1 Write `interview__session_persists_across_invocations`: asserts an in-progress brownfield onboarding session is detected and resumed at the next outstanding turn after re-invocation against the same change directory. Stub uses `unimplemented!()` until the session-persistence fixture exists.
- [ ] 8.2 Write `interview__final_transcript_lands_at_genesis_path`: asserts the transcript writes to `openspec/changes/<id>/research/genesis.md`, carries the user-visible Q/A turns and final premise, and the `nodes` field carries the change ID per `openspec/conventions.md` Section 9.
- [ ] 8.3 Write `interview__session_state_never_leaks_outside_change_dir`: asserts all reads and writes happen inside `openspec/changes/<change>/research/`; asserts no session state lands in main `meta/` or in `cairn.blueprint`.

## 9. Templated Authoring Tests (spec Req 7)

- [ ] 9.1 Write `templates__matching_template_guides_stub_authoring`: asserts a project-config-declared contract template whose match rule covers a generated candidate is applied; the draft uses the template's required headers and optional sections; summariser-supplied content fills the body sections per the documented precedence rule. Stub uses `unimplemented!()` until the project-config fixture exists.
- [ ] 9.2 Write `templates__non_matching_candidates_fall_back_to_builtin`: asserts a candidate with no matching template uses the built-in minimum-viable stub; asserts authoring completes without error.
- [ ] 9.3 Write `templates__ill_formed_template_does_not_block_authoring`: asserts an unparseable template body causes a logged warning naming the offending template; asserts authoring continues using the built-in stub for affected candidates.

## 10. Decision-Attached Obligations Tests (spec Req 8, conditional)

- [ ] 10.1 Write `obligations__populated_when_field_exists`: asserts that when decision artefacts in this phase declare an `obligations` field, an AI-suggested decision carries the obligations identified by the summariser; asserts the generated change directory exposes the obligations alongside the decision body. Stub guard-comment names current schema state; carries `unimplemented!()` if fixture not present.
- [ ] 10.2 Write `obligations__reviewable_before_archive`: asserts the obligations field is editable and removable in a generated change; asserts archive applies only the human-reviewed obligations.
- [ ] 10.3 Write `obligations__no_op_when_field_absent`: asserts that when decision artefacts in this phase do not declare an `obligations` field, the brownfield generator emits decisions using the existing schema with no obligations-related output. This stub is the always-callable branch; the other two stubs become callable when the schema gains the field.

## 11. Required Verification

- [ ] 11.1 `cargo build` passes with zero warnings.
- [ ] 11.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 11.3 `cargo fmt --check` passes.
- [ ] 11.4 `cargo test` passes (all phase-9 tests are skipped via `#[ignore]`).
- [ ] 11.5 `cargo test -- --ignored` shows all 30 phase-9 tests as failing (not compile errors); the count is 23 acceptance-criterion stubs plus 7 heuristic-invariant stubs.
- [ ] 11.6 `bash scripts/pre-archive-rust-gates.sh` passes end-to-end.
- [ ] 11.7 `python3 .claude/skills/cflx-proposal/scripts/cflx.py validate phase-9.0-tests --strict` exits 0.
