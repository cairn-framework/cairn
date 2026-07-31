---
id: dec.decision-ratification-tiers
nodes:
  - cairn.kernel.artefacts
  - cairn.kernel.scanner
  - cairn.kernel.hooks
status: accepted
ratification: binding
date: 2026-07-29
informed_by: [src.pr-528-w8-ratification]
related: [dec.north-star-continuous-loop]
revisit_triggers:
  - "a batch of ten machine-accepted local decisions completes its audit (widening or narrowing input)"
  - "a durable error in a machine-accepted decision (a rule that had to be superseded, not a wording nit)"
---
# Decision ratification tiers: local is machine-acceptable under receipts, binding never is

Accepted 2026-07-29 by maintainer ratification (PR #528 sheet W8), recorded in
substance in `todo.decision-ratification-tiers` and implemented by the
`decision-ratification-tiers` change on 2026-07-30. This record makes the
ruling itself queryable; the todo carries the full protocol text and evidence.

## Ruling

A decision artefact declares `ratification: local` or `ratification: binding`;
absent means `binding`. The loop may set `status: accepted` only on a `local`
decision, only under the receipt protocol (two committed Review receipts from
independent lenses, bound by `subject_hash` to a canonical manifest of
everything the decision governs), and only with a queryable
`ratified_by: machine` marker. `local` is machine-validated, never
self-asserted: single-container node span, no supersession, and a declared
`affects:` list wholly outside the binding-surface allowlist
(`docs/registries/binding-surface.md`). Everything else is `binding` and
maintainer-only, permanently: spec invariants, artefact schemas, registries,
shipped pack content, and supersessions of accepted decisions. The receipt
protocol applies to every `local` acceptance whoever signs; `ratified_by`
records who signed, never which checks run.

## The rubric, applied to this decision

- **Tier**: `binding`. Mechanical facts: its `nodes:` sit inside the single
  container `cairn.kernel` and it supersedes nothing, but it changes the
  artefact schema every adopting repository inherits and its affected paths
  include `src/artefacts/registry/`, `docs/registries/`, and
  `tools/agent-pack/content/`, all inside the allowlist it creates. By its
  own definition the loop may not ratify it; the maintainer did (W8).
- **Unblocks**: `todo.decision-ratification-tiers` (this is its ruling); the
  self-serve class under `dec.north-star-continuous-loop` goal 3;
  `cairn pending` tier rendering from declared data
  (`todo.maintainer-pending-queue` v1 hardcoded the default); the
  machine-auditable acceptance path `dec.bootstrap-fixture-corpus-split`
  self-assessed against.
- **Alignment**: against `dec.cairn-mission`, this is the gate half of
  "decisions and gates": it keeps machine-made rulings investigable
  (receipts, lens identity, queryable `ratified_by`) while keeping the
  inherited surface maintainable by exactly one signer. Goal 1, the long
  tail of local rulings no longer waits on a human round trip. Goal 2, every
  machine acceptance carries auditable evidence of record, so correction
  starts from receipts rather than archaeology. Goal 3, it implements the
  signature boundary at the binding surface, verbatim. Goal 4, a binding
  need discovered anywhere still enqueues rather than self-ratifying, and
  understating `affects:` fails closed at the hook. Goal 5, the queue's tier
  column now renders declared data, so triage reads one field.
- **Options**: (a) keep the all-maintainer gate: correct for binding rulings,
  measured to stall automation on the long tail (three consecutive
  bookkeeping iterations, PRs #518 to #520). (b) Open the gate globally:
  rejected outright, the binding surface must stay maintainer-only forever.
  (c) Two tiers with a receipt protocol and a data-file allowlist (this
  decision, ratified W8). Recommendation: (c). Cost of no: every local
  ruling costs a signature, and the queue grows with exactly the class of
  entries the maintainer gains nothing by reading.

## Consequences

- Implemented by the `decision-ratification-tiers` change: schema fields,
  canonical manifest hasher, scanner checks, range-based commit hook (fails
  closed without a merge base), wire version 8, committed lens prompts, and
  the tier-aware loop assets.
- Widening starts narrow and moves only on audit evidence: after every batch
  of ten machine acceptances the maintainer audits durable-error count; a
  clean batch may support one widening step (two-container span with no
  shared dependant), each widening its own binding decision. A durable error
  narrows the tier instead.
- Extending `docs/registries/binding-surface.md` is a binding decision by
  construction: the file lives inside `docs/registries/`.
