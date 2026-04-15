# Proposal: Phase 9 Brownfield Extraction

## Dependencies

- Requires: `phase-8-summariser`.
- Execution: MUST run after Phase 8 and before Phase 10.

## Problem/Context

Cairn now has a mature DSL, reconcilers, artefacts, change system, hooks, MCP access, and summariser. Existing projects still need a path to adopt Cairn without manually authoring the initial ontology from nothing.

Phase 9 implements brownfield extraction from `docs/spec.md` sections 12, 15, and 14.

## Proposed Solution

Add:

- `cairn init --from-code` to generate an initial `cairn.dsl` and stub contracts in `meta/changes/brownfield-init/`.
- `cairn refine` to propose deltas against an existing DSL based on code changes.
- Structural candidate extraction from reconciler output.
- Summariser-assisted naming, descriptions, tags, and obvious edges.
- Human review through the Phase 3 change archive workflow.

## Acceptance Criteria

- Brownfield init never writes directly to main `cairn.dsl` or main `meta/` artefacts.
- Generated candidates include nodes, paths, stub contracts, and obvious edges.
- Summariser outputs are marked as proposed and require human archive.
- `refine` produces a delta instead of a full redraft when a DSL already exists.
- False positives can be deleted from the generated change before archive.
- All strict Rust gates pass.

## Out of Scope

- Perfect architecture inference.
- Autonomous archive of generated brownfield output.
- Distribution packaging, LSP, and editor plugins.
