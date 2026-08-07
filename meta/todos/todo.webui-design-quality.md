---
node: cairn.ui
status: blocked
created: 2026-07-03
---

# Webui Design Quality

Roadmap item surviving the archive of `meta/changes/webui-design-quality/`
(a ~20-task, 0-done checklist that was never finished but whose direction
is already ratified). Bring the webui's visual design up to the standard
set by `docs/design-system/`: tokens-only styling, no hardcoded hex/rem,
consistent component reuse across the graph explorer, inspector, and
command palette.

dec:dec.webui-write-authority

## Status (2026-07-08)

- Bet B (`severity/drift` encoding) merged via #212; change dir archived.
- Bet C (`trace-the-truth hinge`, missing-proof gap) merged via #213; change dir archived.
- Bet A (Map becomes a map) is **blocked**: `dec.webui-design-quality-direction` gates it on the bet D scorer (deferred, harness-gated) AND a maintainer aesthetic call (refined-current vs full geological metaphor). Neither is decidable from the loop. Left as a deferred, maintainer-gated unit.
- Bet D (visual defect scorer / `ux_defect_score`) deferred: depends on the visual harness, not a standalone code task.

## Review disposition (2026-07-11)

Backlog review (adversarial + simplification) recommends this be treated as
**superseded**. Bets B and C shipped (#212/#213); bet D is deferred and
harness-gated (not a code task); the only live scope, bet A, is superseded by
`todo.design-studio-exploration` (backed by accepted
`dec.design-studio-exploration-method`), which owns the bet A direction question.
No actionable loop work remains here. Kept `blocked` as a pointer. (The
"close when the direction resolves" instruction here is superseded by the
2026-07-12 section below: the todo stays blocked as the bet D home.) Do
not start work under this slug; use `design-studio-exploration`.

## Supersession (2026-07-12)

Both design-studio tracks have run. Bet A's aesthetic call is resolved by
`dec.webui-design-direction` (calibrated instrument, geological state
vocabulary as motif), informed by `res.design-studio-greenfield` and
`res.webui-review-audit`. This todo's bet A line is superseded by
`todo.design-studio-exploration` and that decision, ratified 2026-07-12.
This todo stays open (blocked) as the home for bet D tracking only: bet D
(design-quality scorer) remains deferred and harness gated. Bets B and C
shipped; bet A is resolved by the decision above.

## Bet A implementation (2026-07-17)

Bet A (graph canvas state legibility) implementation has started under `dec.webui-design-direction` (gh:#305): canvas selection emphasis, pan-to-selection, the module state keel, and an explained legend are being landed in dedicated worktrees. This todo stays `in_progress` as the home for bet D (design-quality scorer) tracking only.

## Status correction (2026-07-28)

Reopened as `blocked`, not `done`. Bet D (the deterministic design-quality
scorer) is still unbuilt and still gated on the visual harness, and
`dec.webui-write-authority` clause 5 names this todo as its tracker, so a
`done` status misrepresented the plan. Bets A, B, and C are finished; this todo
covers bet D only. Unblock by setting it `open` when the visual harness can host
the scorer.

## Mission disposition

2026-08-02: blocked against dec.cairn-mission. Serves fit-for-purpose. Bet D design-quality scorer tracker per dec.webui-write-authority clause 5, gated on the visual harness; bet A alone was superseded by dec.webui-design-direction.

2026-08-07 audit (todo.roadmap-assumption-audit): res.chatgpt-issue-audit says unblock and narrow to Bet D proxies, but the body is decision-gated (dec.webui-design-quality-direction) plus maintainer-gated, so it stays blocked; narrowing needs that ruling.
