---
node: cairn.brownfield
status: blocked
created: 2026-07-27
related: [todo.brownfield-parent-package-cycle, todo.brownfield-parent-child-edge-model]
---

# Make a nested-package brownfield round-trip scan clean

Implementation unit split out of `todo.brownfield-parent-package-cycle` under
the sizing rule.

Blocked on `todo.brownfield-parent-child-edge-model` (node `cairn.brownfield`),
which rules on how brownfield models mutual imports between a package root and
its subpackages. This is gated on a verdict, not on a unit: stay blocked until
that decision exists with `status: accepted`, then unblock with
`cairn todo set brownfield-nested-package-scan-clean open` and implement the
rule it chose rather than picking one here. A `proposed` decision awaiting the
maintainer is not enough.

## Scope

- Implement the chosen rule where the edges are produced: `derive_import_edges`
  (`src/brownfield/import_edges.rs`) and, if the rule needs shape rather than
  edge changes, `discover` (`src/brownfield/discovery.rs`) and `blueprint_delta`
  (`src/brownfield/mod.rs`).
- If and only if the chosen rule nests nodes, the delta pipeline work in the
  parent todo's verified fact 2 comes with it: `flatten_nodes`
  (`src/changes/delta.rs`) re-emits every child at top level while the parent
  keeps its `children`, so a nested `## ADDED Nodes` section applies as
  duplicate ids (`CAIRN_INTEGRITY_DUPLICATE_ID`, `src/map/build/mod.rs`). That
  half is large enough to split again if it grows past one reviewable PR.
- Cover the round-trip with a test over a nested-package fixture: discovery,
  delta application, then a scan assertion. Assert on the absence of
  `CAIRN_ORDER_CYCLE`, not on a total finding count, so the test fails for the
  right reason.

## Acceptance

- After `cairn init --from-code` and `cairn change apply brownfield-init` on a
  project with a package root and a subpackage that import each other,
  `cairn scan` reports no `CAIRN_ORDER_CYCLE` finding and exits zero.
- A test pins that round-trip and fails if either edge direction reappears
  unmodelled.
