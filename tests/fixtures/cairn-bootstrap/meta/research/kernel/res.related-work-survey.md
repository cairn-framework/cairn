---
id: res.related-work-survey
nodes: [cairn, cairn.kernel.parser, cairn.kernel.artefacts, cairn.kernel.reconciler, cairn.kernel.changes]
date: 2026-04-13
sources:
  - src.karpathy-llm-wiki
  - src.structurizr-blueprint
  - src.openspec-repo
  - src.openspec-deepwiki
  - src.akash-llm-project-wiki
  - src.dual-graph-codex-compact
  - src.adr-tools
  - src.dlthub-map-first
tags: [prior-art, positioning]
---

# Related work: prescriptive ontologies, descriptive wikis, and change workflows

## Question

Where does Cairn sit relative to existing blueprint, wiki, and architectural-enforcement tools? What does it borrow, and what does it deliberately not borrow?

## What we considered

**Structurizr blueprint** is the closest prior art for the architectural layer. C4 hierarchy as a declarative blueprint with strict model/view separation. Cairn borrows the hierarchy and declarative style but drops the view layer (rendering is downstream), adds artefact pointers and path ownership, and adds stable IDs and reconciliation.

**Karpathy's LLM Wiki pattern** (April 2026) describes an LLM-maintained markdown wiki between raw sources and agent. Same substrate as Cairn; opposite direction. Karpathy's wiki is descriptive — the LLM compiles it from sources, inventing schema as it goes. Cairn is prescriptive — the human authors intent upfront. Cairn borrows the three-layer framing (sources → compiled layer → code) and generalizes it into the two-chain model.

**akash-r34's llm-project-wiki** applies Karpathy's pattern to codebases. Descriptive. Diff-based ingest and gap-logging mechanics transfer to Cairn's change-directory model.

**OpenSpec (Fission-AI)** is AI-native spec-driven development with a workflow system (OPSX), change directories, and delta operations (ADDED / MODIFIED / REMOVED / RENAMED) for semantic merging. As of early April 2026, OpenSpec has ~37k GitHub stars and has shipped v1.2.0. Philosophy explicitly includes brownfield support. Cairn and OpenSpec solve adjacent but different problems: OpenSpec is a change-lifecycle workflow (how to propose, design, apply, archive a change); Cairn is a structural reconciliation framework (what the system is, what depends on what, does reality match intent). Cairn borrows OpenSpec's change-isolation pattern and delta semantics wholesale. Cairn deliberately does not adopt OpenSpec's workflow layer — the two tools are complementary and could coexist in one repo.

**Dual-Graph / Codex-CLI-Compact** builds a bottom-up code index and exposes it via MCP. Descriptive where Cairn is prescriptive. Its `graph_neighbors` tool inspired Cairn's neighbourhood query shape.

**ADR tooling (adr-tools, log4brains)** established the ADR format. Cairn adopts it with four frontmatter additions: `nodes`, `revisited`, `revisit_triggers`, `informed_by`.

**Map-first / context-engineering patterns** (dltHub and others) frame LLM skills as intent + skill + map, where the gap is almost always the map. Cairn produces an map for a project. The framing sharpens why Cairn refuses procedural detail: the framework gives the agent a map, not a recipe.

## What we're leaning toward

Borrow the C4 hierarchy from Structurizr, the three-layer framing from Karpathy (generalized to two chains), the neighbourhood query shape from Dual-Graph, the ADR format from adr-tools, the map-first framing from dltHub, and the change-directory + delta pattern from OpenSpec.

Do not borrow OpenSpec's workflow layer. Workflow is a non-goal for Cairn; OpenSpec occupies that space.
