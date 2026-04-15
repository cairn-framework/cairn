# Phase 0 Debate Round 1: Attack

## Summary

The Phase 0 change is implementable and appropriately scoped, but several details still leave room for a headless Codex agent to make inconsistent choices.

## Critiques

1. The Rust edition fallback is ambiguous. The design says to use edition 2024 unless unsupported, then use 2021 and record the reason. A headless agent may not know where to record the reason, and different agents may choose different editions based on local toolchains.

2. The Conflux `pre_archive` language still depends on local tool discovery. The design requires `scripts/pre-archive-rust-gates.sh`, which is good, but task 3.2 asks the implementer to wire repository-local Conflux configuration only if the tool exposes a machine-readable hook location. That conditional can lead to a half-complete result with no concrete documentation location.

3. The fixture smoke assertion is slightly vague. "Contains a top-level Cairn DSL declaration keyword" should name the accepted keywords so tests do not accidentally pass because a word appears in a comment or nested example.

4. The CLI smoke behavior permits either `--version` output or no-argument usage output. That gives implementers unnecessary freedom and can cause inconsistent CLI baselines for later phases.

5. Verification should include `cargo test --locked` or make clear why `cargo test` alone is sufficient despite committing `Cargo.lock`. Otherwise reproducibility is asserted but not exercised.

## Recommended Changes

- Require Rust edition 2024 and make unsupported toolchains fail fast instead of falling back.
- Require `scripts/pre-archive-rust-gates.sh` plus a documented line in README naming it as the Conflux `pre_archive` command; add config wiring only when available.
- Define accepted fixture declaration prefixes as `System`, `Container`, `Module`, and `Actor` at the start of a non-comment line.
- Standardize CLI smoke behavior on `cairn --version`.
- Add `cargo test --locked` to verification, while keeping `cargo test` in the hook if desired.
