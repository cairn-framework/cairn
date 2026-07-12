# Critique, iteration 1 (both candidates)

Evaluated in headless Chrome (system Chrome CLI at 1440x900 and 390x844; interactive gate in a CDP-driven tab at 485px).

## Strata Survey
- BROKEN: only 1 of 25 stones rendered; a script error aborted rendering (gate failure, craft capped at 5, scored 4).
- Two modules (Persist, Workspace) had no parent in the graph fixture and were never placed (gate failure).
- Page frame overflowed horizontally at narrow widths; offender was the findings ledger table (nowrap code and path cells).
- Eye lands on the inspector heading, not the canvas; canvas under-weighted at rest because default selection dims everything else to 0.28 opacity.
- Bedrock band mostly empty right of the Cairn slab.
- Filter toggle pressed state indistinguishable.
- Metaphor verdict: legible and restrained, not kitsch; distinctive.

## Calibrated Instrument
- Same unplaced-modules and narrow-overflow gate failures (findings path span).
- Dimmed non-neighbour labels read as disabled or broken (est. below 2:1 contrast).
- Command rail reads as scattered, no grouping band.
- Header counters have dead gaps; hash clipped at 390.
- Colour discipline strong: teal reserved for signal, amber only in legend.
- Verdict: about 70 percent instrument, 30 percent generic dark dashboard; the framing is in the chrome, not yet the canvas.

Decision: REFINE both.
