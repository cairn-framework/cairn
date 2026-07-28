---
id: dec.github-todo-sync-projector
nodes:
  - cairn.root
status: superseded
date: 2026-07-10
informed_by: [res.github-todo-sync]
---

# GitHub issue sync: one-way projector, files stay canonical

## Context

Roughly 16 issues were filed directly on GitHub by an agent while this
repo's own work moved to native todo artefacts, and the two sets drifted.
`res.github-todo-sync` evaluated sync topologies (bidirectional sync,
issue-canonical import, one-way projection) against the architectural
invariant that files in git are the source of truth and any external
store is a derived projection (see `bead-github-sync.md` for the same
conclusion in the beads era).

## Decision

Adopt a one-way projector: `meta/todos/*.md` is canonical, GitHub issues
are a derived view.

- A small script (`scripts/sync-github-todos.sh`, target ~120 lines,
  run by CI on pushes to main) upserts one issue per todo artefact,
  keyed by a stable marker line in the issue body.
- Todo status `done` closes the mirrored issue.
- Issues filed directly on GitHub are never auto-imported; the projector
  labels them `cairn-todo-unmapped` for human triage (close, or mint a
  native todo by hand).

Implementation is tracked by `todo.github-todo-sync` and runs after the
legacy-issue cleanup (`todo.github-issues-cleanup`) so the initial sync
starts from a clean issue set.

## Consequences

- No second source of truth: a lost or vandalised issue set is
  regenerable from the repo.
- External contributors' issues surface through triage rather than
  silently entering the todo backlog.
- Bidirectional editing of todo content via GitHub is explicitly not
  supported; edits happen in files.

revisit_triggers:
  - cairn gains multi-writer hosted coordination that makes GitHub the
    natural front door for task intake
  - the unmapped-issue triage queue becomes a recurring bottleneck
