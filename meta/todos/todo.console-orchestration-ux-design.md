---
node: cairn.ui
status: open
created: 2026-08-03
related: [dec.orchestration-placement, dec.control-plane-programme, dec.webui-design-authority, todo.console-signed-widening, todo.console-state-legibility, todo.driver-in-repo, todo.parallel-dispatch-granularity]
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
orchestration surface, blocked on this unit).

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
   render for active, waiting, or blocked.
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
   `dec.webui-design-authority` clause 5's foreclosed vision-patch loop.
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
   `dec.webui-design-authority` clause 4 says the webui stays read-only
   and its line 28 revisit trigger fires when scope grows beyond
   read-only exploration. That trigger has fired:
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

## Sequencing

Tasks 1 to 5 need no signature and no code. Task 6 ends in a maintainer
signature. `todo.console-signed-widening` is blocked on this unit and
owns every line of orchestration console implementation the mockups
specify. `todo.console-state-legibility` is independent and can run first
or in parallel: it fixes the shipped read-only surface and makes the
mockup comparison honest by removing "the console showed nothing useful"
as a confound. `todo.driver-in-repo` is open (the placement decision
was accepted 2026-08-04) and does not wait on this unit; the lease
ruling is recorded in `todo.parallel-dispatch-granularity`, whose rung
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

Agreed at the close of the orchestration grill. The interviewer renders
static screens; the maintainer accepts or rejects each; a rejection
routes back through the owning artefact per the brief's ratification
proviso. Round 1: the state-source matrix and the journeys (tasks 1 and
2), settling the planned, ghost, declared, buildable vocabulary (task
4). Round 2: the required mockup scenarios (task 3), including the
lease and granularity screens handed to
`todo.parallel-dispatch-granularity` as evidence (task 5). Round 3:
rejections reworked and the webui write-authority decision authored
(task 6).