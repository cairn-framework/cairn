# Proposal: Phase 7 MCP

## Dependencies

- Requires: `phase-6-multi-target`.
- Execution: MUST run after Phase 6 and before Phases 8-10.

## Problem/Context

Phases 1-6 expose Cairn through CLI commands and JSON output. Agents need direct structured access to the ontology without shelling out and parsing CLI text.

Phase 7 implements the MCP wrapper described in `docs/spec.md` sections 12 and 14. The wrapper SHALL expose the existing query layer and compose project context and rules into responses.

## Proposed Solution

Add an MCP server binary that:

- Wraps existing Cairn query APIs.
- Exposes tools for `get`, `neighbourhood`, `contract`, `files`, `dependents`, `depends`, `order`, `lint`, `status`, `rationale`, artefact queries, and change queries.
- Reads `cairn.config.yaml` context and rules.
- Prepends project context and relevant rules to agent-facing responses.
- Returns stable structured content with explicit errors.

## Acceptance Criteria

- MCP tools call the same library query functions as the CLI.
- Tool responses include project context and artefact-specific rules where applicable.
- The MCP server supports stdio transport.
- Errors use stable machine-readable codes and human-readable messages.
- CLI JSON schemas and MCP response schemas remain aligned.
- All strict Rust gates pass.

## Out of Scope

- LSP server.
- Editor packaging.
- Summariser execution.
- Brownfield extraction beyond exposing existing commands if already implemented.
