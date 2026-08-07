---
node: cairn.kernel.artefacts
status: blocked
created: 2026-07-31
related: [dec.decision-ratification-tiers, todo.review-gate-machine-check, todo.local-gate-attestation]
---

# Receipts vocabulary: compare against in-toto/SLSA before it ossifies

Unratified candidate from the slate's post-ratification intake
(res.inversion-convergence-minutes), as relayed: "Before the ratification
receipt/subject_hash vocabulary ossifies, compare it against
in-toto/SLSA provenance shape; record deltas as research. Cheap now,
expensive later."

The receipt protocol (dec.decision-ratification-tiers: two committed
Review receipts bound by subject_hash to a canonical manifest) is young
and cairn-specific. in-toto attestations and SLSA provenance solve the
adjacent problem (binding an artefact to evidence of how it was
produced) with an ecosystem behind them. A deliberate comparison now
costs one research artefact; a vocabulary migration after adopters
depend on receipts costs a binding supersession.
The comparison feeds the evidence-format design that
`todo.review-gate-machine-check` and `todo.local-gate-attestation`
consume.

## Task

Author a research artefact comparing the shipped receipt/subject_hash
shape against the in-toto attestation and SLSA provenance model:
field-by-field deltas, what each vocabulary can express that the other
cannot, and whether an export/import mapping is cheap. No schema change
in this unit; if the comparison argues for one, enqueue a decision via
`cairn pending`.

## Acceptance

- A research artefact with the delta table and a recommendation
  (keep as-is, add a mapping, or amend the vocabulary), citing the
  external specs as sources with verification stated honestly.
- Any recommended change lands as an enqueued proposed decision, not an
  edit in this unit.

## Mission disposition

2026-08-02: blocked against dec.cairn-mission. Serves investigable. It remains maintainer-triage-gated with no declared blocker by design.

2026-08-07 audit (todo.roadmap-assumption-audit): res.chatgpt-issue-audit says unblock (no real blocker); the 2026-08-02 note here says maintainer-triage-gated by design. Tension recorded; stays blocked pending the maintainer's call.
