---
node: cairn.root
status: done
created: 2026-07-15
related: [dec.github-todo-issue-body-fidelity, todo.github-todo-full-issue-body, dec.todo-relationship-model]
---

# Typed todo relationships and deterministic GitHub issue links

## Problem

The owner wants the GitHub issue create/update path to include deterministic links
when todos have tasks, subtasks, or dependencies. Cairn todos have no typed
relationship model today: the `Todo` struct is path / node / status / created /
satisfies (optional) / body only (spec.md section 8.2;
`src/artefacts/registry/types.rs`). Free-text frontmatter such as `related` or
`informed_by` appears on a few todos but is not typed, validated, or projected.
Change `tasks.md` files are free-form checkboxes with no stable IDs. Emitting
deterministic sub-issue / blocked-by / related links from the projector therefore
requires a typed relationship model first.

This work is larger than full-body fidelity (`dec.github-todo-issue-body-fidelity`,
`todo.github-todo-full-issue-body`) and is deliberately separate from it.

## Status note

The prerequisite implementation landed in PR #570. This unit's GitHub
projector remainder is complete in this change.

## Prerequisite

`todo.todo-relationship-schema-implementation` landed end to end in PR #570.
Field and finding semantics live in `dec.todo-relationship-model` and are not
restated here.

## Task

1. Once typed relationships exist, extend the projector so a single sync run
   emits deterministic GitHub links (sub-issue / blocked-by / related) for every
   non-done todo (open, in_progress, or blocked) that declares them.
2. Use a two-phase design to avoid pre-create inventory non-determinism:
   - Phase 1: upsert every issue (create or rebody identity fields) so every
     non-done todo has a GitHub issue number.
   - Phase 2: refresh the slug-to-issue-number inventory, then render and rebody
     all issues with cross-issue number links.
   Link lists must be rendered in a stable sorted order so multiple links do not
   reshuffle between runs and force a perpetual rebody. This avoids the trap
   where a newly created sibling would fall back to an artefact-path link until
   a second run, and keeps the second-run no-op.
3. Keep the projector one-way, idempotent, and dry-run safe. The cairn binary
   still never calls GitHub.

## Acceptance

- (a) Prerequisite: typed relationship schema decision accepted, and schema +
  CLI verb + scanner support for typed todo dependencies / subtasks /
  parent-child are in place. Without that, this todo stays blocked.
- (b) Once typed relationships exist, the projector emits deterministic GitHub
  links (sub-issue / blocked-by / related) in a single sync run via the two-phase
  design above (phase 1 upsert every issue; phase 2 refresh inventory then
  rebody with issue-number links). Link lists are rendered in a stable sorted
  order.
- (c) Tests assert first-run convergence: links resolve to issue numbers in one
  run, with no artefact-path fallback. Link lists are rendered in a stable sorted
  order so a second run is a no-op (no perpetual rebody from link reshuffling).

## Non-goals

- Do not invent relationship semantics inside `scripts/sync-github-todos.sh`
  without the schema decision.
- Do not treat free-text frontmatter as typed relationships.
- Do not fold this work into `todo.github-todo-full-issue-body`; full-body
  fidelity ships independently.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves maintainable. It unblocks when todo-relationship-schema-implementation lands.

cairn.root anchor justified (2026-08-07): remaining work is the GitHub projector in unowned scripts/.

2026-08-07 audit (todo.roadmap-assumption-audit): schema shipped (PR #570); narrow to the GitHub projection remainder.
