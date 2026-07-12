---
node: cairn.root
status: done
created: 2026-07-10
informed_by: [res.github-todo-sync]
---

# Deterministically mirror native todos into GitHub issues

## Problem
On 2026-07-10 the repo carried 44 open GitHub issues against 71 total, while all
in-flight repo work is tracked in native `meta/todos/*.md` artefacts (the front
door per AGENTS.md and `dec.native-todos-first`). Roughly 16 issues were filed
directly on GitHub, many via the `cairn feedback` friction channel, with no
corresponding todo. The two trackers have diverged: GitHub reads as the live
backlog to outsiders, but it is not the source of truth, so work is lost
between the two and re-filed. This breaks the invariant ratified in
`dec.bead-github-sync`: files in git are canonical; GitHub issues are a derived
projection, never a second store.

## Evidence
- `dec.bead-github-sync` (accepted) ratifies a one-way, opt-in mirror: GitHub is
  a read-only projection, never canonical, never bidirectional, and cairn itself
  stays out of the GitHub write path. Its mechanism assumed beads/Dolt, which
  this repo has since replaced with native todos (`dec.native-todos-first`).
- Spec §8.2 (Todo authority) defines the canonical shape: one file per node,
  `status: open|in_progress|done|blocked`, exactly one node ID per todo.
- `res.native-task-state-gap` documents the same drift symptom and the
  filesystem-as-source-of-truth convention for this repo.
- `cairn feedback` (issue #246) prints a prefilled upstream issue URL, so
  external issues keep arriving; an inward triage path is required.

## Proposed approach
Implement the design in `res.github-todo-sync`: a bespoke ~120-line projector
script at `scripts/sync-github-todos.sh` that mirrors `meta/todos/*.md` to
GitHub issues one way, plus an inward unmapped-issue flagger. Summary:

1. **Outward (todo -> issue), idempotent.** Stable marker line
   `cairn-todo: todo.<slug>` in each issue body plus label `cairn-todo`. On each
   run the script lists issues with that label (open and closed), builds a
   slug-to-number map, and upserts: a new todo creates an issue, a status change
   updates it, `done` closes it. Re-running is a no-op.
2. **Inward (issue -> triage flag), no auto-import.** Issues without the marker
   (externally filed) get label `cairn-todo-unmapped` and a triage comment; the
   script never creates a todo from them.
3. **Runner.** A GitHub Actions job on push to `main` (single writer, after
   merge) invokes the script with a `GH_TOKEN`; also runnable locally as
   `scripts/sync-github-todos.sh` for ad-hoc sync. Cairn code never touches
   GitHub (honours `dec.bead-github-sync` ruling 2).
4. **Lifecycle of deletes.** A todo removed from `meta/todos/` closes its open
   issue (with a note) and is left closed; GitHub issues are never deleted, so
   history is preserved.

## Acceptance
- Running the script twice in a row is a no-op (idempotent upsert; current state
  is read before any `gh` write).
- Creating one new todo produces exactly one GitHub issue carrying the
  `cairn-todo` label and the `cairn-todo: todo.<slug>` marker line.
- Flipping a todo to `done` closes exactly that issue; the marker line is
  retained so a later re-sync stays a no-op.
- An externally filed issue (no marker) is labelled `cairn-todo-unmapped` and
  commented for triage; no todo is auto-created.
- The script lives under `scripts/` and is exercised by CI (or a documented
  hook); `cairn lint`/`scan` show no new findings from the artefact change.

## Dependencies / ordering
- **Land after `todo.github-issues-cleanup`.** The first sync must start from a
  clean issue set so the mirror maps existing todos to fresh issues rather than
  inheriting the 16-drift noise. That cleanup todo and this one are both on
  `cairn.root`.
- Research dependency: `res.github-todo-sync` (this todo's `informed_by`).
- **Consumes the `todo.unified-todo-write-surface` seam (soft dep).** Once the
  unified write verb exists, todo mutations flow through one call the projector
  can observe, making the mirror clean and PR-free: updating task state stops
  requiring a code PR because state changes propagate through the verb to this
  projection. The script can ship before the verb (reading files directly), but
  design it so the verb becomes its trigger.

## Done (2026-07-12)

Shipped `scripts/sync-github-todos.sh` (one-way projector, marker line
`cairn-todo: todo.<slug>` plus `cairn-todo` label, idempotent upsert,
done/deleted closes, inward `cairn-todo-unmapped` flagging with no
auto-import, `--dry-run` mode, repository scope guard) and
`.github/workflows/todo-sync.yml` (push to main on `meta/todos/**`,
issues:write only, also workflow_dispatch; runnable locally). Behavioural
coverage in `tests/sync_github_todos.rs` against a stub `gh` recording
mutations: create, no-op, done-close, reopen, deleted-todo close,
external flag without import, mutation-free dry run. Live `--dry-run`
verified against the real repo. The first real sync runs when this
merges to main.
