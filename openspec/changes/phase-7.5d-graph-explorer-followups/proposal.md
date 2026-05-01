# Proposal: Phase 7.6 Graph Explorer Follow-ups

**Change Type**: spec-only

## Dependencies

- `phase-2.5-graph-explorer` (archived).

## Problem/Context

Bundle C from `docs/strongholds/getcairn-cross-check-C.md` named four follow-ups against the graph-explorer spec: Prerequisite/Enables widget (C4.b), click-to-recenter (C4.c), default-visible verb labels on dependency edges (C5.b), and uniform inspector chrome (C9). All four read existing `CairnResponse` shapes without modification.

## Proposed Solution

Add four ADDED Requirements to `openspec/specs/graph-explorer/spec.md`. C4.b is a derived UI surface, not a new artefact type. C9 binds chrome styling to `docs/design-system/tokens.css`.

## Acceptance Criteria

- The four ADDED requirements appear and `cflx openspec validate phase-7.5d-graph-explorer-followups --strict` passes.
- No `CairnResponse` shape changes per the UI Maintenance Contract requirement.

## Out of Scope

Systemigram visual render mode (C5.d), decision-attached obligations as a widget source, and URL persistence of viewport state.
