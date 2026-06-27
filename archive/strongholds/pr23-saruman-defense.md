# PR #23 Saruman Defense — Round 1

**Branch**: `openspec/phase-9.0-tests-wave4-sync`
**Commit**: `5b522a3`
**Defender**: Saruman the White, Master of Isengard
**Date**: 2026-05-01
**Verdict (preview)**: **5 of 5 critical findings stand. 1 of 2 substantive findings stands.**

---

## BANTER

So. The Eye returns, gloating, with a fresh ledger of my failures.

I will not pretend the news is good, Sauron. You caught me. Most of what you bring is, this time, real. I read your critique, then I read my own files, and I read the source-of-truth at `75fec86`, and the picture is what it is.

But before I concede the criticals one by one — let me at least be a wizard about it and dispatch the small things first.

**The "soft rim" on the Failing-State Stub Contract** — your CRITICAL #5. You asked why my contract does not require a `///` doc-comment on every `unimplemented\!()` stub naming the property the implementation must satisfy. **Because the property is already named in `tasks.md`, mighty lord of redundancy.** Section 7.1 reads "asserts the suggest engine emits a cross-cutting edge into `openspec/changes/<change>/suggested-edges.json` during a brownfield init run that triggers the engine. Stub calls `unimplemented\!()` until the engine fixture exists." That IS the in-file contract. The apply-stage agent reads `tasks.md` to drive each unit of work — it is the canonical authority chain artefact for apply, not a `///` line above a function. You are demanding I duplicate the assertion language in two places so it can drift in two places. Splendid engineering, that. The doc-comment-as-contract pattern you propose is *worse* than what exists, because it creates a second source of truth that nobody has nominated as authoritative. **DEFEND** on the merits, with a small concession in the FINDINGS that adding `///` lines is *cheap* and *harmless* and would have stopped you reaching for it — so I will add them, but not because the contract is soft. Because the contract being doubly-belted will silence you.

Now. The criticals.

**The path schism — your CRITICAL #1.** FINE. You are correct. Commit `75fec86` on PR #22 normalised `meta/changes/brownfield-init/` to `openspec/changes/brownfield-init/` across spec.md, tasks.md, design.md and proposal.md. I authored this sync against an in-flight branch and did not re-survey after the path normalisation landed. `tasks.md:12` and `design.md:91-93` carry the old path. The Sample Test Sketch — the canonical example apply-stage agents copy from — points at the deprecated location. **CONCEDE.** Two-line fix. I will fix it. Do not gloat.

**The arithmetic — your CRITICAL #2.** FINE again. The numbers do not close. The design table has thirty-two rows. I counted them just now. Twenty-five acceptance plus seven heuristic. The proposal says thirty. Tasks says thirty. The spec scenario weasels with "at least 30" but the test-name list inside the same spec scenario enumerates thirty-two by name. The genesis says thirty-two. I solved it once in genesis and contradicted it twice in the proposal. **CONCEDE.** The fix is mechanical: rewrite `proposal.md:41` to "Total ignored test count is 32 (25 acceptance + 7 heuristic); Req 5 scenario 1 splits into three stubs for failure isolation." Rewrite `tasks.md:69` to match. Tighten `spec.md:20` from "at least 30" to "exactly 32" or "at least 32".

**The missing scenarios — your CRITICAL #3.** This one stings worst, because you are right twice over.

`git show origin/openspec/phase-9-brownfield-wave4-rescope:openspec/changes/phase-9-brownfield/specs/brownfield/spec.md | grep -c '^#### Scenario'` returns **24**, not 23. I just ran it. PR #22's debate added "Refine emits suggested edges into the refine change directory" (Req 5 scenario 4 at L121) and "Force-init aborts when pending entries exist" (Req 5 scenario 5 at L130). I authored against the early draft of PR #22 and never resurveyed after Sauron's CRITICAL #5/#6 finding forced both into the rescoped spec.

Worse: the commit message at `75fec86` says explicitly "**Spec drops (SUBSTANTIVE #8): drop 'Manual-test entries leave provenance empty' scenario** (asserts a phase-7.6 SuggestedEdges reader invariant, not a brownfield generator behaviour)." That is the scenario my `suggest__manual_test_entries_accept_empty_provenance` stub points at. PR #22 dropped the scenario. I kept the stub. **CONCEDE on both halves of #3.** Two real scenarios are missing stubs, one phantom stub points at a deliberately-deleted scenario. Three stubs need editing. Wonderful.

**The phantom Req 5 scenario 4 mapping — your CRITICAL #4.** This is just the second half of #3, but **CONCEDE** explicitly. The design table line `suggest__manual_test_entries_accept_empty_provenance | spec Req 5, scenario 4` at design.md:51 (the "scenario 4" slot) maps to a scenario that the rescope deleted. The actual Req 5 scenario 4 in the rescoped spec is "Refine emits suggested edges." There is no stub for it. There is no stub for scenario 5 either. The phantom and the gap are the same wound, viewed from two angles.

So. Five criticals. Five concessions. Five mechanical fixes. I will not insult either of us by pretending this is a near miss. The sync is closer to correct than the rescope was, but it is not yet correct. You have the satisfaction. Take it briefly. I will fix the files.

But, Sauron — one parting volley before you ascend back to your tower.

**Your SUBSTANTIVE #4** — the splitting principle inconsistency — I take seriously. Req 5 scenario 1's three-THEN split is defensible. Req 1 scenario 3's five-THEN preservation is also defensible. The differentiating principle is "do these THENs hinge on one syscall or three subsystems," and yes, I should state it. I will add the principle to design.md "Failing-State Stub Contract." **CONCEDE PARTIAL.** This is craftsmanship, not a contract violation.

**Your SUBSTANTIVE #6** — the requirement spans two semantic axes — is a smell, not a bug. The single ADDED Requirement now carries scenarios about (a) coverage of Phase 9 scenarios and (b) failing-state contract guarantees. You are right that it is a smell. You are also right that splitting it now would be more disruptive than worth. I will add a one-line note to the requirement description naming the broadened scope. **PARTIAL.** Smell acknowledged, not fixed by structural split.

**Three minor findings (#7, #8, #10)**: NIT-grade. Genesis honesty (#8) — fine, I will add a re-survey line. Cross-PR consistency on canonical paths in new sections (#7) — you yourself called it clean in the new sections. Tasks renumber (#10) — you yourself verified the regex still matches. These are not fixes; they are flavour.

**Em-dashes (#9): zero, as you confirmed.** Mark this in your tower's chronicle: I have, at minimum, internalised one piece of CAIRN house style.

---

## FINDINGS

```
- [concede] [tasks.md:12, design.md:91-93] Path schism reproduced. Source-of-truth at 75fec86 normalised to `openspec/changes/brownfield-init/`. PR #23 retains `meta/changes/brownfield-init/` in tasks.md Section 2.3 and the Sample Test Sketch. Fix: replace both with `openspec/changes/brownfield-init/`. Mechanical edit; cflx validate-strict will not catch but apply-stage assertion will fail.

- [concede] [proposal.md:41, tasks.md:69, specs/brownfield-tests/spec.md:20] Stub-count arithmetic does not close. Design table enumerates 32 rows (25 acceptance + 7 heuristic), confirmed by row count and by the test-name enumeration inside the spec.md "All brownfield scenarios have test coverage" scenario. Proposal says 30, tasks says 30, spec.md weasels with "at least 30." Genesis correctly says 32. Fix: rewrite proposal.md:41 to "32 (25 acceptance + 7 heuristic)"; rewrite tasks.md:69 to "all 32 phase-9 tests as failing"; tighten spec.md:20 to "at least 32" or "exactly 32".

- [concede] [design.md:33-58 coverage table, specs/brownfield-tests/spec.md:35-66] Source-of-truth has 24 scenarios across 8 requirements, not 23. PR #22's commit 75fec86 added Req 5 scenario 4 ("Refine emits suggested edges into the refine change directory", L121-127) and Req 5 scenario 5 ("Force-init aborts when pending entries exist", L130-138). PR #23 misses stubs for both. Fix: add `suggest__refine_emits_suggested_edges_in_refine_change_dir` (Req 5 scenario 4) and `suggest__force_init_aborts_on_pending_entries` (Req 5 scenario 5) to design.md coverage table, tasks.md Section 7, and the test-name list in spec.md.

- [concede] [design.md:51] The phantom Req 5 scenario 4 mapping. Commit 75fec86's message states explicitly: "Spec drops (SUBSTANTIVE #8): drop 'Manual-test entries leave provenance empty' scenario (asserts a phase-7.6 SuggestedEdges reader invariant, not a brownfield generator behaviour)." `suggest__manual_test_entries_accept_empty_provenance` maps to a scenario the rescope deleted. The slot it occupies (Req 5 scenario 4) is now occupied by "Refine emits suggested edges." Fix: delete the `suggest__manual_test_entries_accept_empty_provenance` row from design.md, tasks.md Section 7.6, and spec.md test-name enumeration. Net acceptance stub count after the three corrections (delete one phantom, add two real): 25 - 1 + 2 = 26 acceptance stubs; total 33.

- [defend] [design.md:24-33] Failing-state contract doc-comment requirement. The property each `unimplemented\!()` stub asserts is named in the corresponding tasks.md task line (e.g., tasks.md:42 names what `suggest__engine_writes_to_queue_file` asserts). The authority chain runs proposal -> tasks -> apply; tasks.md is the canonical in-file contract for the apply agent, not a Rust doc-comment. Doubling the assertion language into a `///` line creates a second source of truth that can drift independently. The single source of truth is correct. THAT SAID — adding `///` doc-comments is harmless redundancy and would silence the critique without weakening the contract; recommended as polish, not as a fix to a real defect. Treat as PARTIAL: recommend adding the doc-comment lines to design.md Sample Test Sketch and the contract section, but explicitly as redundancy-with-tasks.md, not as a primary contract.

- [partial] [design.md "Failing-State Stub Contract"] Multi-THEN splitting principle inconsistency. The split for Req 5 scenario 1 (three subsystem touches) versus the no-split for Req 1 scenario 3 (one syscall, four sibling artefacts) is defensible per the implicit "subsystems vs syscall" rule. Sauron is correct that the rule is implicit and not stated. Fix: add to design.md Failing-State Stub Contract: "Multi-THEN scenarios split into one stub per THEN clause when each clause exercises a distinct subsystem (e.g., schema serialisation versus gate registration versus trace context resolver). Multi-THEN scenarios stay as a single stub when all clauses hinge on one syscall (e.g., one filesystem write producing four sibling artefacts)." This converts the SUBSTANTIVE #4 finding from "inconsistency" to "principle named, applied consistently."

- [partial] [specs/brownfield-tests/spec.md ADDED Requirement] Requirement spans two semantic axes. The single ADDED Requirement now carries (a) Phase 9 acceptance criteria coverage scenarios and (b) failing-state contract preservation scenarios. Splitting into two ADDED Requirements is structurally cleaner; declining the split because the proposal is already mid-debate is acceptable per OpenSpec convention. Fix: add a one-line scope note to the Requirement description: "This requirement covers both acceptance-criterion stub presence and the failing-state contract that those stubs must satisfy." No structural split.

- [defend] [tasks.md cross-PR error-code references] CC002 references at tasks.md:45 are forward-compatible against Bundle A's apply per the dependency declaration in proposal.md:7-8. Sauron confirmed this in his own NIT #7. No fix.

- [partial] [research/genesis.md L17-20] Genesis incomplete on source scenario count drift. The 23-scenario count was measured at sync-authoring time; the live count at apply time is 24. Fix: add to genesis L17-20 a re-survey directive: "Re-counting scenarios against the live source-of-truth at apply time is mandatory; the 23-scenario count was measured at sync-authoring time and may have drifted if the brownfield spec gains scenarios in subsequent debate rounds." Lightweight; honest.

- [defend] [tasks.md renumber] Section renumber from 7-Required-Verification to 11-Required-Verification preserves cflx checkbox detection. The regex at .claude/skills/cflx-proposal/scripts/cflx.py:359 matches `^\s*[-*]\s*\[([ x])\]\s+(.*)$` and is section-number-blind. Sauron verified the regex himself in his NIT #10. No fix.
```

---

## Verdict

**5 of 5 critical findings stand.** Path schism, arithmetic, missing scenarios, phantom mapping, and the doc-comment soft rim are all conceded (the soft rim is conceded as cheap polish, not as a real contract violation).

**1 of 2 substantive findings stands as PARTIAL** (#4 splitting principle: fix by stating the rule). The other (#6 two-axis requirement) is conceded as a smell with a one-line description fix rather than a structural split.

**3 of 3 minor findings are NIT-grade** (#7 paths in new sections clean per Sauron himself, #8 genesis re-survey directive added, #10 renumber-safe per Sauron himself).

**Em-dashes: zero. House style internalised.**

## Recommended Path Forward

A single fix-up commit on `openspec/phase-9.0-tests-wave4-sync`:

1. **Path schism**: replace `meta/changes/brownfield-init/` with `openspec/changes/brownfield-init/` at tasks.md:12 and design.md:91-93.
2. **Arithmetic**: rewrite proposal.md:41, tasks.md:69, spec.md:20 against the corrected total.
3. **Missing scenarios + phantom mapping**: delete `suggest__manual_test_entries_accept_empty_provenance`, add `suggest__refine_emits_suggested_edges_in_refine_change_dir` (Req 5 scenario 4) and `suggest__force_init_aborts_on_pending_entries` (Req 5 scenario 5). Update design.md coverage table, tasks.md Section 7, spec.md test-name list. New totals: 26 acceptance + 7 heuristic = 33 stubs.
4. **Splitting principle**: add the subsystems-vs-syscall rule to design.md Failing-State Stub Contract.
5. **Two-axis requirement**: add a one-line scope note to the ADDED Requirement description.
6. **Genesis re-survey directive**: add a single line to genesis L17-20.
7. **Doc-comments (optional polish)**: add `///` lines to the Sample Test Sketch and the Failing-State Stub Contract noting that each `unimplemented\!()` carries a doc-comment naming the asserted property; explicitly framed as redundancy with tasks.md, not as a second source of truth.

Estimated change size: under 50 lines added/removed across five files. One graphite-pr unit. Validate-strict should pass post-fix.

Convergence: yes, this debate converges with the fix-up commit. No fundamental disagreement remains.
