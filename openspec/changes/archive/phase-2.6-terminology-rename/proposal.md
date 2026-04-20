# Proposal: Phase 2.6 Terminology Rename

## Dependencies

- Requires: `phase-2.5-graph-explorer`.
- Execution: MUST run after Phase 2.5 and before `phase-3-changes`.

## Problem/Context

CAIRN's audience is broadening from career developers to people building with AI tools, and the framework will extend beyond code to non-code domains. Two top-level user-facing terms in spec v0.6 gatekeep that audience without earning their keep:

- `DSL` — a programming-language term that overclaims executability. The `.dsl` file is a declarative description, not a grammar users run.
- `ontology` — a knowledge-representation term implying formal machinery (OWL, DL reasoners) CAIRN does not possess.

Both have plainer, more accurate replacements. The rest of spec v0.6's vocabulary is load-bearing technical taxonomy and MUST NOT be flattened. Four rounds of structured adversarial debate converged on exactly three renames.

Phase 3 authors "changes" vocabulary in new specs, designs, and tasks; writing phase 3 in old terminology would bake the wrong vocabulary into the changes system itself and accrue rename debt across phases 4–10. Phase 2.6 is the natural window: after 2.5 ships, before 3 starts implementation.

## Proposed Solution

- **Rename `DSL` / `.dsl` to `blueprint` / `.blueprint`.** Household metaphor; declarative-not-executable; cross-domain; aligns with the blueprint→build flow AI coding agents follow.
- **Rename `ontology` to `map`.** Already used colloquially in README and spec §2; plainer for non-devs; the "stacked map" framing is sharper than "ontology."
- **Rename generated snapshot `index.md` to `map.md`.** Consistency with ontology → map; avoids collision with web-server and docs-site `index.md` defaults.
- **Scripted bulk substitution** covers the mechanical majority (~500 edits, ~100 files); manual review covers semantic edges (parser grammar rules, module renames, fixture extensions, public type identifiers).

## Acceptance Criteria

- `docs/spec.md` uses `blueprint` for the authored file and `map` for the reconciled graph throughout; file extension references updated to `.blueprint`; §6 state layout and §10 scanner output reference `map.md`; spec header bumped to v0.7.
- `README.md`, `AGENTS.md`, `openspec/conventions.md`, and `openspec/registries/*.md` reflect the new vocabulary.
- Rust source module `src/dsl/` renamed to `src/blueprint/`; public types embedding `Dsl` renamed to `Blueprint` equivalents where user-visible.
- CLI accepts `.blueprint` as the canonical file extension.
- Fixture files under `test/fixtures/**/cairn.dsl` renamed to `cairn.blueprint`; generated `index.md` outputs renamed to `map.md`.
- Phase 3–10 proposals, designs, tasks, and specs updated for prose-level `DSL` and `ontology` references.
- Final grep sweep confirms zero user-facing occurrences of `DSL` or `ontology` in `docs/`, `README.md`, `AGENTS.md`, `openspec/`, and `src/` (with documented allow-list for historical fixture filenames).
- All verification gates pass (`cargo build`, `clippy`, `fmt --check`, `test`, cflx validate).

## Out of Scope

- `reconciler`, `scanner`, `scan` — three distinct kernel components; kept distinct.
- `artefact` — typed-schema kernel primitive (§6 component 2); kept as umbrella term.
- `rationale tension` — advisory non-blocking finding class (§10.2); kept distinct from `interface contradiction` (blocking).
- `change` / `changes/` — carries delta semantics (§9); `proposal.md` lives inside it; OpenSpec coexistence is explicit.
- `neighbourhood` — graph primitive (§12, §4); `related` collides with ADR frontmatter (§8.3).
- `provenance chain` / `authority chain` — spine of §3; hinge framing collapses without them.
- `interface hash` — mechanical primitive (§3.2, §10); kept.
- `ghost` / `synced` / `orphaned`, `drift`, `divergence`, `verified` / `external` / `unverified`, `hinge` — already plain; kept.
- Delta keywords `ADDED` / `MODIFIED` / `REMOVED` / `RENAMED` — OpenSpec interop; verbatim.
- `graph`, `node`, `edge` — internal data-structure primitives (not user-facing copy).
- Direct artefact type names: `contract`, `decision`, `todo`, `research`, `review`, `source` — kept.
- `finding`, `structural error`, `interface contradiction` — enforcement taxonomy; kept.
- DSL grammar rules themselves — only the file extension and the umbrella term change; grammar structure unchanged.
- External blog posts, tweets, historical docs, agent memory files outside this repo — out of scope.
