---
node: cairn.kernel.scanner
status: done
created: 2026-07-16
---

# Contract Blueprint Staleness

Two of three staleness directions are covered: code-vs-contract by the
interface hash freshness rule (docs/spec.md:342), blueprint-vs-decision by
CAIRN_BLUEPRINT_CHANGE_NO_DECISION backed by BlueprintSnapshot and
NodeFingerprint (`src/scanner/state.rs`, `src/scanner/checks.rs`). The
uncovered direction: contract prose authored against an old node shape
stays silently current after the node's declaration changes.

Fix with existing machinery, no git and no contract frontmatter (both
were proposed and refuted; contracts are purely human-authored per
docs/spec.md:338 and the scanner is deliberately git-free): record the
node's NodeFingerprint as a baseline in a versioned
`.cairn/state/contract-baselines.json` when the contract's interface hash
is recorded or re-recorded (`src/summariser/accept.rs` already re-records
there), then add a Warning-tier check comparing the current
NodeFingerprint against the baseline, naming the changed fields (parent,
kind, edges). Content-based, so formatting-only blueprint edits cannot
false-positive.

Motivation: `res.a2ui-analysis` finding 3 (a2ui pins codebase blueprints
to module-blueprint commits; the adaptation keeps the kernel, drops git).
Warning tier interacts with `scan --strict`, so wording and tier should
get a change proposal before implementation.

Prerequisite delivered: todo.contract-node-shape-drift-proposal (node
cairn.kernel.scanner) is done; its change proposal settles the finding
tier, code, message wording, and the
`.cairn/state/contract-baselines.json` schema. Implement against that
proposal.

## Delivered (status restored 2026-08-03)

The unit shipped: `src/scanner/contract_shape.rs` emits
`CAIRN_CONTRACT_NODE_SHAPE_DRIFT` on every scan and
`src/summariser/accept.rs` records the baseline. A stray pre-landing
edit had reopened this record; it is closed against the shipped code,
not re-enqueued.
