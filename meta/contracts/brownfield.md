---
node: cairn.brownfield
---

# Contract: cairn.brownfield

## Purpose

Phase 9 brownfield extraction: typed candidate, confidence, and coupling-score
helpers plus orphan grouping shared across `cairn init --from-code`,
`cairn refine`, and the suggest engine. It turns an existing, un-blueprinted
codebase into proposed graph nodes by traversing the filesystem, scoring
directory cohesion, and clustering orphaned files into actionable suggestions.

## Public interface

- `heuristics` (re-exported): `Candidate` (path-derived `id` via
  `Candidate::new`), `CandidateConfidence` bucket enum, `coupling_score` and
  `classify_score`, and the threshold constants `MIN_CANDIDATE_FILE_COUNT` (3),
  `DIRECTORY_DEPTH_LIMIT` (4), `EDGE_OBSERVATION_THRESHOLD` (2), `CONFIDENCE_HIGH`
  (2.0), `CONFIDENCE_MEDIUM` (1.0).
- `onboard`: `analyze(&[Finding]) -> OnboardReport`, grouping orphan findings
  into `OrphanCluster`s with a `ClusterSuggestion`, plus `render_human` and
  `render_json`.
- `mod` top level: `stub_contract`, `write_change`, and `blueprint_delta` build
  a brownfield change directory (proposal, blueprint delta, stub contracts).
- Further submodules: `discovery`, `init`, `refine`, `suggest`, `summarise`,
  `interview`, `templates`.

## Invariants

- Coupling score is `(internal_imports + 1) / (external_imports + 1)`; the +1
  offset avoids division by zero and rewards internal cohesion at small totals.
- `Candidate.id` is path-derived by construction (`path_derived_id`); a
  different id source requires the explicit `with_id`.
- Onboard groups only findings with code `CAIRN_RECONCILE_ORPHANED_FILE`;
  directories matching `IGNORE_PATTERNS` classify as ignore candidates.
- Blueprint node names are emitted as barewords (`bareword`), since the grammar's
  name slot is a bareword, not a string.

## Dependencies

`cairn.brownfield -> cairn.kernel.map`: reads orphan findings, consuming the
`Finding` type (`crate::map::graph::Finding`) in `onboard::analyze`. It is
dispatched by the CLI (`cairn.kernel.cli -> cairn.brownfield`) for the onboard
command. Also uses the crate-wide `CairnError` for write paths.

## Tests

Unit tests live in `#[cfg(test)]` modules within `src/brownfield/mod.rs`,
`heuristics.rs` (score and bucket classification), and `onboard.rs` (orphan
grouping, classification, and human/JSON rendering), exercised alongside the
crate's integration tests under `tests/`.
