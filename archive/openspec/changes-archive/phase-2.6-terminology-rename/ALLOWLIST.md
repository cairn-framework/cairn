# Terminology Rename Allowlist

This file records intentional legacy-term matches surfaced by the final sweeps.

## Active Implementation Matches

| Path | Lines | Justification |
|---|---:|---|
| `src/blueprint/parser.rs` | 19, 24, 29 | Required legacy `.dsl` rejection branch and diagnostic text. |
| `src/cli/mod.rs` | 227, 232 | Required legacy-only default scan error when only `cairn.dsl` exists. |
| `src/cli/mod.rs` | 314, 315 | Required collision warning when `cairn.dsl` exists beside `cairn.blueprint`. |
| `CHANGELOG.md` | 5, 6, 7 | v0.7 changelog names the three renames. |

## Phase 2.6 Change Record

| Path | Lines | Justification |
|---|---:|---|
| `openspec/changes/phase-2.6-terminology-rename/proposal.md` | 12, 13, 21, 22, 23, 30, 32, 33, 34, 51 | This phase's Problem/Context, Proposed Solution, Acceptance Criteria, and Out of Scope sections name the source terms being renamed. |
| `openspec/changes/phase-2.6-terminology-rename/design.md` | 5, 17, 18, 19, 20, 21, 22, 33, 34, 36, 42, 43, 48, 54, 55, 78, 85, 88, 93, 96 | This phase's References, Rename Mapping, execution instructions, trade-offs, risks, and testing sections name the source terms being renamed. |
| `openspec/changes/phase-2.6-terminology-rename/tasks.md` | 5, 6, 7, 8, 17, 27, 31, 32, 33, 42, 50, 60, 62 | Task text describes the migration and sweep commands for this phase. |
| `openspec/changes/phase-2.6-terminology-rename/specs/terminology-rename/spec.md` | 18, 25, 27, 28, 39, 50 | Legacy-extension and legacy-output rejection scenarios must name the old file or term. |

## Archived OpenSpec Records

All matches under `openspec/changes/archive/**` are historical records intentionally left untouched per task 6.10. The relevant files are:

- `openspec/changes/archive/phase-0-foundation/{proposal.md,design.md,tasks.md,specs/foundation/spec.md}`
- `openspec/changes/archive/phase-1-kernel/{proposal.md,design.md,tasks.md,specs/kernel/spec.md}`
- `openspec/changes/archive/phase-2-artefacts/{proposal.md,specs/artefacts/spec.md}`
- `openspec/changes/archive/phase-2.5-graph-explorer/{proposal.md,design.md,specs/graph-explorer/spec.md}`
