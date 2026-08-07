---
node: cairn.root
status: open
created: 2026-07-15
satisfies: dec.task-tracking-authority
---

# Project full todo markdown into the GitHub issue body

## Problem

The one-way projector (`scripts/sync-github-todos.sh`, ratified by
`dec.github-todo-sync-projector`, now consolidated into
`dec.task-tracking-authority` clause 7) currently writes only a stub body: the
`cairn-todo: todo.<slug>` marker, a minimal node/status/artefact header, and a
multi-line one-way disclaimer. The todo's real markdown (H1, Problem, Task,
Acceptance, and so on) never reaches GitHub. The issue inventory materialises
only status and node from the fetched body, so rebody fires only when status or
node changes; a body-only edit is invisible to the next sync.
`dec.task-tracking-authority` clause 7 carries the full-body mirroring ruling
(originally `dec.github-todo-issue-body-fidelity`); this todo implements that
ruling only.

## Scope

FULL BODY VERBATIM only. No relationship-link logic, no subtask or dependency
graph, no typed frontmatter beyond what already exists. Relationship projection
is tracked separately by `todo.todo-relationship-model-and-issue-links`.

## Task

1. Change `scripts/sync-github-todos.sh` so the rendered issue body is:
   - the stable marker line `cairn-todo: todo.<slug>` first;
   - a minimal deterministic header of `node:`, `status:`, `artefact:`, and a
     single one-line one-way note (for example `one-way mirror of a cairn todo;
     edits here are not read back, dec.task-tracking-authority`); the prior multi-line
     disclaimer paragraph is dropped so the implementer neither keeps both nor
     loses the one-way signal;
   - then the complete todo markdown body after the todo's frontmatter (H1 and
     every subsequent section, verbatim).
2. Retain the rendered body (or a stable content hash of it) in the projector
   inventory. `gh` already fetches `.body`. Rebody if and only if the newly
   rendered body differs from the issue's current body (or hash). Do not rebody
   only on status/node change, and do not rebody every run.
3. Keep the projector strictly one-way and idempotent: GitHub edits are never
   read back into the canonical store; two consecutive runs with no file change
   perform no `issue edit`; `--dry-run` stays mutation-free.
4. Keep `scripts/sync-github-todos.sh` as the only GitHub writer; the cairn binary
   must not call GitHub.
5. Extend `tests/sync_github_todos.rs` so create asserts the full body payload, a
   body-only edit (status and node unchanged) records an `issue edit --body`, and
   a no-file-change second run records no `issue edit`.

## Acceptance

- (a) Issue body equals marker line + minimal node/status/artefact header + a
  single one-line one-way note + the complete todo markdown body after
  frontmatter (H1 and all sections). The prior multi-line disclaimer paragraph
  is not present.
- (b) A body-only edit (status and node unchanged) triggers a rebody on the next
  sync because the rendered body (or its hash) differs from the issue's current
  body retained in the inventory.
- (c) Two consecutive runs with no file change perform no `issue edit`
  (idempotent), proven by comparing the rendered body to the fetched issue body
  / hash. `--dry-run` records no mutations.
- (d) `scripts/sync-github-todos.sh` remains the only writer; the cairn binary
  never calls GitHub.
- (e) `tests/sync_github_todos.rs` is extended so create asserts the full body
  payload, a body-only edit records an `issue edit --body`, and a no-file-change
  second run records no `issue edit`.

## Non-goals

- No relationship, subtask, parent-child, or blocked-by link emission.
- No schema or CLI changes for typed relationships.
- No change to the inward unmapped-issue flagger behaviour.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves investigable. It improves fidelity when todo work is carried into GitHub issues.

cairn.root anchor justified (2026-08-07): scripts/ and .github/ own no node.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; required by the accepted task-tracking authority.
