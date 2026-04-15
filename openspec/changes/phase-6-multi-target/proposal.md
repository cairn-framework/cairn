# Proposal: Phase 6 Multi-Target and Languages

## Problem/Context

Earlier phases parse path lists but reconcile primarily against Rust source. Cairn v0.6 requires modules with multiple paths to reconcile every target independently, compare per-target interface hashes, and support additional languages.

Phase 6 implements multi-target semantics from `docs/spec.md` sections 7, 10, 14, and 16.

## Proposed Solution

Add:

- Target records for every claimed path in a path list.
- Per-target interface hashes in `.cairn/state/interface-hashes.json`.
- Multi-target divergence findings.
- Reconciler dispatch by target language and path.
- Additional code reconcilers for TypeScript, Python, and Go.
- CLI output that reports target-level files, states, hashes, and findings.

## Acceptance Criteria

- A module with `path ["./core-rust", "./core-ts"]` reconciles both targets.
- Interface hashes are stored per node and target.
- Divergent target interfaces surface as structural errors when targets claim the same contract and rationale tensions when documented as intentional asymmetry.
- `files`, `get`, `scan`, `lint`, and JSON output expose target-level state.
- Rust, TypeScript, Python, and Go reconcilers run through the shared trait interface.
- All strict Rust gates pass.

## Out of Scope

- Non-code reconcilers for org structure, product BOMs, or research programmes.
- MCP wrapping, summariser, brownfield extraction, LSP, and plugin packaging.
