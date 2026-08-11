---
node: cairn.ui
status: done
created: 2026-08-11
---

# Fix the webui harness ancestor-opacity unit tests failing in CI

Discovered by todo.context-pass-pack-loop (2026-08-11): the CI `webui` job's
"harness unit tests" step failed on main itself (commit 48a59e1a) and on every
recent branch, so the failure predated and was independent of the pack context
passes.

## Root cause and fix

All six "DOM pass" tests threw `TypeError: Cannot read properties of
undefined (reading 'contains')`. The `nodeStateNamed` check added by #692
(594cbb8e) called `mod.classList.contains(...)` at `harness/lib/audit.mjs:394`,
but the unit-test DOM double (`El` in `harness/lib/audit.test.mjs`) exposes
only `className`/`classSet`, matching the file's own `sig()` convention.

Fixed in the same PR that filed this todo (the CI-green gate required it):
`audit.mjs` now parses `className` tokens (behaviour-identical in the browser,
where `.node-module` is an HTML button), and the double gained a `classList`
getter backed by `classSet` so future `classList` readers in `auditPage` do
not re-arm the trap.

## Acceptance

- `node --test harness/lib/*.test.mjs` passes 10/10 locally and the `webui`
  job passes in CI. Met: webui green on the fixing push.
