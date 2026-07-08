# Proposal: Webui trace-the-truth hinge (bet C)

## Motivation

`dec.webui-design-quality-direction` bet C extends the severity/drift encoding
language (bet B) to provenance. The decision detail already renders a
`hinge-diagram` that draws the decision-to-proof relationship: provenance
(`informed_by` research and sources) on one side, authority (the attached node
and its on-disk state) on the other. But when a decision has no recorded
evidence, the provenance side renders `no sources recorded` with the `muted`
utility class, which is the *quietest* text on screen (faint, italic). That
buried the single most important quality signal: a decision with no proof.

This change is bet C of `dec.webui-design-quality-direction`, tracked by
`meta/todos/todo.webui-design-quality.md` (node `cairn.ui`), continuing the
archived `meta/changes/archive/2026-07-03-webui-design-quality/`.

## Scope (bet C only)

- Make the missing-proof state (`no sources recorded`) a visible, distinct gap
  instead of the quietest text: a dashed `--orphaned` border, visible
  (`--ink-char`) weight, on `--stone-2`.
- Keep the hinge reading real `informed_by` data (already landed); no
  placeholders.
- Hold the existing conformance metric at its pre-change baseline.

## Out of scope

- The D scorer and A map-layout bets (separate units).
- The authority-side "no module attached" gap: it is a workflow/attachment
  state, not a proof gap, so it stays `muted` (deliberate distinction: a
  decision lacking evidence is a quality gap worth highlighting; an unattached
  decision is a quiet workflow state).
- Any new node or blueprint structural change.

## Acceptance criteria

- A decision with no `informed_by` shows `no sources recorded` as a distinct
  dashed `--orphaned` gap, not faint italic text.
- A decision with `informed_by` still lists its evidence refs in the provenance
  side.
- `scripts/check-design-tokens.sh`, `scripts/check-a11y.sh`, and
  `biome check --error-on-warnings` stay green.
- `harness/eval.mjs` `ux_defect_score` unchanged from baseline (pre-existing
  structural defects only; no regression).
