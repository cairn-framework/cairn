---
id: dec.loop-reconcile-step
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-07-25
informed_by:
  - res.harness-engineering
refines:
  - dec.loop-command-harness-model
related:
  - dec.unified-cairn-dev-entry
  - dec.no-orchestrator
  - dec.native-todos-first
---
# Plan reconciliation is a required loop asset, not a maintainer habit

## Context

A Cairn development campaign runs across many fresh sessions with no memory
between them. Much of the work is research based, so a landed unit routinely
invalidates a later one. Until now the only thing that repaired the plan was a
maintainer step written in prose in `todo.agent-guidance-program`, run between
units. Prose between units is not part of any unit: a session that ends without
it leaves the next session selecting from a plan the evidence already killed,
and nothing detects that.

`dec.loop-command-harness-model` clause 2 already says procedures belong in
required assets with typed exits, and `dec.unified-cairn-dev-entry` clause 8
puts the canonical procedure in `cairn-dev` loop mode plus exactly its required
closure. The reconciliation obligation had no such home.

## Decision

1. `cairn-loop-reconcile` joins the loop-mode required asset closure as a fifth
   procedure, with typed exits `RECONCILED` and `LOOP HALTED`. Like every other
   member of the closure, failing to load it is `LOOP HALTED`, never
   improvisation.
2. Loop mode reaches it after Verify and before Land, so its edits are staged
   into the unit's single commit and the next fresh session reads the reconciled
   plan from main. A reconciliation outside that commit does not exist.
3. It replaces the old `## Record` step rather than sitting beside it. Decision
   authoring now has one home in the procedure instead of two.
4. It is provenance only. It never selects the next unit, never repeats, never
   retries, and never emits or interprets a terminal token. Selection between
   units stays with the operator or harness under `dec.no-orchestrator`.
5. Status changes go through `cairn todo set`, new units through `cairn todo
   new`, and decisions through `cairn decision new`, per
   `dec.cli-agent-workflow-consolidation` and `dec.native-todos-first`.
   A dependant is opened
   only after every entry in its own `Depends on` list is done; a child gated on
   a verdict is opened only against an accepted decision.

## Rationale

The alternative was leaving reconciliation as maintainer prose and trusting
supervision. That is what produced the gap: the obligation was real, written
down, and still skippable, because nothing in the one thing an iteration must
follow mentioned it. Making it an asset in the closure gives it the same
fail-closed treatment as scope, implement, recovery, and landing, and the
position (after proof, before Land) is what makes it land in the commit rather
than in someone's memory.

Placing it after Verify rather than before is deliberate: a plan reconciled
before the gate can be invalidated by the fix the gate forces.

Cost: one more asset every loop session may load, and roughly 350 bytes of
advertised metadata against the first-turn budget. The router does not grow, and
loop mode is loaded only on explicit invocation, so ordinary interactive
sessions pay nothing but the metadata.

## Consequences

- The maintainer step in `todo.agent-guidance-program` ("End-of-unit
  reconciliation") is now enforced procedure for anything a landing unit can
  see. What stays with the operator is choosing which eligible child runs next,
  which is scheduling and belongs outside Cairn.
- The eight ratified clauses of `dec.loop-command-harness-model` are unchanged;
  this refines clause 2 by naming a fifth procedure asset. No clause is
  contradicted, so no supersession is required.
- `tools/agent-pack/tests/reconcile_step_tests.rs` pins the position and the
  obligations, so a later edit that quietly drops the step or moves it after
  Land fails the build rather than the campaign.
- Those tests only bite because this unit also widened the gates. The workspace
  root is itself a package, so a bare `cargo test` or `cargo clippy` covered the
  root crate alone and every `tools/agent-pack` test had been unexercised by
  `scripts/pre-archive-rust-gates.sh` and CI since the pack landed. Both now run
  `--workspace`, which required converting three `u64 as usize` size casts in
  `first_turn_budget_tests.rs` that the pedantic lint set had never seen.
