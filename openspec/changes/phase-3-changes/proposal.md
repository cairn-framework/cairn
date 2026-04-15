# Proposal: Phase 3 Changes

## Dependencies

- Requires: `phase-2-artefacts`.
- Execution: MUST run after Phase 2 and before Phases 4-10.

## Problem/Context

Phases 1 and 2 make Cairn capable of reading current architectural truth. Cairn now needs a safe way to propose modifications without mutating the main tree directly.

Phase 3 implements `docs/spec.md` section 9 and the change commands from section 12. Proposed ontology modifications SHALL live in isolated change directories until archive applies them atomically.

## Proposed Solution

Implement the change system:

- `meta/changes/<change-id>/` discovery and validation.
- `dsl.delta` parsing for added, modified, removed, and renamed nodes and edges.
- Artefact operation frontmatter: `added`, `modified`, `removed`, and `renamed`.
- Archive ordering: renamed, removed, modified, added.
- Atomic archive with rollback on validation, scan, structural error, or interface contradiction failure.
- Change-aware queries: `changes`, `show`, `archive`, `rename`, and `neighbourhood --include-changes`.

## Acceptance Criteria

- Active changes never affect default current-truth queries.
- `cairn changes` lists active change directories with summaries and operation counts.
- `cairn show <change>` displays DSL and artefact deltas.
- `cairn archive <change>` applies deltas atomically, validation-scans with the archiving change excluded from active-change discovery, rolls back on failure, moves the change to dated archive, final-scans generated outputs, and appends `.cairn/log.md`.
- `cairn rename <old-id> <new-id>` creates a reviewable change directory that propagates references through DSL edges and artefact frontmatter.
- Change-aware neighbourhood output is opt-in.
- All strict Rust gates pass.

## Out of Scope

- Concurrent change conflict detection beyond single-change archive validation.
- Hook wiring for structural, interface, or tension gates.
- Edge validation against imports, docstring generation, multi-target reconciliation, MCP, summariser, brownfield extraction, LSP, and plugin packaging.
