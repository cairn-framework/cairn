---
node: cairn.root
status: open
created: 2026-07-12
---

# Cairn Dev Docs Sync

gh:#243

Bundled skills and finding-code reference have drifted badly from the current
command surface; drift worsened after simplify-architecture (#223-#231).

## Evidence (verified on main, 2026-07-12)
- `.claude/skills/cairn-dev/SKILL.md` command table omits `frontier`, `next`,
  `brief`, `bundle`, `health`, `workspace`, `backlog`, `draft`, `gap`,
  `docstring`, `remediate`, `rename`, `import-openspec`, `watch` among others
  (`cairn --help` lists 44 commands).
- `.claude/skills/cairn-dev/references/finding-codes.md` lacks CT001, CC002,
  and CAIRN_SOURCE_UNVERIFIED.
- `.claude/skills/cairn-dev/SKILL.md:248-253` documents the retired
  `{command, status, data}` JSON envelope; the ratified contract is a
  schema-versioned data payload (dec.query-json-schema-version). This is the
  documentation residue of gh:#240 (wont-fix by design).

## Task
Regenerate the cairn-dev command table from `cairn --help`, complete
finding-codes.md (CT001, CAIRN_SOURCE_UNVERIFIED; CC002 definition-or-removal
is owned by todo.cairn-apply-parallel-mode), and document the schema_version
JSON contract and exit-code semantics. Scope: the cairn-dev skill and its
references only; cairn-apply/cairn-propose wording belongs to
todo.cairn-apply-parallel-mode (gh:#245). Urgent: this drift misleads every
agent session.
