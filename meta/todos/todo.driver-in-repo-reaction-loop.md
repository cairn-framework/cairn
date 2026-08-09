---
node: cairn.root
status: open
created: 2026-08-09
parent: todo.driver-in-repo
blocked_by: [todo.driver-in-repo-workflow-artefacts, todo.driver-in-repo-selector-wire, todo.driver-in-repo-blueprint-node]
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

The parent `## Grill rulings`, Q3, is provisional grill direction for the
derived classes pending an accepted owning decision. The three terminal token
strings are independently ratified by accepted `dec.orchestrator-skills-layering`,
clause 4, which also ratifies the no-orchestration rule:

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

The Q3-derived classes, routing keys, and storage details remain provisional
pending that owning decision. The parent `## Acceptance` requires this sequence
end to end, including an outcome that must not advance the loop, and retains
fail-closed handling for a dirty park, off-main HEAD, surviving branch or PR,
nonzero session exit, a non-token final line, exhausted without the todo `done`
on main, and a completion where the todo became `blocked`.

The parent `## Grill rulings`, **Stall and no-return handling**, is also part of
this scope:

> **Stall and no-return handling.** Abort reasons are driver-observed,
> never agent claims: `crashed` (session exit without a token, seen by
> supervision immediately), `stalled` (renewal is driver-performed and
> conditional on observed progress, output or commits on the unit
> branch; no progress means renewal withheld, TTL expiry, supervised
> kill, residue quarantined, bounded retry then human moment). A driver
> restart re-reads lease facts and derives, from an explicit
> observation time, which leases expired while it was away. Invariant,
> stated honestly: lease expiry bounds ambiguity, not completion, and
> not driver availability. Renewal can extend a productive unit
> indefinitely, so the enforceable bound is this: while a driver
> supervises, once renewal is withheld the unit reaches a recorded
> terminal outcome within its final lease term plus the supervised-kill
> grace. With no live driver, an expired lease renders as stale and
> unclassified until a driver returns and classifies it; the console
> never promises a terminal fact that has not been recorded, and a
> driver outage is itself a rendered state (no driver attached).

The parent `## Grill rulings`, Q9, leaves two task-3 obligations explicit:

> (3) Authorised-caller trust model: deferred as an unmitigated risk with a
> named revisit trigger (first observed verb abuse, or any multi-tenant use).
> Q3 verification catches a token without a landed flip; it does not catch an
> authorised caller falsely flipping a todo `done`, and no current machinery
> does. The future mitigation shape is independent landed-PR or acceptance
> evidence checked beyond todo status.

> (4) Outcome-fact retention and compaction: deferred into task 3's design as
> a storage detail.

The parent `## Lease shape` says that concrete lease schema, store layout,
sanctioned verb shapes, hotspot ownership, and promotion trigger mechanics
belong to `todo.parallel-dispatch-granularity`. Do not freeze those details in
this sub-todo.

## Dependencies
The blueprint-node, workflow-artefacts, and selector-wire dependencies are
typed `blocked_by` edges on this child. The selector-wire edge is an explicit
prerequisite for the reaction loop because re-query and first-member equality
rely on its ready-set evidence. Lease implementation details remain with the
separate lease owner named above. The parent carries typed `blocked_by` edges
to all four children.

## Sizing
M. The later implementation is one driver-control subsystem with focused
end-to-end harness and fail-closed scenario tests, kept under roughly 600
changed lines. It must consume, rather than redesign, the selector, workflow,
and lease contracts.

## Non-goals
Do not add core orchestration, invent terminal tokens, treat an agent claim as
verification, widen `cairn watch`, or implement the lease store and console
surface.
