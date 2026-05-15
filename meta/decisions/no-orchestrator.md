---
type: decision
node: cairn.root
status: accepted
date: 2026-05-13
revisit_triggers:
  - "Gas City ceases active development or changes its extension model in a way that breaks pack-based integration"
  - "A CAIRN-specific scheduling primitive emerges that cannot be expressed as a Gas City formula"
  - "Community demand for a zero-dependency CAIRN orchestrator exceeds adapter complexity"
informed_by:
  - meta/research/gas-city-cairn-integration/analysis.md
---

# No Orchestrator: CAIRN does not ship its own orchestrator

## Context

Cairness was scoped as a lightweight orchestrator on top of CAIRN: graph-walking wave scheduling, YAML DAG flow engine, adapter registry, metrics dashboard, and self-improvement loop. The original estimate was 2,000 to 3,000 lines of Rust for a standalone harness.

Gas City (Steve Yegge's orchestrator, `gastownhall/gascity`) has since matured to production grade. Its controller provides declarative `city.toml` configuration, fsnotify-driven hot reload, pool evaluation in parallel, crash quarantine with `max_restarts`/`restart_window`, graceful two-pass shutdown, single-controller `flock` on `.gc/controller.lock`, and Unix-socket IPC. The extension model (packs, formulas, prompt templates, runtime providers) is designed for external contributors: `gastownhall/gascity-packs` exists as the community pack home.

Beads (`gastownhall/beads`) provides distributed graph storage with hash-based merge-safe IDs, Dolt versioning, and federation via Wasteland. It is independently installable (`brew install beads` / `npm install -g @beads/bd`), orchestrator-independent.

The cairness issue inventory (#1, #2, #6, #7, #9, #10, #14) was evaluated against Gas City's actual codebase. Overlap analysis follows in the Rationale section.

## Decision

CAIRN does not own an orchestrator. Four consequences follow:

1. **Integration via contract.** A documented integration contract (GH #96) defines the stable CLI surface, JSON schema per command, exit-code taxonomy, event envelope, and subscription primitive that any orchestrator needs to drive CAIRN.

2. **Reference adapters under `adapters/`.** The first adapter (`adapters/gascity/`) packages CAIRN as a Gas City pack: formula definitions, prompt templates, and a thin shim that shells to `cairn`. Future runners get their own adapter directory.

3. **Cairness as scoped is retired.** The standalone cairness project is superseded. The graph-walking scheduler (~400 LOC), the one novel piece with no Gas City analogue, survives as a Gas City formula in `adapters/gascity/`.

4. **cflx is retired.** CAIRN's own `accept`/`archive` primitives run under any external orchestrator (or none). The cflx workflow runner is no longer maintained.

## Rationale

Building a CAIRN-owned orchestrator would duplicate approximately 70% of Gas City's mature surface while losing community, audit, and federation benefits. The overlap matrix from the analysis:

| Cairness scope | Gas City equivalent | Verdict |
|---|---|---|
| #1 Epic: lightweight harness (2-3k LOC) | Full control plane in Go | Standalone form duplicative |
| #2 Flow engine + YAML DAG (500-700 LOC) | Formulas + molecules (TOML + bead trees) | Duplicated |
| #6 Adapter registry (200+150/adapter) | Runtime providers + prompt templates | Mostly duplicated |
| #7 Wave scheduler walking CAIRN graph (400-500 LOC) | Controller is config-driven, not graph-driven | **Not duplicated** (novel) |
| #9 Stats + dashboard + self-improvement (1,150 LOC) | Event bus + Dolt audit | Data layer duplicated; self-improvement loop novel |
| #14 SQLite cache + DB state | Dolt via Beads | Dolt strictly better for versioning/branching/federation |

The unique CAIRN value (typed artefacts, two-chain topology, drift gate, blueprint reconciliation) has zero analogue in Gas City. Grep of `gascity/engdocs/` for ontology, blueprint, authority, and provenance returns only stray hits, never as an architectural concept. Gas City explicitly excludes declarative schema specifications from its scope (`gascity/specs/architecture.md` section 7).

The structural argument resolves cleanly into three non-competing layers:

- **Layer 3 (Orchestration, optional):** Gas City controller, sessions, packs, formulas. CAIRN consumed as formula steps.
- **Layer 2 (Semantic, CAIRN's lane):** blueprint, typed artefacts, two-chain topology, reconciler, drift gate, interface hashes. No equivalent in Gas City.
- **Layer 1 (Storage, pluggable):** Default filesystem. Optional Beads (Dolt-backed). CAIRN trait: `ArtefactStore`.

These compose. They do not compete.

Yegge's own framing confirms the complementary positioning. Gas City's "knowledge graph" is work-as-graph (beads with dependencies), not architecture-as-graph. Its reliability story is probabilistic (more agents reviewing each other), not deterministic (gate at commit). Its pitch is "replace SaaS / business process automation," not architectural governance.

## Consequences

1. **The graph-walking scheduler from cairness (#7) survives as a Gas City formula in `adapters/gascity/`.** The CAIRN side (~50-100 LOC) is a query primitive: `cairn query --ready --change <id> --json` walks blueprint plus active change, applies `needs:` edge resolution, groups results by topological depth, emits waves as JSON. The orchestrator side (~300-400 LOC) is operational dispatch that Gas City already provides via formula `needs:` edges, runtime pools, and label-based routing.

2. **CAIRN stays focused on architecture-truth.** The framework's enforcement value comes from the semantic layer: typed artefacts encoding obligations, the two-chain topology splitting evidence from norms, and the deterministic drift gate at commit. None of this requires owning a process supervisor.

3. **Spec section 4 requires amendment.** The current text states CAIRN "deliberately does not adopt OpenSpec's workflow layer." With cflx retired and workflow externalised, this needs updating to reflect that workflow lives in external skills and optional formulas, not as a CAIRN non-goal to be reconsidered.

4. **Three operational paths for graph-state-driven work emerge.** (a) CAIRN queries drive Gas City formulas. (b) Beads-mediated: typed beads become work items via `bd ready`. (c) SSE reactive: CAIRN emits events on graph state changes, Gas City Orders react. All three are adapter concerns, not kernel concerns.

5. **Contribution path is tractable.** The upstream target is `gastownhall/gascity-packs` as `packs/cairn-governance/`. This is TOML plus Markdown plus prompt templates plus a thin shim. No contribution into 250k lines of Go required.
