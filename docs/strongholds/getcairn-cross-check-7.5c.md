# Cross-check: phase-7.5c (verification states)

## Scope

Cross-check the proposed `phase-7.5c-verification-states` scope (5-state enum on verification artefacts; `#[cflx_planned(phase = N)]` proc-macro to replace `#[ignore = "awaits phase-N"]`; conventions §5 + AGENTS.md updates; Blocked-vs-Failed semantics) against current openspec content, the active phase-{8.0,9.0,10.0}-tests proposals, the Cargo workspace shape, the Makefile's `status-phases` target, and the artefact spec. Resolve the phase-8.0-tests collision question. Resolve the Blocked-vs-Failed error-code question as far as possible.

## Inputs read

- `docs/strongholds/getcairn-refined-batch-D.md` (full): the C11 refinement that proposed phase-7.5c.
- `docs/strongholds/getcairn-roadmap-debate.md` (greppped 7.5c sections, lines 7, 51-55, 125-129, 215, 242-252, 301, 372, 418-423, 515-519, 576-580, 622): roadmap-level placement and one open question N3 (sequencing vs Bundle A).
- `openspec/specs/artefacts/spec.md` (full): six v1 artefact types: contract, todo, decision, review, research, source. **No `verification` artefact type defined.** (Validation status note from the prompt: spec is currently failing `validate` per image #65; recorded but not blocking this analysis.)
- `openspec/specs/cli/spec.md` (full): three high-level requirements: kernel-query exposure, stable human + JSON output, library-backed services. No verification command surface yet.
- `openspec/specs/testing-baseline/spec.md` (full): the canonical home of the `#[ignore = "awaits phase-<N>"]` convention and the `phase-<N>.0-tests` pre-phase pattern. Five requirements; the test-first pre-phase is requirement 4.
- `openspec/conventions.md` (full): section 5 is "Testing Conventions"; the `#[ignore]` pattern lives in the "Test-First Pre-Phase" subsection (lines 213-215).
- `AGENTS.md` (full, 46 lines): line 25 is the load-bearing directive ("remove the matching `#[ignore = "awaits phase-<N>"]` markers as the feature lands").
- `openspec/changes/phase-8.0-tests/{proposal,design,tasks}.md`: scope is 12 ignored summariser tests; pure test-authoring pre-phase.
- `openspec/changes/phase-8-summariser/proposal.md` and `phase-9-brownfield/proposal.md`: feature-phase headers; both will leverage `#[ignore]` removal as their first task per group.
- `openspec/changes/phase-{9.0,10.0}-tests/`: same shape as 8.0-tests; both directly cite `#[ignore = "awaits phase-N"]` in their tasks.
- `Cargo.toml` workspace block: `members = ["."]`: single-crate workspace, no proc-macro precedent.
- `Makefile` `status-phases` target: parses `tasks.md` checkboxes (`grep -c '^[[:space:]]*[-*][[:space:]]*\['`), **not** `#[ignore]` attributes.
- `openspec/registries/error-codes.md`: 11 category letters (P/K/A/C/H/E/T/M/S/B/D), no V or L. Most categories are empty or sparse; CE, CT have a handful of allocations.
- Live `#[ignore]` grep across `/src/` and `/tests/`: **zero matches.** The pattern exists only in the conventions text, the testing-baseline spec, the archived phase-7.5a proposal, and the *unapplied* phase-{8.0,9.0,10.0}-tests proposals.

## Findings

### F1: No verification artefact type currently exists

The artefacts spec (`openspec/specs/artefacts/spec.md`) defines six v1 types: contract, todo, decision, review, research, source. There is no `verification` artefact. The C11 refinement and the roadmap-debate both speak of "verification artefacts" carrying a state enum, but cairn today has no such kernel object. The closest analogues are: (a) Rust tests as files-on-disk skipped by `#[ignore]`, (b) the `phase-N.0-tests` *directory* convention as a phase-level "planned" marker, (c) `tasks.md` checkboxes as per-task progress, (d) the verification battery (`cargo build`/`clippy`/`fmt`/`test` plus `cflx.py validate`) as a one-shot pass/fail gate. **None of these is a typed verification artefact.** Phase-7.5c, as scoped in batch D, would either (a) introduce a new artefact type or (b) attach the state enum only to *test attributes* without promoting verification to artefact-rank.

### F2: Zero live `#[ignore]` call sites today

Grepping `/src/` and `/tests/` returns no `#[ignore]` matches. The pattern is *declared* in conventions and the testing-baseline spec but has not yet hit the codebase. The phase-{8.0,9.0,10.0}-tests directories are proposals: their `tasks.md` files declare future `#[ignore = "awaits phase-N"]` writes (e.g., phase-8.0-tests has 12 such tasks; phase-10.0-tests has seven), but none have been applied. **This means phase-7.5c has zero retroactive-rewrite cost** if it lands before any of phase-{8.0,9.0,10.0}-tests applies. If it lands after, the structured replacement must rewrite call sites.

### F3: Cargo workspace has no proc-macro precedent

`Cargo.toml` declares `[workspace] members = ["."]`: the cairn crate is the only workspace member. There is no `cairn-macros`, `cflx-macros`, or any `proc-macro = true` crate. Introducing `#[cflx_planned(phase = N)]` as a proc-macro requires either (a) adding a new workspace member crate (e.g., `cairn-macros/`), or (b) using a non-proc-macro implementation strategy. Option (a) is cheap structurally: Rust supports proc-macro crates as workspace members and the only constraint is that they cannot be the same crate they're consumed from. Option (b) options are weaker: `macro_rules!` cannot generate `#[ignore]` plus a registry-write side effect; build.rs codegen would need to scan source. **Recommend option (a):** add `cairn-macros/` as a new workspace member during phase-7.5c.

### F4: `status-phases` Makefile target is checkbox-based, not ignore-based

`make status-phases` walks `openspec/changes/phase-*/tasks.md` and counts `[x]` vs `[ ]` checkboxes. It does **not** parse `#[ignore]` attributes from Rust source. **No Makefile change required by phase-7.5c.** If phase-7.5c later wants a per-test status command (e.g., `make status-planned` listing all `#[cflx_planned]` sites grouped by target phase), that's a net-new target, not a modification of the existing `status-phases` parsing.

### F5: phase-8.0-tests scope is test-authoring only, not state-machine design

Reading phase-8.0-tests proposal/design/tasks: the change adds **one file** (`tests/phase_8_summariser.rs`) containing 12 `#[ignore = "awaits phase-8"]` tests with `todo!()` bodies. Acceptance criteria are: file compiles, ignored tests skipped under `cargo test`, ignored tests fail under `cargo test -- --ignored`, strict gates pass. **Nothing in phase-8.0-tests's scope discusses verification lifecycle, state enums, ignore discipline, or test taxonomy.** It is a mechanical test-stub authoring task, not a workflow-design task. The roadmap-debate's call-out (line 622) confirms: "the per-test state enum coexists with the per-phase pre-phase-tests directory convention." Different granularities; not the same scope.

### F6: AGENTS.md has exactly one `#[ignore]` reference (line 25)

AGENTS.md is short (46 lines). Line 25 is the single load-bearing reference: "remove the matching `#[ignore = "awaits phase-<N>"]` markers as the feature lands rather than rewriting those tests from scratch." This is the only place AGENTS.md couples to the existing convention. Phase-7.5c's AGENTS.md update is a one-line change (or a small paragraph if the structured replacement requires more nuanced agent instructions).

### F7: Error-code registry is sparse and has no V/L category

The registry (`openspec/registries/error-codes.md`) defines 11 category letters: P, K, A, C, H, E, T, M, S, B, D, covering Parser, Kernel/Map, Artefacts, Changes, Hooks, Edges, Targets, MCP, Summariser, Brownfield, Distribution. Most categories have zero allocated codes; CE has 10 (phase-5), CT has 2 (phase-6). **There is no V (Verification) or L (Lifecycle) category.** Phase-7.5c, if it introduces a `Blocked` error code, must (a) reuse an existing category, most plausibly C (Changes, since cflx-accept gate logic lives in the change-archival flow), or (b) add a new category letter. Adding a new category requires updating both `conventions.md` section 1 (the Category Letters table) and the registry header. This is a small but real cross-cutting change.

### F8: The state enum's "Passed" and "Failed" map cleanly to existing battery semantics; "Planned" is the load-bearing addition

Cairn's verification battery today (`cargo build`/`clippy`/`fmt --check`/`test`/`test --locked`/`cflx.py validate <phase> --strict`) is binary: pass or fail at gate time. The proposed states map as:

- **Draft**: a verification authored but not yet wired to the battery. *No current analogue.* This is a pre-`apply` state.
- **Planned**: a verification scoped to a *future* phase, deliberately skipped now. *Current analogue: `#[ignore = "awaits phase-N"]`.* Hand-rolled, comment-string-typed.
- **Passed**: ran and asserted. *Current analogue: a non-ignored test that returned green under `cargo test`.*
- **Failed**: ran and asserted false. *Current analogue: a non-ignored test that returned red under `cargo test`.*
- **Blocked**: cannot run because of an upstream missing piece (env, dep, fixture). *No current analogue.* Today this is conflated with `Failed`.

The most surgical interpretation of phase-7.5c's scope: **only `Planned` and `Blocked` are net-new state additions.** `Draft`, `Passed`, `Failed` are renamings or formalisations of existing implicit states. This sharpens the scope and reduces phase-7.5c's surface area meaningfully.

### F9: The state enum's "phase-level" granularity question is already answered

Batch D explicitly states (refined-batch-D line 159, and again roadmap-debate line 301) that the state enum lives at *individual test/verification* granularity, **not** phase-level. Phase-level "Planned" is already covered by the `phase-N.0-tests` directory convention. The two coexist at different granularities. **This is settled, not an open question.** Phase-7.5c's spec must call this co-existence out explicitly so future readers don't try to unify them.

## Recommendations

### R1: Phase-7.5c ships before any of phase-{8.0,9.0,10.0}-tests applies

Three pre-phases are sitting unapplied with `#[ignore]`-shaped tasks. If phase-7.5c lands first, those three pre-phases pick up `#[cflx_planned]` from the start; zero retroactive rewrite. If phase-7.5c lands after, every applied test stub must migrate. The cost-asymmetry is sharp: ship 7.5c first.

### R2: Add `cairn-macros/` as a workspace member during phase-7.5c

The proc-macro crate. ~50 LOC of macro implementation (parse phase number from attribute args, emit `#[ignore]` plus a registry-write side effect or compile-time-known constant). Workspace structurally accommodates it; no precedent to break.

### R3: Implement `#[cflx_planned]` as `#[ignore]`-plus-registry in the first iteration

Batch D recommends option (a) over (b) for the same reason this analysis arrives at: keeping `#[ignore]` semantics underneath means `cargo test` continues to work without any test-runner changes. The proc-macro emits both `#[ignore = "cflx_planned: phase-N"]` (or similar) and registers the test name + phase target into a build-time-known set. A future phase can swap to a native skip mechanism if churn warrants. **Do not bundle the native-skip ambition into phase-7.5c.**

### R4: Limit phase-7.5c's artefact-spec impact to a *cross-reference*, not a new artefact type

Adding a `verification` artefact type to `openspec/specs/artefacts/spec.md` is a meaningful kernel change (loader, registry, validation rules, all the machinery v1 types carry). **Phase-7.5c should not do this.** Instead, phase-7.5c attaches the state enum to test attributes via `#[cflx_planned]` and registers the planned set in a sidecar (e.g., `target/cflx/planned.json` or similar build-derived file). A future phase can promote verification to an artefact type if and when there's pull from a downstream consumer.

### R5: Use error-code category C (Changes) for `Blocked`, not a new category

cflx-accept gate logic is the consumer of the Blocked-vs-Failed distinction, and accept lives in the change-archival flow. The C category is currently empty (no codes allocated). Allocating `CC001 -- verification blocked by upstream dependency -- phase-7.5c` is the cheapest path. If phase-7.5c does grow into a verification artefact type later, that future phase can add a V category and migrate. **Do not add a new category letter for one code.**

### R6: Update conventions.md §5 in-place, not by adding a new section

The `#[ignore]` reference in `conventions.md` (lines 213-215) is two paragraphs. The phase-7.5c update replaces "MUST be marked `#[ignore = "awaits phase-<N>"]`" with "MUST be marked `#[cflx_planned(phase = <N>)]`" plus a one-paragraph note explaining the proc-macro expands to `#[ignore]` underneath. Same paragraph budget. The testing-baseline spec at `openspec/specs/testing-baseline/spec.md` requirement 4 needs the same update applied in lockstep.

### R7: Update AGENTS.md line 25 and add a one-line note about the state enum

The agent-facing instruction stays simple: "remove the `#[cflx_planned(phase = <N>)]` attribute as the feature lands." Add one line: "The attribute is structured (proc-macro), not a comment, do not parse the comment string." Total AGENTS.md delta: under 5 lines.

## Decisions made (with reasoning)

### D1: phase-8.0-tests collision resolution: **NO COLLISION; phase-7.5c lands FIRST**

**Decision:** Phase-7.5c and phase-8.0-tests are complementary, not contradictory. Phase-7.5c ships **before** phase-8.0-tests (and phase-9.0-tests and phase-10.0-tests).

**Reasoning:**

1. Phase-8.0-tests's scope is *test-authoring*: write 12 ignored test stubs for the summariser. It is a mechanical pre-phase. Its scope contains zero state-lifecycle, ignore-discipline, or test-taxonomy design. F5 confirms this from a direct read of all three files.
2. Phase-7.5c's scope is *workflow lifecycle*: name the states, build the proc-macro, update conventions and AGENTS.md.
3. The two work at different layers. Phase-8.0-tests authors test stubs; phase-7.5c defines the attribute used to mark stubs as planned. If 7.5c ships first, 8.0-tests uses `#[cflx_planned(phase = 8)]` from the start. If 8.0-tests ships first, 7.5c must rewrite the 12 stubs (and the seven in phase-10.0-tests, and however many in phase-9.0-tests).
4. The retroactive-rewrite cost (F2 + roadmap-debate N3) is the deciding factor. Zero live `#[ignore]` exists today. Three pre-phases are queued. Order them so the workflow phase ships first and the test-authoring phases inherit the new convention. Rewriting test stubs is mechanical, but it's still a phase-archival event for each pre-phase, with its own verification battery and review cycle. Avoidable.
5. The roadmap-debate already flagged this as N3 (line 576-580): "Whether the phase-8.0/9.0/10.0 pre-phase tests already in `openspec/changes/` (per the recent commit `c98d506`) are blocking on the structured `#[cflx_planned]` replacement, or if their `#[ignore = "awaits phase-N"]` form is fine for now." This cross-check resolves N3: **the existing `#[ignore]` form is fine for the *proposal text* of those pre-phases, but phase-7.5c should land before any of them applies, so the apply step uses the new attribute.**

**Alternative considered and rejected:** Fold phase-7.5c into phase-8.0-tests. *Rejected* because (a) phase-7.5c also services phase-9 and phase-10's pre-phases, (b) it modifies cross-cutting docs (conventions.md, AGENTS.md, error-codes registry, testing-baseline spec) that have nothing to do with the summariser, (c) phase-8.0-tests's stated scope is "no `src/` changes" and adding a proc-macro crate violates that, (d) bundling dilutes both phases' scopes, phase-8.0-tests becomes harder to verify and phase-7.5c loses its standalone usefulness for phase-9 and phase-10.

**Sequence recommendation:**

```
phase-7.5b-cleansing-splits (archived)
  → phase-7.5c-verification-states  (NEW: workflow phase)
    → phase-8.0-tests  (uses #[cflx_planned(phase = 8)] from the start)
      → phase-8-summariser  (removes #[cflx_planned] as features land)
        → phase-9.0-tests
          → phase-9-brownfield
            → phase-10.0-tests
              → phase-10-distribution
```

### D2: Blocked-vs-Failed semantics: **Resolved with one new error code**

**Decision:** `Blocked` = the test could not be executed because a precondition outside the test's responsibility was unmet (missing fixture, unavailable network, missing upstream artefact, missing phase still in `Planned` state). `Failed` = the test executed and an assertion was false. The distinction surfaces via a new error code `CC001 -- verification blocked by upstream dependency -- phase-7.5c` in the existing C (Changes) category.

**Reasoning:**

1. The operational distinction matters because the *fix path* differs. Failed → look at the test, the code, the assertion. Blocked → look at the environment, the upstream phase, the missing fixture. Conflating them sends agents and humans down the wrong investigation path.
2. The error code lives in C (Changes) because the consumer is `cflx accept` gate logic, and gate logic is the change-archival flow. The C category is currently empty: allocating `CC001` is free of collision risk (F7).
3. A new category letter (V for Verification, L for Lifecycle) is overkill for a single code. The C category is the right home. If phase-7.5c later promotes verification to a kernel artefact, that future phase can split out a V category and migrate `CC001` (with a deprecation note per registry rule 3, codes are stable once assigned, so the migration would be additive: allocate `CV001` and mark `CC001` as superseded, never re-use).
4. The trigger conditions for `Blocked` need explicit specification in the phase-7.5c spec deltas. Three sub-categories worth distinguishing (potentially three error codes if precision warrants):
   - **Missing upstream phase**: the test asserts behaviour from a phase that hasn't applied yet. Detected by checking whether `phase-N` (the target of the `#[cflx_planned(phase = N)]`) is `archived` in `openspec/changes/archive/`.
   - **Missing infrastructure**: the test requires a binary, fixture, or environment variable that's absent. Detected at test runtime via a `cflx::block_if_missing` helper or similar.
   - **Missing artefact reference**: the test exercises an artefact (contract, decision, etc.) that doesn't exist in the map. Detected during pre-test setup.
5. **Recommendation: ship one code (`CC001 -- verification blocked by upstream dependency`) initially, add `CC002` and `CC003` in a follow-on if the three sub-categories prove operationally distinct in practice.** Premature codification of three sub-categories without operational evidence is over-engineering. The single code carries the semantic distinction from `Failed`, which is the load-bearing change.

**What I would have deferred but no longer need to:** Whether `Blocked` semantics need their own design pass before phase-7.5c can ship. They don't. The single-code formulation above is sufficient for phase-7.5c's scope. The three-sub-category split is a follow-on if needed.

### D3: Verification artefact type: **NOT introduced by phase-7.5c**

**Decision:** Phase-7.5c does **not** add a `verification` artefact type to `openspec/specs/artefacts/spec.md`. The state enum attaches to test attributes via `#[cflx_planned]` and is registered in a build-derived sidecar (e.g., `target/cflx/planned.json`).

**Reasoning:**

1. F1 establishes that no verification artefact type exists today. Adding one is a kernel change with full v1-artefact-type machinery (loader, validation rules, frontmatter schema, scenario coverage), easily 200+ LOC and 5+ scenario additions.
2. Phase-7.5c's stated scope is "small dedicated phase" (refined-batch-D §5; roadmap-debate line 422 estimates "500-800 LOC + ~1000 words of spec/docs").
3. The attribute-plus-sidecar approach gives phase-7.5c its full claimed value (replacing `#[ignore = "awaits phase-N"]` with structured data; per-test state queryable; cflx accept gate consults the state) without paying the artefact-type tax.
4. If a downstream consumer (e.g., a `cflx verifications --status planned` command, deferred to LATER per batch D) materialises and demands a richer schema, that's the moment to promote verification to a kernel artefact. Not before.
5. This decision is consistent with the C11 batch-D verdict and the cluster observation that cairn welcomes named-state surfaces but rejects framing changes that re-shape kernel topology.

### D4: Workspace structure: **Add `cairn-macros/` as a new workspace member**

**Decision:** Phase-7.5c adds `cairn-macros/` as a new Cargo workspace member, declaring `proc-macro = true`. This is the home for `#[cflx_planned]` and any future cflx-side proc macros.

**Reasoning:**

1. F3 establishes there's no proc-macro precedent in the workspace. Choice between (a) new workspace member, (b) macro_rules!, (c) build.rs codegen.
2. macro_rules! cannot do what `#[cflx_planned]` needs: emit `#[ignore]` plus a side-effect (writing to a registry). Out.
3. build.rs codegen would scan source for the attribute and write the registry. This works but is fragile (re-scan on every build, file-system race conditions, harder to test in isolation). Out.
4. New workspace member is structurally cheap. ~50 LOC for the macro itself, plus a `Cargo.toml` and a `lib.rs`. Naming: `cairn-macros` follows the Rust ecosystem convention (`thiserror-macros`, `tokio-macros`, `serde-macros` historically). Avoid `cflx-macros` because proc-macros run at compile time as part of cairn-the-crate, not cflx-the-workflow-runner.
5. Naming the attribute `#[cflx_planned]` (per refined-batch-D) is *user-facing*: the attribute name is what test authors type. The crate name (`cairn-macros`) is internal. The two need not match.

## Open questions for next session

### O1: Should `#[cflx_planned(phase = N)]` accept additional kinds beyond `phase`?

Batch D's refinement implies `phase = N` is the only argument. But the `Blocked` discussion suggests other planned-reasons exist (missing infrastructure, missing artefact). Should the attribute accept variants like `#[cflx_planned(infra = "docker")]` or `#[cflx_planned(artefact = "decision:foo")]`? **Recommendation for next session:** start with `phase = N` only, defer alternative arguments to a follow-on phase if Blocked sub-categorisation proves needed. But explicitly call out in phase-7.5c's design.md that the attribute syntax is *forward-compatible* with additional named arguments.

### O2: Where does the phase-7.5c spec delta live?

Three candidate areas: (a) `openspec/specs/testing-baseline/`: most natural home, since the existing convention lives there; (b) a new `openspec/specs/verification-states/` area, clean but adds a new spec area for one phase; (c) split between testing-baseline and conventions, most accurate but most diffuse. **Recommendation for next session:** option (a). The testing-baseline spec already owns the `#[ignore]` convention; phase-7.5c is its evolution. Adding a fourth requirement to testing-baseline ("Verification states attached to test attributes") is the cleanest delta.

### O3: Does phase-7.5c need to run before Bundle A or in parallel?

Roadmap-debate N3 (line 576) raises this. Bundle A is provenance-foundation work; phase-7.5c is verification-lifecycle work; the two are infrastructurally independent. Both should ship before the next feature pre-phase applies (phase-8.0-tests), but neither blocks the other. **Recommendation for next session:** parallel: assign whichever the orchestrator has free capacity for. The only constraint is that both ship before phase-8.0-tests applies.

### O4: Does the proc-macro need to support `#[ignore]` interop for tests that are *both* planned and explicitly ignored for an unrelated reason?

Edge case: a test marked `#[cflx_planned(phase = 9)]` that also happens to need `#[ignore = "flaky on macOS"]`. Stacking `#[ignore]` attributes is allowed by Rust but the proc-macro emitting `#[ignore]` underneath could collide. **Recommendation for next session:** accept this as a real edge case but defer to first-encounter. Most planned tests will not be also-ignored. If it comes up, the macro can detect existing `#[ignore]` and either layer or emit a compile error.

## Recommended phase-7.5c final scope

**CONFIRMED. Standalone. No scope change beyond clarifications.**

The roadmap-debate's proposed scope holds. With the decisions above:

1. **Add `cairn-macros/` workspace member.** ~50 LOC for the `#[cflx_planned(phase = N)]` proc-macro. Macro emits `#[ignore = "cflx_planned: phase-<N>"]` plus registry-write side effect.
2. **State enum on test attributes** (Draft, Planned, Passed, Failed, Blocked). Attached via the proc-macro and the existing `cargo test` pass/fail signal. **No verification artefact type added.** State is sidecar-tracked.
3. **`cflx accept` gate logic update.** Consult the planned set; treat `Planned` as out-of-scope for the current phase's gate; surface `Blocked` distinct from `Failed` via error code `CC001`. ~100 LOC Rust.
4. **conventions.md §5 update.** Replace `#[ignore = "awaits phase-<N>"]` references with `#[cflx_planned(phase = <N>)]` in the Test-First Pre-Phase subsection. Same paragraph budget.
5. **testing-baseline spec update.** Requirement 4's prose and scenarios update in lockstep with conventions §5. Optionally add a fourth (or fifth, depending on numbering) requirement: "Verification states attached to test attributes."
6. **AGENTS.md line 25 update.** ~5 lines.
7. **error-codes registry update.** Allocate `CC001 -- verification blocked by upstream dependency -- phase-7.5c`. First C-category code.
8. **Sequencing decision.** Phase-7.5c ships **before** phase-8.0-tests applies. This is a hard ordering constraint; the apply agent for phase-8.0-tests reads conventions.md and uses whichever attribute is current.

**Total estimated size:** 500-700 LOC Rust + ~1000-1500 words of spec/docs + 1 new workspace member crate. Mid-small phase.

**Phases that follow phase-7.5c are unchanged in scope.** Phase-8.0-tests, phase-8-summariser, phase-9.0-tests, phase-9-brownfield, phase-10.0-tests, phase-10-distribution all continue exactly as proposed; the only difference is they pick up `#[cflx_planned]` instead of `#[ignore = "awaits phase-N"]` from the start.

**Stronghold path:** `/Users/george/repos/cairn/docs/strongholds/getcairn-cross-check-7.5c.md`
