# Tasks: Phase 8 Summariser

## 1. Backend and Configuration

- [ ] 1.1 Add summariser configuration with disabled default.
- [ ] 1.2 Define backend trait, request type, draft type, and error type.
- [ ] 1.3 Define provider-neutral `SummariserRequest` and `SummariserResponse` JSON schemas.
- [ ] 1.4 Implement local command backend using stdin/stdout JSON, timeout, non-zero exit handling, and invalid response handling.
- [ ] 1.5 Implement hosted API adapter boundary without committing secrets.
- [ ] 1.6 Add deterministic fake backend for tests.

## 2. Prompt Inputs and Draft Storage

- [ ] 2.1 Build prompt inputs from ontology facts, contracts, interface changes, docstring findings, context, rules, and bounded code samples.
- [ ] 2.2 Persist drafts under `.cairn/state/summariser/`.
- [ ] 2.3 Track draft status transitions.
- [ ] 2.4 Add tests for prompt input byte limits, sample truncation, and persisted metadata.

## 3. Resolution Actions

- [ ] 3.1 Implement draft generation without applying output.
- [ ] 3.2 Implement accept action and interface hash recording.
- [ ] 3.3 Implement edit action and explicit edited-content acceptance.
- [ ] 3.4 Implement discard action that preserves unresolved contradictions.
- [ ] 3.5 Add tests for every action.

## 4. CLI and Documentation

- [ ] 4.1 Implement `summarise`, `drafts`, `draft show`, `draft accept`, `draft edit`, and `draft discard`.
- [ ] 4.2 Add JSON schemas and output snapshots.
- [ ] 4.3 Register summariser and draft commands in the shared MCP query tool registry with correct read-only versus mutating safety classes.
- [ ] 4.4 Document backend configuration, safety model, MCP exposure, and resolution actions.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes.
- [ ] 5.5 `cargo test --locked` passes.
- [ ] 5.6 `python3 /Users/george/repos/cairn/.agents/skills/cflx-proposal/scripts/cflx.py validate phase-8-summariser --strict` passes.
