---
node: cairn.ui
status: done
created: 2026-07-17
---

# Webui Topbar Dedupe

Unit 3c of `dec.webui-design-direction` priority 3 (gh:#305): deduplicate the
topbar/inspector stat readouts and tidy the mobile topbar.

## Problem
`res.webui-review-audit` flagged a duplicated summary: the project-level
`N nodes, M edges, F findings` count was rendered in both the topbar command
placeholder and the inspector's empty-state `ins-slug`, so the eye ping-ponged
between two competing primary zones (audit finding 5). Separately, the Track B
audit (finding 8) noted the mobile topbar was cramped below 860px, with the report chip and actions tight enough that a control was partially occluded at 390px.
## Fix
- Removed the duplicated project-level stat readouts from the inspector empty
  state: the `ins-slug` `N nodes · M edges · F findings` line and the
  `findings-link-count` badge (the findings total also lives in the topbar
  `graphStats`). The topbar command placeholder keeps the canonical
  project-level count (nodes, edges, findings); the inspector keeps only
  map-health stats in its overview (`stat-grid` modules/ghost/orphaned for the
  map) and selected-node detail (the per-node decisions/contracts/todos/research
  row for a selected module). The
  inspector's findings link stays as a navigation action without a duplicated
  count.
- Tidied the mobile topbar: added a phone-only short report label
  (`webui.report.topbar_short` = "Report") rendered alongside the full label
  and toggled at `max-width: 480px` via `.report-full`/`.report-short`, keeping
  the full text as the button `title`. The brand, search, and actions now fit
  390px without wrapping or clipping; the 44px tap minimum already covered by
  the `max-width: 860px` block is preserved.
- Added the user-facing string through `docs/design-system/copy.toml` and the
  `copy()` helper (British spelling, no em-dashes); no hardcoded strings.

## Gates
`bunx @biomejs/biome@2.4.4 check --error-on-warnings` (changed files clean;
three pre-existing non-goal canvas/shared files fail at baseline),
`scripts/check-design-tokens.sh` and `scripts/check-file-sizes.sh` pass,
`node harness/eval.mjs` reports `ux_defect_score=0` and `scenarios_ready=11/11`,
and the focused Rust UI suites (`phase_7_7_ux_foundation`, `graph_explorer`,
`ui_mobile`) pass.

dec:dec.webui-design-direction
