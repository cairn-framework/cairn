# Proposal: fix-webui-harness-defects

## Motivation

`node harness/eval.mjs` reports `ux_defect_score = 41` on main: 40 from the
`blueprint-modal` scenario missing the `blueprintModal` landmark, and 1 from a
single tiny tap target on `mobile-portrait`. The documented invariant is "score
must be 0", but no CI job enforces it. The two defects have known root causes
documented in `todo.webui-harness-defects`.

## Scope

- Fix the blueprint-modal harness defect: two elements share the
  `.blueprint-trigger` class; `querySelector` picks the wrong one (the "Report an
  issue" button), so the modal never opens during the harness scenario.
- Fix the mobile-portrait tiny tap target: one interactive control fails the
  44px minimum hit area.
- Wire the visual harness into the `webui` CI job so a non-zero score fails CI.

## Out of scope

- General webui redesign or visual polish beyond the two reported defects.
- Changes to the harness scenario set or scoring algorithm.
- The `simplify-ui-query-api` endpoint migration (separate in_progress todo).
