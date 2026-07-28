---
id: dec.product-perimeter
nodes:
  - cairn.root
  - cairn.reconcile
  - cairn.sse
status: accepted
date: 2026-07-28
informed_by:
  - res.decision-accumulation-cairn-root
  - res.messaging-workshop
  - res.cairn-identity
  - res.gas-city-cairn-integration
  - res.herdr-plugin-feasibility
  - res.cairn-domain-expandability
supersedes:
  - dec.agent-first-positioning
  - dec.cairn-identity
  - dec.no-orchestrator
  - dec.herdr-dashboard-integration
  - dec.domain-expandability
  - dec.simplify-cut-sse
related:
  - dec.task-tracking-authority
  - dec.artefact-layout-authority
  - dec.spec-authority-retirement
  - dec.marketing-visual-world
  - dec.close-blueprint-drift
revisit_triggers:
  - "a third-party case study or testimonial corpus materialises and changes the proof strategy"
  - "the brownfield first-run gap closes through a one-step first map and permits stronger time-to-value claims"
  - "a competitor occupies the free bidirectional-reconciliation slot"
  - "a cairn-specific scheduling primitive emerges that cannot be expressed outside cairn, or community demand for a zero-dependency cairn orchestrator exceeds integration complexity"
  - "a non-code reconciler is proposed for the distribution and extension band"
  - "a future decision revisits the public map framing itself"
---
# Product perimeter: agent-first map, domain-agnostic kernel, external orchestration

## Context

`cairn.root` carried six accepted decisions describing what cairn is and what
it refuses to ship. They were taken separately over three months and each
restated the others to stay coherent: positioning depends on identity, identity
depends on the orchestration boundary, and the boundary is what decided that a
dashboard, an event-stream client, and a scheduler all live outside cairn.

`res.decision-accumulation-cairn-root` holds the measurement and the precedent.
This decision consolidates the perimeter. It changes no behaviour, reopens
nothing, and revives no scope that the superseded decisions had already
retired.

## Decision

### 1. Public identity: a falsifiable map, authored by the agent

1. The coding agent is the primary author and maintainer of the map. The
   developer converses; the agent writes the blueprint, contracts, and
   decisions; cairn reconciles the map against real code in both directions and
   gates drift. Hand authoring stays supported as the secondary path, never the
   pitch.
2. The distinction is falsifiability: the map can be proven wrong, which is
   why it can be trusted.
3. Public copy uses named guarantees rather than generic reassurance, the Clean
   Exit Guarantee and the Nothing-Leaves-Your-Machine Guarantee, whose terms
   stay factually complete against what `cairn init` actually creates.
4. No scarcity or urgency devices, and never lead with "free".
5. Copy matches binary behaviour exactly. An overclaim is softened and filed as
   a product todo rather than shipped, and a stronger claim ships only after the
   capability it describes.
6. Every public surface, README, landing, and future docs, frames the agent as
   author. Wording that asks the developer to maintain artefacts by hand is a
   regression.
7. `res.messaging-workshop` stays the living document for future copy rounds.

### 2. Internal identity: a reconciliation controller

Cairn is publicly a map and internally a reconciliation controller. The
operative rule is the vocabulary: prose about cairn's mechanism stays consistent
with setpoint, sensor, error signal, boundary, and actuator, and no competing
metaphor is invented.

Public wording is not this decision's to set. `dec.marketing-visual-world` owns
`docs/index.html`, where the hero already prints
`Type: architecture reconciliation controller`. The 2026-07-03 two-sentence
rollout and the `docs/landing/index.html` path it protected are historical; that
file no longer exists.

### 3. Domain boundary: an agnostic kernel, pluggable reconcilers

1. The blueprint grammar and the two-chain model are domain-agnostic and are
   not specialised for code.
2. The code reconciler is the only domain-specific component, and the one v1
   reference implementation.
3. `cairn.reconcile` stays a pluggable, domain-agnostic `Reconciler` trait: given
   a node, it produces a content-addressable fingerprint of current state and a
   list of claimed sub-elements.
4. A non-code domain arrives by implementing and registering a new
   deterministic `Reconciler`, leaving the kernel, grammar, and artefact model
   untouched. Non-code reconcilers are deferred to the distribution and
   extension band, and non-code domains stay analysis-only until one lands.

### 4. Orchestration boundary: cairn is driven, not driving

1. Cairn does not own an orchestrator. It does not repeat, schedule, retry, or
   supervise; the invoking user or harness owns iteration.
2. `docs/integration-contract.md` is the documented contract for any external
   orchestrator: stable CLI surface, JSON schema per command, exit-code
   taxonomy, event envelope, and subscription primitive.
3. Cairn's own primitives run under any external orchestrator or none. Cairn
   maintains no workflow runner.
4. Cairn stays focused on architecture truth: blueprint, typed artefacts,
   two-chain topology, reconciler, drift gate, and interface hashes.
5. Artefact content is file-backed, and the only other work-item surface is the
   read-only beads view governed by `dec.task-tracking-authority`. The
   `ArtefactStore` storage-layer sketch in the superseded orchestration decision
   lapsed: no such trait exists in the source tree, and `dec.change-format-only`
   removed the pluggable `StateBackend` that stood next to it. Pluggable storage
   is not a live commitment and is not revived here.
6. Workflow lives in external skills and packs, never as cairn-owned
   scheduling.

Scope that the superseded decisions had already retired stays retired and is
not revived here: the `adapters/` reference-adapter directory, the Gas City
formula scheduler, and the governance pack. The surviving operational path is
the optional beads one, governed by `dec.task-tracking-authority`.

The SSE spike is the worked example of this boundary and its ruling stands:
`src/sse.rs`, its `pub mod sse` export, `meta/contracts/sse.md`, and the
`cairn.sse` Module node are deleted, because the module had zero internal
callers and the Gas City adapter said to consume it never existed. Subscribing
to an orchestrator's event stream is harness territory. The superseding of the
sse-specific ruling in `dec.close-blueprint-drift` stands, that decision's
treatment of `cairn.state` and `cairn.watch` is untouched, and
this decision names `cairn.sse` in its own `nodes:` so an accepted record still
covers the CH001 module-remove gate, which counts accepted decisions only. If a
future Gas City consumer needs SSE, a parser can be reintroduced behind a
contract informed by that consumer, not before.

Two consequences travel with the cut. The semver obligation is discharged: the
release carrying the removal of the public `cairn::sse` export bumped past
0.1.4 in `315f2cf`, so nothing is owed. The fixture rule is still live: the
webui eval ghost-node scenario keeps `cairn.sse` as a fixture-only id in
`harness/eval.mjs`, marked by a comment, and that must stay until the scenario
is rebuilt on a different id.

### 5. External consumers: read-only, and visibly so

The perimeter rule for any external consumer is short: it stays outside cairn
and reads only public, read-only surfaces. The Herdr dashboard pane is the one
consumer this project has ruled on, and the six points below bind it
specifically, not every future consumer.

1. It lives outside cairn as a checked-in script in `scripts/`, depends only on
   the python3 standard library, and calls only public JSON surfaces
   (`cairn lint --json`, `cairn status --json`, `cairn watch`).
2. It adds no blueprint node, no Rust code, and no cairn release artefact.
3. It re-derives ground truth and never caches it: every rendered snapshot
   reruns the commands and stamps collection time and a monotonic counter.
4. `cairn watch` deltas, the status poll, and a timer decide when to render,
   never what is displayed.
5. Optional overlay data from an external orchestrator, read from the
   `CAIRN_DASH_OVERLAY` environment variable, renders only under the explicit
   heading `ORCHESTRATOR CLAIMS (overlay, unverified)`, never inside
   the sidecar or consumer metadata. Cairn state and orchestrator assertions
   stay visibly separate, no orchestration capability is implied, and no
   overlay claim is verified.
6. Shape changes to `lint --json`, `status --json`, and `watch` are breaking
   changes for that dashboard, though not for cairn.

## Rationale

These six were consolidated rather than left in place because they form one
perimeter and were already cross-loaded: the orchestration boundary cites the
identity split, the dashboard rule cites the orchestration boundary, and the
SSE cut cites the orchestration boundary again as its reason. Read separately
they invite the reader to infer the perimeter; read together they state it.

Leaving `dec.no-orchestrator` outside the consolidation was considered, because
shipped assets cite it by name. It was rejected: its operative content is four
sentences, and half its body describes Gas City scope that its own 2026-07-25
revisit already lapsed. Carrying that forward under clause 4, with the lapsed
scope named as lapsed, is more honest than keeping a decision most of which no
longer applies. Citations continue to resolve, as `dec.todo-write-surface` does
under the CLI consolidation.

`dec.spec-authority-retirement` was in an earlier draft of this set and was
removed from it. Its subject is which surface is read for documentation
authority, not what cairn ships, and its own `related` list points at artefact
organisation and the dev entry point rather than at anything here. Folding it
would have been a decision taken for the count rather than for the lineage, and
it would have made this decision carry a documentation-routing clause that has
nothing to do with the perimeter. It stays accepted and unchanged.
`dec.simplify-cut-sse` took its place: it is explicitly `related` to
`dec.no-orchestrator`, it justifies itself by that boundary, and its retired
surface is already described in clause 4.

## Consequences

- The six named decisions are `superseded`. They keep their provenance and
  still count as provenance coverage for the nodes they name, including
  `cairn.sse`, which `dec.simplify-cut-sse` retains in its own `nodes:` to
  satisfy the CH001 module-remove gate.
- Assets citing `dec.no-orchestrator` by name (`.claude/skills/` loop mode and
  reconcile skills and their `tools/agent-pack/` twins, several todos,
  `src/cli/commands/pack_campaign.rs`, `src/state/mod.rs`, and prose tests that
  assert the disclaimer wording) continue to resolve and are not rewritten.
  Clause 4 is where the rule now lives.
- `cairn.root` loses six accepted decisions and gains one. `cairn.reconcile`
  replaces one accepted decision with one, so its count is unchanged.
  `cairn.sse` is not in the graph, so the accumulation check counts it not at
  all; it is named here only to keep an accepted decision over the module
  removal.
- This decision does not touch the marketing surface. `dec.marketing-visual-world`
  owns `docs/index.html`, including the hero line that now names the controller
  identity in public.
