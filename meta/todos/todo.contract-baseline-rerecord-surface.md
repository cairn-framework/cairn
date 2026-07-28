---
node: cairn.summariser
status: done
created: 2026-07-28
---

# Contract Baseline Rerecord Surface

Provide a non-generative way to record or re-record a node's contract baseline
in `.cairn/state/contract-baselines.json`, so clearing a
`CAIRN_CONTRACT_NODE_SHAPE_DRIFT` Warning never requires an LLM backend.

The enforcer's design gives that file one other writer: `accept()` in
`src/summariser/accept.rs`, which will record at accept time and cannot be
re-entered without generating a draft. Neither writer exists yet; this surface
is the second sanctioned one.
`meta/changes/contract-node-shape-drift/design.md`, under "Prerequisite",
carries the proof and is why this is an unconditional prerequisite of the
enforcer rather than a residual risk.

## Scope

Decide and build the surface. A `--record-baseline` style verb over the existing
contract text, writing the reduced baseline record that
`meta/changes/contract-node-shape-drift/specs/contract-node-shape-drift.md`
defines, is the shape the change anticipates; a different shape is fine if it
meets the acceptance below. It needs two operations: record (or re-record) one
node, and drop one node's entry, so a baseline left behind by a removed node or a
removed contract can be pruned. Nothing else may write the file, so without a
drop operation those entries are unremovable except by hand. Do not write the
drift check itself; that is `todo.contract-blueprint-staleness`.

## Depends on

None. The baseline record this surface writes is defined by
`meta/changes/contract-node-shape-drift/specs/contract-node-shape-drift.md`,
which is delivered.

## Acceptance

- With the summariser set to `disabled` in `cairn.config.yaml`, a user can record
  a node's current shape as its contract baseline, the write goes through the
  same reduced record the enforcer reads, and no draft is generated.
- Recording and dropping are read-modify-write: exactly the requested node's
  entry changes, and every other entry, including inert ones for nodes the
  blueprint no longer declares, survives unchanged.
- Recording requires a node that exists and whose contract loads. A missing node,
  or a missing or unreadable contract, returns an error and leaves the baseline
  file byte-identical, so a contractless node can never acquire a contract
  baseline and can never trip the enforcer.
- Dropping is restricted to inert entries: the node is absent from the
  blueprint, or its contract does not load. Dropping an entry for a declared
  node whose contract loads is an error and leaves the file byte-identical, so
  drop can never silence a live finding without the review the finding asks for.
  Dropping an absent entry is likewise an error.
