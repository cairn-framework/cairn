---
node: cairn.reconcile
status: open
created: 2026-07-25
---

# Node Symbol Coverage

## Priority

P2. Query implementation, raised by measured retrieval evidence rather than by
output size. It gates nothing in the agent-guidance programme; no guidance may
consume it before it is delivered and verified.

## Problem

`cairn get <node> --symbols` and `cairn locate <symbol>` return almost nothing
for a Rust binary crate. `rust_is_exportable` (`src/reconcile/code.rs:63-69`)
admits an item only when it carries a `visibility_modifier`, so a crate whose
items are crate-private exposes no symbols at all. Measured on the pinned
ripgrep fixture (`4649aa97`), `cairn get crates.core.flags --symbols` returned
one symbol for a module whose `defs.rs` alone declares 104 structs, and
`cairn locate TypeList` returned an empty array. Python has no equivalent
filter, so the pinned flask fixture returned 688 symbols across sixteen files.

The consequence is measured, not hypothetical. In
`res.loop-efficiency-observations` (2026-07-25 entry) the two symbol-bearing
compositions, primitive and topology-first, scored 0.000 recall on both ripgrep
tasks and 1.000 on both flask tasks. The composition was not the variable; the
symbols were never in the graph to retrieve. The other two candidates,
bundle-centred and `context_projection_v1`, scored 0.200 on ripgrep IMP and
0.000 on ripgrep LOC, reaching 0.500 and 0.286 on the flask strata. Their single
ripgrep hit comes from `bundle.dependencies[]`, which carries dependency symbols
that `get --symbols` never returns, so that exception is itself evidence of the
coverage gap rather than a counter-example to it.

## Scope

- Decide whether interface-hash exportability and query-visible symbol coverage
  should remain the same predicate. They serve different jobs: an interface hash
  wants the public surface that downstream nodes can break against, while a
  navigating agent wants the definition sites inside the node it was routed to.
- If they separate, keep the interface hash on the exported set so no existing
  drift or ghost semantics change, and widen only the query-visible set.
- Cover the same question for TypeScript, whose `src/reconcile/typescript.rs:94`
  predicate has the same shape (`visibility_modifier` or `export`).
- Re-run the frozen context-bundle evaluation harness in
  `archive/strongholds/agent-context-bundle-evaluation/evidence.tar.gz` against
  the delivered change and report the ripgrep recall delta.

## Non-goals

- No RAG, full-text, or fuzzy symbol search. `locate` stays exact-match.
- No new stored state or second source of truth.
- No change to interface-hash drift, ghost, or synced semantics unless the
  decision below explicitly sanctions it.

## Acceptance

- A decision artefact records the ruling before implementation: this changes the
  public behaviour of `cairn get --symbols` and `cairn locate`, which
  `todo.agent-context-bundle-evaluation` requires to be escalated to a decision.
- Rust and TypeScript nodes return their in-node definition sites for the
  queries an agent uses to navigate.
- Interface-hash output is unchanged for every node in this repository, proven
  by a clean `cairn scan` with no drift findings introduced.
- The re-run reports ripgrep recall from the same frozen manifest, so the gain
  is measured on the corpus that surfaced the gap.

Informed by: res.loop-efficiency-observations

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves investigable. It improves investigation when symbols need precise graph coverage.

2026-08-07 audit (todo.roadmap-assumption-audit): keep as written.
