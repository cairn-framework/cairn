---
id: res.contract-baseline-rerecord-reachability
nodes:
  - cairn.summariser
  - cairn.kernel.scanner
sources: [src.summariser-accept-path, src.query-api-draft-generation]
date: 2026-07-28
---
# Re-accepting a contract is unreachable with the summariser disabled

Evidence gathered while designing `meta/changes/contract-node-shape-drift/`. It
constrains any check whose remediation is "re-accept the contract", and it is the
evidence `dec.contract-node-shape-drift-deferred` rests on.

## Question

The proposed node-shape drift check records a baseline when a contract is
accepted, and its remediation is to re-read the contract and record a fresh
baseline. Is that remediation reachable in every repository that can trip the
finding?

## What the code shows

Read at `00c212a` on `main`, recorded as `src.summariser-accept-path`.

1. `Accepted` is a terminal draft state, so a node whose contract was accepted
   cannot re-enter the accept path through its existing draft.
2. Obtaining a fresh draft returns `CAIRN_SUMMARISER_DISABLED` while the
   summariser is disabled.
3. `accept()` is the only writer of accepted contract state, and it runs a
   post-write scan with rollback, so what it records is a state the graph
   accepted.

## Result

A repository that accepted drafts while the summariser was configured, and later
disabled it, holds accepted contracts it cannot re-accept. Any finding whose only
remediation is re-acceptance is unclearable there. At Warning severity that
permanently reddens `cairn scan --strict` with no in-tool remedy.

## What it does not show

Nothing here measures how common that configuration is; no adoption data was
gathered. The finding is about reachability, not frequency. It also says nothing
about whether re-acceptance is the right remediation shape, only that it cannot
be the only one.

## Consequence taken

A non-generative surface for recording, re-recording, and dropping a baseline
becomes a prerequisite of the enforcer rather than a residual risk, recorded in
`dec.contract-node-shape-drift-deferred` and tracked by
`meta/todos/todo.contract-baseline-rerecord-surface.md`.
