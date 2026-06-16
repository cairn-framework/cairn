---
id: dec.build-and-extension
nodes:
  - cairn.macros
  - cairn.brownfield
  - cairn.provenance
  - cairn.suggested-edges
  - cairn.summariser
status: accepted
date: 2026-06-16
---

# Build and extension modules

## Context

cairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.

## Decision

Keep these as first-class modules:

- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).
- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.
- **Provenance**: trace sidecar primitives and provenance-chain helpers.
- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.
- **Summariser**: LLM-assisted contract summarisation backend.

## Rationale

These are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.

## Consequences

- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.
- Provenance types are consumed by the artefact registry and decision coverage gate.
- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management.
