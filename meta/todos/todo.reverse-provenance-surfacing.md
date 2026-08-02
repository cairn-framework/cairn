---
node: cairn.kernel.query
status: open
created: 2026-07-31
related: [res.inversion-convergence-minutes]
---

# Surface reverse provenance edges on decisions

`res.inversion-convergence-minutes` row R4. Supersession is bilateral
(status flip plus `supersedes:`, scanner-validated), but `refines`
deliberately leaves the prior ruling active with no reverse pointer:
`refined_by`/`superseded_by` are not parsed fields and no query renders
them, so a qualified decision reads as unqualified authority even when
the qualifying decision sits in the same checkout. The 2026-07-31
incident (`dec.no-orchestrator` read as unqualified) mixed two failure
classes: this gap, and checkout staleness (the qualifying decisions,
`dec.north-star-continuous-loop` and `dec.product-perimeter`, were not
in the checkout at all). This todo owns only the first class.

## Task

1. Compute reverse edges at load (no new authored field): for each
   decision, the set of decisions whose `refines:`/`supersedes:` name it.
2. Render them in `cairn rationale`, `cairn get`, `cairn pending`, and
   the webui decision panes ("refined by dec.x, accepted 2026-07-29").
3. Advisory (Info) finding when an accepted decision has newer accepted
   refining decisions on an overlapping node set, so a reader citing it
   as sole authority gets a prompt to check the chain.
4. Tier per `dec.decision-ratification-tiers`: local if no wire shape
   changes; binding if the JSON contract moves (then bump schema versions
   and regenerate snapshots).

## Non-goal

Checkout freshness. A checkout missing newer decision files cannot
compute reverse edges for them; no rendering fixes that. Staleness
belongs to sync discipline (slate row R7) and, if evidence recurs, a
separate freshness warning unit (for example a status-surface note when
HEAD is behind origin/main); file that unit on its own evidence rather
than widening this one.

## Acceptance

- `cairn rationale cairn.root --json` shows `refined_by` on at least one
  decision in this repository.
- The advisory fires on a fixture with an accepted refinement and stays
  silent when the newer decision is proposed.
- Command-reference consistency tests pass; wire snapshots regenerated if
  bumped.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It is campaign step 6 for surfacing decision lineage in the console.
