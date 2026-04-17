# Tasks: Phase 1 Kernel

## 1. Configuration and DSL Parser

- [x] 1.1 Implement `cairn.config.yaml` loading with defaults for missing config, known Phase 1 fields, and forward-compatible unknown sections.
- [x] 1.2 Implement layered ignore handling from built-in defaults, `.gitignore`, `.cairnignore`, and config while protecting Cairn-owned paths.
- [x] 1.3 Implement tokenization with source spans for DSL keywords, identifiers, tags, strings, arrows, braces, comments, and lists.
- [x] 1.4 Implement AST types for systems, containers, modules, actors, fields, tags, paths, `owns-files`, artefact pointers, and edges.
- [x] 1.5 Implement recursive descent parsing for nested node declarations and top-level edges.
- [x] 1.6 Implement source-positioned parser errors for malformed fixtures.
- [x] 1.7 Add unit tests for config loading, ignore precedence, every grammar production, and negative tests for malformed input.

## 2. Ontology and Integrity

- [x] 2.1 Build node, name, parent, child, inbound edge, outbound edge, and path ownership indexes with leaf-default ownership and internal-node `owns-files: true` opt-in.
- [x] 2.2 Implement ID validation, duplicate ID detection, required field validation, path tie detection, and invalid edge endpoint validation.
- [x] 2.3 Implement dependency cycle detection for `order` and `lint` findings without blocking basic ontology queries, keeping the finding reusable by Phase 4 hooks.
- [x] 2.4 Implement name-or-ID node resolution with deterministic closest-match suggestions on failure.
- [x] 2.5 Add graph and integrity tests using `test/fixtures/cairn.dsl` and purpose-built malformed fixtures, including internal-node files that are orphaned by default and claimed when `owns-files: true` is present.

## 3. Contract Artefacts

- [x] 3.1 Implement Markdown frontmatter parsing for contract files with `node: <id>`.
- [x] 3.2 Validate contract pointers and referenced node IDs.
- [x] 3.3 Implement `cairn contract <node>` over parsed contract data.
- [x] 3.4 Add contract loading, missing pointer, broken pointer, and wrong-node tests.

## 4. Reconciler and Scanner

- [x] 4.1 Define the `Reconciler` trait, request type, report type, finding type, and interface fingerprint type.
- [ ] 4.2 Implement the Rust code reconciler using Tree-sitter for Rust source discovery and public interface fingerprints.
- [x] 4.3 Implement synced, ghost, and orphaned node state assignment.
- [x] 4.4 Implement staged `cairn scan` orchestration: config, DSL parse, preliminary node index, contract load, reconciliation, final ontology, state-aware contract severity, output persistence.
- [x] 4.5 Implement `.cairn/state/interface-hashes.json` persistence.
- [x] 4.6 Generate `index.md` and append `.cairn/log.md` scan events.
- [x] 4.7 Add scanner integration tests using temporary directories and Rust source fixtures.

## 5. CLI Queries

- [x] 5.1 Implement shared typed query/service request and response structs in the library.
- [x] 5.2 Implement a command registry with command name, request/response type identity, and `read_only` or `mutating` safety class.
- [x] 5.3 Implement `get`, `neighbourhood`, `files`, `dependents`, `depends`, `order`, `lint`, and `scan` as CLI wrappers over the shared library services.
- [x] 5.3a Implement `cairn init` for greenfield project scaffolding: create a starter `cairn.dsl` skeleton, `cairn.config.yaml` with sensible defaults, `meta/` directory structure, and `.cairn/` machine-state directory.
- [x] 5.4 Add `--file` and `--json` support to every command.
- [x] 5.5 Render JSON from the shared response structs and human-readable output from those same structs.
- [x] 5.6 Add CLI integration tests for success and failure paths, plus tests proving `scan` is registered as `mutating` and Phase 1 query commands are registered as `read_only`.

## 6. Documentation

- [x] 6.1 Update README with Phase 1 command examples.
- [x] 6.2 Document contract-only artefact support and Phase 2 artefact exclusions.
- [x] 6.3 Document scan output files and their generated status.

## 7. Required Verification

- [x] 7.1 `cargo build` passes with zero warnings.
- [x] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 7.3 `cargo fmt --check` passes.
- [x] 7.4 `cargo test` passes.
- [x] 7.5 `cargo test --locked` passes.
- [x] 7.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-1-kernel --strict` passes.

## Implementation Blocker #1

RESOLVED 2026-04-17: codex sandbox `network_access = true` enabled in `~/.codex/config.toml [sandbox_workspace_write]`. `cargo add tree-sitter tree-sitter-rust` now works inside the worktree. Re-queue phase-1 to complete task 4.2 with real Tree-sitter integration.
