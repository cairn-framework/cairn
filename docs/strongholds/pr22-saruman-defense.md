# PR #22 Saruman defense (Wave 4 rescope vs Sauron critique)

**Subject**: `openspec/phase-9-brownfield-wave4-rescope`, commit `d2a3ee7`
**Defender**: Saruman the White, in answer to the Lidless Eye
**Date**: 2026-05-01

This document is the orchestrator-facing assessment. The rhetorical banter is filed in the round transcript; this file is the truth-shaped record.

---

## CRITICAL #1 — Output-path schism between `meta/changes/` and `openspec/changes/`

**Sauron's claim**: `proposal.md:22` (and the suggest-engine queue references) point at `openspec/changes/`, while `design.md:46,53` and `specs/brownfield/spec.md:27,35,42` pin `meta/changes/brownfield-init/`. The rescope contradicts itself on the most basic question: where brownfield output lands.

**Verdict: CONCEDED.**

**Evidence**.
- `proposal.md:22`: `cairn init --from-code` writes to `openspec/changes/brownfield-init/`. Confirmed by file read.
- `proposal.md:34` (suggest engine output): `openspec/changes/<change>/suggested-edges.json`. Confirmed.
- `design.md:47`: "`cairn init --from-code` SHALL create `meta/changes/brownfield-init/`". Confirmed.
- `design.md:54`: "`meta/changes/brownfield-init/` already exists unless `--force`". Confirmed.
- `design.md:80`: queue file at `openspec/changes/<change>/suggested-edges.json`. Confirmed.
- `specs/brownfield/spec.md:27,35,42,45`: scenarios pin `meta/changes/brownfield-init/`. Confirmed.
- Integrator decision 5 (`getcairn-cross-check-integrated.md:315-317`): "All bundles must use `openspec/changes/` path references in their proposal/design/tasks/specs ... Bundles must align with live tree to avoid retro-fixes." This is normative for Wave 4.
- Bundle A's design.md L82: queue file lives at `openspec/changes/<change>/suggested-edges.json` because "the path matches the live tree (which uses `openspec/changes/`, F3)".
- The rescope's own genesis.md frontmatter explicitly informs from `getcairn-cross-check-integrated.md` and `phase-7.6-ai-provenance-foundation/design.md`. Both fix the live-tree path as `openspec/changes/`.

The schism is real, not a misreading of distinct concepts. `init` output dir, queue-file dir, and scenario-asserted dir all should be the same `openspec/changes/<change>/` per integrator decision 5. The rescope inherited the pre-rescope phase-9 spec's `meta/changes/` references (those references existed in Phase 9's spec at HEAD pre-rescope) but did NOT update them to match `openspec/changes/` even though Wave 4 was the natural opportunity and integrator decision 5 made it mandatory.

**Required fix**. Single-path normalisation in this PR:
- `design.md:47, 54`: change `meta/changes/brownfield-init/` to `openspec/changes/brownfield-init/`.
- `specs/brownfield/spec.md:27, 35, 42, 45`: same substitution.
- `proposal.md:41` (acceptance criterion "never writes directly to main `cairn.blueprint` or main `meta/` artefacts"): retain "main `meta/`" carve-out only if `meta/` continues to exist as the artefact tree; otherwise drop the carve-out and replace with "main `openspec/specs/` artefacts" or the actual reconciler's truth path.

This is the strongest of Sauron's findings and not defensible. It directly violates the integrator decision the rescope cites in its own genesis. The "F3 cleanup deferred" defence does not stand because integrator decision 5 explicitly closed that escape hatch ("All bundles must use `openspec/changes/`").

---

## CRITICAL #2 — phase-9.0-tests is structurally stale

**Sauron's claim**: phase-9.0-tests still says "ten acceptance-criterion scenarios across four requirements" but the rescoped phase-9 now has 23 scenarios across 8 requirements. 13 new scenarios have no failing-test counterpart. The timing stronghold's Option-B verdict required "Phase 9 rescope before phase-9.0-tests apply" and this PR did not update phase-9.0-tests.

**Verdict: PARTIAL — Sauron right that desync exists, wrong that this PR was required to fix it.**

**Evidence**.
- `openspec/changes/phase-9.0-tests/proposal.md:13` (read on the rescope branch): still says "ten acceptance-criterion scenarios across four requirements." Confirmed.
- Rescoped spec scenario count: 23 across 8 requirements. Confirmed by file read of `specs/brownfield/spec.md` (counted: 5+2+1+2 original = 10, plus 4+3+3+3 added = 13, total 23 across 8). Sauron's count is exact.
- Timing stronghold (`oq4-phase9-rescope-timing.md`): the verdict at L83 says "Option B with one tightening: Rescope AFTER Bundle A's design.md ratifies the deferred identifiers." The implementation-specifics block at L102-140 lays out the chronological sequence:
  - T+? Rescope merges.
  - **T+8 (Wave 5 begins after all of Wave 0-4)**: phase-9.0-tests applies.
  - L132 (T+7, Wave 3): "phase-9.0-tests' proposal authoring (if not already done) authors against the rescoped Phase 9."
  - L140: "T+8 trigger: phase-9.0-tests apply must run after the rescope merge, not before."
- Integrated cross-check `sequencing rule 4` (L161): "Phase 9 rescope -> phase-9.0-tests apply. The rescope reshapes Phase 9's proposal; phase-9.0-tests' stubs must reflect the rescoped Phase 9."

The stronghold sequence is: (a) rescope merges, then (b) phase-9.0-tests' proposal is re-authored or initially authored against the rescoped scope, then (c) phase-9.0-tests applies. The stronghold does NOT say the rescope-PR itself must contain the phase-9.0-tests update; it says phase-9.0-tests' proposal-authoring must happen between rescope merge and phase-9.0-tests apply. Sauron is treating these as one commit; the stronghold treats them as two.

However: phase-9.0-tests' proposal already exists on dev (it predates the rescope). The stale text "ten ... four" lives there now, on the rescope branch and on dev. So while Sauron's claim that the rescope-PR is "incomplete" is technically too strong, the broader observation — that there is a stale proposal in the tree right now — is correct, and shipping the rescope without updating phase-9.0-tests means dev will carry a self-contradicting state until the next phase-9.0-tests proposal-update commit.

The right resolution is one of:
1. Add a sibling commit in this PR that updates phase-9.0-tests' proposal.md and (placeholder) spec to match 23 scenarios across 8 requirements. This is what Sauron requests and is the cleaner posture.
2. Land the rescope as-is with an explicit out-of-scope entry in proposal.md ("phase-9.0-tests proposal update is a follow-on commit; this rescope only updates phase-9-brownfield's plan") and open a tracking task for the follow-on.

Option 1 is preferable because it maintains the project's "declarative state should not lie about the next phase's planned scope" load-bearing constraint (cited in the timing stronghold L43). Option 2 is acceptable but leaves a known stale file on dev.

**Required fix** (recommended): bundle a phase-9.0-tests proposal-update commit into this PR or a stacked PR before merge. The phase-9.0-tests spec scenarios need not be 1:1 with phase-9-brownfield's (the test wall is allowed to assert subsets and invariants), but the prose count "ten ... four" must match reality after rescope.

---

## CRITICAL #3 — "PR #18 merged Phase 7.6 in full" is false premise

**Sauron's claim**: `proposal.md:30` and `research/genesis.md:16` claim Phase 7.6 "merged on PR #18" / "merged in full". But PR #18 was a proposal-draft commit only. `openspec/specs/provenance-foundation/` does not exist on dev. `CC002` is not in the error registry. The suggest-edges queue file class is unimplemented. Acceptance criterion `proposal.md:47` references unallocated `CC002`.

**Verdict: CONCEDED on the prose; PARTIAL on the consequence.**

**Evidence**.
- PR #18 (commit `d7797b1`) file list (verified via `git show`): seven proposal/design/tasks/specs files under `openspec/changes/phase-7.6-ai-provenance-foundation/`. Zero source files under `src/`. Zero changes under `openspec/specs/` (consolidated specs). Zero registry edits to `openspec/registries/error-codes.md`.
- `git ls-tree -r dev -- openspec/specs/provenance-foundation`: empty (capability area absent on dev). Confirmed.
- `openspec/registries/error-codes.md` on dev: section "## CC -- Changes" exists but contains no `CC001` or `CC002` entry. Sauron is correct that neither is registered.
- The rescope's proposal.md L30 says: "Phase 7.6 (Bundle A) merged on PR #18; its design.md ratified the identifiers".
- The rescope's research/genesis.md L16 says: "Phase-7.6 has merged in full (PR #18) so this question is moot for the current rescope; the design.md identifiers are stable."

The prose conflates two distinct cflx lifecycle stages: (a) a proposal-draft commit that lands a `phase-7.6/{proposal,design,tasks,specs}.md` set, and (b) a phase apply that lands implementation in `src/` and registry mutations. PR #18 is only (a). The genesis.md "merged in full" phrasing is factually wrong.

**However**, on the consequence:
- The timing stronghold's Option B refined verdict (L83) is explicit that the trigger is "design.md ratifies the deferred identifiers", NOT the apply. The rescope unblocks once phase-7.6's design.md fixes the file format, capability area name, error code symbol, and sidecar path. PR #18's design.md does fix all four. The rescope is therefore correctly timed even though its prose mis-describes the trigger.
- The acceptance criterion `proposal.md:47` referencing `CC002` is forward-compatible: this rescope is a docs-only proposal-update commit; the criterion will be checked at phase-9-brownfield's apply time, by which point Bundle A's apply must have run (per the integrated plan's sequencing rules and the rescope's own dependency declaration on L7-8). The rescope does not need `CC002` allocated to be valid; it needs `CC002` allocated to apply. These are different gates.
- The dependency declaration on `proposal.md:7-10` says `phase-7.6-ai-provenance-foundation` is a required dependency and execution must run after phase-7.6. Per cflx vocabulary (CLAUDE.md "Phases execute via `cflx`. Lifecycle: apply -> accept -> archive."), "execution" refers to the apply stage. The dependency text is correct.

**Required fix** (prose-only):
- `proposal.md:30`: replace "Phase 7.6 (Bundle A) merged on PR #18; its design.md ratified the identifiers" with "Phase 7.6 (Bundle A)'s proposal-draft merged on PR #18; its design.md ratifies the identifiers (the apply stage will register `CC002` and ship the queue file class before phase-9-brownfield apply runs)."
- `research/genesis.md:16`: replace "Phase-7.6 has merged in full (PR #18)" with "Phase-7.6's design.md has been authored and merged on PR #18, ratifying the identifiers; full apply is a downstream prerequisite that runs before this phase's apply."
- Optionally, sharpen `proposal.md:8` dependency line to "phase-7.6-ai-provenance-foundation apply must precede phase-9-brownfield apply" (Sauron's #12 covers this).

This is a documentation-correctness fix, not an architectural fix. The rescope's structural correctness is preserved.

---

## SUBSTANTIVE #4 — Genesis cites `docs/strongholds/session-handoff.md` which is not committed

**Sauron's claim**: `research/genesis.md:24` cites `docs/strongholds/session-handoff.md` as the mission brief. That file is untracked on disk and not committed.

**Verdict: CONCEDED.**

**Evidence**.
- `git status` (per the snapshot at session start): `?? docs/strongholds/session-handoff.md` — untracked.
- The genesis transcript's authority for the entire rescope cites a file that is not part of the PR.
- Per `openspec/conventions.md` §9, the genesis transcript "provides human-readable and codex-readable provenance for the change's elicitation history." Citing a non-committed file breaks the provenance chain at its source.

**Required fix**. Either commit `docs/strongholds/session-handoff.md` as part of this PR (making the citation honest) or rewrite the genesis transcript prose to cite only the live, committed strongholds (`docs/strongholds/getcairn-cross-check-integrated.md`, `docs/strongholds/oq4-phase9-rescope-timing.md`). The committed strongholds already informed the rescope per the genesis frontmatter `informed_by` block, so the transcript can stand on those alone.

Sauron's secondary point (label the Q/A as "synthesised decision record" rather than "transcript") is a defensible stylistic improvement and worth doing.

---

## SUBSTANTIVE #5 — Acceptance Criteria carry subjective verbs

**Sauron's claim**: `proposal.md:42, 47-50` use unfalsifiable language ("obvious", "without losing", "reviewable").

**Verdict: PARTIAL — three of five criticisms stand, two are nitpicks.**

**Evidence**.
- L42 "obvious edges": this is original phase-9 wording, not Wave 4 introduction. The design.md at L37-38 has the mechanical threshold (>=2 import observations or 1 high-confidence public API reference). Sauron is right that "obvious" is fuzzy; the mechanical predicate exists in design.md and the acceptance criterion can be tightened to point at it. **Worth fixing, low blast radius.**
- L48 "without losing earlier turns": the spec scenario at `specs/brownfield/spec.md:132-137` ("Session persists across invocations") makes the predicate mechanical: "resumes at the next outstanding turn rather than restarting." The acceptance criterion summarises that scenario and is intentionally less mechanical because it is a summary. The mechanical check lives in the scenario. **Defensible as-is; redundancy with scenario is acceptable.** Optional tightening: rephrase to "resumes from the persisted session state at the next outstanding turn."
- L49 "templated authoring resolves ... falls back": this is a faithful summary of `specs/brownfield/spec.md:158-177` which contains three mechanical scenarios. Same pattern as L48. **Defensible.**
- L50 "reviewable": the spec scenarios at `specs/brownfield/spec.md:190-195` make this mechanical ("the obligations field is editable and removable; archive applies only the human-reviewed obligations"). The summary criterion is less mechanical. **Defensible.** Slight prose improvement: replace "reviewable" with "editable and removable before archive."
- L41 "main meta/ artefacts" — see CRITICAL #1. The carve-out is doing implicit work that the criterion does not state. After path normalisation per CRITICAL #1, this criterion needs revisiting.

Net: only L42 ("obvious") needs prose tightening. The others are summaries pointing at mechanical scenarios and are acceptable in proposal.md's voice. Sauron is over-strict on the proposal.md vs spec.md split: proposal.md is the elevator pitch, spec.md is the predicate. They have different jobs.

**Required fix**: tighten L42 "obvious" to cite the design.md threshold. L48-50 may stay as proposal-voice summaries.

---

## SUBSTANTIVE #6 — ADDED-vs-MODIFIED Requirements miscategorisation

**Sauron's claim**: The new "Suggest cross-cutting edges" requirement is a strengthening of the existing "Keep human review authoritative" requirement. Per OpenSpec conventions, narrowing an abstract requirement should produce a `## MODIFIED Requirements` entry, not a parallel ADDED. Also missing: a force-vs-pending-queue interaction scenario.

**Verdict: PARTIAL — the MODIFIED claim is debatable; the missing force-vs-pending scenario is a real gap.**

**Evidence**.
- The existing "Keep human review authoritative" requirement at `specs/brownfield/spec.md:66-75` is an abstract assertion ("Brownfield output SHALL remain proposed until archived") with one scenario ("False positive can be removed").
- The new "Suggest cross-cutting edges through the phase 7.6 queue" requirement at `specs/brownfield/spec.md:95-127` introduces a gate (`CC002` blocks archive on pending). This is a new, narrower mechanism for the same abstract goal but applied to a new artefact (the queue file).
- OpenSpec convention rules on ADDED vs MODIFIED do not appear in `openspec/conventions.md` §1-9 explicitly (I scanned for "ADDED" and "MODIFIED" tokens; conventions §3 covers state versioning, not delta categorisation). The convention is implicit in `openspec/specs/changes/spec.md` and the `cflx.py validate` rules. Without explicit convention text, "narrowing should be MODIFIED" is Sauron's interpretation, not a documented rule.
- Pragmatic test: does the new requirement override or extend? It extends to a new file class (the queue) and a new gate (`CC002`). It does not narrow the existing requirement's scenario ("False positive can be removed" still holds). The two requirements operate on different artefacts (generated change directory contents vs queue-file entries).

**Defence**: pure ADDED is structurally defensible. The new requirement adds new behaviour over a new file class; the existing requirement is unchanged. If `cflx.py validate --strict` accepts pure ADDED here (it does, per the verification block in the genesis), the categorisation passes the project's mechanical gate.

**However**: the force-vs-pending interaction (Sauron's second point) IS a real gap.
- `specs/brownfield/spec.md:40-45` Force scenario says force "replaces the existing generated brownfield change directory atomically."
- If the suggest engine has emitted pending entries and `--force` re-init wipes them, the human triage queue is destroyed silently. The interaction is genuinely unspecified.

**Required fix**. Add a force-vs-pending-queue scenario to either the "Generate initial Cairn state from code" requirement or the new "Suggest cross-cutting edges through the phase 7.6 queue" requirement. Recommended phrasing:
- GIVEN a generated brownfield change containing pending entries in `suggested-edges.json`
- WHEN the user runs `cairn init --from-code --force`
- THEN Cairn refuses to overwrite OR warns and preserves the queue file OR documents the wipe explicitly (apply-time decision)

The ADDED-vs-MODIFIED decision can stand. Add the missing scenario.

---

## SUBSTANTIVE #7 — C4.b conditional guard syntax inconsistency

**Sauron's claim**: `design.md:122` says scenarios use a "WHEN obligations field is present" guard; spec scenarios use parallel-scenarios approach instead. Reader of design.md will look for guard syntax that doesn't exist in spec.

**Verdict: CONCEDED, low severity.**

**Evidence**.
- `design.md:122`: "The conditionality is captured in the spec deltas with a 'WHEN obligations field is present' guard so dormant scenarios do not regress validate-strict."
- `specs/brownfield/spec.md:183, 197`: the spec uses parallel scenarios with GIVEN clauses ("decision artefacts in this phase declare an `obligations` field" / "decision artefacts in this phase do not declare an `obligations` field") rather than a "WHEN guard" syntax.

The parallel-scenarios approach is functionally equivalent and works for `cflx.py validate --strict`. The design.md prose is just imprecise about the mechanism.

**Required fix**. One-line edit at design.md L121-122: "The conditionality is captured in the spec deltas with parallel scenarios for the field-present and field-absent cases, so dormant scenarios do not regress validate-strict." Sauron's secondary suggestion to name the apply-time decision-maker for the conditional is also worth a short sentence.

---

## SUBSTANTIVE #8 — "Manual-test entries leave provenance empty" scenario duplicates phase-7.6 invariant

**Sauron's claim**: `specs/brownfield/spec.md:121-127` asserts a property of phase-7.6's `SuggestedEdges` library reader, not of the brownfield generator. Phase-7.6's tests (per its design.md L188) cover this. The brownfield spec is duplicating an upstream test.

**Verdict: CONCEDED.**

**Evidence**.
- The scenario asserts: "entries with no producing trace context are accepted with an empty `provenance` object" — this is a reader/schema property.
- Phase-7.6 design.md L110: "[The provenance object] is empty for entries authored manually for testing." — this IS phase-7.6's responsibility.
- Phase-7.6 design.md (testing block, around L188): unit tests round-trip a queue file with one entry per `triage_state` value. Empty `provenance` fixture is part of phase-7.6's test surface.
- The brownfield generator does not produce manual-test entries. It produces engine-emitted entries with populated provenance. The "manual-test" path is by definition not exercised by brownfield code.

**Required fix**. Drop the "Manual-test entries leave provenance empty" scenario from `specs/brownfield/spec.md`. The reader-discipline assertion belongs in `openspec/specs/provenance-foundation/spec.md` (which phase-7.6's apply will create).

---

## SUBSTANTIVE #9 — Atomic-commit grouping invents Groups A-D retroactively

**Sauron's claim**: `design.md:132` says "The four Wave 4 groups commit in any order with respect to Groups A through D from the original phase scope." But original tasks.md has no Groups A-D declared.

**Verdict: PARTIAL — Sauron is technically right that A-D aren't declared in tasks.md, but Bundle A's design.md established Groups A-D for phase-7.6, not phase-9.**

**Evidence**.
- `tasks.md` original sections 1-4 (now sections 1-4 with sections 5-8 added by rescope and 5-6 renumbered to 9-10): no atomic-group annotation.
- Bundle A's design.md L196-201: "Group A: trace sidecar / Group B: queue / Group C: gate, depends on B / Group D: islands query." These are phase-7.6's groups, not phase-9's.
- The rescope's design.md L132 references "Groups A through D from the original phase scope" — this phrasing is ambiguous. If it means "phase-9's pre-existing sections 1-4 treated implicitly as groups", that is a fiction tasks.md does not encode. If it means "phase-7.6's Groups A-D", that is wrong because phase-7.6's groups are not phase-9's.

**Required fix**. Two options:
1. Drop the A-D framing entirely. Rewrite design.md L123-133 as "The Wave 4 sub-components form four atomic-commit groups (E, F, G, H). Groups commit in declared order; cflx-runner enforces group boundaries when the phase declares them. The pre-existing task sections 1-4 commit as individual logical units per the graphite-pr discipline."
2. Declare Groups A-D explicitly in tasks.md (each pre-existing section becomes a group with internal coupling stated).

Option 1 is lighter-touch. Either resolves the fiction.

---

## SUBSTANTIVE #10 — "or equivalent" in templated authoring config surface

**Sauron's claim**: `design.md:106`, `tasks.md:51`, `proposal.md:36` all defer the templated-authoring config surface to apply ("`[templates]` block in `cairn.blueprint` or an equivalent project config file, design choice deferred to apply"). This is the same fragility class the timing stronghold rejected.

**Verdict: PARTIAL — defensible but the prose should explicitly bound the decision to a date/phase.**

**Evidence**.
- `design.md:106`: "Project-config surface: a new `[templates]` block in `cairn.blueprint` (or an equivalent project config file, design choice deferred to apply) declares templates."
- The timing stronghold rejected Option A (rescope before Bundle A) because of token-level fragility on identifiers Bundle A's design.md hadn't ratified. That's an upstream-dependency fragility.
- The "or equivalent" here is a downstream-implementation choice within phase-9-brownfield itself. It does not depend on another phase's identifier ratification. It is the apply-stage agent's choice, made within the scope of phase-9-brownfield.
- The kernel's "tag-extensible, never closed-enum" principle (CLAUDE.md "What cairn is, positively" §2) actively encourages keeping config surfaces extensible. The rescope's "or equivalent" is consistent with that principle but only barely; a phrase like "in `cairn.blueprint` under a new top-level block; the exact block name is fixed at apply time" would be tighter.

**Required fix** (recommended). Replace "or equivalent" with a tighter scope statement: "in `cairn.blueprint` under a new top-level block; if the apply-time agent finds blueprint grammar constraints prevent this, the agent SHALL document the alternative location and update the spec scenario accordingly." This pins the surface to `cairn.blueprint` while leaving an escape hatch for grammar issues.

Net: not a blocker. Tighten the prose; do not block merge on it.

---

## SUBSTANTIVE #11 — Refine + suggest-engine interaction unspecified

**Sauron's claim**: `tasks.md:35` says the suggest engine consumes section-1 candidates plus section-2 summariser output. Section 1 is `init`'s discovery; section 4 is `refine`. Whether the suggest engine fires for `refine` is unspecified.

**Verdict: CONCEDED, real gap.**

**Evidence**.
- `design.md` C8.c block (L74-89): mentions only `cairn init --from-code` and the running-phase trace context. Refine appears only at L82 in the `provenance.stage = "propose"` discussion, ambiguously.
- `tasks.md:35`: "consumes the bounded code samples and structural candidates from section 1 plus summariser output from section 2." Section 4 (refine) reuses the discovery pipeline but is not named.
- `proposal.md:34`: queue path is `openspec/changes/<change>/suggested-edges.json` without distinguishing init from refine.

If refine does not fire the suggest engine, refine cannot capture cross-cutting edges that emerge as code grows — defeating the purpose of refine for AI-assisted authoring. If refine does fire, refine's change directory needs a queue file too and the `CC002` gate applies.

**Required fix**. Add explicit prose to design.md C8.c: "The suggest engine fires for both `cairn init --from-code` and `cairn refine`. Refine-emitted entries set `provenance.stage = 'propose'` against the refine-time change directory and are subject to the same `CC002` gate." Add a refine+suggest scenario to spec.md (one scenario suffices: "Refine emits suggested edges into the refine change directory's queue file").

---

## SUBSTANTIVE #12 — Execution-order qualifier missing on phase-7.6 dependency

**Sauron's claim**: `proposal.md:10` says "MUST run after Phase 8 and Phase 7.6." Should specify "after phase-7.6 apply" since the dependency is on shipped code, not draft design.

**Verdict: CONCEDED, but the redundancy claim is debate-noise.**

**Evidence**.
- `proposal.md:10`: "Execution: MUST run after Phase 8 and Phase 7.6, and before Phase 10."
- The suggest engine code depends on the `SuggestedEdges` library API (phase-7.6 design.md L114 onward) and on `CC002` registration (phase-7.6 design.md C8.b). Both are apply-stage deliverables, not design-stage.
- Sauron's secondary point about Wave numbering being uninformative is correct but irrelevant; Wave numbers are stronghold-internal coordination, not contract surface.

**Required fix**. Tighten `proposal.md:10` to "Execution: MUST run after `phase-8-summariser` apply, after `phase-7.6-ai-provenance-foundation` apply, after `phase-9.0-tests` apply, and before Phase 10." The phase-9.0-tests apply qualifier is inferred from the integrated cross-check sequencing rule 4 (L161) and is worth being explicit about given critical #2.

---

## NIT #13 — No em-dashes

Sauron noted zero hits on U+2014 across all five files. This is a pass; no action required.

---

## NIT #14 — Wave 4 acceptance criterion grouping

Sauron's suggestion: cluster Wave 4 criteria under a labelled subsection. Pure formatting; non-blocking. Defer.

---

## NIT #15 — Out-of-scope list duplicates phase-7.6 rejections

Sauron acknowledges this is fine. No action required.

---

## Final tally

**CRITICAL findings standing after defence**: **2.5 of 3.**
- #1 Path schism: stands. Real, blocker.
- #2 phase-9.0-tests desync: stands as a real coordination gap. Sauron's framing slightly over-strict (the stronghold did not literally require the same PR), but the project ships in a self-contradicting state without the fix. Treat as blocker.
- #3 "Merged in full" premise: stands as a prose defect. The structural posture is correct (timing-stronghold trigger is design.md ratification, met) but the genesis and proposal prose mis-describe the dependency state. Documentation fix only, not architectural.

**SUBSTANTIVE findings standing**: **8 of 9.**
- #4 session-handoff.md citation: stands. Provenance break.
- #5 subjective acceptance criteria: 1 of 5 stands (L42 "obvious"); rest are proposal-voice summaries.
- #6 ADDED-vs-MODIFIED: half stands (force-vs-pending-queue interaction is a real gap; the categorisation argument is debatable).
- #7 guard syntax inconsistency: stands. One-line fix.
- #8 manual-test scenario duplication: stands. Drop the scenario.
- #9 atomic-commit grouping fiction: stands. Drop A-D framing.
- #10 "or equivalent" config surface: stands as a prose tightening; not a blocker.
- #11 refine + suggest interaction: stands. Real gap.
- #12 execution-order qualifier: stands. Easy fix.

**NIT findings**: 0 of 3 require action.

## Recommended path forward

**Fix critical + substantive before merge**, in approximately this order:

1. CRITICAL #1: path normalisation across design.md and spec.md to `openspec/changes/`. Touches 6 lines.
2. CRITICAL #3: prose fix in proposal.md L30 and genesis.md L16 ("design.md ratifies", not "merged in full"). Touches 2 lines.
3. CRITICAL #2: bundle a phase-9.0-tests proposal-update commit into this PR (or a stacked PR before merge). Touches phase-9.0-tests/proposal.md and its spec; the spec doesn't need 1:1 coverage but must reflect 23 across 8.
4. SUBSTANTIVE #4: commit `docs/strongholds/session-handoff.md` (preferred) or rewrite genesis.md L24 to cite committed strongholds.
5. SUBSTANTIVE #6 force-vs-pending-queue scenario: add one scenario.
6. SUBSTANTIVE #7, #8, #9: small prose/scenario edits (drop manual-test scenario; align guard-syntax prose; drop A-D framing).
7. SUBSTANTIVE #11: add refine+suggest sentence to design.md C8.c plus one spec scenario.
8. SUBSTANTIVE #12: tighten proposal.md L10 dependency text.
9. SUBSTANTIVE #5 (L42 only) and #10: prose tightening.

Total estimated edit size: under 60 lines added/removed across 5 files (rescope branch) plus the phase-9.0-tests commit (separate scope).

The rescope's overall structural intent (docs-only proposal-update, four absorbed sub-components, phase-7.6 dependency, validate-strict passing) is sound and worth preserving. The defects are coherence and prose issues, not concept defects. **Do NOT scrap and re-author.** Fix in-place.

