# Proposal: Phase 5 Edge Validation and Docstrings

## Dependencies

- Requires: `phase-4-hooks`.
- Execution: MUST run after Phase 4 and before Phases 6-10.

## Problem/Context

The Phase 1 code reconciler records files and interface fingerprints, but it does not validate whether declared DSL edges match observed source dependencies or whether authored docstrings drift from ontology facts.

Phase 5 implements the semantic reconciliation capabilities described in `docs/spec.md` sections 10, 12, 14, and 16.

## Proposed Solution

Extend the code reconciler and CLI with:

- Observed dependency extraction for Rust imports, module references, and public API references.
- Edge divergence findings when declared DSL edges and observed dependencies disagree.
- Authored docstring fact extraction for module name, dependencies, tags, and contract references.
- Docstring drift findings when authored docstrings contradict ontology facts.
- `cairn docstring <node> [--language <lang>]` template generation for Rust, Python, TypeScript, and Go.

## Acceptance Criteria

- Missing observed dependencies for declared edges surface as rationale tensions.
- Observed dependencies without declared edges surface as rationale tensions.
- Docstring facts that contradict ontology facts surface as rationale tensions.
- `cairn docstring` emits deterministic language-aware templates grounded in ontology facts.
- Hooks from Phase 4 report these tensions without blocking by default.
- All strict Rust gates pass.

## Out of Scope

- Multi-target interface comparison across path lists.
- Additional reconciler languages beyond template formatting support.
- Summariser-generated docstring prose.
- MCP, brownfield extraction, LSP, and plugin packaging.
