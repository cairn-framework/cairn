# Tasks: Phase 2.6 Terminology Rename

## 1. Spec and canonical docs

- [x] 1.1 Update `docs/spec.md` §0 Vocabulary to define `blueprint` and `map`; remove `DSL` and `ontology` as primary terms.
- [x] 1.2 Rename all `DSL` and `.dsl` references in `docs/spec.md` to `blueprint` and `.blueprint`.
- [x] 1.3 Rename all `ontology` references in `docs/spec.md` to `map`.
- [x] 1.4 Update `docs/spec.md` §6 state layout and §10 scanner step 8 to reference `map.md` instead of `index.md`.
- [x] 1.5 Update `README.md` introduction and examples to adopt the new vocabulary.
- [x] 1.6 Update `AGENTS.md` vocabulary references.
- [x] 1.7 Bump spec version header in `docs/spec.md` from v0.6 to v0.7.

## 2. Source code — modules and types

Tasks 2.1 through 2.5 plus 3.1 (parser entry-point extension) land as a single atomic commit; intermediate states fail `cargo build` and are not acceptable on `main`. Tasks 3.2 and 3.3 MAY land in follow-up commits on the same PR branch.

- [x] 2.1 `git mv src/dsl/ src/blueprint/` and update module references throughout the workspace.
- [x] 2.2 Run `rg '\bDsl[A-Z]' src/` and rename every identifier hit to its `Blueprint` equivalent (`DslAst` → `BlueprintAst`, `DslNode` → `BlueprintNode`, `DslGrammar` → `BlueprintGrammar`, and any others the rg surface finds).
- [x] 2.3 Update `Cargo.toml` metadata (package `description`, `keywords`, `categories`, feature flag names, `[[bin]]` / `[[example]]` names) and module declarations.
- [x] 2.4 Update internal variable, function, and parameter names that carry user-visible vocabulary.
- [x] 2.5 Update error messages, log strings, user-visible JSON field names, and error codes in `openspec/registries/error-codes.md` (e.g., `E_DSL_*` → `E_BLUEPRINT_*`).

## 3. File extension and generated output

- [x] 3.1 Parser entry points accept `.blueprint` as the canonical input extension.
- [x] 3.2 Scanner emits `map.md` at project root instead of `index.md`.
- [x] 3.3 Parser rejects `.dsl` extension with an error message suggesting renaming to `.blueprint`. Hard break — no transitional support. (Planning-phase decision; see design.md Trade-offs.)

## 4. Fixtures

- [x] 4.1 Rename `test/fixtures/**/cairn.dsl` to `test/fixtures/**/cairn.blueprint` via `git mv`.
- [x] 4.2 Update fixture snapshots that reference `index.md` or contain old vocabulary in asserted output.
- [x] 4.3 Preserve historical decision-record filenames in the bootstrap fixture (e.g., `dec.dsl-as-current-state.md`); update prose inside only where inconsistent with the actual decision content.

## 5. OpenSpec conventions and registries

- [x] 5.1 Update `openspec/conventions.md` vocabulary references.
- [x] 5.2 Update `openspec/registries/declared-items.md` and `openspec/registries/error-codes.md` vocabulary.

## 6. OpenSpec and cross-repo surface sweeps

Each sweep task runs `rg -i '\b(dsl|ontology)\b' <path>` (plus `rg 'index\.md' <path>` where relevant) and asserts zero matches OR every match is justified in `ALLOWLIST.md` (see task 7.0).

- [x] 6.1 `openspec/changes/phase-3-changes/` — low impact; `change` vocabulary unchanged.
- [x] 6.2 `openspec/changes/phase-4-hooks/`.
- [x] 6.3 `openspec/changes/phase-5-edges-docstrings/`.
- [x] 6.4 `openspec/changes/phase-6-multi-target/`.
- [x] 6.5 `openspec/changes/phase-7-mcp/`.
- [x] 6.6 `openspec/changes/phase-8-summariser/` — heaviest impact.
- [x] 6.7 `openspec/changes/phase-9-brownfield/` — heavy impact; DSL authoring flow.
- [x] 6.8 `openspec/changes/phase-10-distribution/`.
- [x] 6.9 `openspec/specs/**/spec.md` — parser, cli, query, foundation, kernel, artefacts, graph-explorer.
- [x] 6.10 Archived phases under `openspec/changes/archive/**` remain untouched as historical record.
- [x] 6.11 Cross-repo surfaces: run the rg assertion against `.github/` (workflows, issue/PR templates, CODEOWNERS), `CHANGELOG.md`, docs-site config if present (`mkdocs.yml`, `book.toml`, `docusaurus.config.js`), JSON Schema files (`**/*.schema.json` — `$id`, `title`), tree-sitter grammar files (`**/*.scm`), benchmark names (`benches/`), `.claude/` and `.agents/` in-repo directories, `harness-output/` snapshots. Surfaces that do not plausibly contain the target terms (`.gitattributes`, `clippy.toml`, `rustfmt.toml`) are skipped unless the rg sweep produces a hit.
- [x] 6.12 Snapshot/transcript discovery: run `fd -e snap -e trycmd -e stdout . test/` (or equivalent) to enumerate existing snapshot/transcript files; for each hit, update via the owning tool's review command (`cargo insta review` for `.snap`, `TRYCMD=overwrite cargo test` for `trycmd`). If discovery returns empty, mark this task complete with a note.

## 7. Final sweep and changelog

- [x] 7.0 Create `openspec/changes/phase-2.6-terminology-rename/ALLOWLIST.md`. Iteratively populated: run tasks 7.1 and 7.2 first, record every surfaced match with file path, line number, and justification, then re-run 7.1/7.2 until they pass with every hit justified. Permitted justifications: historical fixture filenames (per 4.3), external citations, this phase's own Problem/Context and Rename Mapping subjects.
- [x] 7.1 Run `rg -i '\b(dsl|ontology)\b' docs/ README.md AGENTS.md openspec/ src/` and assert every match appears in `ALLOWLIST.md`.
- [x] 7.2 Run `rg 'index\.md' docs/ src/ openspec/` and assert every match appears in `ALLOWLIST.md` or is unrelated to the scanner snapshot.
- [x] 7.3 Run `rg -i '\b(dsl|ontology)\b' openspec/changes/phase-2.6-terminology-rename/` and assert every match appears in this phase's `ALLOWLIST.md` (same allowlist as 7.1/7.2). Permitted subjects-of-rename locations — proposal.md Problem/Context, design.md Rename Mapping and References, Out of Scope enumeration, spec.md legacy-rejection scenarios — are the expected allowlist entries. Any match outside the allowlist blocks the phase.
- [x] 7.4 Append a `v0.7` entry to `CHANGELOG.md` documenting the three renames and linking to this phase directory.

## 8. Required Verification

- [x] 8.1 `cargo build` passes with zero warnings.
- [x] 8.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 8.3 `cargo fmt --check` passes.
- [x] 8.4 `cargo test` passes.
- [x] 8.5 `cargo test --locked` passes.
- [x] 8.6 `python3 .agents/skills/cflx-proposal/scripts/cflx.py validate phase-2.6-terminology-rename --strict` passes.
- [x] 8.7 Integration test: `cairn scan` against a `.blueprint` fixture produces `map.md` with expected content.
- [x] 8.8 Spec version header in `docs/spec.md` reads `v0.7`.
