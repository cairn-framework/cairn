# Design: Phase 2.6 Terminology Rename

## References

- `docs/spec.md` §0 Vocabulary — definitions of `DSL`, `ontology`, `artefact`, `reconciler`, `scanner`, `scan`.
- `docs/spec.md` §3 — provenance / authority chain spine; informs which terms stay load-bearing.
- `docs/spec.md` §6 — kernel components (reconciler interface, artefact type system, scanner).
- `docs/spec.md` §9 — change-directory delta semantics (`ADDED` / `MODIFIED` / `REMOVED` / `RENAMED`).
- `docs/spec.md` §10.2 — three-class finding taxonomy (structural error / interface contradiction / rationale tension).
- `docs/spec.md` §12 — `neighbourhood` query primitive.
- Prior art: `phase-2.5-graph-explorer` (archived) — cross-cutting mid-phase insertion pattern.

## Rename Mapping

| Old | New | Surface |
|---|---|---|
| `DSL` (all-caps prose acronym) | `blueprint` (lowercase) or `Blueprint` (sentence-initial) | Documentation prose, README, spec body |
| `Dsl` (Rust PascalCase prefix) | `Blueprint` | Rust type identifiers (`DslAst` → `BlueprintAst`) |
| `dsl` (lowercase stem) | `blueprint` | Module paths (`src/dsl/` → `src/blueprint/`), file stems, variable names |
| `.dsl` | `.blueprint` | File extension, CLI args, parser entry points |
| `ontology` (lowercase prose) | `map` | Documentation prose, user-facing output |
| `Ontology` (sentence-initial or type prefix) | `Map` | Sentence-initial prose, Rust types where user-visible |
| `index.md` | `map.md` | Scanner-emitted snapshot filename at project root |

`graph`, `node`, and `edge` remain as internal data-structure primitives (not user-facing copy). Every other term in spec v0.6 is preserved per the proposal's Out of Scope.

## Execution Approach

### Scripted bulk substitution

Most edits are mechanical text substitution across prose. Preferred tooling: `sd` with case-preserving expressions, or `ast-grep` for code-aware edits.

- Substitute `DSL` / `Dsl` / `dsl` → `Blueprint` / `Blueprint` / `blueprint` across `**/*.md`, `**/*.rs` comments, `**/*.json` user-visible fields.
- Substitute `ontology` / `Ontology` → `map` / `Map` across the same file set.

Review the full diff before committing. Flag ambiguous matches (e.g., `DSL` inside a quoted external citation, or `ontology` in a footnote referencing prior-art literature) for manual review.

### Module and filename renames

Use `git mv` to preserve history:

- `src/dsl/` → `src/blueprint/` (update all imports and module declarations).
- `test/fixtures/**/cairn.dsl` → `cairn.blueprint`.
- Scanner output path: `index.md` → `map.md`; update fixture snapshots that assert on this path.

### Type identifier renames

Rust types embedding `Dsl` (e.g., `DslAst`, `DslNode`, `DslGrammar`) rename to their `Blueprint` equivalents. Internal graph primitives (`GraphNode`, `GraphEdge`) are untouched.

Public API boundary — CLI, user-visible JSON shapes, error messages, and log lines — adopts the new vocabulary. Internal implementation identifiers may retain old names temporarily if touching them materially expands scope, but the public surface MUST be consistent at phase end.

### Parser / file extension handling

- Parser entry points that match on `.dsl` update to `.blueprint`.
- Default: hard break — no transitional `.dsl` support.
- If bootstrap fixtures or embedded tooling prevent single-commit migration, a transitional path (parser accepts both with a deprecation warning) MAY be introduced and documented as a task-level decision. This path is opt-in.

### OpenSpec phase-doc updates

Phases 3–10 have proposals, designs, tasks, and specs referencing the old vocabulary in prose. Update these in-place as part of this phase. Structural content (what each phase builds, in what order) is unchanged; only prose vocabulary adjusts.

Phase 8 (summariser) and phase 9 (brownfield) have the heaviest prose impact because they explicitly reference ontology snapshots and DSL authoring flows. Phase 3 (changes) has near-zero impact because `change` is kept.

Archived phases under `openspec/changes/archive/` are historical records. Do NOT rewrite them; they stand as the original record. New phases authored after 2.6 use new vocabulary natively.

### Ordering

1. Update `docs/spec.md` first — canonical source; all other docs align to it.
2. Rename Rust modules — establishes the new import paths.
3. Rename types, variables, parameters, and user-facing strings inside modules.
4. Update CLI, fixtures, and JSON output shapes.
5. Sweep OpenSpec phase docs for prose-level references.
6. Update `openspec/conventions.md` and `openspec/registries/*.md`.
7. Run the verification battery.

## Trade-offs

- **Hard break vs transitional `.dsl` extension.** Hard break is the default. CAIRN's Rust rewrite is pre-ship; no external users hold `.dsl` files that must keep working. Transitional support only materializes if the rename cannot land in a single commit without breaking internal tooling.
- **Module rename bundled with type renames.** One logical change, one commit. Larger diff but cleaner semantic boundary than splitting.
- **Prose-only vs identifier-level renames.** Both, because mixed vocabulary in the codebase would cost more later than it saves now.
- **Archived phases preserved.** Historical record retained; future readers see the terminology of the era in which each phase was authored.

## Risks

- **Silent text in non-tracked locations.** Rendered docs, generated changelogs, harness outputs may lag. Mitigation: final grep sweep for `DSL`, `Dsl`, `dsl`, `ontology`, `Ontology`, `index.md` at repo root.
- **Third-party references.** Agent memory files and external docs reference old terms; those are updated separately and do not block this phase.
- **cflx workflow tooling.** The rename does not touch `cflx` itself but may interact with its snapshots. Verified via the cflx validate gate.
- **Decision-record filenames in bootstrap fixture.** `dec.dsl-as-current-state.md` and similar historical filenames are preserved as-is; prose inside them is updated only where inconsistent with the actual decision content.
- **Mid-execution regression.** Because tasks 2.1–2.5 land as one atomic commit and the phase lands as a single squash-merged PR, rollback is `git revert <merge-sha>`. Partial-rename states on `main` are forbidden; if verification fails mid-phase, the work remains on a feature branch until fixed.

## Testing

- `rg -i '\b(dsl|ontology)\b' docs/ README.md AGENTS.md openspec/ src/` returns zero matches, with a documented allow-list for historical fixture filenames.
- `rg 'index\.md' docs/ src/ openspec/` returns only intentional occurrences (not the scanner snapshot file).
- `cargo build` with zero warnings; `cargo clippy --all-targets --all-features` with `-D warnings`; `cargo fmt --check`; `cargo test`; `cargo test --locked` all pass.
- Fixture snapshot tests confirm scanner output produces `map.md` rather than `index.md`.
- CLI integration test: `cairn scan` against a `.blueprint` fixture produces the expected `map.md` output.
- cflx validate on `phase-2.6-terminology-rename` passes `--strict`.
