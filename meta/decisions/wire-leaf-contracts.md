---
id: dec.wire-leaf-contracts
nodes:
  - cairn.kernel.artefacts
status: accepted
date: 2026-06-27
---

# Wire per-node contract pointers for cairn's leaf modules

## Decision

Author a contract artefact per leaf module under `meta/contracts/<slug>.md` and
wire a `contract` pointer for each leaf node in `cairn.blueprint`, clearing the
`CAIRN_CONTRACT_LEAF_UNCOVERED` (CK003) warnings that `cairn-481` introduced.
Each contract carries `node:` frontmatter matching its declaring node and the
five sections mandated by convention: Purpose, Public interface, Invariants,
Dependencies, Tests.

## Context

CK003 (added by `cairn-481`) warns on every synced leaf node that owns code but
declares no contract. Cairn's own blueprint had 22 such leaves. Before this work
no node declared a contract pointer, so `conventions.md` section 10 described
per-node contracts as "present but not scanner-loaded". Wiring contract pointers
makes them scanner-loaded; that sentence is updated in the same change.

## Rationale and tradeoffs

- Wiring contract pointers (rather than tagging every node `@no-contract`)
  realises the intent of CK003: a navigable, provenance-bearing description of
  each module's interface and invariants, which is the contract layer cairn was
  designed to carry.
- The 22 contracts land across three PRs grouped by area (top-level core,
  kernel, frontends) to respect the repository's per-PR size cap. CK003 emits
  warnings, not errors, and the dogfood gate (`cairn lint` + `cairn hook all`)
  exits zero on warnings, so intermediate PRs merge cleanly while remaining
  leaves keep warning until their PR lands.
- A dogfood test (`tests/conventions.rs::test_blueprint_contract_pointers_resolve`)
  guards every wired pointer: a missing file, missing `node:`, or mismatched
  node now fails the suite.

## Scope

Tracked by bead `cairn-y5f`. This decision covers the convention and the wiring
mechanism; the per-node contract content is authored from each module's source.
