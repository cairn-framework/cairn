---
node: cairn.kernel.artefacts
status: open
created: 2026-08-07
related: [dec.reviewer-panel-ratification]
---

# ratified_by vocabulary: add the reviewer-panel marker

`dec.reviewer-panel-ratification` lets a convergent binding decision be
accepted on panel receipts, with acceptance provenance carried in prose.
`parse_ratified_by` (`src/artefacts/registry/parse.rs:104`) accepts only
`machine`, so the panel regime has no queryable marker.

## Task

1. Add `reviewer-panel` to the `ratified_by` vocabulary; anything else stays
   CA046.
2. Render it in `cairn pending`, `cairn rationale`, and the decision panes,
   distinct from `machine` and from maintainer acceptance.
3. Extend the receipt-matching surfaces (`CAIRN_REVIEW_SUBJECT_UNMATCHED`,
   pending evidence) to match receipts bound to panel-ratified binding
   decisions, not only local-tier ones, so panel receipts stop rendering as
   unmatched.

## Acceptance

- A binding decision with `ratified_by: reviewer-panel` and valid receipts
  scans clean with its receipts matched.
- An invalid value still emits CA046.
