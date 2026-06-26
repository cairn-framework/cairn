# Working with cairn in this repository

This project uses [cairn](https://github.com/cairn-framework/cairn) to keep the
declared architecture (`cairn.blueprint`) and the code in sync. Append this
section to your CLAUDE.md or AGENTS.md, or reference it from there.

## Orientation

- `cairn context`: structural overview (nodes, edges, findings). Start here.
- `cairn get <id>` and `cairn neighbourhood <id>`: inspect a module and its
  neighbours. IDs are dotted (see `cairn.blueprint`).
- Every command accepts `--json` for machine-readable output.

## State and decisions

For project status, outstanding work, or the reasoning behind a decision, query
cairn directly. Do not infer state from freeform notes, scratch files, or
memory; the graph is the source of truth.

- `cairn status`: project summary (nodes, findings, backlog).
- `cairn changes`: active change proposals.
- `cairn decisions` and `cairn research`: the provenance chain (research feeds
  decisions, which feed changes), listed per node.
- `cairn sources <id>`: external material a node cites. The link is on the
  graph; the content stays in the referenced file.

If you are asked "what next", start with `cairn status` and `cairn changes`,
then your issue tracker. Treat any scratch or `docs/` note as secondary
context, never as current state.

## Creating artefacts

When recording a decision, research finding, or external source, place it in
the correct typed directory under `meta/` and link it to the graph:

- `meta/decisions/dec.<slug>.md` — decisions. Require `id`, `nodes:`, `status`,
  `date`. Chain to research or sources via `informed_by: [res.X, src.Y]`.
- `meta/research/res.<slug>.md` — research. Require `id`, `nodes:`. Cite
  sources via `sources: [src.Z]`.
- `meta/sources/src.<slug>.md` — external material. Require `id`, `file:`,
  `verification`. Anchors transitively; carry no `nodes:` field.

Files are FLAT (no subfolders). Use slug namespacing for grouping:
`res.gas-city.analysis`, not `research/gas-city/analysis.md`.
Non-artefact material (docs, specs, PDFs) enters provenance only as a `source`
citation: never inline its content as a typed artefact.

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
