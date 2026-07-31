---
node: cairn.kernel.scanner
status: done
created: 2026-07-29
---

# Ratify the parked and deferred composition rule

`dec.parked-deferral-composition` (proposed) defines the intersection
`todo.lint-selection-folding` item 1a left open: an Info finding that is both
decision-deferred and referenced by a `blocked` todo's `defers:` list is not
re-classified as parked; deferral wins. The implementation already ships that
conservative reading, pinned by
`test_todo_defers_deferred_finding_stays_deferred_not_parked`.

blocked on: `dec.parked-deferral-composition` reaching `status: accepted`
(maintainer ratification; refines the semantics of a ratified, adopter-inherited
schema field, so a loop iteration may not self-ratify it).

## Scope

- Accepted as written: set this todo `done`; no code change.
- Ruled otherwise (dual classification): remove the `deferred_by` guard in
  `src/scanner/todo_defers.rs`, flip the pinned test, and author the rendering
  rule for the collapse conflict dual classification reopens (the acceptance's
  full-line rule and the deferred collapse cannot both hold unmodified).

## Depends on

- `dec.parked-deferral-composition` accepted or superseded.
