# Changelog

## v0.8.0

### Graph explorer relayout

- The map is now a dependency-layered architecture layout: entry surfaces on the left, foundations on the right, isolated modules in a band below, and the kernel container drawn as a labelled frame. Dependency edges are curved, directional, and arrowheaded; selecting a node highlights its in and out edges and calmly dims non-neighbours. Nothing is auto-selected on first visit (#428).
- Node cards drop the state text, path chips, and ownership badges in favour of the leaf id, a two-line description, and the existing state dot and keel. The whole 25-node dogfood graph fits the workspace without scrolling at 1440x900, and keyboard arrows navigate spatially (#428).

### Evidence rail

- The blueprint tab scopes to the selected node's declaration block with an expand-to-full toggle and a full-height pane, replacing a fixed 52px scroll trap that held the entire blueprint. The facts grid tightens, lineage renders one row per artefact in sentence case, and the rail collapses to a hint when nothing is selected (#428).

### Chrome and channels

- The shell fills wide viewports, the query rail becomes one search input plus labelled Type and State segmented groups, and the status bezel keeps a single severity-coded annunciator. The bottom channel rebuilds as compact severity-sorted rows with colour-coded badges, a collapse toggle, and readable findings, backlog, and changes items (#428).
- Errors gain a distinct ember accent (`--ci-ember`) so error and warning severities read as different colours at a glance, contrast-checked on every chassis surface (#429).
- A restrained motion layer animates cards, edges, tabs, and panel entries, entirely disabled under `prefers-reduced-motion` (#428).

### Site and docs

- GitHub Pages serves the landing page from the `docs/` root directly, dropping the redirect stub. The README gains fresh graph-explorer screenshots, a reliable release badge, and a plain-language copy pass alongside the landing page (#428).

## v0.7.0

### Webui trust and resilience

- Project-load failures now reach the map error banner with their structured diagnosis instead of the generic `/api/graph (500)` message. The terminal logs failures, and a failed mid-session reload serves the last successful cached scan rather than taking down every API route (#416).
- The webui report action and crash hook open the structured bug-report form with useful context. Freeform `cairn feedback` links remain blank issues by design (#418).
- The private webui wire moves to schema v4: map metadata carries the server-owned time of the successful scan currently being served, so the clean findings drawer can report honest freshness and offer a copyable `cairn scan` next step (#421).

### Calibrated Instrument direction

- Canvas state vocabulary implements `dec.webui-design-direction`: unmistakable selection, calm readable de-emphasis, offscreen neighbourhood framing, and shape-backed state keels (solid synced, hollow dashed planned, amber tilted orphaned). The legend explains the encodings, and the canvas chrome is split into sub-500-line modules (#419).
- Dead ends become guidance: clean findings show freshness plus a next command, and palette results expose accurate Enter hints with combobox/listbox semantics (#421).
- Project counts have one canonical home in the topbar; the inspector keeps only map-health and node-scoped detail. Phone-width report copy is shorter without losing its accessible name or 44px target (#422).
- The deterministic webui harness is back at `ux_defect_score=0`: touch layouts hide the pointer-sized minimap and give the coach-mark dismiss action a full tap target (#420).

## v0.6.0

### Live operational surface

- New `scripts/herdr-cairn-dashboard.py`: a dependency-free herdr pane process rendering cairn ground truth (findings by severity, backlog, next recommended step). Every snapshot re-derives `cairn lint --json` and `cairn status --json`, writes a verifiable sidecar with a refresh-marker handshake, and optionally renders an orchestrator-claims overlay under an explicit label. Documented in `docs/herdr-dashboard.md` (#407).
- Research artefact `res.herdr-plugin-feasibility` probes herdr's real extension surface (pane processes, report-metadata/report-agent, watch versus poll versus MCP-wrapper event push) and records the recommended increment path (#405).

### Wire contract hardening

- The query API's outer response envelope gains a machine-checkable schema (`schemas/envelope.schema.json`), and `StatusResponse`/`RemediateResponse` move from ad-hoc JSON literals to typed serde structs with committed schemas and validation tests; the registry's unschema'd allowlist shrinks accordingly (#409).

### Modularity enforcement

- `CAIRN_MODULE_OVERSIZED` is promoted from Info to Warning: unmarked node-claimed source files over 500 lines now fail `cairn scan --strict`. The repository's seven oversized test suites were baselined first (six file-specific allow markers, one Part A/Part B split) (#408).

### Webui

- Phone-width graph navigation: two-row 390px topbar, pointer-event pan, pinch zoom with finger anchoring, enlarged node hit targets, and narrow-width edge-label decluttering with traced-edge reveal (#410).
- Progressive disclosure: inspector artefact sections collapse by default (Decisions open when present) and zero-artefact nodes show one calm summary line; findings unify into the drawer as the single canonical surface; the chain banner becomes a dismissible first-session coach-mark; selecting a node frames its dependency neighbourhood; minimap dots gain labels and button semantics (#411).

## v0.5.0

### One queue vocabulary

- `cairn status --json`, `next --json`, and `remediate --json` all emit one shared work-item projection: {source: finding|todo|bead, title, node, command, rank}. Query and webui `schema_version` bump to 3 (#398).
- Wire shapes gain machine-checkable definitions under `schemas/` (map, finding, work-item), validated in tests against both freshly built snapshots and the committed map.json; every registry response label must resolve to a schema or a documented allowlist entry (#398).

### Modularity guardrails

- The 500-line file-size gate covers JavaScript and CSS sources wherever the blueprint claims them: claimed directories and files are discovered from cairn.blueprint path declarations (list forms included, malformed declarations fail closed) and walked live, with JS/CSS allow-marker protocols (#395, #396).
- New CAIRN_MODULE_OVERSIZED scan finding mirrors the shell gate inside cairn itself: node-claimed rs/js/css files over 500 lines surface in scan, lint, and remediate (split_module action), honouring the same markers (#399).
- The webui monolith is split: app.js (2013 lines) becomes a 305-line composition root plus nine feature modules, all under the gate, served as native ES modules from the single binary with byte-identical wire output (#401).

### Provenance hygiene

- The two comparative research analyses anchor into the decision chain via topic-scoped decisions (dec.locate-result-semantics, dec.finding-coverage-strategy), clearing the standing orphan-research findings (#394).

### Fixes

- File-size gate fixture roots are collision-free under parallel test runs (#397).
## v0.4.0

### Agent navigation

- New `cairn locate <symbol>` (CLI `--json` and `cairn_locate` MCP tool): repo-wide public-symbol reverse lookup over the reconciler's persisted SymbolRecords; collisions return every match with its owning node id, landing agents on the node's contract and decisions (#389).

### Orientation truth

- `cairn status` and `cairn next` now agree on `next_recommended`: both delegate to one shared selection (top remediation action, then top open native todo, then top ready bead, per dec.native-todos-first). Query and webui `schema_version` bump to 2 (#387).
- `cairn bundle` and `brief` surface the project's real verification gates (explicit `gates:` config, else the language battery accept runs) instead of stale static copy; accept resolves and executes gates from the `--file`-derived project root (#391).

### Ownership and scanning

- Claim-only assets targets: `cairn.config.yaml` targets accept `language: assets` for non-code directories; such targets claim files without a reconciler and without the unknown-language warning. `cairn.ui` now owns `src/ui_assets` (#390).

### Process automation

- `map.json` merge contention between concurrent PRs is resolved by a custom git merge driver that regenerates the snapshot on the merged tree; setup is one `make`-wired per-clone config, documented with its GitHub-UI limitation (dec.map-snapshot-merge-driver) (#388).

### Guards and hygiene

- New meta-test fails when a `CAIRN_*` finding code ships untested (documented allowlist burn-down) or unregistered; the error-codes registry is reconciled with every emitted code and descriptions rewritten from actual validator conditions (#392).
- The three command-reference guards retarget consolidated AGENTS.md (#385).
- Flaky `test_complete_session_writes_genesis` fixed: interview test helper used a fixed temp path shared across processes; per-test `TempDir` now isolates it (#386).
## v0.3.0

### CLI truth

- Human `cairn neighbourhood` now shows the same inbound and outbound edges as `--json`; the `--include-orphans` flag, whose only effect was hiding inbound edges, is removed (#354).
- Human `cairn health` info count now matches the JSON summary, and the health JSON key `summary.info` is renamed to `summary.total_info` to match its sibling keys (#353).

### Query ergonomics

- `cairn get` now carries accepted-decision pointers in both JSON and human output (#355).
- `cairn rationale` labels neighbour-sourced decisions as transitive, with a `via` array in JSON and a `(via <node>)` suffix in human output (#355).
- Node lookup accepts unambiguous dotted-suffix aliases (e.g. `scanner` for `cairn.kernel.scanner`); ambiguous suffixes list the candidates (#355).

### Todos and orientation

- Bare `cairn todos` lists todos project-wide; `cairn todos <container>` aggregates descendants via containment edges; the node argument resolves correctly after leading flags (#356).
- `cairn brief todo.<slug>` targets a native todo directly, fusing the same decision, contract, and gate context as the bead path (#357).

### Feedback

- `cairn feedback` gains structured `--area` and `--severity` flags carried into the log, issue body, and JSON output; generated issue titles truncate at word boundaries (#358).

### Workflow

- The cairn-vibe session workflow isolates director work in disposable worktrees and guards the main checkout from build and scan artefacts (#352).


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

## Legacy spec milestone v0.7 (pre-package release)

- Renamed the authored architecture file from `.dsl` to `.blueprint`, with `cairn.blueprint` as the canonical default.
- Renamed user-facing ontology terminology to map terminology across docs, CLI-facing prose, specs, and Rust API surfaces.
- Renamed generated scanner snapshots from `index.md` to `map.md`.

See `openspec/changes/phase-2.6-terminology-rename/` for the full change record.
