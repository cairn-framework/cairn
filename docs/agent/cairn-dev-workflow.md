# The Cairn Dev Loop

How to iterate on cairn, using cairn. A short descriptive overview only.

## Authority

The sole normative orchestrator is the `/cairn-loop` command
(`.claude/commands/cairn-loop.md`) plus the skills it loads
(`cairn-loop-recovery`, `cairn-loop-landing`). This document is descriptive,
never normative. Where this overview and the command disagree, the command
wins. See `dec.loop-command-harness-model`.

## What the loop is

A one-unit-per-session coding workflow that develops cairn with cairn's own
queries and gates. Under a harness loop (`/loop N`) the command is the
injected message: one unit, one branch, one PR, one squash commit on main,
then end. The harness owns repetition; the command never selects a second
unit.

At a glance:

| Phase | Question | Primary tool |
|---|---|---|
| Orient / preflight | What state is the tree and graph in? | git status, `cairn lint` |
| Scope | What does this change touch, and why is it shaped this way? | `cairn neighbourhood`, `cairn rationale`, `cairn deps` |
| Propose | What am I building, and what counts as done? | `cairn-propose` skill, or a decision artefact |
| Implement + test | Make the change, keep the map honest | edit code + `cairn.blueprint`, `cargo test` |
| Verify | Did I introduce drift or break the build? | language gates, `cairn scan`, `cairn hook all` |
| Record | Why did this change happen? | decision artefact |
| Land + merge | Is the change reviewable and on main? | `cairn-loop-landing` skill |

Each iteration ends with exactly one of `ITERATION COMPLETE`,
`LOOP EXHAUSTED`, or `LOOP HALTED` as the final line.

## How to run it

```bash
# Attended: one iteration
/cairn-loop

# Unattended harness: re-inject the same message N times
/loop N
/cairn-loop
```

Optional MISSION text after the command names a unit, scopes selection, or
describes new work. See the command for the full precedence table.

## Where the procedures live

| Procedure | Home |
|---|---|
| State recovery (dirty tree, open PR, interrupted cleanup, quarantine) | `.claude/skills/cairn-loop-recovery/SKILL.md` |
| Land (commit, PR, two-lens review) and fail-closed merge | `.claude/skills/cairn-loop-landing/SKILL.md` |
| CLI surface, blueprint syntax, artefact schemas | `.claude/skills/cairn-dev/SKILL.md` |
| Substantial change scaffolding | `.claude/skills/cairn-propose/SKILL.md` |
| Apply a change | `.claude/skills/cairn-apply/SKILL.md` |

## Prerequisites

Cairn must reflect the current source when the loop runs queries. The command
owns the bind: in this repo it builds `./target/debug/cairn` in the loop
worktree after the preflight verdict. Outside this repo, the shipped default
is a PATH `cairn` verified with `--version`.

## Further reading

- `dec.loop-command-harness-model` — harness-mode router, fail-closed
  recovery, single-commit landing, authority rule.
- `dec.adopt-cairn-dev-loop` — original adoption of the ten-phase loop
  (refined, not superseded; its "doc is canonical" consequence is now
  inoperative).
- `dec.loop-resolves-knowable-gaps` — investigate, frame, recommend before
  escalating a blocked unit.
