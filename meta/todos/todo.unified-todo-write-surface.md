---
node: cairn.kernel.cli
status: open
created: 2026-07-11
related: [dec.native-todos-first, dec.bead-github-sync, todo.github-todo-sync]
---

# Unified todo write surface: one command to create, edit, and set status

## Problem

Today the todo lifecycle is split across two interfaces: create and read go
through the CLI (`cairn todo new`, `cairn todos`, `cairn status`, `cairn next`),
but every mutation after creation (filling the body, flipping
`open -> in_progress -> done -> blocked`) is a hand edit of the markdown
frontmatter. The owner's objection (2026-07-11): mutation should live on the
same surface as creation and reads, so a single call is the one place that knows
how to update a todo. That is the right seam for the future: if a server or DB is
wired, or a GitHub/beads projection is added, one command updates the file and
notifies the backend, instead of every writer re-implementing the frontmatter
format. This is the same pattern the change path already uses: with
`config.state_backend=='beads'`, `cairn change new` writes `tasks.md` and mints
beads from it in one write.

## Blocker: this revisits a ratified decision

`dec.native-todos-first` ruling 2 deliberately shipped `cairn todo new` with
**no claim verb and no close verb**, on the principle that "validating or
applying declared state is cairn's job, creating/claiming/sequencing work items
is workflow and **cairn does not do workflow**." A `cairn todo set` verb reverses
that ruling. This todo therefore requires a **decision artefact first**
(`cairn gap` or `cairn decision new`) that the owner ratifies, re-drawing the
line between "cairn does not do workflow" and "cairn owns the one write surface
for its own declared state." Do not implement before that decision is accepted.

## Proposed direction (subject to the decision)

- `cairn todo set <slug> <status>` validates the status enum and rewrites only
  the frontmatter field, leaving the body untouched. Files stay canonical; git
  stays the history engine (honours files-are-truth).
- Consider folding body editing (`cairn todo edit <slug>` opens `$EDITOR`, mirror
  of `cairn draft_edit`) so create/edit/status/read are one family.
- Design the verb as the backend seam: it is the single point a future
  StateBackend (server, DB, or the GitHub projection in `todo.github-todo-sync`)
  hooks, so "update the file" and "update the backend" happen atomically behind
  one call. Default remains file-only so state travels with the repo on fork.
- The `CAIRN_TODO_STATUS_INVALID` scan finding
  (`src/artefacts/registry/parse.rs`) already guards manual edits; the verb makes
  the guarantee write-time instead of next-scan.

## Relationship to GitHub sync

`dec.bead-github-sync` already ratifies GitHub as a one-way read-only projection,
and `todo.github-todo-sync` specifies the mirror. The owner's goal ("if tasks
sync to GitHub or another manager, we stop opening PRs just to update task
state") is that todo's payoff. This write-surface verb is the seam that makes the
projection clean: mutations flow through one call the projector can observe. Land
the decision and this verb, then `todo.github-todo-sync` becomes the first
consumer.

## Acceptance

- A decision artefact revisiting `dec.native-todos-first` ruling 2 is written and
  ratified by the owner before any code.
- If ratified: `cairn todo set <slug> <status>` is added per
  docs/skills/cairn-add-cli-command (dispatch, `--json`, command_reference tests),
  rewrites only the status field, and rejects invalid status with a clear error.
  Body edit verb added or explicitly deferred.
- The write path is structured so a backend projection can hook it without every
  caller re-parsing frontmatter; file-only remains the default.
- All Rust gates pass; wire-format snapshots unchanged.
