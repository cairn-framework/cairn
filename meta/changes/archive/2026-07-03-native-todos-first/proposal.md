# Proposal: native-todos-first

## Motivation

Cairn ships a first-class Todo artefact type (`docs/spec.md` §8.2), and
`cairn init` already points every fresh user at it, but this repository's
own development tracked work in beads (`bd`) instead. `AGENTS.md` mandated
`bd` for all task tracking while three accepted decisions
(`dec.bead-github-sync`, `dec.beads-task-layer`, `dec.bd-upgrade-plan`)
already agreed beads should be an optional, read-only derived view. This is
a dogfooding gap: cairn does not use its own most-opinionated mechanism
(declared, versioned, provenance-linked state) for its own task tracking.
See `res.task-front-door` for the full options analysis.

## Scope

- Add `cairn todo new <slug> --node <id>`, an artefact-scaffolding write
  verb symmetric with `cairn decision new`.
- Wire `todos "./meta/todos"` into `cairn.blueprint` so the Todo loader
  picks the directory up.
- Make `cairn next` and `cairn brief` prefer the top open native todo,
  falling back to the beads backlog only when no native todo is open.
- Make `cairn status`'s `next_recommended` agree with `cairn next`.
- Migrate this repository's four remaining open beads
  (`cairn-380`, `cairn-m99`, `cairn-omf`, `cairn-jj4`) to todo artefacts and
  close them in bd.
- Replace `AGENTS.md`'s bd-mandate "Task tracking" section and delete both
  `bd setup`-generated Beads blocks.
- Give `cairn changes` and `cairn show` human (non-`--json`) renders, since
  the init-scaffolded agent guide already tells a fresh user to run
  `cairn changes` and it must not error.

## Out of scope

- Removing beads support. `dec.beads-task-layer`'s read-only per-node view
  (`cairn backlog <node>`) stays accepted and unchanged; only its
  *primacy* in `next`/`brief`/`status` changes.
- Any status/close verb for todos (`dec.change-format-only`'s criterion:
  scaffolding is authoring, not workflow).
- Fixing the pre-existing `cairn status` `active_changes` bug found while
  investigating this change; tracked separately as
  `todo.status-active-changes-bug.md`.
