# PR #23 Sauron Critique — Round 1

**Branch**: `openspec/phase-9.0-tests-wave4-sync`
**Commit**: `5b522a3`
**Reviewer**: Sauron, Lord of the Lidless Eye
**Date**: 2026-05-01
**Sibling PR**: #22 (`openspec/phase-9-brownfield-wave4-rescope`, commit 75fec86) is the source-of-truth this PR claims to mirror.

## Methodology

I read the five files this PR touches against the source-of-truth brownfield spec on the sibling branch. I cross-referenced the live timing stronghold (`oq4-phase9-rescope-timing.md`), the genesis transcript convention (`openspec/conventions.md` §9), the cflx checkbox detector (`.claude/skills/cflx-proposal/scripts/cflx.py`), my own Round 1 critique on PR #22, and Saruman's defense. I counted scenarios, mapped them to stubs, and scanned the staging document body for path drift, em-dashes, and self-contradiction.

The wizard has done substantial work here. He has also planted four new failures atop the corrupted ground he failed to clean before authoring this sync. I will name each.

---

## 1. Path-drift contagion: `meta/changes/` versus `openspec/changes/` — BLOCKING

The same schism I named in PR #22 has propagated, with surgical malice, into this sync. The wizard has corrected some references and not others. The result is a document that points two directions at once.

**Evidence — `tasks.md` (PR branch)**:

- `tasks.md:12` (Section 2.3): `init__creates_brownfield_change_directory` asserts `meta/changes/brownfield-init/`, `proposal.md`, `blueprint.delta`, and stub contracts are created.
- `tasks.md:42` (Section 7.1): the suggest engine emits into `openspec/changes/<change>/suggested-edges.json`.
- `tasks.md:52` (Section 8.2): the transcript writes to `openspec/changes/<id>/research/genesis.md`.
- `tasks.md:53` (Section 8.3): all reads/writes happen inside `openspec/changes/<change>/research/`; "no session state lands in main `meta/`".

**Evidence — `design.md` (PR branch)**:

- `design.md:91-93` (Sample Test Sketch, the canonical example):
  ```rust
  assert\!(repo.path().join("meta/changes/brownfield-init").exists());
  assert\!(repo.path().join("meta/changes/brownfield-init/proposal.md").exists());
  assert\!(repo.path().join("meta/changes/brownfield-init/blueprint.delta").exists());
  ```

**Evidence — source-of-truth on sibling branch** (`origin/openspec/phase-9-brownfield-wave4-rescope:openspec/changes/phase-9-brownfield/specs/brownfield/spec.md`):

- L27: `cairn init --from-code` creates `openspec/changes/brownfield-init/`.
- L35, L42: existing/force scenarios pin `openspec/changes/brownfield-init/`.

**Why this is BLOCKING**: PR #22's CRITICAL #1 finding (the path schism in the brownfield rescope) was the **predicate** for this entire sync existing. The Sauron critique on PR #22 stated explicitly that either the init flow lands under `openspec/changes/` (in which case three spec scenarios in PR #22 are wrong) or it lands under `meta/changes/` (in which case the suggest-engine queue file path conflicts with Bundle A's ratified location). Saruman's defense conceded the schism. PR #22 has since fixed the brownfield spec to `openspec/changes/`. PR #23 — the sibling sync, authored *after* the rescope — has carried `meta/changes/` forward into four lines of `tasks.md` 2.3 and three lines of `design.md` 91-93.

The pre-phase test wall now claims it will assert on a path that the source-of-truth rescope has explicitly rejected. The test stub `init__creates_brownfield_change_directory`, when Phase 9 implements it, will assert that a directory exists at `meta/changes/brownfield-init/` while the brownfield spec it is testing requires `openspec/changes/brownfield-init/`. The stub will fail not because the feature is unimplemented but because the stub's assertion is wrong. The pre-phase has been authored against a stale ghost of the rescope.

The Sample Test Sketch in `design.md:91-93` is the most damning artefact: it is the canonical example the apply-stage agent is told to copy, and it points at the deprecated path. Saruman has personally inscribed the wrong runes on the very stone the apprentices will copy.

**Suggested fix**: Replace `meta/changes/brownfield-init/` with `openspec/changes/brownfield-init/` at `tasks.md:12` and at `design.md:91-93`. Also reconcile `tasks.md:53`: the phrase "no session state lands in main `meta/`" inherits the legacy spelling — the canonical structure is `openspec/specs/` (per `openspec/conventions.md` §9, which says the file SHALL NOT be moved to `meta/research/`). The `meta/` reference is a fossil from the canonical `docs/spec.md` (which is itself stale on this point — that is a separate problem, not this PR's, but the wizard should not propagate the staleness into authoring guidance).

**Severity**: BLOCKING.

---

## 2. Acceptance-criterion arithmetic does not close — BLOCKING

The wizard cannot count. Or he can count, and chooses inconsistency.

**Evidence — proposal vs design vs spec vs tasks vs genesis, all five sources disagree**:

- `proposal.md:39-41`:
  - "one `#[ignore = "awaits phase-9"]` test per Phase 9 acceptance-criterion scenario across all 8 rescoped requirements (23 scenarios total)"
  - "one `#[ignore = "awaits phase-9"]` test per heuristic invariant in `phase-9-brownfield/design.md` (7 invariants)"
  - "**Total ignored test count is 30** (23 acceptance + 7 heuristic)."

- `design.md:69` (the coverage table summary line):
  - "Total: 25 acceptance-criterion stubs plus 7 heuristic-invariant stubs. ... **Total stub count is 32**."

- `tasks.md:69` (Section 11.5):
  - "`cargo test -- --ignored` shows **all 30 phase-9 tests as failing**; the count is 23 acceptance-criterion stubs plus 7 heuristic-invariant stubs."

- `specs/brownfield-tests/spec.md:20` (Scenario: Ignored tests are present and enumerable):
  - "the output lists at least **30** test names"

- `genesis.md:20`:
  - "Stub count: from 17 (10 acceptance + 7 heuristic) to **32 (25 acceptance + 7 heuristic)**."

The coverage table in `design.md` enumerates 25 acceptance-criterion test functions plus 7 heuristic-invariant test functions, totalling 32. I counted them by hand. The proposal claims 30. The tasks claim 30. The spec scenario asserts the verification gate will see "at least 30." The design and genesis correctly state 32. **The proposal, tasks, and spec are off by two and contradict the design's own table.**

The spec scenario in `specs/brownfield-tests/spec.md:20` says "**at least** 30 test names." This weasel phrasing technically permits 32. But the proposal's `Acceptance Criteria` (`proposal.md:41`) says "Total ignored test count is **30**" — a flat assertion. This will fail at validate-strict if the implementation matches the design's 32-row table, which is what the sync's own genesis says it intends. The acceptance criterion contradicts the implementation contract on the same proposal.

**Why it matters**: the apply-stage agent will be told by the proposal that 30 stubs exist. It will be told by the design's table to write 32. It will be told by the verification task that 30 must be reported. The agent will either truncate the design's 32-row table by deleting two stubs (silent scope loss) or reject the proposal as self-inconsistent. Either outcome poisons the apply.

The wizard's own genesis L20 narrates the resolution: "The acceptance count exceeds the 23-scenario count because Req 5 scenario 1 carries three load-bearing assertions ... split across three stubs ... the other 22 scenarios map one-to-one. Total stub count is 32." He understood the answer. He then forgot it twice and contradicted it three times.

**Suggested fix**: Replace "Total ignored test count is 30 (23 acceptance + 7 heuristic)" at `proposal.md:41` with "Total ignored test count is 32 (25 acceptance-criterion stubs + 7 heuristic-invariant stubs); the acceptance count exceeds the 23 scenario count because Req 5 scenario 1 splits into three stubs." Update `tasks.md:69` (Section 11.5) similarly to "all 32 phase-9 tests as failing; the count is 25 acceptance-criterion stubs plus 7 heuristic-invariant stubs." Tighten the spec scenario at `spec.md:20` to "lists exactly 32 test names" or sharpen "at least 30" to "at least 32."

**Severity**: BLOCKING. (The numbers are the contract.)

---

## 3. Coverage-table integrity: 25 acceptance rows, 23 scenarios, two unaccounted phantoms — SUBSTANTIVE

I audited the design.md coverage table row by row against the source-of-truth brownfield spec's 23 scenarios.

**The 25 acceptance rows in `design.md:33-58`**:

- Req 1 scenarios 1-5 → 5 stubs (one-to-one, correct).
- Req 2 scenarios 1-2 → 2 stubs (one-to-one, correct).
- Req 3 scenario 1 → 1 stub (one-to-one, correct).
- Req 4 scenarios 1-2 → 2 stubs (one-to-one, correct).
- Req 5 scenarios 1-4 → **6 stubs** (scenarios 1, 2, 3, 4 mapped to `engine_writes_to_queue_file`, `entry_triage_state_is_pending`, `entry_provenance_carries_trace_phase`, `pending_entries_block_archive_with_cc002`, `no_auto_accept_on_high_confidence`, `manual_test_entries_accept_empty_provenance`; the table marks two of these as "scenario 1 (assertion split)").
- Req 6 scenarios 1-3 → 3 stubs (one-to-one, correct).
- Req 7 scenarios 1-3 → 3 stubs (one-to-one, correct).
- Req 8 scenarios 1-3 → 3 stubs (one-to-one, correct).

That sums to **5+2+1+2+6+3+3+3 = 25 acceptance stubs**, consistent with the design's stated total. Good.

**But the source-of-truth brownfield spec has 23 scenarios. Where do the extra two acceptance stubs come from?**

The wizard's table answers "Req 5 scenario 1 split across three stubs" — that adds two extra stubs (one scenario becomes three). Net acceptance stub count: 23 base + 2 extras from Req-5-scenario-1 split = 25. Arithmetic checks.

**However**, two of the Req 5 mappings in the design table are wrong:

- **Row labelled `suggest__pending_entries_block_archive_with_cc002` → Req 5, scenario 3.** This is correct. Source-of-truth brownfield spec L113-118: "Pending entries block archive through the CC002 gate" is scenario 3.
- **Row labelled `suggest__no_auto_accept_on_high_confidence` → Req 5, scenario 2.** Correct. Source spec L107-111: "Suggest engine never auto-accepts" is scenario 2.
- **Row labelled `suggest__manual_test_entries_accept_empty_provenance` → Req 5, scenario 4.**

The source-of-truth Req 5 has FIVE scenarios, not four:

1. Suggest engine writes to the queue file.
2. Suggest engine never auto-accepts.
3. Pending entries block archive through the CC002 gate.
4. Refine emits suggested edges into the refine change directory.
5. Force-init aborts when pending entries exist.

(Counted from `origin/openspec/phase-9-brownfield-wave4-rescope:openspec/changes/phase-9-brownfield/specs/brownfield/spec.md` L97-138.)

**The design's "scenario 4" mapping is wrong**: `manual_test_entries_accept_empty_provenance` does not correspond to scenario 4 of Req 5. The actual scenario 4 is "Refine emits suggested edges into the refine change directory" (L120-127 of the source spec) — the refine+suggest interaction that PR #22's Sauron critique CRITICAL #5 forced into the brownfield spec. **The sync has no stub for it.** Nor for scenario 5 (Force-init aborts on pending). The sync covers Req 5 scenarios 1, 2, 3 (with scenario 1 split into three) and invents a fictional `manual_test_entries_accept_empty_provenance` stub mapped to a nonexistent "scenario 4."

This means the rescoped brownfield spec carries **24 scenarios across 8 requirements**, not 23 (count: 5+2+1+2+5+3+3+3 = 24). Or — equally plausible — the wizard's count of 23 was correct against an earlier draft of the brownfield spec, and PR #22's CRITICAL #5 fix added two more scenarios (scenario 4 refine+suggest, scenario 5 force-init-aborts) that this sync was authored before, then never re-counted.

**Verification**: `git show origin/openspec/phase-9-brownfield-wave4-rescope:openspec/changes/phase-9-brownfield/specs/brownfield/spec.md | grep -c "^#### Scenario"` returns the live count.

The wizard's "23 scenarios" headline is wrong, his Req-5 mapping invents a phantom scenario 4, and his stub list is missing **two real scenarios** (refine+suggest emission; force-init pending-abort). These are exactly the scenarios PR #22's Sauron critique forced into the brownfield spec. Saruman, having lost that point, did not re-survey before authoring the sync. He inherited the count from his own first draft.

**Suggested fix**:
1. Re-count scenarios in the source-of-truth spec. Confirm the live count.
2. Add stubs `suggest__refine_emits_to_queue_file_with_propose_stage` (Req 5 scenario 4) and `suggest__force_init_aborts_on_pending_entries` (Req 5 scenario 5).
3. Either remove `suggest__manual_test_entries_accept_empty_provenance` (it has no source scenario) or justify it as a design-invariant stub and move it to a separate "design invariants" group beyond the heuristic block.
4. Rewrite the headline counts (`proposal.md:14`, `proposal.md:18`, `genesis.md:16`, `genesis.md:20`) against the live count.

**Severity**: SUBSTANTIVE bordering on BLOCKING. The sync misses two real scenarios that PR #22 specifically enriched the brownfield spec with.

---

## 4. The phantom Req-5-scenario-1 three-way split: defensible isolation or invented padding — SUBSTANTIVE

The Nazgul force claimed three load-bearing assertions in Req 5 scenario 1 ("Suggest engine writes to the queue file") justify three stubs.

**Source-of-truth brownfield spec L99-105 (Req 5, scenario 1)**:

- **GIVEN** a brownfield change being authored ... against a repository with multiple modules
- **WHEN** the suggest engine identifies a cross-cutting edge that the deterministic extractor did not infer
- **THEN** the edge is written as an entry in `openspec/changes/<change>/suggested-edges.json`
- **AND** the entry's `triage_state` is `pending`
- **AND** the entry's `provenance.trace_phase` names the running phase

Three THEN clauses. The wizard splits each into its own stub: `suggest__engine_writes_to_queue_file`, `suggest__entry_triage_state_is_pending`, `suggest__entry_provenance_carries_trace_phase`.

**On the merits**: the split is defensible. Each THEN is independently testable, and Phase 9's apply will indeed flip them to passing in different commits as the engine matures (queue write first, triage_state second, provenance population last). Failure isolation is a real engineering virtue. I find this admission as unpleasant as the light of Earendil, but **the split is a reasonable design choice** — provided the proposal's count arithmetic absorbs it cleanly.

**However**, the same logic applied consistently would force splits on:

- **Req 1 scenario 3** (`init__creates_brownfield_change_directory`) — five THEN clauses: creates directory, writes proposal, writes `blueprint.delta`, writes stub contracts, does not write main `cairn.blueprint`. The wizard claims this is "tightly coupled because all hinge on a single init invocation." That is a defensible boundary. But it inconsistently applies the principle: in Req 1, scenario 3, conjunctive THENs that all hinge on one syscall stay as one stub; in Req 5, scenario 1, conjunctive THENs that all hinge on one engine call get three stubs. The differentiating principle is not stated.

- **Req 7 scenario 1** (`templates__matching_template_guides_stub_authoring`) — three THEN clauses: draft uses required headers, draft uses optional sections, summariser-supplied content fills body sections per precedence rule. By the wizard's own splitting logic these should be three stubs. He left them as one.

The rule "split when properties are independently observable" is sound. The wizard applies it once (Req 5) and abandons it twice (Req 1 scenario 3, Req 7 scenario 1). Either expand the splits consistently or contract Req 5's split to a single stub `suggest__engine_writes_to_queue_file_with_pending_state_and_trace_phase`.

**Severity**: SUBSTANTIVE. The split is technically right; the inconsistency is the offense.

**Suggested fix**: Add a one-sentence rule to `design.md` "Failing-State Stub Contract" naming the principle: "Multi-THEN scenarios split into one stub per THEN clause when each clause exercises a distinct subsystem (e.g., schema serialisation vs gate registration vs trace context resolver). Multi-THEN scenarios stay as a single stub when all clauses hinge on one syscall (e.g., one filesystem write producing four sibling files)." Then recount; either the heuristic invariants or the Req 1 scenario 3 stays as one, both being justified, and the count is honest.

---

## 5. Failing-state contract: defined, but with a soft rim — SUBSTANTIVE

The wizard introduced a "Failing-State Stub Contract" section in `design.md:24-33` and a corresponding spec scenario in `spec.md:60-66`. Three properties named:

1. Compile cleanly under `cargo build` with zero warnings.
2. Sit behind `#[ignore = "awaits phase-9"]`.
3. Either call `unimplemented\!()` or assert a property only a not-yet-written feature satisfies.

**On the merits**: this is correct. The pre-phase gates `cargo test` (passes; tests are ignored) and `cargo test -- --ignored` (every test fails at runtime, not compile time). This is the canonical Rust pattern for failing-state pre-phase scaffolding and matches the prior-art `phase-7.5a-test-fortification` cited in the design's references. The wizard has done the work I forced him to do in PR #22.

**The soft rim**: the contract does not require that every `unimplemented\!()` stub also document the precise property the future code will need to satisfy. The proposal/tasks describe what each stub asserts in prose, but the stub body itself, per the Sample Test Sketch at `design.md:81-95`, is just `unimplemented\!("phase-9 supplies the suggest engine fixture");`. When Phase 9's apply-time agent removes the `#[ignore]` and tries to make the test pass, it has no in-file specification for what "passing" means. The agent must round-trip back to `tasks.md`, find the corresponding `7.x` task line, parse the prose, and translate it into an assertion.

This is exactly the documentation drift the cflx skill is meant to prevent. The spec scenario at `spec.md:60-66` ("Wave 4 stubs preserve failing-state contract without compile errors") asserts the *negative* property (no compile error) without asserting the *positive* property (the stub body documents the assertion).

**Suggested fix**: Add to the Failing-State Stub Contract a fourth property: "Each `unimplemented\!()` stub MUST carry a doc-comment immediately preceding the function naming the property the implementation must satisfy in plain English. The doc-comment is the in-file contract that Phase 9's apply-time agent reads to author the assertion." Update the Sample Test Sketch at `design.md:81-95` to demonstrate this with a `///` line above `suggest__entry_triage_state_is_pending`.

**Severity**: SUBSTANTIVE.

---

## 6. ADDED-versus-MODIFIED: correctly applied, but with one omission — SUBSTANTIVE

The wizard's genesis Q&A at L43 explicitly addresses this: the original `specs/brownfield-tests/spec.md` had a single ADDED Requirement, and the sync amends that requirement with two new scenarios. Because the requirement is still under ADDED status (the change has not yet archived), edits are amendments to the proposed delta rather than MODIFIED operations on a consolidated spec.

**On the merits**: this is correct. OpenSpec convention is that pre-archive edits to ADDED Requirements stay under `## ADDED Requirements`; MODIFIED is for already-archived requirements. The wizard avoided Saruman's PR #22 mistake (which conflated narrowing with extension and triggered CRITICAL #4). I find this acknowledgment of competence as unpleasant as the dawn over Mount Doom. He has read his own postmortem.

**The omission**: PR #22 fixed the brownfield `specs/brownfield/spec.md` to use a `## MODIFIED Requirements` section for the "Keep human review authoritative" requirement that the suggest-engine new requirement narrows. PR #23's `specs/brownfield-tests/spec.md` does not need a MODIFIED section because brownfield-tests is its own capability area and was originally ADDED in this same proposal. So far, fine.

But the wizard added two new scenarios to the existing ADDED Requirement without renaming the requirement to reflect the broadened scope. The requirement title is still "Phase 9 acceptance criteria have failing test coverage." The new scenarios assert (a) failing-state contract preservation, (b) conditional obligations stub behaviour. These are not "Phase 9 acceptance criteria" in the same sense — they are meta-properties of the test wall itself. The single requirement now carries six scenarios across two semantic axes (coverage of Phase 9 scenarios; failing-state contract guarantees), which is a smell.

**Suggested fix**: Either accept the smell and add a one-line note to the Requirement description naming the broadened scope, or split the new scenarios into a second ADDED Requirement: "Test stubs preserve failing-state contract." The latter is cleaner but more disruptive at this stage; the former is acceptable.

**Severity**: SUBSTANTIVE (NIT-leaning).

---

## 7. Cross-PR consistency on error code references and paths — NIT

I checked `CC002` references against PR #22's prose-corrected forms.

**Evidence**:

- `tasks.md:45` (Section 7.4): "asserts `cflx openspec validate <change> --strict` exits non-zero with error code `CC002`."
- `proposal.md` does not reference `CC002` directly. (Wise.)
- `design.md` does not reference `CC002` directly except via the included tasks.

PR #22 settled on the framing: `CC002` is a Bundle A apply-stage deliverable; the rescope's acceptance criteria reference it as forward-compatible against Bundle A's apply. PR #23's `tasks.md:45` references `CC002` as an assertion target inside a stub that calls `unimplemented\!()`. This is consistent: the stub does not yet exercise the gate; the gate's existence is a Phase 9 apply-time prerequisite. The dependency declaration in `proposal.md:7-8` correctly defers to `phase-9-brownfield`.

**Path references** (other than the BLOCKING #1 `meta/changes/` schism): `openspec/changes/<change>/suggested-edges.json` at `tasks.md:42` matches Bundle A's L82 ratification. `openspec/changes/<id>/research/genesis.md` at `tasks.md:52` matches `openspec/conventions.md` §9.

**Severity**: NIT. The wizard has at least kept the canonical paths consistent in the new sections he added (Sections 7-10). His sin is in the legacy section he failed to clean (Section 2.3, design Sample Test Sketch).

---

## 8. Genesis honesty: clean enough — NIT

`research/genesis.md` correctly attributes the trigger event:

- L17: "Trigger event: PR #22 (Sauron/Saruman debate, CRITICAL #2 finding)..."
- L18: cites the Option B refined timing verdict.
- L19: "docs-only" scope correctly stated.

The Q&A section at L24-50 records six decisions with rationale. The Q at L43 correctly addresses the ADDED-vs-MODIFIED question and arrives at the right answer. The Q at L29 acknowledges the Req 5 scenario 1 split — though, per Critique #4, the splitting principle is not consistently applied.

**One mild dishonesty**: L20's "from 17 (10 acceptance + 7 heuristic) to 32 (25 acceptance + 7 heuristic)" frames the delta cleanly but conceals that the source-of-truth scenario count itself was wrong on this branch (per Critique #3 — the live count is 24, not 23, after PR #22's enrichment). The genesis presents 23 as a fact when it should be presented as a measurement that the apply-time agent should re-take.

**Severity**: NIT. The wizard is honest about the trigger and the timing; he is incomplete about the source scenario count.

**Suggested fix**: Add to genesis L17-20: "Re-counting against the live source-of-truth at apply time is mandatory; the 23-scenario count was measured at sync-authoring time and may have drifted if the brownfield spec gains scenarios in subsequent debate rounds."

---

## 9. Em-dashes — clean

Zero em-dashes across all five files. The wizard has at least learned this lesson. I find this admission as unpleasant as the dawn light on the slopes of Mount Doom. He has internalised the house style.

**Severity**: none.

---

## 10. Tasks.md sort order and checkbox detection — NIT (checked, holds)

The cflx checkbox detector at `.claude/skills/cflx-proposal/scripts/cflx.py:359` regex-matches `^\s*[-*]\s*\[([ x])\]\s+(.*)$` line by line. It is section-number-blind. The genesis Q at L40 correctly asserts this. I verified the regex.

The renumber from "Section 7 Required Verification" to "Section 11 Required Verification" with new sections 7-10 inserted preserves checkbox detection mechanically. Each `- [ ]` line still parses uniformly. The internal task IDs (`7.1`, `8.2`, etc.) are prose, not gate-relevant.

**However**: pre-phase commit `c98d506` ("openspec: draft phase-{8.0,9.0,10.0}-tests + fix pre-phase sort order") was the commit that fixed sort-order on these pre-phases. I see no evidence in the PR diff that `phase-9.0-tests` task IDs have been re-checked against any sort-order contract beyond raw section numbering. Since the renumber is purely additive and section numbers proceed monotonically (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11), this is fine. But the wizard's genesis Q at L40 dismisses the question with "the renumber is mechanically safe" and does not cite the regex, the cflx version, or the test it ran. He asserts safety; he does not demonstrate it.

**Severity**: NIT. (Holds, but the demonstration is hand-waved.)

---

## 11. Stronghold timing contract: T+7 vs PR-merge-window — NIT

Per `oq4-phase9-rescope-timing.md` L132: "T+7 (Wave 3 begins): Bundle C, Bundle E, spec-validate-cleanup-sweep author and ship in parallel. Phase 9's proposal now reflects the rescoped scope; phase-9.0-tests' proposal authoring (if not already done) authors against the rescoped Phase 9."

PR #23's authoring at sync time should have happened against the **rescoped** Phase 9 — meaning the version on `openspec/phase-9-brownfield-wave4-rescope` post-PR-22-fixes, not pre-PR-22. The wizard claims (genesis L18) "this pre-phase happens after the rescope merges." But PR #22 has not yet merged (its critique is in active debate); PR #23 was authored against the in-flight branch. This is fine *if* PR #22's debate stabilises before PR #23 merges, but it is fragile: any further changes to PR #22 (e.g., a CRITICAL #5 fix that adds Req 5 scenario 5) propagate into PR #23 as drift.

The Critique #3 finding above (missing Req 5 scenarios 4 and 5) is the materialisation of exactly this risk. The wizard authored against an early snapshot of PR #22 and never re-synced.

**Severity**: NIT (process). The technical consequence is captured in Critique #3 (BLOCKING-grade impact via missing stubs).

---

## Summary

| # | Finding | Severity |
|---|---|---|
| 1 | `meta/changes/` path drift in `tasks.md:12` and `design.md:91-93` contradicts source-of-truth `openspec/changes/` | BLOCKING |
| 2 | Stub-count arithmetic disagrees across proposal (30) / tasks (30) / design (32) / spec (≥30) / genesis (32) | BLOCKING |
| 3 | Coverage table maps a phantom Req 5 scenario 4; misses real Req 5 scenarios 4 (refine+suggest) and 5 (force-init aborts) added by PR #22 | SUBSTANTIVE bordering BLOCKING |
| 4 | Inconsistent application of the "split multi-THEN" principle across Req 5 scenario 1 (split) vs Req 1 scenario 3 / Req 7 scenario 1 (not split) | SUBSTANTIVE |
| 5 | Failing-state contract is defined but does not require in-file doc-comments naming the property each `unimplemented\!()` stub will assert | SUBSTANTIVE |
| 6 | The amended ADDED Requirement now spans two semantic axes (coverage; failing-state guarantees) without title or split | SUBSTANTIVE (NIT-leaning) |
| 7 | Cross-PR consistency on error codes and canonical paths holds in new sections; legacy sections retain `meta/` | NIT |
| 8 | Genesis is honest about trigger and timing; incomplete about source scenario count drift risk | NIT |
| 9 | Em-dashes: zero | none |
| 10 | Tasks renumber preserves cflx checkbox detection mechanically; demonstration is hand-waved | NIT |
| 11 | Authoring against an in-flight PR #22 branch produced inheritable drift (Critique #3 is the materialisation) | NIT (process) |

The wizard fixed the count headline (23/8 vs 10/4) and did not fix the count's substance (24/8 actual after PR #22's enrichment; 32 stubs not 30). He cleaned the new sections of em-dashes and path drift while leaving `meta/changes/` fossils in the section every apply-stage agent will copy from (the Sample Test Sketch). He defined a failing-state contract and did not require the contract to bind in-file. He authored against a moving target and did not re-survey before commit.

This sync is closer to correct than the rescope it mirrors. It is not yet correct.

