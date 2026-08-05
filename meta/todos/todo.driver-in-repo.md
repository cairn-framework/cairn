---
node: cairn.root
status: open
created: 2026-08-03
related: [dec.orchestration-placement, dec.control-plane-programme, todo.console-orchestration-ux-design, todo.console-signed-widening, todo.parallel-dispatch-granularity, todo.review-gate-machine-check]
---

# Driver In Repo

`dec.orchestration-placement` (accepted 2026-08-04, binding) lets the
driver live in this repository as a distinct layer beside the passive
core, fronted by the webui. This unit builds that layer.

The blocking gate cleared on 2026-08-04: the maintainer signed the
placement decision at the close of the orchestration grill, superseding
`dec.product-perimeter`, and this unit went `open` in the acceptance
commit.

The boundary is clauses 1 to 3 of that decision and it is not negotiable
here: the core answers queries and applies sanctioned mutations and
starts nothing. The driver observes, decides, and dispatches. A harness
executes one assigned action and returns its outcome.

## Task

1. Blueprint node or nodes for the driver layer with no core-to-driver
   dependency; the driver consumes the query wire and the sanctioned
   verbs.
2. Declarative workflow artefacts the driver reads: a task routed to a
   harness with context guidance, and on-outcome routing to another
   harness or destination. These are inert policy under clause 3. Cairn
   parses, validates, stores, and exposes them and evaluates none of
   them.
3. The reaction loop, stated explicitly rather than implied. The
   harness ends its session with one exact terminal token; the driver
   combines its own assignment metadata (unit id, lease, commit at
   grant) with repository verification on refreshed `main` into the
   structured outcome fact. Graph facts (lease and outcome records) are
   written only through sanctioned verbs; ephemeral live execution
   state (session liveness, run activity) stays on the driver's own
   declared surface. The driver then re-queries the canonical selector
   and dispatches the next unit. Nothing in that sequence is a core
   behaviour.
4. The selector wire the loop needs: a ready-set query, per the Q1 and
   Q8 rulings below. `cairn next` today exposes no stable unit id and
   no reproducible selection evidence, groups findings into remediation
   actions, and orders todos by creation date and path rather than the
   loop's own precedence. Establish the ready-set contract (commit and
   schema version; per unit: unit id, node closure, selection ground,
   reproducible evidence) and land it as a passive query, or file the
   exact missing field against the owning node. This is the
   prerequisite for the acceptance contract: the wave's first member
   equals a manual Orient selection at the same commit, and every
   additional member carries the same eligibility evidence plus
   pairwise write-set disjointness.
5. Steering surface: `todo.console-signed-widening` wires the console to
   this driver under `dec.control-plane-programme`'s ownership split. It
   is blocked on `todo.console-orchestration-ux-design` and does not gate
   this unit's non-UI work.

## Acceptance

- The placement decision's layer rules are quotable against the shipped
  structure: the driver has its own node or nodes, the core has no
  dependency into it, and the substrate gained no orchestration
  behaviour.
- A dry run prints the wave it would dispatch, each unit with id,
  selection ground, and evidence lines; the wave's first member equals
  what a loop session's Orient step selects at the same commit, and
  every additional member satisfies the same eligibility evidence plus
  pairwise write-set disjointness.
- The outcome-to-next-dispatch sequence in task 3 is exercised end to
  end, including a harness outcome that must not advance the loop.
- Every fail-closed condition the external v1 driver enforced still stops
  this one: a dirty park, HEAD off `origin/main`, a surviving loop branch
  or open loop PR, a nonzero session exit, a final line that is not
  exactly a terminal token, `LOOP EXHAUSTED` without the unit's todo
  `done` on main, and a completion where the todo went `blocked`.

## Grill rulings (2026-08-04, maintainer in session)

The orchestration grill (`studio/orchestration-grill-brief.md`) put its
questions to the maintainer. The answers below are provisional grill
direction for tasks 2 to 4, under the brief's ratification proviso
(mockup rounds and first driver experience are the falsifier); only an
accepted owning decision makes any of them binding. Independently
accepted ground cited inside them (the three terminal tokens, the Q1
slate constraint) keeps its own authority.

- **Q3, outcome vocabulary: the harness emits only the three ratified
  terminal tokens** (`ITERATION COMPLETE`, `LOOP EXHAUSTED`,
  `LOOP HALTED`) as the exact final line. The driver derives the real
  classification from verifiable repo state and records it as an
  outcome fact keyed by unit id and commit. Derived classes: complete
  and verified (todo `done` on refreshed main); complete but parked
  (todo went `blocked`; never advances, the persisted recommendation
  enters the signature queue); claim failed verification (token without
  the landed flip; fail-closed, human moment); exhausted (driver
  cross-checks its own selector read, two readers must agree); halted
  (fail-closed, human attention); aborted (no token). `LOOP HALTED`
  gains no source-side split: finer routing keys on repo evidence, not
  on new tokens.
- **Stall and no-return handling.** Abort reasons are driver-observed,
  never agent claims: `crashed` (session exit without a token, seen by
  supervision immediately), `stalled` (renewal is driver-performed and
  conditional on observed progress, output or commits on the unit
  branch; no progress means renewal withheld, TTL expiry, supervised
  kill, residue quarantined, bounded retry then human moment). A driver
  restart re-reads lease facts and derives, from an explicit
  observation time, which leases expired while it was away. Invariant,
  stated honestly: lease expiry bounds ambiguity, not completion, and
  not driver availability. Renewal can extend a productive unit
  indefinitely, so the enforceable bound is this: while a driver
  supervises, once renewal is withheld the unit reaches a recorded
  terminal outcome within its final lease term plus the supervised-kill
  grace. With no live driver, an expired lease renders as stale and
  unclassified until a driver returns and classifies it; the console
  never promises a terminal fact that has not been recorded, and a
  driver outage is itself a rendered state (no driver attached).
- **Q4, workflow definition: an inert typed cairn artefact, evaluated
  only by the driver** (task 2, confirming clause 3 of the placement
  decision). Shape: a match predicate over dispatch units, a harness
  route with context (skills, briefing), limits (wave size, TTL), and
  an outcome-class routing table over Q3's derived classes. All slots
  are closed vocabularies cairn validates at scan time; a workflow
  never carries executable logic inline. Workflows name gates as rules
  of engagement: a `require:` list of registered deterministic checks
  (rust gates, scan strict, review receipts) bound to a named moment;
  the driver enforces them on driver-observed evidence before the
  routed action fires. Policy updates are ordinary artefact edits
  landing through PR gates with provenance; the driver re-reads policy
  each cycle, so a landed change binds at the next dispatch decision.
- **Q8, core seam order: selector wire, then lease surface, then
  findings blast radius; `cairn watch` widening deferred indefinitely**
  (task 4). The selector wire is upgraded by the Q1 ruling from a
  single recommendation to a ready-set query: every dispatchable unit
  at this commit with unit id, node closure, selection ground, and
  reproducible evidence lines; its first milestone is the dry-run
  driver: the printed wave's first member provably equals the one-unit
  selection a manual Orient step makes at the same commit, and every
  additional member satisfies the same eligibility evidence plus
  pairwise write-set disjointness. The lease surface (sanctioned grant,
  renew, and
  release verbs, the lease-facts read query, one shared store across
  worktrees) is promoted from on-demand to required by the Q2 ruling.
  The findings blast radius fix (one Error finding must not disable
  unrelated read queries) lands before unattended operation, since a
  red finding must never blind the driver. Watch stays finding-change
  only: placement clause 3 makes notifications invalidation hints, the
  driver re-reads authoritative queries before acting, and polling
  suffices until measurably painful.
- **Q9, surfaced gaps.** Four gaps no question owned, ruled by the
  maintainer. (1) Driver singleton: one driver per coordination store,
  detected everywhere else. Within the shared coordination store of one
  checkout family (the Q2 seam), the singleton grant is atomic and
  enforced. It is a distinct singleton grant record with the same fact
  discipline as leases, never a unit lease: Q2's lease facts are held
  only on dispatch units. Across independent clones, repo-synced facts
  cannot mutually exclude: two drivers may both dispatch until their
  stores synchronise, so the guarantee there is detection after sync,
  rendered as a first-class conflict, never an undetected split. True
  cross-device exclusion is deferred until a shared coordination
  authority exists, when multi-device handoff becomes a singleton-grant
  transfer. (2)
  Budget: the workflow `limits:` slot
  carries per-unit and per-wave spend caps beside wave size and TTL;
  the driver refuses dispatch past a cap and queues a budget-exhausted
  human moment. (3) Authorised-caller trust model: deferred as an
  unmitigated risk with a named revisit trigger (first observed verb
  abuse, or any multi-tenant use). Q3 verification catches a token
  without a landed flip; it does not catch an authorised caller falsely
  flipping a todo `done`, and no current machinery does. The future
  mitigation shape is independent landed-PR or acceptance evidence
  checked beyond todo status. (4) Outcome-fact retention and
  compaction: deferred into task 3's design as a storage detail.

## Lease shape

The grill's Q2 ruling (recorded in
`todo.parallel-dispatch-granularity`) fixes what a lease is held on
(the dispatch unit), its identity fields, expiry, renewal, staleness,
and the fact-versus-action ownership split. What stays open for that
unit's rung 3 design document: the concrete schema and store layout,
the sanctioned verb shapes, hotspot ownership, and the promotion
trigger mechanics. Do not freeze those details here; that unit owns
them, and `todo.console-orchestration-ux-design` contributes the mockup
evidence they are made against.

## Relationship to the driver v2 change

`meta/changes/driver-v2-selection` remains as authored: it audits the
read surface and hardens the external v1 supervisor. Its read-surface
audit is the direct input to task 4 here. This unit is the in-repo
successor that change deliberately did not become.
