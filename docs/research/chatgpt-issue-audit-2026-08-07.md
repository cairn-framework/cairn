# Cairn open-issue audit (verbatim external document)

Produced by ChatGPT at the maintainer's request; handed over in-session
2026-08-07. Preserved verbatim (em-dashes included; this directory is
exempt). Provenance artefact: `src.chatgpt-issue-audit`. Capture with a
staleness record (staleness verified, dispositions as received):
`res.chatgpt-issue-audit`.

---

## Verdict

Cairn's backlog is **mostly directionally sound**, but the active issue set overstates the amount of real work because it mixes:

* current product and integrity defects;
* completed work whose issue was never closed;
* umbrella trackers whose child issue now owns all remaining implementation;
* blocked ideas whose unblock condition does not exist;
* broad speculative projects that should return only when evidence triggers them.

My recommended disposition across the **42 open mirrored issues** is:

| Disposition                                |  Count |
| ------------------------------------------ | -----: |
| Keep substantially as written              | **22** |
| Keep, but amend scope/status/relationships | **10** |
| Close or replace                           | **10** |

The interpretation standard was the accepted mission: extreme agent automation must still produce software that is maintainable, investigable, extendable, and fit for purpose.

The newer architecture direction is also clear:

* Cairn's core remains a passive declarative substrate.
* Cairn owns selection truth, policy, readiness and recorded facts.
* A driver owns assignment, scheduling, retries and supervision.
* A harness executes.
* The driver may move into the repository as a separate layer only after `dec.orchestration-placement` is signed.

The canonical records are the native todo files, not the GitHub issues, so these changes should be applied under `meta/todos/` and then projected outward.

---

## Close, drop, or replace

| Issue | Recommendation | Reason |
| --- | --- | --- |
| #582 Roadmap Assumption Audit | **Close after this audit is encoded** | This issue asks for exactly the sweep now performed: inspect every open todo after the relationship, console, briefing and provenance campaign, then correct or close stale work. It should be the finite audit record, not an evergreen tracker. |
| #425 UI Asset Refresh | **Close as obsolete/completed** | Its named blockers, pending briefings and reverse provenance, have landed. The current README and landing page no longer consume the old `webui.gif` or `webui.mp4`; a recording script now exists if media is needed again. Delete orphan media/script through a concrete cleanup finding rather than leaving this blocked issue alive. |
| #298 Positioning: outcome-first copy | **Close as delivered and partially superseded** | The README now opens with session pain and user outcomes, while the current landing hero opens with code drifting from the plan. The mechanism is explained below. The old requirement to make "Star us on GitHub" the primary CTA has been superseded by the newer marketing direction and setup CTA. |
| #338 Repo organisation cleanup | **Close after filing concrete findings** | It is an unbounded "find miscellaneous mess" umbrella, now duplicated by #582's specific health audit. Concrete dead files, branches, duplicate keys or stale artefacts should each become bounded defects with acceptance criteria. |
| #482 Parent/child package cycle | **Close as an exhausted umbrella** | Its governing decision is accepted and the remaining executable implementation is wholly owned by #504. Keeping both open duplicates one outcome without adding an independently landable unit. |
| #560 Ghost-anchored todos guidance | **Close the current rule** | Requiring a ghost architecture node whenever a future-capability todo is authored conflates the work graph with the structural blueprint and forces architecture to be guessed before its decision is accepted. The driver work itself correctly anchors at the root while placement remains proposed. A narrower future rule could say: once structure is decided, planned-but-unbuilt structure should be declared as ghosts. |
| #498 Revisit-trigger correlator | **Close as a dormant non-work tracker** | Its body explicitly says it is "a record, not a work item," the required semantic capability does not exist, and the accepted deferral decision plus standing Info finding already preserve the obligation. Refile when the named semantic gate actually exists. |
| #380 Update awareness | **Drop from the active backlog** | It introduces release discovery, network failure handling, caches, acknowledgement state and installer-channel mutation without a recorded user failure or adoption experiment showing version skew is currently material. Add a revisit trigger based on observed support incidents or incompatible client/server versions. |
| #526 Lint selection folding | **Close and split one focused successor** | The parked-todo classification and strict-green Info folding already landed. The remaining decision-side `defers:` field is unratified and has no demonstrated live case. The only current problem is decision-accumulation signal quality; file that as a small standalone issue and drop the unused schema expansion. |
| #527 Local gate attestation | **Close and replace with toolchain/gate parity** | Its own measurements refute the primary premise: a perfect local receipt would save roughly 45 seconds, while CI was already around 159 seconds. Phase 1 remains valid—pin the toolchain and reconcile local/CI commands—but Phase 2's signing and receipt infrastructure has not earned its complexity. |

### Replacement issues to file

Only two new units are justified by these closures:

1. **Decision-accumulation signal correction** — determine whether the finding should count live binding authority, use node-role thresholds, or be withdrawn.
2. **Hermetic gate parity** — pin Rust and other tool versions, reconcile local and CI commands, and remeasure. No attestation or CI-skipping work until a recorded threshold fires.

---

## Keep, but repair the issue first

| Issue | Required correction |
| --- | --- |
| #584 Todo taxonomy | **Keep, but do after this cleanup.** Derive a small stable set of kinds from the cleaned corpus before changing schema. The likely set is defect, feature, decision gate, research/evaluation, audit and operational milestone. Avoid encoding temporary workflow states as types. |
| #579 Driver in repo | **Keep blocked as the programme epic.** Do not implement until `dec.orchestration-placement` is accepted. After signature, split blueprint/layer structure, workflow artefacts and first dispatch lifecycle into separate PR-sized children. |
| #575 Console signed widening | **Keep blocked, make it a child of #579.** Its earlier control-plane blocker is resolved; its only live gate is orchestration placement. The console must steer the driver and record sanctioned rulings, never acquire dispatch authority itself. |
| #562 Parallel dispatch granularity | **Keep and re-parent under #579.** Replace stale "driver-v2" terminology. Preserve the three-rung distinction: ordering, advisory overlap and actual merge-safety. It should remain research/design until a driver exists to consume the result. |
| #559 Build/CI observation overlay | **Keep as a later bounded experiment.** Remove the relationship-schema blocker because that implementation landed. Keep it blocked behind #562 or the equivalent evidence/index design. |
| #543 Review gate machine check | **Keep, but subordinate it to declarative workflows.** The evidence should be a normal workflow outcome consumed by the driver, not a bespoke "two reviews happened" protocol embedded in Cairn's core. |
| #563 Receipts provenance interop | **Change from blocked to open.** It is a bounded research comparison, has no real blocker, and becomes more valuable as receipt vocabulary starts supporting additional workflows. It should happen before that vocabulary spreads further. |
| #335 Typed relationships and GitHub links | **Unblock and update the body.** PR #570 shipped the typed relationship schema, link/unlink verbs, validation and wire surfaces. The issue now owns only deterministic two-phase GitHub relationship projection. |
| #305 WebUI design quality | **Change from blocked to open and narrow it.** The deterministic visual harness and `ux_defect_score` exist. The remaining work is specifically the Bet D proxies—layout-dimensionality, dead-zone, brand language and motion-safe affordance metrics—plus the required human visual-verification step. |
| #291 Guard against stack-merge code drops | **Keep the defect, rewrite the task.** The current "investigate a hook" body is not executable. Reframe it as exact-head verification: build the actual merge candidate, assert expected artefacts/paths survive, and run the final-tree gates against that candidate rather than trusting constituent PR heads. |

Recent merged work makes these corrections necessary: typed relationships landed in #570, the read-only console in #572, pending briefings in #574 and reverse provenance in #576.

---

## Keep: these earn their place

### Adoption and first-use defects

| Issue | Why it earns its keep |
| --- | --- |
| #578 Brownfield init review handoff | **Highest-priority product defect.** The advertised cold-start path ends without review instructions, ignores `--json` on one branch and presents structural confidence too much like architectural correctness. |
| #580 Module-size adopter scope | Backed by real MAG onboarding evidence: 30 of 32 findings were Cairn's own 500-line house rule rather than reconciliation information. This directly harms brownfield signal quality. |
| #367 Init-time ignore scaffolding | Prevents hundreds of first-run orphan warnings and is already part of the specified onboarding path. |
| #558 Brownfield decision extraction | Addresses the first external user's central value complaint: Cairn could create machinery but offered no path to extract existing invariants and decisions from a real codebase. |
| #504 Brownfield nested-package scan | A reproduced brownfield correctness issue with an accepted governing decision and detailed adversarial acceptance cases. |
| #280 AutoDocs Arm A stress test | Retains value as a real polyglot, nested-layout brownfield stress case. Keep it deferred and consider making it one case in a broader frozen brownfield corpus rather than a competitor-centred programme. |

### Integrity and silent-failure defects

| Issue | Why it earns its keep |
| --- | --- |
| #546 Ratification candidate pointer | A non-default decisions directory can silently bypass the local ratification gate. This is a genuine security/integrity boundary failure. |
| #583 Status todo path double prefix | Reproduced human and JSON wire inconsistency between `status` and `todos`; small, concrete and externally visible. |
| #478 Map snapshot freshness gate | A stale committed `map.json` already merged while every gate passed. The issue has a concrete regression case and clear ownership. |
| #538 Spec-rules reanchor | The registry's normative line anchors are systematically stale, undermining exactly the investigability Cairn claims to provide. |
| #375 Hook remediation pointer | Very small change that turns a blocking dead end into an actionable repair path using an engine Cairn already has. |
| #378 Tag registry | Some supposedly free-form tags already change behaviour; typos silently alter enforcement. An opt-in registry preserves extensibility while making behaviour-affecting vocabulary visible. |

### Agent investigation and external contracts

| Issue | Why it earns its keep |
| --- | --- |
| #458 Node symbol coverage | Measured retrieval failure: Rust symbol queries had zero useful coverage because navigation reused an exportability predicate intended for interface hashing. |
| #379 Wire-format schemas | Real consumers exist—MCP, web UI, harnesses, LSP and scripts—and many response labels remain unschematized. Continue in bounded batches rather than one large burn-down. |
| #296 Node-overlap conflicts query | Provides the honest rung-2 advisory signal for concurrent work without pretending to guarantee merge safety. It is a useful query regardless of eventual lease design. |
| #573 Decision-to-todo unblock edge | The current relationship vocabulary still cannot express that a decision unblocks a todo. The live orchestration-placement gate demonstrates the need. |
| #334 Full GitHub issue body | Not optional backlog polish: accepted task-tracking authority explicitly requires full-body projection and records that it is still unimplemented. It would also make future issue audits materially easier. |

### Evaluation, communication and release discipline

| Issue | Why it earns its keep |
| --- | --- |
| #369 Blueprint authorability evaluation | Measures whether agents can author Cairn's formats through production validators. Its fixture prerequisite is now satisfied, making it executable. |
| #525 Greenfield completion evaluation | Tests the central claim that Cairn produces a better completed project, with anti-gaming oracles and deterministic sensors. Keep at release cadence because it is expensive. |
| #301 Plain-language terminology | Direct owner feedback shows the vocabulary is still too hard to understand. Start with a terminology ruling/table, then update surfaces; do not begin with a repository-wide copy rewrite. |
| #552 Next milestone release | A finite, condition-based operational tracker. Keep it, but refresh its "landed since v0.9.0" inventory immediately before the release rather than maintaining that list continuously. |
| #581 Orca plugin triage | Bounded evaluation of an existing unstructured experiment. It should happen promptly because the work is currently an uncommitted local worktree; preserve its diff in a branch or patch before judging it. |

---

## Recommended execution order

### 1. Make the backlog truthful

Apply the dispositions above to the canonical todos, fix stale blockers and relationships, then close #582. This should include a branch inventory that distinguishes:

* merged historical residue;
* branches with unique commits;
* local-only experimental work;
* branches safe to delete.

There are currently no open PRs, so branch names should not be interpreted as active implementation merely because they still exist.

### 2. Preserve and triage the Orca experiment

Commit or otherwise preserve the current diff, then run #581. It may contain useful capability ideas, but nothing should be adopted directly into the core or CLI merely because an experiment already wrote it.

### 3. Resolve the binding placement decision

The next genuine programme fork is `dec.orchestration-placement`:

* **Accept:** begin decomposing #579 and #575, with workflows and driver code in a separate layer above the passive core.
* **Reject:** close or move those implementation issues outside the repository and retain Cairn as the control substrate only.

Until that ruling, building the driver would be premature.

### 4. Fix the adopter-facing failures

Recommended order:

1. #578 brownfield review handoff
2. #546 ratification pointer
3. #580 module-size adopter policy
4. #583 path normalization
5. #504 nested-package scan behaviour
6. #367 ignore scaffolding
7. #558 decision extraction

These are the strongest combination of reproduced defects, external-user evidence and first-session value.

### 5. Finish the task/control-plane spine

After or alongside the adoption fixes:

1. #334 full issue-body projection
2. #335 relationship-link projection
3. #573 decision-to-todo unblock edges
4. #579 driver
5. #575 console widening
6. #296 advisory overlap
7. #562 merge-safety design
8. #543 workflow evidence

### 6. Keep experiments from displacing defects

#559, #369, #280 and #525 all earn their place, but they should remain behind product defects and accepted control-plane work. Their purpose is to test or expand the product, not to substitute research activity for closing known failures.

## Bottom line

**Retain 32 of the 42 issues. Close or replace 10.**

The backlog does not need a wholesale reset. Its main problem is that completed work, dormant trackers, speculative infrastructure and executable defects all appear equally alive. Correcting that distinction will make the remaining roadmap substantially sharper without discarding the useful recent direction.

No GitHub issues or repository files were modified during this audit.
