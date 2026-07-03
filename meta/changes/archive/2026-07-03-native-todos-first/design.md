# Design: native-todos-first

## Approach

Mirror the shipped `cairn decision new` pattern for the new write verb, and
flip render priority (native todos before beads) at the two consumption
points (`cairn next`/`cairn brief`, `cairn status`) rather than introducing
a new abstraction over "task sources". Both branches already share the same
shape (a title, an optional node, a "run this to see more" hint), so the
priority flip is a small `if let` guard ahead of the existing beads branch,
not a rewrite.

## Changes

ADDED:
- `src/cli/commands/todo.rs`: `cairn todo new <slug> --node <id>`, mirroring
  `decision.rs` (kebab-slug validation, exists-check, `create_dir_all`,
  deterministic frontmatter). Shares `is_kebab_slug`/`flag_values`/
  `title_from_slug`/`today_utc` with `decision.rs` (widened to
  `pub(super)`) instead of duplicating them.
- `src/cli/render/changes_view.rs`: `render_changes`/`render_show`, human
  renderers for `cairn changes`/`cairn show`, removed from the JSON-only
  guard in `src/cli/mod.rs`.
- `[todo]` table in `docs/design-system/copy.toml` (`usage`,
  `invalid-slug`, `missing-node`, `exists`, `created`), mirroring
  `[decision]`.
- `[empty-states.cli-no-changes]` copy entry for the new `cairn changes`
  empty state.
- `meta/todos/todo.<slug>.md` × 5: four migrated beads
  (`todo.webui-hud-overlap.md`, `todo.crates-io-publish.md`,
  `todo.windows-support.md`, `todo.homebrew-tap.md`) plus one roadmap todo
  surviving the archive of `meta/changes/webui-design-quality/`
  (`todo.webui-design-quality.md`).
- `meta/todos/todo.status-active-changes-bug.md`: tracks the pre-existing
  `cairn status` `active_changes` bug found during this change, out of
  scope for it.
- `meta/research/task-front-door.md`, `meta/decisions/dec.native-todos-first.md`.

MODIFIED:
- `cairn.blueprint`: added `todos "./meta/todos"` pointer.
- `src/cli/render/remediate.rs`: `render_next` split into
  `render_next_clean`/`render_next_dirty` (line-count gate), both prefer
  the top open native todo; `render_brief` resolves a `BriefSource` enum
  (`Todo`/`Bead`) instead of a bead-only `BriefData`, native todo first
  when no explicit id argument is given (todos have no id to target
  directly, so an explicit id always resolves against the backlog).
  `open_native_todos`/`decision_summary` widened to `pub(super)` for reuse.
- `src/cli/render/project.rs`: `render_status`'s `next_recommended` prefers
  the top open native todo, same priority as `cairn next`.
- `AGENTS.md`: "Task tracking" section replaced; both `bd setup`-generated
  Beads blocks deleted.
- `src/cli/agent_guide.md`, `.claude/skills/cairn-dev/SKILL.md`,
  `docs/commands.md`: `cairn todo new` added to the artefact-creation
  surfaces; `cairn backlog` moved under "Optional integrations".
- `docs/conventions.md` §10: corrected the stale "node-partitioned"
  todos-layout claim to match the shipped non-recursive, flat loader.
- `meta/decisions/native-task-state-and-agent-guidance.md`: `revisited`
  date added, ruling 2 annotated as implemented by this decision.
- `.beads/issues.jsonl`: four issues closed with a migration note.

REMOVED:
- `meta/changes/webui-design-quality/` (archived, not deleted: 0/20 tasks
  checked, direction already ratified in
  `dec.webui-design-quality-direction`; its roadmap survives as
  `todo.webui-design-quality.md` per Decision 8 of the finish-cairn plan).

RENAMED:
- none.
