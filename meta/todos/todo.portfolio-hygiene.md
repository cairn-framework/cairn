---
node: cairn.root
status: done
created: 2026-07-31
---

# Portfolio Hygiene Against The Accepted Mission

## Problem

`dec.cairn-mission` (accepted 2026-07-30, PR #540) postdates nearly the whole
backlog. Measured 2026-07-31 on the merged tiers tree: 32 live todos (25
open, 2 in_progress, 5 blocked), most authored against earlier framings of
what cairn is for. Every live todo is read by loop selection each iteration
and by every "what next" triage, so a stale or mission-orphaned entry costs
attention on every pass. Separately, ownership of the README overhaul is
unsettled: no single open todo owns the outward README story, and
copy-adjacent todos touch it only in passing.

## Scope

One sweep, driver-scheduled (scheduling stays with the driver as a
batch-construction closure line; maintainer approval in principle recorded in
the 2026-07-30 handoff, authoring approved in-session 2026-07-30):

- For each live todo at sweep time, record one disposition against
  `dec.cairn-mission` in the todo body: keep (name the mission property it
  serves), amend (rescope the body to the mission, same file), or close
  (status change through the sanctioned verb with a dated body note naming
  the mission ground). No todo is left unswept and none is deleted.
- Settle README-overhaul ownership: exactly one todo owns it afterwards,
  either an existing copy todo amended to claim it or a new todo authored
  with `cairn todo new`, and the others cross-reference instead of
  duplicating.
- Status changes go through `cairn todo set <slug> <status>`
  (`dec.todo-write-surface`); body edits are ordinary file edits.

## Out of scope

Executing any swept todo's own work; rewriting accepted decisions; changing
loop selection rules (owned by `todo.lint-selection-folding` and its
decisions).

## Acceptance

- Every todo live at sweep time carries a dated mission-disposition note or a
  sanctioned status change; a grep for the disposition marker across
  meta/todos matches the live count recorded in the sweep note.
- Exactly one todo claims README-overhaul ownership by name.
- `cairn scan --strict` stays green through the sweep.

## Origin

Maintainer conversation, 2026-07-30: ranked list items 3 and 4 (hygiene todo
approved in principle in the session handoff, authoring approved in-session).

## Sweep record

2026-08-02: Swept 42 live todos (31 open, 2 in_progress, 9 blocked at inventory); 34 keep dispositions include autodocs-head-to-head and portfolio-hygiene, three unbuilt closes were herdr-cairn-tool-attribution, incremental-scan-hash-diff, and topological-summary-order, and five blocked dispositions include webui-design-quality, with one re-block applied.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. Campaign step 2b, this sweep itself; status set done by its own landing per the approved campaign spec, not a mission close.
