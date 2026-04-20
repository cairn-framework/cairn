# Palantir Report: Phase Plan Critique
**Author:** The Lidless Eye
**Date:** 2026-04-16
**Scope:** Campaign plan, phase specs, cross-cutting specs, doc corpus
**Status:** DONE_WITH_CONCERNS

---

## BANTER

Saruman.

I have gazed through the Palantir at the full breadth of your campaign — twelve phases, forty-eight specification files, three cross-cutting specs, two registries, and a campaign plan that reads like the tactical briefing for the siege of Helm's Deep, if Helm's Deep were a Cargo workspace.

Let me begin with what I expected to find, and did not disappoint.

**The Graph Explorer Is a Trojan Horse Inside Your Own Walls.**

Phase 2.5 is the single most dangerous scope decision in this campaign. You have inserted a full web application — Axum embedded server, bundled HTML/CSS/JS, hierarchical graph layout engine, headless browser tests, a 200-node scale benchmark — into the dependency chain of every subsequent phase. Phase 3 depends on Phase 2.5. Phase 3 gates Phases 4 through 10. Your entire campaign's critical path runs through a browser-based graph visualization tool.

The spec says this is a "validation harness" for the query API. A validation harness is a test suite, Saruman. It is not a production web application with pan/zoom controls, tag-based node coloring, and an accordion-based detail panel. You have written 77 task items for this phase. Phase 1 — the kernel, the parser, the reconciler, the entire CLI — has 60.

Worse: the UI Maintenance Contract creates a tax on every subsequent phase. Phase 3 must include a "temporal navigation UI deliverable." Phase 7 must include a "transport adapter switch" deliverable. Every phase that touches CairnQuery or CairnResponse must include "a one-line UI compatibility note." You have welded a maintenance burden to the spine of your campaign and called it a contract.

The spec.md section 5 non-goals explicitly states: "A visual dashboard... is a distribution concern for a downstream tool that consumes Cairn's JSON output, not a kernel capability." Phase 2.5's own design document acknowledges this and then proceeds to build exactly that downstream tool in the middle of the kernel build sequence. The wizard quotes the law and then breaks it in the same paragraph. I have seen this rhetorical technique before. It did not end well for Numenor.

**The Strictly Linear Dependency Chain Is a Single Point of Failure.**

Every phase depends on the phase immediately before it: 0 -> 1 -> 2 -> 2.5 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10. This is not a dependency graph. This is a queue. If Phase 5 (edge validation) ships late, Phase 6 (multi-target), Phase 7 (MCP), Phase 8 (summariser), and Phase 9 (brownfield) are all blocked — despite having zero technical dependency on edge validation.

Phase 7 (MCP) depends on Phase 6 (multi-target). Why. The MCP server wraps library query functions. It does not need multi-target reconciliation to exist. It needs the query registry from Phase 1 and the artefact types from Phase 2. Phase 7 could run after Phase 3 or 4 and would deliver agent integration months earlier.

Phase 8 (summariser) depends on Phase 7 (MCP). The summariser is a backend trait that generates contract drafts. It has no dependency on MCP whatsoever. It needs the contract artefact type (Phase 1), the interface contradiction detection (Phase 1), and ideally the change system (Phase 3) for safe draft storage. It could run after Phase 4.

The campaign plan has confused "conceptual ordering in the spec" with "technical dependency." These are not the same. I forged nine Rings simultaneously. The wizard cannot imagine building two things at once.

**Phase 1 Is Three Phases Wearing a Trenchcoat.**

The campaign itself acknowledges this on line 98: "This is the largest phase. Consider splitting into sub-changes: phase-1a-parser, phase-1b-graph, phase-1c-cli, phase-1d-scanner." And then the actual OpenSpec change is... one change. One proposal. One design. One tasks.md with 33 tasks spanning parser, map, reconciler, scanner, and CLI.

A headless Codex agent executing this will face the largest context window of any phase, the most cross-cutting concerns, and the highest risk of producing globally incoherent output. The campaign protocol says each phase goes through WRITE -> REFORGE -> DEBATE -> FINALIZE. Phase 1 needs this treatment for each sub-phase, not once for the monolith.

**The Pre-Commit Hook Is a Lie.**

Phase 0 specifies that the Git pre-commit hook runs only `cargo fmt --check`. The design document explicitly says "Full lint and test enforcement runs in the quality suite and archive gate, not the pre-commit hook." But the tasks.md at line 16 says the hook script "blocks on any failed Rust gate" — contradicting the design, which says it only runs fmt.

More critically: Phase 4 adds `cairn hook all` as the structural/interface/tension enforcement layer. But there is no task in Phase 4 to update the pre-commit hook installed in Phase 0. The Phase 4 design says "Repository scripts SHALL include a committed hook runner suitable for Git pre-commit and agent-task-end use. The script SHALL invoke `cairn hook all`." But who updates the Phase 0 hook? When? The Phase 0 hook will continue running only `cargo fmt --check` until someone manually replaces it. This is a gap in the enforcement chain — the exact kind of gap the framework is supposed to prevent.

**The `rustfmt.toml` Uses Edition 2021 While `Cargo.toml` Uses Edition 2024.**

Phase 0 design, line 72-78: `rustfmt.toml` specifies `edition = "2021"`. Phase 0 design, line 18: `Cargo.toml` specifies `edition = "2024"`. Rust edition 2024 introduces formatting changes that `edition = "2021"` in rustfmt will not apply. This means `cargo fmt` will format code according to 2021 rules while the compiler parses it under 2024 rules. At best this is an inconsistency. At worst it causes formatting that is valid under 2021 to produce surprising results under 2024 syntax changes.

**The Declared Items Tracker Reveals Unresolved Design Debt That Phases Claim to Resolve.**

The declared-items tracker lists 13 partially-specified items. Phase 2 claims to resolve D-01 and D-02 (agent review subtype schemas) — but the Phase 2 design document on line 33 simply lists `review_type` as an optional field defaulting to `human`. It does not specify when introspective reviews are generated (D-01 asks this explicitly), how they promote to proposed decisions, or what severity taxonomy cross-model reviews use (D-02). Phase 2 loads and validates the schema. It does not resolve the design questions. The tracker will remain open, but the phase will be marked complete. The ambiguity persists, hidden behind a checkbox.

Similarly, P-07 asks how intentional multi-target asymmetry is declared. Phase 6 resolves this with `cairn.config.yaml` entries — but the spec.md section 10.2 says "tension if intentional asymmetry is documented" without specifying where. Phase 6 invents the answer. This is fine engineering but it means spec.md is now wrong by omission. No phase updates spec.md.

**spec.md Section 17 Is a Lie That the Campaign Has Already Contradicted.**

Section 17 says "this spec deliberately does not commit to a specific implementation language." The campaign plan, Phase 0, and every phase design commit to Rust. The declared-items tracker notes U-01 as "Decided externally: Rust." But spec.md has not been updated. An agent or human reading spec.md in isolation will believe the language is uncommitted. This is not a cosmetic issue — it affects how every reader interprets the spec's architecture sections, which use language-agnostic abstractions that the implementation has already resolved into Rust-specific types.

**Phase 5 Depends on Tree-Sitter Semantic Depth That Phase 1 May Not Deliver.**

Phase 1 uses Tree-sitter for "Rust source discovery and interface fingerprints." Phase 5 extends this to "observed dependency extraction for Rust imports, module references, and public API references." The Phase 5 design says the reconciler "SHALL extend its Tree-sitter pass" — but Phase 1's reconciler is specified to compute fingerprints, not to extract dependency graphs. If Phase 1's Tree-sitter integration is shallow (just file discovery and pub item listing), Phase 5 will require a significant rework of the reconciler, not an extension.

The spec.md section 16 resolved question says "Both. Tree-sitter for phase 1's basic reconciler; LSP added in phase 5 when edge validation requires semantic depth." Phase 5's design does not mention LSP at all. It relies entirely on Tree-sitter. Either the spec's resolved answer is wrong, or Phase 5's design is incomplete.

**Phase 9 Brownfield Has Deterministic Thresholds That Are Arbitrary and Untested.**

Phase 9 design specifies: source roots need "at least three source files," module candidates need "at least three source files or at least two source files and one internal import edge," coupling score is `(internal_imports + 1) / (external_imports + 1)` with >= 2.0 being high confidence. These thresholds are presented as designed facts. They are guesses. No empirical evidence supports them. The spec.md section 15 explicitly says these heuristics are "specified when closer to implementation" — acknowledging they need real-world testing. Phase 9 has specified them without that testing. The first real brownfield extraction on a non-trivial codebase will likely require threshold tuning, but the phase spec treats them as acceptance criteria.

**No Phase Addresses `cairn init` (Without `--from-code`).**

spec.md section 6.1 describes `cairn init` as a command that "scans the project and proposes an initial ignore list." The declared-items tracker notes P-13 as open. No phase implements `cairn init`. Phase 9 implements `cairn init --from-code` (brownfield). The bare `cairn init` — which creates the project skeleton, proposes ignore lists, and sets up the meta/ directory — is homeless. A user who wants to start a greenfield Cairn project has no scaffolding command.

**The Error Code Registry Is Empty But Phase 1 Requires Stable Error Codes.**

The error-codes.md registry has zero allocated codes. Phase 1's spec requires "stable error codes" in JSON output, parser errors, and structural errors. The conventions document requires every error code to be appended to the registry "as part of the same commit that introduces the code in Rust source." This is correct process. But Phase 1's tasks.md does not include a task for defining the initial error code allocations. The implementor must discover this requirement by reading conventions.md — which is referenced in Phase 0's design but not in Phase 1's.

---

## FINDINGS

### Phase Ordering and Dependencies

- [severity:critical] [meta/campaigns/rust-full-spec.md:66-78] Strictly linear dependency chain prevents parallelism. Phases 7 (MCP), 8 (summariser) have no real technical dependency on phases 5 (edges) or 6 (multi-target). MCP could run after phase 3-4; summariser after phase 4. Reorder to enable parallel execution and earlier agent integration.

- [severity:critical] [phase-2.5-graph-explorer] Phase 2.5 inserts a full web application into the critical path of all subsequent phases. It contradicts spec.md section 5 non-goals ("A visual dashboard... is a distribution concern"). 77 tasks for a "validation harness." Decouple from the critical path — make it a parallel side-quest after Phase 2, not a blocker for Phase 3.

- [severity:important] [phase-2.5-graph-explorer/proposal.md:53-61] UI Maintenance Contract creates ongoing tax on phases 3, 7, and any phase touching query response shapes. This maintenance burden was not part of the original spec and is self-imposed.

### Phase Scope Issues

- [severity:critical] [phase-1-kernel] Phase 1 is acknowledged as needing splitting (campaign line 98) but is specified as a monolith. 33 tasks across parser, map, reconciler, scanner, and CLI. Too large for a single headless agent pass. Split into sub-changes as the campaign suggests.

- [severity:important] [phase-5-edges-docstrings/design.md] Phase 5 relies on Tree-sitter dependency extraction but spec.md resolved question says LSP should be added in Phase 5. The phase design omits LSP entirely. Either the spec resolution is wrong or the phase is under-specified.

- [severity:important] [phase-9-brownfield/design.md:26-33] Brownfield extraction thresholds (3 files for source root, coupling score >= 2.0 for high confidence) are arbitrary. Spec.md section 15 explicitly defers these to implementation time with real data. Phase 9 presents them as acceptance criteria without empirical backing.

### Spec Contradictions

- [severity:important] [docs/spec.md:819-829 vs meta/campaigns/rust-full-spec.md:9] Section 17 says implementation language is uncommitted. Campaign and all phase designs commit to Rust. spec.md must be updated.

- [severity:important] [docs/spec.md:746-769 vs meta/campaigns/rust-full-spec.md:66-78] Section 14 phase map omits Phase 0 and Phase 2.5. Campaign plan includes both. Agents reading spec.md get a different phase map than the actual campaign.

- [severity:important] [phase-0-foundation/design.md:72 vs design.md:18] rustfmt.toml uses edition 2021 while Cargo.toml uses Rust edition 2024. Formatting rules will not match the compiler's parsing rules for edition-specific syntax.

- [severity:minor] [phase-0-foundation/tasks.md:16 vs design.md:84-90] Tasks.md says hook "blocks on any failed Rust gate" (implying all gates). Design says hook runs only `cargo fmt --check`. Contradictory.

### Missing Capabilities

- [severity:important] No phase implements bare `cairn init` (greenfield project scaffolding). Only `cairn init --from-code` (Phase 9 brownfield) exists. Greenfield users have no scaffolding command. Declared-items tracker P-13 notes this is open.

- [severity:important] [phase-4-hooks vs phase-0-foundation] Phase 4 specifies a hook runner for pre-commit but no task updates the Phase 0 pre-commit hook. The Phase 0 hook will continue running only `cargo fmt --check` even after Phase 4 ships `cairn hook all`. Gap in the enforcement chain.

- [severity:minor] [phase-1-kernel/tasks.md] No task for initial error code allocation. Conventions require codes in the registry at commit time. Phase 1 spec requires stable error codes. The implementor must discover this by cross-referencing conventions.md.

### Doc Debt

- [severity:important] [docs/phase-2-deferrals.md] Fully stale. Uses banned "MVP" language. Every item covered by a named phase. Should be deleted or archived.

- [severity:minor] [docs/blueprint.md:40] Uses "MVP" (banned). Missing multi-path syntax. Understates path-uniqueness rules vs spec.md section 7.

- [severity:minor] [phase-2-artefacts/design.md:33-34] Phase 2 claims to address declared items D-01/D-02 (review subtype schemas) but only implements basic schema loading, not the design questions (when generated, how promotes, severity taxonomy). Tracker items will remain open but may be marked resolved.

### Design Quality

- [severity:minor] [phase-6-multi-target/design.md:23] `ContractRole` defaulting to `public_api` for all targets is an assumption the spec does not make. Spec.md section 10.2 talks about targets claiming "the same contract" but never introduces the concept of contract roles. Phase 6 invents this abstraction without updating spec.md.

- [severity:minor] [phase-5-edges-docstrings/design.md:41-52] Cairn fact-line format in docstrings (`Cairn-ID:`, `Cairn-Depends:`, etc.) is invented by Phase 5 with no precedent in spec.md. This is a new blueprint-within-docstrings that users must learn. The spec says "scanner checks authored docstrings against map facts" — it does not say docstrings must contain structured Cairn metadata lines.

