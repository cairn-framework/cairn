# Design: Webui severity and drift encoding (bet B)

## Token strategy

`docs/design-system/tokens.css` defines three signal colours and three
reconciliation colours. Crucially `--drift` and `--ghost` alias the same value
(`#d47854`) but are distinct tokens so UI code can distinguish *finding
severity* from *reconciliation ghost state*. The brief's "de-overload amber"
is satisfied at the token level: severity stops borrowing the reconciliation
token.

| Severity | Token (was)      | Token (now) | Computed value |
|----------|------------------|-------------|----------------|
| error    | `var(--ghost)`   | `var(--drift)`  | `#d47854` (warm red) |
| warning  | `var(--orphaned)`| `var(--orphaned)`| `#b6ac96` (weathered) |
| info     | `var(--settled)` | `var(--settled)` | `#82a893` (mossy green) |

The computed colours are unchanged for warning/info and identical for error
(`--drift` == `--ghost` value), so contrast and palette gates are unaffected.
`--block` (`#b84c38`) is intentionally **not** used for any text or pill,
because on the dark stone surface it fails the 4.5:1 normal-text contrast
threshold (`harness/lib/audit.mjs` line 177); it remains available for
non-text borders only.

## Surfaces touched

- `src/ui_assets/app.js`
  - `severityPill` returns `drift` / `orphaned` / `settled` (was `ghost` /
    `orphaned` / `info`).
  - Graph node `SystemNode` / `ContainerNode` / `ModuleNode` stroke and inner
    marker use `var(--drift)` for error (was `var(--ghost)`); warning/info
    unchanged.
  - Findings buckets render `pill drift` / `pill orphaned` / `pill settled`.
  - Recent-row buttons carry `sev-<severity>` class for the artery hook.
- `src/ui_assets/style.css`
  - `.pill.drift` and `.pill.settled` added (mirror existing pill modifier
    pattern, using `*-wash` backgrounds).
  - `.recent-row` gains a transparent 3px left border; `.recent-row.sev-*`
    colour it per severity; `.recent-row.sev-error` adds `font-weight: 600`.
  - `.prose-nudge.error` border colour `--ghost` -> `--drift`.

## Verification

- `biome check --error-on-warnings src/ui_assets/app.js src/ui_assets/style.css`
  (CI pins `@biomejs/biome@2.4.4`): clean.
- `scripts/check-design-tokens.sh`: clean (no hardcoded hex/rem).
- `scripts/check-a11y.sh`: clean.
- `harness/eval.mjs`: `ux_defect_score` identical to pre-change baseline (41);
  contrast, svg_contrast, palette_violations all 0.
