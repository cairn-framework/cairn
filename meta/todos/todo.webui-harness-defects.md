---
node: cairn.ui
status: done
created: 2026-07-10
---

# Webui Harness Defects


## Problem
`node harness/eval.mjs` is the deterministic 11 scenario visual defect harness.
On main it reports `ux_defect_score = 41`: 40 from the `blueprint-modal` scenario
missing the `blueprintModal` landmark, plus 1 from a single tiny tap target on
`mobile-portrait`. The documented invariant is "score must be 0", but no CI job
enforces it. `.github/workflows/ci.yml`'s `webui` job runs biome, design token
conformance, and a static a11y audit only; it never invokes the harness.

## Evidence
- `harness-facts.md` (verified 2026-07-10): score 41 = 40 + 1, identical across
  two consecutive runs.
- Blueprint-modal root cause. `ACTIONS.openBlueprint.fire` (`harness/eval.mjs:91`)
  does `document.querySelector('.blueprint-trigger')`. There are TWO elements with
  that class in `app.js`: the "Report an issue" button at `app.js:494` and the real
  "View blueprint source" button at `app.js:497` (`onClick=${onOpenBlueprint}`).
  `querySelector` returns the first match, so the click opens a GitHub issue URL
  and never the modal. `missingLandmarks()` then reports `blueprintModal` missing
  (weight 40). The modal markup itself is correct (`.blueprint-modal`,
  `app.js:1673` to `1689`); the `settled` check `.blueprint-modal pre` times out,
  but the harness still scores, so the 40 point penalty lands.
- Tap target root cause. `auditPage` (`harness/lib/audit.mjs`) flags any
  interactive element (`a`, `button`, `input`, `select`, `textarea`, or
  `role=button`) whose `rect.width < 44` or `rect.height < 44` when `checkTap` is
  set. `mobile-portrait` and `tablet-portrait` set `checkTap: true`. Exactly one
  interactive control on `mobile-portrait` fails the 44px hit area that the
  responsive CSS already enforces for most controls via `@media (max-width: 480px)`
  and `@media (max-width: 860px)` `min-height: 44px` rules. The offending element
  is reported in `harness/out/report.json` under `scenarios[].detail.tap` as
  `<tag>.<classes>@WxH` (see `sig()` at `harness/lib/audit.mjs:120`). Leading
  suspects are controls not covered by those min height rules: `.copy-btn`
  (`style.css:2093`, `padding: 2px var(--s-2)`, `--t-micro` font, roughly 15px
  tall) and `.btn-text` (`style.css:2023`, `padding: 0`, `--t-small` = 12px,
  roughly 18px tall). Confirm the exact element from `report.json` before fixing.

## Approach
1. Fix the blueprint-modal defect. Either fix is acceptable; prefer the one that
   keeps the harness selector honest:
   - (a) Give the open blueprint button a distinct class (for example
     `.blueprint-open-trigger`) at `app.js:497` and update
     `ACTIONS.openBlueprint.fire` to select it. Touches UI and harness action.
   - (b) Change `ACTIONS.openBlueprint.fire` to a more specific selector, for
     example `.topbar-right .blueprint-trigger:last-child`. Harness only.
   After the fix the click must open `.blueprint-modal` so `landmarks.blueprintModal`
   is true and the modal `<pre>` satisfies `settled`.
2. Fix the tap target defect. Run `node harness/eval.mjs`, read
   `harness/out/report.json`, find the `mobile-portrait` scenario `detail.tap`
   entry, map the `sig()` signature to the DOM control, then give it a hit area of
   at least 44px in both dimensions on mobile. Extend the `@media (max-width: 480px)`
   block that already sets `min-height: 44px` for `.tool-btn`, `.graph-zoom button`,
   `.drawer-handle`, `.pill`, `.artefact`, and others (`style.css:2657` to `2670`),
   or bump padding. Re-run until `mobile-portrait` `tap` = 0.
3. Wire the harness into CI. Add a step to the `webui` job in
   `.github/workflows/ci.yml` that runs the harness and fails when the score is
   non-zero. The harness prints `METRIC ux_defect_score=<total>` and returns exit
   code 0 only when at least one scenario rendered; it does NOT exit non-zero on a
   non-zero score, so the CI step must parse the metric and fail. Concrete recipe:
   - The runner needs Node and headless Chrome (the harness launches Chrome via
     `launchChrome()`, see `harness/lib/cdp.mjs`). Confirm `ubuntu-latest` ships
     Chrome or install it (for example `npx @puppeteer/browsers install chrome`).
   - Step: `node harness/eval.mjs > harness/out/ci.log 2>&1 || true; score=$(grep -oE 'ux_defect_score=[0-9]+' harness/out/ci.log | cut -d= -f2); if [ "$score" != "0" ]; then echo "ux_defect_score=$score (must be 0)"; exit 1; fi`.
   - Optionally upload `harness/out/screenshots/` as a CI artifact on failure.
4. Verify locally: run `node harness/eval.mjs` twice in a row; both must print
   `ux_defect_score=0`.

## Acceptance criteria
- `node harness/eval.mjs` prints `ux_defect_score=0` on two consecutive local runs.
- `harness/out/report.json` shows `total: 0`, `missing_landmarks: 0`,
  `tiny_tap_targets: 0`.
- The `blueprint-modal` scenario reports `landmarks.blueprintModal` true with an
  empty `missingLandmarks` array.
- The CI `webui` job fails when the harness score is greater than 0 (verify by
  temporarily reverting a fix and confirming red, then restoring) and passes at
  score 0.

## Dependencies and ordering
- The executor edits `harness/eval.mjs` (scenario action) and
  `.github/workflows/ci.yml`; both are in scope even though they sit outside
  `src/ui_assets`. `harness/` is present on main (observed while writing this todo).
- No dependency on `todo.simplify-ui-query-api`; the harness runs on frozen
  fixtures, independent of the live API wire shape.
- Ordering: fix the two defects first so the baseline is clean, then wire the CI
  gate. The gate is meaningless until the baseline is 0.
- Not blocked by any other todo.
