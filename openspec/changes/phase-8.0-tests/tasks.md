# Tasks: Phase 8.0 Tests (Summariser Pre-Phase)

## 1. Backend and Configuration Tests

- [ ] 1.1 Write `#[ignore = "awaits phase-8"]` test: disabled summariser does not generate a draft when an interface contradiction is detected.
- [ ] 1.2 Write `#[ignore = "awaits phase-8"]` test: configured backend creates a pending draft under `.cairn/state/summariser/` when `cairn summarise <node>` runs.
- [ ] 1.3 Write `#[ignore = "awaits phase-8"]` test: local command backend receives one `SummariserRequest` JSON on stdin and stores only `draft_text` from the `SummariserResponse`.
- [ ] 1.4 Write `#[ignore = "awaits phase-8"]` test: backend failure (non-zero exit or timeout) does not create or modify a draft.

## 2. Resolution Action Tests

- [ ] 2.1 Write `#[ignore = "awaits phase-8"]` test: `draft accept <draft-id>` replaces the target contract with draft text and records the current interface hash.
- [ ] 2.2 Write `#[ignore = "awaits phase-8"]` test: `draft edit <draft-id>` writes an editable file under `.cairn/state/summariser/editable/` without modifying the contract.
- [ ] 2.3 Write `#[ignore = "awaits phase-8"]` test: `draft accept <draft-id> --edited` applies the editable draft file content and records the interface hash.
- [ ] 2.4 Write `#[ignore = "awaits phase-8"]` test: accepting a draft with invalid frontmatter exits code `1`, restores the original contract, and leaves the draft pending.
- [ ] 2.5 Write `#[ignore = "awaits phase-8"]` test: `draft discard <draft-id>` marks the draft discarded and leaves the underlying contradiction unresolved.
- [ ] 2.6 Write `#[ignore = "awaits phase-8"]` test: generation never modifies a contract file until a resolution command runs.

## 3. MCP Exposure Tests

- [ ] 3.1 Write `#[ignore = "awaits phase-8"]` test: `cairn_drafts` and `cairn_draft_show` are listed as read-only tools when the MCP server starts in default mode.
- [ ] 3.2 Write `#[ignore = "awaits phase-8"]` test: `cairn_summarise`, `cairn_draft_accept`, `cairn_draft_edit`, and `cairn_draft_discard` are absent from the tool list when the MCP server starts in default mode.

## 4. Required Verification

- [ ] 4.1 `cargo build` passes with zero warnings.
- [ ] 4.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 4.3 `cargo fmt --check` passes.
- [ ] 4.4 `cargo test` passes (all 12 new tests are skipped as ignored).
- [ ] 4.5 `cargo test -- --ignored` shows all 12 new tests as FAILED (confirming bodies are `todo!()`).
- [ ] 4.6 `bash scripts/pre-archive-rust-gates.sh` passes.
