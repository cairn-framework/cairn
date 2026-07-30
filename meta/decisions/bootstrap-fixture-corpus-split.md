---
id: dec.bootstrap-fixture-corpus-split
nodes:
  - cairn.tests
status: proposed
date: 2026-07-30
informed_by: [res.bootstrap-fixture-repair, src.pr-528-w10-ratification]
---
# Bootstrap fixture corpus split

## Context

The REPAIR verdict for `tests/fixtures/cairn-bootstrap` (ratified 2026-07-29,
PR #528 sheet W10) requires the fixture to scan clean, gate-asserted by
`tests/examples_gate.rs`. Separately, the landed
`todo.artefact-filename-test-fixtures` premise requires the fixture's
`meta/sources/` to remain a corpus no reconciler reaches, pinned by
`tests/fixtures_smoke.rs` without the loader. `res.bootstrap-fixture-repair`
verified the coupling between the two: a loaded research artefact must cite
loaded sources, so with `meta/sources/` unreached, loading `meta/research/`
breaks the clean scan (`CAIRN_RESEARCH_MISSING_SOURCES` at Error, or
`CAIRN_RESEARCH_UNKNOWN_SOURCE` at Warning when the research cites the
unloaded sources).

## Decision

The bootstrap fixture splits its artefact corpora. The evidence corpus
(`meta/sources/`, `meta/research/`) stays unclaimed by both fixture
blueprints and is cited in prose only. The authority corpus (decisions,
contracts, todos, reviews) stays loaded and machine-verified, and every
declared node carries an anchoring decision.

## Rationale

Under the two standing constraints, the repair had to choose between
trimming citation frontmatter from loaded research and keeping the evidence
corpus unreached. Trimming was rejected: the scanner classifies sourceless
research as an Error, so the citations would have to be falsified or
removed, hollowing the artefact. Keeping the evidence corpus intact and
unreached preserves every citation as written. The split is this repair's
implementation choice, not a ratified rule; ratifying it as binding for
future fixture work is the maintainer's call, which is why this record is
proposed.

## Consequences

- Neither fixture blueprint may gain a `sources` or `research` pointer while
  the smoke-test corpus premise stands; the fixture blueprint header carries
  the same warning.
- Authoring tasks against the fixture (`todo.blueprint-authorability-eval`)
  must keep prompts inside the loaded authority corpus.
- Loaded fixture artefacts cite evidence files in prose (Provenance
  sections), never in frontmatter refs that the graph would try to resolve.
- Whether loading both evidence corpora together would scan clean was not
  exercised and is left undecided by this record.
