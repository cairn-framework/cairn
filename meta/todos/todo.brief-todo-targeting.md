---
node: cairn.kernel.cli
status: open
created: 2026-07-13
related: [dec.cairn-brief-orientation, dec.native-todos-first]
---

# cairn brief cannot target native todos

Owner field report (2026-07-13, downstream TypeScript repo): `cairn brief
<arg>` resolves the argument as a bead id only (`resolve_brief_bead`,
`src/cli/commands/brief.rs`). Native todos have no ids, so a repo tracking
work per dec.native-todos-first cannot name a specific todo unit for
orientation; the argument parser confuses node identifiers with bead ids.

## Task

Accept todo slugs (and possibly node ids) as brief targets alongside bead
ids: resolve `todo.<slug>` against `meta/todos/`, fuse the same decision,
contract, and gate context the bead path gets. Define precedence when a slug
and bead id collide. Align with the native-todo brief path that already
exists for the no-argument case.

## Relations

Read-side targeting only. Listing is owned by todo.todo-listing; mutation
surface by todo.unified-todo-write-surface.
