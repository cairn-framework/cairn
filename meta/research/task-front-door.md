---
id: res.task-front-door
nodes:
  - cairn.root
date: 2026-07-03
method: primary
---

# Task front door: closing the gap between how cairn is built and how it tells users to work

## The question

Cairn ships a first-class Todo artefact type (`docs/spec.md` §8.2, "Todo
(authority)": markdown per node, `node:`/`status:`/`created:` frontmatter,
surfaced per node in the webui Todos panel and via `cairn todos <node>`), but
this repository's own development has never used it: task tracking lives in
beads (`bd`), and `AGENTS.md` mandated `bd` for all task tracking until this
research's companion decision (`dec.native-todos-first`) landed. Meanwhile
`cairn init`'s agent guide (`src/cli/agent_guide.md`) never mentions beads at
all, and `README.md:97` already frames external trackers as optional
read-only views. A fresh user who follows the shipped onboarding gets a
purely native-Todo story; a contributor to cairn itself gets a purely
beads-shaped story. That split is the "dogfooding coherence" gap: cairn does
not use its own shipped mechanism for the thing it is most opinionated about
(declared, versioned, provenance-linked state).

Three options were on the table.

## Option A: native consolidation

Move this repository's own task tracking onto `meta/todos/` artefacts. Add
one write verb, `cairn todo new <slug> --node <id>`, scaffolding a todo file
exactly the way `cairn decision new` scaffolds a decision (format/authoring,
not workflow). `cairn next` and `cairn brief` prefer open native todos, and
fall back to the beads backlog when `.beads/issues.jsonl` exists (so a
project that already runs beads keeps a working `next`/`brief` without any
migration). Beads stay as `dec.beads-task-layer`'s ratified read-only,
per-node derived view (`cairn backlog <node>`); only its *primacy* in
`next`/`brief` changes, not its existence.

Fresh-install UX trace: `cairn init` already never mentions beads → no
change needed there. A fresh user runs `cairn todo new`, `cairn todos`,
`cairn next`; never touches `bd`. This repo's own contributors get the
identical loop. One coherent story, zero required migration for external
users, small migration surface for this repo (the beads backlog was already
small: 5 issues at the time of this research, one closed as pure hygiene in
the same change cycle).

## Option B: StateBackend routing

Give `cairn change new`/`cairn change apply` a `state_backend` config that
seeds and claims beads on the caller's behalf (`create_change_epic`,
`create_task_beads`, `claim_change` on `BeadsStateBackend`,
`src/state/beads.rs`). This was the actual shipped machinery until
2026-07-02: it made cairn schedule and claim work, which
`dec.change-format-only` found already contradicted `dec.no-orchestrator`
("creating, claiming, or sequencing work items is workflow, and cairn does
not do workflow") and had it deleted in full. Option B is dead: reviving it
to solve the task-front-door gap would re-open a decision this project just
closed, for a UX problem it does not actually solve (a workflow layer inside
cairn still gives a fresh user nothing if they never install `bd`).

## Option C: two documented worlds

Leave beads for this repo's own development and native Todo artefacts for
what `cairn init` scaffolds for users, but document the split explicitly
(a "how we build cairn" doc distinct from "how cairn tells you to build").
This is honest but does not close the gap: it keeps cairn eating a different
diet than it recommends, which is precisely the friction that prompted this
research. It also means every "what should `cairn next` show" investigation
still forks on "which world is this project in", so `render_next`/
`render_brief`/`next_recommended` accumulate special cases with no unifying
default. Rejected: it treats the symptom (the two docs disagree) rather than
the cause (the two mechanisms disagree).

## Finding: Option A is a small, additive change

- The Todo artefact type is already fully implemented and loaded
  (`src/artefacts/registry/mod.rs:47`, `load_todos`); only the write verb
  and the render-priority flip are new code. No new artefact schema, no new
  loader.
- `dec.beads-task-layer`'s own revisit trigger 3 ("cairn adopts a non-beads
  task tracker, making the beads-derived view the wrong source") names this
  exact move as the one that would fire it — it does not forbid it, it
  anticipates it, and does not retract the read-only view when it fires.
- `dec.change-format-only`'s criterion ("validating or applying declared
  state is cairn's job; creating, claiming, or sequencing work items is
  workflow") is satisfied by `cairn todo new` under the identical reading
  that already shipped `cairn decision new`: scaffolding a file is
  authoring declared state, not scheduling it. There is no claim verb, no
  close verb, no sequencing; status changes are file edits, same as a
  decision's `status:` field.

## Conclusion

Option A. See `dec.native-todos-first` for the ruling and its consequences.
