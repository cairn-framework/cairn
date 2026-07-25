---
node: cairn.kernel.cli
status: done
created: 2026-07-22
---

# Agent Guidance Apply, Proof, and Authority

## Priority

P1. Keep separate from retrieval guidance so its effect can be measured.

## Depends on

`todo.agent-guidance-baseline` and
`todo.agent-pack-canonical-foundation`.

## Problem

The shipped `cairn-apply` skill still hardcodes a Cargo battery, prescribes a
commit after every task, leaves CC002 unexplained, and does not distinguish
internal gates from proof at the user's claim boundary. The binary's
language-aware acceptance gate already exists; the skill is stale.

## Scope

- Make `cairn-propose` name the outcome, nearest observable acceptance
  boundary, evidence that proves it, and exclusions.
- Remove the Cargo-only battery and per-task commit prescription from
  `cairn-apply`.
- Follow the target repository's instructions and configured gates, then run
  `cairn change accept` for Cairn's acceptance boundary.
- Explain CC002 where it is used or remove the dangling skill reference.
- Name concrete mutation authority, precise denials, and state-preserving
  recovery for code, git, release, and external actions. Do not add a Cairn
  authorization engine.
- Keep optional parallel guidance harness-neutral. Cairn frontier and
  blueprint path claims may identify safe disjoint scope, but the target
  harness owns worker topology and scheduling. The invoking job remains
  responsible for integration, claim-matched proof, and returning control to
  the router.

## Acceptance

- A TypeScript host follows its own gates without encountering Cargo
  instructions.
- A Rust host still reaches the configured or fallback acceptance battery.
- Behavioural, UI, generated-output, and operational examples ask for evidence
  at the actual claim boundary rather than treating a green internal check as
  universal proof.
- Denied or ambiguous consequential operations preserve state and return a
  clear blocked outcome.
- Parallel examples never imply that Cairn schedules agents.

