---
node: cairn.brownfield
status: open
created: 2026-07-26
related: [todo.brownfield-contract-pointer]
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
  subpackages.
- After `cairn init --from-code` and `cairn change apply brownfield-init` on a
  nested-package project, `cairn scan` reports no `CAIRN_ORDER_CYCLE` finding
  and exits zero.
