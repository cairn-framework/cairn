# Tasks: fix-webui-harness-defects

- [x] Fix blueprint-modal selector: give the "View blueprint source" button a distinct `.blueprint-open-trigger` class in `app.js`, update harness selector in `harness/eval.mjs`.
- [x] Fix mobile-portrait tap target: `.cmd-trigger` was 30x44; added `min-width: 44px` to `@media (max-width: 480px)` rule.
- [x] Fix tablet-portrait tap target: added `.blueprint-open-trigger` to `@media (max-width: 860px)` and `@media (max-width: 480px)` `min-height: 44px` rules.
- [x] Wire harness into CI: added `actions/setup-node@v4` (node 22) and a score-asserting step to the `webui` job in `.github/workflows/ci.yml`.
- [x] Verify: `node harness/eval.mjs` prints `ux_defect_score=0` on two consecutive runs.
