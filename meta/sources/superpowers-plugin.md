---
id: src.superpowers-plugin
file: https://github.com/obra/superpowers
verification: external
type: repo
date: 2026-07-13
---

# Superpowers: Claude Code plugin packaging precedent

Canonical Claude Code skills plugin (core skills library, MIT), surveyed
2026-07-13 for how the plugin ecosystem packages and versions skills.

Load-bearing findings:

- Every Claude Code plugin carries a `.claude-plugin/plugin.json` manifest
  (name, version, description, author, homepage, repository, license,
  keywords) with skills in a `skills/` directory; versioning is pack-level,
  never per-skill.
- Vendor directories stay separate from user directories across the ecosystem
  (plugin cache vs `.claude/skills/`); user-authored skills are never
  overwritten by plugin installs.
- Claude Code discovers user skills via the flat
  `.claude/skills/<skill-name>/SKILL.md` layout. Nested vendor subdirectories
  (`.claude/skills/vendor/<name>/`) are a plugin-layer feature and unverified
  for base-directory discovery.
