---
id: src.reddit-gregerw-first-user-test
file: docs/research/reddit-gregerw-thread-2026-07-31.md
verification: tracked
type: thread
date: 2026-07-31
---

# Reddit user gregerw: first detailed external user test

A Reddit user (gregerw) engaged twice with the maintainer's cairn post and
then installed cairn on one of their own codebases. The full transcript is
preserved at the tracked path above; the thread URL was not pinned. This
record extracts the observations; analysis and backlog mapping live in
`res.inversion-convergence-minutes` row R6 and its follow-up todos.

## Round one (pre-test, on the landing page)

- The workflow is hard to understand from the landing page: the outside
  and inside perspectives blur, and the reader cannot tell what a user
  must understand versus what is internal cairn architecture.
- The user frames cairn's value in ADR terms: turning emergent code
  patterns into reviewable decision records, making agents adhere to
  them, and keeping humans aware so they can steer.
- Two explicit suggestions: document the workflows, and separate
  user-facing concepts from internal ones on the landing page.
- Pointer to Humanlayer as a tangent or comparison product.

## Round two (after installing on a real codebase)

- Uncertainty before install about what would change and whether normal
  work could continue; the documented uninstall path is what made trying
  it acceptable, on a separate branch.
- Surprise at how many files install created; it read as machinery whose
  purpose was not self-evident.
- A dead end after install: no pointer to what to do next. The user found
  the UI on port 3000 only by remembering the earlier thread.
- The module and dependency overview was pleasant but underwhelming
  relative to install weight; the connection to ADRs and higher-level
  patterns was not discoverable.
- The user looked for, and could not find, a way to extract decisions
  already embedded in the existing codebase into the graph as invariants
  that would be gated later.
- Outcome: uninstalled; immediate value did not justify the repo files
  and gating. The user notes their spend (one install, a look around, a
  browse of the UI) is representative of what most evaluators will give.

## Maintainer statements in the thread (direction, as said)

The UI is broken-in-place pending a full overhaul; the lint-driven
decision and ghost-node discipline works ("it forces the AI to write the
decision behind why it is working on something"); using cairn only from
inside harness sessions is the wrong level and causes friction between
sessions and against the bigger project vision; the direction is cairn
within the harness (keeping agents in line) and outside it as the
declarative workflow surface, with UX that guides a human towards
load-bearing problems; evals comparing outcomes with and without cairn
are planned before further public posting.
