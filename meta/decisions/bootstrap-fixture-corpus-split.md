---
id: dec.bootstrap-fixture-corpus-split
nodes:
  - cairn.tests
status: accepted
ratification: local
affects:
  - meta/decisions/bootstrap-fixture-corpus-split.md
  - tests/fixtures/cairn-bootstrap/
  - tests/fixtures_smoke.rs
  - tests/examples_gate.rs
  - meta/reviews/rev.bootstrap-fixture-corpus-split-correctness.md
  - meta/reviews/rev.bootstrap-fixture-corpus-split-simplicity.md
date: 2026-07-30
receipts:
  - rev.bootstrap-fixture-corpus-split-correctness
  - rev.bootstrap-fixture-corpus-split-simplicity
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

The bootstrap test fixture keeps two kinds of files apart: rule files
that cairn loads and checks, and evidence files it only points at. This
record makes that split a rule.

The bootstrap fixture splits its artefact corpora. The evidence corpus
(`meta/sources/`, `meta/research/`) stays unclaimed by both fixture
blueprints and is cited in prose only. The authority corpus (decisions,
contracts, todos, reviews) stays loaded and machine-verified, and every
declared leaf node carries an anchoring decision. Leaf is the operative
word: `check_provenance_coverage` (`src/scanner/checks.rs`) requires a
decision only for nodes with no children, and the fixture's own System node
`cairn` carries none, so a claim about every declared node would be false
against the tree this decision governs.

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

## The rubric, applied to this decision

- **Tier**: `local`. It touches one test area (`cairn.tests`), replaces
  nothing, and changes none of the files that need the maintainer's own
  signature: no spec, no registry, no schema, and nothing that ships to
  users. Anyone may sign it once the reviews recorded on it agree, and
  the record keeps who signed separate from which checks ran.
- **Unblocks**: nothing is waiting on it. Signing turns a shipped
  implementation choice into a rule future fixture work can cite instead
  of working the reasoning out again.
- **Alignment**: against `dec.cairn-mission` first, the split keeps the
  fixture doing both of its recorded jobs (the test that requires a
  clean scan, and a set of files kept as untouched evidence) without
  faking citations, so its behaviour stays checkable against what was
  intended. Against
  `dec.north-star-continuous-loop`: goal 1, the repair landed and work
  never waited on this signature. Goal 2, the record ties the split to
  measured evidence, so a later session cannot quietly "fix" it away.
  Goal 3, anyone may sign it, and it waits only until acceptance checks
  run without a person. Goal 4, the choice was put in
  this queue the moment it shipped, not relabelled as settled. Goal 5, this entry
  carries the rubric, so the queue reads it in one pass.
- **Options**: (a) keep the two sets of files split, evidence files
  cited in prose but never loaded, rule files loaded and checked
  automatically, which is what shipped and the recommendation; (b) strip the citations
  from loaded research, leaving files that no longer say where their
  claims came from (rejected in Rationale); (c) also load the evidence files, which an
  earlier test already rules out. The cost of rejecting (a) is research that
  hides its sources, or reversing a premise an earlier test already
  landed.

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
