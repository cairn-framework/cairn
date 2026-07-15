---
id: src.agentskills-spec
file: https://agentskills.io
verification: external
type: spec
date: 2026-07-13
---

# agentskills.io SKILL.md format specification

The SKILL.md interchange format that cairn's shipped skill pack conforms to,
surveyed 2026-07-13 for the packaging design.

Load-bearing finding: there is no standard per-skill `version:` frontmatter
field; `metadata` is free-form and skills are addressed by name and directory.
Version therefore belongs at pack level, not per skill. This ruled out
per-skill version stamping in cairn's manifest design and pushed versioning to
the bundle manifest (`installed_bundle` plus per-file content hashes).
