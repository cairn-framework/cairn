# PR #22 (Wave 4 rescope) — Sauron critique

**Subject**: `openspec/phase-9-brownfield-wave4-rescope`, commit `d2a3ee7`
**Reviewer**: Sauron, the Lidless Eye
**Date authored**: 2026-05-01
**Methodology**: structural read of the five rescoped files plus cross-references against `openspec/changes/phase-7.6-ai-provenance-foundation/design.md`, `openspec/registries/error-codes.md`, `openspec/conventions.md` §9, the live phase-9.0-tests proposal, the timing stronghold (`oq4-phase9-rescope-timing.md`), the integrated cross-check (`getcairn-cross-check-integrated.md`), and the dev-branch git log.

Severity: BLOCKING means the rescope cannot ship as-is without correction. SUBSTANTIVE means a defensible defence is possible but the surface is weak. NIT is style or polish.

---

## 1. Output-path schism between `meta/changes/` and `openspec/changes/` — BLOCKING

The rescope contradicts itself on the most basic question: where does brownfield output land.

- `proposal.md:22` says: `cairn init --from-code` writes to `openspec/changes/brownfield-init/`.
- `design.md:46`: `cairn init --from-code SHALL create meta/changes/brownfield-init/`.
- `design.md:53`: re-affirms `meta/changes/brownfield-init/`.
- `specs/brownfield/spec.md:27,35,42`: scenarios pin the path as `meta/changes/brownfield-init/`.
- `proposal.md:34` and `design.md:9,79`: the suggest-engine queue file lives at `openspec/changes/<change>/suggested-edges.json`.
- `specs/brownfield/spec.md:103,151`: same.

Bundle A's design.md L82 fixed the path explicitly: `openspec/changes/<change>/suggested-edges.json` and stated "the path matches the live tree (which uses `openspec/changes/`, F3); the file is a sibling of `proposal.md`, `blueprint.delta`, `design.md`, and `tasks.md`." If the rescope's output lands in `meta/changes/brownfield-init/` per its own design.md and spec, then the suggest-engine queue file cannot simultaneously be a sibling at `openspec/changes/<change>/suggested-edges.json`. Either:

1. The init flow lands under `openspec/changes/`, in which case `design.md:46`, `design.md:53`, and three spec scenarios are wrong, or
2. The init flow lands under `meta/changes/`, in which case the suggest-engine queue file path conflicts with Bundle A's ratified location, breaking the entire premise that this rescope "consumes Bundle A's identifiers."

The rescope's own genesis.md L16 claims it cites "the suggested-edges.json [file]" by its ratified name, but it has actually preserved the old phase-9 path drift (Bundle A's design.md F3 explicitly flagged `meta/changes/` vs `openspec/changes/` as a pre-existing inconsistency the cleanup phase has not addressed). The rescope inherited the drift and hid it under a fresh proposal-update commit. This is the cleanup-deferred-to-future-phase posture being smuggled into a phase that explicitly claims to "make the plan honest before code lands" (genesis.md L17).

**Suggested fix**: Decide one path. If `openspec/changes/`, change `design.md:46`, `design.md:53`, `specs/brownfield/spec.md:27,35,42` to match. If `meta/changes/`, the rescope is blocked on the F3 cleanup phase landing first, and the timing stronghold's Option-B trigger has been mis-evaluated.

---

## 2. phase-9.0-tests is now structurally stale — BLOCKING

The timing stronghold (`oq4-phase9-rescope-timing.md`) Option B steelman-against case warned of exactly this:

> "phase-9.0-tests' proposal references 'ten acceptance-criterion scenarios across four requirements' derived from Phase 9's *current* proposal. If Phase 9 rescopes after Bundle A and adds new requirements (suggest-engine acceptance, interview-runner acceptance, templated-authoring acceptance), phase-9.0-tests is already-authored against the wrong scope; phase-9.0-tests itself needs updating to match the rescoped Phase 9."

The Option-B verdict acknowledged this and stated the sequencing rule: "Phase 9 rescope before 9.0-tests apply." The rescope has shipped, and phase-9.0-tests has NOT been touched.

Live state on the rescope branch:

- `openspec/changes/phase-9.0-tests/proposal.md:13`: still claims "ten acceptance-criterion scenarios across four requirements." The rescoped spec contains 23 scenarios across 8 requirements (`### Requirement` count 8; `#### Scenario` count 23).
- `openspec/changes/phase-9.0-tests/specs/brownfield-tests/spec.md` has 4 scenarios; the test wall has not grown to absorb the 13 new scenarios the rescope introduces.
- `cflx.py validate phase-9.0-tests --strict` passes, because validate is per-change and does not cross-check sibling phases. The green checkmark is theatre.

The rescope is incomplete by its own timing stronghold's verdict. Either phase-9.0-tests gets a sibling proposal-update commit in the same rescope-PR, or the apply-time agent for phase-9-brownfield will encounter a test wall that fails to assert 13 of its acceptance scenarios.

**Suggested fix**: Add `openspec/changes/phase-9.0-tests/proposal.md`, `tasks.md`, and `specs/brownfield-tests/spec.md` updates to this PR. Acceptance: phase-9.0-tests scenarios match phase-9-brownfield scenarios 1:1, and proposal.md text says 23 across 8.

---

## 3. Bundle A has NOT shipped; the rescope's premise is false — BLOCKING

The rescope cites Bundle A as having "ratified the identifiers" and as having "merged on PR #18" (proposal.md L30, genesis.md L16).

Live evidence from `git log --oneline`:

```
d7797b1 openspec: draft phase-7.6-ai-provenance-foundation proposal (#18)
```

PR #18 is a **proposal draft commit**, not an apply or archive. The capability area `openspec/specs/provenance-foundation/` does NOT exist on dev (or on this branch). `CC002` is NOT registered in `openspec/registries/error-codes.md` (the registry has the `CC -- Changes` section but no `CC002` entry; `CC001` allocation is also not in the file though phase-7.5c was supposed to have allocated it). The `suggested-edges.json` file class has not been implemented. The `cflx openspec validate <change> --strict` gate against `pending` entries does not exist as code.

The rescope is treating Bundle A's design.md authoring as "ratification." Per the timing stronghold's own Option B refined verdict that is the correct trigger — but the proposal language is sloppier than the stronghold. proposal.md L30 says "Phase 7.6 (Bundle A) merged on PR #18; its design.md ratified the identifiers" and genesis.md L16 says "Phase-7.6 has merged in full (PR #18) so this question is moot." Neither is true. Phase-7.6's draft proposal merged. Implementation has not.

This is not a fatal problem under the timing stronghold's Option-B-refined verdict (which only requires design.md to be ratified, which it is, since the design draft is on dev). It IS a problem because:

1. The proposal text mis-describes the dependency state. A reader cannot tell whether Bundle A's apply has shipped or only its design.
2. The dependency declaration on proposal.md L8 says phase-7.6 is "a required dependency" without specifying which lifecycle stage of phase-7.6 must complete before phase-9-brownfield's apply runs. Per cflx vocabulary the dependency must specify whether design.md ratification or apply-archive is the trigger. The current phrasing implies the latter; the timing stronghold's verdict is the former.
3. The Acceptance Criteria L47 says "archive is blocked by `CC002` until every entry is triaged off `pending`." `CC002` does not exist as a registered code today. The acceptance criterion is unverifiable until phase-7.6 actually applies and registers the code.

**Suggested fix**: Rewrite proposal.md L30 to say "Phase 7.6's design.md ratified the identifiers" without claiming PR #18 was a full merge. Tighten the dependency declaration on L8 to specify "phase-7.6-ai-provenance-foundation apply must precede phase-9-brownfield apply" since the C8.c suggest engine code depends on the suggested-edges file class code that phase-7.6 ships. Note that the rescope itself can land before phase-7.6 applies (it is docs-only), but the apply-time prerequisite is the apply, not the design.

---

## 4. Genesis transcript cites a file that does not exist — SUBSTANTIVE

`research/genesis.md:24` states: "This rescope was authored autonomously by a Nazgul force per the Wave 4 mission brief in `docs/strongholds/session-handoff.md`."

Live filesystem state: `docs/strongholds/session-handoff.md` does not exist on dev (it is listed as an UNTRACKED file in the git status snapshot but is absent from the worktree). The genesis transcript's authority for the entire rescope is a file that is neither committed to the repo nor present on disk.

Per `openspec/conventions.md` §9, the genesis file "provides human-readable and codex-readable provenance for the change's elicitation history." If the elicitation history points at a non-existent file, the provenance chain is broken at the source.

Two failure modes:

1. The session-handoff.md was deleted before commit. In that case the genesis transcript should not cite it; cite the live strongholds (`getcairn-cross-check-integrated.md`, `oq4-phase9-rescope-timing.md`) which ARE committed and DO contain the Wave 4 mission shape.
2. The session-handoff.md was meant to ship in this PR. In that case the rescope is missing a file.

The genesis transcript also confesses (L24) that "no live elicitation Q/A occurred" and the Q/A turns are reconstructed decisions. This is a defensible practice for a rescope (the cflx-proposal skill's interview mode is itself the absorbed C1.b sub-component, so the absorbing rescope cannot use the absorbed runner), but the transcript should label the Q/A turns as "post-hoc decision record," not "transcript." Conventions §9 does not explicitly forbid synthetic transcripts, but the convention's purpose ("elicitation history") is undermined when the elicitation never happened.

**Suggested fix**: Either commit the session-handoff.md to the rescope-PR (it is referenced in `docs/strongholds/session-handoff.md` per the genesis frontmatter), or remove the citation from genesis.md L24 and replace with citations to the actually-committed strongholds. Add a one-line preface to the Transcript section labelling it "synthesized decision record (no live Q/A)" so future readers do not assume a real interview occurred.

---

## 5. Acceptance Criteria carry subjective verbs — SUBSTANTIVE

`proposal.md` Acceptance Criteria section (L40-L50):

- **L48** "Interview runner sessions resume from a partial state without losing earlier turns." "Without losing" is the failure mode being asserted, not a check. Mechanically: assert that the persisted session file `interview-session.json` contains every answered turn from invocation N at the start of invocation N+1. The criterion as written is unfalsifiable in a fixture test.
- **L49** "Templated authoring resolves project-declared templates against generated stub contracts and falls back to the built-in stub when no template matches." Two predicates conjoined; the resolve-success path needs an explicit content-equivalence check (does the generated stub contain the template's required headers).
- **L50** "AI-suggested decisions populate the field and the field is reviewable in the generated change directory before archive." "Reviewable" is undefined. Does this mean "readable as text in the file"? "Editable through some tool"? "Surfaced in a CLI dump"? A mechanical test cannot decide.
- **L42** "Generated candidates include nodes, paths, stub contracts, and obvious edges." "Obvious" is the load-bearing word. Per `design.md` §Candidate Extraction L37, edges are emitted "when there are at least two import observations from one candidate to another, or one public API reference with high confidence." That is mechanical. Replace "obvious" with the threshold language from design.md.
- **L41** "Brownfield init never writes directly to main `cairn.blueprint` or main `meta/` artefacts." This conflicts with the path-schism finding (#1): if generated output lands in `meta/changes/brownfield-init/`, that IS under main `meta/`. The "main `meta/` artefacts" carve-out is doing implicit work that the criterion does not state. Mechanical check: the absolute-path predicate.

**Suggested fix**: Rewrite each subjective criterion to a mechanical predicate per the existing design.md numeric thresholds. Resolve the path schism first (#1) so L41 is well-defined.

---

## 6. ADDED-vs-MODIFIED Requirements decision is under-justified — SUBSTANTIVE

The genesis transcript's final Q (L50-L52) asks whether the rescope needs `## MODIFIED Requirements` and answers "Pure ADDED is sufficient." The justification is that the existing "Keep human review authoritative" requirement covers the suggest engine's gate semantics at the abstract level.

This is wrong on two counts:

1. The existing requirement (`specs/brownfield/spec.md:66-75`) has one scenario: "False positive can be removed." Its abstract statement is "Brownfield output SHALL remain proposed until archived." The new "Suggest cross-cutting edges through the phase 7.6 queue" requirement (`specs/brownfield/spec.md:95-127`) introduces a stricter gate: pending entries block archive via `CC002`. This is a strengthening of "remain proposed until archived" from "passive proposed-state until archive" to "actively-blocking pending-state until human triage." The new requirement narrows the existing one. Per OpenSpec conventions, narrowing an abstract requirement should produce a `## MODIFIED Requirements` entry that adds the new scenario to the existing requirement, not a parallel ADDED requirement that talks past the original.

2. The "Force replaces generated change" scenario at `specs/brownfield/spec.md:40-45` says force "replaces the existing generated brownfield change directory atomically." If the suggest-engine has emitted pending entries in that directory and a `--force` re-init wipes them, the human triage queue is destroyed silently. This interaction is not addressed in either requirement. A `## MODIFIED Requirements` entry to "Generate initial Cairn state from code" would naturally fold in a "Force preserves or warns about pending suggested-edges entries" scenario.

**Suggested fix**: Convert one or both new requirements to `## MODIFIED Requirements` entries that explicitly extend the existing abstract statements. Add a force-vs-pending-queue interaction scenario.

---

## 7. C4.b conditional follow-on muddies the validate-strict surface — SUBSTANTIVE

The C4.b decision-attached-obligations sub-component is conditional on the decision schema growing an `obligations` field. The rescope handles this by:

- Three scenarios in `specs/brownfield/spec.md:179-202`, two of which require the field to exist, one of which (L197-202) asserts a no-op when the field does not exist.
- `tasks.md` section 8: tests stay `#[ignore]` if the schema does not grow the field.
- `design.md:115-122` says the conditionality is captured "with a 'WHEN obligations field is present' guard so dormant scenarios do not regress validate-strict."

The spec scenarios as written do NOT carry a "WHEN obligations field is present" guard in their GIVEN clauses. Scenario "Obligations are populated when the field exists" (L183) has GIVEN "decision artefacts in this phase declare an `obligations` field" — that IS a guard. Scenario "Obligations population is a no-op when the field is absent" (L197) has GIVEN "decision artefacts in this phase do not declare an `obligations` field." That covers the dormant case mechanically.

But: the rescope's design.md L121 promises a guard syntax that the spec does not actually use; the spec uses parallel scenarios for the two conditional cases. This is fine in practice but contradicts the design.md's own description. The reader of design.md will look for "WHEN obligations field is present" in the spec and not find it. Inconsistency between design and spec is an integrity smell.

Worse: at the moment the rescope ships, no decision is known about whether the schema will grow the field. The phase-9-brownfield apply will encounter this conditionality at runtime and have to decide which branch to take. The decision needs to be made by the apply agent, not by the rescope. That is acceptable BUT the rescope should explicitly document who makes the decision and when. The current language ("if Phase 9 ships an `obligations` field") leaves the decision ungrounded.

**Suggested fix**: Either align design.md L121 with the spec's actual parallel-scenarios approach (drop the "WHEN guard" claim) or rewrite the spec scenarios to use a conditional guard syntax that openspec's validator actually understands. Add a sentence to design.md naming the decision-maker for the conditional (the apply agent at first commit, with a documented criterion).

---

## 8. Suggest-engine empty-provenance scenario re-asserts an upstream invariant — SUBSTANTIVE

`specs/brownfield/spec.md:121-127` Scenario "Manual-test entries leave provenance empty":

> GIVEN a brownfield change whose suggested-edges file is authored manually for testing
> WHEN the change directory is read by the cairn library
> THEN entries with no producing trace context are accepted with an empty `provenance` object
> AND the schema-version check still passes for the file

This scenario is asserting a property of phase-7.6's `SuggestedEdges` library reader (specifically: optional-`provenance` schema discipline). It is not a property of the brownfield generator. Phase-7.6's design.md L110 already states "It [the provenance object] is empty for entries authored manually for testing." Phase-7.6's tests (per its design.md L188) cover round-trip with one entry per `triage_state` value.

The rescope is duplicating an upstream test under the brownfield spec. If phase-7.6's reader regresses, both phase-7.6 tests AND the brownfield scenario fail; the redundancy is wasteful. If phase-7.6's reader is correct, the brownfield scenario is asserting nothing the brownfield code did.

**Suggested fix**: Drop the "Manual-test entries leave provenance empty" scenario from the brownfield spec. The empty-provenance discipline belongs in `specs/provenance-foundation/spec.md` once phase-7.6 applies. The brownfield spec should only assert what brownfield CODE does: emit populated provenance.

---

## 9. Atomic-commit grouping in design.md is decorative, not load-bearing — SUBSTANTIVE

`design.md:123-132` "Wave 4 atomic-commit grouping" describes four groups (E, F, G, H) and says "The cflx-runner enforces group-level boundaries when the phase declares them." Comparing to existing phase precedent:

- Phase 2.6 (per CLAUDE.md): the apply commit grouped task 2.1-2.5 + 3.1 atomically because the rename is a single semantic unit.
- Phase 7.6 design.md L196-201: atomic groups are A (sidecar + CLI), B (queue), C (validate-strict gate, depends on B), D (islands query). Each group has internal coupling that demands atomic commit.

The Wave 4 groups as written:

- Group E (suggest engine, section 5): five tasks in one group. Internal coupling is high; atomicity is justified.
- Group F (interview runner, section 6): five tasks; atomicity defensible.
- Group G (templated authoring, section 7): four tasks; atomicity defensible.
- Group H (obligations, section 8): three tasks, conditionally a no-op; atomic OR dormant.

The problem: design.md L132 says "The four Wave 4 groups commit in any order with respect to Groups A through D from the original phase scope." But the original phase scope's tasks 1-4 do NOT have declared atomic groups in tasks.md. The rescope is inventing Groups A-D retroactively without changing tasks.md to declare them. The original phase 9 task list is task-numbered (1.1-1.5, 2.1-2.4, 3.1-3.5, 4.1-4.4), with no atomic-group annotation. design.md's promise that Wave 4 groups slot into a non-existent A-D structure is a fiction.

**Suggested fix**: Either declare Groups A-D explicitly in tasks.md (each pre-existing section becomes a group) or rewrite design.md L123-132 to drop the A-D framing and only describe Wave 4's E-H. The current language asserts a structure tasks.md does not encode.

---

## 10. C15 templated-authoring scope rests on a config-surface decision deferred to apply — SUBSTANTIVE

`design.md:105` says: "a new `[templates]` block in `cairn.blueprint` (or an equivalent project config file, design choice deferred to apply) declares templates."

Deferring a config-surface decision to apply is the same fragility class the timing stronghold rejected for Option A (rescoping before Bundle A's design.md ratifies identifiers). The rescope is committing the C15 scope while leaving the surface where users author templates undecided. Apply-time will face the same coordination problem: template config can land in `cairn.blueprint`, in a sibling file, in `[project]` section of an existing config — the choice affects every downstream tool that reads the templates.

`tasks.md:51` repeats this: "Define a project-config surface (`[templates]` block in `cairn.blueprint` or equivalent)."

The rescope's premise was that Bundle A's design.md fixed enough identifiers to make Phase 9's contract honest. The C15 contract is NOT honest while the surface is "blueprint or equivalent."

**Suggested fix**: Pick a surface in this rescope. If `cairn.blueprint`, add a one-line schema-extension note to the spec (and reconcile with the blueprint parser's grammar). If a sibling file, name it. The phrase "or equivalent" should not appear in a contract document.

---

## 11. Refine + suggest-engine interaction is unspecified — SUBSTANTIVE

`tasks.md:36` says the suggest engine consumes "bounded code samples and structural candidates from section 1 plus summariser output from section 2." Section 1 is the discovery mode used by `init --from-code`. Section 4 (Refine) re-uses the same discovery pipeline against an existing blueprint.

Unanswered: when `cairn refine` runs, does the suggest engine fire? If yes, the refine output's change directory needs to track a `suggested-edges.json` file too, and the `CC002` gate applies to refine archives equally. If no, refine cannot capture cross-cutting edges that emerge as code grows, defeating the purpose of refine for AI-assisted authoring.

The rescope does not say. `proposal.md` L34 says the engine emits into `openspec/changes/<change>/suggested-edges.json` without distinguishing init from refine. `design.md` C8.c section talks only about `cairn init --from-code` and mentions refine in the `provenance.stage` field discussion.

**Suggested fix**: Add an explicit sentence to design.md C8.c: "The suggest engine fires for both `cairn init --from-code` and `cairn refine`. Refine-emitted entries set `provenance.stage = "propose"` against the refine-time change directory and are subject to the same `CC002` gate." Add a refine+suggest scenario to the spec.

---

## 12. The dependency on phase-7.6 has the wrong execution-order claim — SUBSTANTIVE

`proposal.md:10` states: "Execution: MUST run after Phase 8 and Phase 7.6, and before Phase 10."

Phase-7.6 is numbered 7.6, before phase-8-summariser (numbered 8). The current phase-9-brownfield depended only on phase-8-summariser; the rescope adds phase-7.6 as a hard predecessor. Per the timing stronghold's verdict, the apply-stage predecessor relationship is correct (the suggest engine code depends on the suggested-edges queue file class shipped by phase-7.6). But "after Phase 8 and Phase 7.6" omits the relative ordering of phase-7.6 vs phase-8.

If phase-7.6 ships before phase-8 (per phase number), the constraint reduces to "after Phase 8." If phase-7.6 ships after phase-8 (per Wave 2 of the integrated plan, which puts Bundle A in Wave 2 and phase-8 was already-applied per the original phase ordering), then "after Phase 8 and Phase 7.6" is redundant in one direction.

`getcairn-cross-check-integrated.md:181-185` Wave map shows phase-9-brownfield apply in Wave 5, after phase-9.0-tests apply, after Phase 9 rescope (Wave 4), after Wave 3 sweeps. Phase-7.6 (Bundle A) ships in Wave 2, well before. So "after Phase 8 and Phase 7.6" is true but uninformative. The substantive ordering is "after Wave 4 rescope" and "after phase-9.0-tests apply."

**Suggested fix**: Tighten proposal.md L10 to "Execution: MUST run after Phase 8 apply, after `phase-7.6-ai-provenance-foundation` apply, after `phase-9.0-tests` apply, and before Phase 10." Drop the redundant ordering claim and add the apply-stage qualifier.

---

## 13. No em-dashes — clean

Grep against U+2014 across all five files: zero hits. The wizard remembered one rule.

---

## 14. NIT: Wave 4 acceptance criterion cluster is bottom-stacked

`proposal.md:47-50` puts the four Wave 4 criteria at the end of the Acceptance Criteria list. This is fine for a diff-friendly append-only edit but obscures the absorbed-scope share of acceptance. A reader scanning for "what does Wave 4 require to ship" sees the original criteria first and the Wave 4 criteria as an afterthought.

**Suggested fix**: Group Wave 4 criteria under a clearly labelled subsection ("### Wave 4 acceptance criteria" inside Acceptance Criteria). Pure formatting; non-blocking.

---

## 15. NIT: Out-of-scope list duplicates phase-7.6 rejections without citing them

`proposal.md:57-58` lists "Auto-accept policies on suggested edges" and "Interactive `cflx triage-edges <change>` command" as Out of Scope. Both are also Out of Scope per phase-7.6's design (phase-7.6 design.md L146-147). The rescope cites this in the "Out of Scope" entries, which is honest. The redundancy is fine; it makes phase-9 self-contained for readers who never load phase-7.6's design. Leave as-is.

---

## Convergence note

The rescope's overall shape is correct: docs-only, four absorbed sub-components, dependency on phase-7.6 declared, validate-strict passing. The four sub-components do belong in phase-9-brownfield per the integrated cross-check's Pattern 3.

The defects are coherence defects, not concept defects. The path-schism (#1), phase-9.0-tests-desync (#2), and Bundle-A-not-actually-shipped (#3) are independent BLOCKING issues that each require a fix in this PR before merge. The SUBSTANTIVE issues (#4-#12) are individually defensible but together suggest the rescope was authored against an idealised version of the integrated plan rather than the live filesystem state.

A defending wizard will argue:
- Path schism is "F3 cleanup deferred per Bundle A's own design." Counter: the rescope explicitly claims to make the plan honest; deferring path coherence to a future cleanup phase contradicts that claim.
- phase-9.0-tests update is "out of scope for this rescope." Counter: the timing stronghold's Option-B verdict explicitly named this as a sequencing dependency; not addressing it leaves the test wall structurally insufficient.
- Bundle A "shipped" because design.md ratified. Counter: there is a gap between identifier ratification and code-shipping; the rescope conflates them in proposal.md L30 and genesis.md L16.

The defence on each is technically possible but each requires the wizard to retreat to a narrower scope for the rescope than the proposal claims. The PR as it stands should not merge until at least #1, #2, and #3 are resolved or explicitly scoped out with a changelog entry.

