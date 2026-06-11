# Working with cairn in this repository

This project uses [cairn](https://github.com/cairn-framework/cairn) to keep the
declared architecture (`cairn.blueprint`) and the code in sync. Append this
section to your CLAUDE.md or AGENTS.md, or reference it from there.

## Orientation

- `cairn context`: structural overview (nodes, edges, findings). Start here.
- `cairn get <id>` and `cairn neighbourhood <id>`: inspect a module and its
  neighbours. IDs are dotted (see `cairn.blueprint`).
- Every command accepts `--json` for machine-readable output.

## While coding

- New source files must fall under a module `path` in `cairn.blueprint`.
  If none fits, extend a module's paths or declare a new module. This includes
  test directories (`./tests` and the like), not just production code.
- Run `cairn scan` before committing. Zero findings is the target.
- `cairn hook all` is the strict gate; exit 0 means the commit is safe.

## Feeding back to cairn itself

If cairn misbehaves, surprises you, or gets in your way (a confusing message,
a wrong finding, a missing capability), record it before moving on:

    cairn feedback "what you expected, and what happened instead"

Entries accumulate in `.cairn/feedback.md`, and the command prints a prefilled
link for filing the report upstream on the cairn issue tracker.
