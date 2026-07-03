# Design: Webui design-quality

Implements `dec.webui-design-quality-direction`. Four bets, sequenced D, B, C, A.
The sequence is deliberate: D builds the scorer that lets B, C, and A be measured
rather than judged on vibes; B is the lowest-risk large win and the first defect
the scorer should catch; C extends the same encoding language to provenance; A is
the largest redesign and is gated on both the scorer and a maintainer aesthetic
call.

## Approach

All four bets modify the existing `cairn.ui` surface and the autoresearch harness.
No new blueprint nodes: `cairn.ui` already declares the webui, and the benchmark
lives in `harness/` (the frozen autoresearch workload, not a repo gate). Each bet
is token-based (colours, spacing, motion from `docs/design-system/tokens.css`),
reduced-motion-safe, and reversible.

## D. Design-quality scorer (first)

The benchmark `WEIGHTS` (`harness/eval.mjs`) measure only conformance and layout.
Add a deterministic design-quality dimension computed from the rendered DOM and
the decoded screenshot, scored by saturated proxies so no single signal can be
stuffed:

- **Severity is colour-encoded**: finding cards and node markers carry a
  severity-distinct colour channel, not just a word.
- **Layout dimensionality**: graph nodes occupy two or more layout dimensions
  (not a single column); measured from rendered node coordinates.
- **Dead-zone ratio**: fraction of the primary canvas with no content-bearing
  pixels, penalised past a threshold.
- **Brand-tone lexicon**: copy uses the product's voice, with a documented
  refresh path for the committed lexicon (the lexicon drifts as copy evolves;
  the proxy checks against a small, version-controlled lexicon file that is
  reviewed when the scorer is retuned), not generic dashboard chrome.
- **Motion-safe affordance density**: interactive affordances are present and
  reduced-motion-safe.

Each proxy uses a saturating transform (diminishing returns past a target) and
the loop requires a mandatory `inspect_image` visual-verification pass before a
keep. Committing the scorer is a benchmark change, so it lands via
`init_experiment new_segment: true`, not as a normal keep.

## B. Severity and drift encoding (second)

MODIFIED `cairn.ui` (`src/ui_assets/style.css`, `src/ui_assets/app.js`):

- Finding cards gain a severity-distinct left artery and weight (error reads
  hotter than warning), sourced from design-system tokens.
- The graph node a finding points to gains a severity marker, so drift is felt on
  the map, not only in the drawer text.
- Amber is de-overloaded: reserve it for one meaning and re-point the others
  (ghost pill, CLI badge, axis ticks) to distinct tokens.

## C. Trace-the-truth hinge (third)

MODIFIED `cairn.ui` (decision-detail render in `app.js`, hinge styles in
`style.css`):

- Draw the decision-to-proof relationship explicitly: the decision linked to the
  `informed_by` research and sources that earned it.
- Render a missing-proof state ("no sources recorded") as a visible, distinct gap
  rather than the quietest text on screen.
- Build on the typed-history rendering already landed (the hinge reads real
  `informed_by` / `related` / `supersedes` data, not placeholders).

## A. Map becomes a map (fourth)

MODIFIED `cairn.ui` (graph layout in `app.js`, node styles in `style.css`):

- Place nodes on the declared PROVENANCE / HINGE / AUTHORITY axis: position
  encodes distance-from-source-of-truth and layer; size and weight encode
  importance; edges are visible.
- Replace the single-column list with the two-dimensional layout.
- Gated on the D scorer (to confirm the new layout measures better) and the
  maintainer's aesthetic call (refined-current versus a full geological "cairn"
  metaphor: nodes as stacked stones, drift as cracks, reconciled as settled).

## Changes

ADDED:
- A design-quality scoring dimension in the autoresearch harness (`harness/`).

MODIFIED:
- `cairn.ui`: `src/ui_assets/style.css` and `src/ui_assets/app.js` (severity and
  drift encoding, hinge, map layout).

REMOVED:
- None.

RENAMED:
- None.

## Verification

- Per bet: `node harness/eval.mjs` shows the design-quality score improving and
  `ux_defect_score` holding at 0; `scripts/check-design-tokens.sh` and
  `scripts/check-a11y.sh` stay green; 0 console errors; an `inspect_image` visual
  pass on the touched scenarios.
- A is verified only after the aesthetic direction is chosen.
