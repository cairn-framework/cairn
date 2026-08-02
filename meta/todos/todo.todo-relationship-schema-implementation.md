---
node: cairn.kernel.artefacts
status: done
created: 2026-07-31
related: [dec.todo-relationship-model]
---

# Implement the todo relationship schema

Implementation unit for `dec.todo-relationship-model` rulings 1 to 4. The
decision is the ruling; this todo owns the surfaces.

## Task

1. Parse `blocked_by:` (list of todo stems), `parent:` (single todo stem),
   and `related:` (list of `dec.`/`res.`/`src.` ids or todo stems) on the
   Todo struct, per the reference rules in ruling 2 (stem identity,
   resolution against loaded todos, no rename cascade, symmetric reading
   of `related:`).
2. Scanner findings with copy in `docs/design-system/copy.toml` under
   `[findings.codes]`: dangling relationship reference (Warning), a cycle
   through `blocked_by` or through `parent`, detected per graph, not
   across their union (Error), and the status-contradiction advisory.
   The unresolved-blocker rule, both forms, and the exemptions live in
   ruling 4 of `dec.todo-relationship-model`, the single normative
   copy; this unit implements them without restating them.
3. CLI: an author-and-inspect surface for the three fields (surgical
   frontmatter edits through the sanctioned todo write path, plus
   read-side rendering in `cairn todos`/`cairn get`); follow the
   command-reference consistency tests for any new verb or flag.
4. Expose the parsed fields on the todo wire shapes; bump
   `query_api::SCHEMA_VERSION` and `ui::SCHEMA_VERSION` per
   `dec.query-json-schema-version` and `dec.webui-json-schema-version` if
   the serialised shape changes, and regenerate wire snapshots.
5. Tests per conventions (`test_<unit>_<condition>_<outcome>`): parse of
   each field, each finding on a fixture (including a mixed
   parent/blocked_by chain that must NOT fire the cycle Error, and a
   blocked todo with no `blocked_by:` that must stay silent), and a
   no-finding pass over this repository's own todos, which include
   blocked-without-declared-blocker cases today.

## Acceptance

- The three fields parse and validate on this repository's todos; the
  forward-declared entries already present in `meta/todos/` (for example
  on `todo.roadmap-derived-view` and `todo.overharness-console-ux`)
  resolve cleanly.
- Each finding fires on a fixture and carries copy.toml text; the mixed
  parent/dependency chain fixture stays silent.
- The schema + CLI + scanner prerequisite of
  `todo.todo-relationship-model-and-issue-links` is fully satisfied by
  this unit, leaving that todo blocked on nothing else.
- `cairn scan` on this repository stays at zero non-Info findings; gates
  in `scripts/pre-archive-rust-gates.sh` pass.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It is campaign step 1 for declaring todo relationships.
