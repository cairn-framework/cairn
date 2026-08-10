---
node: cairn.ui
status: done
created: 2026-08-03
related: [dec.orchestration-placement, dec.control-plane-programme, dec.webui-write-authority, todo.console-signed-widening, todo.console-state-legibility, todo.driver-in-repo, todo.parallel-dispatch-granularity]
---

# Console orchestration UX: journeys and mockups before implementation

`dec.orchestration-placement` clause 4 gates orchestration-facing console
implementation on this unit. The maintainer's stated defect, 2026-08-03:
opening a decision and following it to the map does not show what is
current and actual against what is aspirational and in progress. The
journey concept the maintainer described in session solves part of that
with state chips, dimming, and dependency bands grouped by dependency
depth, and does not show execution state or a driver at all. The
observations below are self-contained, so this unit needs no untracked
input.

**This unit produces design artefacts and proposed decisions only. It
ships no console code, no CSS, and no harness change.** Implementation
lives in `todo.console-state-legibility` (the read-only legibility fixes,
which need no signature) and in `todo.console-signed-widening` (the
orchestration surface, blocked on `todo.console-round-three-closeout`,
which holds this unit's maintainer criterion).

## Evidence already gathered (2026-08-03)

Recorded here so the design phase starts from measurement, not taste. All
of it was observed against the live webui and the journey mockup.

1. **The map's four states are a 2x2 of declared against observed**
   (`synced` = both agree, `drift` = both disagree, `ghost` = declared
   only, `orphaned` = observed only), presented as four peer colours. A
   reader cannot see that `ghost` and `orphaned` are opposites.
2. **Execution state has no representation anywhere.** `lease` appears in
   no Rust source and no UI asset, so `dec.control-plane-programme`
   clause 1's lease truth is unimplemented and the console has no fact to
   render for active, waiting, or blocked. *Superseded 2026-08-10: the
   read surface shipped as `cairn lease list` and `cairn ruling list`
   (`src/cli/commands/coord.rs`,
   `src/query_api/handlers/coordination.rs`) under
   `dec.rung-three-coordination-substrate` and `dec.coord-fact-write-once`.
   The console still renders none of it, which is what
   `todo.console-signed-widening` implements; the matrix rows were
   refreshed the same day.*
3. **The signal colour is spent on the default state.** On the demo
   fixture, 23 of 24 nodes render the same mint keel and dot, so mint
   means "this node exists". The journey concept inverts this: neutral
   chassis by default, signal colour only where a move is needed.
4. **Three defect classes on the shipped surface** are recorded as
   executable tasks in `todo.console-state-legibility`, not repeated
   here: wire evidence the renderer fetches and discards, state
   distinctions that collapse under greyscale and reduced motion, and an
   `opacity` dimming failure the contrast audit cannot see. Read that
   todo before designing, because the shipped surface it describes is the
   baseline any mockup is compared against.
5. **The journey concept implies components the design system does not
   have**: a work-state tag with a fixed lamp slot, a progress band, a
   stratum container, an agent-run card, and a dispatch preview. None
   exist in `docs/design-system/components.css`. The concept used the
   same canonical tokens, so the gap is discipline and vocabulary rather
   than palette.
6. **The concept's own gaps**: its scene held only declared nodes, so
   declared against observed was separated in prose and never in pixels,
   and its dependency threads were drawn too faint to survive a rendered
   capture.

## Task

1. **Author the state-source matrix.** One row per state the console
   shows; columns for the source that produces it, whether it is intent,
   fact, projection, or execution state under
   `dec.orchestration-placement` clause 3, and the non-colour channels
   that carry it. Three families minimum: declared intent, observed
   actuality, execution state. Every state must differ from every other
   in at least two non-colour channels. Execution state has no query
   today, so its rows name a proposed, versioned driver-owned contract
   rather than an existing endpoint; the driver unit consumes that
   contract when it lands.
2. **Author the journeys and the brief.** Tracked output under `studio/`.
   The spine is the maintainer's own path: orient, open a ruling that
   waits on you, follow it to the map, see what signing it moves, and
   understand what the driver will do next and why. Name, for each step,
   the query that answers it.
3. **Author the mockups.** Static, tracked, manually reviewed. Required
   scenarios: mixed repository carrying every grammar row at once;
   decision-to-consequence (open a ruling, see on the map what signing
   moves, stated as a sentence rather than a badge); driver connected
   with runs, leases, and a dispatch preview; driver not connected,
   distinguishing no driver attached from attached and idle from crashed;
   and the narrow layout. Static mockups under manual review sit outside
   `dec.webui-write-authority` clause 5's foreclosed vision-patch loop.
4. **Settle the vocabulary as a design output.** `planned`, `ghost`,
   `declared`, and `buildable` are four words for one state today. Pick
   one and record it in the brief. The rename itself belongs to
   `todo.console-signed-widening` task 3, which implements the matrix;
   `todo.console-state-legibility` deliberately does not carry it, so
   that unit stays independent of this one.
5. **Contribute the mockup evidence the lease design needs.**
   `todo.parallel-dispatch-granularity` owns rung 3: the write-set and
   lease model, including what a claim is held on, its identity, expiry,
   renewal, and what stale looks like, and it already outputs a design
   plus an enqueued decision. This unit does not author a competing
   ruling. It renders those questions as screens so the ruling is made
   against something a reader can see, and it consumes the answer.
6. **Enqueue the webui write-authority resolution.**
   `dec.webui-design-authority` clause 4 said the webui stays read-only
   and its line 28 revisit trigger fires when scope grows beyond
   read-only exploration. That trigger fired, and
   `dec.webui-write-authority` retired the rule on 2026-08-06:
   `dec.control-plane-programme` clause 3 granted the console sanctioned
   writes and `dec.orchestration-placement` clause 4 makes it the
   driver's steering surface. Author the decision that resolves the
   contradiction, following the precedent in
   `dec.control-plane-programme` lines 41 to 46 (supersede the prior
   record, carry every surviving obligation forward in the successor). Do
   not edit the accepted record in place.

## Acceptance

- The state-source matrix exists, and every state it lists names either
  an existing query or a proposed versioned driver contract, plus at
  least two non-colour channels.
- The journeys and mockups render every required scenario, including
  populated execution state, and a reader can tell driver-absent from
  driver-idle from driver-crashed without opening a terminal.
- The lease and granularity screens are handed to
  `todo.parallel-dispatch-granularity` as evidence, and this unit records
  which of its questions the mockups answered.
- The webui write-authority decision is enqueued in `cairn pending`.
- No file under `src/` or `harness/` changed in this unit.
- The maintainer close-out of the mockup rounds is NOT an acceptance criterion
  of this unit: it moved to `todo.console-round-three-closeout` on 2026-08-10.
  The driver-state criterion above is read against the amended round-2 scope,
  and that amendment is part of what the close-out puts to the maintainer.

## Sequencing

Tasks 1 to 5 need no signature and no code. Task 6 ends in a maintainer
signature. `todo.console-signed-widening` is blocked on
`todo.console-round-three-closeout` (2026-08-10, when this unit's
maintainer criterion moved there) and
owns every line of orchestration console implementation the mockups
specify. `todo.console-state-legibility` is independent and can run first
or in parallel: it fixes the shipped read-only surface and makes the
mockup comparison honest by removing "the console showed nothing useful"
as a confound. `todo.driver-in-repo` is blocked on its four decomposition
sub-todos (blueprint node, workflow artefacts, reaction loop, selector wire)
and remains independent of this unit; the lease ruling is recorded in
`todo.parallel-dispatch-granularity`, whose rung
3 design document still owns the remaining schema details.

## Grill rulings (2026-08-04, maintainer in session)

The orchestration grill (`studio/orchestration-grill-brief.md`) put Q5
to the maintainer; the answer below is provisional grill direction for
task 2's journeys, under the brief's ratification proviso (mockup
rounds and early driver UX are the falsifier; a contradicted ruling
amends through its owning artefact, never as an untracked tweak).

- **Q5, human moments: one typed "waiting on you" queue, drained on the
  maintainer's schedule.** Entry types: signature (binding decisions,
  including parked-blocked recommendations from Q3), retry approval
  (with residue: branch, attempt history, last activity), quarantine
  release (claim failed verification, never auto-retried), feedback
  triage (the intake lane), budget approval (a unit or wave hit its
  spend cap; the Q9 budget-exhausted moment), and park/unpark. Each
  type names its write path: signature entries are display-only routing
  to the maintainer's own signing flow; feedback triage promotes
  through the already-sanctioned verbs; retry, quarantine, park/unpark,
  and budget rulings are recorded facts the driver obeys, whose
  sanctioned verbs do not exist yet and must be specified by the
  write-authority decision task 6 enqueues before
  `todo.console-signed-widening` implements them. Nothing here is a
  console action on the driver, so a ruling survives restarts and
  appears in history. Tier split: binding-tier items interrupt by
  queueing; local-tier decisions land agent-side with receipts and
  never block a wave. No free-form command surface exists anywhere in
  the console.
- **Correction to task 1's assumption, from the Q2 ruling.** Lease
  facts (holder, grant, expiry, renewal) are cairn truth: their matrix
  rows name cairn queries, not a proposed driver contract. Only live
  execution state (session liveness, driver presence, run activity)
  names the versioned driver contract. Driver-absent, driver-idle, and
  driver-crashed stay distinct rendered states, and for an expired
  lease with no live driver the console derives staleness from the raw
  facts and an explicit observation time and renders it as stale and
  unclassified, never promising a terminal outcome fact that was not
  recorded.

## Mockup rounds (scheduled 2026-08-04)

Agreed at the close of the orchestration grill; protocol amended in
session on 2026-08-05 at the maintainer's direction: a round is worked
through with the maintainer rather than rendered for a verdict.
Alignment happens in the session, a disagreement amends the material
there or routes through the owning artefact per the brief's
ratification proviso, and the round is done when the maintainer says
it is aligned. No separate accept or reject step exists.
Round 1: the state-source matrix and the journeys (tasks 1 and
2), settling the planned, ghost, declared, buildable vocabulary (task
4). Round 2: the required mockup scenarios (task 3), including the
lease and granularity screens handed to
`todo.parallel-dispatch-granularity` as evidence (task 5). Round 3:
rejections reworked and the webui write-authority decision authored
(task 6).

### Round 2 ruling: prototype first (2026-08-06, maintainer in session)

The maintainer redirected round 2 in session (recorded with the full
vision in the brief's dated amendment, `studio/orchestration-console-brief.md`):
the primary register is plain language with progressive disclosure, the
guided creation journey is the flagship, and the path runs to a
prototype the maintainer tests as a user on a demo project rather than
through further hypothetical scenario mocks. This subsection is
provisional under the ratification proviso; the maintainer ratifies the
amended scope at the round-3 close-out. Consequences for this unit's
task list, recorded here as the owning artefact:

- Task 3's scenario list is amended, provisionally. Built and put
  before the maintainer in session, each with its exact standing: the
  guided creation journey (`orchestration-guided-journey.html`, the
  only screen the maintainer called aligned); driver connected with
  runs, leases, and a dispatch preview
  (`orchestration-plan-dispatch.html`, called right direction, not
  aligned); return and orient (`orchestration-return-orient.html`,
  called good as the reactive layer, density critique standing); mixed
  repository (`orchestration-mixed-repository.html`, not individually
  ruled on, kept as grammar evidence). Deferred to
  design-against-prototype-feedback rather than mocked:
  decision-to-consequence as its own screen (its consequence sentence
  is rendered inside the dispatch preview's held list), the
  driver-states four-up, and the narrow layout.
- Task 5 is partially served and stays open: claim identity, holder,
  expiry, and renewal render on the mixed and return screens as an
  expired held claim (`r-041`, residue rows, no outcome recorded,
  stale and unclassified), and the no-claim contrast is carried by the
  backlog's `no lease recorded` cross-check line; write-set
  disjointness is stated in the dispatch preview, but an overlap case
  is not yet rendered and stays owed to
  `todo.parallel-dispatch-granularity` with the round-3 close-out.
- Task 6 grows one input: the write-authority decision must also rule
  the run verb (run as a recorded ruling the driver obeys, or not at
  all); the brief amendment carries the collision.
- Acceptance is read against this ruling: the driver-states
  distinction remains demonstrated (the lamp vocabulary strip on the
  mixed and return screens), not as a dedicated four-up screen.

### Round 3 record (2026-08-06, worked through in session)

Provisional under the ratification proviso until the maintainer closes
the round as aligned:

- Task 6 discharged by enqueue: `dec.webui-write-authority` is authored
  and was accepted on 2026-08-06 (binding, nodes `cairn.ui` and
  `cairn.root`). It follows the `dec.control-plane-programme`
  supersession precedent against `dec.webui-design-authority`, carries
  every surviving obligation in its clause 5, legislates the four J6 gap
  verbs as recorded facts under one CLI noun (`cairn ruling
  retry|release|park|unpark|budget`), and rules run in: a recorded
  ruling the driver obeys, commit-pinned plan identity, stale plans
  declined and never dispatched, no console execution. The `ruling`
  family is additive: `dec.control-plane-programme` clause 3's existing
  grant (`cairn todo set`, `cairn feedback`, and the paired `cairn todo
  link` and `cairn todo unlink` when their schema lands) stays in force,
  because this record refines that decision rather than superseding it.
  The signature itself stays a separate maintainer act in the queue;
  until it lands the console keeps exactly that clause 3 grant, records
  no ruling, and every specimen labels run as not wired.
- Task 5 residue discharged: the write-set overlap case renders in
  `orchestration-plan-dispatch.html`'s held list (a unit queuing behind
  a wave member's claim because both would change the shared
  `docs/registries/` prefix, which is all a phase-0 preview may name
  (`dec.rung-three-coordination-substrate` clause 5), plain register, no
  new scenario
  mock), and the handoff is recorded in
  `todo.parallel-dispatch-granularity` under "Mockup evidence received".
- Task 3 under the prototype-first ruling:
  `todo.guided-console-prototype` is scaffolded on `cairn.ui`, scoped to
  the creation journey only (describe, the map forms visually, the
  grill drains doubts, run), targeting Docker on the maintainer's cloud
  server with the maintainer's own harness; acceptance is the
  maintainer testing a demo calculator project as a user and the
  recorded feedback driving the next design round.

Remaining for the round-3 close-out: the maintainer ratifies the
amended round 2 scope (the provisional subsection above) and this
round's record, and says aligned. That act moved to
`todo.console-round-three-closeout` on 2026-08-10, which
`todo.console-signed-widening` now declares as its blocker: no
orchestration console implementation starts before the maintainer says
aligned.

### Tooling (impeccable, installed 2026-08-05)

The impeccable skill pack (github.com/pbakaus/impeccable) is installed
checkout-local: `npx impeccable install --scope=project
--providers=claude` writes `.claude/skills/impeccable/` (gitignored by
a tracked entry; rerun that one command in each fresh checkout), and
the guarded hook wiring in `.claude/settings.local.json` is tracked.
The checkout-local exception to the tracked-skills policy is recorded
in `.claude/skills/README.md`. Vendoring was deliberately rejected on
different grounds than first recorded: the payload is 147 third-party
files that update upstream (`npx impeccable update`), and the repo's
skills policy tracks first-party operational assets, not third-party
payload, so the pack stays a recorded checkout-local exception. (An
earlier claim that the pack's markdown trips the repo's em-dash hook
was checked and is false: that hook matches only `*.md` files, and the
pack's markdown carries none.)

Round mapping, subordinate to repo authority (`docs/design-system/`
tokens and components stay canonical; any DESIGN.md impeccable writes
defers to them; the console is product lane per
`dec.marketing-visual-world`):

- Setup, once per checkout: `/impeccable init` (product context:
  audience and confirmed voice; init does not ask for visual
  direction) then `/impeccable document` (derive design context from
  the incumbent system; confirmed visual anti-references belong
  here). Both outputs (PRODUCT.md,
  DESIGN.md, `.impeccable/`) are generated, gitignored, and
  non-authoritative: token and font values derive from
  `docs/design-system/`, any conflict resolves toward it by
  regenerating, and durable vision content graduates into the tracked
  brief under `studio/` (task 2), never into these files.
- Round 1 (matrix, journeys, vocabulary): `shape` for the journeys,
  `distill` for the state grammar, `clarify` for the settled
  vocabulary.
- Round 2 (scenario mockups): explicit static-screen requests per
  required scenario (`craft` is a deprecated alias in the pack and adds
  nothing), `onboard` for the three driver empty states, `harden` for
  edge and error states, then `critique` and `audit` before each
  maintainer review.
- Round 3 (rework and polish): `polish`, with `quieter` whenever a
  screen drifts toward stock SaaS boldness; `typeset` and `layout` only
  within the token and font authority.
- Off-limits on the product lane: `bolder`, `overdrive`, `delight`
  (Calibrated Instrument: neutral chassis, signal colour only where a
  move is needed). `live` mode waits for implementation in
  `todo.console-signed-widening` and must respect
  `dec.webui-write-authority` clause 5.
- The pack's deterministic detector rules complement
  `scripts/check-design-tokens.sh` and `scripts/check-a11y.sh`; run
  them on mockup HTML only, not on `src/ui_assets`, until
  implementation starts.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; gates todo.console-signed-widening.
