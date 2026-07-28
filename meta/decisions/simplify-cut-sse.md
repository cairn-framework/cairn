---
id: dec.simplify-cut-sse
nodes:
  - cairn.root
  - cairn.sse
status: superseded
date: 2026-07-07
related: [dec.close-blueprint-drift, dec.no-orchestrator]
---
# Cut the SSE spike: delete src/sse.rs and the cairn.sse node

## Context

`src/sse.rs` (372 LOC) was an SSE event-stream parser written as a Gas City
integration spike (issue #101, part of epic #95). The 2026-07-06 four-audit
investigation ratified in `todo.simplify-architecture` (wave 1,
`todo.simplify-cut-sse`) found it had zero internal callers: the only
reference in the crate was its `pub mod sse` export in `src/lib.rs`.

The accepted decision `dec.close-blueprint-drift` (2026-06-03) had declared
the `cairn.sse` node partly on the claim that `sse` is "consumed by the Gas
City adapter". The step-0 check required by the cut task falsified that
claim: no Gas City adapter consuming `cairn::sse` exists, the gas-city
research produced no code changes, and issue #101 remains an aspirational
spike, not a live consumer. The spike's direction was cairn-as-client of
Gas City's stream, so no external system links this module either.

## Decision

Delete `src/sse.rs`, its `pub mod sse` export, `meta/contracts/sse.md`, and
the `cairn.sse` Module node from `cairn.blueprint` (it had no edges). This
supersedes the sse-specific ruling of `dec.close-blueprint-drift`; that
decision's treatment of `cairn.state` and `cairn.watch` stands unchanged.

## Rationale

Dead code with a node, a contract, and a public export overstates the
system's surface and dilutes the dogfooding signal the drift decision
wanted to strengthen. Keeping a reactive-reconciliation client would also
sit awkwardly with `dec.no-orchestrator`: subscribing to an orchestrator's
event stream is harness territory. If a Gas City integration materialises,
a parser is trivial to reintroduce behind a contract, informed by a real
consumer.

## Consequences

- `cairn.sse` no longer exists in the graph; this decision intentionally
  keeps it in `nodes:` as the accepted-decision record satisfying the
  CH001 module-remove gate for the deletion.
- `dec.close-blueprint-drift` is amended in the same change: `cairn.sse`
  dropped from its `nodes:` and the falsified consumer claim annotated.
- Semver: `cairn-framework` 0.1.4 is published with `cairn::sse` public;
  the next crates.io release containing this cut must bump to 0.2.0 (or
  consciously accept the break given the no-consumer evidence).
- The webui eval harness ghost-node scenario keeps using `cairn.sse` as a
  fixture-only id in its frozen fixtures; a comment marks it.
