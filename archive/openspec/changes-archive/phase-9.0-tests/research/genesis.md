---
id: genesis-phase-9.0-tests
nodes: [phase-9.0-tests]
date: 2026-05-01
sources: []
informed_by:
  - docs/strongholds/getcairn-cross-check-integrated.md
  - docs/strongholds/oq4-phase9-rescope-timing.md
  - openspec/changes/phase-9-brownfield/specs/brownfield/spec.md
  - openspec/changes/phase-9-brownfield/research/genesis.md
type: genesis
---

## Summary

- This proposal-update syncs `phase-9.0-tests` with the Wave 4 rescope of `phase-9-brownfield`. Before this update the proposal claimed "ten acceptance-criterion scenarios across four requirements"; the rescoped brownfield spec carries 24 scenarios across 8 requirements (5 existing requirements unchanged, 3 net-new in Wave 4 plus 1 conditional). The count was measured at sync-authoring time and may have drifted; re-counting against live source-of-truth at apply time is mandatory. This pre-phase mirrors that shape as failing-state test stubs.
- Trigger event: PR #22 (Sauron/Saruman debate, CRITICAL #2 finding) flagged the self-contradicting state where the brownfield rescope had landed on `openspec/phase-9-brownfield-wave4-rescope` while `phase-9.0-tests` still asserted the pre-rescope count. The two proposals could not coexist on dev without one superseding the other; this update closes that gap.
- Timing: per `docs/strongholds/oq4-phase9-rescope-timing.md` (Option B refined) authoring this pre-phase happens after the rescope merges and before phase-9.0-tests applies (T+7 in the implementation specifics chronology). The rescope and this sync land as separate proposal-update commits on dev; phase-9.0-tests' apply waits until both have merged.
- Scope of edits: docs-only. `proposal.md`, `tasks.md`, `design.md`, `specs/brownfield-tests/spec.md`, and this `research/genesis.md`. No `src/` changes, no `tests/` changes.
- Stub count: from 17 (10 acceptance + 7 heuristic) to 33 (26 acceptance + 7 heuristic). The acceptance count exceeds the 24-scenario count because Req 5 scenario 1 carries three load-bearing assertions (queue write, pending state, trace phase) split across three stubs for clear failure isolation; the other 23 scenarios map one-to-one. The headline metric the proposal cites is the 24-scenario / 8-requirement shape mirroring the brownfield spec.
- Failing-state contract: each stub compiles cleanly, sits behind `#[ignore = "awaits phase-9"]`, and either calls `unimplemented!()` (when Phase 9 fixtures are not yet available) or asserts a property a not-yet-written feature satisfies. Either form yields a runtime panic when run with `--ignored`, not a compile error. This preserves the pre-phase-archives-green property of the original proposal.

## Transcript

This sync was authored autonomously by a Nazgul force per the CRITICAL #2 mission brief on PR #22. The mission supplied the source-of-truth pointer (the rescoped brownfield spec on `openspec/phase-9-brownfield-wave4-rescope`) and the timing stronghold; no live elicitation Q/A occurred. The transcript below records the key decisions made during authoring, in lieu of a turn-by-turn user dialogue.

**Q: Should every brownfield scenario become a single stub, or should multi-assertion scenarios split into multiple stubs?**

A: Multi-assertion scenarios split. Req 5 scenario 1 ("Suggest engine writes to the queue file") asserts three properties: the entry exists in the queue file, the entry's `triage_state` is `pending`, and the entry's `provenance.trace_phase` names the running phase. Splitting into three stubs (`suggest__engine_writes_to_queue_file`, `suggest__entry_triage_state_is_pending`, `suggest__entry_provenance_carries_trace_phase`) gives each property an isolated failure signal so Phase 9 can flip them off the ignore list one at a time as the engine fixture matures. Other scenarios map one-to-one because their THEN clauses are conjunctive but tightly coupled (e.g., Req 1 scenario 3's "creates change directory AND writes proposal AND writes blueprint.delta AND writes stub contracts AND does not write main blueprint" all hinge on a single init invocation).

**Q: How should stubs handle Req 8's conditional ("when the schema supports them") nature?**

A: Carry a guard comment naming the current schema state. The two field-present stubs (`obligations__populated_when_field_exists`, `obligations__reviewable_before_archive`) panic via `unimplemented!()` until Phase 9 either adds the field or confirms it stays absent. The field-absent stub (`obligations__no_op_when_field_absent`) asserts the no-op rider directly and is the always-callable branch. This mirrors the brownfield rescope's own conditional handling (per `phase-9-brownfield/research/genesis.md`) and preserves the failing-state contract in either schema branch.

**Q: Should the suggest engine fixture (queue file format, trace_phase resolver, CC002 gate hook) be authored in this pre-phase or deferred to Phase 9?**

A: Deferred. The pre-phase's job is failing assertions, not fixture infrastructure. Stubs that depend on the suggest engine fixture call `unimplemented!()` until Phase 9 supplies the fixture. This matches the original proposal's "no `src/` changes, no fixture authoring" boundary and keeps the pre-phase docs-and-test-stubs-only.

**Q: Does the renumbered task block (sections 7-10 added, 7 renumbered to 11) follow the existing numbering style?**

A: Yes. The original tasks.md ended at section 7 (Required Verification). Wave 4 task groups slot in as 7 (Suggest), 8 (Interview), 9 (Templates), 10 (Obligations); verification renumbers to 11. Task IDs continue the `<group>.<task>` pattern. The cflx checkbox detector at `.claude/skills/cflx-proposal/scripts/cflx.py:359` matches `^\s*[-*]\s*\[([ x])\]\s+(.*)$` and is section-number-blind, so the renumber is mechanically safe.

**Q: Should this update also fix any path drift in the existing tasks (e.g., `.agents/skills/` versus `.claude/skills/`)?**

A: The original `phase-9.0-tests/tasks.md` did not carry the `.agents/skills/` path drift that the brownfield rescope fixed in its own tasks.md task 6.6. Section 11.7 in the updated `phase-9.0-tests/tasks.md` cites the live `.claude/skills/...` path directly. No drift fix needed in this proposal.

**Q: Does this proposal-update need a Modified Requirements section, or are pure ADDED-spec edits sufficient?**

A: The original `specs/brownfield-tests/spec.md` had a single ADDED Requirement ("Phase 9 acceptance criteria have failing test coverage") with four scenarios. This update extends that requirement with two new scenarios (Wave 4 stubs preserve failing-state contract; Conditional obligations stubs reflect schema state) and grows the test-name list inside the existing fourth scenario. Because the requirement is still under ADDED status (the change has not yet archived), the edits are amendments to the proposed delta rather than MODIFIED operations on a consolidated spec. No Modified Requirements section needed.

**Q: Does the rescope-then-sync ordering risk validate-strict failures during the window?**

A: Per `oq4-phase9-rescope-timing.md` and `CLAUDE.md` ("intermediate broken states on a feature branch are acceptable"), per-change `cflx openspec validate <change> --strict` is the gate. The rescope's PR validates `phase-9-brownfield`; this PR validates `phase-9.0-tests`. The two PRs touch zero shared files. Order of merge does not matter; both must merge before phase-9.0-tests applies. The validate-strict failure window during the docs-staleness gap (between rescope merge and this sync merge) is bounded and acceptable.
