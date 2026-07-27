---
node: cairn.root
status: done
created: 2026-07-22
---

# Agent Guidance Program

Umbrella for the Harness Engineering derived agent-guidance and agent-pack
work. It gives the maintainer one master backlog to orchestrate and keeps the
child units in a reviewed dependency order. It is not a Cairn scheduler:
sequencing lives here as reviewed prose and in each child's blocked or open
status, and an external harness drives repetition per `dec.no-orchestrator`,
`dec.loop-command-harness-model`, and `dec.native-todos-first`.

## Goal

Ship the reconciled agent-guidance strategy: a measured baseline, a canonical
harness-neutral pack, one logical `cairn-dev` entry with an explicit loop mode,
apply, proof, and authority guidance, evaluated treatment, an OMP adapter, and
a campaign-reconciliation step, each landed as its own reviewed unit.

## Waves

Waves grouped children by dependency tier. All sixteen are terminal; this is
the delivered ledger.

Wave 1 (done, 2026-07-24):

- todo.agent-guidance-provenance (done, PR #451; `dec.unified-cairn-dev-entry`
  accepted in PR #452)
- todo.agent-guidance-baseline (done, PR #450; `res.agent-experiment-linklint`
  and the `archive/strongholds/agent-guidance-baseline/` bundle)
- todo.agent-pack-canonical-foundation (done, PR #453; `tools/agent-pack`)
- todo.agent-context-bundle (done, PR #449)

Wave 2 (done, 2026-07-25):

- todo.agent-context-bundle-evaluation (done, PR #456; compose existing verbs,
  do not build `context_projection_v1`)
- todo.agent-guidance-apply-proof-authority (done, PR #459)
- todo.agent-guidance-router-playbooks (done, PR #460; `cairn-dev` loop mode is
  the canonical loop authority, `/cairn-loop` is adapter transport)

Wave 3 (done, 2026-07-26):

- todo.agent-pack-claude-bootstrap (done; split under the sizing rule, PR #462)
  - todo.pack-install-lifecycle (done, PR #466)
  - todo.pack-init-delegation (done, PR #467)
  - todo.pack-campaign-resolver (done, PR #468)
- todo.agent-guidance-campaign-reconciliation (done, PR #461;
  `dec.loop-reconcile-step`)
- todo.spec-authority-retirement (done, PR #469; `dec.spec-authority-retirement`)

Wave 4 (done, 2026-07-27):

- todo.agent-guidance-treatment-evaluation (done, not executed; PR #470 landed
  the evidence boundary as `res.agent-guidance-treatment-evaluation-blocker`).
  Its sealed materials and authenticated worker epoch were kept outside the
  repository by design, so no session could reach the terminal verdict. Retired
  under `dec.pack-publication-on-activation-evidence`.
- todo.agent-pack-omp-adapter (done, PR #471; validated against live OMP 17.1.3,
  `res.pack-omp-adapter-validation`, `dec.pack-adapter-roots`)

Wave 5 (done, 2026-07-27):

- todo.agent-pack-omp-publication (done, PR #483). OMP is
  documented as a supported adapter in `docs/commands.md` and
  `docs/agent-setup.md`, on live validation alone once
  `dec.pack-publication-on-activation-evidence` retired the treatment gate.

Out of programme: todo.blueprint-authorability-eval keeps its own dependency on
todo.example-corpus-scan-assertions.

## Outcome

Sixteen units, all terminal; the campaign closed on 2026-07-27. The two rulings
that closed it are `dec.pack-publication-on-activation-evidence` (adapter
publication gated on live-harness validation alone, with the pack's supported
claim narrowed to the activation effect the baseline measured) and
`dec.pack-adapter-roots` (an adapter is a pack root; one install owns one
adapter).
