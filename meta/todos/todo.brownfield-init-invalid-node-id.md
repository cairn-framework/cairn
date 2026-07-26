---
node: cairn.brownfield
status: done
created: 2026-07-25
---

# Brownfield Init Emits Invalid Node Ids

## Priority

P2 defect. It blocks the advertised first-run path on a real, popular target, so
it costs more at the cold start than its size suggests.

## Problem

`cairn init --from-code --apply` derives node ids straight from directory names
without sanitising them, then fails its own integrity check on the result. On
pallets/flask at `7fff56f5` the discovery step proposes
`examples.celery.src.task_app`, `tests.test_apps.cliapp`, and `tests.type_check`,
and applying the change reports:

```
CAIRN_INTEGRITY_INVALID_ID: node id `tests.type_check` must be a lowercase
dotted identifier (a-z, 0-9, `.`, `-`; underscores are not allowed)
```

Discovery reports success, the apply fails, and the repository is left with a
seed `cairn.blueprint` carrying no modules. Reproduced on both `cairn` main
(`4390e23`) and `24a328f`. Any target with an underscore in a source directory
name hits this, which on Python projects is common.

## Scope

- Sanitise derived node ids in the brownfield deriver so a discovered directory
  name that is legal on disk always produces a legal node id. Underscore to
  hyphen matches the existing id grammar and the ids already in this repository.
- Preserve the human-readable module name and the `path` verbatim; only the id
  is constrained.
- Handle the collision that sanitisation can create (`type_check` and
  `type-check` in sibling directories) deterministically rather than emitting a
  duplicate id.
- Decide whether the orphaned-file findings that accompany this failure
  (`docs/conf.py`, `examples/javascript/**`) should be an apply-blocking error at
  all, or whether `cairn onboard` is the right place to resolve them. Do not
  widen this todo into that change without a separate ruling.

## Acceptance

- `cairn init --from-code --apply` on pallets/flask at `7fff56f5` completes and
  leaves a blueprint containing the discovered modules.
- A regression test covers a discovered directory name containing an underscore
  and a sanitisation collision between two sibling directories.
- No existing node id in this repository or in
  `tests/fixtures/cairn-bootstrap/` changes.

Found while building the pinned flask fixture for
`todo.agent-context-bundle-evaluation`; the workaround used there is recorded in
the 2026-07-25 entry of `res.loop-efficiency-observations`.
