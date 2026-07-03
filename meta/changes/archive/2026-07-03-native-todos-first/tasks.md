# Tasks: native-todos-first

- [x] Write `res.task-front-door.md` and `dec.native-todos-first.md`
- [x] Add `todos "./meta/todos"` pointer to `cairn.blueprint`; verify `load_todos` picks it up
- [x] Implement `cairn todo new <slug> --node <id>` (`src/cli/commands/todo.rs`) with unit tests
- [x] `cairn next`/`cairn brief` prefer the top open native todo over the beads backlog
- [x] `cairn status`'s `next_recommended` agrees with `cairn next`
- [x] Migrate the 4 remaining open beads to todo artefacts; close them in bd
- [x] File the pre-existing `cairn status` `active_changes` bug as a native todo (out of scope to fix here)
- [x] Add human renders for `cairn changes` and `cairn show`; drop them from the JSON-only guard
- [x] Mark `cairn docstring`/`cairn rename` JSON-only in `docs/commands.md`
- [x] Replace `AGENTS.md` Task tracking section; delete both Beads blocks
- [x] Add `cairn todo new` to `src/cli/agent_guide.md` and `.claude/skills/cairn-dev/SKILL.md`
- [x] Update `dec.native-task-state-and-agent-guidance` frontmatter (`revisited`, ruling 2 note)
- [x] Archive `meta/changes/webui-design-quality/`; its roadmap survives as `todo.webui-design-quality.md`
- [x] `make check`, `cairn hook all`, `sh scripts/check-file-sizes.sh` all green
