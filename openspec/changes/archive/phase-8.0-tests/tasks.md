# Tasks: Phase 8.0 Tests (Summariser Pre-Phase)

## 1. Backend and Configuration Tests

- [x] 1.1 Write `#[cflx_planned(phase = 800)]` test: disabled summariser does not generate a draft when an interface contradiction is detected.
- [x] 1.2 Write `#[cflx_planned(phase = 800)]` test: configured backend creates a pending draft under `.cairn/state/summariser/` when `cairn summarise <node>` runs.
- [x] 1.3 Write `#[cflx_planned(phase = 800)]` test: local command backend receives one `SummariserRequest` JSON on stdin and stores only `draft_text` from the `SummariserResponse`.
- [x] 1.4 Write `#[cflx_planned(phase = 800)]` test: backend failure (non-zero exit or timeout) does not create or modify a draft.

## 2. Resolution Action Tests

- [x] 2.1 Write `#[cflx_planned(phase = 800)]` test: `draft accept <draft-id>` replaces the target contract with draft text and records the current interface hash.
- [x] 2.2 Write `#[cflx_planned(phase = 800)]` test: `draft edit <draft-id>` writes an editable file under `.cairn/state/summariser/editable/` without modifying the contract.
- [x] 2.3 Write `#[cflx_planned(phase = 800)]` test: `draft accept <draft-id> --edited` applies the editable draft file content and records the interface hash.
- [x] 2.4 Write `#[cflx_planned(phase = 800)]` test: accepting a draft with invalid frontmatter exits code `1`, restores the original contract, and leaves the draft pending.
- [x] 2.5 Write `#[cflx_planned(phase = 800)]` test: `draft discard <draft-id>` marks the draft discarded and leaves the underlying contradiction unresolved.
- [x] 2.6 Write `#[cflx_planned(phase = 800)]` test: generation never modifies a contract file until a resolution command runs.

## 3. MCP Exposure Tests

- [x] 3.1 Write `#[cflx_planned(phase = 800)]` test: `cairn_drafts` and `cairn_draft_show` are listed as read-only tools when the MCP server starts in default mode.
- [x] 3.2 Write `#[cflx_planned(phase = 800)]` test: `cairn_summarise`, `cairn_draft_accept`, `cairn_draft_edit`, and `cairn_draft_discard` are absent from the tool list when the MCP server starts in default mode.

## 4. Required Verification

- [x] 4.1 `cargo build` passes with zero warnings.
- [x] 4.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 4.3 `cargo fmt --check` passes.
- [x] 4.4 `cargo test` passes (all 12 new tests are skipped as ignored).
- [x] 4.5 `cargo test -- --ignored` shows all 12 new tests as FAILED (confirming bodies are `todo!()`).
- [x] 4.6 `bash scripts/pre-archive-rust-gates.sh` passes.
