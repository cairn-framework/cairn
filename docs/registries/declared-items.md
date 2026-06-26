# Declared Items Tracker

Items from cairn-spec-v0.6.md at Declared maturity level. Each phase MUST check this
tracker and either resolve items in its scope or explicitly note why they remain open.

## Format

| ID | Item | Source (v0.6 section) | Status | Resolving Phase | Notes |
|----|------|-----------------------|--------|-----------------|-------|

## Status values
- `open`: not yet resolved by any phase
- `partial`: partially addressed, details noted
- `resolved`: fully specified in a phase spec
- `deferred-v2`: explicitly punted to post-v1

---

## Explicitly Declared sections

These are items the spec itself marks as "Declared" maturity level: named and scoped
but not yet designed enough to implement against.

| ID | Item | Source (v0.6 section) | Status | Resolving Phase | Notes |
|----|------|-----------------------|--------|-----------------|-------|
| D-01 | Agent introspective review subtype schema | 8.4 (design note) | open | Phase 2 | When generated, how promotes to proposed decisions, interaction with change system. Spec says "refinements to schema will land once real usage data exists." |
| D-02 | Agent cross-model review subtype schema | 8.4 (design note) | open | Phase 2 | Severity taxonomy, aggregation of multiple reviews on same node, reviewer identifier format. |
| D-03 | Edge validation against reality-layer dependencies | 10 (scanner step 5) | resolved | Phase 5 | Tree-sitter-backed Rust observations compare `use` paths and `mod` declarations against declared edges. Divergence surfaces as rationale tension. |
| D-04 | Docstring drift detection | 10 (scanner step 6) | resolved | Phase 5 | Scanner checks exact Cairn fact lines in Rust module docstrings against map facts. Divergence surfaces as rationale tension. |
| D-05 | `cairn docstring <node>` command | 12 | resolved | Phase 5 | Emits graph-grounded templates for Rust, Python, TypeScript, and Go. Structural facts are generated; human or agent completes prose. |
| D-06 | `cairn init --from-code` (brownfield extraction) | 12, 15 | open | Phase 9 | Generates initial blueprint and contracts from existing codebase. Reconciler extracts candidates, summariser names/describes, human refines. |
| D-07 | `cairn refine` (brownfield delta) | 12, 15 | open | Phase 9 | Re-runs brownfield extraction proposing delta against existing blueprint rather than fresh draft. Merge semantics unspecified. |
| D-08 | Brownfield extraction approach (full section) | 15 | open | Phase 9 | Entire section at Declared level. Approach decided (LLM generates, human refines) but detailed schema and prompt strategy deferred. |

## Open questions (unresolved)

Items from section 16 that remain open design questions, no resolution yet committed.

| ID | Item | Source (v0.6 section) | Status | Resolving Phase | Notes |
|----|------|-----------------------|--------|-----------------|-------|
| Q-01 | Shared utilities pattern | 16.1 | open | Convention | Where "this is a module" vs "this is two helpers": convention, not grammar. Projects decide. May need guidance in docs. |
| Q-02 | Todo coverage strictness | 16.2 | open | Phase 2 or 4 | Default is loose (warnings, no enforcement). Config allows strict. When/whether to promote strict to default is open. |
| Q-03 | meta/ directory layout | 16.3 | resolved | mid-2026 | Closed by `dec.artefact-organization-and-provenance` (2026-06-26): flat artefact-type-first layout, non-recursive loader, slug namespacing. See docs/conventions.md section 10. |
| Q-04 | Detailed schemas for agent review subtypes | 16.4 | open | Phase 2 | Overlaps D-01/D-02. When introspective review is generated, how it promotes to decisions, severity taxonomy for cross-model, multi-review aggregation. |
| Q-05 | Product name | 16.5 | open | Pre-release | "Cairn" is a working placeholder. Decision needed before code ships. |

## Deferred to v2 or post-v1

Items explicitly described as v2, post-v1, or phase 10+ distribution concerns.

| ID | Item | Source (v0.6 section) | Status | Resolving Phase | Notes |
|----|------|-----------------------|--------|-----------------|-------|
| V2-01 | Decision-to-blueprint violation flagging | 3.2 | deferred-v2 | v2 | "Decisions can declare the blueprint nodes they apply to; the framework can then flag when a change to those nodes appears to violate the decision (v2 capability, deferred)." |
| V2-02 | LSP server | 14 (phase 10+) | deferred-v2 | Phase 10+ | Editor UX: autocomplete on IDs, hover for node metadata, jump-to-definition on edges. Distribution concern. |
| V2-03 | Claude Code plugin packaging | 14 (phase 10+) | deferred-v2 | Phase 10+ | Distribution packaging. |
| V2-04 | Non-code reconcilers | 14 (phase 10+) | deferred-v2 | Phase 10+ | Org structure, product BOMs, research programmes. The reconciler interface is designed in v1; additional reconcilers are post-v1. |
| V2-05 | Visual rendering / dashboard | 5 (non-goals) | deferred-v2 | Post-v1 | Explicitly a non-goal for v1. Downstream tool consuming Cairn JSON output. |

## Uncommitted implementation choices

Section 17 items: the spec deliberately does not commit to these. Each must be
decided at implementation time for its respective phase.

| ID | Item | Source (v0.6 section) | Status | Resolving Phase | Notes |
|----|------|-----------------------|--------|-----------------|-------|
| U-01 | Implementation language for the toolchain | 17 | open | Phase 0/1 | Decided externally: Rust (per project memory). Not in spec. |
| U-02 | Tree-sitter setup / LSP integration strategy | 17 | open | Phase 1 (TS), Phase 5 (LSP) | Tree-sitter for phase 1, LSP added phase 5. Strategy details unspecified. |
| U-03 | MCP framework or transport | 17 | open | Phase 7 | No commitment to specific MCP implementation. |
| U-04 | Summariser model or provider | 17 | open | Phase 8 | Config specifies backend (local Ollama/llama.cpp, API, or disabled). Specific model unspecified. |
| U-05 | Visual rendering format | 17 | open | Post-v1 | Downstream of non-goal V2-05. |
| U-06 | Test framework binding | 17 | open | TBD | No specific test framework integration committed. |
| U-07 | Source file format beyond markdown sidecars | 17 | open | TBD | Only markdown sidecars specified for metadata. |

## Partially specified items

Items that appear in designed sections but have specific sub-aspects called out as
incomplete or dependent on later phases.

| ID | Item | Source (v0.6 section) | Status | Resolving Phase | Notes |
|----|------|-----------------------|--------|-----------------|-------|
| P-01 | Actor node type | 7 | open | TBD | "Actor is optional in v1": declared in grammar but no schema, integrity rules, or reconciliation behaviour specified. |
| P-02 | Brownfield prompt template | 15 | open | Phase 9 | "The exact prompt template the summariser uses": explicitly deferred. |
| P-03 | Brownfield structural candidate heuristics | 15 | open | Phase 9 | "The heuristics the reconciler uses to detect 'module-like' structural candidates": explicitly deferred. |
| P-04 | Brownfield refine merge semantics | 15 | open | Phase 9 | "The merge semantics for `cairn refine`": explicitly deferred. |
| P-05 | Brownfield-init change directory frontmatter schema | 15 | open | Phase 9 | "The frontmatter schema for the brownfield-init change directory": explicitly deferred. |
| P-06 | Summariser drafting non-contract artefact types | 13 | open | Phase 8 | "The summariser can optionally draft other artefact types (e.g. research from a conversation transcript), but contracts are the primary use case." No schema or interface specified. |
| P-07 | Multi-target interface divergence: error vs tension | 10.2 | open | Phase 6 | "Structural error if targets claim to implement the same contract but diverge; tension if intentional asymmetry is documented." How to declare intentional asymmetry is unspecified. |
| P-08 | Cross-model review aggregation | 16.4 | open | Phase 2 | "How do multiple reviews on the same node aggregate": raised as open question, no design. |
| P-09 | Introspective review cluster detection | 8.4 | open | Phase 2 | "When the framework detects clusters of introspective reviews on the same node, that node's spec is a candidate for revision." Clustering heuristic unspecified. |
| P-10 | MCP query interface | 12 | open | Phase 7 | "Same underlying queries exposed via MCP (v2)": noted as future wrapping of CLI queries. No transport or protocol details. |
| P-11 | LSP query interface | 12 | open | Phase 10+ | "LSP (v3)": noted alongside MCP. No details. |
| P-12 | Concurrent change conflict detection | 14 (phase 4) | open | Phase 4 | "Detect conflicts between concurrent active changes at authoring time, not only at archive time." Mechanism unspecified. |
| P-13 | Init-time ignore list assistance | 6.1 | open | Phase 1 | `cairn init` scans and proposes initial ignore list. Heuristics described (detects package.json, monorepo dist/) but exact implementation unspecified. |
