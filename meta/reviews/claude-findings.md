# Claude Headless Review Findings

## Summary
Pass-with-notes

## Critical Issues

| # | File | Issue | Recommended Fix |
|---|---|---|---|
| 1 | `openspec/changes/phase-7.6.0-tests/{design,tasks,spec}.md` | Phase argument uses `phase = 76`, violating the zero-padded injectivity rule in `openspec/conventions.md` §5 which mandates `major * 100 + minor` (i.e. `phase = 706` for phase-7.6). | Replace all `phase = 76` with `phase = 706` across design, tasks, proposal, and spec delta. |
| 2 | `openspec/changes/phase-7.7.0-tests/{design,tasks,spec}.md` | Phase argument uses `phase = 77`; conventions.md requires `phase = 707` for phase-7.7. | Replace all `phase = 77` with `phase = 707`. |
| 3 | `openspec/changes/phase-7.8.0-tests/{design,tasks,spec}.md` | Phase argument uses `phase = 78`; conventions.md requires `phase = 708` for phase-7.8. | Replace all `phase = 78` with `phase = 708`. |

## Major Issues

| # | File | Issue | Recommended Fix |
|---|---|---|---|
| 4 | `openspec/changes/phase-7.7.0-tests/{design,tasks,spec}.md` | Test function names use `check__`, `empty_state__`, `explorer__`, and `reconciliation__` prefixes without the required `test_` prefix, violating `openspec/conventions.md` §5 naming convention (`test_{feature}_{scenario}_{outcome}`). | Prefix all test function names with `test_` (e.g. `test_check__whole_map_inspection_without_arguments`). |
| 5 | `openspec/changes/phase-7.8.0-tests/{design,tasks,spec}.md` | Test function names are bare (e.g. `default_format_is_json`) without the required `test_` prefix, violating conventions.md §5. | Prefix all test function names with `test_` (e.g. `test_default_format_is_json`). |
| 6 | `openspec/changes/phase-7.6.0-tests/design.md` | Scenario-to-test mapping table lists function names without `test_` prefix (e.g. `sidecar_is_state_versioned`), but tasks.md and spec.md use `test_sidecar_is_state_versioned`. | Update the mapping table to match the canonical `test_` prefixed names used in tasks.md and spec.md. |
| 7 | `openspec/changes/phase-{8.0,9.0,10.0}-tests/` | These pre-phase proposals still reference the legacy `#[ignore = "awaits phase-N"]` and `todo!()` patterns, while `openspec/conventions.md` and `openspec/specs/testing-baseline/spec.md` now mandate `#[cflx_planned(phase = <N>)]` and `unimplemented!()`. This creates policy drift across the pre-phase queue. | Bulk-update 8.0/9.0/10.0 proposals to use `#[cflx_planned]` with correct phase encoding and `unimplemented!()` bodies, aligning with the convention established by 7.5c. |
| 8 | `src/cli/accept.rs` | `read_planned_tests()` result is computed but discarded (`let _planned_tests = ...`), making the planned-test sidecar dead code. `PlannedTest` struct fields carry `#[allow(dead_code)]` without `// Reason:` comments, violating the AGENTS.md guardrail. The `VerificationState` enum only models 3 of 5 documented states (`Draft` and `Planned` are missing). The manual JSON parser is brittle when `serde` is already a workspace dependency. | Either wire `planned_tests` into the gate logic (e.g. report planned-test counts in output) or remove the dead code. Add `// Reason:` comments to any retained `#[allow(...)]`. Expand `VerificationState` to include `Draft` and `Planned`, or document why the gate only tracks three states. Replace manual JSON parsing with `serde_json`. |
| 9 | `openspec/changes/phase-7.7.0-tests/design.md` | The "token-only styling" scenario (scenario 8) is flagged as vague, but the recommended reduced-scope assertion ("component class exists and no new hex values appear in the component's rule block") still requires CSS parsing in a Rust integration test. This is a non-trivial dependency. | Either add a snapshot-test strategy to the design note, or move the hex-value assertion to a design-system-level gate/script rather than a Rust test. |

## Minor Issues / Notes

| # | File | Issue | Recommended Fix |
|---|---|---|---|
| 10 | `.pre-commit-config.yaml` | The `openspec validate --specs --strict` hook is correctly placed in the `pre-push` stage, not `pre-commit`. This is the right long-term boundary for a relatively slow spec-validation step. | No action required; placement is correct. |
| 11 | `openspec/specs/*/spec.md` (12 files) | `## Purpose` sections were added by commit `c1e8d34`. The prose is concise, descriptive, and free of em-dashes. They improve spec navigability. | No action required. |
| 12 | `openspec/changes/phase-7.7.0-tests/` and `phase-7.8.0-tests/` | Em-dash compliance is correctly asserted as a test scenario (7.7 scenarios 7/11, 7.8 scenario 4). No literal U+2014 characters were found in user-facing copy of the reviewed files. | No action required. |
| 13 | `src/cli/accept.rs` | Uses `std::path::PathBuf` instead of `camino::Utf8PathBuf` per conventions.md §4. This is acceptable because `accept.rs` is internal CLI code, not a public API surface. | Optional: migrate to `camino` for consistency. |
| 14 | `openspec/changes/phase-7.6.0-tests/design.md` | The 27-scenario → 24-unique-test mapping is clearly documented, including the three duplicate trace scenarios shared between provenance-foundation and CLI specs. | No action required. |
| 15 | `openspec/conventions.md` §5 | Aligns with `openspec/specs/testing-baseline/spec.md` Requirement 4 and the TDD posture investigation conclusion: test-first at the phase boundary, not per-function TDD. No contradiction. | No action required. |
| 16 | `openspec/changes/phase-9.0-tests/tasks.md` | Task items 11.6 and 11.7 reference `pre-archive-rust-gates.sh` and `cflx.py validate`, but the commit that created 7.x pre-phases moved openspec validation out of `cairn accept` and into `.pre-commit-config.yaml`. Ensure phase-9.0-tests does not re-introduce a direct `openspec validate` call inside `cairn accept` when it applies. | Watch for this during phase-9.0-tests apply review. |

## Consensus Recommendation

**Pass-with-notes.** The three `.0-tests` pre-phase proposals are structurally sound and follow the established pattern, but they contain a critical phase-encoding error (`76`/`77`/`78` instead of `706`/`707`/`708`) that must be fixed before any apply agent writes Rust source. The naming inconsistency (`test_` prefix missing in 7.7.0 and 7.8.0) and the policy drift with 8.0/9.0/10.0 are secondary but should be normalized to avoid confusing future apply agents. The `accept.rs` dead code around `read_planned_tests()` should be either wired up or removed before the next phase that touches the gate logic.
