# Tasks: Phase 8 Summariser

## 1. Backend and Configuration

- [x] 1.1 Add summariser configuration with disabled default.
- [x] 1.2 Define backend trait, request type, draft type, and error type.
- [x] 1.3 Define provider-neutral `SummariserRequest` and `SummariserResponse` JSON schemas.
- [x] 1.4 Implement local command backend using stdin/stdout JSON, timeout, non-zero exit handling, and invalid response handling.
- [x] 1.5 Implement hosted API configuration parsing and validation as an adapter boundary without committing secrets; a concrete hosted provider may return an unsupported-backend error until a later adapter exists.
- [x] 1.6 Add deterministic fake backend for tests.

## 2. Prompt Inputs and Draft Storage

- [x] 2.1 Build prompt inputs from map facts, contracts, interface changes, docstring findings, context, rules, and bounded code samples.
- [x] 2.2 Persist drafts under `.cairn/state/summariser/`. (DraftStore implemented; CLI plumbing pending.)
- [x] 2.3 Track draft status transitions.
- [x] 2.4 Add tests for prompt input byte limits, sample truncation, and persisted metadata.

## 3. Resolution Actions

- [x] 3.1 Implement draft generation without applying output.
- [x] 3.2 Implement accept action with contract validation, atomic replacement, rollback on failure, and interface hash recording.
- [x] 3.3 Implement edit action that writes an editable draft file without applying output.
- [x] 3.4 Implement `draft accept <draft-id> --edited` with the same validation, atomic replacement, and rollback guarantees as generated draft accept.
- [x] 3.5 Implement discard action that preserves unresolved contradictions.
- [x] 3.6 Add tests for every action.

## 4. CLI and Documentation

- [x] 4.1 Implement `summarise`, `drafts`, `draft show`, `draft accept`, `draft accept --edited`, `draft edit`, and `draft discard`.
- [x] 4.2 Add JSON schemas and output snapshots.
- [x] 4.3 Register summariser and draft commands in the shared MCP query tool registry with correct read-only versus mutating safety classes.
- [x] 4.4 Document backend configuration, safety model, MCP exposure, and resolution actions.

## 5. Required Verification

- [x] 5.1 `cargo build` passes with zero warnings.
- [x] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 5.3 `cargo fmt --check` passes.
- [x] 5.4 `cargo test` passes.
- [x] 5.5 `cargo test --locked` passes.
- [x] 5.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-8-summariser --strict` passes.
