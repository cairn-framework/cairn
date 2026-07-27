---
id: res.cairn-oax-skill-promotion
nodes:
  - cairn.kernel.cli
date: 2026-06-27
sources: [src.cairn-oax-skill-promotion-eval]
---
# Which managed cairn-* skills earn a place in the `cairn init` install pack

Evidence for bead cairn-oax. `cairn init` compiles a curated skill pack into the
binary via `include_str!` (cairn-dev plus references, cairn-explore, cairn-propose,
cairn-apply, cairn-archive) and emits it into a fresh repo as the loop on-ramp.
The bead asks which of seven GENERAL-capability personal managed skills should be
promoted into that pack, "eval-harden before publish".

## Adversarial debate

Two steelman positions were run (reformer vs conservative). They converged on:
promote `cairn-first-state-investigation` (unanimous, highest leverage), keep
`cairn-verify-shipped-before-next-steps` personal (unanimous, bound to this repo's
own history). The conservative surfaced a hard disqualifier for
`cairn-provenance-coverage`: it tells agents to wire decision pointers, which the
cairn repo itself deliberately does not do (dec.adopt-cairn-dev-loop), so shipping
it would broadcast a convention the reference repo contradicts. Both flagged that
several candidates overlap skills already in the pack.

## Eval: marginal lift over the SHIPPED pack, not over nothing

A first pass compared the skill against no skills and showed a large but
misleading lift. The decision-relevant baseline is the five-skill pack, which
auto-loads in every `cairn init`'d repo. The eval was rerun on a fresh
`cairn init`'d test bed (real nodes shop.auth and shop.cart, one accepted
decision with rationale) across two scenarios, comparing the pack against the
pack plus `cairn-first-state-investigation`.

Findings (directional, small sample):

- Provenance question ("what decisions shaped shop.auth"): the pack alone
  triggered `cairn-explore` but never used `cairn rationale`; it fell back to
  reading the decision file by hand. With the extra skill the agent went straight
  to `cairn rationale`, the purpose-built provenance command.
- Health-check question: the pack alone already covered it well via
  `cairn-explore` (status, lint, scan). The extra skill added no lift.

Root cause, confirmed in the text not just the runs: `cairn-explore` is the
pack's "query project state" skill, but its command list is structure and health
only (status, get, neighbourhood, files, islands, lint, check, ui). It does not
mention `cairn rationale`, `decisions`, `research`, or `sources` at all. So the
only durable, non-overlapping value of `cairn-first-state-investigation` over the
shipped pack is the provenance and decisions query path, plus the caveat that the
graph is not a source-symbol index.

## Conclusion

Shipping a fourth "state" skill that largely duplicates `cairn-explore` would add
trigger surface and a second skill saying similar things for little gain. The
proportionate move is to fold the genuine non-overlap (the provenance and
decisions commands, and the source-index caveat) into `cairn-explore`, which
already ships. The other six managed skills stay personal: redundant with the
pack, niche to this repo's spec or history, or contradicting an accepted decision.
