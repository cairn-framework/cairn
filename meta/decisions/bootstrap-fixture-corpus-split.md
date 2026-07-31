---
id: dec.bootstrap-fixture-corpus-split
nodes:
  - cairn.tests
status: proposed
ratification: local
affects:
  - meta/decisions/bootstrap-fixture-corpus-split.md
  - tests/fixtures/cairn-bootstrap/
  - tests/fixtures_smoke.rs
  - tests/examples_gate.rs
  - meta/reviews/rev.bootstrap-fixture-corpus-split-correctness.md
  - meta/reviews/rev.bootstrap-fixture-corpus-split-simplicity.md
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

## The rubric, applied to this decision

- **Tier**: `local`. Mechanical facts: one node (`cairn.tests`, a module
  directly under the system node, so there is no cross-container span); it
  supersedes nothing; and every affected path
  (`tests/fixtures/cairn-bootstrap/`, `tests/fixtures_smoke.rs`,
  `tests/examples_gate.rs`) sits under `cairn.tests`, outside the
  binding-surface allowlist `todo.decision-ratification-tiers` defines: no
  spec, registry, artefact-schema, or shipped-pack path. `cairn pending`
  renders the declared `local` tier and prints the candidate subject hash;
  this is the self-serve class under the ratified boundary, awaiting the
  maintainer's signature through the receipt protocol.
- **Unblocks**: nothing mechanically: no todo blocks on this signature and
  no finding names it. Signing converts the standing corpus split from a
  repair-local implementation choice into a ratified rule future fixture
  work cites instead of re-deriving from the coupling evidence.
- **Alignment**: against `dec.cairn-mission`, the split keeps the fixture
  fit for both recorded purposes (clean-scan gate and reconciler-free
  source corpus) without falsifying citations, so fixture behaviour stays
  investigable against recorded intent. Against
  `dec.north-star-continuous-loop` as its operational strategy: goal 1,
  the repair landed and work continued without waiting on this signature.
  Goal 2, the record ties the split to the verified coupling evidence, so
  a later session re-derives the constraint instead of "fixing" it away.
  Goal 3, it sits in the self-serve class under the ratified boundary and
  waits for a signature only until the tiers machinery makes machine
  acceptance auditable. Goal 4, the choice was enqueued as this proposed
  record the moment the repair shipped it, not relabelled as settled.
  Goal 5, this entry now carries the rubric, so the queue triages in one
  read.
- **Options**: (a) split the corpora, evidence unclaimed and cited in
  prose, authority loaded and machine-verified (this decision, shipped);
  (b) trim citation frontmatter from loaded research, hollowing the
  artefacts (rejected in Rationale); (c) also load the evidence corpus
  (`meta/sources/`, `meta/research/`), unexercised, and foreclosed
  because the smoke-test premise (`todo.artefact-filename-test-fixtures`)
  requires `meta/sources/` to stay unreached. Recommendation: (a). Cost
  of no: hollowed citations, or the reversal of a landed premise plus
  the unexercised clean-scan verification.

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
