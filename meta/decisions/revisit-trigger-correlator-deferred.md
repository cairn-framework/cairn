---
id: dec.revisit-trigger-correlator-deferred
nodes:
  - cairn.root
status: accepted
date: 2026-06-29
informed_by: [res.spec-designed-audit, res.revisit-trigger-correlator-probe]
supersedes: [dec.revisit-trigger-relevance]
---
# spec:634 revisit-trigger relevance: keep Declared/pending, do not build the term-coverage correlator

## Why this supersedes `dec.revisit-trigger-relevance`

`dec.revisit-trigger-relevance` ruled "build the changes-corpus correlator, sequenced
behind cairn-1me", resting on one load-bearing claim: the correlator "fires a real true
positive today" and is "not net-negative". cairn-1me has since landed
(`dec.changes-in-artefact-set`), so the substrate exists. Building on it, the matching
semantics that `res.spec-designed-audit` (gap 2) said the work still needed were
investigated empirically against the live corpus, and the result falsifies that premise.

## What the probe found

Over the live corpus (7 decisions carrying 21 `revisit_triggers`, 2 active changes):

- Verbatim full-trigger substring matching yields 0 hits (useless).
- Significant-term coverage (trigger terms intersected with change terms, over trigger
  term count) is the only deterministic signal that produces anything. It cleanly
  isolates exactly one pair: `dec.webui-design-quality-direction` triggers at 0.75 and
  0.77 coverage against the `webui-design-quality` change, with the next unrelated
  cross-pair at 0.40. A threshold near 0.6 flags exactly that one pair.

That single "true positive" is a **structural tautology, not trigger relevance**. The
`webui-design-quality` change *is* the implementation log of
`dec.webui-design-quality-direction` (its proposal says "this change tracks that work";
its design says "implements dec.webui-design-quality-direction"), so it necessarily
echoes that decision's trigger vocabulary. Worse, the change text explicitly states the
trigger conditions have **not** fired (the paid-vision provider is "precluded"; the
aesthetic-direction choice is "the maintainer's call", not yet made). Term-coverage
measures **topical proximity** ("a change is working in this decision's area"), which a
bag-of-words cannot distinguish from "the trigger condition fired" or even "the change
explicitly declined the trigger".

## The ruling (O2: supersede and defer)

A reformer/conservative debate weighed building the proximity correlator now (O1), adding
self-reference and rare-term precision guards (O3), or deferring (O2). Decisive
fundamentals:

- cairn has a **single Info channel**. Its one real-world flag would be self-referential
  (every `meta/changes/<slug>` echoes `dec.<slug>`), so the advisory mostly fires where a
  re-read concludes "not fired". That trains inattention on the one advisory channel, and
  trained inattention is not revertable.
- This is the **"a proxy rewards a change a visual review judges worse"** failure mode that
  `dec.webui-design-quality-direction` explicitly carries a standing revisit trigger
  against. Shipping a proximity proxy as a relevance signal repeats it.
- Promoting `spec:634` to `enforced` would replace an **accurate** CK004 "pending" Info
  ("designed, no enforcer yet") with a **misleading** "may be relevant" Info at the same
  severity: strictly worse.
- By the superseded decision's own test (only **net-negative** value vetoes an
  idle-capacity build), the falsified true-positive makes the proximity correlator
  net-negative. The honest move is therefore not to build it.

So:

- `spec:634` stays `pending` in `docs/registries/spec-rules.md`; the CK004 Info remains the
  honest living tracker of a Designed-but-unbuilt rule. No `enforced` promotion.
- The deterministic term-coverage correlator is **not** built. Node-id matching (0 recall)
  and git-log correlation (breaks `dec.no-orchestrator` scan purity) remain foreclosed.
- The real enforcer is parked behind a capability that can judge relevance rather than
  proximity: the `cairn-iy2` ghost-rule primitive, or a maintainer-sanctioned semantic /
  vision gate (which would also supersede `dec.webui-ai-vision-loop-declined`). Re-file an
  implementation bead when such a capability lands.

This satisfies bead cairn-9w9's acceptance criterion ("an explicit decision to keep it
Declared pending cairn-iy2; no silent prose-only state").
