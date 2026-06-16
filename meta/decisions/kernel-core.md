---
id: dec.kernel-core
nodes:
  - cairn.kernel.blueprint
  - cairn.kernel.artefacts
  - cairn.kernel.map
  - cairn.kernel.scanner
status: accepted
date: 2026-06-16
---

# Kernel core: blueprint, artefacts, map, and scanner

## Context

CAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.

## Decision

Keep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.

## Rationale

Separating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.

## Consequences

- Changes to the blueprint grammar must update parser tests and scanner integration tests.
- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.
- The scanner owns caching and incremental-scan state.
