---
id: res.harness-engineering
nodes:
  - cairn.kernel.cli
  - cairn.kernel.query
sources:
  - src.harness-engineering
method: secondary
date: 2026-07-23
---

# Harness Engineering mechanisms for Cairn loop guidance

Secondary distillation of the pinned Harness Engineering anthology at commit
`226c8d35fb6ea3ed55467753dba6dea2b5fd5778`, limited to the six reviewed files
in `src.harness-engineering`. Every claim below is a mechanism observation
with an evidence anchor and an explicit limit. The anthology is advisory
corroboration only; Cairn's accepted local decisions remain the authority for
allocation of semantic versus runtime ownership.

## 1. Whole-job accountability

**Mechanism.** Delegate the outcome at the highest safely supported level and
give one primary trajectory responsibility for retrieval, investigation,
method choice, proof, and delivery. Humans retain direction, judgment, and
consequential authority.

**Evidence anchor.** `docs/whole-job/README.md` (outcome ownership; primary
trajectory as integration point; lifecycle closure).

**Limit.** One primary trajectory is integration ownership, not solitary
execution. Parallel discovery, adversarial review, people, and subagents may
contribute independent evidence; the primary integrates their results. The
source does not forbid subagents or parallel discovery, and it does not
establish that Cairn owns runtime topology.

## 2. Just-in-time routing

**Mechanism.** Keep durable knowledge searchable, expose what context exists
and where, and let the task retrieve the next relevant slice. Delivery is
phased: a terse grounding map; code, tools, failures, logs, and diagnostics in
the messy middle; role-specific review, proof, and delivery policy at landing.
Skills teach approaches; runbooks preserve versioned contracts (intent,
preconditions, safety, evidence, escalation, rollback). Thin automation points
to that repository-owned contract.

**Evidence anchor.** `docs/just-in-time-context/README.md` (JIT routing; three
phases; skills versus runbooks; document presence proves only writing).

**Limit.** A unified entry is supportable as a route, not as an always-loaded
monolith. Writing or loading guidance does not prove mechanism success; the
anthology requires a fresh trajectory where the intervention was available,
retrieved or invoked, and relevant (`playbooks/improve-harness.md`). Document
presence alone is insufficient.

## 3. Continuous-loop contract

**Mechanism.** Continuous maintenance begins only from accepted direction and
answers five questions: maintained condition, drift signal, restoration proof,
autonomous-versus-approved authority, and durable state for the next
iteration. Retirement is a separate required runbook element. External
triggers (cron, events, tail calls) stay thin; the checked-in runbook supplies
intent, candidate selection, scope, proof, authority, recovery, and durable
state. Each iteration should terminate in a durable state.

**Evidence anchor.** `docs/continuous-maintenance/README.md` (five questions;
thin trigger; repository-owned runbook; durable world state; claim-matched
proof).

**Limit.** The thin-trigger / repository-contract split is consistent with
Cairn's local harness-ownership decision, but the anthology does not itself
establish that allocation. Scheduling, repetition, retry policy, concurrency,
and process or workspace lifecycle remain external concerns under
`dec.no-orchestrator`.

## 4. Claim-boundary proof

**Mechanism.** Proof has two layers: target-native checks protect internal
contracts, while the user or operational journey establishes the accepted
outcome. Authority and proof remain separate at each boundary: canary proof
does not grant production authority, and production access does not establish
deployed-system health. Mechanism claims require a fresh equivalent trajectory
and confirmation that the intervention was available, retrieved or invoked,
and relevant. One before-and-after run supports only a bounded observed-
condition claim.

**Evidence anchor.** `playbooks/improve-harness.md` and
`docs/authority/README.md`.

**Limit.** The anthology does not prove that a unified `cairn-dev` entry
improves quality or owns runtime authority; those allocations come from
Cairn-local decisions.

## 5. Manifest integrity

**Mechanism.** The anthology's `sources/scripts/validate_manifest.py`
implements precise path, ownership, and content checks for its source
manifest.

**Evidence anchor.** `sources/scripts/validate_manifest.py`.

Observed behaviour, stated only as implemented:

1. `repository_file` rejects a symlink at the referenced leaf path, resolves
   strictly, rejects resolution outside the repository root, and requires a
   regular file. This is narrower than rejecting every path that contains any
   symlink.
2. Snapshot artefacts under `sources/raw/` or `sources/twitter/` may have only
   one source owner per snapshot artefact path. Linked evidence and repository
   notes take different branches; the validator does not enforce one owner per
   arbitrary destination.
3. SHA-256 comparison is performed for snapshot artefacts in the
   snapshot-evidence branch. Linked, private-review, and repository-note
   evidence are not content-hashed by this function.
4. Every non-guide entry under `sources/raw` and `sources/twitter` must appear
   in the ownership map, or validation fails as unowned.

**Limit.** These checks ground provenance and integrity claims for the
anthology's own source inventory. They do not justify orchestration, runtime
authority, or quality claims.

## Synthesis boundary for the refining decision

Taken together, the five mechanisms support a thin repository-owned one-
iteration procedure under an explicit route, with claim-matched proof and
honest evidence limits. They do not authorize:

- moving scheduling, repetition, session or runtime lifecycle, retries, or
  parallelism into Cairn;
- treating the anthology as proof of quality improvement;
- treating document presence or pack loading as mechanism success;
- expanding public query surface or runtime authority from the 2026-07-23
  baseline.

Those limits are load-bearing inputs to the proposed refining decision
`dec.unified-cairn-dev-entry`.
