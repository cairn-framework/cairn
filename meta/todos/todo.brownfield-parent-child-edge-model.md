---
node: cairn.brownfield
status: open
created: 2026-07-27
related: [todo.brownfield-parent-package-cycle]
---

# Rule how brownfield models mutual imports between a package root and its subpackages

Decision unit split out of `todo.brownfield-parent-package-cycle` under the
sizing rule. Nothing is implemented here: this unit produces the decision that
`todo.brownfield-nested-package-scan-clean` implements.

## Problem

Discovery emits one candidate per directory, so a package root and its
subpackages become flat sibling Modules. When imports run both ways between
them, both directions are observed in code and both are emitted, which is a
two-node dependency cycle and a `CAIRN_ORDER_CYCLE` Error on the first scan of
a fresh brownfield map. Mutual imports between a package root and a subpackage
are ordinary in Python, so this is the common case, not an edge case.

## Evidence (verified 2026-07-27 against origin/main `7411c5d`, cairn 0.9.0)

Fixture: `pkg/{__init__,config,util}.py` and `pkg/sub/{__init__,extra,helper}.py`,
where `pkg/util.py` imports `pkg.sub.helper` and `pkg/sub/extra.py` imports
`pkg.util`.

1. `cairn init --from-code` writes flat siblings plus both edges:

   ```text
   pkg -> pkg.sub "Observed imports of sub (2 in code)"
   pkg.sub -> pkg "Observed imports of pkg (2 in code)"
   ```

2. After `cairn change apply brownfield-init`, `cairn scan` reports
   `Error: CAIRN_ORDER_CYCLE dependency cycle: pkg -> pkg.sub -> pkg` and exits 1.

3. Containment does not dissolve the cycle. Hand-writing the nested shape
   directly into `cairn.blueprint` (a `Container Pkg` with `Module Sub` inside
   it, bypassing the delta pipeline entirely) reports the identical
   `CAIRN_ORDER_CYCLE`, because `cycle_findings` is dependency-only and both
   edges survive nesting. It also raises three
   `CAIRN_RECONCILE_ORPHANED_FILE` findings, because a container with children
   no longer owns the package root's loose files.

   This is additional to the parent todo's verified fact 2, not a replacement
   for it. The delta pipeline still cannot apply a nested `## ADDED Nodes`
   section, and now the nested shape would not clear the finding even if it
   could. The decision therefore has to rule on the edges, not only on nesting.

4. The cycle is aggregation-induced, not a cycle in the code. The fixture's
   file-level import graph is acyclic: `pkg/__init__.py -> pkg.sub.helper`,
   `pkg/util.py -> pkg.sub.helper`, `pkg/sub/extra.py -> pkg.util`,
   `pkg/sub/helper.py -> pkg.config`. Grouping those files by directory is what
   creates the reciprocal pair. Whether that holds for real projects is
   unproven and worth measuring before choosing a rule that assumes it.

5. Verified separately (`/tmp/bf-nest`, a hand-written nested `## ADDED Nodes`
   section run through `cairn change apply`): the parent todo's verified fact 2
   is accurate. The failure is
   `CAIRN_INTEGRITY_DUPLICATE_ID: duplicate node id "pkg.sub"` at map build,
   not the pre-apply duplicate-add check in `src/changes/validate/mod.rs`,
   because the nested child is one entry in `added_nodes` until `flatten_nodes`
   re-emits it.

## Options to stress-test

1. **Dominant direction only.** Between any pair of candidates importing each
   other, emit the higher-count direction and drop the other, tie-broken by id,
   recording the dropped direction in the change proposal. General (covers
   sibling cycles too) and small, but the first map under-declares a coupling
   that exists in the code, which is in tension with
   `dec.brownfield-init-round-trip` clause 2 calling the flat modules plus
   their inferred edges a faithful cold-start map. Weigh that tension
   explicitly rather than assuming a dropped edge is free.
2. **Ancestor and descendant coupling becomes containment.** When one
   candidate's path is a prefix of another's, emit no dependency edge either
   way and nest instead. Needs the delta-pipeline work in the parent's verified
   fact 2, needs a leaf home for the root's loose files (evidence item 3), and
   leaves leaf-to-leaf mutual imports uncovered. It also reverses an accepted
   rule: `dec.brownfield-init-round-trip` clause 2 calls flat discovered
   modules plus inferred edges the faithful cold-start map and leaves nesting
   to refinement. Choosing it means the decision supersedes that one and marks
   it superseded; `refines` is informational and cannot override a ruling.
3. **Finer granularity.** Split candidates so the mutual import resolves
   between distinct leaves. Evidence item 4 shows this fixture's cycle is
   aggregation-induced, so splitting both sides resolves it; splitting only the
   package root leaves `pkg.util <-> pkg.sub`, which is a narrower and weaker
   option worth separating from the full split when stress-testing. The open
   question is what it costs on a real repository: candidate count, and whether
   `MIN_FILES` grouping survives at all.
4. **Keep the edges, change how the finding reads.** A cycle observed by
   discovery is reported as an advisory rather than an Error, so a first map is
   honest and `cairn scan` still exits zero. This contradicts two accepted
   things at once. `dec.order-containment-rule` routes cycle detection through
   `topological_order` so that a cycle fails lint and hooks rather than
   surfacing as advice, and the parent's acceptance requires no
   `CAIRN_ORDER_CYCLE` finding at all rather than a non-blocking one. Choosing
   it means a superseding decision, a way to carry discovery provenance on an
   edge (the graph holds none today), and a rewrite of the acceptance of both
   the parent and `todo.brownfield-nested-package-scan-clean`, whose test
   contract currently fails if either edge direction reappears. Leaving those
   bodies standing while choosing option 4 makes the implementation unit
   impossible to finish as written.

## Acceptance

- A decision artefact under `meta/decisions/` naming `cairn.brownfield` records
  the chosen rule and why each rejected option was rejected, citing the
  evidence above.
- If the chosen option contradicts an accepted decision, the new decision
  supersedes it explicitly, restates every surviving obligation of the decision
  it supersedes so nothing unrelated is retired by accident, and the superseded
  one is marked. Check every accepted decision the option touches, not only the
  ones named here: option 2 reaches `dec.brownfield-init-round-trip`, option 4
  reaches `dec.order-containment-rule`, and option 1 may reach
  `dec.brownfield-init-round-trip`. Where the acceptance of the parent or the
  implementation child no longer holds under the chosen rule, this unit
  rewrites those bodies too.
- `todo.brownfield-nested-package-scan-clean` names the chosen rule in its
  body. It is unblocked only if the decision landed `status: accepted`; if
  acceptance is the maintainer's call, it stays blocked and this unit says so.
- `cairn scan` reports no new findings.
