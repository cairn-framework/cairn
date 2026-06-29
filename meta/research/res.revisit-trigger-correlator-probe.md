---
id: res.revisit-trigger-correlator-probe
nodes:
  - cairn.root
date: 2026-06-29
method: primary
tags: [spec-634, revisit-triggers, scan]
---
# Probe: does term-coverage measure revisit-trigger relevance, or topical proximity?

## Method

Original observation over the live cairn corpus (the methodology is the evidence; no
external sources). Loaded the 2 active changes under `meta/changes/`
(`webui-design-quality`, `scanner-gate-artefact-links`, proposal + design + tasks text)
and the 7 decisions carrying `revisit_triggers` (21 triggers total). Evaluated two
deterministic matchers per (trigger, change) pair:

1. **Verbatim**: is the full trigger sentence a substring of the change text?
2. **Term-coverage**: of a trigger's significant terms (lowercased words of length >= 4,
   minus stopwords), what fraction appears in the change's terms?

## Findings

- **Verbatim: 0 matches.** Trigger sentences never appear literally in change prose.
- **Term-coverage isolates exactly one pair.** `dec.webui-design-quality-direction`
  trigger[2] = 0.77 and trigger[0] = 0.75 against the `webui-design-quality` change; the
  highest *unrelated* cross-pair is 0.40 (`bd-upgrade-plan` vs `webui-design-quality`). A
  threshold in [0.5, 0.7] flags exactly that one decision-change pair.
- **That single hit is a structural tautology, not trigger relevance.** The
  `webui-design-quality` change *is* the implementation log of
  `dec.webui-design-quality-direction` (its proposal states "this change tracks that
  work"; its design states "implements dec.webui-design-quality-direction"), so it
  necessarily echoes that decision's trigger vocabulary. Inspecting the hit terms confirms
  domain vocabulary (`aesthetic`, `geological`, `metaphor`, `refined-current`, `vision`,
  `paid`, `inspect_image`), not the decision id, so a self-reference filter would not
  remove it.
- **The change explicitly negates the trigger conditions it "matches".** Its text records
  the paid-vision provider as "precluded" and the aesthetic-direction choice as the
  maintainer's call "not yet made". So the one positive points at triggers that have *not*
  fired.

## Conclusion

Significant-term coverage measures **topical proximity** ("a change is working in this
decision's area"), which a bag-of-words cannot distinguish from "the trigger condition
fired" or "the change explicitly declined the trigger". It is therefore not a faithful
deterministic signal for `spec:634` ("revisit_triggers appear relevant"). This falsifies
the load-bearing "verified true positive" premise of `dec.revisit-trigger-relevance` and
grounds the deferral recorded in `dec.revisit-trigger-correlator-deferred`.
