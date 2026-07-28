---
id: dec.github-todo-issue-body-fidelity
nodes:
  - cairn.root
status: superseded
date: 2026-07-15
informed_by: [res.github-todo-sync]
refines: [dec.github-todo-sync-projector]
---

# GitHub issue body fidelity: mirror the full todo markdown

## Context

`dec.github-todo-sync-projector` (accepted) and `res.github-todo-sync` established a
one-way projector: native todos in `meta/todos/*.md` are canonical; GitHub issues
are a derived view. The shipped script (`scripts/sync-github-todos.sh`) upserts one
issue per non-done todo (open, in_progress, or blocked), keyed by the stable body
marker `cairn-todo: todo.<slug>`, and closes issues for done or deleted todos.

Today the projected issue body is only a generated stub: the marker line, a
`node:` / `status:` / `artefact:` header, and a multi-line one-way disclaimer. It
does not include the todo's actual markdown body (Problem, Task, Acceptance, and
so on). The issue inventory materialises only status and node from the fetched
body (`scripts/sync-github-todos.sh` around lines 104-108), so rebody fires only
on a status or node change, not on a body-content edit. As a result the GitHub
issue is a pointer, not a readable projection of the work unit.

The owner proposed full-body mirroring in-session on 2026-07-15: the synced issue
should carry the full todo information so a reader on GitHub sees the same content
as the artefact in git. That proposal is the force this decision ratifies.

Separately, the owner asked whether todos with tasks, subtasks, or dependencies
should project deterministic cross-issue links. Cairn todos have no typed
relationship model today: the `Todo` struct is path / node / status / created /
satisfies (optional) / body only (spec.md section 8.2). Free-text frontmatter such
as `related` or `informed_by` appears on a few todos but is not typed, validated,
or projected. Deterministic subtask and dependency links therefore require a typed
relationship schema first. That larger feature is scoped out of this decision.

## Decision

The GitHub issue for a todo mirrors the full todo markdown body verbatim. The
projected body is assembled in this fixed order:

1. The stable marker line `cairn-todo: todo.<slug>` first.
2. A minimal deterministic header of `node:`, `status:`, `artefact:`, and a single
   one-line one-way note (for example `one-way mirror of a cairn todo; edits here
   are not read back, dec.bead-github-sync`). The prior multi-line disclaimer
   paragraph is dropped; the one-line note keeps the one-way signal without
   retaining both.
3. The complete markdown body of the todo after its frontmatter (H1 and every
   subsequent section, unchanged).

Rebody rule (against today's inventory, which only materialises status/node): the
projector must retain the rendered body, or a stable content hash of it, in its
inventory (`gh` already fetches `.body`). Rebody if and only if the newly rendered
body differs from the issue's current body (or hash). That is what makes the
projector (i) catch body-only edits and (ii) stay idempotent: it must not rebody
every run. Status-or-node-only rebody is insufficient once the body is
load-bearing. The projector remains strictly one-way: GitHub edits are never read
back into the canonical store (`dec.bead-github-sync`). Two consecutive runs with
no file change perform no `issue edit` (idempotent).

## Explicit non-goal

Cross-artefact relationship links and any subtask or dependency graph are scoped
out of this decision. They are deferred to a separate typed-relationship schema
decision (suggested `dec.todo-relationship-model`) and tracked by
`todo.todo-relationship-model-and-issue-links`. This decision does not introduce,
imply, or require any relationship schema.

## Rationale

- Full-body mirroring makes the GitHub projection useful as a human-readable view
  of the work unit without inventing a second store or a second authoring surface.
- Comparing the rendered body (or its hash) to the fetched issue body is the
  minimal change that closes the body-edit gap; status-or-node-only rebody is
  insufficient once the body is load-bearing, and rebodying every run would break
  idempotency.
- Keeping the marker line first preserves the existing identity key used by the
  projector inventory.
- A single one-line one-way note in the header keeps the one-way signal without
  the multi-line disclaimer paragraph that currently pads the stub.
- Scoping relationship links out avoids smuggling an untyped frontmatter
  convention into the projector; the larger feature needs schema, CLI, and scanner
  support that do not exist today.

## Consequences

- `scripts/sync-github-todos.sh` must render the full post-frontmatter body into
  every create and rebody. The deterministic header includes the single one-line
  one-way note; the prior multi-line disclaimer paragraph is dropped.
- The projector inventory must retain the rendered body (or a stable content hash
  of it). `gh` already fetches `.body`; rebody if and only if the rendered body
  differs from the issue's current body (or hash). Two consecutive runs with no
  file change perform no `issue edit`, proven by comparing the rendered body to
  the fetched issue body / hash. Status-or-node-only rebody is retired.
- `tests/sync_github_todos.rs` must assert the full body payload on create, a
  body-only edit path that records an `issue edit --body`, and a no-file-change
  second run that records no `issue edit`.
- The cairn binary still never calls GitHub; the script remains the only writer.
- Relationship and subtask projection stay blocked until a typed relationship
  model is decided and implemented.
- Implementation of the full-body path is tracked by
  `todo.github-todo-full-issue-body`.

revisit_triggers:
  - a typed todo relationship model is accepted and the projector is asked to emit
    cross-issue links in the same body
  - GitHub issue body size limits start truncating full todo bodies in practice
