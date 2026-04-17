# Proposal: Phase 2.5 Graph Explorer

## Dependencies

- Requires: `phase-2-artefacts`.
- Execution: MUST run after Phase 2 and before Phase 3.

## Problem/Context

Phase 2 delivers the complete query API: `get`, `neighbourhood`, `rationale`, `dependents`, `depends`, `order`, `lint`, `scan`, `sources`, `research`, `decisions`, `todos`, and `status`. These queries return typed, protocol-neutral JSON covering the full ontology — nodes, edges, artefact layers, and integrity findings.

No human-facing visual interface exists to explore the ontology graph, drill into artefact layers, or inspect relationships between nodes. The CLI serves power users and scripts; MCP (Phase 7) will serve agents. A visual graph explorer serves the third audience: humans who need to understand the architecture at a glance and navigate it by clicking rather than typing.

Building the graph explorer immediately after Phase 2 serves two purposes:

1. **Validation harness.** The UI consumes every query the API exposes. If a query's response shape is awkward, incomplete, or ambiguous for rendering, that is a signal the API needs adjustment — better to learn at Phase 2 than Phase 9.
2. **Living documentation.** A visual map of the ontology makes the system self-explaining. Every subsequent phase benefits from a tool that shows what the system currently knows.

## Proposed Solution

Build a web-based graph explorer served by a `cairn ui` command:

- **Graph view.** Render the structural graph (systems, containers, modules, actors) with edges showing ownership and dependencies. Support pan, zoom, collapse/expand for dense graphs.
- **Node detail panel.** Click any node to drill into its artefact layers: contract, decisions, todos, research, sources, reviews. Navigate layers with expand/collapse or step-through.
- **Edge inspection.** Click dependency edges to see the relationship label and connected nodes. Highlight connected edges when a node is selected.
- **Integrity overlay.** Surface `cairn lint` findings (structural errors, interface contradictions, rationale tensions) as visual indicators on affected nodes and edges.
- **Query-consumer architecture.** The UI SHALL consume `cairn` query output exclusively. No separate data path, no direct file system access, no parallel parsing. The query API is the single source of truth.
- **UI Maintenance Contract.** Define how the UI tracks query schema evolution across subsequent phases without requiring per-phase UI amendments.

## Acceptance Criteria

- `cairn ui` starts an embedded web server and opens the graph explorer in the default browser.
- The graph view renders all nodes and edges returned by `cairn neighbourhood <root> --transitive`.
- Nodes display their type (system/container/module/actor), name, and stable ID.
- Clicking a node opens the detail panel showing all artefact types attached to that node, loaded via the relevant query commands.
- The detail panel supports layer-by-layer navigation through artefact types.
- Dependency edges are visually distinct from ownership edges.
- `cairn lint` findings appear as severity-coded indicators on affected nodes/edges.
- The graph handles projects with 200+ nodes without layout degradation (no overlapping labels, no unreadable clusters).
- The UI Maintenance Contract is documented and defines schema tracking, forward-compatibility rules, and the addendum requirements for Phases 3 and 7.
- All strict Rust gates pass for the `cairn ui` command and embedded server.

## Out of Scope

- Temporal navigation (change proposals, proposed-vs-current split view) — Phase 3 addendum.
- MCP transport (switching from CLI-exec to MCP streaming) — Phase 7 addendum.
- Editing capabilities (the explorer is read-only; mutations go through the CLI).
- IDE embedding, LSP integration, or plugin packaging — Phase 10.
- Mobile or offline support.

## UI Maintenance Contract

This contract governs how the graph explorer stays current across Phases 3-10 without requiring per-phase UI amendments:

1. **Schema tracking.** The UI reads typed JSON responses with a `schema_version` field. Unknown fields are ignored (forward-compatible). Removed fields degrade gracefully with a placeholder.
2. **Artefact auto-discovery.** New artefact types that follow the existing response schema (frontmatter + content + node linkage) render automatically using the generic artefact template. No UI code change required.
3. **Query auto-discovery.** The UI enumerates available query commands at startup. New commands added in later phases appear in the interface without UI code changes.
4. **Phase 3 addendum (temporal UX).** The change system introduces a "proposed vs. current truth" paradigm requiring split-view or overlay rendering. Phase 3 SHALL include a UI deliverable section specifying temporal navigation requirements.
5. **Phase 7 addendum (MCP transport).** The MCP server replaces CLI-exec as the query transport. Phase 7 SHALL include a UI deliverable section specifying the transport adapter switch.
6. **Compatibility notes.** Any phase that alters `CairnQuery` or `CairnResponse` struct shapes SHALL include a one-line UI compatibility note in its acceptance criteria confirming that existing UI rendering is unaffected or specifying the required change.
