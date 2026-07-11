# Proposal: change-apply-alias

## Motivation

`cairn change archive <id>` is the verb that applies a proposal, but "archive"
reads as shelving, not activating. First-run users encounter it at the most
delicate moment of the funnel. The owner ratified adding `apply` as the primary
verb in `todo.change-apply-alias` (2026-07-10).

## Scope

- Add `cairn change apply <id>` as an alias for the archive operation.
- Keep `archive` working as a legacy alias.
- Switch all user-facing docs and agent-facing guides to `apply` as the primary
  verb.
- Add a behavior test for both `apply` and legacy `archive`.

## Out of scope

- Removing the `archive` verb entirely (backwards compatibility preserved).
- Changing the archive directory path (`meta/changes/archive/`).
- Renaming the internal `run_archive_command` function or the `ArchiveReport`
  type.
