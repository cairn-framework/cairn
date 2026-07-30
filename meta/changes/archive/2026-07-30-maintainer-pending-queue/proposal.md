# Proposal: maintainer-pending-queue

Implements `todo.maintainer-pending-queue` (node `cairn.kernel.query`), typed-data v1.

## Motivation

Nothing surfaces what is waiting on the maintainer. Measured 2026-07-29: six
decisions sat at `status: proposed`, blocking at least seven todos, and the
maintainer had been told about two of them. `cairn status` shows todos,
`cairn change list` shows changes, `cairn frontier` shows buildable ghosts; no
surface shows proposed decisions.

## Outcome

The maintainer can run one command, `cairn pending`, and see every decision at
`status: proposed` with its age in days, its nodes, and its ratification tier,
oldest first. The webui shows the same queue read-only. Nothing else is listed,
parsed, or inferred.

## Acceptance boundary

The CLI command's output and exit code, and the webui HTTP response:

- `cairn pending --json` on a fixture holding decisions in `proposed`,
  `accepted`, `superseded`, and `deprecated` states lists every proposed
  decision and no other, oldest first (age descending, ties by id ascending).
- Flipping the fixture decision to `accepted` empties its row on the next run
  with no other edit.
- `GET /api/pending` on the embedded server returns the same payload the CLI
  prints.

## Evidence

- Handler unit tests with an injected "today" pin the age arithmetic, the
  status filter, the sort order, the `binding` default, and the deterministic
  error on an unparseable `date:`.
- Integration tests run the fixture through `cairn pending --json`, flip the
  decision, and rerun.
- The `command_reference_consistency` battery (dispatch, `--json`, help, docs
  row) passes with the new command registered.
- `/api/pending` is asserted in the graph-explorer tests and pinned in the
  wire-format snapshots the way existing webui surfaces are.
- Run on this repository itself: the live proposed decisions appear in the
  queue, oldest first.

## Out of scope (exclusions)

- Ruling-blocked todo rows and unblock-count sorting: they need typed
  decision-to-todo edges (`dec.todo-relationship-model` is not ratified).
- Printing the pending count from the loop's End step: a shipped-pack edit,
  excluded by the todo and by the MISSION for this iteration.
- Parsing a `ratification:` frontmatter field: the field does not exist in the
  artefact schema yet (`todo.decision-ratification-tiers` owns it, node
  `cairn.kernel.artefacts`). Every row renders the documented default
  `binding`; the queue picks the real tier up when that todo lands the field.
- Date-format validation at the artefact parser boundary: `date:` is required
  but format-unvalidated for all decision consumers; changing that is
  `cairn.kernel.artefacts` scope. Here, a proposed decision whose `date:` does
  not parse as `YYYY-MM-DD` fails the query deterministically
  (`CAIRN_PENDING_INVALID_DATE` naming the decision), so every listed row
  always carries a real signed age.
