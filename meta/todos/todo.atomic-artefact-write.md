---
node: cairn.kernel.cli
status: open
created: 2026-07-14
related: [dec.todo-write-surface, todo.unified-todo-write-surface]
---

# Route artefact status writes through a cross-platform-safe atomic replace

## Problem

`cairn todo set <slug> <status>` (landed in PR #316) rewrites the todo
frontmatter `status` field with a plain `fs::write(&path, &updated)`. A
CodeRabbit post-merge review on PR #316 flagged that this non-atomic write
leaves a (small) window where a crash mid-write produces a truncated todo
file. The natural fix is to write to a temporary file in the same directory
and rename it into place.

## Blocker / constraint

No cross-platform-safe atomic-replace helper currently exists. The only
candidate, `crate::persist::atomic_write` (`src/persist.rs`), uses the same
`fs::write(tmp)` -> `fs::rename(tmp, path)` pattern. Under the project's
stated Windows semantics, `std::fs::rename` will not overwrite an already-
existing file, so reusing it for `run_todo_set` (which rewrites an existing
artefact) would break the `windows-check` gate. `cairn`'s `files-are-truth`
convention already keeps plain `fs::write` everywhere (e.g.
`write_new_artefact` in `src/cli/commands/decision.rs`), so the existing
behaviour is internally consistent.

## Proposed direction

- Add a genuinely cross-platform-safe atomic same-filesystem replace helper
  (temp file + `fs::rename` on Unix; a Windows-atomic replace such as
  `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING` on Windows), centralised in
  `src/persist.rs`.
- Route `run_todo_set`'s final write through it once available.
- Migrate existing `persist::atomic_write` call sites too if the helper
  supersedes it.

## Acceptance

- A cross-platform-safe atomic-replace helper exists and passes both Unix and
  `windows-check` CI.
- `cairn todo set` uses it; a test asserts the target file is replaced with no
  leftover temp file, preserving permissions and content on the crash-free path.
- No plain `fs::write` remains on the status-update path.
