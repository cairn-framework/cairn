# Tasks: Webui design-quality

Sequenced per `dec.webui-design-quality-direction`: D, then B, then C, then A.
Within each bet, top to bottom.

## D. Design-quality scorer (first; benchmark change, segment bump)

- [ ] Specify the design-quality proxies and their saturating transforms
- [ ] Implement severity-colour-encoded proxy in the harness audit
- [ ] Implement layout-dimensionality and dead-zone-ratio proxies
- [ ] Implement brand-tone lexicon and motion-affordance proxies
- [ ] Wire the design-quality dimension into the scorer and report
- [ ] Add the mandatory inspect_image verification step to the loop
- [ ] Bump segment (init_experiment new_segment) and baseline the scorer

## B. Severity and drift encoding (second; lowest-risk large win)

- [ ] Add severity-distinct artery and weight to finding cards (tokens)
- [ ] Mark the finding's node on the graph by severity
- [ ] De-overload amber: reserve one meaning, re-point the rest
- [ ] Verify against the D scorer; hold ux_defect_score at 0

## C. Trace-the-truth hinge (third)

- [ ] Draw the decision-to-proof relationship in decision detail
- [ ] Render a distinct missing-proof ("no sources recorded") state
- [ ] Verify hinge legibility and the D scorer; gates green

## A. Map becomes a map (fourth; gated on D scorer + maintainer aesthetic)

- [ ] Maintainer chooses aesthetic direction (refined-current vs full geology)
- [ ] Place nodes on the PROVENANCE / HINGE / AUTHORITY axis (2D layout)
- [ ] Encode importance via node size and weight; render visible edges
- [ ] Verify the new layout measures better on the D scorer; gates green
