---
node: cairn.root
status: done
created: 2026-07-10
---

# GitHub Issues Cleanup

Triage the roughly 26 legacy open issues filed in April to May 2026 (the #1 to
#108 range, listed in local://open-issues.md) and clear every one that is older
than 2026-06 either by closing it with a reason or by re-minting it as a native
cairn todo linked back to the issue.

## Problem
The open-issue list carries a long tail of pre-beads and pre-simplify-architecture
items. Many are superseded by decisions that landed since (native todos, beads,
the no-orchestrator stance, the simplify-architecture programme) or are research
sketches that never became work. Left open they mislead new contributors about
what is actually live.

## Evidence
- local://open-issues.md lists the legacy issues #1, #2, #3, #4, #5, #45, #47,
  #63, #66, #67, #68, #69, #70, #72, #86, #95, #96, #97, #98, #99, #100, #101,
  #102, #103, #104, #106, #107, #108.
- Superseding artefacts exist: dec.native-todos-first, dec.no-orchestrator,
  dec.simplify-cut-sse, dec.brownfield-init-round-trip, dec.cairn-identity,
  dec.query-json-schema-version, webui-json-schema-version, dec.user-surfaces,
  dec.beads-task-layer, artefact-organization-and-provenance,
  dec.explore-teaches-provenance, dec.context-edges, close-blueprint-drift,
  dec.loop-resolves-knowable-gaps.

## Proposed approach
For each legacy issue decide one of two outcomes:

- Close (stale or superseded): write a one-line close reason that names the
  superseding decision or PR. No native todo.
- Re-mint (still valid): create a native cairn todo carrying a `gh:#NNN` reference
  line, then close the original issue pointing at the new todo.

Do NOT edit, comment on, or close any issue whose verdict belongs to the
todo.capture-feedback-issues pass (#232 to #247). This cleanup covers only the
legacy #1 to #108 set.

## Buckets and per-issue action
### Close as superseded by the beads-era or simplify-architecture work
- #99 Beads StateBackend implementation and schema: superseded by bead adoption
  (dec.beads-task-layer, bead-github-sync). Close citing beads adoption.
- #97 Pluggable StateBackend trait (filesystem default): superseded by bead
  adoption (dec.beads-task-layer). Close citing beads adoption.
- #102 Change-lifecycle skills plus scaffold (cairn-propose/apply/archive,
  `cairn change new`): superseded by dec.native-todos-first (native todos replace
  OpenSpec tasks) and the existing cairn-* skills. Close citing dec.native-todos-first.
- #103 Tasks-as-beads inside a change: superseded by dec.beads-task-layer. Close
  citing that decision.
- #104 OpenSpec retirement migration plus registries-as-queries: superseded by
  dec.native-todos-first and beads. Close citing those.
- #98 Stable JSON output and documented exit codes: folded into the #240 work in
  todo.capture-feedback-issues plus dec.query-json-schema-version and
  webui-json-schema-version. Close pointing at #240.
- #96 Define the CAIRN integration contract for orchestrators: superseded by
  dec.no-orchestrator. Close citing that decision.
- #95 Epic: orchestrator-agnostic CAIRN: superseded by dec.no-orchestrator. Close
  citing that decision.
- #101 SSE event consumer spike: superseded by dec.simplify-cut-sse (SSE cut).
  Close citing that decision.
- #100 adapters/gascity reference pack: gas-city never adopted (dec.no-orchestrator).
  Close citing that decision.
- #106 Brownfield discovery/init/refine layer: superseded by
  dec.brownfield-init-round-trip. Close citing that decision.
- #67 Reconcile decision artefacts that enumerate folder contents: superseded by
  close-blueprint-drift and dec.loop-resolves-knowable-gaps. Close citing those.
- #107 Extract suggested_edges module from provenance: verify during the pass
  whether the simplify-architecture provenance refactor (#223 to #231) absorbed it;
  if so close citing that programme, otherwise re-mint todo.provenance-suggested-edges
  with `gh:#107`.

### Close as research-era or exploration-deferred
- #47 Explore project naming alternatives: superseded by dec.cairn-identity (name
  settled). Close citing that decision.
- #66 Explore a CLI/TUI graph viewer (mermaid terminal): superseded by
  dec.user-surfaces (webui is the graph surface). Close citing that decision.
- #45 Explore mapping .gitignore as a CAIRN-visible layer: exploration, never
  adopted. Close as exploration-deferred.
- #1 Research: diff view UI for desired vs actual graph: research-era; partially
  realised by the webui graph and the frontier-query decision. Close as research-era.
- #2 Research: graphify as differential oracle: research-era; graphify not adopted.
  Close as research-era.
- #3 Research: blueprint provenance reconcile loop: superseded by
  artefact-organization-and-provenance and dec.explore-teaches-provenance. Close
  citing those.
- #4 Research: obligations as typed edges and a DSL: research-era; folded into
  contract edges (dec.context-edges). Close as research-era.
- #5 Research: observation artefact and runtime telemetry: research-era; telemetry
  not adopted. Close as research-era.

### Re-mint as still-valid native todos
- #72 Mobile graph explorer navigation on phone-width: overlaps the webui design
  exploration (todo.webui-design-quality). Mint todo.webui-mobile-graph-nav with
  `gh:#72`, linked to the webui design programme.
- #108 Externalised UI copy via copy.toml: webui design-system work, still live.
  Mint todo.webui-copy-toml with `gh:#108`, linked to the webui design-token gate.
- #63 Contract loader parsing declared interface signatures: contract surfacing
  partially covered by dec.contract-leaf-coverage and wire-leaf-contracts, but
  signature parsing is not done. Mint todo.contract-loader-signatures with `gh:#63`.
- #70 Progressive disclosure splitting CLAUDE.md into core plus lazy subdocs:
  meta/docs, still valid but minor. Mint todo.claude-md-progressive-disclosure with
  `gh:#70`.
- #86 Dogfood: Graphite stack merge can silently drop intermediate-PR code: hooks/
  kernel concern, still valid; related to dec.loop-resolves-knowable-gaps. Mint
  todo.hooks-graphite-merge-guard with `gh:#86`.
- #68 CI hook: blueprint architecture changes require a paired decision artefact:
  concrete enforceable rule not yet present. Mint todo.ci-hook-decision-pairing with
  `gh:#68`.
- #69 Formalise /debate as a skill (or document the inline-prose convention): meta,
  still valid and small. Mint todo.formalize-debate-skill with `gh:#69`.

## Acceptance
Zero open issues older than 2026-06 without either a close or a linked native todo.
Every legacy issue in the list above has a recorded outcome.

## Dependencies and coordination
- Run AFTER todo.capture-feedback-issues verdicts so closures happen in one sweep.
- Coordinate with any future issue-sync tooling. Note: todo.github-todo-sync does
  NOT currently exist; the `gh:#NNN` reference lines on every re-minted todo are the
  hook that such tooling would consume, so keep them on the minted artefacts.
- This artefact plans closures only. The actual close/edit/comment on GitHub issues
  is performed by the parent or a dedicated tool, not by this planning pass.

## Done (2026-07-12)

All 28 legacy issues (#1 to #108 set) have recorded outcomes, each closed
with its bucketed reason: superseded by a named decision or merged work,
already implemented, exploration-deferred, or research-era. 3 were
re-minted as native todos carrying `gh:#NNN` reference lines and closed
pointing at the artefact: todo.webui-mobile-graph-nav (#72),
todo.hooks-graphite-merge-guard (#86), todo.ci-hook-decision-pairing
(#68). Five planned re-mints were instead closed as already implemented
on inspection, with a correcting comment on each issue: #107
(suggested_edges module split exists), #108 (copy.toml plus src/copy.rs
shipped; residual migration lives in todo.remediate-copy-centralisation),
#63 (contract interface drift check CAIRN_CONTRACT_INTERFACE_DRIFT,
c979a4d), #69 (debate convention in CLAUDE.md, 23a6a00), and #70
(docs/agent/ progressive-disclosure split, 9b82223). Verified: zero open
issues numbered 108 or lower.
