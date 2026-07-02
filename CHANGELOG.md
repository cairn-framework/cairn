# Changelog

## v0.1.0

- Added `cairn feedback "<message>"`: records friction in `.cairn/feedback.md` and prints a prefilled upstream issue link, closing the dogfood loop from host projects (decision: `meta/decisions/feedback-loop.md`).
- `cairn init` now writes `.cairn/AGENTS.md` (agent guidance for the host project, including the feedback loop) and prints next steps; the starter blueprint calls out test directories.
- Clarified the `CAIRN_INTEGRITY_INVALID_ID` message with the allowed ID charset.
- Web UI: boot failures now show a visible error state with retry, boot and inspector fetches show loading states, and the command palette supports ArrowUp/ArrowDown/Enter keyboard navigation.
- Reworked README for external adopters; fixed the invalid example blueprint in `docs/quickstart.md` (wrong grammar and underscore IDs); repointed stale `George-RD` URLs to `cairn-framework`.
- Crash panic hook and a webui "Report an issue" surface (command palette + topbar), both opening a prefilled upstream issue link. Nothing is ever sent automatically.
- Prebuilt release binaries for macOS (arm64, x86_64) and Linux (x86_64, arm64) via a one-line shell installer; `cairn`, `cairn-mcp`, and `cairn-lsp` all ship in each release tarball.
- Removed the Graphite (`gt`) workflow integration (no longer used): deleted `docs/agent/graphite.md`, dropped the CLAUDE.md Graphite section, and replaced the `gt` commands in the dev-workflow and `cairn-loop` PR steps with plain git + `gh`. PRs now go through standard GitHub against the `main` trunk.

## v0.7

- Renamed the authored architecture file from `.dsl` to `.blueprint`, with `cairn.blueprint` as the canonical default.
- Renamed user-facing ontology terminology to map terminology across docs, CLI-facing prose, specs, and Rust API surfaces.
- Renamed generated scanner snapshots from `index.md` to `map.md`.

See `openspec/changes/phase-2.6-terminology-rename/` for the full change record.
