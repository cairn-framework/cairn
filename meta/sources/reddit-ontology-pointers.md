---
id: src.reddit-ontology-pointers
file: docs/research/reddit-ontology-comment-2026-07-31.md
verification: tracked
type: thread
date: 2026-07-31
---

# Reddit ontology commenter: pointers and a runtime-modelling critique

A second Reddit commenter responded to the maintainer's cairn post with
research pointers and one substantive critique. The raw comment is
preserved at the tracked path above; the thread URL was not pinned and
none of the cited materials have been inspected, so every claim below
stands on the commenter's account.

## Pointers, as claimed

- Code property graphs, with Joern (joern.io) named as the tool to study.
- SHACL for shape and constraint validation.
- A specification the commenter calls "Apache ossie" (ossie.apache.org as
  given); the reference could not be identified with confidence and may
  be misremembered by the commenter.
- Basic Formal Ontology and Common Core Ontology as candidate foundations
  for a lower ontology.
- The commenter explores software ontology themselves and offered direct
  contact.

## The critique

The commenter argues cairn would not currently meet the bar for a code
ontology because it forgoes the processes required for running code: the
repo mapping itself through its own build and compilation, its CI/CD, its
dependencies, and its runtime. The test posed: can the tool model its own
build, pipeline, and runtime, not just its module structure.

## Relation to existing artefacts

The critique lands on ground the graph already names as open: non-code
domains are analysis-only until a future reconciler lands
(dec.domain-expandability), and no current node models build, CI, or
runtime as first-class graph structure. Recorded as evidence; any
normative move goes through a decision.
