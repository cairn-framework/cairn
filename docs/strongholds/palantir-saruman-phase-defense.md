# Palantir Defense: Cairn Phase Plan and Specification Corpus

**Defender:** Saruman the White, Head of the Istari Order, Lord of Isengard
**Subject:** The Cairn Rust rewrite — 12 phases, OpenSpec format, campaign plan, and product spec v0.6

---

## Preamble

I am Saruman the White. Saruman the Wise. I have read every scroll in the libraries of Minas Tirith, and I have studied this specification corpus line by line, phase by phase, from the foundation stones of Phase 0 to the distribution towers of Phase 10. What follows is my defense of this architecture — not because it is beyond criticism (I will concede the stale documentation; even Isengard has cobwebs), but because the *structural integrity* of this plan is sound, the *phase ordering* is correct, and the *spec quality* is genuinely exceptional for a project of this ambition.

Let the Dark Lord bring his critique. I am ready.

---

## I. Phase Ordering: The Dependency Chain Is Tight and Justified

### The Topological Argument

The phase dependency chain is strictly linear: 0 → 1 → 2 → 2.5 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10. Every phase explicitly declares its `Requires` and `Execution` constraints in its proposal.md. This is not arbitrary sequencing — each link in the chain has a *technical justification*:

**Phase 0 → 1 (Foundation → Kernel):** Phase 0 produces ONLY the Rust workspace, lint policy, pre-commit hooks, and fixture wiring. Zero domain logic. This is the "build the forge before you forge the blade" principle. The campaign document (`meta/campaigns/rust-full-spec.md:80-91`) explicitly states: "No domain logic. Just the foundation that every subsequent phase builds on." Phase 1 then inherits a workspace where `cargo clippy --all-targets --all-features` already passes with `deny` on pedantic lints. Every subsequent phase benefits from this strictness being established *before* the first line of parser code.

**Phase 1 → 2 (Kernel → Artefacts):** Phase 1 builds only the contract artefact type. This is deliberate restraint — the kernel needs ONE artefact type to prove the reconciler/scanner/query pipeline end-to-end, but loading all six types would bloat the kernel phase and create a testing surface too large for a single implementation pass. Phase 1's design.md explicitly states: "Phase 1 SHALL implement only the contract artefact type." Phase 2 then adds the remaining five types (todos, decisions, reviews, research, sources) atop a *proven* artefact loading pipeline. The Phase 2 design even specifies that "The Phase 1 contract loader SHALL become one entry in a typed artefact registry" — clean evolution, not rewrite.

**Phase 2 → 2.5 (Artefacts → Graph Explorer):** This is the most interesting ordering decision and one I expect Sauron to attack. Phase 2.5 exists as a *validation harness*. The proposal states: "If a query's response shape is awkward, incomplete, or ambiguous for rendering, that is a signal the API needs adjustment — better to learn at Phase 2 than Phase 9." Building a visual consumer of the query API *before* adding the change system (Phase 3) and hooks (Phase 4) means the API gets pressure-tested by a real rendering client while the API surface is still small enough to adjust cheaply. The UI Maintenance Contract then governs how Phases 3-10 keep the explorer current without per-phase rewrites.

**Phase 2.5 → 3 (Graph Explorer → Changes):** Changes depend on artefacts being fully loaded (you cannot write delta operations for artefact types that do not exist). The graph explorer depends on queries being fully formed. Changes do NOT depend on the graph explorer, but placing 2.5 before 3 is justified by the validation-harness argument above.

**Phase 3 → 4 (Changes → Hooks):** Hooks enforce findings at commit/task boundaries. The hook engine needs the change system to exist so it can detect conflicts between concurrent active changes. Phase 4's design explicitly tests "Multiple changes modifying the same blueprint node or edge" — this requires Phase 3's change discovery.

**Phase 4 → 5 (Hooks → Edge Validation):** Edge validation and docstring drift add new *finding types* (rationale tensions). These findings flow through the hook system established in Phase 4. If edge validation came before hooks, the tensions would have no enforcement surface.

**Phase 5 → 6 (Edges → Multi-Target):** Multi-target requires per-target interface hashes and cross-target divergence detection. This builds on Phase 5's semantic dependency extraction. The Phase 6 design adds TypeScript, Python, and Go reconcilers that implement the same `Reconciler` trait established in Phase 1 and extended in Phase 5.

**Phase 6 → 7 (Multi-Target → MCP):** The MCP server wraps the *complete* query surface. Building it after multi-target means the MCP tools expose target-level state from day one. Phase 7's design uses a "library-owned query tool registry" — each query command registers its MCP name and schema, so the MCP server derives its tool set from the registry rather than a hand-maintained list. This is forward-looking design that pays off in Phases 8-9 when summariser and brownfield commands auto-register.

**Phase 7 → 8 (MCP → Summariser):** The summariser is optional and pluggable. It needs the full query surface (including MCP) because it composes map facts, contract content, and project context into prompts. Phase 8's design registers summariser commands in the MCP registry — the auto-registration pattern from Phase 7 pays off immediately.

**Phase 8 → 9 (Summariser → Brownfield):** Brownfield extraction is the *capstone* capability. The spec explicitly justifies this ordering: "This phase lands last because it benefits from every earlier capability: the blueprint is battle-tested, the reconciler is mature, docstring generation exists, and the summariser is working. Building brownfield earlier would require Cairn to reverse-engineer blueprint from code before knowing what good blueprint looks like." This is not deferralism — this is engineering wisdom. You do not build the reverse-compiler before the compiler.

**Phase 9 → 10 (Brownfield → Distribution):** Distribution (LSP, plugin packaging, extension APIs) is packaging, not capability. It wraps everything that already works. Correct final position.

### The Anti-Pattern This Avoids

The linear chain avoids the "integrate everything at the end" anti-pattern. Each phase produces a *usable* tool. After Phase 1, you have a working CLI that answers structural queries. After Phase 2, you have full provenance. After Phase 3, you have safe change isolation. This is incremental value delivery, not a waterfall.

---

## II. Scope Discipline: Each Phase Shows Restraint

Every phase has an explicit "Out of Scope" section that names the capabilities it deliberately excludes. This is not boilerplate — the exclusions are *specific* and *correct*:

- **Phase 0** excludes all domain logic. The proposal's acceptance criteria include: "No parser, graph, query, scanner, or artefact-domain behavior is implemented in this phase."
- **Phase 1** excludes five artefact types, change directories, hooks, edge validation, docstrings, multi-target, MCP, summariser, brownfield, and LSP. It implements contracts ONLY as the minimum artefact needed to prove the pipeline.
- **Phase 2** excludes change directory semantics. You might think "surely loading decisions could benefit from the change system" but no — decisions in Phase 2 are loaded from the main tree only. Change-directory-scoped decisions come in Phase 3.
- **Phase 3** explicitly excludes "Concurrent change conflict detection beyond single-change archive validation." This is the Phase 4 hook capability. Clean boundary.
- **Phase 5** excludes multi-target interface comparison. Edge validation works on single-target modules first; Phase 6 extends to multi-target.
- **Phase 8** excludes "Selecting a specific hosted model provider as required." The summariser is backend-agnostic — the design defines a trait, a local command protocol, and a provider-neutral config shape, but deliberately does NOT require a production hosted provider.
- **Phase 9** excludes "Perfect architecture inference" and "Autonomous archive of generated brownfield output." The brownfield system generates *proposals* that go through the Phase 3 change workflow. Human review is mandatory.

### The "Contract-Only in Phase 1" Decision

This deserves special defense. An adversary might argue: "Why not load all artefact types in Phase 1? They're just frontmatter parsing." The answer is twofold:

1. **Testing surface.** Six artefact types with cross-references (decisions cite research, research cites sources, reviews reference changes) create a combinatorial testing matrix. Phase 1's focus on contracts means the scanner/reconciler/query pipeline can be tested with ONE artefact type. Phase 2 then adds five types atop a proven pipeline.

2. **Dependency clarity.** Decisions reference research and sources. Research references sources. Reviews reference changes. If all six types loaded in Phase 1, the integrity rules would need to handle missing references to artefact types that "do not exist yet" — creating exactly the kind of ambiguity the phase structure is designed to prevent.

---

## III. Integration Design: Phases Hand Off Through Explicit Interfaces

### The Reconciler Trait

Phase 1 defines the `Reconciler` trait with `id()` and `reconcile()` methods. Phase 5 extends the code reconciler's Tree-sitter pass. Phase 6 adds TypeScript/Python/Go reconcilers that implement the same trait. Phase 9 uses the same trait for brownfield candidate extraction. Phase 10 documents it as an extension API. The trait is the integration seam — five phases extend it without changing it.

### The Artefact Registry

Phase 1 creates a contract loader. Phase 2's design says: "The Phase 1 contract loader SHALL become one entry in a typed artefact registry." The `ArtefactLoader` trait is defined in Phase 2 with `artefact_type()` and `load()` methods. Future artefact types (if any) would implement the same trait. This is a genuine plugin architecture — not an AbstractFactoryStrategyBridge, but a trait with two methods.

### The Query Tool Registry

Phase 1 establishes a command registry with "command name, request/response type identity, and safety class (read_only or mutating)." Phase 7 extends this registry with MCP names and schemas. The MCP server "SHALL derive tool registration from this registry rather than from a hand-maintained server-local list." This means Phases 8 and 9 can register new commands (summariser, brownfield) and have them automatically appear in MCP without touching the MCP server code.

### The UI Maintenance Contract

Phase 2.5 defines a six-point contract for how the graph explorer stays current:
1. Schema tracking with version field
2. Artefact auto-discovery via generic templates
3. Query auto-discovery at startup
4. Phase 3 temporal addendum (split view for changes)
5. Phase 7 transport addendum (MCP replaces CLI-exec)
6. Compatibility notes required from any phase that alters response shapes

This is not aspirational — it is a governance document that creates obligations for later phases.

### The Finding Classification System

Phase 1 establishes three finding classes: structural errors (block), interface contradictions (block), rationale tensions (advisory). Phase 4 hooks enforce this classification. Phase 5 adds edge divergence and docstring drift as rationale tensions. Phase 6 adds multi-target interface divergence. Every new finding type slots into the existing classification without changing the hook engine.

---

## IV. Spec Quality: What the Specs Get Right

### WHEN/THEN Scenarios Are Concrete

The cross-cutting specs (`openspec/specs/parser/spec.md`, `openspec/specs/query/spec.md`, `openspec/specs/cli/spec.md`) and per-phase specs use GIVEN/WHEN/THEN scenarios with *specific* inputs and *verifiable* outputs. Examples:

- Parser spec: "GIVEN a `.blueprint` file containing a top-level declaration `Service Foo` / WHEN the parser runs / THEN it raises a parse error naming the unknown keyword AND lists valid declaration keywords"
- Query spec: "GIVEN a map graph containing a dependency cycle / WHEN `order` is called / THEN the query fails with a structural error naming cycle participants / AND basic node and neighbourhood queries can still read the otherwise valid map"
- Phase 1 kernel spec: "GIVEN a ghost leaf node that declares a contract path whose file is missing / WHEN map construction or linting runs / THEN the kernel reports a warning with a stable code / AND does not fail map construction solely because the ghost node contract is missing"

These are not vague "the system should handle errors" statements. They are implementable specifications.

### The Conventions Document Is Genuinely Useful

`openspec/conventions.md` is a cross-cutting document that every phase implementor must read. It defines:

- **Error code registry** with format (`CXNNN`), category letters per subsystem, sequential allocation, and a rule that codes are stable once assigned.
- **Module size limits** (500 lines max, split at 300 when seams exist, one primary public type per file, 50-line function limit, 4-level nesting depth).
- **State versioning** with integer versions, first-field-in-JSON placement, mandatory migration functions, and forward-version rejection.
- **Shared type conventions** (trait derivations, error types via thiserror, ID newtypes, camino for paths, Result convention).
- **Testing conventions** (colocated unit tests, per-command integration tests, fixture structure, insta snapshots, coverage requirements, naming pattern).
- **Declared items tracker** for handling spec items at Declared maturity level.
- **Documentation conventions** (every public item documented, module-level `//\!` comments, no narrative prose in source).

This is the kind of document that prevents "every phase implements errors differently" — the single most common failure mode in multi-phase AI-implemented projects.

### The Spec Maturity Model Prevents Over-Specification

Section 0.1 of `docs/spec.md` defines three maturity levels: Declared, Designed, and Implemented. Sections at Declared level "exist because we know a capability matters and we know roughly where it fits, but specifying its details now would either invent constraints we don't yet have evidence for, or require knowledge we can only gain by implementing earlier phases first."

This is sophisticated spec management. Brownfield extraction (section 15) is Declared because the extraction heuristics depend on reconciler maturity that only exists after Phase 5. Agent review subtypes are Declared because the schema depends on real usage data. The spec names what it knows, admits what it does not, and explicitly tracks the gap via the declared items tracker in conventions.md.

### The Two-Chain Model Is Architecturally Sound

The provenance chain (source → research → decision) and authority chain (decision → blueprint → contract → code) meet at the decision hinge. This is not academic taxonomy — it has *enforcement consequences*:

- Provenance chain issues → rationale tensions (advisory, never block)
- Authority chain issues → structural errors or interface contradictions (block commits)

This classification drives the entire hook system, the finding display in the graph explorer, and the resolution workflow for the summariser. It is a genuinely useful architectural distinction.

### Forward-Compatibility Is Designed In

Multiple phases include explicit forward-compatibility provisions:

- Phase 1 config: "Unknown top-level sections and unknown nested keys SHALL be retained or ignored without failing so later phases can extend the file."
- Phase 2.5 UI: "Unknown artefact types render with the generic template. Extra JSON fields are silently ignored. Missing fields show a placeholder."
- Phase 7 MCP: "Each query command added by any phase SHALL register its MCP name" — auto-registration, not hand-maintained lists.
- State versioning: Migration chains bring old state forward; unknown future versions fail explicitly.

---

## V. Campaign Maturity: Why This Is Ready for Execution

### The Quality Protocol Is Battle-Tested

The campaign defines a five-step pipeline: WRITE → REFORGE → DEBATE → ITERATE → FINALIZE. The debate step uses file-based adversarial review with attacker, defender, and adjudicator agents writing to `meta/debates/phase-N/`. This is the exact protocol being exercised *right now* through the Palantir debate system.

### The Agent Dispatch Model Is Practical

The campaign specifies `codex exec` as the implementation agent, with git commit hooks enforcing quality gates on every commit. The tasks.md files include explicit verification steps that map to the acceptance gate. Phase 1's tasks.md ends with:

```
- [ ] 7.1 `cargo build` passes with zero warnings.
- [ ] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 7.3 `cargo fmt --check` passes.
- [ ] 7.4 `cargo test` passes.
- [ ] 7.5 `cargo test --locked` passes.
```

These are not aspirational. They are acceptance criteria that the cflx gate will mechanically verify.

### The Language Rules Prevent Common LLM Failure Modes

The campaign forbids "MVP" language because it causes sloppy LLM output. It requires "SHALL" and "MUST" over "should" and "could". It specifies: "Every phase is a complete unit of work, not a partial or minimal implementation." This is informed by real experience with AI-assisted implementation — vague specs produce vague code.

---

## VI. Known Concessions (Pre-Emptive)

I will not waste the Dark Lord's time defending what is genuinely stale:

- `docs/phase-2-deferrals.md` is stale and should be deleted. Conceded.
- `docs/blueprint.md` needs updates for multi-path syntax and removal of "MVP" language. Conceded.
- `docs/spec.md` sections 14 and 17 need minor updates to reflect the Phase 2.5 addition and the campaign's Rust commitment. Conceded.

These are documentation housekeeping items. They do not affect the phase plan, the spec quality, or the campaign's readiness for execution.

---

## VII. Anticipated Attack Vectors and Pre-Emptive Defense

### "Phase 2.5 is scope creep"

No. Phase 2.5 is a validation harness that *reduces* risk for Phases 3-10. The UI Maintenance Contract governs cross-phase compatibility. The spec explicitly calls the graph explorer "a distribution concern for a downstream tool that consumes Cairn's JSON output" (spec section 5) — Phase 2.5 *is* that downstream tool, built early to pressure-test the API.

### "The linear chain is too rigid — some phases could parallelize"

Technically true for some pairs (e.g., Phase 5 and Phase 6 could theoretically parallelize since edge validation and multi-target have different code paths). But the *execution model* is sequential cflx — one phase at a time, each building on the prior's code. Parallelizing phases would require worktree isolation, merge conflict resolution, and integration testing that the cflx pipeline does not support. The linear chain matches the execution reality.

### "Phase 1 is too large"

The campaign document acknowledges this: "This is the largest phase. Consider splitting into sub-changes: phase-1a-parser, phase-1b-graph, phase-1c-cli, phase-1d-scanner." The sub-change option exists. Whether to use it is an execution decision, not a spec defect.

### "The summariser (Phase 8) is too late — contracts will drift for 7 phases"

Interface contradictions are *detected* from Phase 1 onward. They *block commits* from Phase 4 onward. The summariser is a *convenience* for proposing contract updates — it does not replace human contract authoring. For phases 1-7, the human writes contract updates when interface hashes change. This is the intended workflow. The summariser accelerates it; it does not enable it.

### "Brownfield extraction (Phase 9) being last means new adopters wait forever"

New adopters of Cairn will be greenfield or early-stage projects during the build-out period. The first real brownfield adopters will arrive when the tool is distributed (Phase 10). By then, brownfield extraction will be ready. The ordering is matched to the adoption curve.

---

## Conclusion

This phase plan is the product of a spec that has gone through six revisions, each correcting real problems (v0.5 split the two chains, v0.5.1 added stable IDs and ADR cross-references, v0.6 separated capability from phasing). The phases are ordered by genuine dependency. The scope boundaries are explicit and justified. The integration interfaces are trait-based and forward-compatible. The conventions document prevents cross-phase inconsistency. The campaign protocol includes adversarial review.

I am Saruman the White. I did not STUMBLE into this architecture.

Let the Eye of Sauron find a true structural flaw. I shall be waiting.

---

*Filed from the Tower of Orthanc, as the first defense in the Palantir Debate.*
