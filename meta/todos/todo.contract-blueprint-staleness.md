---
node: cairn.kernel.scanner
status: done
created: 2026-07-16
---

# Contract Blueprint Staleness

Two of three staleness directions have coverage: code-vs-contract partially, by
the opt-in `interface:` block check (`CAIRN_CONTRACT_INTERFACE_DRIFT`), and
blueprint-vs-decision by `CAIRN_BLUEPRINT_CHANGE_NO_DECISION`. The uncovered
direction: a contract can stay marked current after its recorded review baseline
no longer matches the node's declared shape.

Build the check against
`meta/changes/contract-node-shape-drift/specs/contract-node-shape-drift.md`,
which is the binding contract for this unit. Its `design.md` holds the
rationale, including the two premises in this todo's original wording that the
proposal disproved.

Motivation: `res.a2ui-analysis` finding 3 (a2ui pins codebase blueprints to
module-blueprint commits; the adaptation keeps the kernel, drops git).

## Depends on

`todo.contract-baseline-rerecord-surface` (node `cairn.summariser`). Without a
non-generative way to re-record a baseline, this check can emit a Warning that a
repository with the summariser disabled cannot clear. The proposal's evidence is
in `meta/changes/contract-node-shape-drift/design.md`, under "Prerequisite".

`todo.contract-node-shape-drift-proposal` (node `cairn.kernel.scanner`),
delivered by `meta/changes/contract-node-shape-drift/`.
