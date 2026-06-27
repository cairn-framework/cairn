# TDD posture investigation

## Method

Files read:

- `/Users/george/repos/cairn/CLAUDE.md` (workflow + verification + "what to avoid" sections)
- `/Users/george/repos/cairn/AGENTS.md` (codex apply-stage agent orientation, full file)
- `/Users/george/repos/cairn/openspec/conventions.md` (full file; sections 5 and 7 closely)
- `/Users/george/repos/cairn/openspec/specs/testing-baseline/spec.md` (full file, five requirements)
- `/Users/george/repos/cairn/openspec/changes/phase-8-summariser/proposal.md` and `tasks.md`
- `/Users/george/repos/cairn/openspec/changes/phase-8.0-tests/proposal.md` and `tasks.md`
- `/Users/george/repos/cairn/openspec/changes/archive/phase-7.5a-test-fortification/proposal.md`
- `/Users/george/repos/cairn/.cflx.jsonc` (apply_prompt + lifecycle hooks)
- `/Users/george/repos/cairn/.claude/skills/cflx-workflow/SKILL.md` (test/verification clauses)
- `/Users/george/repos/cairn/Makefile` (`check` target)

Greps run:

- `grep -ri "TDD" --include="*.md"` over `/Users/george/repos/cairn`: **zero hits in all canonical prose** (only worktree shadows)
- `grep -ri "test-driven|test driven|test-first|test first|tests-first|tests first" --include="*.md"`: many hits, all on the phrase "test-first pre-phase" (never "test-driven development")
- `grep -r "#\[test\]" src/ | wc -l` → **27**
- `grep -r "#\[cfg(test)\]" src/ | wc -l` → **6** (lib.rs + the five post-7.5b directory module roots)
- `find tests/ -name "*.rs" | xargs grep -c "#\[test\]"` → **kernel 29, mcp 7, graph_explorer 5, fixtures_smoke 3, check_file_sizes 1, wire_format_snapshots 1** (≈46 integration `#[test]` markers; total ≈73)
- `find src/ -name "*.rs" | wc -l` → **58 source files**

## Findings

### F1: Current TDD posture

Cairn does **not** practice TDD as the term is normally used (write a failing unit test, watch it fail, implement the smallest change to make it green, refactor under green; loop at the function level, in a single session). Three pieces of direct evidence:

**(a) The string "TDD" appears zero times in canonical repo prose.** Greps over `/docs`, `/openspec`, `/CLAUDE.md`, `/AGENTS.md` produce no hits. The only matches are inside `.claude/worktrees/` shadow copies (transient agent worktrees) and one mention of TDD-adjacent rigor inside `docs/strongholds/` adoption-matrix files where I (the investigator) am the author. The framework's authors have never used the acronym in their own normative documents.

**(b) `openspec/conventions.md` §5 ("Testing Conventions") describes a different discipline.** It mandates colocated unit tests, integration tests at `tests/`, the `insta` snapshot-test pattern, a coverage requirement (every public fn has a happy-path test, every error code has a triggering test), and a **test-first pre-phase convention** (lines 211–215). That last paragraph is the closest cairn comes to TDD, and the difference matters: it is **test-before-feature-phase**, not test-before-feature-commit. The pre-phase ships a separate `apply` cycle whose entire output is failing-but-ignored test stubs; the feature phase later removes the `#[ignore]` markers as code lands.

**(c) The verification battery is regression-shaped, not cadence-shaped.** `cflx`'s `pre_archive` hook is `scripts/pre-archive-rust-gates.sh` and the codex `apply_prompt` requires `cargo build` (zero warnings), `cargo clippy -D warnings`, `cargo fmt --check`, `cargo test`, plus `cflx.py validate <phase> --strict`. None of these gates check authoring order, commit interleaving, or whether tests preceded their target code within the phase. The gate verifies the destination, not the path. (Source: `/Users/george/repos/cairn/.cflx.jsonc` and `Makefile:check` target.)

The `cflx-workflow` skill (`/Users/george/repos/cairn/.claude/skills/cflx-workflow/SKILL.md`) does instruct the agent to keep tests honest at task granularity ("Code task → matching `src/` diff exists. Test task → matching `tests/` diff exists. Wiring/integration task → real entrypoint exists.") and explicitly fails an accept review when "code was discussed but no runtime/test artifact was added". This is anti-stub-completion enforcement, not TDD: it ensures tests *exist* for completed tasks, not that tests *preceded* the implementation.

**Net posture: cairn practices a structured test-after discipline at the per-task scale, embedded inside a test-first discipline at the per-phase scale.** The two scales are not the same thing.

### F2: phase-N.0-tests pattern

The pattern is genuinely test-first **at the phase boundary** but not at the commit-by-commit boundary that classical TDD operates on.

Evidence: `phase-8.0-tests/tasks.md` enumerates 12 task lines (1.1–3.2) of the form "Write `#[ignore = \"awaits phase-8\"]` test: <scenario>". Verification 4.5 explicitly checks `cargo test -- --ignored` reports all 12 new tests as **FAILED** (because their bodies are `todo!()`). So when the pre-phase archives:

1. Tests exist on disk in `tests/phase_8_summariser.rs`.
2. They compile (the trait/type stubs they reference must exist or be `todo!()`).
3. `cargo test` is **green** because every new test is `#[ignore]`.
4. `cargo test -- --ignored` is **red** because every body is `todo!()`.

Then `phase-8-summariser/tasks.md` (the feature phase) groups its work by Backend & Configuration (1.x), Prompt Inputs & Storage (2.x), Resolution Actions (3.x), CLI & Documentation (4.x), Verification (5.x). Per the spec at `openspec/specs/testing-baseline/spec.md` Requirement "Test-first pre-phase convention" scenario 2, the apply agent removes `#[ignore]` attributes "as the corresponding feature code lands", not before, not in a separate later commit.

So the phase-N.0-tests pattern matches **classical TDD's red→green** beat, but at a coarser cadence:

- **Pre-phase apply** = write all tests at once, mark them red-but-skipped, archive. This IS test-before-implementation.
- **Feature phase apply** = the codex agent goes group by group; for each group, removes `#[ignore]` from the matching test, watches it fail (because the implementation is still missing), implements, watches it pass.

The micro-loop *inside* a group is up to the codex agent. It could write the implementation first then unskip the test (test-after); it could unskip first then implement (test-first); the gate doesn't see the difference because the gate only checks the end-of-phase state. Reviewing `phase-8-summariser/tasks.md`, none of the 23 implementation task lines mandates an order, they say "implement X" and "add tests for X" as separate sub-tasks (e.g., 3.6 "Add tests for every action"), suggesting the assumed cadence is implement-then-test-fill at the function level.

**Verdict on F2: The pattern is test-first at the phase level (commit hash before commit hash) but test-after or unspecified at the function level.** Not TDD in the Beck/Kent sense; closer to "acceptance-test-driven development" (ATDD) where acceptance criteria become committed test stubs before implementation begins.

The earliest archived evidence of this pattern is `phase-7.5a-test-fortification` (proposal: "Phase 7.5a establishes that wall so the split [phase-7.5b] can be executed by a codex agent whose only correctness signal is the gate battery"). The pattern was *invented* to make codex agents grade-able. It is more an oracle scaffold for autonomous agents than a developer-facing TDD discipline.

### F3: Test coverage spot-check

Counts:

| Location | `#[test]` markers | Notes |
|---|---|---|
| `src/lib.rs` | (one block) | crate-level smoke |
| `src/changes/mod.rs` | (block) | post-7.5b mod root |
| `src/cli/mod.rs` | (block) | post-7.5b mod root |
| `src/query_api/mod.rs` | (block) | post-7.5b mod root |
| `src/artefacts/registry/mod.rs` | (block) | post-7.5b mod root |
| `src/ui/mod.rs` | (block) | post-7.5b mod root |
| **Total `#[test]` in `src/`** | **27** | across **58 source files** |
| `tests/kernel.rs` | 29 | the heavyweight integration file |
| `tests/mcp.rs` | 7 | |
| `tests/graph_explorer.rs` | 5 | |
| `tests/fixtures_smoke.rs` | 3 | |
| `tests/check_file_sizes.rs` | 1 | |
| `tests/wire_format_snapshots.rs` | 1 (file) | uses `insta` per-snapshot, not per-test |
| **Total `#[test]` in `tests/`** | **≈46** | excluding insta variants |
| **Grand total** | **≈73** | |

Density observation: **6 of 58 source files carry an inline test block (~10%).** The 6 are all the post-7.5b "god module roots" (which inherited their inline tests from the pre-split monoliths via the `phase-7.5a` fortification phase) plus `lib.rs`. The other 52 source files: including everything under `src/blueprint/`, `src/scanner/`, `src/reconcile/`, `src/edges/`, `src/targets/`, `src/hooks/`, `src/mcp/` (recursive submodules), carry **zero inline unit tests**.

This is consistent with cairn's stated posture: heavy reliance on integration tests + snapshot tests at the public API surface, plus inline tests for the most behavior-rich top-level dispatchers, plus a five-requirement testing-baseline spec that pins JSON wire formats and the 500-line ceiling. It is **not** consistent with a "we test everything at the unit level" posture. The kernel-level integration test file (`tests/kernel.rs`, 29 tests) carries most of the assertion weight for the parser + reconciler + scanner stack.

The conventions.md "Coverage Requirements" rule ("Every public function MUST have at least one test exercising its success path") is aspirational and not gate-enforced. There is no public-fn-coverage check anywhere in `pre-archive-rust-gates.sh` or the Makefile `check` target.

### F4: Prior TDD discussion in repo

**Zero direct mentions.** The string "TDD" is absent from canonical prose. The phrase "test-driven" never appears (greps over `--include="*.md"`).

The closest prior discussions are:

- `phase-7.5a-test-fortification` archive (2026, recent): the originating thinking for the test-first pre-phase pattern. Frames it as a "regression wall" + "correctness signal for codex agents", **not** as TDD adoption. The author's stated motivation is that "the framework that catches drift should catch its own", a self-defense argument, not a methodological one.
- The newer phase pairs (`phase-8`/`phase-8.0`, `phase-9`/`phase-9.0`, `phase-10`/`phase-10.0`) all reference "the test-first pre-phase convention defined in `openspec/specs/testing-baseline/spec.md`". They never call it TDD.
- `docs/strongholds/getcairn-cross-check-7.5c.md` discusses the `#[ignore = "awaits phase-N"]` convention extensively. It treats the pattern as **planned-test infrastructure** (related to the proposed `#[cflx_planned(phase = N)]` macro for verification states), not as a TDD methodology.
- `docs/strongholds/getcairn-refined-batch-D.md` notes that "the codebase already encodes [progressive verification] informally via `#[ignore = \"awaits phase-N\"]` markers", again framed as *encoded planning*, not as developer rhythm.

**No phase, spec, or stronghold has previously asked the question "should cairn adopt TDD."** This investigation is the first.

## Adversarial debate: should TDD be added to cflx/openspec?

### Case-for

1. **Eat-own-dog-food.** Cairn ships drift detection, accept-gates, and verification states (Bundle E + 7.5c). A framework whose job is to defend code against drift should have a maximally-defended kernel. TDD's per-function red→green→refactor loop produces denser test coverage *during* implementation than test-after almost ever does. The 27 `#[test]` markers across 58 source files (10% inline coverage) is not "we test everything"; classical TDD would push that closer to 70–90%.
2. **Codex agents benefit from finer-grained oracles.** The phase-N.0-tests pattern was invented because "the codex agent's only correctness signal is the gate battery" (phase-7.5a proposal). That argument scales: the more granular and intermediate the failing tests, the less an agent has to think. A `cflx tdd` watch-mode that sees a red unit test and turns it green is a *simpler* control-loop than the current "implement-the-whole-group-then-unskip-and-pray" cadence.
3. **Phase 7.5c verification states + Bundle A accept-gate are safety-critical paths.** Bugs in the gate machinery silently approve broken code. TDD on the gate's own logic would force the failure mode to exist as a test before the fix exists as code.
4. **Phase-N.0-tests is explicitly test-AFTER for the unit level.** That's the inverse of TDD. If the framework wants to claim "test-first" rigor in marketing, the unit-level interleaving needs to actually happen, not just the phase-boundary stub.
5. **Bug-driven development.** Whenever a regression is found post-archive, the natural cairn flow is: write a failing test reproducing the bug, fix the code until it passes. That's TDD. Codifying it as a convention removes ambiguity for the codex agent fixing the bug.

### Case-against

1. **The phase-N.0-tests pattern already gives 80% of TDD's value at 20% of the cost.** Acceptance-criterion stubs ship before implementation; the codex agent grades itself against `cargo test -- --ignored`. Adding a per-function TDD loop on top is gold-plating relative to the user's actual ask (which was: should TDD also be adopted, given six getcairn-derived bundles already in flight). The pre-phase pattern is a strict superset of ATDD (acceptance-test-driven), which is itself a strict superset of TDD's *outcome* (tests-before-implementation).
2. **TDD is a per-developer style, not a framework-level constraint.** Forcing it via cflx would over-constrain authors. The codex agent's apply prompt already runs `cargo test` after every meaningful change inside a phase; whether the agent writes the test first or last inside a single sub-task matters less than whether the gate is green at archive time.
3. **The verification battery already catches regressions before merge.** `cargo build`, `cargo clippy -D warnings`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx.py validate --strict`, `pre-archive-rust-gates.sh`. TDD adds no new safety net at the merge point. It changes the *sequencing* of work inside a phase, not the *outcomes* the gate checks.
4. **Adding TDD on top of Bundles A/B/C/E + 7.5c is genuine scope creep beyond the user's actual ask.** The session has been about getcairn.dev adoption candidates. Adding C16 (TDD adoption) to the matrix is a reasonable thought, but the cost-benefit ratio is materially worse than the existing C1–C15 candidates because TDD adoption *cannot be enforced by tooling* in the same way (you can detect post-hoc that a test exists; you cannot reliably detect that it was written first without commit-archaeology heuristics that are easy to fool).
5. **Cairn's voice deliberately emphasises pluralism.** CLAUDE.md notes "user-facing vocabulary should prefer plain concise English" and the spec explicitly accommodates "people building with AI tools, including non-devs". Mandating TDD: a methodology with a tribal connotation in the JS/Ruby ecosystems and contested status in the Rust community, narrows the audience for no obvious technical gain.
6. **The 10% inline coverage isn't the problem the integration suite claims to solve.** Cairn's `tests/kernel.rs` (29 tests) plus `tests/mcp.rs` (7) plus snapshot pinning of every `/api/*` endpoint is the actual quality wall. The codebase chose an integration-heavy, snapshot-pinned posture deliberately; raising unit-test density via TDD would change the architecture, not just the cadence.

### Pivotal question

**Does the marginal value of per-function red→green discipline exceed the marginal cost of adding a fifth methodology layer (after gate-battery + phase-N.0-tests + cflx-workflow accept-review + conventions.md coverage rules)?**

The honest answer turns on whether you trust the codex agent to do reasonable test-after work without a TDD framing. Empirically (last 11 archived phases): yes. The codex agent has consistently shipped `cargo test`-passing phases under the existing prompt. Forcing TDD on top would mostly produce procedural noise (the agent would write `let result = todo!(); assert_eq!(result, expected);` first, then immediately fill in `result = ...` the same authoring pattern, with two extra commits per function). That noise has a real cost in commit-history readability and in agent token usage.

If the goal is "the codebase has good tests", the right lever is **raising the conventions.md coverage requirement from aspirational to gate-enforced** (a public-fn coverage check in `pre-archive-rust-gates.sh`), not adopting TDD. That gives the same outcome (every public fn has a test) without prescribing authoring order.

## Decision recommendation

- **Verdict:** **Don't add TDD.** Keep the existing test-first pre-phase + integration-heavy + snapshot-pinned posture. If coverage feels thin, harden the existing conventions.md coverage rule into a gate (a separate, scoped change), not via methodology mandate.
- **Reasoning:** The phase-N.0-tests pattern already delivers test-before-implementation at the phase level, which is where it matters for codex-agent grade-ability. Adding a per-function TDD discipline on top is methodology theatre: the gate doesn't see authoring order, the codex agent doesn't need it, and the user's audience (non-devs building with AI tools) doesn't speak TDD vocabulary. The legitimate underlying concern (low inline test density) is better solved by gate-enforcing the existing public-fn coverage rule than by adopting a new methodology label.
- **Confidence:** ~75%. The 25% downside is the eat-own-dog-food argument: a drift-defense framework should have unusually robust internal tests, and 10% inline coverage is genuinely modest. But that's a *coverage* problem, not a *TDD* problem.
- **What would flip this:** (a) A regression in the gate machinery itself (Bundle A or 7.5c) traceable to under-tested logic, that would push toward kernel-only TDD as a focused defense. (b) A codex-agent failure mode that turns out to be specifically "agent wrote implementation then forgot to write the test" rather than "agent wrote a wrong implementation", that's evidence the test-after habit is missing tests, not just mis-implementing. (c) The user explicitly broadening cairn's positioning toward TDD-fluent dev audiences (e.g., a "for serious software shops" positioning), in which case TDD vocabulary becomes a marketing asset rather than a tax.

## If adopted: candidate framing for the adoption matrix

Treating this as a hypothetical **C16: Adopt TDD as a kernel-only convention**:

| Field | Value |
|---|---|
| **What** | Add a "Kernel TDD requirement" subsection to `openspec/conventions.md` §5 stating: for changes touching `src/scanner/`, `src/reconcile/`, `src/blueprint/`, `src/kernel/` (or whichever modules constitute "the kernel"), each `#[test]` SHALL be authored in a commit prior to its target implementation, verifiable via `git log --reverse` over the phase branch. UI work, CLI surface work, and brownfield/distribution work are exempt. |
| **Why** | The kernel is cairn's safety-critical path. The phase-N.0-tests pattern provides phase-level test-first; this would extend it to commit-level test-first within the kernel only, mitigating the "implement-then-unskip-and-pray" failure mode for the most consequential code. |
| **Case-for** | Eat-own-dog-food on a drift-defense framework. Codex-agent failure-mode mitigation (forces the agent to articulate the assertion before the implementation). Restricts scope to where TDD is unambiguously valuable (kernel) without imposing it on UI/CLI/marketing work. |
| **Case-against** | Authoring-order convention is fragile (agent can author both in one commit and the gate doesn't notice). Hard to enforce automatically without commit-archaeology heuristics (squashing a phase branch destroys the signal). Adds methodology vocabulary to a project that has deliberately avoided it. Creates a two-tier convention (kernel vs not) which itself needs maintenance as modules move. |
| **Leaning** | **Defer.** Net negative against the existing phase-N.0-tests + gate-enforced coverage path. Reconsider if a kernel regression demonstrably traces to under-tested logic. |
| **Evidence pointers** | `openspec/conventions.md` §5 lines 211–215 (existing test-first pre-phase); `openspec/specs/testing-baseline/spec.md` Req "Test-first pre-phase convention"; `phase-8.0-tests/tasks.md` (12 stub tasks demonstrating the existing rhythm); zero historical mentions of "TDD" in repo prose; `cflx-workflow/SKILL.md` "Code task → matching `src/` diff exists. Test task → matching `tests/` diff exists." (current granularity of the workflow's test discipline). |

## What this changes for the integrated plan

**Nothing material.** Bundle A (accept-gate inside `cflx.py validate --strict`), Bundle B, Bundle C, Bundle E, and 7.5c (verification states / `#[cflx_planned(phase=N)]` macro) all stay as-scoped. The verification-states macro in 7.5c is, in fact, a *cleaner* expression of the existing `#[ignore = "awaits phase-N"]` pattern, moving toward TDD-style visibility (red planned-test) rather than TDD-style authoring discipline. If anything, 7.5c reduces the marginal benefit of adopting TDD because it makes the planned-but-not-yet-passing test state more legible without requiring per-developer TDD habits.

If the user wants to capture the underlying concern (kernel coverage), the right addition to the integrated plan is **a small "gate-enforce the existing public-fn coverage rule" change**, not a TDD adoption. That would be a half-day scoped change, not a methodology pivot.
