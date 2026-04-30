# Refined adoption analysis: Batch A (UX cluster)

## Scope

Candidates: C2 (three-axis fidelity radar plus prose-nudge), C3 (Quality Check panel with severity buckets), C13 (empty-state CTAs). Framework: 4-dimension analysis (problem clarity, layer classification, partial adoption breakdown, refined verdict). Adversarial stance: refine or flip the matrix verdicts where deeper analysis warrants.

The cluster is described as "UX-heavy" but each candidate has a non-trivial process or architectural undercurrent that the matrix glossed over. This batch surfaces those undercurrents.

## C2: Three-axis fidelity radar with prose-nudge banners

### 1. Problem and solution clarity

**The cairn-side problem.** cairn already produces three classes of node-level signal: reconciliation state (`synced` / `ghost` / `orphaned`), evidential strength (`verified` / `external` / `unverified`), and `drift` for declared-vs-real divergence. These are computed and surfaced individually inside the webui graph explorer (`src/ui_assets/app.js`, ~56KB, the only consumer). What is missing is a *rollup surface* that lets a non-specialist read a single node and answer "is this node strong, and if not, where is it weak?" without learning three orthogonal vocabularies. The friction lives concretely in the node-detail panel rendered from `app.js`: a user opens a node, sees three independent badges, and has to mentally compose them into a verdict.

The prose-nudge problem is sharper and more concrete: cairn's reconciler produces structured findings (interface contradictions, rationale tensions) that today render as terse strings in the same panel. There is no plain-English translation layer that says "the blueprint declares this contract but no source file matches" in audience-appropriate language.

**getcairn.dev's solution.** Two coupled mechanisms. (a) A radar widget plotting three numeric scores (Entities, Processes, Relationships) producing a triangle whose shape encodes node weakness. (b) A yellow inline banner translating the radar's numeric finding into a plain-English diagnostic ("the model knows what it is, but not what it does") with a `Fix with AI` CTA. Triggered on every node detail view; updated when the underlying model changes.

**Solution to problem fit.** The prose-nudge mechanism (b) addresses cairn's actual problem cleanly: it is a templating layer over reconciler findings that picks a copy variant per finding class. The radar mechanism (a) addresses a *related-but-different* problem. getcairn.dev has three orthogonal completeness axes because their domain is MBSE (Entities, Processes, Relationships are the classical SysML decomposition triple). cairn has *two* chains, not three axes. The radar shape imports a triangular topology that does not exist in cairn's data model. Forcing cairn signals into three axes either invents a third axis to fill the slot, which corrupts the framing, or keeps two axes filled and leaves the third empty, which makes the radar look broken.

**Ambiguities or missing data.** (i) Whether the chain-balance widget referenced in `docs/strongholds/cairn-domain-expandability.md` actually exists in `src/ui_assets/` today or is described as future work, the matrix assumed it exists. (ii) Whether reconciler findings carry severity at the producer side or are all uniform-severity strings that the UI must categorise. (iii) Whether webui has any current notion of "node detail panel" to host either widget, or if this is greenfield UI work.

### 2. Layer classification

Split: roughly 70 percent UI/UX, 20 percent process, 10 percent architectural.

- **UI/UX (70%):** The radar widget, the prose-nudge banner copy, the visual treatment using design-system tokens (`--prov-*`, `--auth-*`, `--drift`, `--block`, `--settled`), the placement in the node-detail panel.
- **Process (20%):** A "finding to copy template" pipeline (which finding class maps to which prose nudge; whether copy is hardcoded or configurable); a `Fix with AI` action surface that has to dispatch *somewhere* (cflx command? webui write affordance? CLI handoff?). The matrix's open question 1 ("read-only graph explorer versus write surface") lands here: without a write affordance, the Fix button is a broken promise.
- **Architectural (10%):** If reconciler findings need a structured `severity` field, or a stable `category` taxonomy that downstream UI can switch on, that is a schema decision that touches the reconciler interface (`openspec/specs/<area>/spec.md` for reconciler). Not a kernel-level change but a contract-level one.

### 3. Partial adoption breakdown

Sub-components, with NOW / LATER / RESEARCH / MILESTONE / REJECT:

| Sub-component | Verdict | Rationale |
|---|---|---|
| **Three-axis radar widget shape** | REJECT | Imports MBSE-shaped triangle topology that does not exist in cairn's two-chain data. Re-borrows the flat-framing risk that v0.5 explicitly rejected, by way of visual rather than schema. |
| **Multi-dimensional rollup surface (any shape)** | RESEARCH | The case-for of multi-axis rollup is real; the question is whether cairn's native shape is the chain-balance widget extended with severity gradients (two bars, prov + auth) or something else. Question to investigate: does a two-chain balance widget already exist in `src/ui_assets/app.js`, and what would extending it with severity gradients look like? |
| **Prose-nudge banner: templated copy per finding class** | NOW | Mechanically straightforward. Existing reconciler findings get a copy template per class. Lives in a single config file (e.g., `docs/design-system/copy/findings.toml` or equivalent) so it stays maintainable and matches the design-system "tokens, not hardcoded values" discipline. |
| **Prose-nudge banner: severity rendering (color, icon, weight)** | NOW | Maps directly onto existing tokens: `--block` for blocking findings, `--drift` for advisory tensions, `--settled` for info. No new tokens needed. |
| **`Fix with AI` button on the banner** | LATER | Precondition: webui write affordance exists, OR the button is reframed as a copy-pasteable CLI command (e.g., "Run `cflx fix <node-id>`") that the user executes in their terminal. The current read-mostly webui means an in-UI Fix button is a broken promise (matrix open Q1). Defer until that direction is settled. |
| **Inline banner placement at top of node-detail panel** | NOW | Standard UI pattern; design-system tokens cover the visual treatment. |
| **Per-node "completeness number" headline** | REJECT | Single-number completeness is exactly what getcairn.dev *avoids* (they use a radar precisely to refuse the single number); cairn should also refuse it. Picking one number flattens what the prose-nudge is meant to expose. |

### 4. Refined verdict

**Matrix verdict:** DEFER radar, ADOPT nudge (flipped from initial scout adopt-all).

**My deeper analysis:** Refines the matrix verdict. Specifically:

- **REJECT the three-axis radar shape outright** rather than DEFER. The matrix said "defer pending a UI design that uses two-chain balance instead." That is the right instinct but the wrong verdict label: there is no version of "three-axis radar" that survives once you accept cairn is two-chain. The thing to defer is "multi-dimensional rollup" generically, not "three-axis radar" specifically. Calling the deferred thing a radar pre-decides the shape.
- **RESEARCH multi-dimensional rollup as an open question**, with a concrete investigation: extend the existing chain-balance widget with severity gradients, OR design a native two-axis rollup. This is research because it depends on artifacts not yet built (matrix open Q4).
- **ADOPT the prose-nudge layer NOW**, but with two refinements: (1) copy lives in a single configurable location (matrix's drift-concern from C13 applies here too); (2) the `Fix with AI` button is replaced with a copy-pasteable CLI command string until the webui write surface direction is settled.

**Refined verdict:** ADOPT prose-nudge banner (templated copy + severity rendering + placement); REJECT three-axis radar shape; RESEARCH multi-dimensional rollup native to two-chain; DEFER `Fix with AI` button until webui write-surface direction is decided.

### 5. Open research questions

1. Does `src/ui_assets/app.js` already have a chain-balance widget, or is that future work? (Determines whether "extend existing widget" is even an option.)
2. Do reconciler findings carry a structured `severity` field today, or is severity inferred at render time from finding class? (Determines whether prose-nudge needs a reconciler schema change.)
3. What is the canonical home for finding-class-to-copy mapping? A new file in `docs/design-system/`, a TOML in `src/ui_assets/`, or an `openspec/registries/` entry? Affects maintainability.
4. Is the `Fix with AI` action eventually a webui button, a CLI command, or both? Gates not just C2 but C3 also.

## C3: Quality Check panel with severity buckets and inline remediation

### 1. Problem and solution clarity

**The cairn-side problem.** Reconciler findings exist as structured data but are surfaced today only at the per-node level (per matrix-implied current state of `src/ui_assets/app.js`) and through `cflx accept`'s pass/fail summary at commit-gate time. There is no *whole-model rollup* that lets a reviewer answer "across this entire blueprint, what is broken, what is advisory, and what is informational?" The friction lives at two surfaces: (a) the webui has no panel that aggregates findings across nodes; (b) the CLI has no `cflx check` command that produces a structured findings dump (the current `cflx accept` is gate-pass-or-fail, not an inspection mode).

**getcairn.dev's solution.** A dedicated `Quality Check` panel that aggregates findings across the whole model into typed severity buckets (errors, warnings, info), with: a single headline count, scope toggles (`Entire Model` vs `This Node`), category filters (`COMPLETENESS`, `TRACEABILITY`), per-row `Fix` actions, and a `Re-run` button with last-run timestamp. Triggered manually via `Re-run` or automatically on model change.

**Solution to problem fit.** Strong fit on the rollup-surface and severity-bucket sub-components. Weaker fit on the category-filter sub-component (their categories `COMPLETENESS`, `TRACEABILITY` are MBSE-shaped; cairn's natural categories would be different, interface-contradiction, rationale-tension, ghost, orphaned, drift). The `Fix` button has the same read-mostly problem as C2's button. The `Re-run` semantics also need scrutiny: getcairn.dev re-runs an AI pipeline; cairn would re-run a deterministic reconciler, which is much cheaper and might not need an explicit button at all (could refresh on file watch).

**Ambiguities or missing data.** (i) Whether reconciler findings have a stable `category` field today or only `class` (interface-contradiction vs rationale-tension), affects whether the category-filter UI is data-ready or needs a schema add. (ii) Whether `cflx accept` already produces the same finding stream that the Quality panel would consume, or the panel needs a separate read path. The matrix flagged this as the "single source vs parallel pipeline" pivotal question. (iii) Whether the webui has a TUI/CLI parallel: a `cflx check --json` command would let CI consumers and non-webui users reach the same data.

### 2. Layer classification

Split: roughly 50 percent UI/UX, 35 percent process, 15 percent architectural.

- **UI/UX (50%):** The panel layout, severity bucket rendering, scope toggles, category filters, design-system token mapping (`--block` errors, `--drift` warnings, `--settled` info), the panel's home in the webui chrome.
- **Process (35%):** A `cflx check` command that produces the same finding stream as `cflx accept` but in inspection mode (no gate). The shared-data-source discipline (the matrix's pivotal question). The Re-run trigger semantics (file-watch vs manual). CI integration shape if `cflx check --json` is meant to be machine-readable.
- **Architectural (15%):** If reconciler findings need a stable `category` field added to support filters, that is a reconciler-interface schema change. Also: defining what counts as info-severity (orphaned files? unverified contracts?) is a taxonomy decision that affects how reconciler producers tag their output.

The matrix's framing of C3 as "mechanical fit" understated the process layer. The panel is shallow; the structured finding feed is the load-bearing piece, and that feed is process work, not UI work.

### 3. Partial adoption breakdown

| Sub-component | Verdict | Rationale |
|---|---|---|
| **Whole-model rollup panel in webui** | NOW | Pure UI on top of existing data; design-system tokens cover the visual treatment. |
| **Severity bucket UI (errors / warnings / info)** | NOW | Map: interface-contradictions and structural errors → block; rationale-tensions and drift → warning; orphaned/unverified → info. Uses existing tokens. |
| **`cflx check` CLI command (inspection mode of accept's reconciler run)** | NOW | Forcing function for the "single source" discipline. The command runs the same reconciler, dumps findings as JSON or human-readable text, returns 0 regardless of severity (it is inspection, not gate). |
| **Shared-data-source contract: panel reads `cflx check` output** | NOW | This is the architectural discipline that prevents the matrix's case-against. Specced as: panel only renders findings the CLI also produces; no parallel inspection path. |
| **Scope toggle: Entire Model vs This Node** | NOW | Cheap UI toggle on a filter applied to the same finding stream. |
| **Category filters (interface-contradiction, rationale-tension, drift, etc.)** | RESEARCH | Question to investigate: are reconciler findings already tagged with a stable category enum, or is category inferred from finding class? If the latter, this needs a small reconciler-interface change to formalise category as a first-class field. |
| **Per-row `Fix` button** | LATER | Same precondition as C2's Fix button: webui write-surface direction needs to be settled. Until then, render the row with a copy-pasteable CLI snippet ("Run `cflx fix <finding-id>`") instead of an in-UI button. |
| **Re-run button with timestamp** | LATER | cairn's reconciler is deterministic and fast; a file-watch refresh is the better default than a manual re-run button. Defer until a concrete user need for manual re-run surfaces. The "last-run timestamp" is fine to render even with file-watch refresh. |
| **Naming the panel "Quality Check"** | REJECT | Matrix correctly flagged "Quality" as importing a fitness vocabulary cairn does not own. Use `Findings` or `Reconciler findings` instead. The word "Quality" implies a judgement (good/bad); cairn's findings are reconciliation-state observations, not quality assessments. |
| **`COMPLETENESS` and `TRACEABILITY` as the category names** | REJECT | MBSE-shaped categories. cairn's categories are `interface-contradiction`, `rationale-tension`, `drift`, `ghost`, `orphaned`, `unverified`. Use cairn vocabulary. |

### 4. Refined verdict

**Matrix verdict:** ADOPT.

**My deeper analysis:** Confirms the verdict but sharpens the scope. The matrix called this "mechanical fit" and treated the work as primarily UI. It is not: the load-bearing pieces are (a) the `cflx check` CLI command that establishes the single-source discipline, (b) the shared-data-source contract that the panel reads only from `cflx check` output, and (c) the rename away from MBSE vocabulary ("Quality", "COMPLETENESS", "TRACEABILITY"). The UI panel is the thinnest part.

**Refined verdict:** ADOPT severity bucket UI + structured finding feed (`cflx check` + shared-data-source discipline) + cairn-vocabulary naming; DEFER per-row Fix button (gated on webui write-surface direction); DEFER Re-run button (file-watch refresh is the better default); RESEARCH category taxonomy (one investigation to decide whether category becomes a first-class reconciler-finding field).

### 5. Open research questions

1. Do reconciler findings today carry a stable `category` field, or only a `class`? (Gates the category-filter sub-component.)
2. Does `cflx accept` emit findings in a form that `cflx check` can reuse verbatim, or does inspection-mode need a separate code path? (Gates the single-source discipline.)
3. Is there a CI consumer that wants `cflx check --json`, or is the JSON form speculative? (Affects scope of the CLI command.)
4. Same as C2 Q4: webui write-surface direction. Gates the Fix button.

## C13: Empty-state CTAs that name the next concrete action

### 1. Problem and solution clarity

**The cairn-side problem.** A first-time user reaches several states where the webui or CLI has nothing to render and no instruction about what to do next. Concrete examples:

- A fresh repo with `cairn.config.yaml` initialised but no blueprint yet: the webui graph explorer shows an empty graph; nothing tells the user "your next move is to declare a System node."
- A blueprint declared but no source files matching: the reconciler reports all-ghost; nothing tells the user whether to write code or fix the blueprint paths.
- A node selected with no contracts attached: the node-detail panel shows empty contract section; nothing tells the user to author a contract.
- CLI: `cflx` invoked with no subcommand probably prints help (current behavior unverified from this batch's reading); nothing in CLAUDE.md or the design-system suggests a guided onboarding text.

The friction lives across `src/ui_assets/app.js` (graph explorer empty states) and the CLI surface. CLAUDE.md's voice direction is explicit: the bar is "would a non-dev feel nervous typing this command or reading this doc?" Empty-state silence is the maximum-nervousness state.

**getcairn.dev's solution.** Every empty state surfaces (a) a placeholder visual (icon, illustration), (b) a heading naming what is missing, (c) a one-sentence body explaining the value of filling it, (d) a primary CTA button naming the next concrete action ("Open Command Palette", "Generate 3D"). Triggered on render of any view whose data set is empty.

**Solution to problem fit.** Excellent fit on the principle (always name the next move) and on most sub-components. Weaker fit on (d) for cairn specifically: getcairn.dev's CTA buttons are in-UI actions because their webui *is* the surface where work happens; cairn's webui is read-mostly. The matrix correctly identified this as the pivotal question. CTAs in cairn need to be honest about whether they trigger an in-UI action or hand off to the CLI.

**Ambiguities or missing data.** (i) The exact list of empty states in the current webui, needs a sweep of `src/ui_assets/app.js` to enumerate. (ii) Whether the CLI has equivalent empty-output states (e.g., `cflx accept` on a fresh repo) and whether those deserve the same treatment. (iii) Whether voice/copy authority lives in the design-system directory or somewhere else; today's `docs/design-system/` is tokens + components, not copy.

### 2. Layer classification

Split: roughly 80 percent UI/UX, 20 percent process.

- **UI/UX (80%):** The empty-state component (icon + heading + body + CTA), design-system token usage, copy strings, placement in each surface. Voice and audience compliance (CLAUDE.md's "would a non-dev feel nervous" bar).
- **Process (20%):** Where copy strings live (a single configurable file vs scattered across components); maintenance discipline (matrix case-against #1: drift when CLI commands rename); whether the CLI gets parallel empty-state treatment (e.g., `cflx` with no args prints "Your repo has no blueprint yet. Run `cflx propose` to draft one."). The voice-authority question: who reviews copy when the CLI surface changes? The Phase 2.6 rename burned this lesson, copy strings in user-facing surfaces are a maintenance surface, not a fire-and-forget.
- **Architectural (0%):** No schema or kernel changes. Pure surface work.

The matrix said this was "pure UX investment with low backend cost." That undersold the process layer slightly. The 20 percent is small but it is the part that determines whether the work *stays* good or rots.

### 3. Partial adoption breakdown

| Sub-component | Verdict | Rationale |
|---|---|---|
| **Empty-state component (icon + heading + body + CTA)** | NOW | Standard pattern; design-system tokens cover all visual properties; needs a new component variant in `docs/design-system/components.css`. |
| **Sweep of current webui empty states + write copy for each** | NOW | The blocking work. Enumerate empty states in `src/ui_assets/app.js`; write copy per state in cairn voice. |
| **CLI empty-state copy (parallel treatment)** | NOW | Same principle, different surface. `cflx` with no args, `cflx accept` with empty repo, etc. Lower visual complexity but same voice discipline. |
| **CTAs that name CLI commands (copy-pasteable)** | NOW | Honest for cairn's read-mostly webui. CTA reads: "Run `cflx propose` to draft your first blueprint." User copies into terminal. No broken promise. |
| **CTAs that trigger in-webui actions** | LATER | Precondition: webui write-surface direction settled (matrix open Q1). Without write affordances, in-UI CTAs are broken promises. |
| **Centralised copy strings (single configurable location)** | NOW | Matrix case-against #1 (drift) is mitigated by storing copy in one file, not scattered across components. Candidate location: `docs/design-system/copy.toml` or `src/ui_assets/empty-states.json`. Same discipline that C2's prose-nudge needs; consider co-locating. |
| **Voice review checklist (CLAUDE.md compliance)** | NOW | Process-layer discipline. A short checklist applied to every copy string: no em-dashes, plain English, names cairn vocabulary correctly (`blueprint`, `map`, not `dsl`, `ontology`), passes the "non-dev nervousness" bar. Lives in `docs/design-system/README.md` voice section or as a separate `voice.md`. |
| **Empty-state illustrations beyond plain icons** | LATER | getcairn.dev uses a diamond icon and 3D placeholder. cairn could ship NOW with token-based icon-only treatment and add custom illustrations LATER if a marketing or onboarding push surfaces a need. |

### 4. Refined verdict

**Matrix verdict:** ADOPT.

**My deeper analysis:** Confirms ADOPT but adds two implementation guardrails the matrix mentioned only briefly:

1. **Centralised copy**: store all empty-state strings in one file. Matrix case-against #1 (drift on terminology rename) is real (the Phase 2.6 rename was a working example of how scattered copy strings create silent breakage). Single file makes a future rename a one-edit task instead of a cross-component sweep.

2. **CTAs name CLI commands until webui write-surface direction is settled**, keeps CTAs honest given the current read-mostly webui. The user still sees the next move; they execute it in the terminal where work happens today.

**Refined verdict:** ADOPT empty-state component + sweep + cairn-voice copy + centralised copy strings + CLI-handoff CTAs; LATER in-webui CTA actions and custom illustrations (gated on webui write-surface direction and onboarding push respectively).

### 5. Open research questions

1. What is the exhaustive list of webui empty states in `src/ui_assets/app.js`? (Scoping question; needs a sweep.)
2. Where do copy strings live canonically? (`docs/design-system/copy.toml`? `src/ui_assets/copy.json`? new `docs/design-system/voice.md`?), affects both C13 and C2's prose-nudge.
3. Same as C2 Q4 / C3 Q4: webui write-surface direction. Gates the in-UI CTA sub-component.
4. Does the CLI surface deserve parallel empty-state treatment? Probably yes per voice direction, but worth confirming scope.

## Cluster-level observation

All three candidates share a single architectural dependency that the matrix only partially exposed: a **centralised copy + structured finding feed** that lives outside any individual UI component. C2's prose-nudge banner needs templated copy keyed by finding class. C3's panel needs the same finding stream rolled up by severity. C13's empty states need copy keyed by surface state. If the implementation order is C13 → C3 → C2, each later candidate inherits the copy-string-location discipline established by C13 and the finding-feed discipline established by C3, and C2 ships almost for free.

A second cross-cutting dependency is the **webui write-surface direction** (matrix open Q1). It gates exactly three sub-components: C2's `Fix with AI` button, C3's per-row `Fix` button, and C13's in-UI CTA actions. None of those are blocking for shipping the cluster; they all have CLI-handoff fallbacks that are honest about today's read-mostly webui. But the question should be answered before any of those three sub-components are designed in detail, otherwise the work gets done twice.
