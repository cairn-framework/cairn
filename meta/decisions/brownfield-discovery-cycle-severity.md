---
id: dec.brownfield-discovery-cycle-severity
nodes:
  - cairn.brownfield
status: accepted
date: 2026-07-27
informed_by:
  - res.brownfield-observed-cycle-measurement
supersedes:
  - dec.order-containment-rule
related:
  - dec.order-containment-rule
  - dec.brownfield-init-round-trip
---
# A cycle cairn inferred is advisory; a cycle a human declared is an Error

## Status note

Filed `proposed`, not `accepted`, deliberately: clause 3 narrows a
consequence of the then-accepted `dec.order-containment-rule`, and the dev loop
does not self-ratify a ruling that bends an accepted invariant
(`dec.loop-resolves-knowable-gaps`). Accepted 2026-07-29 by maintainer
ratification (PR #528 sheet W5), taking option A of "What ratification must do"
below: `dec.order-containment-rule` is marked `superseded` and this decision
carries `supersedes: [dec.order-containment-rule]`, landed together in the same
commit. `todo.brownfield-nested-package-scan-clean` is unblocked accordingly.

## What ratification must do

Accepting this is not a status flip alone. Clause 3 contradicts one consequence
of `dec.order-containment-rule` (that a combined-constraint contradiction always
fails lint and hooks), so ratification must also resolve that contradiction in
the graph, by one of:

- Mark `dec.order-containment-rule` `status: superseded` and add `supersedes:
  [dec.order-containment-rule]` here. Clause 6 exists to make this safe: it
  restates every surviving obligation of that decision so nothing unrelated is
  retired. Note `supersedes` only validates once the target is already marked, so
  the two edits land together.
- Or amend `dec.order-containment-rule` in place to carve out discovery-observed
  cycles, leaving it accepted, and demote the `related` link here to a
  cross-reference.

Until option A landed, this decision stayed `proposed` and the child todo
stayed blocked. A `proposed` decision cannot retire an accepted one without
self-ratifying, which is why the marking was not in the authoring commit; it
landed with the acceptance on 2026-07-29.

## Context

`cairn init --from-code` groups a package root and its subpackages into flat
sibling Modules and derives dependency edges from observed imports. When imports
run both ways, both directions are emitted, and the resulting two-node cycle is a
`CAIRN_ORDER_CYCLE` Error, so the first `cairn scan` of a fresh brownfield map
exits 1.

`todo.brownfield-parent-child-edge-model` framed this as a modelling question
with four candidate answers, and left one measurement open: whether the fixture's
aggregation-induced cycle generalises to real projects.

`res.brownfield-observed-cycle-measurement` answers it against thirteen
repositories, five of which emit any edges, using `cairn scan`'s own findings.
Three results reshape the question:

1. Four of the five edge-emitting repositories fail the first scan. Among first
   maps that observe any coupling at all, failure is the common case rather than
   a corner case. The eight repositories that emitted no edges cannot exhibit the
   finding and say nothing either way.
2. No edge-suppression rule tested clears the finding on every affected
   repository. Suppressing the non-dominant direction clears one repository of
   four, suppressing ancestor and descendant edges clears two, both together
   clear three. A 152-candidate repository stays cyclic under every combination,
   and its surviving cycle spans five nodes in unrelated subtrees. Each rule
   helps somewhere; none guarantees a clean scan.
3. Only 30 of 50 reciprocal pairs are ancestor and descendant pairs. The rest are
   siblings, which no parent/child rule touches.

So the premise the todo inherited, that fixing the package-root model makes a
brownfield scan clean, does not survive contact with real repositories. Result 2
bounds how far that goes: it shows parent/child filtering leaves directory-level
cycles standing, not that those cycles exist in the file-level import graph.
Finer-grained discovery was never simulated, so some of them may still be
aggregation artefacts. Either way, no rule about package roots removes them, and
that is enough to rule on.

## Decision

Recommended, then ratified 2026-07-29:

1. **Discovery keeps both observed edges.** `derive_import_edges` continues to
   emit every direction it can prove from the code. A first map does not delete
   evidence to satisfy a gate.

2. **Discovery does not nest.** Package roots and subpackages stay flat sibling
   Modules, exactly as `dec.brownfield-init-round-trip` clause 2 already rules.
   That decision stands unchanged and is not superseded.

3. **Severity, not shape, is what changes.** `CAIRN_ORDER_CYCLE` stays an Error
   for a cycle whose component holds any hand-declared edge. It becomes a
   non-blocking advisory finding when every edge with both endpoints inside that
   component carries discovery provenance. Clause 5 defines the component and why
   the test is over its whole edge set rather than over the path the finding
   happens to print. The finding still names the cycle, so the user sees the
   coupling and can refine it; it no longer fails the gate on a map the user has
   not authored yet.

4. **This requires edge provenance in the graph, which does not exist today.**
   Edges written by `blueprint_delta` must be distinguishable from edges a human
   wrote. That marker is the implementation prerequisite. Severity is decided
   where edge identity is still available, inside cycle detection against the
   graph, never by parsing a rendered cycle path out of a `Finding` message.
   `Finding` therefore does not need to carry per-edge identity, but the check
   must not be reconstructed from its string.

5. **The reporting unit is the strongly connected component.** "Every
   independent cycle" means one finding per cyclic SCC of the dependency graph,
   not every simple cycle (exponential in the worst case) and not a cycle
   basis (the basis is not unique, so the output would not be stable). SCCs
   partition the nodes, so overlap and deduplication do not arise. Order SCCs by
   their lexicographically smallest member id, and pick the representative path
   within an SCC deterministically, so output is reproducible. An SCC is advisory
   when every edge with both endpoints inside it is discovery-observed, and an
   Error otherwise; a single hand-declared edge anywhere in the component makes
   the whole component blocking.

   "Cyclic SCC" means a component of more than one node, or a single node
   carrying a dependency edge to itself. A self-dependency is a real cycle and
   keeps the same provenance and severity treatment; it must not vanish because
   its component has size one.

6. **Scope of the severity change.** This narrows `dec.order-containment-rule`
   only where every edge inside the cyclic component is discovery-observed. Its
   subject, a contradiction between declared containment and declared
   dependency, keeps Error severity and keeps failing lint and hooks. All of its
   other consequences stand: `topological_order` remains the single traversal,
   `cairn frontier` and lint continue to inherit it, `cycle_findings` stays
   dependency-only and each finding still carries a cycle path, and a declared
   blueprint whose child depends on its own ancestor still fails. What clause 5
   changes there is how many findings `cycle_findings` emits, not what they
   contain or which graph they read.

7. **Severity cannot be bolted onto the current traversal.** Two properties of
   `topological_order` (`src/map/integrity.rs`) make a naive severity branch
   unsound, and clause 3 is not satisfied until both are addressed:

   - It reports one cycle per run. `cycle_findings` returns on the first cycle it
     finds, so OmniRoute's five disjoint strongly connected components surface as
     a single `CAIRN_ORDER_CYCLE`. Downgrading that one cycle would hide every
     blocking cycle behind it. Cycle detection must enumerate all independent
     cycles rather than stopping at the first, including after a blocking one is
     found, so a scan can report an Error and an advisory together.
   - It short-circuits containment. `topological_order` calls dependency-only
     `cycle_findings` and returns immediately when it is non-empty
     (`src/map/integrity.rs:83-85`), before the combined containment and dependency
     deadlock branch further down ever runs. If the only dependency cycle is
     discovery-only and becomes advisory, a hand-declared child-to-ancestor
     contradiction would never be evaluated, and the scan would exit zero on
     exactly the contradiction clause 6 promises still blocks. The containment
     pass must therefore run whenever any dependency SCC is reported, advisory or
     Error alike, rather than returning at the first one. Restricting the
     fall-through to advisory components would leave the existing masking bug in
     place: a hand-declared dependency cycle would still hide an independent
     containment contradiction, and clause 6's promise that declared
     contradictions still block would not hold across the two detection paths.

   Both properties need regression coverage;
   `todo.brownfield-nested-package-scan-clean` pins the fixtures.

## Why the other options were rejected

**Option 1, dominant direction only.** Rejected on evidence. Of 30 ancestor and
descendant pairs, 12 lean parent to child, 14 lean child to parent, and 4 are
exact ties, with 11 below a dominance ratio of 2. The canonical fixture in the
todo is itself a 2 to 2 tie, so on the example the rule was designed for it
decides by id sort order, not by evidence. It also under-declares a coupling the
code proves, which is in tension with `dec.brownfield-init-round-trip` clause 2,
and it clears the finding on only one of four affected repositories. Arbitrary
and ineffective is a poor trade for lost information.

**Option 2, ancestor and descendant coupling becomes containment.** Rejected on
cost against benefit. It needs the delta-pipeline work in
`todo.brownfield-parent-package-cycle` verified fact 2, needs a leaf home for the
package root's loose files (hand-nesting the fixture raises
`CAIRN_RECONCILE_ORPHANED_FILE` findings, per evidence item 3 of
`todo.brownfield-parent-child-edge-model`), and would have to supersede
`dec.brownfield-init-round-trip` clause 2. Measured as an edge filter, which is
an optimistic upper bound on it, it clears two of four repositories. Buying a
partial fix at the price of superseding an accepted decision and rebuilding the
delta pipeline is not worth it.

**Option 3, finer granularity.** Rejected on cost and on termination, not on the
cycle table, which did not simulate it. Both sides of the fixture's pair are
whole directories, so resolving it needs splitting below directory level.
Dropping `MIN_FILES` from 3 to 1 already roughly doubles candidate counts;
splitting within directories tends toward one candidate per file, which is 4253
against today's 152 on the largest sample. Nothing guarantees finer nodes stop
cycling, so the cost is certain and the benefit is not.

**Option 4 as originally written, blanket advisory.** Refined rather than
rejected. Downgrading every cycle would retire the Error for hand-declared
blueprints too, which is the case `dec.order-containment-rule` exists for and
which the measurement gives no reason to weaken. Clause 3 keeps the Error where a
human declared the contradiction and relaxes it only where cairn itself inferred
the edges.

## Consequences

- `todo.brownfield-nested-package-scan-clean` is rewritten; it stayed blocked
  until ratification and opened 2026-07-29. Its implementation surface moves
  off `derive_import_edges`: the work is edge provenance, cycle enumeration,
  and a severity branch, not a change to what discovery emits.
- The acceptance shared by `todo.brownfield-parent-package-cycle` and
  `todo.brownfield-nested-package-scan-clean`, that `cairn scan` reports no
  `CAIRN_ORDER_CYCLE` and exits zero after a brownfield round-trip, is corrected
  to the claim this decision can deliver: the finding may still be reported, it is
  non-blocking when every edge inside the cyclic component is discovery-observed,
  and the scan exits zero.
- If the maintainer rejects clause 3, the fallback is to accept that a brownfield
  first scan of a repository with observed coupling usually exits 1 by design, and
  to say so in the brownfield quickstart, since neither measured edge rule avoids
  it. That fallback needs no code.
- Sibling reciprocal cycles, 20 of the 50 pairs measured, are untouched by any
  package-root-only shape rule and are covered by clause 3 for the same reason
  package-root cycles are.
- 2026-08-07 pointer correction, no clause changed:
  `todo.brownfield-nested-package-scan-clean` was decomposed under the sizing
  rule and is now `blocked`. The enumeration half of clause 5 and clause 7 are
  carried by `todo.order-cycle-scc-enumeration`, clause 4 by
  `todo.blueprint-edge-provenance`, and clause 3 plus the severity half of
  clause 5 by `todo.order-cycle-discovery-severity`. Where clause 7 says that
  todo pins the fixtures, read the three of them.

## Revisit triggers

- Discovery gains file-level or symbol-level candidates, which would change the
  aggregation argument in option 3 and would let someone measure whether the
  surviving directory-level cycles are real.
- A measured repository shows a discovery-only cycle that a user considers a real
  defect in their architecture rather than an observation, which would argue for
  keeping the Error.
