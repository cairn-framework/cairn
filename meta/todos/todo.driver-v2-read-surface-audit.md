---
node: cairn.kernel.query
status: open
created: 2026-08-10
related: [res.inversion-convergence-minutes, todo.build-ci-observation-overlay]
---

# Driver V2 Read Surface Audit

Created so `todo.build-ci-observation-overlay` can name the surviving
driver-v2 prerequisite as a typed `blocked_by` edge. `blocked_by` targets a todo
stem, and a change is not referenceable (`dec.todo-relationship-model`), so the
horizon clause on that todo had no way to become machine-visible without this
stem.

## Task

Complete tasks 1 and 2 in `meta/changes/driver-v2-selection/tasks.md`. That file
stays authoritative for the task text, the pinned-commit requirement, and the
artefact path; this todo deliberately does not restate it.

## Acceptance

- Tasks 1 and 2 in `meta/changes/driver-v2-selection/tasks.md` are checked.
