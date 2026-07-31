---
id: res.inversion-convergence-minutes
nodes: [cairn.root]
date: 2026-07-31
sources: [src.reddit-gregerw-first-user-test, src.reddit-ontology-pointers, src.maintainer-design-threads-2026-07-30]
---

# Inversion convergence minutes, 2026-07-31

Session convergence over: the roadmap-artefact question, todo hierarchy and
dependencies, parallel-dispatch granularity, ghost-node discipline, decision
evolution visibility, repo hygiene, B-queue merge verification, and the first
external user feedback (Reddit, gregerw). Written against origin/main
782b9d5 (#555) read-only; the local main checkout was 32 commits stale during
the investigation, which is itself evidence for rows R5 and R7.

Ratification: row R1 was agreed individually in session on 2026-07-31, then
the maintainer ratified all rows ("Agreed, queue the landing"). R1's ruling
is recorded as `dec.todo-relationship-model` in the same change that lands
this file. The remaining rows land as todos, not decisions; each row's
eventual decision (where one is needed) is authored by its unit and signed
per `dec.decision-ratification-tiers`.

## Verified context (evidence, not proposals)

- The over-harness direction IS captured upstream:
  `res.overharness-design-threads` (threads a-d),
  `meta/changes/driver-v2-selection` (missions constructed from repository
  reads alone; no static queue), `dec.decision-ratification-tiers`,
  `dec.cairn-mission`, and `cairn pending`. `dec.no-orchestrator` is
  qualified by `dec.north-star-continuous-loop` and `dec.product-perimeter`;
  thread d parks the driver-in-repo question for a future decision.
- B-queue (the v1 driver at ~/repos/cairn-missions/driver.sh, outside this
  repository) work landed with named evidence tiers. Tier 1, queue
  completeness, proven: both remaining b-queue.txt lines md5-match
  b-done.txt and map to merged PRs #541 and #542; HANDOFF.md maps all
  twelve missions (W-batch, B1-B7, M1-M3) to merged PR and SHA; no
  closed-unmerged PRs sit in the #528-#555 window (number gaps are
  todo-sync issues). Tier 2, gates, proven: scripts/dogfood.sh builds the
  tree's own binary and runs lint plus hook all on every main push, green
  at 782b9d5 on 2026-07-31; HANDOFF records scan --strict exit 0 at
  8f0bbad0. Tier 3, semantic composition, attested rather than re-derived:
  HANDOFF's own verification line, plus PR #547 showing the process
  catching and correcting two false claims in a decision.
- The local main checkout's modified todo.contract-blueprint-staleness.md
  is the maintainer's deliberate uncommitted edit;
  mission-a-ratification.md orders sessions to leave that checkout
  untouched. Not residue. Three untracked todos there
  (brownfield-init-review-handoff, module-size-limit-adopter-scope,
  status-todo-path-double-prefix) are local-only and unlanded.
- Frontier is empty (zero ghost nodes) while 21+ todos are open: the
  forward programme is invisible to the graph's own buildability view.
- Decision reverse edges do not exist: supersedes/refines/related are
  parsed (src/artefacts/registry/types.rs); refined_by/superseded_by are
  not, so a qualified ruling reads as unqualified authority to anyone
  opening the old file or a stale checkout.

## Rows

| Row | Status | Ruling | Tier |
|---|---|---|---|
| R1 | AGREED, recorded | `dec.todo-relationship-model` records the ruling in the same change that lands this file: three typed todo edges (blocked_by, parent, related), stem-based reference identity, per-graph cycle detection, unresolved-blocker advisory semantics, and the roadmap as a DERIVED projection (an authored roadmap artefact type is declined; priority deferred behind a revisit trigger). Field and finding semantics live in the decision and are not restated here. Unblocks todo.todo-relationship-schema-decision (done) and the implementation chain. | binding (schema) |
| R2 | AGREED | Parallel-dispatch granularity named honestly as three rungs: (1) order, from R1 edges; (2) advisory overlap, the one-hop conflicts query (todo.node-overlap-conflicts-query), committed state only, explicitly NOT merge-safety; (3) merge-safety, declared write-sets/leases plus a shared multi-ref derived index (res.overharness-design-threads thread c successor; the B-queue ledger is its acknowledged single-writer prototype). Registries, cairn.blueprint, copy.toml, and wire snapshots are named serialisation hotspots needing explicit ownership regardless. Filed as todo.parallel-dispatch-granularity under the driver-v2 umbrella; no schema change in this row. | n/a (filing) |
| R3 | AGREED | Ghost-node discipline: a todo whose work implies new structure anchors to declared ghost node(s) at authoring time; todos on existing behaviour anchor to existing nodes. Frontier remains the structural roadmap axis; R1's todo DAG is the work axis. Filed as todo.ghost-anchored-todos-guidance; lands as guidance (agent pack and cairn-dev references), binding at signing time because pack content is binding surface. | binding (pack content, at its unit) |
| R4 | AGREED | Reverse-provenance visibility: compute refined_by/superseded_by reverse edges at load (no new authored field), render in cairn rationale, cairn get, cairn pending, and the webui decision panes; advisory finding when a decision cited as authority has newer refining decisions on the same nodes. Scope stated honestly: this qualifies an old ruling only when both files are co-present in the checkout; the 2026-07-31 incident additionally involved checkout staleness (the qualifying decisions were absent entirely), which no reverse edge can fix and which belongs to sync discipline (R7) and a possible freshness warning, tracked at its unit. Filed as todo.reverse-provenance-surfacing. | binding if wire bump, else local (at its unit) |
| R5 | AGREED | Inversion programme framing: the graph is the programme; drivers and harnesses are stateless executors of it. driver-v2-selection proceeds as proposed (external, read-only). Driver-in-repo stays parked per thread d and dec.product-perimeter; not reopened by this slate. Filed as todo.overharness-console-ux (pending + frontier + roadmap DAG as the webui's human steering surface), depends on R1. | binding (programme decision, at its unit) |
| R6 | AGREED, recorded | First external user evidence captured: src.reddit-gregerw-first-user-test and src.reddit-ontology-pointers (both verification: tracked over raw captures in docs/research/). Pain-to-backlog mapping: landing workflow opacity (copy pass in flight on copy/landing-plain-prose-pass), install weight (todo.init-ignore-scaffolding), post-install dead end and brownfield decision-extraction gap (todo.brownfield-decision-extraction, filed alongside). Ontology pointers recorded as unverified claims; no normative move. | local |
| R7 | AGREED | Hygiene triage (procedural, no decision): the maintainer's todo.contract-blueprint-staleness edit stays untouched until the maintainer lands it; the three local-only stray todos in the main checkout land or retire in a maintainer-supervised pass, not by an unattended session; tmp/ (Cairn Journey html) is the maintainer's keep-or-kill; the stale checkout syncs only after in-flight sessions land. Root cause is R5's imperative residue, not missing machinery. | n/a |

## The one genuine fork

R2 rung 3: how heavy the lease model gets (declared per-todo write-sets vs
derived node-closure; live coordination backend vs committed-state
baseline). Recommendation, recorded in todo.parallel-dispatch-granularity:
start with derived node-closure over committed state (zero new authoring
burden), promote to declared write-sets only on measured false-overlap
evidence.

## Post-ratification note on R5

The programme decision that unit eventually authors must explicitly
separate three ownerships rather than blur them: policy and control
(cairn: selection truth, dependencies, readiness, leases, dispatch
policy), dispatch (the driver: when to start work, retries, supervision),
and execution (the harness: how work runs). If cairn owns the first set,
it is the declarative control plane by construction; the decision must
say so plainly and declare `refines:` (or, at the maintainer's signing
call, `supersedes:`) against `dec.no-orchestrator`, not claim containment
by rewording.

## Dependency order

R1 (recorded) before R2 rungs 1-2 and before R5's console todo. R4
independent. R6 recorded here; R7 stays with the maintainer. R3 rides
with R5's unit or lands as guidance alone.

## External-AI ideation relayed post-ratification (2026-07-31)

Six items of ideation from other AI sessions arrived after
ratification, relayed by the maintainer who was explicitly unsure of
their placement. They are proposals pending triage, not maintainer
rulings; all six were recorded verbatim via `cairn feedback` in the
local intake seam. Placement of the actionable ones as CANDIDATES: the
programme-decision boundary wording
(investigation boundary vs actuation boundary) and the console intake
lane were folded into todo.overharness-console-ux; the derived-fact
evidence-metadata rules into todo.parallel-dispatch-granularity; the
bounded build/CI dogfood overlay filed as
todo.build-ci-observation-overlay (next horizon, not next unit); the
in-toto/SLSA receipts comparison filed as
todo.receipts-provenance-interop; the copy rule (never "code ontology";
say "architecture, intent and evidence graph") stays in the feedback
queue for placement in the voice guidance by its own unit.
