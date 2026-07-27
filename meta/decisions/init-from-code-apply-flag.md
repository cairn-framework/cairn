---
id: dec.init-from-code-apply-flag
nodes:
  - cairn.brownfield
  - cairn.kernel.cli
status: accepted
date: 2026-07-12
---
# One-step first map: `cairn init --from-code --apply`

## Decision

`cairn init --from-code` gains an opt-in `--apply` flag that applies the
brownfield-init proposal immediately after discovery, so a fresh user gets a
populated `cairn.blueprint` (and a non-empty first map) in one command. The
flag delegates to the same code path as `cairn change apply`
(`run_archive_command`), so it shares the active-change conflict gate,
validation, rollback, and archive-log behaviour.

todo.brownfield-one-step-first-map listed three options for closing the
empty-first-map gap:

1. **Auto-apply when the user has not customised the draft.** Rejected:
   contradicts the documented review-before-apply contract
   (docs/brownfield.md, dec.brownfield-init-round-trip) and makes behaviour
   depend on a fragile "was it customised" heuristic.
2. **Scan reads the active brownfield delta.** Rejected: changes scan
   semantics repo-wide; active proposals would leak into the reconciled map
   for every project, not just first-run.
3. **Explicit `--apply` flag.** Chosen: conservative, preserves the reviewable
   default, uses the ratified `apply` verb (todo.change-apply-alias), and
   makes the quickstart's one-step promise literally true.

## Scope

CLI dispatch in `src/cli/mod.rs`; first-run copy in README, docs/quickstart.md,
docs/brownfield.md, docs/agent-setup.md, and the `cairn init` next-steps hint
in docs/design-system/copy.toml. In JSON mode the flag emits the archive
command's envelope unchanged; the prose prefix is text-mode only.
