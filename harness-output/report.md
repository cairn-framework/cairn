# Design Studio report — Cairn Graph Explorer greenfield run

Track A of dec.design-studio-exploration-method. Run under context denial: the current cairn webui, its CSS, and its design tokens were never inspected. All mocks render the real frozen fixtures (map.json: 25 nodes, 27 edges, 1 info finding; api fixtures; copy.json).

## Directions explored

1. **Strata Survey** (geological / cairn-stones pole): warm limestone paper, strata beds as containment, stone-slab nodes, engraved serif labels, oxidised-copper accent, hazard ochre reserved for drift, survey-marker selection benchmark. Rejected sibling concepts: trail-map pastiche; literal illustrated rock garden.
2. **Calibrated Instrument** (refined technical pole): warm graphite chassis in three steps, teal-green signal accent, instrument amber for drift, bezel counters, state keels, container brackets, mono for machine identity. Rejected sibling concepts: terminal noir; blueprint cyanotype.

## Iteration history

| Iter | Strata Survey | Calibrated Instrument | Decision |
|---|---|---|---|
| 1 | wA 6.0 (dQ6 o8 c4 f4), gate failures: render-aborting JS error, 2 unplaced modules, narrow overflow | wA 5.8 (dQ6 o7 c5 f5), gate failures: 2 unplaced modules, narrow overflow | REFINE both |
| 2 | wA 7.0 (dQ7 o8 c6 f6), gates clean, interactions verified | wA 6.8 (dQ7 o8 c6 f7), gates clean, interactions verified | REFINE both |
| 3 | wA 7.8 (dQ8 o8 c8 f7) | wA 8.0 (dQ8 o8 c8 f8) | SHIP both, pick winner |

Adversarial gate at iteration 2+ (CDP-driven tab): node selection, edge-link navigation in the inspector, search highlight, kind toggles all verified; zero console errors; no page-frame horizontal overflow at 683px or 390px (canvas scrolls internally by design).

Evaluator reconciliation note: the iteration-2 evaluator claimed the bedrock band was still empty; a DOM check (slab rect 740 of 860) and a targeted re-inspection (about 95 percent span) showed the fix had landed, so that finding was recorded as stale carry-over, not a defect.

## Winner and why

**Calibrated Instrument** wins: weighted average 8.0 vs 7.8, all four criteria at 8, and the higher zone-rubric total (126 vs 124), with the strongest inspector clarity. Strata Survey's canvas state vocabulary (dashed ghost outline, tilted slab with ochre underline for orphaned) scored best-in-class state clarity (9) and is adopted into the codified DNA as the keel and tilt motifs. Strata Survey remains a fully scored, openable candidate for the downstream aesthetics decision.

## Artefacts

- Mocks: `harness-output/mocks/strata-survey.html`, `harness-output/mocks/calibrated-instrument.html`, shared data `harness-output/mocks/data.js` (generated from the frozen fixtures, no network, openable via file://).
- Spec and contract: `harness-output/spec.md`, `harness-output/sprint-contract.md`.
- Design descriptions: `harness-output/design-description-1-geological.md`, `harness-output/design-description-1-instrument.md`.
- Critiques: `harness-output/critique-1.md` to `critique-3.md`; scores: `harness-output/scores.json`.
- Codified system: `harness-output/design-dna.md`, `harness-output/tokens.css`, installable skill at `harness-output/design-system/skill/cairn-explorer-design/`.
- Screenshots (headless Chrome, 1440 and 390, three iterations): `harness-output/screenshots/ds-{strata,instr}[-2|-3]-{1440,390}.png` (also in /tmp).

## Zone rubric scores

Shared rubric (1-10): hierarchy / density / colour discipline / state clarity, final iteration.

| Zone | Strata Survey | Calibrated Instrument |
|---|---|---|
| Header | 8 / 7 / 9 / 6 | 8 / 7 / 9 / 7 |
| Graph canvas | 8 / 7 / 8 / 9 | 8 / 6 / 9 / 8 |
| Inspector | 9 / 8 / 9 / 8 | 9 / 8 / 9 / 8 |
| Command surface | 7 / 6 / 8 / 7 | 7 / 8 / 8 / 7 |
| **Total** | **124** | **126** |
