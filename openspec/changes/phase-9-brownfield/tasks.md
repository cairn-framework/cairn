# Tasks: Phase 9 Brownfield Extraction

## 1. Candidate Extraction

- [x] 1.1 Add repository-wide discovery mode that extracts source roots and module-like directories without requiring an existing `cairn.blueprint`.
- [x] 1.2 Cluster files by path ownership and observed dependency density.
- [x] 1.3 Produce candidate nodes, paths, edges, confidence scores, and evidence paths.
- [x] 1.4 Implement deterministic fallback heuristics for minimum file counts, max depth, coupling score bands, observed edge thresholds, and sample byte limits.
- [x] 1.5 Add fixture tests for simple, nested, mixed-language, low-confidence, and high-coupling repositories.

## 2. Summariser Integration

- [x] 2.1 Build bounded summariser inputs from candidates and code samples with at most five files per candidate and 4,000 bytes per file.
- [x] 2.2 Generate names, descriptions, tags, and stub contract prose.
- [x] 2.3 Implement disabled-mode fallback using mechanical path-derived names.
- [x] 2.4 Add deterministic fake-backend tests.

## 3. Init from Code

- [x] 3.1 Implement `cairn init --from-code`.
- [x] 3.2 Generate `openspec/changes/brownfield-init/proposal.md`.
- [x] 3.3 Generate `blueprint.delta` with added nodes and edges.
- [x] 3.4 Generate stub contracts in the change directory.
- [x] 3.5 Fail safely when the target change exists unless `--force` is provided.

## 4. Refine

- [x] 4.1 Implement `cairn refine` against an existing blueprint.
- [x] 4.2 Generate delta operations instead of a full blueprint replacement.
- [x] 4.3 Detect likely renames from path and similarity evidence.
- [x] 4.4 Add tests for additions, removals, modifications, and renames.

## 5. Suggest Engine (Wave 4 rescope, absorbs C8.c)

- [x] 5.1 Implement a cross-cutting edge suggester that consumes bounded code samples and structural candidates from section 1 plus summariser output from section 2.
- [x] 5.2 Emit suggested edges into `openspec/changes/<change>/suggested-edges.json` using the schema ratified by phase-7.6-ai-provenance-foundation; populate the `provenance` object with the producing trace sidecar phase and stage.
- [x] 5.3 Set `triage_state` to `pending` for every emitted entry; never write `accepted` directly from the engine.
- [x] 5.4 Confirm interaction with the phase-7.6 `CC002` accept-time gate: a brownfield change with pending entries fails `cflx openspec validate <change> --strict` until triage is complete.
- [x] 5.5 Add fixture tests covering populated provenance, empty provenance fallback, schema-version mismatch handling, and gate-block on pending entries.

## 6. Interview Runner (Wave 4 rescope, absorbs C1.b)

- [x] 6.1 Extend the `cflx-proposal` skill with a multi-round elicitation mode scoped to brownfield onboarding sessions.
- [x] 6.2 Persist intermediate session state inside the change directory so a partial session survives across invocations without leaking outside `openspec/changes/<change>/`.
- [x] 6.3 Add a `--resume` form (or equivalent) that picks up an in-progress session at the next outstanding question.
- [x] 6.4 Write the final transcript to `openspec/changes/<id>/research/genesis.md` per `openspec/conventions.md` Section 9; preserve the no-rewrite-on-apply discipline.
- [x] 6.5 Add tests covering session start, resume across two invocations, abandonment cleanup, and final transcript shape.

## 7. Templated Authoring (Wave 4 rescope, absorbs C15)

- [x] 7.1 Define a project-config surface (`[templates]` block in `cairn.blueprint` or equivalent) for declaring contract templates with name, glob/tag matchers, and a body schema.
- [x] 7.2 Resolve matching templates against generated stub contracts during init and refine; merge template-supplied content with summariser-supplied content per a documented precedence rule.
- [x] 7.3 Fall back to the existing built-in stub when no template matches; never fail authoring on missing templates.
- [x] 7.4 Add fixture tests for matching templates, non-matching fallback, multi-template precedence, and ill-formed template rejection.

## 8. Decision-Attached Obligations Follow-On (Wave 4 rescope, conditional on C4.b)

- [x] 8.1 If decision artefacts grow an `obligations` field in this phase, populate it on AI-suggested decisions emitted by the brownfield generator. (No-op: decision schema does not declare `obligations` field at apply time.)
- [x] 8.2 Surface the populated `obligations` field in the generated change directory so a human reviewer can triage it before archive. (No-op: conditional on 8.1.)
- [x] 8.3 If decisions retain the existing schema (no `obligations` field), record the no-op explicitly in design.md and skip implementation; tests for this section may stay `#[ignore]` until the schema lands.

## 9. Documentation

- [x] 9.1 Document human review workflow for generated brownfield changes.
- [x] 9.2 Document confidence scores and evidence paths.
- [x] 9.3 Register `init --from-code` and `refine` in the shared MCP query tool registry as mutating tools.
- [x] 9.4 Document limitations of architecture inference.
- [x] 9.5 Document the suggest engine's queue-file contract, the interview runner's resume semantics, and the templated authoring precedence rule.

## 10. Required Verification

- [x] 10.1 `cargo build` passes with zero warnings.
- [x] 10.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 10.3 `cargo fmt --check` passes.
- [x] 10.4 `cargo test` passes.
- [x] 10.5 `cargo test --locked` passes.
- [x] 10.6 `python3 .claude/skills/cflx-proposal/scripts/cflx.py validate phase-9-brownfield --strict` passes.
