---
id: dec.artefact-filename-rule
nodes:
  - cairn.kernel.artefacts
  - cairn.root
status: superseded
date: 2026-07-27
informed_by: [res.artefact-filename-drift-audit]
---
# Artefact filenames: slug-only for id-bearing kinds, `todo.` for todos

## Context

`docs/conventions.md` section 10 and `AGENTS.md` both state that artefact
filenames are slug-only and that the typed prefix lives only in the `id:`
frontmatter. Measured on 2026-07-27 the repository did not follow its own rule:
26 of 80 decisions, 9 of 24 research files, and 4 of 14 sources carried a
`dec.`/`res.`/`src.` filename prefix, while all 134 todos carried `todo.`.

Nothing detected any of it. The registry reads `id` purely from frontmatter
(`src/artefacts/registry/kinds.rs`) and never compares it to the filename, so
the split widened silently for as long as the convention has existed.

Three things already agreed with the written rule, which is what makes this a
ratification rather than a coin toss:

- `cairn decision new <slug>` writes `meta/decisions/<slug>.md` and treats
  `meta/decisions/dec.<slug>.md` as a legacy path it must not collide with
  (`src/cli/commands/decision.rs`).
- The shipped copy calls the prefixed form "legacy"
  (`docs/design-system/copy.toml`).
- The shipped loop guidance hand-authors research as `meta/research/<slug>.md`
  (`.claude/skills/cairn-loop-reconcile/SKILL.md`).

Todos pull the other way for a mechanical reason: `cairn todo new <slug>` and
`cairn todo set <slug> <status>` both resolve a slug by constructing
`meta/todos/todo.<slug>.md` literally. There the prefix is load-bearing.

One sentence of the convention was also simply false: "This matches every
existing artefact; a filename-prefix rule would flag the entire current corpus."

## Decision

For the three id-bearing artefact kinds (decision, research, source) the
filename stem is the artefact `id` with its typed prefix stripped. `id:
dec.no-orchestrator` lives in `meta/decisions/no-orchestrator.md`. Slug
namespacing survives intact, because only the final extension is stripped:
`id: res.gas-city.analysis` lives in `meta/research/gas-city.analysis.md`.

Todos keep `meta/todos/todo.<slug>.md`, because the CLI resolves slugs through
that exact path and a todo carries no `id` field to compare against.

`CAIRN_ARTEFACT_FILENAME_DRIFT` (CA038, Warning) enforces both halves during
artefact loading. All 41 non-conforming files are renamed in the same change
that adds the check (40 in this repository's own `meta/` tree, plus one in the
bundled `examples/demo` corpus), so the rule and its enforcement land together.

## Rationale

Comparing the filename to the `id` is strictly stronger than checking for an
absent prefix. It also catches a file named `bar.md` that declares
`id: dec.foo`, which is a real defect and was previously invisible.

Two alternatives were rejected.

Typed prefixes everywhere would align filenames with todos and make `ls`
self-describing, but the parent directory already carries the type, it means 79
renames instead of 40, and it contradicts the write surface the tooling already
ships.

Permitting both forms, and checking only that the filename maps to the `id`,
would require no renames at all. It was rejected because it makes the legacy
branch in `cairn decision new` permanent, and it leaves the corpus visibly
inconsistent, which is the drift this decision exists to end.

Warning, not Info, and the consequence is deliberate: `cairn scan --strict`
exits non-zero on Warning as well as Error, so the rule is actually gated
rather than merely reported. An Info finding would reproduce the condition
this decision exists to end, where the convention held only for as long as
someone remembered it. Adopting repositories therefore see drift fail their
first strict scan; that is the same treatment `CAIRN_TODO_ORPHAN_NODE`
already gives artefact hygiene, and renaming to conform is mechanical.

## Consequences

- `docs/conventions.md` section 10 loses its false "matches every existing
  artefact" claim and gains the id-to-filename correspondence rule.
- 41 files are renamed: 39 carrying a typed prefix, one research file named
  after its folder rather than its id, and one demo todo missing the `todo.`
  prefix. Renaming changes no `id`, so no provenance link breaks; the 7 live
  prose references and 1 Rust source reference are updated in the same change.
- Archived material under `meta/changes/archive/` keeps its old paths. It is a
  historical record of what those files were called at the time, and no gate
  resolves prose paths.
- Reviews and contracts are out of scope. Section 10 never named them, and
  contracts are keyed by node rather than by a typed id.
- Slug charset is out of scope. `todo.Bad.md` satisfies this rule but is not
  addressable through `cairn todo set`, which requires a kebab slug. That is a
  separate rule and deserves its own finding rather than being folded into a
  code whose remediation is a filename.
