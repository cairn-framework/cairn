---
node: cairn.kernel.changes
status: done
created: 2026-07-07
---

# Changes Dir Flag Ignored

`changes::discover` hardcodes `meta/changes` (src/changes/mod.rs), so
`cairn --changes-dir <dir>` is ignored by every surface that lists active
changes via `discover`: `status`, `changes`, `show`, and now
`neighbourhood --include-changes`, even though hook/health/remediate/archive
respect the flag. Thread `changes_dir` into `discover` (or its callers) so
all surfaces agree. Review finding from 2026-07-07, split out of
todo.include-changes-hardcoded-empty when that fix landed node-scoped
active changes on the neighbourhood surfaces.
