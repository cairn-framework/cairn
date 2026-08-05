---
node: cairn.root
status: blocked
created: 2026-08-03
related: [dec.orchestration-placement, dec.control-plane-programme, todo.console-orchestration-ux-design, todo.console-signed-widening, todo.parallel-dispatch-granularity, todo.review-gate-machine-check]
---

# Driver In Repo

`dec.orchestration-placement` (proposed, binding) would let the driver
live in this repository as a distinct layer beside the passive core,
fronted by the webui. This unit builds that layer once the record is
signed.

Blocked on the maintainer signing `dec.orchestration-placement`; that
gate is external, so no `blocked_by:` entry is declarable (ruling 4 of
`dec.todo-relationship-model`). The signature is not a formality here: it
supersedes `dec.product-perimeter`, whose clause 4 keeps the driver
outside the repository until then.

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
3. The reaction loop, stated explicitly rather than implied. The harness
   returns a structured outcome keyed by unit and commit; the driver
   verifies that refreshed `main` carries the landed work and reconciles
   it; the driver records lease and outcome state through a declared
   driver-owned surface or a sanctioned verb; the driver re-queries the
   canonical selector; the driver dispatches the next unit. Nothing in
   that sequence is a core behaviour.
4. The selector wire the loop needs. `cairn next` today exposes no
   stable unit id and no reproducible selection evidence, groups findings
   into remediation actions, and orders todos by creation date and path
   rather than the loop's own precedence. Establish what the driver needs
   (commit and schema version, unit id, node, selection ground,
   reproducible evidence) and land it as a passive query, or file the
   exact missing field against the owning node. This is the prerequisite
   for proving that a dispatched mission equals what a loop session's
   Orient step would select at the same commit.
5. Steering surface: `todo.console-signed-widening` wires the console to
   this driver under `dec.control-plane-programme`'s ownership split. It
   is blocked on `todo.console-orchestration-ux-design` and does not gate
   this unit's non-UI work.

## Acceptance

- The placement decision's layer rules are quotable against the shipped
  structure: the driver has its own node or nodes, the core has no
  dependency into it, and the substrate gained no orchestration
  behaviour.
- A dry run prints the mission it would dispatch, with unit id, selection
  ground, and evidence lines, and that mission equals what a loop
  session's Orient step selects at the same commit.
- The outcome-to-next-dispatch sequence in task 3 is exercised end to
  end, including a harness outcome that must not advance the loop.
- Every fail-closed condition the external v1 driver enforced still stops
  this one: a dirty park, HEAD off `origin/main`, a surviving loop branch
  or open loop PR, a nonzero session exit, a final line that is not
  exactly a terminal token, `LOOP EXHAUSTED` without the unit's todo
  `done` on main, and a completion where the todo went `blocked`.

## Grill rulings (2026-08-04, maintainer in session)

The orchestration grill (`studio/orchestration-grill-brief.md`) put Q3
to the maintainer; the answer below is a ratified constraint on task 3.

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
  restart re-reads lease facts and classifies orphaned leases as stale.
  Invariant, stated honestly: lease expiry bounds ambiguity, not driver
  availability. While a driver supervises, every dispatched unit
  reaches a recorded terminal outcome within one lease TTL regardless
  of agent behaviour. With no live driver, an expired lease renders as
  stale and unclassified until a driver returns and classifies it; the
  console never promises a terminal fact that has not been recorded,
  and a driver outage is itself a rendered state (no driver attached).
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
  driver whose printed wave provably equals a manual Orient selection
  at the same commit. The lease surface (sanctioned grant, renew, and
  release verbs, the lease-facts read query, one shared store across
  worktrees) is promoted from on-demand to required by the Q2 ruling.
  The findings blast radius fix (one Error finding must not disable
  unrelated read queries) lands before unattended operation, since a
  red finding must never blind the driver. Watch stays finding-change
  only: placement clause 3 makes notifications invalidation hints, the
  driver re-reads authoritative queries before acting, and polling
  suffices until measurably painful.
- **Q9, surfaced gaps.** Four gaps no question owned, ruled by the
  maintainer. (1) Driver singleton: exactly one driver runs per
  repository, enforced with the same lease machinery ruled in Q2: the
  driver holds a lease fact on the driver singleton itself, and a
  second claimant is a first-class rendered conflict, never a silent
  split of the fleet; multi-device handoff later becomes a
  driver-lease transfer. (2) Budget: the workflow `limits:` slot
  carries per-unit and per-wave spend caps beside wave size and TTL;
  the driver refuses dispatch past a cap and queues a budget-exhausted
  human moment. (3) Authorised-caller trust model: deferred with a
  named revisit trigger (first observed verb abuse, or any
  multi-tenant use); Q3 verification already quarantines the
  consequence of a false status flip. (4) Outcome-fact retention and
  compaction: deferred into task 3's design as a storage detail.

## Lease shape

Do not freeze the lease and claim schema before
`todo.parallel-dispatch-granularity` rules on what a claim is held on,
its identity, expiry, and renewal, and what execution granularity means
when one unit touches several nodes. That unit owns the ruling;
`todo.console-orchestration-ux-design` contributes the mockup evidence
it is made against. `lease` currently appears in no Rust source, so the
shape is genuinely open.

## Relationship to the driver v2 change

`meta/changes/driver-v2-selection` remains as authored: it audits the
read surface and hardens the external v1 supervisor. Its read-surface
audit is the direct input to task 4 here. This unit is the in-repo
successor that change deliberately did not become.
