# Changelog

## v0.2.0

This release contains breaking CLI changes: old subcommand names were folded into flags and grouped verbs.

### CLI surface simplification (breaking)

- `cairn symbols`, `cairn check`, `cairn depends`, `cairn dependents` are folded into `cairn get`, `cairn lint`, and `cairn deps` flags (#226).
- The change lifecycle (`new`, `list`, `show`, `archive`, ...) is collapsed under a single `cairn change` verb (#223).
- The `draft_*` commands are collapsed into one `cairn draft` subcommand (#204).
- The command surface is now derived from the `query_api` registry (#228), and human-readable renderers consume the same canonical `query_api` JSON as `--json` output (#230), so the two output modes can no longer drift.

### Web UI

- Severity/drift visual encoding on the graph (#212).
- "Trace the truth" hinge: a decision's missing provenance ("no sources recorded") is now a visibly distinct gap in the inspector instead of the quietest text on the panel (#213).
- Webui endpoints started migrating onto the `query_api` spine; first five endpoints flipped (#229).

### Fixes

- Scanner now infers a directory's language and suppresses empty-target hashes (#217).
- `cairn status` (#200) and the neighbourhood view (#205) surface real active changes instead of a hardcoded empty list.
- `--changes-dir` is honoured on all discover-based surfaces (#206).
- Unknown top-level keys in `cairn.config.yaml` now produce a warning instead of being silently ignored (#221).

### Internals

- Reconciler is now a single generic engine parameterised by `LanguageSpec` with registry dispatch, replacing per-language reconcilers (#227).
- Artefact frontmatter loaders collapsed into an `ArtefactKind` table (#224).
- LSP shares the watch loop and runs lint via the `query_api` spine (#231).
- Removed the unused SSE spike (#201) and deduplicated CLI/format helpers against `query_api::serialise` (#202, #203).

### Project

- Ko-fi support: `FUNDING.yml`, README badge, and a native support button on the landing page (#207, #208, #210, #211).


## v0.1.4

- Fixed crates.io publish (second fix): `cairn-framework`'s published package was 42.7MiB compressed (the default package ships the whole repo: demo GIFs/MP4s, a PDF, .beads bookkeeping, archived research screenshots, test fixtures), over crates.io's 10MB upload cap. Cargo.toml now carries an `include` allowlist scoped to what `src/**` needs to compile plus its `include_str!`-embedded runtime assets (webui, agent guide, the bundled skills). Packaged tree is now 491.9KiB compressed and verified to compile standalone. `cairn-macros` (already at 0.1.0) is unaffected. v0.1.3 never published `cairn-framework` (4 attempts: 3 transient crates.io 503s, then the 413); both crates land on crates.io starting with this release.
- Hardened `cargo-publish.yml` with retry/backoff around `cargo publish` itself (up to 3 attempts, re-checking already_published at the top of each iteration so a publish that landed server-side despite a client-side error is detected and skipped, not double-attempted).
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
