---
id: res.brownfield-observed-cycle-measurement
nodes:
  - cairn.brownfield
date: 2026-07-27
method: primary
---

# What brownfield discovery emits on real repositories, and whether the tested parent/child edge-suppression rules clear CAIRN_ORDER_CYCLE

## Question

`todo.brownfield-parent-child-edge-model` lists four options for modelling
mutual imports between a package root and its subpackages, and its evidence
item 4 leaves one measurement open: the fixture's cycle is aggregation-induced,
but "whether that holds for real projects is unproven and worth measuring
before choosing a rule that assumes it".

Two questions follow from that, and both had to be answered before a rule could
be chosen:

1. How often does discovery emit reciprocal ancestor/descendant edges on real
   code, and is one direction reliably dominant?
2. Does suppressing those edges actually clear the `CAIRN_ORDER_CYCLE` Error
   that the parent todo's acceptance requires?

## Method

Thirteen local repositories were snapshotted into temporary directories, source
files only, for the five extensions `discover` accepts. The snapshot filter
excluded `.git`, every other dotted directory, `node_modules`, `target`,
`.venv`, `venv`, `__pycache__`, `dist`, `build`, `.next`, and `vendor`. That is
neither a subset nor a superset of `is_ignored_dir`
(`src/brownfield/discovery.rs:188`), which additionally ignores `openspec` and
`meta` and does not ignore dotted directories generally. The difference had no
effect on these measurements: no candidate in any of the thirteen snapshots is
rooted in `meta/` or `openspec/`, checked directly. `cairn init --from-code`
then `cairn change apply brownfield-init` ran in each snapshot with cairn 0.9.0
built from `b4186a9`.

Five snapshots produced any observed import edges at all; the other eight
produced candidates but zero edges, so they cannot exhibit the finding and are
excluded from the table.

Candidate rules were then applied by rewriting the applied `cairn.blueprint`'s
edge lines and re-running `cairn scan`:

- **opt1** ("dominant direction only"): for each reciprocal pair keep the
  higher-count direction, ties broken by the lexicographically smaller source id.
- **opt2** ("ancestor and descendant coupling becomes containment"): drop both
  directions whenever one candidate's path is a prefix of the other's. This
  measures only the edge half of option 2. It is an optimistic upper bound on
  that option: adding real containment adds children-before-parent constraints
  (`dec.order-containment-rule`) that can only introduce further contradictions,
  never remove one.
- **opt1+opt2**: both filters.

Cycle results below are `cairn scan`'s own `CAIRN_ORDER_CYCLE` findings, not an
external graph analysis. `cairn scan` reports the first cycle it finds, not every
cycle, so "cycle" in the table means at least one survives.

## Findings

| repo | candidates | edges | reciprocal pairs | ancestor | sibling | baseline | opt1 | opt2 | opt1+opt2 |
|---|---|---|---|---|---|---|---|---|---|
| automem | 12 | 21 | 2 | 2 | 0 | cycle | cycle | clean | clean |
| GNSS-watch | 15 | 2 | 1 | 1 | 0 | cycle | clean | clean | clean |
| AMA-Bench | 6 | 2 | 0 | 0 | 0 | clean | clean | clean | clean |
| OmniRoute | 152 | 425 | 42 | 25 | 17 | cycle | cycle | cycle | cycle |
| cairn | 27 | 96 | 5 | 2 | 3 | cycle | cycle | cycle | clean |

1. **A first map that observes any coupling usually fails.** Four of the five
   edge-emitting snapshots fail `cairn scan` on a first map, exit 1; that is four
   of thirteen snapshots overall, since eight emitted no edges and cannot exhibit
   the finding. AMA-Bench is the sole clean edge-emitting snapshot. Edge count
   alone does not predict it: AMA-Bench and GNSS-watch both emit exactly two
   edges and only GNSS-watch is cyclic.

2. **Neither tested suppression rule clears the finding everywhere.** Option 1 alone clears
   one of four affected repositories, option 2's edge filter clears two, both
   together clear three. OmniRoute (152 candidates, 425 edges) stays cyclic under
   every combination. Only these two rules and their combination were simulated:
   option 3 changes what discovery emits rather than filtering edges and was not
   simulated (see finding 6), and option 4 changes severity rather than shape, so
   the table does not speak to either. Within that scope, the acceptance shared by
   `todo.brownfield-parent-package-cycle` and
   `todo.brownfield-nested-package-scan-clean`, that `cairn scan` reports no
   `CAIRN_ORDER_CYCLE` and exits zero after a brownfield round-trip, is not
   reachable by suppressing parent/child edges. Finding 5 gives the reason it is
   unlikely to be reachable by any rule that only reshapes package roots, but that
   is an inference from the surviving cycles, not a measured result for option 3.

3. **A parent/child rule addresses at most 60 percent of the pairs.** Across all
   five repositories there are 50 reciprocal pairs, 30 ancestor/descendant and 20
   sibling. On OmniRoute the split is 25 to 17. Whatever is decided about package
   roots leaves sibling reciprocity untouched.

4. **Dominance is close to a coin flip, so option 1's tiebreak is arbitrary.** Of
   the 30 ancestor/descendant pairs, 12 lean parent to child, 14 lean child to
   parent, and 4 are exact ties. Eleven of the 30 have a dominance ratio below 2.
   The canonical fixture in `todo.brownfield-parent-child-edge-model` is itself a
   2 to 2 tie (`pkg -> pkg.sub` and `pkg.sub -> pkg`, both "2 in code",
   reproduced 2026-07-27), so on the very example the rule was written against,
   option 1 decides by id sort order rather than by evidence.

5. **The surviving cycles are not parent/child pairs.** With both filters
   applied, OmniRoute's first reported cycle is
   `open-sse.config -> src.shared.constants -> src.shared.utils -> src.lib.db ->
   open-sse.utils -> open-sse.config`, a five-node cycle across unrelated
   subtrees. What this establishes is bounded: filtering parent/child edges does
   not remove every directory-level cycle. It does not establish that those
   cycles exist in the file-level import graph. Finer-grained discovery was never
   simulated, so a directory-level cycle here could still be aggregation-induced
   in the same way the canonical fixture's is. Whether directory cycles survive
   at file granularity is unmeasured.

6. **Option 3's cost is measurable and large.** Resolving the fixture's cycle by
   finer granularity requires splitting below directory level, because both sides
   of the pair are whole directories. Dropping `MIN_FILES` from 3 to 1 alone
   roughly doubles the candidate count (OmniRoute 153 to 326 qualifying
   directories, automem 12 to 23, cairn 27 to 53). Splitting within directories,
   which is what the fixture actually needs, tends toward one candidate per file:
   4253 source files sit in OmniRoute's qualifying directories today against 152
   candidates. Nothing guarantees termination either, since finer nodes can still
   form cycles.

7. **`cairn scan` reports one cycle per run, not every cycle.** OmniRoute's
   baseline map holds five disjoint strongly connected components (largest 42
   nodes) and `cairn scan` printed a single `CAIRN_ORDER_CYCLE`. Every "cycle"
   cell in the table above therefore means at least one survives, and a rule
   evaluated by "did the finding disappear" is measuring the first cycle only.
   Any future change that varies severity per cycle has to reckon with this: a
   downgraded first cycle would mask a blocking one behind it.

## Limitations

- Five repositories with edges is a small sample, and four are the author's own
  projects. The direction of finding 2 is robust (a single counterexample repo
  is enough to disprove "this rule clears the finding"), but the proportions in
  findings 3 and 4 should not be read as population statistics.
- Snapshots dropped non-source files, so a repository whose grouping depends on
  ignored directories could differ slightly from an in-place run.
- opt2 was measured as an edge filter, not as real containment. See Method for
  why that is an upper bound rather than an approximation.
- The inputs were local working trees on one machine, captured 2026-07-27, with
  no revision pinned and no snapshot archived. A third party cannot rerun this
  and get identical counts. What is reproducible is the procedure, and the two
  conclusions that carry the ruling (a tested rule fails on at least one real
  repository, and dominance is not consistent) are robust to sample drift in a
  way the exact proportions are not.
