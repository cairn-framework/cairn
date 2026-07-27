---
id: dec.pack-publication-on-activation-evidence
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-07-27
informed_by:
  - res.agent-experiment-linklint
  - res.agent-guidance-treatment-evaluation-blocker
---
# Pack publication is gated on live-harness validation, not on a treatment verdict

## Context

The agent-guidance programme gated publication of the OMP pack adapter on a
six-arm randomised trial with blind grading and a sealed holdout. The
prerequisites for that trial, an authenticated worker epoch and the sealed
confirmation prompts with their ground truth, were kept outside this repository
by design, so no session run from this checkout can produce its verdict
(`res.agent-guidance-treatment-evaluation-blocker`).

Meanwhile the three-arm baseline had already measured the pack against
target-only navigation and against a bare Cairn query surface
(`res.agent-experiment-linklint`). That measurement is the evidence actually in
hand. The gate it does not satisfy appeared in todo bodies and in non-binding
proposed and research records, but no accepted decision ever imposed it.

## Decision

1. Publication of a pack adapter is gated on live-harness validation, which is
   what `dec.agent-pack-packaging` clause 2 already requires, and on nothing
   else. The treatment-evaluation verdict is removed as a publication
   precondition.
2. The pack's supported claim is narrowed to what the baseline measured: the
   pack reliably causes agents to use Cairn's query surface. It does not claim
   a measured improvement in answer quality.
3. The six-arm trial is not scheduled. Should an owner later restore the
   environment, it is authored as a fresh unit against the preserved protocol
   in `res.agent-guidance-treatment-evaluation-blocker`, not by reopening a
   landed todo.
4. `todo.agent-guidance-treatment-evaluation` and
   `todo.agent-pack-omp-publication` reach terminal disposition under this
   decision, releasing `todo.agent-guidance-program`.

## Rationale

The three-arm navigation baseline of 2026-07-23 measured a paired
pack-minus-Cairn quality difference of 0.06 points and a paired
Cairn-minus-target difference of 0.19 points, both against a preregistered
one-point threshold that neither reached. Over the same runs the pack arm
invoked Cairn in 16/16 runs where the Cairn-surface arm invoked it in 0/16, and
pack runs averaged 16.31 tool calls against 13.31, 47,034 input tokens against
37,229, and 109.1 seconds against 103.1.

So the activation effect is unambiguous and the outcome effect is already
measured as below the threshold. A further trial would refine the decimals on a
difference the baseline has already placed under the bar, at the cost of
prerequisites this repository cannot supply. The action that gate blocks is
documenting an adapter that already ships, is bound to the ownership ledger,
and is covered by acceptance tests, which is reversible by editing the same two
documents.

## Consequences

- The pack ships with an activation claim rather than an outcome claim.
- `dec.agent-pack-packaging` is unchanged and uncontradicted; this decision
  removes a gate that decision never imposed.
- This decision removes a publication precondition. It touches no integrity
  obligation: the adapter-conformance standard of `dec.unified-cairn-dev-entry`
  clause 9 stands unchanged, and "on nothing else" in ruling 1 means no further
  gate on publication, not a lowered conformance bar.
- Any future public claim that the pack improves answer quality requires new
  evidence, gathered under a freshly sealed confirmation set.
