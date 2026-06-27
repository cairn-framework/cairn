---
node: cairn.macros
---

# Contract: cairn.macros

## Purpose

A standalone procedural-macro crate (`cairn-macros`) supporting the test-first
pre-phase convention. It exposes a single attribute macro, `cairn_planned`, that
marks an integration test as written against acceptance criteria for a feature
phase that has not yet shipped. The macro neutralises such tests so `cargo test`
stays green until the phase lands, while recording them in a sidecar registry.

## Public interface

- `cairn_planned`: `#[proc_macro_attribute]` applied as
  `#[cairn_planned(phase = <N>)]` on a test function. It parses the `phase`
  named argument, validates it, registers the test, and re-emits the original
  function annotated with an `#[ignore]` attribute.

## Invariants

- The single accepted argument is `phase = <integer>`; any other path yields the
  compile error "unsupported argument; expected `phase = <positive_integer>`".
- `phase` must be a positive integer: a non-integer literal, a value below 1, or
  a value that overflows `u32` is a compile error. A missing `phase` argument is
  also rejected.
- Combining `#[cairn_planned]` with a manual `#[ignore]` is a compile error; only
  one ignore mechanism is permitted.
- The expansion is `#[ignore = "cairn_planned: phase-<N>"]` prepended to the
  unmodified input function, so the test compiles but is skipped by default.
- Registration writes the test name and phase to the `target/cairn/planned.json`
  sidecar; this write is best-effort and its failure never aborts compilation.

## Dependencies

Leaf node with no outgoing blueprint edges. It is an independent crate built on
the `proc-macro2`, `quote`, and `syn` macro toolchain. Tagged `@build` (it is a
build/tooling component) and `@no-test-coverage` (excluded from the coverage
gate; its own behaviour is exercised by the fixture under its `tests/`).

## Tests

The crate ships a fixture test at `cairn-macros/tests/planned_attribute.rs` that
applies `#[cairn_planned(phase = 8)]` to a function and asserts the macro
expands successfully and marks the function as ignored, proving the
`#[ignore]`-injection path compiles.
