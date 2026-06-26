# Cross-check: Bundle B (UX foundation)

## Scope

Bundle B as proposed by the roadmap stronghold (`getcairn-roadmap-debate.md`) and Batch A (`getcairn-refined-batch-A.md`) bundles three getcairn.dev candidates into a single near-term phase tentatively named `phase-7.7-ux-foundation`:

- **C13.a-e**: empty-state CTAs across webui and CLI, with centralised copy strings, CLI-handoff CTAs, and a voice review checklist.
- **C2.c**: prose-nudge banner translating reconciler findings into plain-English diagnostics with copy-pasteable CLI snippets. The three-axis radar visual (C2.a) is REJECTED at the roadmap level and does not enter Bundle B; only the prose-nudge layer survives.
- **C3.a-c**: a Findings rollup panel with severity buckets, scope toggles, and a `cflx check` (read: `cairn check`) command establishing single-source-of-truth discipline.

Batch A flagged that C13 and C3 share **centralised copy infrastructure**, and C2.c and C13 share the **CLI-handoff fallback** for the deferred-pending-webui-write-surface sub-components (C2.d Fix button, C3.e Fix-row button, C13.f in-UI CTAs).

This cross-check tests Bundle B against current openspec content and current code reality, makes the load-bearing decisions, and recommends final scope.

## Inputs read

- `docs/strongholds/getcairn-refined-batch-A.md`: full C2/C3/C13 refinements + cluster observation.
- `docs/strongholds/getcairn-roadmap-debate.md`: Bundle B scope statement.
- `openspec/specs/cli/spec.md` (65 lines): query surface, JSON output, CLI-as-rendering-boundary discipline.
- `openspec/specs/reconciliation/spec.md` (70 lines): finding production rules. **Validation status note:** the matrix flagged this spec as failing `cflx validate` per image #65; my own structural read shows three Requirements with two-to-four Scenarios each, well-formed Markdown, and no obvious schema violation. Whatever validation gap exists is likely a missing requirement-id field or a header-shape issue invisible to a manual read; I flag this as Open Question 1 and proceed assuming the spec content is authoritative regardless.
- `openspec/specs/parser/spec.md` (58 lines): blueprint parse error surface.
- `openspec/changes/phase-{8,9,10}*-*/proposal.md` (six files): collision check.
- `src/ui_assets/{index.html, app.js, api/}`: current webui surface (~1474 LOC in `app.js`).
- `docs/design-system/{README.md, tokens.css, components.css}`: design-system authority.
- `openspec/registries/{error-codes.md, declared-items.md}`: centralised-string precedent.
- `src/map/graph.rs`, `src/map/query.rs`, `src/cli/format.rs`, `src/ui/api.rs`: Rust `Finding` and `LintResponse` types as actually defined.

## Findings

### F1: `Finding` is already structured at the producer side, with `code` + `severity`, but only two severity buckets

The Rust kernel defines `Finding { code: String, severity: FindingSeverity, message: String, node: Option<String>, path: Option<String> }` in `src/map/graph.rs:29-40`. `FindingSeverity` has exactly two variants: `Error` and `Warning`. Codes follow the `CXNNN` pattern from the error-codes registry: `CE001-CE010` for edge findings, `CT001-CT002` for target findings (interface contradiction, rationale tension), and ad-hoc `CAIRN_CLI_*` strings used in CLI-internal errors.

This refines Batch A C2 Q2 ("does severity exist at the producer side?") and C3 Q1 ("do findings carry a stable category field?"):

- **Severity**: yes, structured, but only two buckets. Bundle B's three-bucket UI (errors / warnings / info) needs either (a) a third variant added to `FindingSeverity` (`Info`), or (b) a render-time policy that maps "no error and no warning" to "info" using the orphaned/unverified node states as info-class signals.
- **Category**: the `code` field is the de-facto category. The first two characters after `C` are the subsystem (`E` edges, `T` targets, `P` parser, `K` kernel, etc.). UI filters can group by code prefix today, no schema change required.

**Implication for Bundle B:** Both C2.c (prose-nudge: "look up copy by finding code") and C3.a-c (rollup panel: "filter by code prefix") are ready to read existing data. **The structured-finding-feed dependency Batch A flagged is already discharged.** What is missing is not a feed but a *consumer-side rendering vocabulary*, the prose-nudge copy file keyed by code, and the panel filter UI keyed by code prefix.

### F2: The reconciler-finding feed already flows to the webui via `/api/lint`; the panel is a UI-only build

`src/ui_assets/app.js:64-66` wires `fetchLint()` to `/api/lint`, which `src/ui/api.rs:107-115` serves via `query::lint(graph).findings`. The same data already drives:

- The empty-inspector "Recent findings" stub at `app.js:1076-1092` (renders top 5 findings with severity pill).
- The Changes drawer at `app.js:1184-1210` (renders all findings, badged by severity).

The CLI parallel exists too: `src/cli/format.rs:33-55` already renders findings as both human text and JSON. The `cairn lint` and `cairn scan` commands both flow through `query::lint`.

**Implication for Bundle B:** C3.b's "shared-data-source contract" is already structurally in place: `/api/lint` and `cairn lint --json` produce the same finding stream. The Bundle B ask reduces to (a) formalise this discipline in spec/cli (already partially specced, see the `--json` requirement in `specs/cli/spec.md`), and (b) decide whether to surface a new dedicated `cairn check` subcommand or repurpose `cairn lint`. There is no parallel-pipeline risk because no parallel pipeline exists.

### F3: `cflx check` is a category error in the bundle naming; the command belongs on the `cairn` binary, not the `cflx` workflow runner

CLAUDE.md is explicit (lines 119-120): *"Calling `cflx` 'cairn' is wrong; they're different tools. cflx is the workflow runner, cairn is the framework."* `cflx`'s subcommand surface is `apply | accept | archive | validate` (per CLAUDE.md, the `phase-2.6-terminology-rename/design.md`, and grep across openspec). It runs phases through their lifecycle; it does not query the map.

`cairn`'s subcommand surface (per `src/cli/mod.rs` and `specs/cli/spec.md`) is the query/scan/lint/docstring family. `cairn lint` already exists and produces the exact finding stream Bundle B wants to surface.

**Implication for Bundle B:** the proposed `cflx check` should be renamed to either `cairn check` (new subcommand, inspection mode that wraps `lint`+`scan` and prints structured findings without gating) or treated as a documentation alias on top of existing `cairn lint`. The right answer depends on whether the new command earns its own subcommand on UX grounds (e.g. clearer name, JSON-default output mode, a `--scope=node|model` flag mirroring the panel toggle). Both are honest options; Batch A's case-for is the UX clarity argument, which I find load-bearing, see D3.

### F4: Empty states exist in webui but copy is fully inline, scattered across ~10 sites in `app.js`

A grep of `app.js` for "row-empty" and "drawer-empty" classes finds 10+ inline copy strings, including:

- `app.js:871` "No paths declared on this node."
- `app.js:920` "No contracts attached."
- `app.js:926` "No decisions recorded."
- `app.js:944` "No open todos."
- `app.js:950` "No research attached."
- `app.js:956` "No sources cited."
- `app.js:962` "No outbound dependencies."
- `app.js:968` "No inbound dependents."
- `app.js:1092` "Map is clean. No findings."
- `app.js:1158` "No matches."
- `app.js:1195` "Map is clean. No findings." (duplicated)

The design-system already defines a `.row-empty` styling pattern (referenced via the existing CSS), but the design-system README has no "empty-state pattern" section and `components.css` has no dedicated empty-state component variant beyond the pill/badge primitives.

**Implication for Bundle B:** C13's "centralised copy" work has clear ground-truth scope: extract these 10+ strings, plus add a CTA layer where there is currently none (every existing empty state is a single line of declarative text, "No X.", no next-action prompt). The existing strings also fail the "name the next move" test from CLAUDE.md's voice section, they describe absence without naming what to do about it.

### F5: No centralised copy file exists in the repo; the only centralised string registries are `error-codes.md` and `declared-items.md`

A repo-wide search for `copy.*`, `strings.*`, `i18n.*`, `messages.*`, `voice.md` returns no in-repo matches (results are all in `.cache/chrome-shots/` browser-extension test data). The `openspec/registries/` directory has two files: `error-codes.md` (the `CXNNN` registry) and `declared-items.md` (a maturity tracker). Neither is a UI-facing copy registry.

**Implication for Bundle B:** Bundle B *introduces* the centralised-copy pattern; it does not extend an existing one. The location decision (Open Question Q6 from the roadmap synthesis) is therefore a design call without precedent. The two reasonable homes are (a) `docs/design-system/copy.toml` (treats copy as a design-system primitive, alongside tokens), or (b) `src/ui_assets/copy.json` (treats copy as a UI artefact, embedded into the binary like the rest of `ui_assets`). I recommend (a) for reasons I lay out in D4.

### F6: Tokens already cover everything Bundle B needs visually; no new tokens required

`docs/design-system/tokens.css` defines:

- Severity colors: `--block` (blocking contradiction), `--drift` (advisory tension), `--settled` (reconciled). Plus the parallel-but-distinct reconciliation-state set: `--synced`, `--ghost`, `--orphaned`. Note the tokens.css comment at lines 154-156 explicitly distinguishes these two color families.
- Pill components in `components.css:162-165` already render `.pill.drift` and `.pill.settled`.
- Inset/lift shadows for callout blocks (`--inset-sky`, `--lift-2`).
- Type scale, spacing, radius, motion all complete.

What's missing is not tokens but a *severity-bucket aggregate component* (the rollup panel) and an *empty-state component variant* (icon + heading + body + CTA pattern). Both are pure additions to `components.css`; both consume only existing tokens.

**Implication for Bundle B:** the visual work is fully unblocked. No token additions, no design-system-level decisions outstanding. The third-bucket question (info severity) is a token-mapping decision (orphaned/unverified → `--orphaned` wash for info-bucket rendering), not a token-addition decision.

### F7: No active phase collides with Bundle B's surface area

Reviewed proposals for `phase-{8,9,10}-*` and `phase-{8.0,9.0,10.0}-tests`:

- **phase-8-summariser**: writes draft contract updates under `.cairn/state/summariser/`. CLI surface adds `cairn draft list|show|accept|edit|discard`. Touches MCP and the artefact-edit path. **No collision** with empty-state copy or findings panel.
- **phase-9-brownfield**: `cairn init --from-code`, `cairn refine`, structural candidate extraction. Adds new artefacts but does not touch the lint/findings stream or the webui chrome. **No collision.**
- **phase-10-distribution**: LSP server. Task 1.3 says "Implement diagnostics from parser, lint, hook, and scan findings." This *consumes* the same finding stream Bundle B does, but at a different surface (LSP diagnostics protocol, not webui or CLI text). **Beneficial coupling, not collision.** If Bundle B formalises the finding-stream contract more sharply, Phase 10's LSP diagnostics work benefits.
- **phase-{8.0,9.0,10.0}-tests**: pure pre-phase test scaffolds with `#[ignore]` markers. **No collision.**

**Implication for Bundle B:** Bundle B can ship parallel to any of the active phases without coordination cost. The only caveat: if Bundle B adds an `Info` variant to `FindingSeverity`, every existing `match` over the enum (counted ~15 sites in F1) needs an `Info => ...` arm. This is a kernel-touching change that should be sequenced inside Bundle B's first commit, not retrofitted later.

### F8: The "Quality Check" rename is correctly resolved by Batch A; no further refinement needed

Batch A C3 already REJECTS the "Quality Check" name and the `COMPLETENESS`/`TRACEABILITY` MBSE-shaped categories. Confirmed by the existing tokens.css comment block (lines 116-122) explicitly using "finding-class severity" and "reconciliation state colors" as the cairn-vocabulary anchors. The panel name in Bundle B should be `Findings` (or `Reconciler findings`); category labels are the existing code prefixes (interface-contradiction, rationale-tension, edge-divergence, docstring-drift, etc.).

**Implication for Bundle B:** vocabulary scope is settled. The bundle does not need a separate "naming pass."

### F9: The webui is structurally read-only today; no write paths exist on `/api/`

`src/ui_assets/api/` directory listing: `graph`, `lint`, `meta`. All read-only endpoints. `app.js` makes only GET requests (confirmed by grep on the fetch helpers at lines 40-66). There is no `POST`, `PUT`, or `DELETE` surface on the webui's API today.

**Implication for Bundle B:** the CLI-handoff fallback Batch A proposed is not a workaround, it is the *only honest* design today. Building in-UI Fix buttons or in-UI CTA actions that would need a write API requires new backend work that Bundle B explicitly excludes. This validates the deferral of C2.d, C3.e, C13.f without further investigation.

### F10: No active "webui write-surface direction" decision exists in openspec or strongholds

`docs/strongholds/` contains `cairn-domain-expandability.md`, `getcairn-*` (the Batch A-D and roadmap synthesis), `session-handoff.md` (untracked), and `guide-to-surviving-big-code.md`. None propose a webui write surface. No `phase-*-webui-write` or similar exists in `openspec/changes/`. Phase 10 distribution explicitly says (proposal.md "Out of Scope") *"A visual graph dashboard"* and *"Hosted services"*, both gestures away from webui as a write surface.

**Implication for Bundle B:** the deferred sub-components remain deferred without active resolution path. If Bundle B blocks on this question, Bundle B does not ship. The honest move is to make Bundle B's "no webui write" constraint *explicit in the bundle proposal*, ship the CLI-handoff version, and treat the webui-write direction as a separate stronghold-level investigation (per the roadmap synthesis Section 9, N1).

## Recommendations

### R1: Rename `cflx check` to `cairn check` throughout Bundle B's scope

The roadmap stronghold and Batch A both wrote "`cflx check`" but the command surface for map queries is `cairn`, not `cflx`. Update Bundle B's proposal.md to consistently say `cairn check`, and clarify in design.md that the command is a thin inspection wrapper over the existing `query::lint` / `query::scan` flow.

### R2: Use the `code` field as the category-filter key; do not add a new `category` field

The reconciler already emits structured codes (`CE001`, `CT002`, etc.). The UI category filter can group on the second character (`E` for edge, `T` for target) without any schema change. Categorisation by raw code is more honest than introducing a parallel `category` field, because the code is *already* the canonical identifier in the registry.

### R3: Add `Info` to `FindingSeverity` as the first commit in Bundle B

The three-bucket panel UI (errors / warnings / info) needs a producer-side `Info` variant. Adding a third variant to a public enum touches ~15 match sites; this is the first piece of kernel work in Bundle B and should land before any UI consumes it. Producers that today emit nothing for orphaned/unverified states (which the panel's "info" bucket would surface) start emitting `Info` findings in Bundle B's second commit. The bucket is then full.

### R4: Make the centralised copy file a design-system artefact at `docs/design-system/copy.toml`

The file is a design-system primitive: it pairs with tokens (visual language) as the *verbal language*. Locating it inside `docs/design-system/` keeps the "design-system is the authority" discipline (CLAUDE.md) consistent and makes it equally consumable by the marketing landing site (`docs/landing/`) and the embedded webui via `include_str!` (the existing pattern for `tokens.css`). TOML is preferred over JSON because it carries comments cleanly and the file is hand-edited.

### R5: Make Bundle B's "no webui write surface" constraint explicit in the proposal

The proposal.md should explicitly state: *"Bundle B does not introduce write paths on the webui API. All CTAs in webui empty states and all 'Fix' affordances on the Findings panel render as copy-pasteable CLI commands. The webui write-surface direction is an open stronghold-level question; Bundle B's CLI-handoff fallback is correct given the current read-only API."* This pre-empts the work-twice scenario.

### R6: Defer the `--json` mode of `cairn check` if no concrete CI consumer exists

Batch A C3 Q3 asked whether a CI consumer wants `cflx check --json`. Today there is none (the `cairn lint --json` mode already covers the same ground per `specs/cli/spec.md`). Bundle B should ship `cairn check` with human-readable output by default and drop `--json` from the bundle scope. Add it later if a consumer materialises (Phase 10's LSP would be the natural one, but LSP consumes via library API, not subprocess).

### R7: Voice review checklist lives at `docs/design-system/README.md` voice section, not as a separate file

CLAUDE.md already carries the voice section (em-dash ban, plain-English bar, terminology vocabulary). Promoting that into a checklist inside the existing design-system README keeps the voice authority co-located with the design-system token authority. Adding a separate `voice.md` would create a third place where voice rules live (CLAUDE.md, design-system README, voice.md) which is itself a drift hazard.

### R8: Sweep the CLI surface for empty-state parity in the same commit as the webui sweep

Today `cairn` with no args presumably prints help (per `src/cli/mod.rs:88-122`), and `cairn lint` on a clean map prints "Findings:\nNone\n" (per `src/cli/format.rs:42`). Both are silent on next moves. The bundle's CLI sweep should produce parallel CTAs: `cairn` with no args prints "No blueprint found. Run `cairn init --from-code` to draft one from existing code, or author `cairn.blueprint` directly." `cairn lint` on a clean map prints "Map is clean. Run `cairn neighbourhood <node>` to inspect a specific area." Same voice discipline; cheap to implement.

## Decisions made (with reasoning)

### D1: Shared infrastructure question: ship sub-components in sequence, not in parallel, but in one phase

**The question (Batch A cluster observation):** Should C13 + C3 + C2.c ship together because they share centralised copy + structured-finding-feed dependencies, or sequence them in separate phases?

**Decision:** **Ship together as one phase, with strict internal sequencing.** The dependencies are real but they are *consumer-side* dependencies: C3 reads the existing finding feed (already there per F2) and C2.c reads C13's copy file (built first within the bundle).

**Reasoning:**

1. The "structured-finding-feed dependency" Batch A described as needing to be "built first" is in fact already discharged. `Finding` is structured at the producer side (F1); `/api/lint` already serves it (F2); `cairn lint` already renders it (F3). What Bundle B builds is *consumer-side rendering* (a panel and a banner), not the feed itself.

2. The "centralised copy" dependency is real but small: one file, ~50-100 lines of TOML, no schema beyond key-value-with-namespace-sections. Building it in commit 1 unblocks C2.c (which keys its prose nudges by finding code) and C13.a-e (which keys empty-state copy by surface state). Both consumers ship in subsequent commits.

3. Splitting Bundle B across phases would either duplicate the copy infrastructure (each phase owning its own keyspace) or force phase ordering that has no other reason to exist. Both are strictly worse than a single phase with strict internal commit ordering.

4. The `Info` severity addition (R3) is the only kernel-touching change and it lives at the *bottom* of the bundle's commit graph. Splitting Bundle B would require this change to land in a separate phase, which would then need its own scaffolding for one enum variant. Wasteful.

**Bundle B's internal sequencing (revised from the roadmap synthesis):**

1. Add `FindingSeverity::Info` and emit it from orphaned/unverified producers (kernel; first commit; touches ~15 match sites + ~3-5 producer sites). **NEW STEP from this cross-check.**
2. Centralised copy file at `docs/design-system/copy.toml`, with two top-level sections: `[empty-states]` keyed by surface state, `[findings]` keyed by finding code (`CE001`, etc.).
3. Voice section update in `docs/design-system/README.md` (R7); voice review checklist as bullet list there.
4. `cairn check` CLI subcommand: thin wrapper over `query::lint` and `query::scan`; produces structured human-readable output by default (no `--json` flag per R6).
5. Empty-state component in `components.css` (icon + heading + body + CTA pattern). Sweep `app.js` to replace ~10 inline empty-state strings with the new component, reading copy from the centralised file via a small JS lookup helper. Add CLI parallel empty-state copy to `src/cli/format.rs` and `src/cli/mod.rs` no-args path.
6. Findings rollup panel in `app.js`: reads from `/api/lint`, renders three severity buckets, scope toggle (whole-model vs single-node), category filter on code prefix.
7. Prose-nudge banner at top of node-detail panel: reads from `[findings]` section of the copy file keyed by finding code; renders a copy-pasteable CLI command string in place of the rejected "Fix with AI" button.

This sequencing is dependency-correct: each step consumes only artefacts produced by earlier steps. Steps 1-2 are foundation; 3 is documentation; 4 establishes the CLI single-source point; 5-7 are the user-facing work.

### D2: Webui-write-surface question: do not block Bundle B; make the constraint explicit

**The question (roadmap synthesis F1):** four sub-components defer on the webui write-surface direction. Should Bundle B wait for that decision?

**Decision:** **No. Ship Bundle B with the CLI-handoff fallback as the explicit constraint.** Bundle B's proposal.md states the constraint up front. The four deferred sub-components remain deferred. The webui-write direction is a separate stronghold-level investigation that does not gate Bundle B.

**Reasoning:**

1. The CLI-handoff fallback is not a workaround: it is the only honest design given today's read-only `/api/` surface (F9). Even if the webui-write direction were decided tomorrow as "yes, write surface," the API plumbing alone would be a multi-commit phase. Bundle B cannot ship that work in scope.

2. No active phase or stronghold proposes a webui write surface (F10). There is no "alignment target" for Bundle B to wait for. Waiting is indistinguishable from not shipping.

3. The four deferred sub-components are minor relative to the bundle's core: empty-state CTAs, prose-nudge banner, findings panel. The CLI-handoff fallback covers each user-facing need adequately. If the webui-write direction later resolves "yes," the deferred sub-components become a small follow-on phase that augments (not replaces) Bundle B's work.

4. The roadmap synthesis Section 9 N1 explicitly recommends a *separate* stronghold investigation for webui direction. That investigation's output (a positioning artefact at `docs/strongholds/webui-direction.md`) is what later unblocks the deferred sub-components. Bundle B should not absorb that scope.

**Action item for the next session, separate from Bundle B:** open `docs/strongholds/webui-direction.md` as a 2-3-day investigation. Recommended in the roadmap synthesis; not in Bundle B's scope.

### D3: `cairn check` is a new subcommand, not an alias on `cairn lint`

**The question (Batch A C3 + my own R1):** introduce a new subcommand or repurpose `cairn lint`?

**Decision:** **Introduce `cairn check` as a new subcommand.** It is structurally a thin wrapper but UX-distinctly named. `cairn lint` continues to exist for hooks (where the term "lint" is industry-standard for a pre-commit gate). `cairn check` is the user-facing inspection mode named for the panel it powers ("Findings" panel runs the equivalent of `cairn check`).

**Reasoning:**

1. `cairn lint` carries gate semantics by convention (linters block); `cairn check` carries inspection semantics (always exits 0 regardless of severity). The two semantic flavours are worth distinct commands even when their implementations call the same library function.

2. The CLI spec (`specs/cli/spec.md`) already prescribes both human and JSON output modes for every command. Adding `cairn check` does not violate that contract; it extends it.

3. The `cflx check` name in the original Batch A scope was a category error (F3); renaming to `cairn check` is the right correction. Keeping it as `cairn lint --inspect` or similar flag is awkward and fights the discoverability bar of CLAUDE.md's voice section.

4. The CLI spec's "CLI command delegates to library service" requirement (line 41-44) means `cairn check` and `cairn lint` can both call `query::lint(graph)` without semantic divergence. The single-source discipline lives at the library layer; the CLI layer just renders.

### D4: Centralised copy at `docs/design-system/copy.toml`, embedded into webui via `include_str!`

**The question (Batch A C2 Q3, C13 Q2; Tension 1 in roadmap synthesis):** where does the copy file live?

**Decision:** **`docs/design-system/copy.toml`**, embedded into the Rust webui binary at compile time using the existing `include_str!` pattern that `tokens.css` and `components.css` already follow (per `docs/design-system/README.md` "Rust web UI (embedded assets)" section).

**Reasoning:**

1. The design-system directory is already the "tokens, not hardcoded values" authority (CLAUDE.md voice section). Copy is verbal tokens; it belongs in the same authority.

2. TOML over JSON: the file is hand-edited by humans applying voice rules; TOML's comments and section headers are friendlier than JSON's. Toml is also already in the cairn dependency tree (used by `cairn.config.yaml` adjacent tooling, though primary config is YAML, TOML is the conventional Rust ecosystem choice for hand-edited config).

3. `include_str!` keeps the binary self-contained, matching the existing webui pattern. No runtime file-system dependency.

4. Marketing landing site (`docs/landing/`) can read the same file directly (static include or build step), keeping voice consistent across surfaces.

5. **Schema sketch** (one file, two top-level sections):
   ```toml
   # Empty-state copy keyed by surface state
   [empty-states.node-no-paths]
   heading = "No paths declared."
   body = "Author one or more `path` entries on this node, then run `cairn scan`."
   cta = "Run `cairn scan` after editing"

   [empty-states.node-no-contracts]
   heading = "No contracts attached."
   body = "Contracts capture the obligations this node owes its dependents."
   cta = "Author a contract under `meta/contracts/<node>.md`"

   # ... 10+ more empty-state keys

   # Prose nudges keyed by finding code
   [findings.CE001]
   heading = "A declared edge has no observed source dependency."
   body = "The blueprint says `{node}` depends on `{target}`, but no source file under `{node}`'s paths actually imports anything from `{target}`'s paths."
   cta = "Run `cairn neighbourhood {node}` to see what paths actually depend on each other"

   [findings.CT001]
   heading = "Two artefacts claim the same contract role with divergent interfaces."
   body = "..."
   cta = "..."
   ```

6. The lookup pattern from JS is small: parse the TOML once at boot (or pre-parse to JSON at compile time and embed the JSON), index by section + key.

### D5: `Info` is a producer-side severity variant, not a render-time inference

**The question (raised by F1 + R3):** how does Bundle B get its third severity bucket given that today's `FindingSeverity` is two-valued?

**Decision:** **Add `FindingSeverity::Info` as a kernel enum variant in Bundle B's first commit.** Producers for orphaned-file states and unverified-contract states (currently silent or rendered out-of-band) start emitting `Info` findings.

**Reasoning:**

1. Render-time inference of "no error and no warning means info" is an anti-pattern for the same reason that hardcoded hex values are: it puts vocabulary at the consumer that should live at the producer. The consumer would need a global view of what exists today and infer absence; the producer already knows what state it observed.

2. Adding the variant is a 15-site `match` exhaustiveness fix plus producer-side emission code. Both are mechanical. Conventions §3 (state-versioning) is honoured by the new variant being an *addition* (back-compat at the consumer side: any code that doesn't yet handle Info treats it the same as Warning, no panic).

3. The roadmap synthesis Section 4 (Tension 3) is silent on this because Batch A C2/C3 didn't trace the kernel-level implications. This decision discharges that gap.

### D6: Defer `cairn check --json` to a follow-on commit (out of Bundle B scope)

**The question (R6, Batch A C3 Q3):** does Bundle B include the `--json` mode of `cairn check`?

**Decision:** **No.** Bundle B ships `cairn check` with human-readable output only. JSON output is a follow-on if Phase 10's LSP work (or another consumer) materialises a need.

**Reasoning:** the existing `cairn lint --json` covers any CI consumer today; adding `--json` to `cairn check` is duplication without consumer demand. The CLI spec's `--json` requirement (lines 35-49 in `specs/cli/spec.md`) is met by `cairn lint --json`; adding it to `cairn check` is a "should" not a "must" until a real consumer shows up.

## Open questions for next session

### Q1: What exactly is the `cflx validate` failure on `openspec/specs/reconciliation/spec.md`? (Per image #65 referenced in scope.)

A manual structural read shows nothing obviously wrong. The failure may be a missing `## Requirements` h2 sibling or a Scenario shape issue. Needs `cflx.py validate reconciliation --strict` run against the file with the error captured. Affects Bundle B because if the spec needs editing to validate, the edit pass should batch with Bundle B's own spec deltas (which will add a `cairn check` requirement and possibly an `Info` severity scenario).

### Q2: Should the prose-nudge banner copy be templated (with `{node}`, `{target}` substitutions) or static per finding code?

The schema sketch in D4 shows templates with `{node}` and `{target}` placeholders. This requires a string-templating helper at the consumer side. A static-copy alternative is simpler (just the heading and a generic body) but less informative. Recommend the templated form, but the implementer may push back on the JS-side substitution complexity. Light call; can be decided in design.md authoring.

### Q3: What is the empty-state pattern's "icon" component? Use existing badge glyphs from `components.css` or introduce a new icon set?

`components.css:184-294` defines artefact-glyph variants (`.kind-todo`, `.kind-review`, etc.). The empty-state component could reuse these glyphs (e.g. show a faded `kind-contract` glyph for "no contracts attached"). Alternatively, a new icon set with line-art geological motifs to match the stone metaphor. Bundle B should default to glyph reuse for first ship and earmark custom illustrations for later (per Batch A C13 sub-component LATER on illustrations).

### Q4: Does CLI parallel empty-state treatment cover *every* CLI subcommand, or only the entrypoints?

Bundle B as scoped covers `cairn` with no args and `cairn lint` on a clean map. Should it also cover `cairn neighbourhood <unknown-node>`, `cairn get <unknown-node>`, etc.? The current behaviour is "exit code 1 with closest-match suggestions" (per `specs/cli/spec.md` line 25-30). That is already a reasonable empty-state-equivalent for unknown-node cases. Recommend Bundle B touch *only* zero-data empty states (no blueprint, no findings, no children), not error-path messages.

### Q5: Does the `cairn check` subcommand take a node argument for scope-toggle parity with the webui panel?

The webui panel has scope toggle "Entire model vs This node." The CLI parallel could be `cairn check` (entire model) and `cairn check <node>` (single node). Recommend: yes, for parity. The library function `query::lint` already takes a graph; filtering by node is trivial. Light call; design.md.

## Recommended Bundle B final scope

### Per-candidate scope confirmation

| Candidate | Original verdict | Cross-check status | Notes |
|---|---|---|---|
| **C13.a** empty-state component | ADOPT-NOW | **Confirmed** | Reuse existing artefact glyphs for icons (Q3 default). |
| **C13.b** CLI parallel empty-state copy | ADOPT-NOW | **Confirmed, narrowed** | Only zero-data states; not error-path messages (Q4). |
| **C13.c** CLI-handoff CTAs | ADOPT-NOW | **Confirmed** | Necessary given F9 (read-only API). |
| **C13.d** centralised copy strings | ADOPT-NOW | **Confirmed, located** | At `docs/design-system/copy.toml` per D4. |
| **C13.e** voice review checklist | ADOPT-NOW | **Confirmed, relocated** | Lives in `docs/design-system/README.md` voice section per R7, not standalone file. |
| **C2.c** prose-nudge banner | ADOPT-NOW | **Confirmed, refined** | Copy templated by finding code (D4 schema); CTA renders as copy-pasteable CLI snippet, not "Fix with AI" button. |
| **C3.a** Findings rollup panel | ADOPT-NOW | **Confirmed, renamed** | Panel labelled "Findings" or "Reconciler findings"; not "Quality Check" (per F8). |
| **C3.b** `cairn check` CLI + shared-data-source | ADOPT-NOW | **Confirmed, renamed** | `cairn check` (not `cflx check`) per F3/R1. New subcommand, not flag on `cairn lint`, per D3. No `--json` per R6/D6. |
| **C3.c** scope toggle (Entire / This node) | ADOPT-NOW | **Confirmed** | CLI parallel: `cairn check` vs `cairn check <node>` per Q5. |

### New scope item introduced by this cross-check

| Item | Rationale | Sequencing |
|---|---|---|
| **Add `FindingSeverity::Info` to the kernel enum** | Three-bucket panel UI (D5) requires producer-side info severity, not render-time inference. Touches ~15 match sites in the kernel + producer emission for orphaned/unverified states. | First commit in Bundle B, before any UI work consumes it. |

### Out of scope (deferred per D2)

- C2.d "Fix with AI" button (gated on webui write-surface).
- C3.e per-row Fix button (gated on webui write-surface).
- C13.f in-webui CTA actions (gated on webui write-surface).
- C13.g custom illustrations (gated on marketing/onboarding push).
- C3.f Re-run button + timestamp (defer; file-watch refresh is the cheap default; the deterministic reconciler doesn't need a manual re-run UI).

### Net scope change relative to roadmap synthesis

Bundle B gains one new scope item (`FindingSeverity::Info`) and loses one scope item (`--json` mode of `cairn check` per R6/D6). Renaming `cflx check` → `cairn check` is a label correction, not a scope change. The bundle is therefore approximately scope-neutral in size but more dependency-correct at the kernel level. Estimated LOC: ~1100-1800 (slightly higher than roadmap synthesis's ~1000-1700 estimate due to the kernel enum addition; estimated 50-100 LOC for `Info` variant + producers).

### Sub-component sequencing (final)

1. `FindingSeverity::Info` kernel addition + match-site fixes + orphaned/unverified producer emission. *(NEW.)*
2. `docs/design-system/copy.toml` file with empty-states + findings sections.
3. `docs/design-system/README.md` voice section update + voice review checklist.
4. `cairn check` CLI subcommand (human-readable output only).
5. Empty-state component in `components.css` + sweep `app.js` to replace ~10 inline strings + CLI parallel empty states in `src/cli/format.rs` and `src/cli/mod.rs`.
6. Findings rollup panel in `app.js` (severity buckets + scope toggle + category filter on code prefix).
7. Prose-nudge banner at top of node-detail panel (templated copy from `[findings]` section + CLI-snippet CTA).

Each step is atomic-commit-eligible and consumes only artefacts produced by earlier steps. Steps 1-2 unblock everything; 3-4 establish the documentation and CLI single-source points; 5-7 are the user-facing UI work.

---

**End of cross-check.** Bundle B is confirmed shippable as `phase-7.7-ux-foundation`, parallel to Bundle A (`phase-7.6-ai-provenance-foundation`), with one kernel-level addition (`FindingSeverity::Info`), one rename (`cflx check` → `cairn check`), one explicit constraint (no webui write surface), and one deferred sub-item (`--json` mode of `cairn check`). The shared-infrastructure dependency that Batch A flagged is already half-discharged at the producer side; what Bundle B builds is the consumer-side rendering layer plus a centralised copy file at `docs/design-system/copy.toml`.
