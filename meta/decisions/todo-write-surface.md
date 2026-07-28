---
id: dec.todo-write-surface
nodes:
  - cairn.kernel.cli
  - cairn.root
status: superseded
date: 2026-07-13
informed_by:
  - res.loop-efficiency-observations
refines:
  - dec.native-todos-first
related:
  - dec.github-todo-sync-projector
  - dec.change-format-only
---
# Todo Write Surface: cairn owns mutation of its own declared state

## Context

`dec.native-todos-first` ruling 2 shipped `cairn todo new` with no claim verb
and no close verb, on the principle that "validating or applying declared state
is cairn's job, creating/claiming/sequencing work items is workflow and cairn
does not do workflow." Status changes were left as plain file edits, "exactly
as a decision's `status:` field is a file edit."

Three forces have since pushed against that line:

1. **Owner objection (2026-07-11, recorded in
   `todo.unified-todo-write-surface`):** mutation should live on the same
   surface as creation and reads, so a single call is the one place that knows
   how to update a todo.
2. **Observed failure mode (`res.loop-efficiency-observations`, 2026-07-12
   entry):** closing one unit required coordinated manual edits across three
   todo files, and a reviewer caught a genuine lifecycle contradiction
   introduced during those edits. Hand-editing frontmatter is where declared
   state actually corrupts today; `CAIRN_TODO_STATUS_INVALID` only catches it
   at the next scan, after the bad state is written.
3. **The ratified projection seam (`dec.github-todo-sync-projector`):** GitHub
   issues are a derived view of `meta/todos/*.md`. A projector can only
   observe mutations cleanly if mutations flow through one call; a hand edit
   gives the seam nothing to hook.

The owner delegated ratification of this revisit on 2026-07-13.

## Decision

Two rulings.

1. **The workflow line is re-drawn from "no mutation verbs" to "no
   coordination verbs."** What cairn does not do is coordination: deciding who
   does what and in what order (claiming, assigning, sequencing,
   prioritising). Mutating the declared state of cairn's own artefacts, with
   validation, is state stewardship, the same category as the scaffolding
   verbs (`cairn todo new`, `cairn decision new`) that ruling 2 already
   sanctioned. Ruling 2's operative principle survives intact; only its "no
   close verb" application is revised.
2. **`cairn todo set <slug> <status>` is sanctioned** per the direction in
   `todo.unified-todo-write-surface`: validate the status enum
   (`open|in_progress|done|blocked`), rewrite only the frontmatter `status`
   field, leave the body untouched. Files stay canonical; git stays the
   history engine. The verb is designed as the backend seam: the single point
   a future StateBackend or the GitHub projector hooks, with file-only write
   as the default. A body-edit verb (`cairn todo edit`, mirroring
   `draft edit`) may ship in the same family or be explicitly deferred at
   implementation time.

## Rationale

The alternative (keep hand edits, rely on scan-time validation) was rejected
on evidence, not taste: the one observed session of multi-file status editing
produced a real lifecycle contradiction that needed a reviewer to catch.
Write-time validation moves the guarantee from "detected at next scan" to
"cannot be written," which is strictly stronger and costs one small verb whose
implementation mirrors existing scaffolding code.

The "cairn does not do workflow" principle is preserved rather than
overturned: `todo set` records a status the caller has already decided, it
does not choose, assign, or order work. A claim verb (asserting exclusive
ownership against other agents) would still cross the line and remains out of
scope.

Making this the projector seam also keeps `dec.github-todo-sync-projector`
honest: one write path the sync can observe, instead of every writer
re-implementing frontmatter parsing.

## Consequences

- `todo.unified-todo-write-surface` is unblocked; its acceptance criteria
  (dispatch, `--json`, command_reference tests, invalid-status rejection,
  wire-format snapshots unchanged) govern the implementation.
- Hand edits of todo frontmatter remain legal (files are truth) but become
  the discouraged path once the verb ships; `CAIRN_TODO_STATUS_INVALID`
  stays as the backstop for them.
- `dec.native-todos-first` rulings 1, 3, and 4 are untouched.

revisit_triggers:
  - a claim or assignment verb is proposed (coordination, out of scope here)
  - the StateBackend or GitHub projector needs write hooks the single verb
    cannot express
