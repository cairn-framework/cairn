---
node: cairn.ui
status: open
created: 2026-08-11
---

# Fix the webui harness ancestor-opacity unit tests failing in CI

Discovered by todo.context-pass-pack-loop (2026-08-11): the CI `webui` job's
"harness unit tests" step fails on main itself (commit 48a59e1a) and on every
recent branch, so the failure predates and is independent of the pack context
passes. Signature: the four "DOM pass" tests (composites ancestor opacity,
leaves an unfaded ancestor chain alone, composites a translucent card over
its backdrop, skips text faded to nothing) all throw `TypeError: Cannot read
properties of undefined (reading 'contains')`, which points at an environment
or DOM-stub drift in the harness, not at a contrast-logic regression.

## Task

1. Reproduce locally with the webui harness unit-test command the CI job
   runs, pinned to CI's Node version.
2. Find what `contains` is read from and why it is now undefined (jsdom or
   Node upgrade, stub shape change), and fix the harness or the stub at the
   source.

## Acceptance

- The four "DOM pass" tests pass in the CI `webui` job on the first push
  after the fix.
