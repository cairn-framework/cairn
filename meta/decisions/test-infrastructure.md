---
id: dec.test-infrastructure
nodes:
  - cairn.tests
status: accepted
date: 2026-06-16
---

# Test infrastructure

## Context

Integration tests, phase tests, and smoke tests live outside `src/` so they exercise the public crate surface the way external callers would.

## Decision

Maintain a dedicated `cairn.tests` module that points at the `tests/` directory. This module is tagged `@test` and is not part of the production dependency graph.

## Rationale

Separating tests from `src/` keeps the crate's internal modules focused on production code while still letting cairn model the test suite as a node. The `@test` tag lets gates and queries filter it out of build-order or dependency analysis when appropriate.

## Consequences

- New phase tests go into `tests/` and are covered by this decision.
- The module should not claim source files under `src/`.
- CI runs `cargo test --all-targets --all-features`.
