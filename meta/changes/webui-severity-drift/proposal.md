# Proposal: Webui severity and drift encoding (bet B)

## Motivation

The autoresearch loop converged the webui to `ux_defect_score` conformance, but a
grounded design review (`res.webui-design-quality-review`) found the UI is
flat and list-shaped: finding cards do not signal severity, and the graph node a
finding points to is not visually distinguished by severity. Amber (`--ghost`) is
over-loaded: it is used both for the reconciliation *ghost* state and for
*error* severity, so the two meanings are indistinguishable in code.

This change is bet B of the ratified `dec.webui-design-quality-direction`, the
second of four bets (D, B, C, A) and the lowest-risk large win. It is tracked by
`meta/todos/todo.webui-design-quality.md` (node `cairn.ui`) and continues the
scope of the archived `meta/changes/archive/2026-07-03-webui-design-quality/`.

## Scope (bet B only)

- Colour-code finding cards by severity with a left artery plus error weight.
- Mark the finding's node on the graph by severity (stroke + inner marker).
- De-overload amber: reserve `--ghost` for the reconciliation ghost state; route
  severity *error* through the signal token `--drift` (same computed value,
  distinct token per `docs/design-system/tokens.css`), so UI code can tell the
  two meanings apart.
- Hold the existing conformance metric (`scripts/check-design-tokens.sh`,
  `scripts/check-a11y.sh`, `biome`, and the `harness/eval.mjs` defect score) at
  its pre-change baseline.

## Out of scope

- The D scorer, C hinge, and A map-layout bets (separate units).
- Any new node or blueprint structural change (`cairn.ui` owns the webui).
- Light theme and landing/design-system repointing (separate reviews).

## Acceptance criteria

- Finding cards carry a severity-distinct left artery, and error cards read
  heavier than warning cards (weight + hotter signal token).
- A graph node with a finding is marked by severity (distinct stroke + marker).
- Severity *error* no longer uses the reconciliation `--ghost` token; amber is
  reserved for one meaning.
- `scripts/check-design-tokens.sh`, `scripts/check-a11y.sh`, and
  `biome check --error-on-warnings` stay green.
- `harness/eval.mjs` `ux_defect_score` is unchanged from the pre-change baseline
  (no visual regression). The baseline is 41, driven by pre-existing
  `tiny_tap_targets` and `missing_landmarks` structural defects unrelated to
  severity encoding; this change does not move it.
