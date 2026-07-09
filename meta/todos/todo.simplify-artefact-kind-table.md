---
node: cairn.kernel.artefacts
status: done
created: 2026-07-06
---

# Data-Driven ArtefactKind Table Replaces Per-Type Code

Part of todo.simplify-architecture (wave 1). Depends on: nothing.
Follow the shared rules in todo.simplify-architecture.

The frontmatter parser (`src/artefacts/frontmatter.rs`) is shared, but
todo, decision, gap, research, source, and review each carry a bespoke
struct, loader, parser branch, query serialiser, and (for
decision/todo/gap) a bespoke CLI scaffold:
`src/artefacts/registry/types.rs`, `registry/mod.rs` (`load_todos`,
`load_decisions`, `load_research`, `load_sources`, `load_reviews`),
`registry/parse.rs`, `src/cli/commands/{todo,decision,gap}.rs`. The
types are structurally identical: pointer -> directory -> frontmatter
file -> status + typed fields.

- Introduce one `ArtefactKind` descriptor table (slug prefix, directory,
  required fields, optional fields, status vocabulary) and generic
  load/parse/validate/serialise functions over it.
- Collapse the three scaffold commands into the table too (one scaffold
  function; whether the CLI spelling stays `cairn todo new` /
  `cairn decision` / `cairn gap` or becomes one command is decided in
  the CLI-family todos; here only the implementation unifies).
- Contract (`src/artefacts/contract.rs`) and change
  (`src/changes/`, directory-based) are genuine special cases: leave
  them out.
- Preserve exact validation messages where tests assert them, or update
  the tests deliberately; `src/artefacts/registry/validate/tests.rs` is
  the guard.

Acceptance: per-type loader functions gone; one table entry adds a new
artefact type end to end; `cargo test` and `cairn scan --strict` green
on this repo (which exercises every artefact type in meta/).
