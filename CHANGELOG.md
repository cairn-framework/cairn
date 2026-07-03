# Changelog

## v0.1.3

- Fixed crates.io publish automation (added in v0.1.2, landed in this release): crates.io's API enforces a data-access policy that rejects requests with a generic User-Agent (a bare `curl/<version>` counts as generic); `cargo-publish.yml`'s existence-check and index-poll curl calls now identify themselves. v0.1.2's release run hit this as a 403 before either crate was published, so `cairn-macros` and `cairn-framework` go live on crates.io for the first time starting with this release, not v0.1.2 as originally stated.

## v0.1.2

- `cairn-macros` and `cairn-framework` are now published to crates.io on every tagged release: a custom cargo-dist publish-job (`.github/workflows/cargo-publish.yml`) publishes in dependency order and skips a crate/version already live, so `cairn-macros` staying unchanged across future releases will not hard-fail. This release is the first to carry it; both crates go live on crates.io starting now. `cargo install cairn-framework` works as an alternative to the shell/PowerShell/Homebrew installers.

## v0.1.1

- Native task front door: this repo's own development now tracks work in cairn's native Todo artefacts (`meta/todos/todo.<slug>.md`, `docs/spec.md` §8.2) instead of beads. New `cairn todo new <slug> --node <id>` command scaffolds a todo, exactly symmetric with `cairn decision new`. `cairn next`/`cairn brief` prefer open native todos, falling back to the beads backlog only when `.beads/issues.jsonl` exists and no native todo is open. Beads remain a supported, read-only per-node view (`cairn backlog`) for projects that want it.
- `cairn changes` and `cairn show` now render human-readable output instead of erroring "this command currently requires --json"; the other eight JSON-only commands are documented as such in `docs/commands.md`.
- `cairn health` and `cairn remediate` now have real `--help` descriptions (were empty).
- `docs/commands.md`: removed duplicate health/remediate rows, added the missing global flags (`--changes-dir`, `--depth`, `--scope`, `--version`).
- Webui: fixed the graph canvas collapsing under the fixed HUD row on short/narrow viewports, which pushed the SYSTEM node and zoom/legend docks into an overlapping cluster.
- Windows support: `x86_64-pc-windows-msvc` added to the release matrix (prebuilt binaries plus a PowerShell installer). `signal-hook` (SIGINT handling) is now cfg-gated to Unix; Windows gets a no-op stub since Ctrl-C's default OS behaviour already terminates the process.
- Renamed the crates.io package to `cairn-framework` (the names `cairn` and `cairn-cli` were already taken) and made it publish-ready (`cairn-macros` version-pinned, `[package.metadata.dist] formula` set, `--dry-run` verified); installed binaries keep the `cairn`/`cairn-mcp`/`cairn-lsp` names regardless. The v0.1.0-era shell installer URL (`cairn-installer.sh`) keeps resolving via an automated re-upload step. See `meta/todos/todo.crates-io-publish.md` for live-publish status.
- Homebrew: `brew install cairn-framework/tap/cairn` now works via a published tap (`cairn-framework/homebrew-tap`).
- Identity: `docs/spec.md` and `README.md` each gained one sentence naming cairn's internal mechanism (a declarative reconciliation controller); the public "map" framing and tagline are unchanged.

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
