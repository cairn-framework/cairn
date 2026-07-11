# Design: fix-webui-harness-defects

## Approach

Two targeted defect fixes plus a CI wiring step. Each fix is independent.

### Fix 1: blueprint-modal selector collision

Two elements in `app.js` carry the `.blueprint-trigger` class: the "Report an
issue" button (~line 494) and the real "View blueprint source" button
(~line 497). The harness action `ACTIONS.openBlueprint.fire`
(`harness/eval.mjs:91`) does `document.querySelector('.blueprint-trigger')`
which returns the first match (the issue button), so the modal never opens.

Fix: give the real "View blueprint source" button a distinct class
(`.blueprint-open-trigger`) at `app.js:497` and update the harness selector in
`harness/eval.mjs` to match. This keeps the selector honest: it targets the
element whose click handler opens the modal.

### Fix 2: mobile-portrait tiny tap target

One interactive control on `mobile-portrait` fails the 44px minimum hit area.
The exact element is identified from `harness/out/report.json` under
`scenarios[].detail.tap`. The responsive CSS already sets `min-height: 44px`
for most controls in `@media (max-width: 480px)`; the offending control needs
the same treatment.

### Fix 3: CI wiring

Add a step to the `webui` job in `.github/workflows/ci.yml` that runs
`node harness/eval.mjs`, parses the `ux_defect_score` metric, and fails when
non-zero. The runner needs Chrome; install via
`npx @puppeteer/browsers install chrome`.

## Changes

ADDED:
- `.blueprint-open-trigger` class on the real blueprint button (`app.js`).
- CI step running the visual harness with score assertion (`.github/workflows/ci.yml`).

MODIFIED:
- `harness/eval.mjs`: update `ACTIONS.openBlueprint.fire` selector to
  `.blueprint-open-trigger`.
- `src/ui_assets/style.css`: extend mobile `min-height: 44px` rule to cover the
  offending tap target control.

REMOVED:
- (none)

RENAMED:
- (none)
