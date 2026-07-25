---
node: cairn.kernel.cli
status: open
created: 2026-07-22
---

# Agent Pack OMP Adapter

## Priority

P2. Implement and validate the OMP adapter after Claude proves the shared
lifecycle. This unit completes at a validated, unpublished adapter; public
publication is `todo.agent-pack-omp-publication`, gated on the treatment
verdict, so this unit is completable in one iteration without waiting on
treatment.

## Depends on

`todo.agent-pack-claude-bootstrap` (shared lifecycle) and
`todo.agent-guidance-router-playbooks` (the canonical loop mode and its
procedure closure the adapter maps).

## Scope

- Add the OMP adapter through the same lifecycle command family and ownership
  manifest.
- Map the logical `cairn-dev` entry, its private JIT references, and its
  explicit loop mode with required scope, implement, recovery, and landing
  procedures to OMP-native discovery and loading surfaces.
- Keep adapter differences as data and harness-native configuration. Do not
  move iteration, scheduling, or workflow semantics into Cairn core or the
  ownership manifest.
- Validate against a live OMP installation before declaring the adapter
  supported.
- Validate a Ralph-style campaign in which OMP owns repetition and invokes the
  shared resolver to buffer and hash the locally installed closure once into
  an immutable campaign snapshot. Every fresh session loads prompt and
  procedures only from that snapshot and records its bundle and asset hashes.
  No Cairn-owned loader may continue, retry, or select work.

## Acceptance

- Install, update, status, uninstall, legacy adoption, drift handling, and
  user-file protection match the Claude contract.
- A fresh OMP host discovers the general entry, loads exactly one JIT
  reference for a routed task, and reaches the same canonical loop mode through
  an explicit OMP-native invocation.
- A live campaign smoke proves that a changed prompt or required procedure
  halts before work and that a new installed revision is adopted only by
  starting a new campaign.
- The adapter is validated against a live OMP installation but not published;
  publication is handed to `todo.agent-pack-omp-publication`.
- No unverified adapter row ships as fact.
