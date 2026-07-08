# Design: Webui trace-the-truth hinge (bet C)

## Token strategy

The missing-proof gap is a provenance *absence*: the decision is detached from
its evidence, which is exactly what the `--orphaned` token means (weathered,
detached). So the gap is accented with `--orphaned`, distinct from the
reconciliation `--ghost` and from the severity `--drift` (error) used in bet B.
This keeps the three signal meanings separate, per the de-overload direction in
bet B.

All values are tokens from `docs/design-system/tokens.css`; no hardcoded hex/rem.

| Property | Value |
|----------|-------|
| border | `1px dashed var(--orphaned)` |
| border-radius | `var(--r-edge)` |
| padding | `var(--s-2) var(--s-3)` |
| background | `var(--stone-2)` |
| text colour | `var(--ink-char)` (was `var(--ink-faded)`) |
| font-style | normal (was italic) |
| font-weight | 500 (was inherited) |
| bullet `.n` | `var(--orphaned)` |

## Surfaces touched

- `src/ui_assets/app.js`
  - `DecisionDetail` provenance missing-state: class `hinge-item muted` ->
    `hinge-item gap-missing` (line 1254).
- `src/ui_assets/style.css`
  - Added `.hinge-item.gap-missing` and `.hinge-item.gap-missing .n` after the
    existing `.hinge-item.muted` rule.
  - `.hinge-item.muted` is retained for the authority-side "no module attached"
    state (different semantic: attachment, not proof).

## Verification

- `biome check --error-on-warnings src/ui_assets/app.js src/ui_assets/style.css`
  (CI pins `@biomejs/biome@2.4.4`): clean.
- `scripts/check-design-tokens.sh`: clean.
- `scripts/check-a11y.sh`: clean.
- `harness/eval.mjs` `ux_defect_score` unchanged from baseline 41; contrast,
  svg_contrast, palette_violations all 0.
