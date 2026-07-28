---
id: dec.contract-node-shape-drift-deferred
nodes:
  - cairn.kernel.scanner
status: proposed
date: 2026-07-28
informed_by: [res.contract-baseline-rerecord-reachability]
---
# Contract node-shape drift: build parked behind a non-generative re-record surface

## Context

`meta/changes/contract-node-shape-drift/` designs a Warning-tier check that
compares a node's current blueprint shape against the shape its contract was last
reviewed against, and lands a `pending` row for it in
`docs/registries/spec-rules.md`.

The design proved that the check cannot ship on its own. It would have baselines
written by `accept()` in `src/summariser/accept.rs`, a writer that does not exist
yet and that this unit does not build. That path cannot be re-entered without
generating a draft, per `res.contract-baseline-rerecord-reachability`, so a
repository that accepted drafts and later disabled the summariser would receive a
Warning whose only remediation is re-enabling an LLM backend.

## Decision

The rule stays `pending`, and its build is parked behind one prerequisite
capability: a non-generative surface for recording, re-recording, and dropping a
node's contract baseline, tracked as
`meta/todos/todo.contract-baseline-rerecord-surface.md` (node
`cairn.summariser`). The registry row names this decision in its `Deferred-by`
cell, so the parking is visible in the `CAIRN_SPEC_RULE_UNIMPLEMENTED` message
rather than inferable only from a todo's `Depends on` list.

The enforcer todo `todo.contract-blueprint-staleness` stays `blocked` until that
surface lands. The rule is promoted to `enforced`, and its code allocated in
`docs/registries/error-codes.md`, by the enforcer's own commit, per
`docs/conventions.md` rule 2.

## Rationale

Shipping a Warning-tier finding that some repositories cannot clear is the defect,
not a footnote: `cairn scan --strict` exits non-zero on any Warning, so an
unclearable Warning turns a green gate permanently red for a class of users with
no in-tool remedy.

Demoting the finding to Info was rejected in the change's `design.md`: it would
park an exact structural signal next to the honest CK004 advisories and weaken
both. Deferring the build instead keeps the tier honest and costs only time.

Recording the deferral here rather than leaving the `Deferred-by` cell empty
matches `docs/registries/spec-rules.md`, which defines the deliberately-deferred
case as a build parked behind a prerequisite capability with the rationale in a
decision artefact. An empty cell would state that no deferral is recorded, which
is not true of this rule.

## Consequences

- `cairn scan` reports one `CAIRN_SPEC_RULE_UNIMPLEMENTED` Info for this rule,
  naming this decision inline, until the enforcer lands. It does not block
  `--strict`.
- The re-record surface is on the critical path for the enforcer. If it is never
  built, this rule is never enforced, and the honest response is to withdraw the
  row rather than ship the check without remediation.
- This decision is `proposed`. Accepting it is the maintainer's call; the
  registry cell is valid either way, since a live decision is any decision that
  is not superseded.
