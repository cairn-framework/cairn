---
node: cairn.kernel.cli
status: open
created: 2026-07-28
---

# Status Todo Path Double Prefix

## Priority

P3 defect, but it is on the first output any adopter reads and it is a wire
inconsistency, not only a cosmetic one.

## Problem

`cairn status` renders every open todo path with a doubled current-directory
prefix, and `cairn todos` renders the same artefacts without it:

```
$ cairn status
Open todos:
- cairn.kernel.scanner [open] ././meta/todos/todo.contract-blueprint-staleness.md

$ cairn todos cairn.brownfield
- cairn.brownfield [open] meta/todos/todo.init-ignore-scaffolding.md
```

The same divergence is on the JSON wire. `status --json` `open_todos[].path` is
`././meta/todos/...`; `todos --json` `todos[].path` is `meta/todos/...`. A
consumer joining either field against the project root has to know which command
produced it.

Cause: `Todo.path` is stored as walked, so a relative root (`.`) joined with the
blueprint-declared `"./meta/todos"` yields `././meta/todos/...`
(`src/artefacts/registry/io.rs` `path_string`). The artefact query handlers
normalise it through `relative_path(&todo.path, root)` in
`src/query_api/serialise.rs:107` before serialising
(`todo_enriched_json`, line 119). The status surfaces do not: the human branch of
`render_status` maps `format::render::todo_line` (`src/cli/format/render.rs:101`)
straight over the raw field, and the `--json` branch calls `format::todos_json`
on the same raw values (`src/cli/render/project.rs:219`, `:234`).

Reproduced with a `main` build (`00c212a`) in this repository and in an adopter
repository (MAG, onboarded with the released 0.9.0 installer binary), so it is
not fixture-specific.

## Scope

- Normalise the path once at the boundary that owns it rather than at each render
  site. Prefer normalising when the registry records the artefact path, so every
  consumer (status, todos, decisions, research, sources, webui) sees one
  spelling; if that is too wide, route the status surfaces through
  `relative_path` and say why the registry was left alone.
- Check the sibling artefact kinds for the same raw-path leak before choosing the
  boundary; `decision_enriched_json` and friends normalise the same way, which
  suggests the registry is the real owner.
- If the stored spelling changes, `query_api::SCHEMA_VERSION` and the wire
  snapshots move per `dec.query-json-schema-version`; treat the snapshot diff as
  deliberate and path-only.

## Acceptance

- `cairn status` and `cairn todos` print the same path string for the same todo.
- `status --json` `open_todos[].path` and `todos --json` `todos[].path` agree for
  the same todo, covered by a regression test asserting the two surfaces match.
- No path field anywhere emits a `././` prefix.

Surfaced while checking a brownfield onboarding report against this repository.

2026-08-07 audit (todo.roadmap-assumption-audit): confirmed live 2026-08-07: cairn status printed ././meta/todos/... paths in this session's own output.
