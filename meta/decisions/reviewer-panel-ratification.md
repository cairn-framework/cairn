---
id: dec.reviewer-panel-ratification
nodes:
  - cairn.root
  - cairn.kernel.artefacts
status: accepted
ratification: binding
date: 2026-08-07
informed_by:
  - src.context-engineering-claude5
refines: [dec.decision-ratification-tiers]
related:
  - dec.north-star-continuous-loop
  - dec.rung-three-coordination-substrate
revisit_triggers:
  - "the maintainer vetoes a panel-accepted decision, which is the regime's error signal; two vetoes in any ten panel acceptances suspend panel acceptance of binding decisions until the rubric is retuned"
  - "a panel accepts a decision later shown to have had a live alternative the panel marked convergent, which is a rubric miss regardless of veto"
  - "the ratified_by vocabulary gains the reviewer-panel marker (todo.reviewer-panel-ratified-marker), at which point prose provenance migrates to the queryable field"
---

# Reviewer-panel ratification: route by contestedness, not blast radius

Accepted 2026-08-07 by maintainer direction in session. The maintainer's
ruling, in their own words across two messages: "i want the orchestration to
use adverserial reviewers and skeptical reviewers, and then ill let you know if
happy with the end result", and "i had already indicated in the past i wanted
cairn to be more autonomous software factory type thing, i just feel like i
have to keep signing obvious stuff, so i get decision fatigue and its much less
often that a decision really needs my input". That is the signature; this
record makes it queryable and bounded.

## Context

`dec.decision-ratification-tiers` routes acceptance authority by blast radius:
`local` is machine-acceptable under receipts, everything touching the binding
surface is maintainer-only, permanently. The boundary is mechanical and it
misroutes in one direction: a ruling can touch a protected surface and still be
the only sensible answer, and under the tiers rule it costs a maintainer
signature anyway. The maintainer named the result: decision fatigue, signing
obvious stuff, while the rare genuine fork gets no more attention than the
obvious case.

`src.context-engineering-claude5` records the instrument: rubric-driven
verifier agents. The distinction that matters for acceptance is not how wide a
ruling's blast radius is but whether a competent maintainer, seeing the same
evidence, could reasonably choose differently at material, costly-to-reverse
consequence. Blast radius is one input to that; it is not the answer.

## Decision

### 1. Contestedness is the routing test

Every proposed decision is sorted by an adversarial reviewer panel into
**convergent** (correct, and every alternative is clearly worse on the stated
evidence or is taste with no material consequence) or **contested** (a live
alternative exists with material, costly-to-reverse consequences). The rubric
is committed at `docs/agent/lenses/contestedness-*.md`; inflating contestedness
wastes the signature this regime exists to save, and waving through a real fork
is the regime's failure mode. Both are named in the rubric itself.

### 2. Convergent rulings are panel-acceptable, including binding ones

A decision whose clauses are all convergent, or whose contested clauses each
carry a recorded For/Against/Verdict debate that resolves the fork on stated
evidence, may be set `status: accepted` on the panel's receipts without a
pre-hoc maintainer signature. This narrows `dec.decision-ratification-tiers`'
"binding is maintainer-only, permanently" to "binding is
maintainer-accountable": the maintainer reviews outcomes and holds an
unconditional veto, exercised at any time, whose execution is a refining or
superseding decision recorded like any other. The `local` tier's machinery
(mechanical validation, `ratified_by: machine`, hook enforcement) is unchanged.

### 3. The evidence is receipts, not prose assurance

Panel acceptance of a binding decision records, in the accepted artefact: the
lenses run, committed as lens prompt files and cited by hash; per-lens receipt
reviews in `meta/reviews/` bound to the decision's recomputed subject manifest
by `subject_hash`, following the receipt shape `dec.decision-ratification-tiers`
ships; and the For/Against/Verdict debate for every contested clause. The
ratification gate does not mechanically check binding acceptances today
(`src/hooks/ratification.rs` filters to local-tier candidates), so until
`todo.reviewer-panel-ratified-marker` lands the receipts are the audit trail
and the prose names them; a green gate is not evidence of legitimate panel
acceptance and must never be reported as such.

### 4. What still reaches the maintainer before acceptance

Four classes, none waivable by a panel:

- a contested clause whose debate the adjudicator cannot force to a verdict on
  stated evidence (a genuinely balanced fork);
- anything with maintainer-external stakes: money, publishing, licensing,
  security posture, personal data, or a public claim;
- any change to `dec.cairn-mission` or to this regime itself;
- a supersession that would reverse a ruling the maintainer signed personally,
  where the recorded rationale shows the maintainer weighed and rejected the
  now-proposed direction.

Everything else moves. `cairn pending` stops being a queue of everything
binding and becomes the queue of exactly these four classes plus outcome
review.

### 5. Outcome review replaces pre-hoc signing

The maintainer reviews end results in session, on their own cadence. A panel
acceptance is presented as an outcome summary (what was accepted, what the
debates resolved, where the receipts are), never as a request. Silence does not
un-accept anything; a veto refines or supersedes. The first revisit trigger
above is the regime's error budget, chosen deliberately tight: two vetoes in
ten acceptances suspend the regime for binding decisions.

## The rubric

- **Tier**: `binding`. It narrows an accepted authority's central rule and
  changes who may accept what across the whole repository. The maintainer
  signed it in session; this record is the durable form of that signature.
- **Unblocks**: `dec.rung-three-coordination-substrate` acceptance under panel
  receipts, and every future convergent binding ruling. Retires the standing
  cost that produced the fatigue.
- **Alignment**: against `dec.cairn-mission` first, it keeps the maintainable
  and investigable properties by moving the maintainer's scarce attention to
  the decisions that need judgement, while every machine acceptance carries
  auditable receipts.
  - Goal 1: agents keep working because convergent rulings no longer wait on a
    human round trip.
  - Goal 2: guardrails hold because acceptance evidence is committed receipts
    bound by subject hash, not session memory.
  - Goal 3: the signature boundary moves from blast radius to contestedness,
    which is where the maintainer said their input actually matters.
  - Goal 4: the four pre-hoc classes in clause 4 still enqueue rather than
    self-ratify.
  - Goal 5: the queue carries genuine forks and outcome reviews instead of
    obvious rulings.
- **Options considered**: (a) keep tiers as ruled and batch signatures, which
  reduces interruptions but not judgement load, and leaves obvious rulings
  gating implementation; (b) machine-accept everything and rely on veto alone,
  which deletes the one boundary (clause 4) whose crossings are genuinely the
  maintainer's; (c) contestedness routing with panel receipts and post-hoc
  veto. (c) is the maintainer's direction. The cost of rejecting it is the
  fatigue that prompted it, or the audit hole of (b).
