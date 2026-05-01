# Design: Phase 9 Brownfield Extraction

## References

- `docs/spec.md` section 12 for `init --from-code` and `refine`.
- `docs/spec.md` section 14 for brownfield MCP tool registry.
- `docs/spec.md` section 15 for brownfield approach.
- Phase 3 for change directory archive semantics.
- Phase 8 for summariser backend and draft safety.
- Phase 7.6 (`phase-7.6-ai-provenance-foundation`) for the trace sidecar (`<archive-root>/<phase>/.cflx-trace.json`), the suggested-edges queue file (`openspec/changes/<change>/suggested-edges.json`), the `CC002` accept-time block, and the `provenance-foundation` capability area; cited by the Wave 4 rescope sections below.
- `docs/strongholds/getcairn-cross-check-integrated.md` (Pattern 3, Integrator decision 3) for the rescope rationale.
- `docs/strongholds/oq4-phase9-rescope-timing.md` for the Option B refined timing verdict that placed this rescope after phase 7.6's design.md ratification.
- `openspec/conventions.md` Section 9 for the genesis transcript convention referenced by the interview runner.

## Candidate Extraction

Brownfield extraction SHALL use a repository-wide discovery mode that does not
require an existing `cairn.blueprint` or claimed node paths. This mode SHALL reuse
the language scanners and ignore rules from reconcilers, but it SHALL produce
candidate structures rather than normal map reconciliation reports.

The discovery mode SHALL scan the codebase and produce structural candidates:

- Top-level source roots.
- Major subdirectories with cohesive files.
- File clusters with strong internal coupling.
- Observed dependency edges between candidates.

Candidates SHALL include confidence and evidence paths. Low-confidence candidates SHALL remain in generated output but be marked for review.

Deterministic fallback heuristics SHALL apply before summariser naming:

- Source roots SHALL be directories containing at least three source files in supported languages after ignore rules are applied.
- Candidate directory depth SHALL be limited to four levels below the repository root unless explicitly configured.
- A directory SHALL become a module candidate when it contains at least three source files or at least two source files and one internal import edge to another candidate.
- File clusters SHALL be grouped by nearest common directory when no stronger candidate exists.
- Coupling score SHALL be `(internal_imports + 1) / (external_imports + 1)`. Candidates with score `>= 2.0` are high confidence, `>= 1.0` are medium confidence, and below `1.0` are low confidence.
- Observed edges SHALL be emitted when there are at least two import observations from one candidate to another, or one public API reference with high confidence.
- Summariser samples SHALL include at most five files per candidate and at most 4,000 bytes per file, preferring public interface files before implementation files.

## Summariser Role

The summariser SHALL name and describe candidates, suggest tags, and draft stub contract content. It SHALL receive structural candidates and bounded code samples, not the entire repository. If the summariser is disabled, Cairn SHALL generate mechanical names and descriptions from paths.

## Init Flow

`cairn init --from-code` SHALL create `openspec/changes/brownfield-init/` containing:

- `proposal.md`.
- `blueprint.delta` with added nodes and edges.
- Stub contracts under mirrored `contracts/`.
- Optional generated research notes explaining extraction evidence.

The command SHALL fail if `openspec/changes/brownfield-init/` already exists unless `--force` is provided.

## Refine Flow

`cairn refine` SHALL compare current code reality with current blueprint and produce a new change directory containing only proposed additions, removals, renames, or modifications. It SHALL not replace the whole blueprint.

## MCP Tools

Phase 9 SHALL register brownfield commands in the shared query tool registry so `cairn-mcp` exposes them through MCP when mutating tools are enabled. `cairn_init_from_code` and `cairn_refine` SHALL be `mutating` tools because they create change directories and write proposed artefacts.

## Testing

Tests SHALL use fixture repositories. Coverage SHALL include deterministic candidate thresholds, depth limits, coupling score bands, edge thresholds, sample byte limits, disabled summariser fallback, deterministic fake summariser output, init change generation, refine delta generation, force behavior, archive compatibility, and MCP registry exposure for brownfield mutation tools.

The Wave 4 rescope sub-components add fixture coverage for: suggested-edges queue emission with populated provenance, untriaged-block interaction with `cflx openspec validate <change> --strict` (asserting `CC002` exit on pending entries), interview runner resume across two invocations, templated authoring matching and fallback, and (conditionally) decision-attached obligations population.

## Wave 4 rescope sub-components

The Wave 4 rescope absorbs four AI-assisted authoring sub-components into Phase 9. The integrated cross-check (`docs/strongholds/getcairn-cross-check-integrated.md`, Pattern 3) explains why Phase 9 is the natural home for each. The timing analysis (`docs/strongholds/oq4-phase9-rescope-timing.md`) explains why this proposal-update lands after phase 7.6's design.md ratification rather than before or concurrent.

### C8.c. Suggest engine

The suggest engine is the brownfield-side producer for the queue file class shipped by phase 7.6. It runs after section 1's deterministic candidate extraction and section 2's summariser pass, and emits cross-cutting edges that the deterministic extractor cannot infer (e.g., "saas.api.auth depends_on saas.api.identity" inferred from a summariser-read README plus a public-API call pattern, not from a direct import).

**Inputs**: structural candidates (section 1 output), summariser samples (section 2 input bundle), and the trace sidecar context for the running phase (provided by the cflx workflow runner).

**Output location**: `openspec/changes/<change>/suggested-edges.json`, sibling to `proposal.md`, `blueprint.delta`, `design.md`, and `tasks.md`. Schema is the v1 schema ratified by phase 7.6 (`source`, `target`, `relation`, `confidence`, `provenance`, `triage_state`, `triage_note`).

**Provenance population**: every emitted entry sets `provenance.trace_phase` to the running phase ID and `provenance.stage` to the cflx stage that called the engine (typically `propose` for `cairn init --from-code` or `refine`). Manual-test entries authored without a trace context leave `provenance` as an empty object per the phase 7.6 schema's optional-field discipline.

**Triage state**: the engine SHALL only emit entries with `triage_state: "pending"`. Setting any other state at emission time would bypass the phase 7.6 `CC002` gate and break the load-bearing constraint that triage stays human-driven.

**Gate interaction**: `cflx openspec validate <change> --strict` reads the queue, counts pending entries, and fails with `CC002` when the count is non-zero. A brownfield change containing fresh suggest-engine output cannot archive until a human walks the queue and transitions every entry to `accepted`, `rejected`, or `deferred`.

**Confidence policy**: the engine never auto-accepts on confidence. The phase 7.6 design rejected confidence-thresholded auto-accept for the same reason this phase rejects it: the gate's value is its non-bypassability.

**Refine activation**: the suggest engine fires for both `cairn init --from-code` and `cairn refine`. Refine-emitted entries set `provenance.stage = "propose"` against the refine-time change directory's `suggested-edges.json` and are subject to the same `CC002` accept-time gate. This preserves the engine's value as code grows: cross-cutting edges that emerge between refine cycles are captured at the same human-triage gate as init-time suggestions, not silently dropped.

### C1.b. Interview runner

The interview runner extends the `cflx-proposal` skill with a multi-round elicitation mode scoped to brownfield onboarding. The current single-pass elicitation suits well-understood proposals but degrades in brownfield, where the human author may not know the full module structure of an existing codebase ahead of time.

**Session lifecycle**: an interview session opens at `openspec/changes/<change>/research/interview-session.json` (a transient peer to `genesis.md`) and persists open questions, in-progress answers, and a cursor pointing at the next outstanding turn. The session file is private to the interview runner and is removed (or marked complete) when the session terminates, leaving only `genesis.md` as the durable transcript.

**Resume semantics**: invoking the skill against an existing change directory with a non-complete `interview-session.json` SHALL pick up at the cursor, not restart. A `--resume` form (or the absence of a `--new` flag, design choice deferred to apply) makes this explicit.

**Genesis transcript discipline**: when the session completes, the runner writes `openspec/changes/<id>/research/genesis.md` per `openspec/conventions.md` Section 9. The transcript carries the user-visible Q/A turns plus the final premise; system prompts and intermediate session-file content stay out per cross-check C1.a's recommendation.

**Scope boundary**: the interview runner is brownfield-onboarding-specific in this phase. Other proposal types continue to use the existing single-pass elicitation. A future phase MAY generalise the runner; that generalisation is out of scope here.

### C15. Templated authoring scaffolds

Templated authoring lets organisations declare contract templates that the brownfield generator consumes when drafting stubs, beyond the current "minimum viable stub contract" path.

**Project-config surface**: a new `[templates]` block in `cairn.blueprint` declares templates with: a unique `name`, a `match` rule (glob over candidate path or tag list), and a `body` template containing required headers, optional sections, and placeholder markers. If the apply-time agent finds blueprint grammar constraints prevent a `[templates]` block, the agent SHALL document the alternative location and update the corresponding spec scenario accordingly. The block is extensible per the kernel's "tag-extensible, never closed-enum" principle (CLAUDE.md "What cairn is, positively" §2).

**Resolution order during draft**: the brownfield generator resolves templates in declared order; the first matching template wins for a given candidate. If no template matches, the generator falls back to the built-in stub. A failed-to-parse template logs a warning and is skipped, never blocking authoring.

**Merge with summariser output**: template body provides structure (headers, required sections); summariser provides content (names, descriptions, suggested tags). Where both supply text for a section, summariser content takes precedence and template text becomes a guidance comment in the draft. The precedence rule is documented per task 9.5.

**Scope boundary**: templates apply only to contract drafting in this phase. Decision, todo, research, and review templates are deferred to a later phase.

### C4.b. Decision-attached obligations follow-on (conditional)

This sub-component is conditional on the Phase 9 decision-stamping schema growing an `obligations` field. Cross-check C D4 explicitly defers the decision: if the schema gains the field, this phase populates it for AI-suggested decisions; if it does not, this section is a no-op rider and the corresponding spec scenarios remain dormant.

**If the field exists**: AI-suggested decisions emitted by the brownfield generator populate `obligations` with the structured items the summariser identified (e.g., "downstream contract MUST cite this decision" expressed as a typed obligation entry). The generated change directory surfaces obligations alongside the decision body so a human reviewer can triage, edit, or remove obligations before archive.

**If the field does not exist**: the brownfield generator emits decisions with the existing schema; section 8 of `tasks.md` records the no-op explicitly; section 8 tests stay `#[ignore]` until the schema lands.

**Forward compatibility**: when a future phase adds the `obligations` field, the section 8 work activates without re-opening this phase's design. The conditionality is captured in the spec deltas with parallel scenarios for the field-present and field-absent cases, so dormant scenarios do not regress validate-strict. The apply-time agent picks the active branch by reading the decision-stamping schema in force at apply time.

## Wave 4 atomic-commit grouping

The Wave 4 sub-components form four atomic-commit groups layered on top of the existing task sections 1-4. Suggested orderings for review-narrative clarity:

1. **Suggest engine group**: section 5 tasks. Depends on the existing summariser pipeline (section 2) and the phase 7.6 queue file class. Lands as a single grouped commit set.
2. **Interview runner group**: section 6 tasks. Depends on the existing `cflx-proposal` skill body. Independent of the suggest engine group; can land in parallel.
3. **Templated authoring group**: section 7 tasks. Depends on the existing init flow (section 3) and refine flow (section 4). Independent of the suggest engine and interview runner groups.
4. **Obligations follow-on group**: section 8 tasks. Conditional on schema state at apply time; lands last or stays dormant.

The four Wave 4 groups commit in any order with respect to the pre-existing task sections 1-4, subject to the within-group dependency rules above. Pre-existing task sections 1-4 commit as individual logical units per the graphite-pr discipline. The cflx-runner enforces group-level boundaries when the phase declares them.
