---
id: res.decision-accumulation-cairn-root
nodes:
  - cairn.root
date: 2026-07-28
method: primary
---

# What `cairn.root` accumulated, and what consolidating it costs

Measured at `1afd6ae` on `main`, while a loop iteration selected the
`CAIRN_DECISION_ACCUMULATION` finding.

## What was measured

`check_decision_accumulation` (`src/scanner/checks.rs`) counts
directly-attached decisions whose status is `Accepted`, once per node named in
a decision's `nodes:` list, and emits an Info finding when the count exceeds
the threshold (`DEFAULT_DECISION_ACCUMULATION_THRESHOLD`, 10). The check is
node-kind agnostic: moving a decision from a Module to the System node would
relocate the finding rather than clear it.

Accepted decisions per node, counted the same way the check counts them:

| Node | Accepted |
|---|---|
| `cairn.root` | 23 |
| `cairn.ui` | 10 |
| `cairn.kernel.cli` | 10 |
| `cairn.kernel.scanner`, `cairn.kernel.artefacts`, `cairn.kernel.query` | 6 |
| everything else | 5 or fewer |

`cairn.root` is the only node over threshold. The two nodes sitting exactly at
10 are the two the repository has already consolidated:
`dec.webui-design-authority` folded five `cairn.ui` decisions on 2026-07-28 and
`dec.cli-agent-workflow-consolidation` folded eleven `cairn.kernel.cli`
decisions the same day. Both landed as one decision per node in one PR, and
both edited each superseded target by setting its existing `status:` line to
`superseded` and nothing else. Neither added a `superseded_by:` field; the
parser never reads one.

## What the twenty-three are

Three groups account for fifteen of them, and each group is a lineage rather
than an arbitrary slice:

- Task tracking, seven: `dec.beads-task-layer`, `dec.bd-upgrade-plan`,
  `dec.bead-github-sync`, `dec.github-todo-sync-projector`,
  `dec.github-todo-issue-body-fidelity`, `dec.native-todos-first`,
  `dec.native-task-state-and-agent-guidance`. They answer one question, where
  work items live and who may write them, in four successive settings (beads
  primary, beads plus GitHub, native Todos primary, native Todos projected to
  GitHub). Later members restate earlier ones to stay coherent.
- Product perimeter, six: `dec.agent-first-positioning`, `dec.cairn-identity`,
  `dec.no-orchestrator`, `dec.herdr-dashboard-integration`,
  `dec.domain-expandability`, `dec.simplify-cut-sse`. They answer what cairn is
  and what it refuses to ship, the last three by working the boundary from the
  outside: a dashboard, a domain plugin, and an event-stream client that all
  turned out to live beyond it. `dec.spec-authority-retirement` was considered
  for this group and rejected: its subject is documentation authority, not
  product scope, and folding it would have been a choice made for the count.
- Artefact layout, two: `dec.artefact-organization-and-provenance` and
  `dec.artefact-filename-rule`. The second mechanically enforces a rule the
  first stated as policy, so they are one rule in two instalments.

The remaining eight are single-subject and unrelated to each other:
`dec.root-module`, `dec.revisit-trigger-correlator-deferred`,
`dec.cli-agent-workflow-consolidation`, `dec.architecture-modularity-verdicts`,
`dec.finding-coverage-strategy`, `dec.ghost-rule-tracking`,
`dec.spec-authority-retirement`, `dec.simplify-persist-module`.

## What consolidation does not cost

- Provenance coverage survives. `check_provenance_coverage`
  (`src/scanner/checks.rs`) builds its covered set from every loaded decision
  with no status filter, and `check_blueprint_change_decisions` accepts
  `Proposed | Accepted | Superseded`. A superseded decision still covers the
  nodes it names.
- Prose citations survive. `dec.todo-write-surface` is cited by `AGENTS.md`
  and was superseded by the CLI consolidation without amending that citation,
  which is the precedent for how far the ripple is expected to reach.
- `validate_decision_refs` (`src/artefacts/registry/validate/mod.rs`) is the
  only mechanical constraint on the operation: a `supersedes:` target that does
  not exist raises `CAIRN_DECISION_REFERENCE_UNKNOWN`, and one left `accepted`
  raises `CAIRN_DECISION_SUPERSEDES_STATUS`. Both halves of the edit are
  therefore checked.

## What consolidation does cost

Six live pointers name `dec.artefact-filename-rule` as the authority for the
filename rule and its CA038 enforcement: `docs/conventions.md` twice, both
tracked copies of `finding-codes.md` (canonical under
`tools/agent-pack/content/`, rendered under `.claude/skills/`), a doc comment at
`src/artefacts/registry/validate/filenames.rs`, and a test contract comment at
`tests/fixtures_smoke.rs`. Unlike the AGENTS.md precedent
these are shipped guidance and an enforcement site, so they are repointed at
the consolidating decision rather than left naming a superseded one.
`dec.artefact-organization-and-provenance` has no citations at all.

## What this does not settle

Fifteen superseded and two new decisions naming `cairn.root` leaves the node at
exactly 10, the threshold, matching `cairn.ui` and `cairn.kernel.cli`. The next
decision anchored on `cairn.root` re-fires the finding. Nothing here argues
that 10 is the right threshold for a node that legitimately aggregates
repository-wide policy, and nothing here consolidates the eight single-subject
decisions, which share no lineage and would have to be folded on the strength
of the count alone.
