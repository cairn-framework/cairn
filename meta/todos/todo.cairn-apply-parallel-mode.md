---
node: cairn.root
status: open
created: 2026-07-12
---

# Cairn Apply Parallel Mode

gh:#245

The cairn-apply skill assumes a single sequential agent and Rust gates.

## Evidence (verified on main, 2026-07-12)
- `.claude/skills/cairn-apply/SKILL.md:46-49` lists only cargo gates.
- `SKILL.md:58` references gate `CC002`, which is defined nowhere in the skill
  references.
- Neither cairn-apply nor cairn-propose mentions parallel implementation,
  frontier-node assignment, or blueprint `path` claims as write-ownership
  boundaries.

## Field report (owner, 2026-07-13)
- A downstream TypeScript repo received the cargo-flavoured guidance in
  practice and had to override it via its own AGENTS.md; drift confirmed in
  the wild, not just on inspection.

## Task
Rewrite the skill for language-aware gate derivation (from `cairn.config.yaml`
or project language) and add a documented parallel mode: split a change into
frontier-node-sized tasks with path-claim ownership. Define or remove the CC002
reference. Feeds #102. Coordinate with todo.accept-language-aware-gates (gh:#234)
and todo.cairn-dev-docs-sync (gh:#243).
