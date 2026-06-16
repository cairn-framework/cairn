# Cairn Atomic Improvement Loop

## Cadence Definition

The **Atomic Improvement Loop** is a repeatable, triggerable process for making small, self-contained improvements to the cairn codebase:

- One atomic improvement per iteration (test coverage, docs, refactor, minor fix).
- Each iteration: re-orient from `PROGRESS.md`, implement, validate via `scripts/pre-archive-rust-gates.sh` and `cairn lint`, create/close a bead, commit, update `PROGRESS.md`.
- Stop when the session budget is reached or marginal value drops.

To trigger: run `autoresearch.sh` or re-create the workflow manually from this file.

## Last Completed Session
- **Commits**: 113 atomic improvements (module splitting, test extraction, unit tests, module docs, clippy cleanups).
- **Tests**: 880 lib tests pass; integration / phase tests pass.
- **Gates**: `scripts/pre-archive-rust-gates.sh` passes; `cairn lint` clean.
- **Merge to main**: all work merged into `main` and pushed; GitHub default branch set to `main`.

## What This Cadence Is Good For
- Mechanical code quality: tests, docs, clippy, module extraction.
- Small, low-risk, independently committable changes.

## What This Cadence Is NOT Good For
- Repo-wide architectural changes.
- Multi-file schema migrations.
- Bugs that require cross-cutting understanding.

## High-Ticket Items: Planned Sessions

### 1. `cairn-xy1` — Fix Degenerate Interface Hashes
All 27 nodes in `.cairn/state/interface-hashes.json` share one hash (`3f881a6cf699b056`). The fingerprint collapses every module's interface into the same value, making `CAIRN_INTERFACE_HASH_CHANGED` unable to identify which node drifted.

Definition of done:
- `cairn scan` produces distinct hashes for modules with distinct public interfaces.
- Identical public interfaces across modules still produce identical hashes.
- `cairn hook all` passes after a clean `cairn scan` baseline.

### 2. `cairn-v1t` — Wire Decisions into Blueprint Provenance Graph
Epic to migrate `meta/decisions/` to the new schema, add a `decisions` pointer to `cairn.blueprint`, and write covering decisions for all leaf nodes so `CAIRN_PROVENANCE_NO_DECISION` does not fire.

Definition of done:
- All decision files use `id/nodes/status/date` frontmatter.
- `cairn.blueprint` declares a decisions pointer.
- Every leaf node has a covering decision.
- `cairn lint` reports no provenance findings.

## Status
- Atomic-loop work and planned sessions can run independently. If `cairn-xy1` or `cairn-v1t` are already done, future Atomic Improvement Loop sessions can still operate on smaller items.
- When no planned session is active, use the Atomic Improvement Loop to keep code quality improving continuously.
