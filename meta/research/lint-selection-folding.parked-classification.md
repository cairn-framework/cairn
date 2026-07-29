---
id: res.lint-selection-folding.parked-classification
nodes: [cairn.kernel.scanner]
date: 2026-07-29
method: primary
---

# Parked classification (item 1a): implementation evidence

Records what landing `todo.lint-selection-folding` item 1a established beyond
the ratified text. Three observations constrain later units.

## 1. Parking and deferral compose by exclusion

The ratified text leaves one intersection undefined: an Info finding that is
both decision-deferred (published `deferred_by`) and referenced by a `blocked`
todo's `defers:`. Three constraints meet there: item 1a parks every matching
Info finding; the acceptance requires every parked finding to render in full
naming its parking todo; item 1b's carve-out requires the decision-deferred
case to keep its inline annotation and collapsed rendering, untouched by this
todo. All three cannot hold at once for one finding, so parking yields:
`check_todo_defers` (src/scanner/todo_defers.rs) never sets `parked_by` when
`deferred_by` is present. The reference still counts as matched, so it is not
reported stale. Selection outcome is identical under either reading, because
both fields are independently non-selecting. Pinned by
`test_todo_defers_deferred_finding_stays_deferred_not_parked`.

The rule at the intersection is carried by `dec.parked-deferral-composition`
(proposed, informed by this research); this artefact records the evidence
only. Landing 1a also fires the second revisit trigger of
`dec.loop-selection-strict-green-fold` ("item 1a's parked classification lands
and the three composed selection rules need a single home"). Reviewed
2026-07-29: the single home exists, the Select ONE unit section of
loop-mode.md now states all three rules; the composition question at the
deferred intersection is routed to the proposed decision above.

## 2. Emission sites must stay textually anchored

`validate_spec_rule_coverage` detects a rule's enforcement by scanning non-test
source for the code literal immediately preceded by one of `error(`,
`warning(`, `info(`, `error_finding(`, or `code:` (spec_rule_coverage.rs,
`is_emitted`). A constructor helper that takes the code as its first argument
(`finding("CAIRN_X", ...)`) hides the emission and flags the rule
unimplemented. This bit during this unit: a `finding(...)` helper in
scanner/checks.rs turned spec:633 into a standing Warning; reverted to literal
`code:` field construction. Any future de-duplication of Finding construction
at emission sites must either keep the `code:` anchor visible or extend the
detector's anchor list in the same change.

## 3. The live parking application is currently moot

The acceptance's live scenario (the two `CAIRN_SOURCE_UNVERIFIED` findings
declared in `todo.source-tracked-verification-mode`'s `defers:` while that todo
is `blocked`) is satisfied as a fixture-level contract
(`test_cli_lint_parks_the_unverified_pair_behind_a_blocked_todo`), not as a
live-repo edit. That todo's dependency was ratified 2026-07-29 (PR #528 sheet
W3), so it is honestly `open` and parks nothing; the standing pair stays
non-selecting through the strict-green fold instead. Parking activates for the
pair only if that todo ever re-blocks, in which case its `defers:` list is two
lines away.
