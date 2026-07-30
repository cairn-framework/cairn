---
id: res.bootstrap-fixture-repair
nodes:
  - cairn.tests
sources: [src.pr-528-w10-ratification]
date: 2026-07-30
tags: [fixture, scanner-behaviour]
---

# Bootstrap fixture repair: what the clean scan forced

Executing the REPAIR verdict (ratified 2026-07-29, PR #528 sheet W10) on
`tests/fixtures/cairn-bootstrap` took the fixture from 22 pinned warnings to a
zero-finding scan. Getting there surfaced scanner behaviours the plan had not
recorded. All are reproducible with `cairn --file
tests/fixtures/cairn-bootstrap/cairn.blueprint scan --json` at the repair
commit.

## Observations

1. **A loaded research artefact must cite loaded sources.** Research with no
   `sources:` list raises `CAIRN_RESEARCH_MISSING_SOURCES` at Error severity;
   research citing sources the graph has not loaded raises
   `CAIRN_RESEARCH_UNKNOWN_SOURCE` at Warning. Because the fixture's
   `meta/sources/` must stay unreached (it is the filename-rule corpus
   `tests/fixtures_smoke.rs` pins without the loader,
   `todo.artefact-filename-test-fixtures`), the fixture's research cannot be
   loaded either. The repaired fixture therefore splits its corpora: the
   evidence corpus (`meta/sources/`, `meta/research/`) is deliberately
   unclaimed and cited in prose, while the authority corpus (decisions,
   contracts, todos, reviews) is loaded and machine-verified. The split is
   documented in the fixture's blueprint header.
2. **Two `CAIRN_CONTRACT_MISSING` severity data points.** A dangling contract
   pointer surfaced as Warning on the fixture's modules, which all declared a
   (nonexistent) `path`, and as Error in a minimal probe whose module declared
   no `path`. The policy behind the difference was not established. The
   warning-only test fixture in `tests/phase_7_7_ux_foundation.rs` sidesteps
   the question with `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` (a module whose
   declared `path` resolves to nothing), verified to produce exactly one
   Warning, strict exit 1, plain exit 0.
3. **`CAIRN_PROVENANCE_NO_DECISION` appeared only once decisions loaded.** The
   pre-repair fixture loaded no decision artefacts and raised none of these
   findings; after the flattened `meta/decisions/` was pointed at, every node
   without an anchoring decision reported. A repair that introduces decision
   pointers must anchor every node or the finding count grows.

## Consequences

- `todo.blueprint-authorability-eval` gets its clean substrate: iterations to
  a clean scan is measurable from zero. Its prompts must preserve the corpus
  split. Verified failure mode: loading `meta/research/` while
  `meta/sources/` stays unreached breaks the clean baseline (the Error and
  Warning classes in observation 1). Loading both corpora together was not
  exercised and is not claimed either way.
- The two `scan --strict` exit-code tests in
  `tests/phase_7_7_ux_foundation.rs` no longer borrow dirt from a shared
  corpus; they own an inline warning-only project.
