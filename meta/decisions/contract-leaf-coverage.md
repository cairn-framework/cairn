---
id: dec.contract-leaf-coverage
nodes:
  - cairn.kernel.map
status: accepted
date: 2026-06-27
---

# Contract-coverage integrity rule for uncovered leaf nodes

## Context

`docs/spec.md:318` states the integrity rule: "Every leaf node should have a
contract. Missing contracts are warnings, except when a node transitions from
ghost to synced (then required)." The rule was specified but unimplemented.
`validate_contracts` only flagged a contract pointer whose file is absent
(`CAIRN_CONTRACT_MISSING`); a leaf that declared no contract at all went
unnoticed. Cairn's own 24 nodes carry zero contracts yet `cairn scan` was clean,
so cairn was blind to the exact drift it exists to catch, on itself
(bead cairn-481).

## Decision

Add `validate_contract_coverage` (`src/map/contract_coverage.rs`, a sibling of
`test_coverage.rs`) wired into `build_graph`. For every leaf node (no children)
that is `Synced` and owns code (has claimed files) but declares no contract
pointer, emit `CAIRN_CONTRACT_LEAF_UNCOVERED` (registry code CK003) at
`Warning` severity. Exemptions: container/parent nodes (the rule is per-leaf),
ghost nodes (no code yet), nodes that already declare a contract pointer
(`validate_contracts`'s job), and nodes tagged `no-contract`.

## Rationale

The rule mirrors the established test-coverage gate exactly: advisory `Warning`
severity, opt-out via a tag, gated through the existing `cairn scan --strict`
exit-code promotion. The dogfood gate (`scripts/dogfood.sh` = `cairn lint` +
`cairn hook all`, used by both the pre-push hook and CI) blocks only on `Error`
findings, so introducing the rule keeps the gate green while making the gap
visible. Surfacing the drift rather than hiding it is the point: cairn now sees
its own 22 uncovered leaves.

The spec's "Error on ghost to synced transition" nuance is deferred. Detecting a
state transition needs prior-state history this layer does not retain; the
test-coverage gate set the precedent of a transition-agnostic `Warning`, and the
strict-mode promotion already provides the hard gate when a project opts in.

## Consequences

- `cairn scan` on the cairn repo now reports 22 `CAIRN_CONTRACT_LEAF_UNCOVERED`
  warnings. This is intended drift signal, not a regression. `cairn scan
  --strict` on the cairn repo now exits non-zero; the dogfood gate does not use
  `--strict`, so push and CI remain green.
- Authoring the 22 leaf contracts is tracked as a separate follow-up bead.
- Test fixtures that build synced leaves with code must now either declare a
  contract or carry the `no-contract` tag to remain finding-free.
- The finding is intentionally not routed in
  `src/query_api/handlers/remediate.rs`, matching the test-coverage gate's
  unrouted `CAIRN_TEST_COVERAGE_MISSING`. The existing contract-issue arm sets
  `has_orphans`, which would wrongly suggest `cairn refine` (code drift) for a
  leaf whose code is fine and only lacks a contract artefact. A dedicated
  "author a contract" remediation action for both coverage warnings is a
  possible follow-up.
