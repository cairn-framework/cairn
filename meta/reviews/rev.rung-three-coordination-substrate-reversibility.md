---
node: cairn.root
date: 2026-08-07
reviewer: anthropic/claude-fable-5/contestedness-reversibility
review_type: agent_introspective
subject_hash: sha256:e06d95857bac5b640f492d3e3b1b70514a86b695cafe2efa53a0604e84b6c1aa
lens_prompt_hash: sha256:45136bbc19a4732ebacc4bd194791674e1266a4ae11c8fd51bfcfae9c7c4d698
---

# Receipt review: rung three coordination substrate (reversibility lens)

Receipt-grade review of `dec.rung-three-coordination-substrate` under
`docs/agent/lenses/contestedness-reversibility.md`: cost to undo each clause if
wrong in six months.

## Assessment

1. **Clause 1**: cheap. The preimage is versioned by its magic line; a
   `cairn-plan-v2` preimage is one refining decision, and no stored data
   depends on v1 digests beyond consumed rulings.
2. **Clause 2**: the contested one. Dropping Q9's cross-clone
   detection-after-sync is the least reversible move in the record, which is
   why it carries the debate. Mitigations verified in the reviewed content:
   facts are self-contained with a `format` field, unknown formats fail
   closed, and migration to a synced store is a replay in filename order, so
   the family-local choice defers rather than spends the capability. The
   revisit trigger (first multi-clone driver use) is recorded on the decision.
3. **Clause 3**: cheap in the promoting direction (phase 2 is a planned
   refining decision) and honest about the cost direction that is not cheap:
   phase-0 wave narrowing is a throughput cost, not a correctness debt.
4. **Clause 4**: requiring `evidence_class` now is the reversible direction;
   relaxing later is trivial, requiring later strands every existing fact.
   Convergent on this lens's own test despite the correctness lens's flag.
5. **Clauses 5 and 6**: copy and naming, cheap.

## Verdict

PASS

The one costly-to-reverse choice (clause 2) carries a recorded debate, a
replay-based migration path, and a named revisit trigger. Nothing else in the
record is expensive to undo.

## Attestation

Re-attested 2026-08-07, same session: the acceptance prose line was set after the first hash; the reviewed clauses are byte-identical and this receipt attests the recomputed subject manifest in the frontmatter.
