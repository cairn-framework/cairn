# Phases 6–9 Scope Note
**Generated:** 2026-04-20  
**Source:** proposal.md + tasks.md for each phase (design.md and specs/ excluded)

---

## Phase 6 — Multi-Target and Languages

**Goal.** Extend reconciliation so a module with multiple paths (e.g. `["./core-rust", "./core-ts"]`) reconciles every target independently, stores per-target interface hashes, and dispatches through a shared `Reconciler` trait for Rust, TypeScript, Python, and Go.

**Surface area.**
- `src/reconciler/` — new TS/Python/Go reconcilers; shared trait dispatch
- `.cairn/state/interface-hashes.json` — schema change to per-node+target keys; migration required
- CLI: `get`, `files`, `scan`, `lint` — all gain target-level output
- JSON output schemas — target arrays and per-target findings

**Cross-phase dependencies.**
- Requires phase-5-edges-docstrings (declared). No explicit dependency on phase-4-hooks conflict.rs, but the single-path hash migration (task 3.2) touches state that phase-4 hooks read — if hooks run against stale state mid-migration, false positives are possible. Sequence matters.

**Human-judgement tasks.**
- Task 2.6: "canonical public interface extraction and sorting rules" for four languages — what counts as public differs by language idiom; codex will need explicit rules or a spec callout.
- Task 3.4: intentional asymmetry marker syntax — naming and grammar need a decision before implementation.

**Extra verification notes.**
- State migration (3.2) needs a before/after fixture test — not covered by the standard quintet.
- Task 3.5 explicitly requires hash-stability tests (private symbols, formatting, source order); these must be present even if cargo test passes.

---

## Phase 7 — MCP

**Goal.** Ship a `cairn-mcp` binary (new workspace member) that wraps the existing query layer over stdio transport, exposing read-only and gated mutating tools, and prepends project context and artefact rules to every response.

**Surface area.**
- New binary crate `cairn-mcp/` in workspace
- `src/` library boundary — CLI query structs extracted into shared lib modules (task 1.1)
- `cairn.config.yaml` — new `context` and `rules` blocks parsed (task 3.1)
- No CLI command surface changes (wrapper only)

**Cross-phase dependencies.**
- Requires phase-6. No phase-4 hooks dependency explicitly stated. However, the "gated mutating tools" (task 2.5: archive, rename) depend on the archive-bypass regression work from phase-4 being stable — if conflict.rs or the hook gate is flaky, mutating MCP tools are unsafe to expose.

**Human-judgement tasks.**
- Task 2.3: MCP tool naming and safety class taxonomy — tool names are agent-visible API surface; once shipped they are hard to rename.
- Task 2.5: deciding which mutation tools to expose and under what gate condition requires a deliberate UX policy call.

**Extra verification notes.**
- Task 4.3 requires integration tests for stdio request/response cycles — these are outside the standard cargo test unit scope and must be verified explicitly.

---

## Phase 8 — Summariser

**Goal.** Add a pluggable, opt-in summariser subsystem that drafts contract and docstring updates from map facts and interface changes. Humans resolve every draft via accept/edit/discard; nothing is applied automatically.

**Surface area.**
- `src/summariser/` — new module: backend trait, local-command backend, hosted-API adapter boundary, draft storage
- `.cairn/state/summariser/` — new state directory
- CLI: `summarise`, `drafts`, `draft show`, `draft accept`, `draft accept --edited`, `draft edit`, `draft discard`
- MCP tool registry — summariser and draft commands registered (task 4.3)

**Cross-phase dependencies.**
- Requires phase-7 (MCP tool registry must exist for task 4.3). No phase-4 hooks dependency stated, but `draft accept` does atomic contract replacement — it touches the same artefact write path that phase-4 archive-bypass regression guards. That gate must be solid first.

**Human-judgement tasks.**
- Task 1.5: hosted API adapter boundary — which provider(s) to support, authentication surface, and whether to ship a no-op stub or a real adapter is explicitly deferred but needs a decision before phase-10.
- Task 2.1: "bounded code samples" byte limits — 4,000 bytes per file (phase-9 carries the same constant); these numbers need rationale or they will be revised arbitrarily later.

**Extra verification notes.**
- Fake-backend determinism tests (task 1.6) are required; without them, CI is non-deterministic for any hosted adapter.
- Atomic replacement + rollback (task 3.2, 3.4) needs failure-path tests beyond what cargo test typically exercises.

---

## Phase 9 — Brownfield Extraction

**Goal.** Give existing codebases a path into Cairn without manual bootstrapping. `cairn init --from-code` generates a draft blueprint and stub contracts in a change directory for human review. `cairn refine` produces a delta against an existing blueprint when code has drifted.

**Surface area.**
- New CLI commands: `cairn init --from-code`, `cairn refine`
- `src/extractor/` (or similar) — repo-wide discovery, clustering, candidate generation
- `meta/changes/brownfield-init/` — generated output directory in target repos
- Summariser integration (phase-8 backend reused for naming)
- MCP tool registry — `init --from-code` and `refine` registered as mutating tools (task 5.3)

**Cross-phase dependencies.**
- Requires phase-8 (summariser backend reused in task 2.1–2.3). Indirectly requires phases 3–7 in full. Phase-4 hooks archive-bypass is load-bearing: generated brownfield output goes through the change archive workflow (proposal references "Phase 3 change archive workflow"); if archive-bypass regression from phase-4 is incomplete, the entire human-review safety guarantee is broken.

**Human-judgement tasks.**
- Task 1.2: clustering heuristics (path ownership, dependency density) — thresholds are judgment calls that affect false-positive rate; needs a named policy or tunable config.
- Task 1.4: "confidence score bands" and "observed edge thresholds" — these constants are UX-visible and set user expectations; need explicit rationale.
- Task 4.3: rename detection from "path and similarity evidence" — similarity metric and threshold require a design decision.

**Extra verification notes.**
- Task 1.5 requires fixture tests for five repo archetypes (simple, nested, mixed-language, low-confidence, high-coupling) — these are integration-level and must be present beyond the standard quintet.
- `--force` flag on init (task 3.5) needs an explicit test that it does not clobber main blueprint artefacts.

---

## Cross-Phase Risk Summary

| Risk | Phases affected |
|---|---|
| State migration in phase-6 (single→multi-target hashes) breaks phase-4 hook reads if not sequenced carefully | 4, 6 |
| Phase-4 archive-bypass regression (conflict.rs) is a silent load-bearing assumption in phases 7, 8, 9 — none of them declare it as a dependency | 4, 7, 8, 9 |
| MCP tool names (phase-7 task 2.3) are agent-visible API; renaming after phase-9 registers more mutating tools is disruptive | 7, 8, 9 |
| Hosted API provider decision deferred in phase-8 but needed before phase-9 summariser integration ships | 8, 9 |
| Brownfield clustering thresholds (phase-9 tasks 1.2, 1.4) need explicit rationale — currently underspecified | 9 |
