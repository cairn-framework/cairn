# Design: Phase 7.6.0 Tests (AI Provenance Foundation Pre-Phase)

## References

- `openspec/changes/phase-7.6-ai-provenance-foundation/specs/provenance-foundation/spec.md`: source of scenarios 1-8.
- `openspec/changes/phase-7.6-ai-provenance-foundation/specs/changes/spec.md`: source of scenarios 9-16.
- `openspec/changes/phase-7.6-ai-provenance-foundation/specs/cli/spec.md`: source of scenarios 17-23.
- `openspec/changes/phase-7.6-ai-provenance-foundation/specs/query/spec.md`: source of scenarios 24-27.
- `openspec/specs/testing-baseline/spec.md`: test-first pre-phase convention.
- `openspec/conventions.md`: `#[ignore = "awaits phase-<N>"]` pattern.

## Test File Placement

A single new integration test file: `tests/phase_7_6_ai_provenance.rs`.

Integration test files under `tests/` are compiled as separate crates by Cargo. This placement:

- Keeps the pre-phase delta self-contained and easy to diff.
- Avoids touching any `src/` file before phase-7.6 lands.
- Matches the convention established by existing files under `tests/`.

Phase-7.6 will either flesh out these test bodies in-place or move individual tests closer to the modules they exercise, removing `#[ignore]` as each group lands.

## Test Body Convention

Every test body is `todo!("awaits phase-7.6: <scenario name>")`. This ensures:

- The file compiles.
- Running with `--ignored` produces a clear failure message naming the missing scenario.
- Grepping for `todo!` after phase-7.6 apply confirms no scenario was silently dropped.

## Scenario-to-Test Mapping

Four modules within the file, one per spec source, using `mod` blocks for grouping:

```
mod provenance_foundation { ... }  // Requirement: Persist a per-archived-change trace sidecar
mod changes { ... }                // Requirement: Carry a suggested-edges queue inside the change directory
mod cli { ... }                    // Requirement: Surface architectural islands and orphan inclusion
mod query { ... }                  // Requirement: Answer disconnected-subgraph queries
```

Each `mod` contains the tests for its requirement's scenarios. All tests carry both `#[test]` and `#[ignore = "awaits phase-7.6"]`.

| # | Spec source | Scenario | Test function |
|---|---|---|---|
| 1 | provenance-foundation | Sidecar is state-versioned | `sidecar_is_state_versioned` |
| 2 | provenance-foundation | Sidecar covers the four cairn-native stages | `sidecar_covers_four_native_stages` |
| 3 | provenance-foundation | Prompt content is reserved but empty in this phase | `prompt_content_reserved_but_empty` |
| 4 | provenance-foundation | Higher version than understood fails with a clear error | `higher_version_fails_with_clear_error` |
| 5 | provenance-foundation | Default human output is labelled per stage | `trace_human_output_labels_each_stage` |
| 6 | provenance-foundation | JSON output is the schema with promoted version | `trace_json_output_is_schema_with_version` |
| 7 | provenance-foundation | Missing sidecar exits cleanly | `trace_missing_sidecar_exits_cleanly` |
| 8 | provenance-foundation | Trace command does not own semantics | `trace_command_delegates_to_library_reader` |
| 9 | changes | Queue file is state-versioned | `queue_file_is_state_versioned` |
| 10 | changes | Each entry carries source, target, relation, and triage state | `entry_carries_source_target_relation_and_triage_state` |
| 11 | changes | Triage state defaults to pending for newly-emitted entries | `triage_state_defaults_to_pending` |
| 12 | changes | Queue is a sibling, not a delta operation | `queue_is_sibling_not_delta_operation` |
| 13 | changes | Validate without --strict surfaces count as warning | `validate_without_strict_surfaces_warning` |
| 14 | changes | Validate --strict fails with CC002 on pending entries | `validate_strict_fails_cc002_on_pending` |
| 15 | changes | Validate --strict passes when all entries are non-pending | `validate_strict_passes_when_all_non_pending` |
| 16 | changes | Absent queue file is not an error | `absent_queue_file_is_not_error` |
| 17 | cli | Islands command returns whole-graph component breakdown | `islands_returns_component_breakdown` |
| 18 | cli | Islands JSON output is versioned | `islands_json_output_is_versioned` |
| 19 | cli | Neighbourhood with --include-orphans surfaces reverse-only nodes | `neighbourhood_include_orphans_surfaces_reverse_only` |
| 20 | cli | Both forms delegate to the library query | `both_forms_delegate_to_library_query` |
| 21 | cli | Trace human output names each stage | `trace_human_output_labels_each_stage` (same as #5) |
| 22 | cli | Trace JSON output is exactly the sidecar payload | `trace_json_output_is_schema_with_version` (same as #6) |
| 23 | cli | Trace exits non-zero when the sidecar is missing | `trace_missing_sidecar_exits_cleanly` (same as #7) |
| 24 | query | Islands returns one entry per connected component | `query_islands_returns_one_entry_per_component` |
| 25 | query | Islands handles the trivial single-component case | `query_islands_handles_single_component` |
| 26 | query | Neighbourhood with include_orphans surfaces inbound-only neighbours | `query_neighbourhood_include_orphans_surfaces_inbound_only` |
| 27 | query | Islands query response is versioned | `query_islands_response_is_versioned` |

Total: 24 unique tests. Scenarios 21-23 reference the same test names as 5-7 because the CLI trace scenarios duplicate the provenance-foundation trace render scenarios.

## Compile Dependency

The test file imports nothing from `src/` at pre-phase time. Any use statements needed by the eventual implementations are added by phase-7.6 as it removes `#[ignore]` attributes.

## Phase-7.6 Removal Contract

Phase-7.6 tasks.md specifies that the first task in each group removes `#[ignore]` from the corresponding test and makes it pass. The grouping here (4 mods, 24 tests) aligns directly with phase-7.6 task groups.

## Vague Scenario Flagged

No scenarios in the phase-7.6 specs are flagged as vague. All 24 acceptance criteria carry concrete THEN clauses that map directly to assertions.
