---
node: cairn.kernel.cli
status: done
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

## Resolution (2026-07-16)

Shipped in `src/cli/render/remediate.rs`: `cairn brief todo.<slug>` now
resolves the named native todo by file name (`todo.<slug>.md`) and fuses the
same node, decision, contract, and gate context via the existing
`render_brief_data` path. Precedence is deterministic: an argument starting
with `todo.` targets a native todo (an unknown slug errors, never falling
through to beads); any other argument resolves as a bead id. A named todo
briefs in any status and carries a not-ready warning when it is not open.
Unknown-slug and not-ready copy lives in `docs/design-system/copy.toml`
under `[brief]`; help text and `docs/commands.md` document the rule. Bare
node ids were not added: they did not fall out of the slug path naturally.
