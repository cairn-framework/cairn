---
id: res.roadmap-audit-health
nodes:
  - cairn.root
date: 2026-08-07
method: primary
---

# Codebase health run, 2026-08-07 (roadmap assumption audit)

The health pass required by todo.roadmap-assumption-audit item 2. Every
finding is enumerated with an owner; defects without an owner got a unit
filed this session.

## Module sizes against the seam rule

Eleven src files exceed the 500-line maximum: cli/mod.rs (3138),
scanner/tests.rs (1159), map/query.rs (1078), ui/mod.rs (863),
summariser/store.rs (740), query_api/change_queries.rs (709),
query_api/mod.rs (675), scanner/mod.rs (640), query_api/registry.rs
(624), map/graph.rs (593), ui/server.rs (524). All are allow-listed via
the modularity markers shipped by todo.modularity-scan-finding. The
external architecture review's deletion test concurs on cli/mod.rs:
small external interface, continue incremental command extraction, do
not split to satisfy a line count. Owner: the adopter-scope question is
todo.module-size-limit-adopter-scope; in-repo splitting continues
opportunistically.

## Dead code

Two stale `#[allow(dead_code)]` attributes on `LocalCommandBackend`
(struct and impl) claimed "will be constructed by CLI wiring in upcoming
task 4.1" while the type is constructed at src/query_api/mod.rs:588.
Removed in this audit (trivial fix). The remaining allows in
query_api/registry.rs (`is_readonly`, `is_mutating`) are valid:
non-test usage does not exist yet, test usage keeps them honest. Owner:
none needed.

## Duplicate copy keys

None. docs/design-system/copy.toml parses clean under a
duplicate-rejecting TOML parser. Owner: none needed.

## Fixture freshness

tests/fixtures/cairn-bootstrap remains gate-asserted to scan clean by
tests/examples_gate.rs (present, in the suite). Owner: the gate.

## Dependency direction

`cairn scan --strict` exits 0 on this tree with a freshly built binary;
no cycle or direction findings. Owner: the gate.

## Standing Info findings, with owners

- CAIRN_DECISION_ACCUMULATION x5 (root, kernel.artefacts, kernel.cli,
  kernel.scanner, ui): the signal itself is the defect; owner
  todo.decision-accumulation-signal (filed this session).
- CAIRN_REVIEW_SUBJECT_UNMATCHED x3 (the rung-three substrate reviews):
  subject_hash matches no recomputed manifest; owner
  todo.convergence-receipt-hash-drift (re-scoped this session to its
  live remainder).
- CAIRN_RESEARCH_ORPHAN x3 at health-run time (res.chatgpt-issue-audit,
  res.skill-absorption, res.spec-rules-anchor-drift), x5 in the tree
  this audit commits: this session's own res.chatgpt-architecture-review
  and res.roadmap-audit-health join them. Expected state for
  decision-less evidence artefacts; folded by strict-green.
- CAIRN_SOURCE_UNVERIFIED x3 (src.huntley-software-factory,
  src.maintainer-design-threads-2026-07-30,
  src.mission-ratification-2026-07-30): known unverifiable externals.
- CAIRN_TODO_STATUS_CONTRADICTION x2: cleared this session
  (todo.build-ci-observation-overlay set open, blockers done;
  todo.workflow-serialises-validation set blocked, blocker live).
- CAIRN_SPEC_RULE_UNIMPLEMENTED x1: carries
  deferred_by dec.revisit-trigger-correlator-deferred; standing by
  ruling.
- CAIRN_DECISION_REFINED_AUTHORITY x13: advisory refinement pointers,
  folded by strict-green. No action.
