# Cross-check: Bundle E (cflx export) + C1.a (genesis transcript)

## Scope

Two NOW-or-near-NOW items from Batch D's lifecycle refinement:

1. **Bundle E**: `cflx export --json` (highest-leverage single piece) and `cflx export --md` (NOW-or-next), with CSV deferred LATER and PPTX/DOCX rejected as probably permanent. The "Assets stays in provenance chain" pattern was peeled off to RESEARCH.
2. **C1.a**: extend the existing `cflx-proposal` skill to write the elicitation transcript as a `research/genesis.md` artefact tied to the resulting decision. Does NOT include the multi-round interview UI (C1.b, deferred to Phase 9), and does NOT include the confidence-pill (C1.c, REJECTED for unsolved calibration).

Both are CLI-surface NOW work and share command-surface concerns (notably the cflx-vs-cairn binary question that surfaced in Bundle B's cross-check). This pass tests the proposals against current openspec content, the actual CLI source at `src/cli/mod.rs`, the `cflx-proposal` skill body, AGENTS.md, the active phase headers, and the cli + artefacts capability specs.

## Inputs read

- `/Users/george/repos/cairn/docs/strongholds/getcairn-refined-batch-D.md` (C1, C12 sections)
- `/Users/george/repos/cairn/docs/strongholds/getcairn-roadmap-debate.md` (Bundle E and C1.a slotting; verdict table; open questions Q31-37)
- `/Users/george/repos/cairn/openspec/specs/cli/spec.md` (65 lines, three requirements)
- `/Users/george/repos/cairn/openspec/specs/artefacts/spec.md` (72 lines, three requirements)
- `/Users/george/repos/cairn/src/cli/mod.rs` (472 lines: full CLI command registry, parse_args, render dispatch)
- `/Users/george/repos/cairn/.claude/skills/cflx-proposal/SKILL.md` (460 lines: full skill body)
- `/Users/george/repos/cairn/AGENTS.md` (45 lines)
- `/Users/george/repos/cairn/CLAUDE.md` (cflx workflow section)
- Active phase proposal headers via glob: `phase-8-summariser`, `phase-8.0-tests`, `phase-9-brownfield`, `phase-9.0-tests`, `phase-10-distribution`, `phase-10.0-tests`
- `/Users/george/repos/cairn/docs/strongholds/getcairn-cross-check-B.md` (precedent on the cflx-vs-cairn binary question; D3, F3, R1 of that document)
- Direct invocation of `python3 .claude/skills/cflx-proposal/scripts/cflx.py` to confirm what the validator actually checks (changes, not living specs)

## Findings

### F1: `cflx export` does not exist on either binary today; this is genuinely new surface

The CLI registry in `src/cli/mod.rs` lines 168-225 dispatches: `get`, `neighbourhood`, `files`, `todos`, `decisions`, `research`, `sources`, `rationale`, `status`, `hook`, `changes`, `show`, `docstring`, `rename`, `dependents`, `depends`, `contract`, `order`, `lint`, `scan`. Plus top-level `init`, `ui`, `archive`, `--version`. There is no `export` command and no `--export` flag anywhere. There is also no separate `cflx` Rust binary: `cflx` is the Python workflow runner under `.claude/skills/cflx-proposal/scripts/cflx.py` (subcommands: `list`, `show`, `validate`, `archive`), not a Rust CLI. So Bundle E is **not** an extension; it is a wholly new surface, and the binary question (F2 below) determines whether the new surface lands on the Rust `cairn` CLI or on the Python `cflx.py` workflow runner.

### F2: The cflx-vs-cairn binary distinction is sharper than Batch D framed it

Batch D wrote `cflx export` throughout. Bundle B's cross-check (R1, F3, D3 in `getcairn-cross-check-B.md`) already established the principle: **`cflx` operates on phase lifecycle (propose/apply/accept/archive); `cairn` operates on framework data (blueprint, map, artefacts, graph)**. By that test:

- **Export of framework data is unambiguously cairn-side.** A JSON or MD snapshot of the graph + artefact corpus is the same shape of work as `cairn neighbourhood --json` or `cairn rationale`: a renderer over kernel state. Batch D Q35 asks "what is the JSON schema?" and considers "direct serialisation of in-memory graph" as a candidate; that is exactly the `query::*` output mode the existing cli spec already prescribes (`spec.md` lines 36-49: every CLI command provides `--json` mode with a stable schema). The natural shape is `cairn export --json` (or `cairn export --md`), reusing the existing `--file`, `--changes-dir` parsing in `parse_args` (lines 88-122).
- **Proposal authoring (cflx-proposal) is structurally a cflx-side concern.** It writes into `openspec/changes/<change-id>/` which is a phase artefact; it runs `cflx.py validate <id> --strict`; it lives in the `.claude/skills/` directory under the cflx skill family. The skill name `cflx-proposal` is correct as-is.

So C1.a's name stays. Bundle E's name needs to change. This is an exact recurrence of the Bundle B finding: the roadmap synthesis labelled the inspection command `cflx check` but the right name was `cairn check`. The same mistake is being made for export.

### F3: `cflx-proposal` exists as a skill, not a binary command; C1.a is a skill-body edit plus a convention

`.claude/skills/cflx-proposal/SKILL.md` is the canonical proposal-authoring entrypoint. It is a Claude-skill with embedded scripts (`scripts/cflx.py` for list/show/validate/archive) and a documented interactive flow (lines 67-262 of the skill body). The skill currently writes:

- `openspec/changes/<id>/proposal.md`
- `openspec/changes/<id>/tasks.md`
- `openspec/changes/<id>/design.md` (optional)
- `openspec/changes/<id>/specs/<capability>/spec.md`

It does **not** write a `research/` subdirectory or any artefact tying the elicitation transcript to a decision. The skill explicitly captures elicitation context in the user-facing response under a `Premise / Context` section (lines 26, 70-80), but that section lands in chat, not on disk. There is no transcript-as-research artefact today. C1.a is therefore a real addition, not a no-op. The change is small: a new step in the interactive workflow that materialises the elicitation transcript (or its summary) at `openspec/changes/<id>/research/genesis.md` and adds an `informed_by:` reference from the (eventual) decision artefact.

### F4: The `research/` location is novel inside change directories; the schema for `research` is loose

Today `research` artefacts live under `meta/research/` (per `cli/mod.rs` test fixture lines 398, 420-422, 438-441) at the repo level, not inside change directories. The cflx-proposal skill writes to `openspec/changes/<id>/{proposal.md, tasks.md, design.md, specs/}` and nothing else. So `openspec/changes/<id>/research/genesis.md` is a new convention. This is structurally clean: it mirrors the `specs/` subdirectory convention already in use, but introduces a new sibling that the validator (`cflx.py validate <id> --strict`) does not currently look for. Means C1.a may or may not require a corresponding `cflx.py` validator update depending on whether the genesis artefact should be required (block) or optional (skip silently).

The artefacts spec at `openspec/specs/artefacts/spec.md` defines what a `research` artefact looks like at the repo level (lines 6-15: scanner loads research into the map; lines 25-30: research can be in a decision's `informed_by` chain; lines 47-72: query surface). Critically the artefact spec is silent on **where** research artefacts live. It only requires that they parse, link to nodes, link to sources, and be referenced by at least one decision. So genesis.md inside a change directory is consistent with the spec **provided** it has the required frontmatter (id, nodes, date, sources) and **provided** the eventual decision links to it via `informed_by`. The genesis.md schema question (Batch D C1 Q3: "full transcript, summary plus turns, or just architecture-signals output?") is real but bounded by the existing `research` schema in `docs/spec.md` lines 433-437.

### F5: The artefacts spec passes structural validation; the "fails validate per image #65" claim doesn't reproduce against the current head

The brief instructs me to flag the validation status of `openspec/specs/artefacts/spec.md`. Direct invocation of `python3 .claude/skills/cflx-proposal/scripts/cflx.py validate --strict` reports failures only against `phase-0-foundation` (missing proposal.md, missing tasks.md, no spec deltas). The validator does not validate living specs at `openspec/specs/<area>/spec.md` directly, it operates on changes (proposals plus deltas) per `cflx.py`'s subcommand surface (`list`, `show`, `validate`, `archive`). Reading the artefacts spec by hand: it has a clean `## Requirements` heading, three `### Requirement:` blocks, and every requirement has at least one `#### Scenario:` block (lines 16, 22, 33, 41, 51, 59, 67). Format-wise it matches `openspec/specs/cli/spec.md` (which the brief asserts passes). I cannot reproduce a current-head validate failure for the artefacts spec.

The image-batch reference in the brief may be stale (image #65 is from an earlier session) or may refer to a different validator (perhaps an OpenSpec linter that checks per-spec rather than per-change). Either way, the **structural** spec is fine for C1.a's purposes: the `research` artefact type is defined, the integrity rule (every research must link to a node and a source) is stated, and the `informed_by` linkage from decisions to research is specified. C1.a can proceed without waiting for a hypothetical artefacts-spec fix.

### F6: The active phase corpus has zero `cflx export` or `cairn export` references

Grep across `phase-8-summariser`, `phase-8.0-tests`, `phase-9-brownfield`, `phase-9.0-tests`, `phase-10-distribution`, `phase-10.0-tests` proposal/design/tasks files yields no mention of an export command. Phase 10 (`distribution`) covers LSP, plugin packaging, and reconciler extension registration, all *outbound* surfaces, but does not include export. So Bundle E is an entirely orthogonal surface; it does not collide with any active phase, and there is no risk of double-implementation if it lands as its own commit.

### F7: AGENTS.md does NOT instruct codex to capture elicitation transcripts; C1.a is additive there too

AGENTS.md (45 lines) instructs the apply-stage codex agent on: change-directory orientation (line 8), where conventions/registries live (lines 11-15), test-first pre-phase convention (lines 25), UI/design-system rules (lines 27-36), and guardrails (lines 38-46). It does not instruct the agent to write any genesis transcript. This makes sense because AGENTS.md is for the *apply* stage (codex runs `cflx apply` against an already-authored proposal) whereas C1.a runs at *propose* stage. So C1.a's home is squarely in the cflx-proposal skill body, not in AGENTS.md. The skill change is the only required edit.

### F8: The cli spec already requires both human and JSON output modes; this constrains Bundle E's surface positively

`openspec/specs/cli/spec.md` line 36-53 makes both human and JSON output mandatory for every CLI command, with `--json` emitting "exactly one JSON object" including a "schema version" and no extra logging. This means a future `cairn export` cannot ship JSON-only or MD-only, it must support both modes plus their respective formats. The natural surface is `cairn export --format json|md` (or twin commands `cairn export-json` / `cairn export-md`) where the format flag determines payload shape and the existing `--json` flag is *meta* (does the CLI emit a JSON object at the process boundary, with the format-specific payload as a string field, vs writing the format-specific payload directly to stdout). This is a real design call that the Bundle E proposal must make.

The simpler option: `cairn export --format json` writes the JSON payload directly to stdout (or to `--output <path>`), and `--json` here means "machine-readable mode," which is implicitly always-on for the JSON format and would be a no-op (or contradiction) for MD format. The cli spec's "schema version" requirement applies to the JSON format payload itself. MD format ships without `--json` mode by construction (it's not JSON). This requires a small carve-out in the cli spec, or alternative interpretation: the Bundle E command falls under the existing `--json` requirement only when `--format json` is selected.

## Recommendations

### R1: Rename `cflx export` → `cairn export` throughout Bundle E's documents

The roadmap synthesis (`getcairn-roadmap-debate.md`) and Batch D (`getcairn-refined-batch-D.md`) both used `cflx export`. Both are wrong by the same category-error analysis Bundle B applied to `cflx check`: export is a snapshot of framework data, not a phase-lifecycle operation. The Rust CLI binary is `cairn`; the Python workflow runner is `cflx.py`. The export command lives on `cairn`. Update Bundle E's stronghold note, future proposal.md, and any roadmap references.

### R2: Keep `cflx-proposal` skill name (no rename)

C1.a's home stays as `cflx-proposal` because proposal authoring is a phase-lifecycle concern (writes into `openspec/changes/<id>/`, runs `cflx.py validate`, drives the propose stage). Renaming would invert the principle. The skill *can* (and per C1.a, should) write a `research/genesis.md` artefact inside the change directory; that is a side effect of the propose stage, not a framework-data query.

### R3: Bundle E ships as a single small cairn-binary commit, not a phase

The Bundle E scope is two render modes over the same in-memory graph the `cli/mod.rs` `scan_result` already exposes. Estimated LOC: ~150-300 (per Section 7 of the roadmap synthesis: "~250-400 LOC total"). This does not merit a phase scaffold. Recommendation: ship as a small commit attached to whichever near-term infrastructure phase has space, or as a standalone commit with a one-line spec delta against `openspec/specs/cli/spec.md` adding a `cairn export` requirement.

### R4: C1.a ships as a skill-body edit plus a tiny convention note in AGENTS.md and conventions.md

The C1.a change is:
1. Add a step ~7.5 in the cflx-proposal skill workflow (between "Write Spec Deltas" and "Validate Proposal"): "Write elicitation transcript as `openspec/changes/<id>/research/genesis.md`".
2. Define the genesis.md frontmatter: `id`, `nodes`, `date`, `sources` (per the artefacts spec lines 47-72 + docs/spec.md line 433). The `id` convention is `genesis-<change-id>`; the `nodes` field references the node(s) the proposal touches; the `date` is the propose-stage date; `sources` may be empty if the elicitation produced no externally-citable references, or may include conversation logs/issue links.
3. Add a one-line convention note to `openspec/conventions.md` documenting the `research/genesis.md` convention.
4. AGENTS.md gets no edit (apply stage doesn't author genesis).

### R5: Defer the JSON schema versioning question for Bundle E to design time

Batch D Q35 ("what is the JSON schema?") is real but designable inside Bundle E's commit, not before. The cli spec already mandates `schema_version` in every JSON payload (line 51). Recommendation: ship `cairn export --format json` with `schema_version: 1` and a flat `{ schema_version, generated_at, nodes, edges, artefacts, changes }` shape. Iteration is allowed; the `schema_version` field is the migration handle.

### R6: Defer Batch D Q34 (default destination), make `--output <path>` mandatory at first

Batch D Q34 asked: "Where does an exported JSON live by default? User-specified path, `target/cairn-export/` (gitignored), or `openspec/exports/` (tracked)?" Recommendation: punt by making `--output` required. No default means no ambiguity. If a default emerges from usage patterns (likely stdout for piping, or `cairn-export.json` in cwd), add it later. The "Assets stays in provenance chain" research (C12.e) will eventually settle whether tracked exports under `openspec/exports/` are right; until then, no default is safer than the wrong default.

### R7: Genesis.md schema: full transcript by default, with a `summary` field

Batch D Q33 asks whether genesis.md should hold the full transcript, a summary plus turns, or just architecture-signals. Recommendation: full transcript as the body, with a `summary` field in the frontmatter or a `## Summary` H2 at the top. Storage is cheap; provenance is the point; future tooling can derive summaries from the body. The skill already produces a `Premise / Context` section (lines 26, 70-80 of SKILL.md); that section maps directly to the `## Summary` block.

### R8: Bundle E's `--md` format should NOT be the map.md format

The map.md format is for the consolidated post-archive map (per the terminology rename in CLAUDE.md). Bundle E's `--md` is a different shape: a flat human-readable report of the current graph state, suitable for paste-into-issue / GH-discussion contexts (per Batch D's framing). Recommendation: the `--md` format produces a single Markdown document with sections for nodes (grouped by parent system/container), edges (verb-labelled per spec §7), and active artefact summaries (decisions accepted, todos open, contracts present). This is closer to a flattened decision-log than to map.md. Distinct artefact, distinct purpose.

## Decisions made (with reasoning)

### D1: Bundle E command name → `cairn export`, not `cflx export`

**Decision:** Bundle E commands are `cairn export --format json` and `cairn export --format md`. Not `cflx export`.

**Reasoning:** Three independent lines converge:

1. **Binary-distinction principle (Bundle B precedent).** `cflx` runs phase lifecycle; `cairn` runs framework-data queries. Export is a query over framework data (the graph + artefact corpus). Same category-error correction Bundle B applied to `cflx check`.
2. **Implementation locus.** The CLI source for Rust commands is `src/cli/mod.rs`, dispatched through the `cairn` binary. The `cflx.py` Python script under `.claude/skills/` has subcommands `list`, `show`, `validate`, `archive`, none of which are query-rendering. Adding export to `cflx.py` would mean implementing a graph parser in Python that mirrors the Rust scanner. That is parallel-truth-source drift of exactly the kind Batch D's duplication-avoidance table warned against (line 271 of `getcairn-refined-batch-D.md`: "`cflx export` reads same data, different format"). The "same data" only stays the same if both readers share the Rust implementation.
3. **Spec coverage.** `openspec/specs/cli/spec.md` already mandates `--json` mode with schema versioning for every CLI command. Adding `cairn export` to the cli spec is a small ADDED-Requirements delta. Adding `cflx export` to a hypothetical cflx spec creates a new spec area whose surface barely exists today.

### D2: C1.a does not block on artefacts-spec validation; proceed with caveat

**Decision:** Ship C1.a as a NOW item without waiting for the artefacts spec. The artefacts spec at `openspec/specs/artefacts/spec.md` is structurally clean against the current `cflx.py validate --strict` pass; the "fails validate per image #65" claim does not reproduce against current head. The `research` artefact type is sufficiently defined (cli/mod.rs test fixture proves the loader works; docs/spec.md lines 433-437 give the schema; artefacts/spec.md lines 25-30, 53-57 give the integrity rules).

**Caveat:** C1.a should record explicitly in its design.md (or stronghold note) that the genesis.md frontmatter must satisfy the existing artefacts spec, `id`, `nodes`, `date`, plus a `sources` reference list (which may legitimately be empty for purely-elicited research, but the schema field must be present). If a future artefacts-spec edit tightens the `research` schema, genesis.md may need a frontmatter sweep. That is acceptable migration cost.

**Reasoning:** Two points: (a) the validator does not block C1.a today; (b) even if it did, fixing the artefacts spec is orthogonal, C1.a writes a research artefact whose shape is defined by docs/spec.md, not by the openspec capability spec. The capability spec describes what the *scanner* must do with research artefacts; C1.a writes one that the scanner can later consume. The two are loosely coupled.

### D3: Genesis.md frontmatter schema is defined now (in this cross-check) to unblock C1.a

**Decision:** The `research/genesis.md` frontmatter is:

```yaml
---
id: genesis-<change-id>
nodes: [<change-id>]    # use the change ID itself as a placeholder node;
                        # see reasoning below
date: <YYYY-MM-DD>      # propose-stage date
sources: []             # populated if the elicitation cited external links/transcripts
informed_by: []         # always empty (genesis is a primary research artefact)
type: genesis           # new sub-type marker per docs/spec.md line 449 type-list
                        # (reuses the source-type field convention; not strictly required)
---
```

**Body:** `## Summary` H2 block (~3-6 bullets, mirroring the existing `Premise / Context` skill output) followed by `## Transcript` H2 block with the full elicitation conversation rendered as Q/A turns or chat-log lines.

**Reasoning:** The `nodes` field is the trickiest schema choice. The artefacts spec (lines 11-15) requires research to "attach parsed records to the node or global provenance index," which permits both per-node and global research. The proposal stage often does not yet know which graph node the change touches (the proposal is *deciding* that). Recommendation: use the change-id as a placeholder node identifier. When the change applies, the apply-stage codex re-points the genesis research's `nodes` at the actual node(s) touched, or moves it to the global provenance index. This is consistent with how todos, decisions, and other artefacts already use change directories as scratch space.

### D4: Bundle E ships as one commit with a JSON-first ordering

**Decision:** Bundle E is one commit (or two atomic-grouped commits) on the `cairn` binary, not a phase. The implementation order: JSON first (~150-200 LOC), MD second (~100-150 LOC, reuses JSON serialisation as input). Both shipped in the same PR. Spec delta: one ADDED-Requirements section in `openspec/specs/cli/spec.md` covering both formats.

**Reasoning:**
- Per R3, the LOC estimate is ~250-400 total. Below the conventions.md §3 module-size pressure threshold and below the typical phase scaffold cost.
- JSON-first is right because: (a) Batch D explicitly named JSON as "highest-leverage single piece"; (b) MD format is naturally a transformation of the JSON shape (a Markdown renderer over the JSON tree), so building JSON first means the MD implementation gets a free pre-validated input model; (c) the cli spec already mandates `--json` for every command, so JSON is the default-correct output anyway.
- One commit (or atomic-grouped commits) is right because the two formats share a serialisation core; splitting them across commits would either duplicate the core or land an unused surface in the first commit.

### D5: C1.a does NOT introduce a `cflx interview` parallel command

**Decision:** Reaffirming Batch D's verdict: C1.a is purely a `cflx-proposal` skill extension. No new CLI command. No new skill. The interview-runner CLI mode (C1.b) stays gated on Phase 9 brownfield's elicitation needs.

**Reasoning:** The skill already does interactive elicitation today (lines 67-262 of SKILL.md walk through the multi-step interactive workflow). Adding a parallel `cflx interview` command would duplicate that flow. The minimal additive change is one new step in the existing workflow that materialises the transcript to disk. That is the "extend `cflx-proposal`'s output" pattern Batch D recommended.

### D6: Bundle E is lifecycle-orthogonal: no `cflx accept` integration

**Decision:** `cairn export` reads current state regardless of cflx lifecycle stage. It does not block on the verification battery, does not check change-directory state, does not interact with `cflx accept`'s gate logic. It is a pure read of the current graph.

**Reasoning:** Batch D Q36 explicitly asked "Does `cflx export` run before or after the verification battery?" with the recommended answer "orthogonal." Confirming. Export is a snapshot tool; it should work whether the project is in a clean state or mid-phase. The cli spec's existing pattern (commands like `cairn lint` exit 1 on errors but `cairn neighbourhood` always exits 0) suggests `cairn export` should always exit 0 on successful render, regardless of whether the underlying graph has findings. Findings are a separate concern.

### D7: C1.a does NOT update the cflx.py validator to require genesis.md

**Decision:** The genesis.md artefact is convention-encouraged but not validator-enforced. `cflx.py validate <id> --strict` continues to require `proposal.md`, `tasks.md`, and a non-empty `specs/` delta tree. It does not require `research/genesis.md`.

**Reasoning:** Two reasons: (a) the cflx-proposal skill is the canonical authoring path, and the convention is encoded there; not every proposal will be authored through the skill (a human can draft a proposal directly), so the validator should not block on genesis.md absence; (b) the artefacts spec requires research artefacts to link to at least one source (line 466 of `docs/spec.md`'s integrity rule), but a propose-stage genesis with no external sources is a legitimate edge case (the research is the conversation itself). Adding validator enforcement would force every proposal to fabricate sources or skip genesis. Better to ship the convention without enforcement, observe usage, and decide later whether enforcement helps.

## Open questions for next session

1. **Genesis.md lifecycle on apply.** When `cflx apply` runs, does the apply-stage codex re-point genesis research's `nodes` at the actual node(s) touched? Move it to the global provenance index? Leave it in the change directory? The simplest answer is "leave it; archive carries it into the merged spec" but this needs design-pass confirmation. Affects both AGENTS.md (whether codex needs an instruction) and the archive operation (whether `cflx.py archive` must move the file).

2. **Bundle E's `--scope` flag.** Should `cairn export` support `--scope phase|spec|all`? Batch D Q36 hints at this. Recommendation: ship without scope flag in v1 (export everything); add the flag only if a concrete consumer asks for partial export. This is a follow-on YAGNI question.

3. **Whether genesis.md should include conversation system prompts.** The skill body's elicitation includes implicit context from CLAUDE.md, AGENTS.md, the user's prompt, prior session messages. Should genesis.md include those system-level inputs (full reconstruction) or just the user-visible Q/A turns (cheaper, less sensitive)? Echoes Batch C C6 Q1 about cflx-trace sidecar contents. Recommendation: just user-visible turns + the final premise. Skip system prompts unless a forensic need emerges.

4. **MD format's verbosity bound.** A full-graph MD export of a mature cairn map could be very long. Should `cairn export --format md` truncate by default? Have a `--depth N` flag? Always emit everything? Recommendation: always emit everything; truncation is the consumer's job. But this is worth one design call inside Bundle E's commit.

5. **Whether the `cflx-proposal` skill's `Premise / Context` chat output is removed once it lands on disk.** Today the skill prints `Premise / Context` to chat. Once it also writes genesis.md, do we keep both (chat for immediate human review, file for provenance) or only the file (chat shows a "wrote genesis.md" note)? Recommendation: keep both. The chat version is immediate review; the file is durable provenance. They are not redundant.

6. **`cairn export --format md` vs `cairn map`.** There is currently no `cairn map` command but the terminology rename made `map.md` a generated artefact name. Does the export-md command produce something that should be canonicalised as `map.md`, or is it a separate artefact (per R8)? Recommendation per R8: separate artefact. But this could surface as a "two MD shapes is one too many" complaint later; flag for revisit if the existing `map` consolidated spec ever grows a `map.md` generator.

## Recommended Bundle E + C1.a final scope

### Bundle E final scope (renamed and locked)

| Sub-component | Final verdict | Notes |
|---|---|---|
| `cairn export --format json` | ADOPT-NOW (renamed from `cflx export --json`) | Highest-leverage single piece. Lands on the `cairn` Rust binary. Schema version 1 with `{schema_version, generated_at, nodes, edges, artefacts, changes}` flat shape. `--output <path>` required at first (no default). |
| `cairn export --format md` | ADOPT-NOW or NEXT (renamed from `cflx export --md`) | Same commit or immediate follow-on. Renders a flattened decision-log MD report; **not** the map.md format (per R8). Reuses JSON serialisation as input. |
| `cairn export --format csv` | DEFER (LATER) | Demand-gated per Batch D. No flip from roadmap. |
| `cairn export --format pptx|docx` | REJECT (probably permanent) | Confirmed per spec section 5. Render subsystem not in cairn's scope. |
| "Assets stays in provenance chain" pattern | RESEARCH (separate design pass, not in this commit) | Confirmed per Batch D. Decides whether `--output` defaults to a tracked location (`openspec/exports/`) vs untracked. |
| Webui settings-pane export UI | DEFER | Gated on webui write-surface direction (open question N1 in roadmap synthesis). |
| `cairn export` lifecycle integration | NONE (lifecycle-orthogonal per D6) | Always exits 0 on successful render. Reads current graph regardless of cflx stage. |

**Final shape:** A single commit (or atomic-grouped pair) on the `cairn` Rust binary adding `cairn export --format <json|md> --output <path>`, with one ADDED-Requirements delta against `openspec/specs/cli/spec.md`. ~250-400 LOC. No phase scaffold needed.

### C1.a final scope (locked)

| Sub-component | Final verdict | Notes |
|---|---|---|
| `cflx-proposal` skill writes `research/genesis.md` | ADOPT-NOW | Skill-body edit, not a kernel change. |
| Genesis.md frontmatter schema | DEFINED (per D3) | `id: genesis-<change-id>`, `nodes: [<change-id>]`, `date`, `sources: []`, `informed_by: []`, `type: genesis`. |
| Genesis.md body shape | DEFINED (per R7) | `## Summary` H2 (3-6 bullets, from existing `Premise / Context`) + `## Transcript` H2 (full elicitation conversation). |
| Genesis.md location | `openspec/changes/<id>/research/genesis.md` | New convention; mirrors `specs/` subdirectory. No validator enforcement (per D7). |
| AGENTS.md update | NONE | Apply stage doesn't author genesis. |
| Conventions.md update | ONE-LINE NOTE | Document the `research/genesis.md` location convention so the convention is discoverable. |
| `cflx.py validate` update | NONE | Genesis is encouraged, not enforced (per D7). |
| Multi-round interview UI (C1.b) | DEFER (Phase 9-gated) | Confirmed per Batch D. No flip. |
| Confidence-pill UI (C1.c) | REJECT | Confirmed per Batch D. No flip. |
| Architecture-signals panel (C1.d) | DEFER (Phase 9+) | Confirmed per Batch D. No flip. |

**Final shape:** One commit editing `.claude/skills/cflx-proposal/SKILL.md` (adds step 7.5 between Spec Deltas and Validate Proposal) plus a one-line addition to `openspec/conventions.md` documenting the location convention. ~50-100 lines of skill prose change. No code change. No spec delta required (the artefacts spec already covers `research` shape generically).

### Joint sequencing

Bundle E and C1.a are independent and parallel-shippable. They share no infrastructure. Suggested order: ship C1.a first (smaller, no code), then Bundle E (slightly larger, real Rust). Both can land before any of the larger phase work (Phase 7.5c verification states, Phase 7.6 AI provenance foundation, Phase 7.7 UX foundation).

---

**End of cross-check.** Bundle E renames `cflx export` → `cairn export` and ships as a single commit on the Rust binary with two formats (JSON-first, MD-next), no phase scaffold, lifecycle-orthogonal. C1.a is a `cflx-proposal` skill-body edit that adds a `research/genesis.md` write step with a defined frontmatter schema and a defined body shape; no code change, no validator enforcement, AGENTS.md untouched, one-line conventions.md note. Both items are shippable now and proceed independently of the artefacts-spec validation question (which doesn't reproduce against current head). The single load-bearing correction in this cross-check is the `cflx` → `cairn` rename for export, which is the same category-error correction Bundle B applied to `cflx check` and which preserves the binary-distinction principle: cflx for phase lifecycle, cairn for framework-data queries.
