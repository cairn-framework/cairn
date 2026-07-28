---
id: dec.native-todos-first
nodes:
  - cairn.root
status: superseded
date: 2026-07-03
informed_by:
  - res.task-front-door
related:
  - dec.beads-task-layer
  - dec.bd-upgrade-plan
  - dec.change-format-only
  - dec.native-task-state-and-agent-guidance
---
# Native Todos First

## Context

`res.task-front-door` found a dogfooding split: cairn ships a first-class
Todo artefact type (`docs/spec.md` §8.2) that `cairn init` already points a
fresh user at, but this repository's own development tracked work in beads
(`bd`) instead. `AGENTS.md` mandated `bd` for all task tracking while three
accepted decisions (`dec.bead-github-sync`, `dec.beads-task-layer`,
`dec.bd-upgrade-plan`) already agreed beads are an optional, read-only
derived view over the real task-tracking mechanism. `dec.beads-task-layer`
itself named the fix as a numbered revisit trigger: "cairn adopts a
non-beads task tracker, making the beads-derived view the wrong source."
This decision is that trigger firing.

## Decision

Four rulings.

1. **Repo task tracking moves to native Todo artefacts.** This repository's
   own development tracks work in `meta/todos/todo.<slug>.md`, the same
   mechanism a fresh `cairn init` user gets, not beads.
2. **`cairn todo new <slug> --node <id>` ships as artefact scaffolding**,
   under `dec.change-format-only`'s own criterion: validating or applying
   declared state is cairn's job, creating/claiming/sequencing work items is
   workflow and cairn does not do workflow. `cairn todo new` writes a file
   with `node:`/`status: open`/`created:` frontmatter and nothing else; it
   has no claim verb and no close verb. Status changes are plain file edits,
   exactly as a decision's `status:` field is a file edit. This is the same
   reading that already shipped `cairn decision new`.
3. **Beads are demoted to a conditional, read-only integration.**
   `dec.beads-task-layer`'s per-node view (`cairn backlog <node>`,
   `src/state/backlog.rs`) stays accepted and unchanged; only its *primacy*
   in `cairn next` / `cairn brief` changes. When a project has open native
   todos, they are preferred; the beads backlog is consulted only as a
   fallback when `.beads/issues.jsonl` exists and no native todo is open.
   `dec.beads-task-layer`'s revisit trigger 3 is acknowledged as fired by
   this decision; the trigger does not retract the view, it anticipated
   exactly this move.
4. **The `AGENTS.md` bd mandate is replaced.** The "Task tracking" section
   now names native Todo artefacts as this repo's task-tracking mechanism
   and beads as an optional read-only integration other projects may run;
   it no longer instructs contributors to use `bd` for new work in this
   repository.

## Rationale

Dogfooding coherence was the concrete complaint: cairn is most opinionated
about declared, versioned, provenance-linked state, and its own development
was the one place not using that mechanism for tasks. The migration surface
is small (five open beads, one closed as pure hygiene in the same change
cycle; four migrated to todo artefacts by this change) and the new verb is a
near-exact copy of `cairn decision new`, so the implementation risk is low.
`README.md:97` already promised external trackers are optional read-only
views; this decision makes that true of cairn's own repository too, not
just of what it tells other projects.

## Consequences

- `render_next` (`src/cli/render/remediate.rs`) and `render_brief` prefer
  the top open native todo (sorted by `created` then filename) over the
  beads backlog; behaviour is unchanged for projects with no `meta/todos/`
  pointer wired or no open todos.
- The four remaining open beads at the time of this decision
  (`cairn-380`, `cairn-m99`, `cairn-omf`, `cairn-jj4`) are migrated to
  `meta/todos/` artefacts and closed in bd in the same change.
- `dec.beads-task-layer`'s view (`cairn backlog`) is unchanged and remains
  accepted; nothing here removes beads support, it only stops requiring it.
- `AGENTS.md`'s two hand-maintained `bd setup`-generated blocks are deleted
  in favour of a short, accurate task-tracking section, closing
  `dec.native-task-state-and-agent-guidance` ruling 2.
