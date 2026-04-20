# Tasks: Phase 5 Edge Validation and Docstrings

## 1. Semantic Dependency Extraction

- [x] 1.1 Extend the Rust Tree-sitter reconciler to extract `use` paths and `mod` declarations.
- [x] 1.2 Map observed dependencies to owning Cairn nodes using Phase 1 path ownership indexes.
- [x] 1.3 Represent dependency observations with confidence and source spans.
- [x] 1.4 Add tests for direct, missing, extra, and ambiguous dependency observations.

## 2. Edge Divergence

- [x] 2.1 Compare declared blueprint edges against observed dependencies.
- [x] 2.2 Report missing observed dependencies as rationale tensions.
- [x] 2.3 Report observed dependencies without declared edges as rationale tensions.
- [x] 2.4 Include edge divergence in `lint`, `scan`, `map.md`, and hook reports.

## 3. Docstring Drift

- [x] 3.1 Extract module-level docstrings from Rust `//!` comments in `lib.rs`, `main.rs`, and `mod.rs`, plus `///` comments attached to `mod` declarations.
- [x] 3.2 Parse exact, case-sensitive Cairn fact lines for ID, name, dependencies, tags, and contract references.
- [x] 3.3 Compare extracted facts to map facts.
- [x] 3.4 Report contradictions and unknown IDs as rationale tensions, and unknown fact keys as informational findings.

## 4. Docstring Command

- [x] 4.1 Implement `cairn docstring <node> [--language <lang>]`.
- [x] 4.2 Emit templates for Rust, Python, TypeScript, and Go.
- [x] 4.3 Add JSON output containing template facts and rendered text.
- [x] 4.4 Add output snapshot tests for every language.

## 5. Documentation

- [x] 5.1 Document edge divergence as advisory rationale tension.
- [x] 5.2 Document docstring fact line syntax and supported languages.
- [x] 5.3 Document that generated templates require human or agent prose completion.

## 6. Required Verification

- [x] 6.1 `cargo build` passes with zero warnings.
- [x] 6.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 6.3 `cargo fmt --check` passes.
- [x] 6.4 `cargo test` passes.
- [x] 6.5 `cargo test --locked` passes.
- [x] 6.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-5-edges-docstrings --strict` passes.
