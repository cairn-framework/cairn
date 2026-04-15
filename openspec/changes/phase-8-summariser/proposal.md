# Proposal: Phase 8 Summariser

## Dependencies

- Requires: `phase-7-mcp`.
- Execution: MUST run after Phase 7 and before Phases 9-10.

## Problem/Context

Earlier phases detect interface contradictions and docstring drift but require humans or agents to write all corrective prose manually. Cairn needs an optional summariser that drafts contract and docstring updates while keeping humans in control.

Phase 8 implements the optional summariser from `docs/spec.md` sections 11, 13, and 14.

## Proposed Solution

Add a pluggable summariser subsystem with:

- Configurable backends: disabled, local command, or hosted API adapter.
- Prompt inputs grounded in ontology facts, contract content, interface changes, docstring facts, and project context/rules.
- Draft records stored under `.cairn/state/summariser/`.
- Three resolution actions: accept, edit through an explicit editable draft file, and discard.
- CLI commands to list, show, accept, edit, and discard drafts.
- No automatic application of summariser output.

## Acceptance Criteria

- Summariser is disabled by default.
- Interface contradictions can trigger a draft contract update when a backend is configured.
- Drafts are persisted with provenance, prompt inputs, backend metadata, and target node.
- Accept replaces the target contract and records the interface hash.
- Edit writes a controlled editable draft path; `draft accept <draft-id> --edited` applies that edited result.
- Discard marks the draft discarded and leaves the contradiction unresolved.
- All strict Rust gates pass.

## Out of Scope

- Selecting a specific hosted model provider as required.
- Brownfield generation, except reusable prompt and backend plumbing.
- Autonomous contract application.
- LSP and plugin packaging.
