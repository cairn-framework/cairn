---
id: dec.cairn-brief-orientation
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-06-26
informed_by: []
related:
  - dec.adopt-cairn-dev-loop
  - dec.beads-task-layer
---

# One-command agent orientation: `cairn brief`

## Context

The core value proposition is that a fresh agent can pick up the next task and
the repo constrains it from doing the wrong thing, without a human babysitting.
Bead cairn-cfw verified this only half-held. `cairn next` already surfaced the
top ready bead, but a safe handoff still required the agent to know an unwritten
sequence: read AGENTS.md and the dev-workflow doc, run `bd ready`, then run
`cairn rationale` on the touched nodes. The binding constraints were pull-only
and easy to skip, and `cairn next` itself was missing from `cairn --help`.

## Decision

1. Add `cairn brief [<id>]`: a single deterministic entrypoint that fuses the
   next (or named) ready unit with its binding accepted decisions, the linked
   node contract, the task body (which carries acceptance criteria), and the
   gates that will judge it. With no argument it resolves the top ready bead.
2. Surface only `accepted` decisions for the linked node as binding, so a
   proposed or superseded ruling never reads as a constraint.
3. When a bead has no `cairn-node:<id>` label, the brief still emits the
   universal gates and prints a hint to bind a node, rather than failing. This
   makes the orientation gap visible instead of silent.
4. Register both `brief` and `next` in `cairn --help`, `docs/commands.md`, and
   `docs/integration-contract.md`.

## Tradeoff: read the passive export, not live Dolt

cairn-cfw noted that `cairn next` reads the committed `.beads/issues.jsonl`
export, which can lag live Dolt, and suggested making it authoritative on live
state. We kept the read-only export approach: `src/state/backlog.rs` is
deliberately a non-mutating reader and Beads remains the single source of truth
(spec and module contract). Shelling out to `bd` would add a hard runtime
dependency on the bd binary and a second write path. Instead the brief prints a
staleness note pointing to `bd ready` as authoritative, preserving the
two-reader cross-check the dev loop already relies on.

## Consequences

- A fresh agent can run `cairn brief` and land on the correct unit with its
  constraints loaded; the existing scan and hook gates still catch a
  decision-violating change at commit time.
- The staleness note keeps the export-vs-Dolt gap honest without coupling cairn
  to the bd binary.
- Binding constraints only flow when a bead carries a `cairn-node:` label; the
  brief nudges toward labelling but does not enforce it.

## Revisit triggers

- The beads export staleness causes a fresh agent to pick the wrong unit in
  practice.
- A live-state query becomes cheap and dependency-free.
