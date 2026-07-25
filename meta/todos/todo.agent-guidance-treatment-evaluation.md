---
node: cairn.kernel.cli
status: blocked
created: 2026-07-22
---

# Agent Guidance Treatment Evaluation

## Priority

P2. Evaluate before expanding publication of the revised pack.

## Depends on

`todo.agent-guidance-baseline`,
`todo.agent-pack-claude-bootstrap`,
`todo.agent-guidance-router-playbooks`, and
`todo.agent-guidance-apply-proof-authority`.

## Method

Use the baseline protocol and the same worker epoch. If the model or harness
changes materially, rerun every comparator. Keep run order randomised and the
grader blind.

Provision every effectiveness arm through the same Claude lifecycle and
harness setup, varying only the pack contents. Keep the one-command greenfield
and brownfield bootstrap as a separate operational adoption smoke.

Evaluate incremental arms rather than only the full pack:

1. no guidance content with the same installed capability surface;
2. the current shipped pack;
3. compact router only;
4. router plus each JIT task reference;
5. apply/proof/authority guidance on implementation tasks, separately from
   retrieval interventions;
6. the full revised pack.

If `todo.agent-context-bundle-evaluation` justifies a new query surface and its
separate implementation todo ships it, evaluate that surface as its own
intervention. When the surface is adopted, reconciliation adds its
implementation todo as a dependency of this evaluation; if this evaluation is
already `open` at that point, reconciliation first sets it back to `blocked`,
records the new dependency, and reopens it only after verified delivery. Do not
fold a recommendation-only surface into these arms.

## Validity gate

Keep all assigned runs in the intention-to-treat primary analysis. Record
availability, retrieval, invocation, and relevance as fidelity measures; any
engaged-run analysis is secondary. Decide `revise` from development evidence
only and keep iterating there; open a sealed confirmation set only when
revisions are complete, for the preregistered terminal comparison, and make the
`retain` or `remove` decision from that untouched evidence. If a further
revision proves unavoidable after a confirmation comparison, seal a fresh
confirmation set for the next terminal comparison; never reuse a holdout the
pack was tuned against.

## Acceptance

- Report outcome, claim-proof, architecture/ownership, retrieval quality,
  token composition, time-to-correct-file, human relay, retries, and cost.
- Preserve raw runs, failures, variance, and displaced complexity.
- Give each router, playbook, and apply intervention a retain, revise, or
  remove verdict.
- Record the accepted pack composition before OMP publication and the final
  revised-content smoke.
- The terminal verdict comes from a sealed confirmation set opened once, when
  revisions are complete: `retain` (record it as a decision the owner accepts,
  set this unit `done`; reconciliation opens publication only after that decision
  is `status: accepted`) or `remove` (record an accepted superseding decision,
  set `done`, publication dropped per the umbrella). A `revise` is a
  development-evidence call and is not terminal, yet this evaluation round still
  lands `done` (it completed its evaluation): with `cairn todo new <slug> --node
  cairn.kernel.cli`, author a revision follow-up that lands the pack changes and
  a successor evaluation round that lists that follow-up in its `Depends on`, add
  both to the umbrella Waves, record the verdict and required changes with `cairn
  decision new`, and set this round `done`. The campaign continues through the
  successor round, never by reopening a landed unit (the landing contract sets
  each landed unit `done`); each round seals a fresh confirmation set for its
  terminal comparison. `todo.agent-pack-omp-publication` stays unresolved until a
  round returns `retain` (which opens it) or `remove` (which drops it via the
  umbrella's superseding-decision path).

## Blocked

This iteration cannot execute the development arms because the authenticated
worker epoch and replay fixtures are unavailable here. See
`res.agent-guidance-treatment-evaluation-blocker`; a future owner must restore
that environment and execute this prescribed development evaluation before any
sealed confirmation verdict.
