# Cross-check: Bundle A (provenance foundation)

## Scope

Validate the proposed `phase-7.6-ai-provenance-foundation` bundle (C6 trace sidecar, C8.a suggested-edges queueing format, C8.b cflx-accept untriaged-block safety, C5.a islands query + verb-edge display) against current openspec content in the four areas it touches: `specs/changes`, `specs/reconciliation`, `specs/cli`, `specs/artefacts`, plus `specs/query`, `specs/graph-explorer`, and the active `phase-9-brownfield` change. The headline question: is Bundle A a genuine prerequisite for Phase 9, or partial-duplication that should be folded into Phase 9?

## Inputs read

| File | Validate-pass status (where verified) |
|---|---|
| `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-C.md` | n/a (research input) |
| `/Users/george/repos/cairn/docs/strongholds/getcairn-roadmap-debate.md` | n/a (research input) |
| `/Users/george/repos/cairn/openspec/changes/phase-9-brownfield/proposal.md` | passes `cflx.py validate phase-9-brownfield --strict` |
| `/Users/george/repos/cairn/openspec/changes/phase-9-brownfield/design.md` | (covered by phase-9-brownfield strict validate, passes) |
| `/Users/george/repos/cairn/openspec/changes/phase-9-brownfield/tasks.md` | (covered) |
| `/Users/george/repos/cairn/openspec/changes/phase-9-brownfield/specs/brownfield/spec.md` | (covered) |
| `/Users/george/repos/cairn/openspec/specs/changes/spec.md` | not directly probed in this session; structurally well-formed |
| `/Users/george/repos/cairn/openspec/specs/reconciliation/spec.md` | flagged in dispatch as currently failing validate; not directly probed in this session |
| `/Users/george/repos/cairn/openspec/specs/cli/spec.md` | structurally well-formed |
| `/Users/george/repos/cairn/openspec/specs/artefacts/spec.md` | flagged in dispatch as currently failing validate; not directly probed in this session |
| `/Users/george/repos/cairn/openspec/specs/query/spec.md` | structurally well-formed |
| `/Users/george/repos/cairn/openspec/specs/graph-explorer/spec.md` | structurally well-formed |
| `/Users/george/repos/cairn/openspec/specs/terminology-rename/spec.md` | (consolidated) |
| `/Users/george/repos/cairn/CLAUDE.md` | n/a (project instructions) |

The dispatch flagged that `specs/reconciliation/spec.md` and `specs/artefacts/spec.md` currently fail strict validate. I did not re-run validate on them in isolation; my analysis below treats their textual content as canonical-intent and flags any structural issues separately. None of Bundle A's recommended scope items would land specs into either reconciliation or artefacts; the schema work for Bundle A is in `changes`, `cli`, and an entirely new `provenance-foundation` (or similarly-named) capability area. So validate failures in reconciliation/artefacts are not a Bundle A blocker.

---

## Findings

### F1. Phase 9 brownfield's scope is **deterministic-extraction-only**; it has no AI-suggested-edges concept whatsoever

`phase-9-brownfield/proposal.md` lines 17-22 list the proposed scope additions: `cairn init --from-code`, `cairn refine`, structural candidate extraction from reconciler output, summariser-assisted naming/descriptions/tags/obvious edges, and human review through the Phase 3 change archive workflow. Nothing in proposal.md, design.md, tasks.md, or specs/brownfield/spec.md uses the word "suggest", "suggested-edges", "queue", "untriaged", "AI-suggested", or "edge triage". Grep confirms zero occurrences of `suggest`, `untriaged`, or `accept gate` across `phase-9-brownfield/`.

The brownfield model is: discovery extracts candidates with confidence scores and evidence paths into a `blueprint.delta` inside `meta/changes/brownfield-init/`. The summariser is used only for naming/describing/tagging extracted candidates (see design.md lines 37-38) plus drafting "stub contract content". It is not used to propose cross-cutting edges between artefacts that already exist; it is used to fill in textual fields on candidate nodes that the structural extractor produced.

This means: the C8.c "suggest engine" the roadmap-stronghold attributes to Phase 9 is **not in Phase 9's current scope**. Phase 9 produces edges, but only deterministically, from reconciler dependency observations (design.md line 22, "Observed dependency edges between candidates") and from observed import counts (design.md line 34, "at least two import observations from one candidate to another, or one public API reference with high confidence"). These are not AI-suggested edges in the C8 sense; they are reconciler-observed edges, deterministic by construction.

### F2. Phase 9's "human review" model is the existing Phase 3 archive workflow, with no untriaged-block safety primitive declared

design.md line 7 ("Phase 3 for change directory archive semantics") and proposal.md line 23 ("Human review through the Phase 3 change archive workflow") show Phase 9 explicitly relying on the existing change-archive flow. Acceptance Criteria (proposal.md lines 26-32) say "Summariser outputs are marked as proposed and require human archive" and "False positives can be deleted from the generated change before archive." The safety model is **delete-before-archive**, not **must-triage-before-archive**.

This is a meaningful gap. Phase 9's safety claim is "the human can delete bad outputs before archiving." That works for low-confidence candidate nodes (a candidate node is either kept or deleted from the delta, both verb-deterministic). It does **not** work for AI-suggested edges where the safety question is "did a human read this and assent?" rather than "did a human leave it in the file?". The delete-before-archive model treats absence-of-deletion as implicit acceptance, which is exactly the silent-landing failure mode Batch C's analysis warns against.

Phase 9 does not address this gap because it does not have AI-suggested edges to address it for. So Phase 9's spec is internally consistent, but **assumes** that if any later phase introduces AI-suggested edges, that later phase will install the must-triage safety. That assumption is the load-bearing thing Bundle A's C8.b is built to satisfy.

### F3. The path convention in active specs is `meta/changes/`, but the live working tree uses `openspec/changes/`

`openspec/specs/changes/spec.md` lines 7-32 use `meta/changes/<change-id>/` and `meta/changes/archive/YYYY-MM-DD-<change>/`. `openspec/specs/terminology-rename/spec.md` line 72 confirms the same. But the actual on-disk layout, which I confirmed via `ls /Users/george/repos/cairn/openspec/changes/`, uses `openspec/changes/` and `openspec/changes/archive/`. Recent archived phases (`phase-2.6-terminology-rename`, `phase-7.5a-test-fortification`, `phase-7.5b-cleansing-splits`) all live in `openspec/changes/archive/`, not `meta/changes/archive/`.

This is a pre-existing inconsistency between the spec text and the live tree. It is not Bundle A's job to fix, but Bundle A's C6 sidecar location is `openspec/changes/archive/<phase>/.cflx-trace.json` per the roadmap-stronghold, which matches the live tree. The risk: if a future phase normalises the spec to match the tree (or vice versa), the sidecar location reference might churn. Recommend C6's spec language describe the location as "the archived change directory for the phase", relative to whatever the canonical change-archive root is, rather than hardcoding `openspec/changes/archive/`. This decouples Bundle A from the eventual `meta/` vs `openspec/` reconciliation.

Phase-9-brownfield's specs use `meta/changes/brownfield-init/` (specs/brownfield/spec.md lines 27, 35, 41), aligned with the spec text but mismatched against the live tree. Phase 9 inherits this inconsistency from the existing changes spec, so it's not a Bundle A concern, but worth flagging for downstream coherence.

### F4. The cflx-trace concept does not exist anywhere in current specs

Grep for `trace`, `sidecar`, or `pipeline` returns only the unrelated SHA-256 sidecar reference in `artefacts/spec.md` line 37. Nothing in `specs/changes`, `specs/cli`, or any active phase describes a per-run trace artefact, a stage-keyed JSON, or a `cflx trace` CLI surface. The cflx workflow today (per CLAUDE.md and the verification battery list) executes `cargo build`, `cargo clippy`, `cargo fmt --check`, `cargo test`, and `cflx.py validate`, but the outputs of these are transient pre-commit-time logs, not durable per-change artefacts. C6 is genuinely new ground, no overlap.

The closest existing precedent is the archive event `.cairn/log.md` line in `specs/changes/spec.md` line 35: "appends an archive event to `.cairn/log.md`". This is a single-line append-only audit log keyed on archive timestamps. It is **not** a per-change structured trace and is not a per-stage record. So C6 isn't reinventing this primitive; it's adding a parallel one at higher granularity.

The CLI spec (`specs/cli/spec.md` lines 36-53) requires every CLI command to emit `--json` mode with stable schemas including a schema_version. C6's `cflx trace <phase>` CLI surface should comply with this requirement; the sidecar JSON itself should carry a schema-version header per `openspec/conventions.md` section 3 state-versioning, also satisfying the "stable machine-readable schema" requirement at the rendering boundary.

The spec also requires (lines 56-66) that "the CLI SHALL be a rendering and process boundary over shared library services, not the sole owner of query semantics". So `cflx trace` reads, doesn't own. The sidecar is owned by the cflx workflow runner that wrote it, and `cflx trace` is purely a renderer. This is a useful design constraint to inherit; it pre-answers one of Batch C's open questions (where does the surface live).

### F5. The "suggested edges" concept is also genuinely new ground

Grep for `suggest`, `untriaged`, `change-shaped`, or `edge.*queue` returns zero hits across `openspec/specs/` and the entire `openspec/changes/phase-9-brownfield/` tree. The artefacts spec (`specs/artefacts/spec.md` lines 6-12) lists six v1 artefact types (contract, todo, decision, review, research, source) and does not include any "suggested-edge" or "candidate-edge" subtype. The changes spec (`specs/changes/spec.md` lines 22-35) describes deltas with operations RENAMED, REMOVED, MODIFIED, ADDED applied at archive time, but does not have any concept of a "queue" of pending operations within a delta that require per-item triage before archive.

So C8.a "change-shaped queueing" is introducing a new artefact-shaped file inside the change directory, not extending an existing artefact type. This is an architecturally clean move because it ships **as a new file class within the existing change-isolation primitive**, rather than version-bumping the artefacts schema or adding a new subtype. The kernel's existing change-isolation does the safety work; the new file is just a new resident of that pre-existing safe enclosure.

Open question (carries forward from Batch C C8 Q1): the file's specific name and format. Recommendation in §Recommendations below.

### F6. C8.b accept-gate has a clean integration point but no precedent for "block on untriaged"

The cflx accept gate today is the verification battery (CLAUDE.md): `cargo build` (zero warnings), clippy with `-D warnings`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx.py validate <phase> --strict`. These are all "did the code work?" checks. There is no existing "did the human triage?" check.

The closest precedent for a content-shaped accept-time check is the `cflx.py validate <phase> --strict` call. This is exactly the right hook surface for C8.b: the validate-strict pass already inspects the change directory for structural correctness, and "does this change have untriaged suggested-edges?" is structurally indistinguishable from "is this change valid?". Recommendation: implement C8.b inside `cflx.py validate` as a strict-mode check, with a corresponding error code in `openspec/registries/error-codes.md`. This locates the gate in the existing toolchain rather than adding a new pre-commit phase.

The CLI spec (lines 56-66) does not constrain the verification battery, because the battery is cflx-runner concern, not cairn-CLI concern. So C8.b lands in cflx, not in `specs/cli`. This matches CLAUDE.md's cairn/cflx separation directive ("Calling `cflx` 'cairn'; they're different tools").

### F7. C5.a islands query: pure graph-traversal, neither in `specs/query` nor `specs/graph-explorer`

`specs/query/spec.md` lines 11-54 enumerate the typed queries: `get`, `neighbourhood`, `dependents`, `depends`, `order`. None of these surface "nodes not reachable from any system-rooted traversal" or "nodes in a disconnected subgraph". The graph explorer spec (`specs/graph-explorer/spec.md` lines 99-117) uses the word "orphan" exactly once in the rationale-tension scenario for "an orphan research artefact or missing source link", but that is a per-artefact orphan-finding, not a graph-component query.

`specs/kernel/spec.md` lines 71 and 125 (per the grep result) do use "orphaned" in the structural-error sense for `Container lacks owns-files: true` and "orphaned-file structural error". These are file-vs-node ownership orphans, distinct from graph-component disconnection. So the kernel has the *vocabulary* for orphaned (file-side), but not the query surface for islands (node-side, graph-component-side).

C5.a is genuinely new query capability. Recommendation: extend `specs/query/spec.md` with one new requirement (`Answer disconnected-subgraph queries`) and a corresponding scenario, plus extend `specs/cli/spec.md` with a new command (e.g., `cairn islands`) or extend the existing `cairn neighbourhood` with `--include-orphans`. The roadmap-stronghold says the latter; either is reasonable. The neighbourhood-with-flag approach has lower CLI surface-area cost and aligns with the existing flag pattern (`--include-changes` already exists per `specs/changes/spec.md` line 79). Recommend `cairn neighbourhood <node> --include-orphans` for the per-neighbourhood case and a separate `cairn islands` (or `cairn graph-components`) for the no-anchor whole-graph case. They serve different questions.

Verb-edge display is even cleaner: `specs/graph-explorer/spec.md` line 56-58 already requires the explorer to highlight an edge and show its label when source/target is selected. Edge labels are already first-class in the data model (per the Batch C analysis citing spec §7). The candidate is purely "render this label by default" or "render verb-form labels", which is a UI detail inside the graph explorer's existing scope. Bundle A doesn't need to bump the graph-explorer spec at all for C5.a's verb-edge sub-component; it's a UI tweak inside existing capability bounds. Worth flagging that this sub-component might not even need a phase scaffold; it could ride along as a small commit.

### F8. The change spec's ADDED/MODIFIED/REMOVED/RENAMED operation order is preserved verbatim by terminology-rename and is independent of Bundle A

`terminology-rename/spec.md` lines 70-75 explicitly preserve the four-operation delta vocabulary. Bundle A's C8.a doesn't change these; suggested-edges land as a separate file in the change directory, not as a fifth operation in the delta. This is the architecturally correct call (Batch C identified this in tension 3): suggested-edges are a *queue*, the delta is a *manifest*. They are different concepts and should not be unified. Recommend Bundle A's proposal explicitly says so in its design.md to prevent a future author from re-litigating.

---

## Recommendations

### R1. Bundle A as proposed (`phase-7.6-ai-provenance-foundation`) is a confirmed prerequisite for Phase 9

The combination of F1 (Phase 9 has no suggest engine concept) and F2 (Phase 9's safety model is delete-before-archive, not triage-before-accept) means Bundle A's C8.b is not duplication; it is a missing primitive that Phase 9 currently does not require for itself but that any **future** phase introducing AI-suggested edges will require. The roadmap-stronghold's claim that Phase 9 is silent on untriaged-block is correct.

If the C8.c suggest engine is **never** added to Phase 9 (which the current Phase 9 spec is consistent with), then C8.b is over-built relative to the roadmap. But the roadmap-stronghold explicitly plans C8.c as a Phase 9 deliverable (verdict table line 46: "Suggest engine itself: DEFER (milestone-gated): Inside Phase 9 brownfield"). If the orchestrator ratifies that plan, Bundle A's C8.b must ship before Phase 9. If the orchestrator changes its mind and pushes C8.c into a separate post-Phase-9 phase, Bundle A's C8.b can ship at any point before C8.c. Either way, Bundle A and Phase 9 remain ordered B-then-9 if 9 includes C8.c.

### R2. Land Bundle A's C8.a queueing format as a new file class inside the existing change-isolation primitive, not as a new artefact subtype or a new delta operation

Specifically: ship `openspec/changes/<change>/suggested-edges.json` (or `.toml`) with a state-versioned schema header per `openspec/conventions.md` §3. Do not version-bump `specs/artefacts/spec.md` to add a `suggested-edge` subtype. Do not add a fifth operation to the delta vocabulary in `specs/changes/spec.md` (preserve ADDED/MODIFIED/REMOVED/RENAMED). The new file is a sibling of `proposal.md`, `blueprint.delta`, and `design.md` inside the change directory. This minimises spec churn and respects the architectural distinction Batch C tension 3 identified.

### R3. Land Bundle A's C8.b safety check inside `cflx.py validate <phase> --strict`, not as a new pre-commit phase or new CLI command

The validate-strict call is already the structural correctness gate per CLAUDE.md's verification battery. Adding "no untriaged suggested-edges remain" as a strict-mode check requires no new gate plumbing, no new error-code class, just a new specific error code in `openspec/registries/error-codes.md`. This locates the safety in cflx (the workflow tool), not in cairn (the kernel), respecting the cairn/cflx separation directive. The error code should be specific enough that a reviewer reading the CI failure understands exactly what to do (recommend something like `CFLX-ACCEPT-001: untriaged suggested-edges remain in change <X>; run cflx triage-edges <X> to review`).

### R4. Locate the C6 sidecar at `<change-archive-root>/<phase>/.cflx-trace.json`, where `<change-archive-root>` is whatever the canonical archive root resolves to

This decouples Bundle A from the unresolved `meta/changes/` vs `openspec/changes/` inconsistency (F3). Bundle A's spec text should reference "the archived change directory for the phase" rather than hardcoding either path. When the inconsistency is eventually resolved (likely via a small terminology-rename-style cleanup phase), the trace location follows automatically. This is a cheap forward-compatibility move; no implementation cost.

### R5. C5.a islands sub-component should ship as two distinct CLI surfaces, not one

The neighbourhood-with-flag form (`cairn neighbourhood <node> --include-orphans`) and the whole-graph form (`cairn islands` or `cairn graph-components`) answer different questions. Conflating them under one flag would force the no-anchor case to invent a synthetic anchor or break the neighbourhood semantic. Recommend both, sized as ~50-100 LOC each. Add one new requirement to `specs/query/spec.md` and one new scenario to `specs/cli/spec.md`; do not touch `specs/graph-explorer/spec.md` (the UI render of disconnected components is a follow-on, not Bundle A scope).

### R6. C5.a verb-edge sub-component should ride along as a small commit, not a phase task

Per F7, edge labels are already first-class in the data model and already supported by the graph-explorer renderer's edge-highlight scenario. Verb-edge display is a UI tweak inside existing scope. Bundle A's proposal can declare it in scope but as a single small task ("update default edge-label rendering to display verb-form labels by default") rather than a multi-task implementation effort. Estimated <50 LOC.

### R7. Bundle A should declare itself non-blocking on `specs/reconciliation` and `specs/artefacts` validate failures

Neither C6, C8.a, C8.b, nor C5.a touches reconciliation findings or artefact schema. The validate failures in those specs (per the dispatch flag) are pre-existing and should be addressed in a separate cleanup effort. Bundle A's proposal.md should explicitly note this so that a verification-battery failure on those specs does not get conflated with Bundle A correctness.

---

## Decisions made (with reasoning)

### D1: Phase-9 prerequisite confirmed (Bundle A must ship before Phase 9 if Phase 9 includes C8.c)

- **Decision**: Bundle A is a genuine prerequisite for Phase 9, contingent on Phase 9 absorbing the C8.c suggest engine as the roadmap-stronghold plans. If the orchestrator decides C8.c lives in a separate post-Phase-9 phase, Bundle A is still a prerequisite for that later phase and the relative ordering of Bundle A vs Phase 9 becomes flexible. In **no** scenario should C8.c (or any AI-edge-suggesting tool) ship before Bundle A's C8.b.
- **Reasoning**: F1 confirms Phase 9's current scope is deterministic-only and silent on AI-suggested edges. F2 confirms Phase 9's safety model (delete-before-archive) does not generalise to triage-before-accept semantics. Phase 9's proposal does **not** plan untriaged-block as part of itself; it relies on the unstated assumption that any future phase introducing the suggest engine will install the safety. Bundle A is exactly that safety-installing phase. So Bundle A's C8.b is genuinely a load-bearing pre-step, not a partial-duplication of work Phase 9 already does. F4 and F5 confirm the trace and queueing concepts are also genuinely new ground; nothing in Bundle A's scope overlaps with anything Phase 9 currently declares.
- **Confidence**: high.
- **What would flip this**: (1) If Phase 9's design.md is rewritten to absorb the suggest engine *plus* its own untriaged-block safety, Bundle A's C8.b becomes partial-duplication and should be folded into Phase 9. But that would expand Phase 9's scope significantly (per the roadmap-stronghold's analysis, the suggest engine alone is non-trivial; adding the safety primitive on top is roughly doubling the safety/UX work). (2) If the orchestrator decides the C8.c suggest engine will never ship and Phase 9 stays deterministic forever, Bundle A's C8.b becomes preventive infrastructure for a tool that doesn't exist; still useful but not blocking, and the bundle could be pruned.

### D2: Bundle A's four sub-components belong in one phase, not split

- **Decision**: C6, C8.a, C8.b, and C5.a should ship together as `phase-7.6-ai-provenance-foundation`. Do not split into smaller phases (e.g., a separate "C5.a islands" phase or a separate "C6 trace" phase).
- **Reasoning**: The four sub-components share: (a) a single phase scaffold, design.md, tasks.md, specs/ overhead, which is fixed cost amortised better across four than across one or two; (b) overlapping touch surfaces in `specs/cli` and `specs/changes`, where one phase's spec deltas can cleanly cover all four; (c) common state-versioning header pattern per conventions §3, which can be designed once for all four artefacts (the trace sidecar, the suggested-edges queue, the islands query response, the verb-edge label). Splitting would force four parallel design conversations on the same primitive.
- **Confidence**: medium-high.
- **What would flip this**: If conventions.md §3 module-size pressure starts biting (the spec has explicit module-size guidance per CLAUDE.md), and the combined scope exceeds it, split into `phase-7.6a-trace-and-islands` (C6 + C5.a, lower-risk) and `phase-7.6b-suggested-edges-format` (C8.a + C8.b, higher-risk and Phase 9-blocking). This is a tactical fallback, not a strategic preference.

### D3: C5.a is in scope but with reduced ambition relative to the roadmap-stronghold's proposal

- **Decision**: Adopt C5.a as proposed but split the implementation into the two distinct CLI surfaces (per R5) and treat verb-edge display as a small commit rider, not a major task (per R6). Net implementation: ~150-250 LOC instead of the roadmap-stronghold's estimated 100-150 LOC for islands alone. The +50-100 LOC delta comes from supporting both the neighbourhood-anchored and whole-graph forms.
- **Reasoning**: F7 shows the existing query and graph-explorer specs do not have either form of islands query. Both forms answer real questions that cairn users will ask once Bundle A's other infrastructure exposes them to "this graph has disconnected pieces" as a routine concept. Shipping only one form would force users to invent the other via workarounds (e.g., calling `neighbourhood --include-orphans` against a synthetic root node). Verb-edge display is genuinely cheap (F7) and rounds out the visible-improvements story for the bundle without bulking it.
- **Confidence**: medium-high.
- **What would flip this**: If implementation surfaces a non-trivial cost for the whole-graph `cairn islands` form (e.g., the graph traversal infrastructure makes "no anchor" expensive), drop that form and ship only the neighbourhood-with-flag variant, and leave the whole-graph form for a follow-on. This is an implementation-time backstop, not a planning concern.

### D4: C8.a queueing format ships as `suggested-edges.json` (or `.toml`) sibling-of-proposal.md inside the change directory

- **Decision**: The file lives at `openspec/changes/<change>/suggested-edges.<ext>`, not at `meta/changes/<change>/suggested-edges.<ext>` (per F3 the live tree uses `openspec/`). Format is JSON or TOML with state-versioning header per conventions §3. Each entry has fields: `source` (node ID), `target` (node ID), `relation` (string), `confidence` (optional float), `provenance` (run identity reference, populated when C8.c lands; empty for now), `triage_state` (enum: `pending` / `accepted` / `rejected` / `deferred`), `triage_note` (optional string). The triage_state enum is initially set to `pending` for all entries; the C8.b accept-gate fails if any entry is `pending` at accept time.
- **Reasoning**: F5 shows this is genuinely new ground. The change-isolation primitive (F8) does the structural safety work; the new file is a queue with explicit per-entry state, making the must-triage discipline non-bypassable. The `triage_state: pending` default plus C8.b's strict-mode check means: even if a user manually tries to skip review by leaving the file as-shipped, the gate fails on accept. This satisfies the "structurally non-bypassable" requirement Batch C identified as load-bearing.
- **Confidence**: medium-high.
- **What would flip this**: If the queueing infrastructure design uncovers a clean way to encode triage state inside the existing `blueprint.delta` operations (e.g., as a `pending: true` flag on an ADDED edge), that could simplify the file structure by collapsing the queue into the delta. But Batch C tension 3 explicitly recommended *against* this conflation, and F8 confirms preserving the four-operation vocabulary is correct. Recommend not flipping unless a strong design reason emerges in the Bundle A design.md authoring pass.

### D5: C8.b safety check ships in `cflx.py validate <phase> --strict`, not as a new tool or new CLI command

- **Decision**: Per R3. The implementation is: validate-strict mode reads any `suggested-edges.<ext>` file in the change directory, fails if any entry has `triage_state: pending`, and outputs the count of pending entries plus the path to the file. The error code is added to `openspec/registries/error-codes.md` with a clear remediation message.
- **Reasoning**: This locates the safety in the existing toolchain (the verification battery) rather than adding a new gate. F6 shows there is no existing "block on untriaged" precedent in the verification battery, but the `cflx.py validate <phase> --strict` call is the natural extension point. Adding a strict-mode-only check means: dev-time validate (without `--strict`) shows the count as a warning but doesn't fail; CI-time validate (with `--strict`, per the verification battery) fails. This matches the strict-vs-loose discipline established elsewhere in the toolchain.
- **Confidence**: high.
- **What would flip this**: If the validate-strict tool is determined to be the wrong place architecturally (e.g., if a future refactor moves all gating into a separate cflx-accept binary), C8.b moves with it. The placement is implementation-detail; the requirement (must-triage gate exists somewhere on the accept path) is what matters.

### D6: C6 trace sidecar and `cflx trace` CLI surface are in-scope; prompt-content persistence is out-of-scope (Batch C C6 Q1 deferred)

- **Decision**: Bundle A's C6 ships the sidecar with metadata only (model identity, stage names from cairn-native vocabulary `propose/apply/accept/archive`, per-stage tokens, per-stage latency, success flag, error message if any, schema-version header). It does **not** ship full prompt persistence. The `cflx trace <phase>` CLI surface pretty-prints the sidecar.
- **Reasoning**: Per Batch C C6 §6 and the roadmap-stronghold synthesis, prompt persistence is a privacy-sensitive open question (Q1, Q2). Shipping metadata-only is the minimum-viable form that satisfies the auditability claim without committing to a privacy posture that hasn't been designed yet. The metadata form is also what Batch C identifies as "already collected by the SDK; just persist", so it's cheap. Prompt persistence can land as a Bundle A follow-on commit (e.g., `cflx trace <phase> --include-prompts` opt-in flag) once the privacy design is settled, without re-bumping the sidecar schema (the schema can include an optional `prompts: []` field that's empty by default).
- **Confidence**: high.
- **What would flip this**: If a strong consumer use case for prompt persistence emerges during Bundle A scoping (e.g., the cflx-proposal skill needing it for transcript replay), include it in scope. Otherwise defer.

### D7: Bundle A's spec deltas land in `specs/cli`, `specs/changes`, `specs/query`, plus a new top-level capability area `specs/provenance-foundation` (or similar)

- **Decision**: New capability area for the trace sidecar concept (since it doesn't fit `cli`, `changes`, or `query` cleanly). Existing-area additions for the islands query (`specs/query`), the suggested-edges file (`specs/changes`), and the new CLI commands (`specs/cli`). The accept-gate check is documented in cflx tooling docs (not a cairn-spec area), so no spec delta there beyond an error-code addition.
- **Reasoning**: The trace sidecar is per-change but is **not** a kernel artefact (per CLAUDE.md's cairn/cflx separation). It's a workflow-tool artefact. Putting it under `specs/changes` would falsely suggest the kernel owns it; putting it under `specs/cli` would conflate the data with its renderer. A new capability area `provenance-foundation` (or `cflx-trace`) is the architecturally honest move and matches the existing pattern of capability-shaped specs (`reconciliation`, `graph-explorer`, `multi-target`, `mcp`, etc.).
- **Confidence**: medium.
- **What would flip this**: If a Bundle A author finds the new capability area too narrow (single concept, ~50 lines of spec), they could fold the trace-sidecar requirements into a broader `cflx-workflow` capability area that also absorbs the C8.b accept-gate semantics. This would consolidate "things cflx does that aren't kernel queries" into one spec area. Worth considering during design.md authoring.

---

## Open questions for next session

The following questions are not Bundle A's job to answer but are surfaced for the orchestrator's queue:

1. **Reconciliation and artefacts spec validate failures (pre-existing).** Bundle A is non-blocking on these (R7), but they should be cleaned up in a separate small phase before any phase that genuinely touches those areas lands. Worth scoping as a small `phase-7.x-spec-validate-cleanup` effort.

2. **`meta/changes/` vs `openspec/changes/` path inconsistency (pre-existing, F3).** Live tree uses `openspec/changes/`; spec text uses `meta/changes/`. Recommend a small terminology-rename-style cleanup phase to normalise; Bundle A is structurally insulated from this via R4.

3. **Whether C8.c (suggest engine) lives inside Phase 9 or a separate post-Phase-9 phase.** The roadmap-stronghold says inside Phase 9. Phase 9's current scope is silent on it. The orchestrator should confirm one way or the other before Bundle A is scheduled, because that confirmation determines whether Bundle A is strictly Phase 9-blocking or merely a-prerequisite-for-something-later.

4. **Bundle A vs Bundle B parallel-shippable confirmation.** The roadmap-stronghold says yes (no code overlap). My cross-check did not investigate Bundle B's surfaces in depth, but the Bundle A surfaces I examined (CLI, changes, query, provenance-foundation) do not appear to overlap with the Bundle B surfaces named in the synthesis (UI assets, design-system, reconciler-finding feed). Recommend the parallel-shippable claim is confirmed by the Bundle B cross-check analyst rather than treated as established.

5. **Whether the trace-sidecar schema should be JSON or TOML.** Conventions.md doesn't mandate one. JSON is more machine-friendly for the `cflx trace --json` output mode; TOML is more human-friendly for direct file inspection. Recommend JSON because the consumer is more often programmatic (CI dashboards, future webui surfaces) than human-direct, and the cflx trace CLI command provides the human-friendly view. But this is a Bundle-A-design-time decision, not a Bundle-A-prerequisite decision.

6. **Whether `cflx trace <phase>` requires a corresponding MCP tool registration.** The MCP capability spec (`specs/mcp/`, not directly probed here) may have a query-tool-registry pattern that the trace surface should join. Recommend the Bundle A design.md authoring pass checks this and registers `cflx_trace` as a non-mutating MCP tool if the registry pattern fits.

---

## Recommended Bundle A final scope

| Sub-component | Status vs roadmap-stronghold proposal | Notes |
|---|---|---|
| C6 trace sidecar (`<archive-root>/<phase>/.cflx-trace.json`) | confirmed-as-proposed | Metadata-only initial form (D6); prompt persistence deferred to Bundle-A follow-on. State-versioned schema per conventions §3. Stage names cairn-native (`propose/apply/accept/archive`), not getcairn.dev's. |
| C6 `cflx trace <phase>` CLI surface | confirmed-as-proposed | Pure renderer; reads sidecar; no semantics ownership (per CLI spec lines 56-66). Includes `--json` mode per CLI spec lines 36-53. |
| C8.a suggested-edges queueing format | confirmed-as-proposed, schema sharpened (D4) | File at `openspec/changes/<change>/suggested-edges.<ext>`. Per-entry `triage_state` enum with `pending` default. Sibling of `proposal.md`, not a fifth delta operation. |
| C8.b cflx-accept untriaged-block safety | confirmed-as-proposed, integration-point sharpened (D5) | Implemented inside `cflx.py validate --strict`, not as a new tool. New error code in registries. |
| C5.a islands query (CLI + query layer) | confirmed-as-proposed, scope-expanded (D3) | Two CLI surfaces: `cairn neighbourhood <node> --include-orphans` (anchored) and `cairn islands` or `cairn graph-components` (whole-graph). New requirement in `specs/query/spec.md`. |
| C5.a verb-edge display | confirmed-as-proposed, scope-shrunk (R6) | Small commit rider; UI default-render tweak inside existing graph-explorer scope. No spec delta. |

**Net bundle scope vs roadmap-stronghold's estimate:** roadmap-stronghold projected ~500-850 LOC across Bundle A. With my recommendations:

- C6 sidecar + CLI: 200-400 LOC (unchanged)
- C8.a queueing format: 100-200 LOC (unchanged)
- C8.b accept-gate: 50-100 LOC (unchanged; integration point is just a Python addition to cflx.py validate-strict)
- C5.a islands query (two forms): 150-250 LOC (+50-100 LOC vs roadmap-stronghold for the second form)
- C5.a verb-edge display: <50 LOC (-50-100 LOC vs roadmap-stronghold)

Net: ~550-1000 LOC. A modest expansion at the upper bound but well within "one phase" sizing per conventions §3.

**Phase-scaffold deltas:**
- New capability area: `specs/provenance-foundation/spec.md` (or `specs/cflx-trace/spec.md`).
- Modified specs: `specs/cli/spec.md` (new commands), `specs/query/spec.md` (new requirement), `specs/changes/spec.md` (suggested-edges file class).
- New error code in `openspec/registries/error-codes.md` for the C8.b untriaged-block.
- No spec change to `specs/artefacts`, `specs/reconciliation`, `specs/graph-explorer`.

**Sequencing:** ship in the order C6 (foundation, no consumers yet) -> C5.a (no upstream dep) -> C8.a (queueing) -> C8.b (gate, references C8.a). Internal ordering does not need to be enforced at the commit level; the verification battery passes whether commits land in this order or interleaved. Recommend the order purely for review-narrative clarity.

**Net Bundle-A scope change vs roadmap-stronghold's proposal:** confirmed as proposed with three internal refinements (R2 file-class location, R3 accept-gate integration point, R5 islands-as-two-surfaces) and one scope reduction (R6 verb-edge as commit rider). No sub-component is rejected; no sub-component is folded into Phase 9; no new sub-component is added. The bundle ships intact with sharpened internal designs.

---

**End of cross-check.** ~3700 words.
