# Tasks: Phase 9 Brownfield Extraction

## 1. Candidate Extraction

- [ ] 1.1 Add repository-wide discovery mode that extracts source roots and module-like directories without requiring an existing `cairn.dsl`.
- [ ] 1.2 Cluster files by path ownership and observed dependency density.
- [ ] 1.3 Produce candidate nodes, paths, edges, confidence scores, and evidence paths.
- [ ] 1.4 Implement deterministic fallback heuristics for minimum file counts, max depth, coupling score bands, observed edge thresholds, and sample byte limits.
- [ ] 1.5 Add fixture tests for simple, nested, mixed-language, low-confidence, and high-coupling repositories.

## 2. Summariser Integration

- [ ] 2.1 Build bounded summariser inputs from candidates and code samples with at most five files per candidate and 4,000 bytes per file.
- [ ] 2.2 Generate names, descriptions, tags, and stub contract prose.
- [ ] 2.3 Implement disabled-mode fallback using mechanical path-derived names.
- [ ] 2.4 Add deterministic fake-backend tests.

## 3. Init from Code

- [ ] 3.1 Implement `cairn init --from-code`.
- [ ] 3.2 Generate `meta/changes/brownfield-init/proposal.md`.
- [ ] 3.3 Generate `dsl.delta` with added nodes and edges.
- [ ] 3.4 Generate stub contracts in the change directory.
- [ ] 3.5 Fail safely when the target change exists unless `--force` is provided.

## 4. Refine

- [ ] 4.1 Implement `cairn refine` against an existing DSL.
- [ ] 4.2 Generate delta operations instead of a full DSL replacement.
- [ ] 4.3 Detect likely renames from path and similarity evidence.
- [ ] 4.4 Add tests for additions, removals, modifications, and renames.

## 5. Documentation

- [ ] 5.1 Document human review workflow for generated brownfield changes.
- [ ] 5.2 Document confidence scores and evidence paths.
- [ ] 5.3 Register `init --from-code` and `refine` in the shared MCP query tool registry as mutating tools.
- [ ] 5.4 Document limitations of architecture inference.

## 6. Required Verification

- [ ] 6.1 `cargo build` passes with zero warnings.
- [ ] 6.2 `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` passes.
- [ ] 6.3 `cargo fmt --check` passes.
- [ ] 6.4 `cargo test` passes.
- [ ] 6.5 `cargo test --locked` passes.
- [ ] 6.6 `python3 /Users/george/repos/cairn/.agents/skills/cflx-proposal/scripts/cflx.py validate phase-9-brownfield --strict` passes.
