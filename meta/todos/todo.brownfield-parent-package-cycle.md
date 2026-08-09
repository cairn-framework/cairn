---
node: cairn.brownfield
status: done
created: 2026-07-26
related: [todo.brownfield-contract-pointer, todo.brownfield-parent-child-edge-model, todo.brownfield-nested-package-scan-clean]
---

# Resolve the parent/child package cycle in brownfield output

## Problem

On a real brownfield project (a package root with subpackages that import it),
`cairn init --from-code` plus `cairn change apply brownfield-init` leaves one
`CAIRN_ORDER_CYCLE` finding, so `cairn scan` still exits non-zero even after
`todo.brownfield-contract-pointer` cleared the contract warnings.

## Verified facts

1. Discovery emits a package root and its subpackages as flat sibling Modules
   (`blueprint_delta`, `src/brownfield/mod.rs`), while import edges run both
   ways between them, producing a real two-node dependency cycle:
   `parent -> child` from the parent's loose files, `child -> parent` from the
   child importing the parent's loose files.
2. The obvious fix, modelling the parent as a container with its children
   nested, does not work through the current delta pipeline.
   `flatten_nodes` (`src/changes/delta.rs:172`) re-emits every child at top
   level while the parent keeps its `children`, so a nested `## ADDED Nodes`
   section would apply as duplicate ids
   (`CAIRN_INTEGRITY_DUPLICATE_ID`, `src/map/build/mod.rs:53`). Fixing this
   needs delta-pipeline work plus a decision on how brownfield models parent
   packages, so the unit needs decomposition under the sizing rule before it
   can be implemented.
3. Suppressing ancestor/descendant dependency edges without adding containment
   is rejected: it deletes real observed imports and replaces them with
   nothing, because containment in the graph comes from syntactic nesting in
   the blueprint AST (`insert_node`, `src/map/build/mod.rs:52`), never from
   dotted ids.

## Acceptance

- A decision records how brownfield models a parent package and its
  subpackages. Delivered: `dec.brownfield-discovery-cycle-severity`
  (accepted 2026-07-29 by maintainer ratification), informed by
  `res.brownfield-observed-cycle-measurement`.
- After `cairn init --from-code` and `cairn change apply brownfield-init` on a
  nested-package project, `cairn scan` exits zero. Corrected 2026-07-27: the
  original wording required no `CAIRN_ORDER_CYCLE` finding at all, which the
  measurement shows the tested edge rules cannot guarantee across repositories.
  The finding may still be reported and is non-blocking when every edge inside
  the cyclic component is discovery-observed.

## Decomposition (2026-07-27)

Too large for one reviewable PR, per verified fact 2.

At decomposition time, it was blocked on sub-todos:
`todo.brownfield-parent-child-edge-model`,
`todo.brownfield-nested-package-scan-clean`.

The first rules on how brownfield models mutual imports between a package root
and its subpackages and carries the evidence for that ruling. The second
implements the chosen rule and pins the round trip. The original completion
condition was that this todo would flip to `done` when the second landed.

2026-08-08: `todo.order-cycle-scc-enumeration` landed as done in PR #618.
The remaining provenance and severity units landed in PRs #640 and #642.
`todo.brownfield-nested-package-scan-clean` then landed as done, completing
the nested-package round trip and this parent todo.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It kept
the parent-package cycle gate explicit while its prerequisites were unresolved.

2026-08-09 completion audit: the nested-package child and all three
implementation units are done, so this parent todo is done.
