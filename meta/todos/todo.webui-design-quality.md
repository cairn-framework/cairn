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

dec:dec.webui-design-quality-direction

## Status (2026-07-08)

- Bet B (`severity/drift` encoding) merged via #212; change dir archived.
- Bet C (`trace-the-truth hinge`, missing-proof gap) merged via #213; change dir archived.
- Bet A (Map becomes a map) is **blocked**: `dec.webui-design-quality-direction` gates it on the bet D scorer (deferred, harness-gated) AND a maintainer aesthetic call (refined-current vs full geological metaphor). Neither is decidable from the loop. Left as a deferred, maintainer-gated unit.
- Bet D (visual defect scorer / `ux_defect_score`) deferred: depends on the visual harness, not a standalone code task.
