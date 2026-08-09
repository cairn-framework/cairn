---
node: cairn.kernel.map
status: done
created: 2026-08-07
parent: todo.brownfield-nested-package-scan-clean
blocked_by: [todo.order-cycle-scc-enumeration, todo.blueprint-edge-provenance]
related: [dec.brownfield-discovery-cycle-severity, dec.order-containment-rule]
---

# Downgrade a cyclic component whose every edge cairn inferred

Implementation unit split out of `todo.brownfield-nested-package-scan-clean`
under the sizing rule, and the one that delivers the parent's user-visible
outcome: a brownfield first scan reports its observed coupling and exits zero.
It carries clause 3 of `dec.brownfield-discovery-cycle-severity` and the
severity half of clause 5.

Prerequisites landed before this unit: `todo.order-cycle-scc-enumeration`
(`cairn.kernel.map`) in PR #618 and `todo.blueprint-edge-provenance`
(`cairn.kernel.blueprint`) in PR #640.

They were hard prerequisites, not sequencing preferences. Without
per-component enumeration a downgrade would hide every blocking cycle behind
the first one (clause 7), and without edge provenance there would be nothing to
branch on (clause 4). This unit landed in PR #642, and the parent
`todo.brownfield-nested-package-scan-clean` flipped to `done` after it.

## Task

- Branch severity per strongly connected component inside cycle detection,
  where edge identity still exists. A component is advisory when every edge
  with both endpoints inside it carries discovery provenance, and an Error
  otherwise: one hand-declared edge anywhere in the component makes the whole
  component blocking.
- The test is over the component's whole edge set, never over the path the
  finding prints. Clause 4 is explicit that the check must not be reconstructed
  by parsing a rendered cycle path out of a `Finding` message, and `Finding`
  therefore does not need to carry per-edge identity.
- An advisory finding is still reported. Silently dropping discovery-only
  cycles satisfies "exits zero" while violating clause 3: the user must see the
  coupling in order to refine it.
- A self-dependency keeps the same provenance and severity treatment as any
  other component.
- Correct the brownfield quickstart or equivalent adopter-facing copy if it
  still promises a finding-free first scan; the deliverable is a non-blocking
  finding, not no finding.

## Acceptance

- After `cairn init --from-code` and `cairn change apply brownfield-init` on a
  project with a package root and a subpackage that import each other,
  `cairn scan` reports the `CAIRN_ORDER_CYCLE` naming that cycle as a
  non-blocking advisory and exits zero. Assert on the advisory, not just on the
  exit code.
- A test pins a mixed map holding one discovery-only dependency cycle and one
  hand-declared dependency cycle: both are reported, the discovery-only one as
  an advisory and the hand-declared one as an Error, and the scan exits
  non-zero. Reporting only the Error fails this test.
- A test pins the containment case: one discovery-only dependency cycle plus a
  hand-declared child-to-ancestor contradiction in the same map. The scan
  reports the cycle as an advisory and the contradiction as an Error, and exits
  non-zero. This proves the severity branch cannot mask the contradiction.
- A test pins the reporting unit against the alternatives: one component
  containing several simple cycles over the same nodes, with mixed provenance,
  where at least one hand-declared edge is absent from the path the finding
  prints. Exactly one finding is emitted for that component and it is an Error.
  An implementation emitting every simple cycle, or a cycle basis, or deciding
  severity from the printed path, passes every other bullet and fails this one.
  This is the severity half of acceptance bullet 5 of the parent todo; its
  count-and-determinism half is pinned by `todo.order-cycle-scc-enumeration`.
- `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cairn scan --strict` all pass.
