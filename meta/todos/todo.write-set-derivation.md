---
node: cairn.kernel.query
status: open
created: 2026-08-07
blocked_by: [todo.parallel-dispatch-granularity]
related: [dec.rung-three-coordination-substrate, res.parallel-dispatch-rung-3]
---

# Write-set derivation and the disjointness test

Implements `res.parallel-dispatch-rung-3` Part 3 and
`dec.rung-three-coordination-substrate` clause 3.

## Task

1. Derive a unit's write-set as the containment closure of `Todo.node` (the node
   plus its descendants via `NodeRecord.children`), mapped to file prefixes
   through `Node.paths`, with more-specific outside owners subtracted.
   Dependency edges are excluded: they are rung 1 Order.
2. Extract one `pub(crate)` component-boundary path-overlap helper from the
   existing private check at `src/reconcile/generic.rs:410`, used by both
   `most_specific_owner` and the disjointness test, so path containment has a
   single implementation. Prefixes are stored without a trailing slash: the
   existing check reads `file.as_bytes().get(path.len()) == Some(&b'/')`, so a
   stored `docs/registries/` inspects the first byte of the filename and fails.
   Make `trim_dot` `pub(crate)` alongside it.
3. Fail closed and visibly: an unresolvable anchor, an `owns_files: true` node
   with no declared paths, or a graph snapshot off HEAD yields the universal
   prefix `.` with `resolution: "unresolved"` and an `unresolved_reason`. The
   unit dispatches alone rather than vanishing from the preview.
4. Stamp every derived write-set `completeness: "partial"` with a
   `completeness_reason` naming the uncovered hotspot prefixes.
5. `cairn wave` renders the preview and its plan digest; `cairn wave stats`
   renders the false-overlap rate as a reader-side projection over
   `outcome.touched_files` facts, with no mutable counter. The promotion
   threshold stays unset until the first twenty exclusions carry merge evidence.

## Acceptance

- A test asserts `src/ui` does not overlap `src/ui_assets`.
- A test asserts a trailing-slash prefix would have produced a false negative
  and that the normalisation prevents it.
- A test asserts an unresolvable anchor yields `.` and appears in the preview
  with its reason, rather than being dropped.
