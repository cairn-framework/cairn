---
node: cairn.root
status: open
created: 2026-08-09
---

# Driver In Repo Reaction Loop

## Scope
Specify and later implement the explicit reaction loop:
an exact terminal token from the harness, driver verification and
classification, a structured outcome fact, a fresh canonical selector
re-query, and dispatch of the next unit. The driver combines assignment
metadata such as unit id, lease, and commit at grant with repository
verification on refreshed `main`. Graph facts are written only through
sanctioned verbs. Ephemeral session liveness and run activity stay on the
driver's declared surface.

## Parent constraints
The parent todo is `todo.driver-in-repo`, under `## Task`, item 3:

> The reaction loop, stated explicitly rather than implied. The
> harness ends its session with one exact terminal token; the driver
> combines its own assignment metadata (unit id, lease, commit at
> grant) with repository verification on refreshed `main` into the
> structured outcome fact. Graph facts (lease and outcome records) are
> written only through sanctioned verbs; ephemeral live execution
> state (session liveness, run activity) stays on the driver's own
> declared surface. The driver then re-queries the canonical selector
> and dispatches the next unit. Nothing in that sequence is a core
> behaviour.

The parent `## Grill rulings`, Q3, fixes the token and derived-class boundary:

> **Q3, outcome vocabulary: the harness emits only the three ratified
> terminal tokens** (`ITERATION COMPLETE`, `LOOP EXHAUSTED`,
> `LOOP HALTED`) as the exact final line. The driver derives the real
> classification from verifiable repo state and records it as an
> outcome fact keyed by unit id and commit. Derived classes: complete
> and verified (todo `done` on refreshed main); complete but parked
> (todo went `blocked`; never advances, the persisted recommendation
> enters the signature queue); claim failed verification (token without
> the landed flip; fail-closed, human moment); exhausted (driver
> cross-checks its own selector read, two readers must agree); halted
> (fail-closed, human attention); aborted (no token). `LOOP HALTED`
> gains no source-side split: finer routing keys on repo evidence, not
> on new tokens.

The parent `## Acceptance` requires this sequence end to end, including an
outcome that must not advance the loop, and retains fail-closed handling for a
dirty park, off-main HEAD, surviving branch or PR, nonzero session exit, a
non-token final line, exhausted without the todo `done` on main, and a
completion where the todo became `blocked`.

The parent `## Lease shape` says that concrete lease schema, store layout,
sanctioned verb shapes, hotspot ownership, and promotion trigger mechanics
belong to `todo.parallel-dispatch-granularity`. Do not freeze those details in
this sub-todo.

## Dependencies
The blueprint-node sub-todo is first. This sub-todo consumes the workflow
artefact policy and the selector-wire contract. The selector-wire sub-todo is
an explicit prerequisite for the reaction loop because re-query and
first-member equality rely on its ready-set evidence. Lease implementation
details remain with the separate lease owner named above.

## Sizing
M. The later implementation is one driver-control subsystem with focused
end-to-end harness and fail-closed scenario tests, kept under roughly 600
changed lines. It must consume, rather than redesign, the selector, workflow,
and lease contracts.

## Non-goals
Do not add core orchestration, invent terminal tokens, treat an agent claim as
verification, widen `cairn watch`, or implement the lease store and console
surface.
