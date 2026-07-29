---
id: dec.contract-node-shape-drift-deferred
nodes:
  - cairn.kernel.scanner
status: deprecated
date: 2026-07-28
informed_by: [res.contract-baseline-rerecord-reachability]
---
# Contract node-shape drift: build parked behind a non-generative re-record surface

**Deprecated 2026-07-29 as fulfilled, not repudiated** (maintainer
ratification, PR #528 sheet W7 as corrected in review). The parking this
decision ordered ran to completion: the prerequisite re-record surface landed
in #515, the enforcer in #516, and `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` is
`enforced` in `docs/registries/spec-rules.md`. Accepting it instead would have
attached present-tense authority to a parking that no longer exists, so it
takes the schema's non-accepted terminal.

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

- While the parking stood, `cairn scan` reported one
  `CAIRN_SPEC_RULE_UNIMPLEMENTED` Info for this rule, naming this decision
  inline, and it did not block `--strict`. The enforcer landed in #516, so
  that Info no longer fires.
- The re-record surface was on the critical path for the enforcer: had it
  never been built, this rule would never have been enforced, and the honest
  response was to withdraw the row rather than ship the check without
  remediation. It landed in #515, so that fallback was never needed.
- This decision was `proposed`; on 2026-07-29 it was deprecated as fulfilled
  rather than accepted, per the note above. While the parking stood, the
  registry cell was valid either way, since a live decision is any decision
  that is not superseded.
