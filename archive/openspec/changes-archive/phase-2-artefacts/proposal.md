# Proposal: Phase 2 Artefacts

## Dependencies

- Requires: `phase-1-kernel`.
- Execution: MUST run after Phase 1 and before Phases 3-10.

## Problem/Context

Phase 1 builds the kernel and contract-only ontology. Cairn v0.6 requires a complete project metadata layer where authority artefacts and provenance artefacts attach to stable DSL nodes and feed agent-facing queries.

Phase 2 implements `docs/spec.md` section 8 and the Phase 2 query additions from section 12. The ontology SHALL carry todos, decisions, reviews, research, and sources with integrity rules, while retaining the contract support delivered by Phase 1.

## Proposed Solution

Add the full artefact type system:

- Todo artefacts with status, `satisfies`, and node linkage.
- Decision artefacts using extended ADR frontmatter, status, cross-references, and provenance links.
- Review artefacts with `human`, `agent_introspective`, and `agent_cross_model` subtypes.
- Research artefacts linked to nodes and sources.
- Source artefacts with `verified`, `external`, and `unverified` integrity states.
- Query commands: `rationale`, `sources`, `research`, `decisions`, `todos`, and `status`.

## Acceptance Criteria

- The scanner loads all six v1 artefact types: contracts plus todos, decisions, reviews, research, and sources.
- Every artefact type validates required frontmatter fields and reports structural errors or rationale tensions according to `docs/spec.md` section 8.
- Decision-to-decision links validate `supersedes`, `refines`, and `related`.
- Source SHA-256 verification detects tampering for `verified` local files.
- `neighbourhood` defaults include contracts and accepted decisions only.
- New query commands return stable JSON and labelled human output.
- All strict Rust gates pass.

## Out of Scope

- Change directory semantics and proposed artefact operations.
- Archive, rename, and change-aware queries.
- Hook execution beyond scanner/lint findings.
- Edge validation, docstring generation, multi-target reconciliation, MCP, summariser, brownfield extraction, LSP, and plugin packaging.
