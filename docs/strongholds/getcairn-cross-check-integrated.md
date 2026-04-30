# Cross-check integration: final actionable plan

## Methodology

Six area-debate agents independently cross-checked the getcairn.dev adoption candidates that came out of `getcairn-roadmap-debate.md` against current openspec content, the live repo state, and the active phase corpus. This integration reads each cross-check as authoritative for its bundle and treats the union of their findings as the new ground truth. Where two cross-checks agree on a pattern (cflx-vs-cairn rename, validate-fix in-bundle, Phase 9 absorption) the agreement is promoted to a cross-cutting decision; where they collide on assumptions (parallel ship-orderings, shared scaffolding) the integrator resolves it explicitly below.

The output is a sequenced commit/phase plan, an open-question consolidation, and a set of integrator-level decisions that no single bundle could ratify alone. Sources: cross-check-F, A, B, 7.5c, C, E (all in `/Users/george/repos/cairn/docs/strongholds/`).

## Final unified scope table

| Bundle / phase | Status | Net scope change vs roadmap-debate | Sequencing | LOC estimate | Validate-fix included |
|---|---|---|---|---|---|
| **Bundle F** (identity additions) | CONFIRMED with expansion | +50 words for propose-vs-approve sentence on C14 (closes Q43); §3.5 placement confirmed (not §3.4.1); CLAUDE.md positive section between Workflow:cflx and What to avoid | Commit #1, before everything else | ~365 words across `docs/spec.md` + `CLAUDE.md` | n/a (no validate gate on repo-root prose) |
| **phase-7.5c** (verification states) | CONFIRMED standalone | No scope change; clarified: only Planned/Blocked are net-new states; verification artefact type NOT introduced; `cairn-macros/` workspace member added; `CC001` in C category for Blocked | Must precede phase-8.0-tests apply (hard) | ~500-700 LOC + ~1500 words spec/docs + 1 new crate | n/a (touches conventions.md + testing-baseline; both currently OK) |
| **Bundle A** (`phase-7.6-ai-provenance-foundation`) | CONFIRMED with sharpened internals | C5.a islands split into two CLI surfaces; verb-edge as commit rider; C8.b lands inside `cflx.py validate --strict`; C6 sidecar location decoupled from meta/openspec rift | Parallel with Bundle B; both after F + 7.5c | ~550-1000 LOC | NO (Bundle A explicitly non-blocking on reconciliation/artefacts validate) |
| **Bundle B** (`phase-7.7-ux-foundation`) | SCOPE-CHANGED | +`FindingSeverity::Info` kernel enum addition (~15 match sites) at commit #1; -`cairn check --json` deferred; `cflx check` → `cairn check`; copy file at `docs/design-system/copy.toml`; voice checklist folded into `design-system/README.md` | Parallel with Bundle A | ~1100-1800 LOC | OPEN: reconciliation spec validate failure (see Pattern 2) |
| **Bundle C** (`phase-7.x-graph-explorer-followups`) | CONFIRMED with rider declared | C9-salvage marked optional in-bundle rider (largest, riskiest); C5.b clarified as verb-labels not systemigram; v1 widget reads blueprint edges only | After Bundle B's `Info` kernel change is in (no other coupling); collision-free with active phases | ~400-700 LOC + ~150-250 words spec | YES (graph-explorer `## Purpose` fix in commit #1; ~100 words) |
| **Bundle E** (`cairn export`) | SCOPE-CHANGED | `cflx export` → `cairn export`; `--output` mandatory in v1 (no default); `--md` is decision-log shape, NOT map.md; ships as single commit not a phase | Lifecycle-orthogonal; can land any time after F | ~250-400 LOC | NO (no validate-failure in scope) |
| **C1.a** (genesis transcript) | CONFIRMED, locked | Skill-body edit only; genesis.md schema defined here (id/nodes/date/sources/informed_by/type); validator NOT enforcing; AGENTS.md untouched | Independent of all other bundles; ship before or alongside Bundle E | ~50-100 lines skill prose + 1 line conventions.md | NO |
| **Phase 9 brownfield** | SCOPE-CHANGED (re-scoped to absorb) | Must explicitly absorb C8.c suggest engine + C1.b interview runner + C15 templated authoring + C4.b decision-attached obligations follow-on | After Bundle A (C8.b safety must precede C8.c); after phase-9.0-tests | ~unknown until re-scoping | YES (likely; touch points are reconciliation/artefacts) |

## Cross-cutting patterns

### Pattern 1: cflx-vs-cairn binary discipline

**Findings.** Bundle B's cross-check (F3, R1, D3) caught `cflx check` as a category error. Bundle E's cross-check (F2, R1, D1) independently caught `cflx export` as the same category error. Both reasoned from the same principle: **`cflx` operates on phase lifecycle (propose / apply / accept / archive); `cairn` operates on framework data (blueprint, map, artefacts, graph).** Two strikes is a pattern. The roadmap-debate text used `cflx` as a default prefix for several near-term commands without applying the binary-distinction test.

Sweep across the six cross-checks for other commands:

- `cflx trace <phase>` (Bundle A C6): correctly cflx-side. The trace sidecar is *workflow-runner* state for an executed phase, not framework data. Bundle A's design.md should keep this on cflx; cross-check A confirms (D7, F4).
- `cflx accept` gate addition (Bundle A C8.b): correctly cflx-side. Lives in `cflx.py validate --strict`. Cross-check A R3 confirms.
- `cflx-proposal` skill (C1.a): correctly cflx-side. Writes into `openspec/changes/<id>/`. Cross-check E R2 confirms.
- `cairn check` (Bundle B C3.b): renamed in cross-check.
- `cairn export` (Bundle E): renamed in cross-check.
- `cairn islands` / `cairn neighbourhood --include-orphans` (Bundle A C5.a): correctly cairn-side from the start.

So: every command that emits a category-error rename is a *cairn* command initially mislabelled as *cflx*. Zero counter-examples (no cairn commands mislabelled as cflx). The pattern is one-directional and predictable.

**Decision.** Promote to a top-level principle. Two paths:

1. **Embed the rule in Bundle F.** Add one sentence to the CLAUDE.md positive-form section: *"Naming discipline: framework-data queries (`get`, `neighbourhood`, `lint`, `check`, `scan`, `export`, `islands`) live on the `cairn` binary. Phase-lifecycle operations (`apply`, `accept`, `archive`, `validate`, `trace`) live on `cflx`. The binary distinction encodes the cairn/cflx separation principle."* This makes the principle discoverable without authoring a separate doc.

2. **Add to spec.md §3.5 as a fourth bullet point.** Less appealing because §3.5 is identity, not implementation discipline. Naming is a CLAUDE.md and AGENTS.md concern, not a spec-prose concern.

**Recommended action: option 1.** Bundle F (which is already commit #1) absorbs one extra sentence in the CLAUDE.md positive section and a parallel one-liner in AGENTS.md ("Same binary distinction applies to commands you generate or invoke during apply"). Total marginal cost: ~30 words. **No separate sweep phase is needed** because the bundles themselves caught and corrected the only two violations during cross-check; Bundle F just locks the principle so future authors don't recreate them.

### Pattern 2: Validate-failure fixes inside ADOPT scope

**Findings.** The dispatch named 11 specs as currently failing `cflx validate --strict`. Each cross-check probed its own touch surface:

| Failing spec area | Touched by which bundle? | In-bundle fix recommended? |
|---|---|---|
| `specs/graph-explorer` | Bundle C (C4.b/c, C5.b, C9-salvage all add scenarios) | YES, fix `## Purpose` omission in commit #1, ~100 words. Cross-check C D1. |
| `specs/reconciliation` | Bundle B (Findings panel reads from reconciler), but no spec delta on reconciliation | OPEN, Bundle B's Q1 explicitly asks what the failure is; cross-check B did not reproduce it. Recommend: investigate during Bundle B design.md authoring; if it's a single-section omission like graph-explorer, Bundle B absorbs it; if it's structural, defer to a sweep phase. |
| `specs/artefacts` | Bundle A (no delta, but adjacent); C1.a (no delta) | NO, Cross-check E F5 directly invoked the validator and could not reproduce the failure for artefacts. Confirmed: `cflx.py validate --strict` operates on changes, not living specs. The "11 failing specs" claim in the dispatch may have been per-spec-linter results not per-validator results. **Action: re-run validation, identify which gate the 11 failures came from, and triage from there.** |
| Other 8 failing items | Unknown until re-confirmation | Triage at sweep-phase time. |

**Decision.**

1. **Bundle C absorbs its own validate-fix** (`## Purpose` on graph-explorer). Cheapest, scoped, and the bundle is already touching the file.
2. **Bundle B investigates its own validate-fix during design.md authoring.** If reconciliation's failure is a single omission (Purpose, Requirement-ID, scenario shape), Bundle B absorbs it. If it's structural (re-numbering of requirements, mass scenario rewrites), it gets escalated to a separate sweep phase.
3. **No general "all bundles absorb their own validate-fix" rule.** Bundle A, E, C1.a, F, 7.5c don't touch any failing spec. Forcing them to absorb fixes would either gold-plate the bundle or fork the bundle into a fix-and-feature pair.
4. **A separate `phase-7.x-spec-validate-cleanup` sweep phase remains warranted for the residual.** Cross-check A's open question 1 already requested this. Scope: the 8+ failing specs that no current bundle touches. Estimated size: small (most likely a series of `## Purpose` omissions and frontmatter fixes given F1's pattern). **This sweep phase is not a blocker for any bundle**; it can ship in parallel.

### Pattern 3: Phase-9 absorption

**Findings.** Multiple bundles surface sub-components that explicitly land inside or just-before Phase 9:

- **Bundle A's C8.b** is the safety primitive for **C8.c suggest engine**, which the roadmap-debate plans to live inside Phase 9. Cross-check A D1 confirms: if Phase 9 absorbs C8.c, Bundle A is a hard prerequisite; if not, ordering is flexible but Bundle A is still a prerequisite somewhere.
- **C1.b interview runner** (deferred from C1.a) is Phase 9 brownfield-gated per cross-check E.
- **C8.c suggest engine itself** is Phase 9-gated per roadmap-debate verdict table.
- **C15 templated authoring scaffolds** (raised in roadmap-debate, not in cross-check inputs but consistent with Phase 9's "summariser-assisted naming/tagging").
- **C4.b decision-attached obligations** (cross-check C D4) explicitly defers to "Bundle D or Phase 9" depending on whether decision-stamping schema produces an obligations field.

Reading Phase 9's current proposal (per cross-check A F1): Phase 9 today is **deterministic-extraction-only**. Its summariser usage is bounded to *naming/describing/tagging* extracted candidates (cross-check A's reading of design.md lines 37-38), not *suggesting cross-cutting edges*. Its safety model is *delete-before-archive*, not *triage-before-accept*. Its scope is silent on the suggest engine, on multi-round interviews, on confidence-pills, on templated authoring beyond stub contract content.

**Decision.** Phase 9 brownfield needs a re-scoping pass before its apply runs. Without one, a series of "we said Phase 9 covers this" claims in cross-checks will collide with Phase 9's actually-narrow proposal at apply-time.

**Recommended action: open `phase-9-brownfield-rescope` as a small `cflx-proposal` task before phase-9.0-tests applies.** The rescope absorbs:

- C8.c suggest engine (the AI-edge-suggesting tool that consumes Bundle A's queue format and is gated by C8.b's accept-time safety).
- C1.b interview runner (multi-round elicitation upgrade to the cflx-proposal skill, gated on Phase 9's brownfield-onboarding need).
- C15 templated authoring scaffolds (project-config-driven contract templates beyond the current stub scope).
- C4.b's decision-attached-obligations extension (only if Phase 9's stamping schema introduces an obligations field; conditional, not unconditional).

This rescope is a docs-only change (proposal/design/tasks edits + spec delta updates); it doesn't change Phase 9's implementation start. It just makes the plan honest before code lands.

The rescope effort is **out of scope for any of Bundles F/A/B/C/E/C1.a or phase-7.5c**. It's a separate proposal-authoring task that an orchestrator slots between phase-9.0-tests-apply and phase-9-brownfield-apply.

### Pattern 4: Shared infrastructure that's already there

**Findings.** Cross-checks systematically discovered that "we need to build X first" claims in the roadmap-debate were partially or fully discharged by existing code:

- **Bundle B's "structured-finding-feed dependency"** (Batch A's flagged shared dependency) is already discharged. `Finding { code, severity, message, node, path }` exists in `src/map/graph.rs:29-40`; `/api/lint` already serves it; `cairn lint --json` already renders it. What Bundle B builds is *consumer-side rendering*, not the feed. Cross-check B F1, F2.
- **Bundle B's "centralised copy" dependency** is genuinely new (no `copy.toml`, `strings.json`, etc. exist), but small (one file). Cross-check B F5.
- **Bundle C's "C5.b verb-labelled edges"** is potentially already implemented. The graph-explorer spec already specifies that edge labels render on selection; cross-check C F6 flagged this as a verification step. **Status: Bundle C should verify in commit #1 whether labels already render; if yes, C5.b is a no-op.**
- **Bundle C's "node detail panel"** infrastructure already exists; C4.b is a section *inside* the panel, not a new surface. Cross-check C F2.
- **Bundle E's "JSON serialisation core"** doesn't exist as such, but the cli spec already mandates `--json` mode for every CLI command, so the serialisation patterns are well-established. Cross-check E F8.
- **Bundle A's `cflx-trace` concept** is genuinely new ground (zero current implementation). Cross-check A F4.
- **Bundle A's `suggested-edges` concept** is genuinely new ground. Cross-check A F5.

**Decision.** No further action needed beyond logging this pattern. Each bundle's cross-check correctly shifted scope. The pattern's significance is for orchestrator confidence: **the roadmap-debate's LOC estimates were conservative because they didn't consistently subtract already-built infrastructure.** Bundle B's actual build is smaller than the synthesis projected (consumer-side only); Bundle C's actual build for C5.b may be zero. Bundles A and E confirmed the synthesis estimates by surfacing genuinely new ground.

### Pattern 5: Pre-existing path inconsistency (`meta/changes/` vs `openspec/changes/`)

**Findings.** Cross-check A F3 surfaced this: `openspec/specs/changes/spec.md` and `terminology-rename/spec.md` reference `meta/changes/<id>/`, but the live tree is `openspec/changes/<id>/`. Phase 9's specs inherit the spec-text inconsistency; the live tree is otherwise consistent.

**Decision.** Not in any bundle's scope. Defer to a small terminology-style cleanup phase. **All bundles must use `openspec/changes/` path references in their proposal/design/tasks** (not `meta/changes/`), to match live tree. If/when the inconsistency resolves, no bundle's content has to migrate. Bundle A's R4 already builds in this forward-compatibility for C6 sidecar location.

## Conflicts surfaced and resolved

### Conflict 1: Bundle B's "ship together" recommendation vs Bundle A's "ship in parallel" claim

- **Bundles affected**: A, B.
- **Description**: Cross-check B D1 says Bundle B internally ships sequentially-but-as-one-phase (kernel `Info` first, then copy file, then UI). Cross-check A R7 + open question 4 says Bundle A is parallel-shippable with Bundle B. Are these consistent?
- **Resolution**: **Yes, fully consistent.** Bundle B's "ship together" is *internal commit ordering*; Bundle A's "parallel" is *bundle-vs-bundle*. They operate at different scopes:
  - Within Bundle B: 7 commits in fixed dependency order (Info enum → copy file → README voice update → cairn check → empty states → findings panel → prose-nudge banner).
  - Between Bundle A and Bundle B: zero file overlap. Bundle A touches `specs/cli`, `specs/changes`, `specs/query`, `specs/provenance-foundation` (new), `cflx.py`, error-codes registry. Bundle B touches `specs/cli`, `src/map/graph.rs` (FindingSeverity::Info), `src/ui_assets/`, `docs/design-system/`, `src/cli/format.rs`, `src/cli/mod.rs`. The overlap is `specs/cli/spec.md` (both add a new requirement) and possibly `src/cli/mod.rs` (Bundle A adds `cairn islands`/`cairn neighbourhood --include-orphans`; Bundle B adds `cairn check`).
- **Reasoning**: The `specs/cli` overlap is *additive*; both bundles add new requirements without modifying existing ones. Merge conflicts at apply-time are textual but trivially resolvable. The `src/cli/mod.rs` overlap is also additive (different subcommands). The verification battery passes for both bundles independently. **Confirmed parallel-shippable; no sequencing constraint between A and B.**

### Conflict 2: Bundle 7.5c needs `cairn-macros/` workspace member

- **Bundles affected**: 7.5c, all others.
- **Description**: Cross-check 7.5c D4 requires adding `cairn-macros/` as a new workspace member with `proc-macro = true`. The current `Cargo.toml` declares `members = ["."]`. Does any other bundle assume single-crate workspace?
- **Resolution**: **No conflict.** No other bundle's cross-check touches the workspace structure. Bundle B adds source files within the existing crate (`src/map/`, `src/ui_assets/`). Bundle A adds source files plus a Python addition to `cflx.py`. Bundle C adds JS/CSS in `src/ui_assets/`. Bundle E adds Rust source in `src/cli/` and possibly `src/export/`. C1.a is skill-body only. None of these consume macros from a sibling crate, so adding `cairn-macros/` doesn't force them to depend on the new crate.
- **Reasoning**: Cargo workspaces are additive: adding a new member doesn't break existing members. The only cross-cutting concern would be if another bundle wanted to use the proc-macro from `cairn-macros/` before phase-7.5c lands; cross-check confirms none do (the pre-phase-test files in 8.0/9.0/10.0 are not in any cross-check's bundle scope).

### Conflict 3: Bundle B adds `FindingSeverity::Info` variant

- **Bundles affected**: B, A (Bundle A also reads findings via `query::lint`).
- **Description**: Cross-check B D5 R3 adds a third variant `Info` to the `FindingSeverity` enum. ~15 match sites need updating. Does Bundle A have a match site?
- **Resolution**: **Light conflict, fully resolvable.** Cross-check A doesn't grep for `FindingSeverity` matches because A's surfaces (`cflx trace`, `suggested-edges`, islands query) don't read severity. A's C5.a islands query is graph-traversal over nodes, not findings. A's C8.a/b queueing is over a separate file class, not the existing finding stream. **Bundle A has no `match FindingSeverity` site to update.**
- **Reasoning**: Bundle B's commit #1 (the `Info` variant addition) modifies all current match sites; if Bundle A merges to dev before Bundle B, B's commit #1 will rebase cleanly because A added zero match sites. If Bundle B merges first, A merges cleanly because A's surfaces don't touch the enum. **No actual ordering constraint between A and B from this enum change.**

### Conflict 4: C1.a writes to `.claude/skills/cflx-proposal/SKILL.md`

- **Bundles affected**: C1.a, all skill-touching bundles.
- **Description**: C1.a edits the cflx-proposal skill body. Does any other bundle modify cflx skills?
- **Resolution**: **No conflict.** Searching the cross-checks: F edits CLAUDE.md and spec.md (no skill); 7.5c edits AGENTS.md and conventions.md (no skill); A edits cflx.py and Rust source (no skill); B edits design-system, ui_assets, format.rs, cli/mod.rs (no skill); C edits ui_assets only (no skill); E edits cli/mod.rs and adds an export module (no skill). Only C1.a touches `.claude/skills/cflx-proposal/SKILL.md`. **C1.a is the sole consumer of that file in this adoption wave.**
- **Reasoning**: Future phases may want to extend the cflx-proposal skill (for example, the C1.b interview runner inside the Phase 9 rescope). But for the current six bundles + 7.5c + Phase 9 baseline, C1.a is alone.

### Conflict 5: Multiple bundles propose adding `## Purpose` or `cairn-X` rename → shared-precedent question

- **Bundles affected**: B, C, E (and Pattern 1 generalises this).
- **Description**: Bundle B renames `cflx check` → `cairn check`. Bundle E renames `cflx export` → `cairn export`. Bundle C adds `## Purpose` to graph-explorer. Should one shared-precedent commit land first to avoid three separate rename/section-add passes?
- **Resolution**: **Bundle F absorbs the cairn-vs-cflx naming principle (Pattern 1 decision). The `## Purpose` addition is a separate concern.**
  - **Cairn-vs-cflx naming**: Bundle F adds the principle to CLAUDE.md and AGENTS.md as commit #1. Bundles B and E then ship their renames already aligned with the principle. No precedent-setting commit beyond F is needed because the renames are *applications* of the principle, not redefinitions of it.
  - **`## Purpose` addition**: Only Bundle C touches a spec where this is currently missing (per cross-check C F1). Bundle B's potential reconciliation-spec validate-fix (Pattern 2) may also need a `## Purpose` add but that's contingent on what the failure actually is. **Bundle C's commit #1 fixing graph-explorer's missing `## Purpose` establishes the precedent**; if Bundle B's reconciliation fix turns out to be the same shape, it follows the same pattern. **No precedent-only commit is warranted** because the precedent is lightweight (one section addition) and each bundle that needs it absorbs it cheaply.
- **Reasoning**: A pre-bundle "cosmetic sweep" commit would force a separate review cycle for ~200 words of prose changes, which is wasteful. The cleaner discipline is: **bundle absorbs its own validate-fix when it touches the spec; integrator decision applies the rule consistently.**

## Final sequencing recommendation

Hard constraints (must-precede):

1. **Bundle F → everything else.** F adds the cairn-vs-cflx principle that B and E rename against. F is also commit #1 of the entire adoption per the original roadmap-debate ordering. (Predecessor: none.)

2. **phase-7.5c → phase-8.0-tests apply.** 7.5c defines `#[cflx_planned]` which 8.0-tests' test stubs consume. If 8.0-tests applies first, every test stub needs a retroactive rewrite. (Predecessor: F, soft. Successor: 8.0-tests apply.)

3. **Bundle A → C8.c suggest engine** (which lives inside Phase 9 rescope). A's C8.b is the safety gate for C8.c's outputs. (Predecessor: F. Successor: Phase 9 apply, hard.)

4. **Phase 9 rescope → phase-9.0-tests apply.** The rescope reshapes Phase 9's proposal; phase-9.0-tests' stubs must reflect the rescoped Phase 9. (Predecessor: A's apply, indirect. Successor: phase-9.0-tests apply, hard.)

Soft constraints (preferred ordering, no blocking):

5. **Bundle F → phase-7.5c.** F provides the cluster-observation framing that 7.5c's "verification states are at the bottom layer of cairn's deterministic-typed enforcement" naturally references. Not a hard dep.

6. **Bundle C → after Bundle B's `Info` enum lands.** Bundle C might surface findings on the inspector chrome; consuming three severity buckets requires the variant. If Bundle C ships first, its inspector chrome works with two buckets. **No hard dep; ordering is preference.**

7. **Bundle E and C1.a are lifecycle-orthogonal.** They can ship at any point after Bundle F. Recommended: ship C1.a first (smaller, no code), Bundle E after (slightly larger, real Rust). Both can land in parallel with the larger phase work.

8. **Spec-validate cleanup sweep phase** (Pattern 2): can ship in parallel with anything. Recommended: after Bundle B's design.md determines whether reconciliation's failure absorbs into B or escapes to the sweep.

Final ordered plan (with parallelism):

```
Wave 0:  Bundle F                               [commit #1; ~365 words]
Wave 1:  phase-7.5c                             [~700 LOC; new crate]
Wave 2:  Bundle A    ||  Bundle B    ||  C1.a   [parallel; A and B independent; C1.a tiny]
Wave 3:  Bundle C    ||  Bundle E    ||  spec-validate-cleanup-sweep
                                                [parallel; C after B preferred]
Wave 4:  Phase 9 rescope                        [proposal-authoring task]
Wave 5:  phase-8.0-tests apply
         phase-8-summariser apply
         phase-9.0-tests apply
         phase-9-brownfield apply (re-scoped)
         phase-10.0-tests apply
         phase-10-distribution apply
                                                [existing roadmap; unchanged in shape]
```

Wave 5 is the existing pre-test/feature pipeline; the integration above does not modify its sequencing beyond Phase 9's rescope.

## What ships first (the actual concrete commit/phase plan)

This is the file-level commit plan for the next session to pick up directly.

### Commit #1: Bundle F (single commit, two files, ~365 words)

**File 1**: `/Users/george/repos/cairn/docs/spec.md`

Insert new section §3.5 between current line 92 and line 94. Header: `### 3.5 Layer ordering of enforcement, configuration, and AI`. Body per cross-check F's "Recommended Bundle F final scope" (the three-layer principle with C9/C10/C14 expansions, including cross-check F's R3 propose-vs-approve sentence on C14 and R2 wording tweak on C10).

**File 2**: `/Users/george/repos/cairn/CLAUDE.md`

Insert new section between current line 76 (end of `Workflow: cflx (Conflux)`) and line 78 (start of `What to avoid`). Header: `## What cairn is, positively`. Body per cross-check F's Recommended scope (three numbered principles), plus one new sentence (per Pattern 1 decision):

> **Naming discipline.** Framework-data queries (`get`, `neighbourhood`, `lint`, `check`, `scan`, `export`, `islands`) live on the `cairn` binary. Phase-lifecycle operations (`apply`, `accept`, `archive`, `validate`, `trace`) live on `cflx`. The binary distinction encodes the cairn/cflx separation principle.

**File 3** (new addition during integration): `/Users/george/repos/cairn/AGENTS.md`

Add a one-line note in the existing operational section: *"Same binary distinction applies to commands you generate or invoke during apply: `cairn` for framework-data queries, `cflx` for phase-lifecycle operations."* Total AGENTS.md delta: 1 line.

**Commit message**: `docs: add §3.5 layer ordering and cairn/cflx binary discipline`. No code, no tests, no validate gate interaction.

### Commit #2 onwards: phase-7.5c proposal authoring

Run `/cflx-proposal` (or equivalent skill invocation) with the locked scope from cross-check 7.5c's "Recommended phase-7.5c final scope":

- New phase ID: `phase-7.5c-verification-states`.
- New workspace member: `cairn-macros/` with `proc-macro = true`.
- Spec delta: `openspec/specs/testing-baseline/spec.md` requirement-update.
- Conventions update: `openspec/conventions.md` §5 prose change.
- AGENTS.md update: line 25 plus a one-line note about structured attribute (per cross-check 7.5c R7).
- Error code allocation: `CC001` in C category.

Phase scaffold in `openspec/changes/phase-7.5c-verification-states/{proposal,design,tasks}.md` plus `specs/testing-baseline/spec.md` delta.

### Commit #3 onwards: Bundle A and Bundle B parallel proposal-authoring

Two independent `/cflx-proposal` invocations, parallel branches:

**Bundle A**: `phase-7.6-ai-provenance-foundation` per cross-check A's "Recommended Bundle A final scope" table. New capability area `specs/provenance-foundation/spec.md` (or `specs/cflx-trace/spec.md`).

**Bundle B**: `phase-7.7-ux-foundation` per cross-check B's final scope. Sub-component sequencing internal to the bundle is fixed at 7 commits per cross-check B D1.

### Commit-level: C1.a (lightweight, can land anywhere after F)

Single commit editing `.claude/skills/cflx-proposal/SKILL.md` (adds new step ~7.5 between Spec Deltas and Validate Proposal) plus a one-line addition to `openspec/conventions.md`. Per cross-check E's "C1.a final scope". No phase scaffold, no spec delta required.

### Commit-level: Bundle E (after F, lifecycle-orthogonal)

Single commit (or atomic-grouped pair) on the cairn Rust binary. Adds `cairn export --format <json|md> --output <path>`. One ADDED-Requirements delta to `openspec/specs/cli/spec.md`. JSON-first per cross-check E D4.

### Commit-level: Bundle C (after Bundle B preferred)

Phase ID `phase-7.x-graph-explorer-followups` (whichever number is free). Per cross-check C's recommended scope: 4 sub-components + `## Purpose` fix on graph-explorer + UI Maintenance Contract preservation acceptance line. C9-salvage declared optional in-bundle rider.

### Commit-level: Phase 9 rescope (proposal-authoring task only)

After Bundle A's apply lands, before phase-9.0-tests applies, run `/cflx-proposal` to *update* the existing phase-9-brownfield proposal/design/tasks/specs. Scope absorption per Pattern 3: C8.c suggest engine, C1.b interview runner, C15 templated authoring, C4.b decision-attached obligations follow-on. This is a proposal-update commit, not a new phase scaffold.

### Commit-level: Spec-validate cleanup sweep (parallel)

Single small phase: `phase-7.x-spec-validate-cleanup`. Scope: re-run `cflx.py validate --strict` against all `openspec/specs/<area>/spec.md`, capture actual failure modes, and fix per pattern (likely a wave of `## Purpose` omissions). Ships parallel to anything.

## Open questions for next session (consolidated)

Aggregated from the six cross-checks, deduplicated, ranked by what they unblock:

### High priority (block bundle apply or design.md authoring)

1. **What is the actual reconciliation spec validate failure?** [Bundle B Q1.] Unblocks Bundle B design.md decision: absorb fix in-bundle or escalate to sweep phase.
2. **Does `app.js` already render edge labels on selection?** [Bundle C Q1.] Determines whether C5.b is no-op or 30-50-LOC defect-fix. Verify via browser test or keyword sweep early in Bundle C apply.
3. **Genesis.md lifecycle on apply.** [C1.a Q1.] Does the apply-stage codex re-point genesis research's `nodes` at actual touched nodes? Affects AGENTS.md and `cflx.py archive`. Recommended: leave it; archive carries it through.
4. **Does the Phase 9 rescope happen before phase-9.0-tests applies?** [Pattern 3.] If yes, the rescope is the next step after Bundle A. If no, phase-9.0-tests' test stubs may not reflect the rescoped scope.

### Medium priority (refine in design.md, not blocking)

5. **JSON vs TOML for trace sidecar.** [Bundle A Q5.] Recommended JSON because programmatic consumers dominate.
6. **JSON schema versioning for `cairn export`.** [Bundle E Q35.] Recommended `schema_version: 1` flat shape per E R5.
7. **Default destination for `cairn export`.** [Bundle E Q34.] Recommended: no default; `--output` required in v1.
8. **Genesis.md content scope.** [C1.a Q3.] Recommended: user-visible Q/A turns + final premise; skip system prompts.
9. **MD format verbosity bound.** [Bundle E Q4.] Recommended: always emit everything; truncation is the consumer's job.
10. **`cairn check --scope` flag.** [Bundle B Q5.] Recommended: yes, for parity with webui scope toggle.
11. **Empty-state component icon source.** [Bundle B Q3.] Recommended: glyph reuse for first ship; custom illustrations later.
12. **CLI empty-state coverage breadth.** [Bundle B Q4.] Recommended: zero-data states only; not error-path messages.
13. **Inspector chrome refactor structure.** [Bundle C Q3.] Implementation-detail; resolve in design.md.
14. **`#[cflx_planned]` argument variants.** [7.5c O1.] Recommended: `phase = N` only in v1; defer alternatives.
15. **Phase-7.5c spec delta home.** [7.5c O2.] Recommended: testing-baseline spec.

### Lower priority (defer or note for later)

16. **Should §3.5 section title be "Layer ordering of enforcement, configuration, and AI" or tighter?** [F OQ1.] Recommended: roadmap-debate title for now.
17. **Cluster-observation principle in AGENTS.md.** [F OQ3.] Defer to a follow-on AGENTS.md sync commit.
18. **Whether C5.a verb-edge display needs spec change.** [Bundle A R6.] Recommended: no spec delta; commit rider.
19. **`meta/changes/` vs `openspec/changes/` reconciliation.** [Pattern 5.] Defer to small terminology-style cleanup phase.
20. **Webui write-surface direction.** [Bundle B D2.] Open stronghold-level investigation; not in any bundle's scope; recommended to open `docs/strongholds/webui-direction.md` separately.
21. **Decision-attached obligations for C4.b widget.** [Bundle C D4 + Q4.] Defer to Phase 9 rescope; conditional on stamping schema.
22. **`cairn export --scope` flag.** [Bundle E Q2.] Defer; YAGNI until consumer demand.
23. **Templated prose-nudge banner copy.** [Bundle B Q2.] Recommended templated; resolve in design.md.
24. **MCP tool registration for `cflx trace`.** [Bundle A Q6.] Resolve in design.md.
25. **Whether genesis.md and chat `Premise / Context` both ship.** [C1.a Q5.] Recommended: keep both.
26. **`cairn export --format md` vs `cairn map`.** [C1.a Q6 / Bundle E R8.] Recommended: separate artefact; flag for revisit if `map` capability spec grows a generator.

## Decisions ratified at the integrator level

These are decisions no individual cross-check could ratify alone, because they require synthesis across all six.

### Integrator decision 1: Cairn-vs-cflx binary discipline becomes a Bundle F principle

Pattern 1 establishes the rule. Bundle F absorbs it as one extra sentence in CLAUDE.md positive-form section plus a parallel one-liner in AGENTS.md. No separate sweep phase; no precedent commit. The principle propagates naturally into Bundles B and E's renames.

### Integrator decision 2: Validate-fixes are absorbed by the bundle that touches the spec; residual escapes to a sweep phase

Pattern 2 establishes the discipline. Bundle C absorbs `## Purpose` on graph-explorer (commit #1, ~100 words). Bundle B investigates reconciliation's failure during design.md; absorbs if shaped like a single omission, escalates if structural. Other bundles don't touch failing specs and don't absorb anything. A separate `phase-7.x-spec-validate-cleanup` sweep phase handles the residual; ships parallel to anything; not a bundle blocker.

### Integrator decision 3: Phase 9 brownfield gets a re-scoping pass before its apply runs

Pattern 3 establishes that multiple bundles are quietly assuming Phase 9 contains things its current proposal doesn't list (C8.c suggest engine, C1.b interview runner, C15 templated authoring, C4.b obligations). The rescope is a docs-only proposal-update commit, slotted between Bundle A's apply and phase-9.0-tests' apply. Phase 9's implementation start is unchanged; its plan becomes honest before code lands.

### Integrator decision 4: Bundles A and B ship in parallel; no enum-ordering constraint between them

Conflict 3 resolution: Bundle A has zero `match FindingSeverity` sites. The `Info` variant addition is contained inside Bundle B. Either bundle merging first leaves the other rebasing cleanly. Confirmed parallel-shippable per cross-check A R7's request that this be confirmed at integrator level.

### Integrator decision 5: All bundles use `openspec/changes/` path references in their proposal/design/tasks/specs

Per Pattern 5. Live tree is `openspec/changes/`; spec text in `specs/changes/spec.md` and `specs/terminology-rename/spec.md` is `meta/changes/`. The eventual reconciliation is out of bundle scope. Bundles must align with live tree to avoid retro-fixes.

### Integrator decision 6: The `## Purpose` precedent is set by Bundle C, not by a precedent-only commit

Conflict 5 resolution. Bundle C's commit #1 (graph-explorer Purpose addition, ~100 words) establishes the pattern. If Bundle B later finds reconciliation needs the same fix, it follows Bundle C's pattern. No standalone "sweep all specs for Purpose" commit because the precedent is lightweight enough that each consumer absorbs it.

### Integrator decision 7: The 11-failing-validate-spec count needs reconfirmation

Cross-check E F5 directly invoked `python3 .claude/skills/cflx-proposal/scripts/cflx.py validate --strict` and could not reproduce per-spec failures (the validator only operates on changes). The "11 failing specs" number from the original dispatch may have been from a different gate (an OpenSpec linter? a stale state?). **Action item for next session**: re-run the actual validate gate, confirm what fails, redirect cleanup-sweep scope accordingly. Until reconfirmed, treat the "11 failing" claim as a working hypothesis, not a hard count.

### Integrator decision 8: Cross-check 7.5c's `cairn-macros/` workspace addition is the only new crate added by this adoption wave

No other bundle introduces a workspace member. The single-crate-to-multi-crate transition happens in 7.5c and stays at two crates (cairn + cairn-macros) through Wave 5. Conventions §3 module-size pressure can absorb this.

### Integrator decision 9: Sequencing order

Wave 0 → Wave 1 → (Wave 2 parallel) → (Wave 3 parallel) → Wave 4 → Wave 5. Hard constraints: F before everything; 7.5c before 8.0-tests apply; A before C8.c; Phase 9 rescope before 9.0-tests apply. Soft preferences: F before 7.5c; B before C; C1.a and E lifecycle-orthogonal. The full plan above (`Final sequencing recommendation`) is the canonical wave map.

### Integrator decision 10: Bundle F's commit #1 is also the commit that locks the cross-check integration ratification

The integrator is shipping these decisions by ratifying Bundle F as commit #1. Once Bundle F lands with the binary-distinction principle in CLAUDE.md, all subsequent bundles ship inside the principle's authority. There is no separate "ratification commit" beyond F because F itself encodes the cross-cutting decisions that this integration produced.

---

**End of integration.** This document is the next session's start point. Open it, ship Bundle F as commit #1, then run `/cflx-proposal` for phase-7.5c. Everything else follows the wave map.
