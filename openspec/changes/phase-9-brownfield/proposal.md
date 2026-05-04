# Proposal: Phase 9 Brownfield Extraction

**Change Type**: hybrid

## Dependencies

- `phase-9.0-tests` (required; ships the test contract this phase grades against, enabling planned stubs group-by-group).
- `phase-8-summariser` (required dependency).
- `phase-7.6-ai-provenance-foundation` (required dependency, added by Wave 4 rescope; the suggest engine consumes the suggested-edges queue file class and is gated by the `CC002` accept-time block, both ratified by phase 7.6).

Execution: MUST run after `phase-8-summariser` apply, after `phase-7.6-ai-provenance-foundation` apply, after `phase-9.0-tests` apply, and before Phase 10.

## Problem/Context

Cairn now has a mature blueprint, reconcilers, artefacts, change system, hooks, MCP access, summariser, and (post-7.6) an AI provenance foundation with a suggested-edges queue and untriaged-block accept gate. Existing projects still need a path to adopt Cairn without manually authoring the initial map from nothing.

Phase 9 implements brownfield extraction from `docs/spec.md` sections 12, 14, and 15. The Wave 4 rescope absorbs the AI-assisted authoring surfaces that other bundles assumed Phase 9 would carry (see "Wave 4 rescope" below).

## Proposed Solution

Add:

- `cairn init --from-code` to generate an initial `cairn.blueprint` and stub contracts in `openspec/changes/brownfield-init/`.
- `cairn refine` to propose deltas against an existing blueprint based on code changes.
- Structural candidate extraction from reconciler output.
- Summariser-assisted naming, descriptions, tags, and obvious edges.
- Human review through the Phase 3 change archive workflow.

### Wave 4 rescope (absorbed scope)

Per the cross-check integration in `docs/strongholds/getcairn-cross-check-integrated.md` (Pattern 3, Integrator decision 3) and the timing analysis in `docs/strongholds/oq4-phase9-rescope-timing.md` (Option B refined, trigger sharpened to phase-7.6 design.md ratification), Phase 9 absorbs four AI-assisted authoring sub-components that earlier bundles deferred to "the brownfield phase". Phase 7.6 (Bundle A) design.md ratified the identifiers this rescope cites (`suggested-edges.json` queue file path, `CC002` accept-time block, capability area `provenance-foundation`, sidecar location `<archive-root>/<phase>/.cflx-trace.json`); phase-7.6's apply must run before this rescope's apply so `CC002` is registered and the queue file class shipped. The rescope is therefore unblocked at design level and lands as a docs-only proposal-update.

Absorbed sub-components:

- **C8.c suggest engine**: an AI-driven cross-cutting edge suggester. Consumes the bounded code samples and structural candidates produced by the brownfield discovery mode, calls the summariser, and emits entries into `openspec/changes/<change>/suggested-edges.json` (the queue file class shipped by phase 7.6). Each emitted entry carries a populated `provenance` object pointing at the trace sidecar entry that produced it. The engine never writes blueprint edges directly: pending entries flow through the phase 7.6 `CC002` accept-time gate (`cflx openspec validate <change> --strict`), which blocks archive while any entry is `pending`. Humans triage by editing `triage_state` in the queue file; only `accepted` entries are eligible to materialise as blueprint edges in a later apply step.
- **C1.b interview runner**: a multi-round elicitation upgrade to the `cflx-proposal` skill, scoped to brownfield onboarding. Where the current skill runs a single-pass elicitation, the interview runner authors a session that may resume across multiple invocations, persists intermediate state inside the change directory, and supports a `--resume` form so a partial onboarding session can pick up where it left off. The genesis transcript convention from `openspec/conventions.md` Section 9 carries through unchanged: the final transcript still lands at `openspec/changes/<id>/research/genesis.md`.
- **C15 templated authoring scaffolds**: project-config-driven contract templates beyond the current stub-contract scope. Projects declare templates under a new `[templates]` block in `cairn.blueprint` (or an equivalent project config surface); the brownfield generator reads matching templates when drafting stub contracts, allowing organisation-specific authoring conventions (header structures, required sections, glossaries) to flow into generated artefacts at init time. Templates are extensible per the kernel's "tag-extensible, never closed-enum" principle in CLAUDE.md.
- **C4.b decision-attached obligations follow-on** (conditional): if the phase 9 stamping schema introduces an `obligations` field on decision artefacts (per cross-check C D4), the brownfield generator populates it for AI-suggested decisions and the human-review workflow surfaces obligations alongside the decision for triage. This follow-on is conditional on the schema actually growing the field; if Phase 9 ships without an `obligations` field, this sub-component is a no-op rider and the spec scenarios for it remain dormant.

## Acceptance Criteria

- Brownfield init never writes directly to main `cairn.blueprint` or main `openspec/specs/` artefacts.
- Generated candidates include nodes, paths, stub contracts, and observed edges that meet the deterministic threshold defined in `design.md` Candidate Extraction (at least two import observations between candidates, or one public API reference at high confidence).
- Summariser outputs are marked as proposed and require human archive.
- `refine` produces a delta instead of a full redraft when a blueprint already exists.
- False positives can be deleted from the generated change before archive.
- All strict Rust gates pass.
- Suggest-engine outputs land as `pending` entries in `suggested-edges.json` with a populated `provenance` object; archive is blocked by `CC002` until every entry is triaged off `pending`.
- Interview runner sessions resume from a partial state without losing earlier turns, and the final genesis transcript is written to the conventional path on completion.
- Templated authoring resolves project-declared templates against generated stub contracts and falls back to the built-in stub when no template matches.
- If Phase 9 ships an `obligations` field on decision artefacts, AI-suggested decisions populate the field and the field is reviewable in the generated change directory before archive.

## Out of Scope

- Perfect architecture inference.
- Autonomous archive of generated brownfield output.
- Distribution packaging, LSP, and editor plugins.
- Auto-accept policies on suggested edges (e.g., "accept all entries with confidence above 0.95"). The phase 7.6 design rejected this for the same load-bearing reason: triage must remain structurally non-bypassable.
- Interactive `cflx triage-edges <change>` command. Authors transition entries away from `pending` via a text editor; an interactive surface is left to a later phase per phase 7.6's design.
- Templated authoring for non-contract artefact types. Templates apply only to contract drafting in this phase; expansion to decisions, todos, research, or reviews is a future-phase concern.
