# Proposal: Phase 4 Hooks

## Dependencies

- Requires: `phase-3-changes`.
- Execution: MUST run after Phase 3 and before Phases 5-10.

## Problem/Context

Phases 1-3 provide map reconciliation and safe change isolation, but enforcement still depends on humans manually running commands. Cairn needs hook entrypoints that make structural and interface integrity part of task and commit boundaries.

Phase 4 implements `docs/spec.md` section 11 and the phase-4 conflict detection called out in sections 9 and 16.

## Proposed Solution

Add hook commands and integration scripts for:

- Structural hook: blocks on structural errors.
- Interface hook: blocks on unresolved interface contradictions.
- Tension hook: reports rationale tensions without blocking.
- Active-change conflict detection before archive, including overlapping blueprint operations and artefact operation collisions.
- Git pre-commit and agent-task-end entrypoints that can run the same hook engine.

## Acceptance Criteria

- `cairn hook structural` exits non-zero on structural errors.
- `cairn hook interface` exits non-zero on unresolved interface contradictions.
- `cairn hook tension` prints tensions and exits zero.
- `cairn hook all` runs structural, interface, and tension hooks with documented exit semantics.
- Active changes that conflict are detected before archive.
- Hook output is available in human and JSON formats.
- All strict Rust gates pass.

## Out of Scope

- Edge validation and docstring drift detection beyond existing Phase 1-3 findings.
- Summariser-driven contract updates.
- Multi-target reconciliation, MCP, brownfield extraction, LSP, and plugin packaging.
