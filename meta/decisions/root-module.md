---
id: dec.root-module
nodes:
  - cairn.root
status: accepted
date: 2026-06-16
---

# Root module

## Context

cairn needs a single entry point that ties together the library, binary targets, shared error types, and verification helpers.

## Decision

Keep a `cairn.root` module that claims `src/main.rs`, `src/lib.rs`, `src/error.rs`, `src/verification.rs`, `src/bin/cairn-lsp.rs`, and `src/signal.rs`.

## Rationale

Without this module, these files would be orphaned on every scan. Grouping them under one node reflects that they are the crate boundary, not domain logic.

## Consequences

- New top-level source files should either join `cairn.root` or spawn a new top-level module with its own decision.
- Changes to `src/lib.rs` public API are high-impact and should be gated carefully.
