---
node: cairn.kernel.cli
status: done
created: 2026-07-10
---

# Change Apply Alias

`cairn change archive <id>` is the verb that applies a proposal, but
"archive" reads as shelving, not activating. First-run users must run it to
get their first map on an existing project, so the verb actively fights
comprehension at the most delicate moment of the funnel.

Ratified 2026-07-10 (owner sign-off on the messaging-workshop terminology
recommendations): add `cairn change apply <id>` as an alias for the archive
operation. Keep `archive` working; docs and agent-facing guides switch to
`apply` as the primary verb once the alias ships.
