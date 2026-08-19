# Changelog

## v0.10.0

### Brownfield onboarding and decision extraction

- `cairn onboard decisions` indexes decision evidence from an existing codebase; the hybrid onboard-index plus `cairn-dev` reference mechanism was ruled by adversarial panel (`dec.brownfield-extraction-mechanism`, #670, #671, #681). `cairn init --from-code` names an explicit review step for the scaffolded blueprint after discovery (#609), and the flow is exercised end to end against an external repository (`res.autodocs-arm-a-brownfield-run`, #677, #678).
- Discovery anchors candidate nodes on package manifests (`package.json`, `pyproject.toml`, `Cargo.toml`, `go.mod`) instead of file placement and depth: the depth budget restarts at each package root, the innermost root wins, and one package maps to one node (`dec.brownfield-package-root-discovery`, proposed, #669). Nested packages scan clean without redundant `node_modules` ignores (#610, #682), and discovery-observed dependency cycles are classified advisory on measured evidence rather than failing the scan (`dec.brownfield-discovery-cycle-severity`, #506, #618, #642).
- An absent candidate blueprint at hook time is classified and reported rather than refused (#668).

### Decision ratification and the maintainer queue

- Decisions carry a ratification tier: a `local` ruling is machine-acceptable on convergent agent-cross-model review receipts bound to the decision's recomputed subject manifest, while a `binding` ruling, or one touching the binding surface, waits for the maintainer (`dec.decision-ratification-tiers`, `dec.reviewer-panel-ratification`, #544). An accepted local decision without two convergent receipts raises `CAIRN_DECISION_CONVERGENCE_UNMET`, and a receipt whose `subject_hash` matches no local decision's manifest is flagged.
- `cairn pending` lists proposed decisions awaiting ratification oldest first, or renders one decision's full briefing by id (`todo.maintainer-pending-queue`, #536, #574).

### Typed todo relationships

- Todos carry a typed relationship schema (`dec.todo-relationship-model`, #570), projected to GitHub as full issue bodies and relationship links (#614, #619). `cairn roadmap` derives a view over the relationship edges (#571), and decisions surface reverse provenance edges, `refined_by` and `superseded_by` (`dec.reverse-provenance-wire`, #576).

### Loop selection: deferral, strict-green, and parked findings

- Findings on the lint/scan JSON wire carry `deferred_by`: the id of the accepted decision deferring the finding, or `null`. A `Deferred-by` cell naming a decision that is not accepted raises `CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID` and publishes no deferral, so a published deferral always names an accepted decision (`dec.loop-selection-deferred-findings`, #531).
- The lint/scan `data` payload publishes `strict_green`: `true` exactly when `--strict` would exit zero over the emitted finding set. One shared predicate feeds the field and both strict exit paths, so the wire verdict and the gate cannot disagree; previously the shared-JSON path ignored `--strict` and exited 0 on warnings (`dec.loop-selection-strict-green-fold`, `todo.lint-selection-folding` item 2, #532).
- Todos gain an optional `defers:` list that parks a matching live Info finding while the todo is `blocked`: `cairn lint` and `cairn scan` still print it in full, annotated `(parked by todo.<slug>)`, and the wire publishes per-finding `parked_by`. Parking applies to Info findings alone and only through the typed reference; a reference aimed at an Error or Warning raises `CAIRN_TODO_DEFERS_BLOCKING` (CA042), one matching no finding raises `CAIRN_TODO_DEFERS_UNMATCHED` (CA041), and a malformed entry raises `CAIRN_TODO_DEFERS_INVALID` (CA043) (`todo.lint-selection-folding` item 1a, `dec.parked-deferral-composition`, #533).
- Loop-mode selection skips a finding with a published `deferred_by`, folds Info findings while the wire publishes `strict_green: true`, and skips a parked Info finding on its published `parked_by`; an Error or Warning with no published deferral stays selectable whatever any artefact says.

### Authorability evaluation

- A new `cairn-authoreval` binary scores how authorable a blueprint is, with a published baseline from one corpus run (`dec.authoreval-instrument-placement`, `todo.blueprint-authorability-eval`, #654, #663). It scores an unparseable blueprint instead of aborting the run (#661), and the gate grades the running binary rather than a `cairn` found on PATH (#658).

### Over-harness console (web UI)

- The web explorer gains a read-only over-harness console with three lanes (`dec.control-plane-programme`, `dec.orchestration-placement`, #572). Node state carries a grammar that survives greyscale, stillness, and a screen reader (#692); the console renders the evidence the browser already downloaded (#693); and the dimmed-node contrast audit is honest (#691). Web UI write authority is scoped by `dec.webui-write-authority`.

### Sources, artefacts, and scanner gates

- Sources gain a `verification: tracked` mode for live in-repo material (`dec.source-tracked-verification`, #537), and a source whose file cites the source itself warns (`dec.source-file-never-self`, #539). The artefact filename convention is enforced as CA038 (`dec.artefact-layout-authority`, #490). Accepted-decision accumulation on a node is flagged (#517), and a configured tag registry is validated (#626).

### Contracts

- Contract asserted-numeral drift is gated (`todo.contract-asserted-numeral-drift`, #705). Contract node-shape drift against the blueprint is enforced with non-generative baseline management (#515, #516), replacing the deferred stance (`dec.contract-node-shape-drift-deferred`, deprecated).

### Change lifecycle read surface

- `cairn change show` and `cairn change list` report task progress: JSON carries `progress: {completed, total, remaining}` parsed from the change's `tasks.md` checkboxes, and `change show` prints `Tasks: n/m complete`, sharing the checkbox parser with `CAIRN_CHANGE_TASKS_COMPLETE` (#508).
- `cairn change accept --dry-run` previews the acceptance gate: it resolves the battery, lists every step as `planned` with the command it would run, and reports `gate_outcome: preview`. Previously `--dry-run` was parsed as the change id (#508, gh:#241).

### Landing page and CLI

- The public landing page is rebuilt on a dedicated marketing design lane (`dec.marketing-visual-world`, #500, #507). `cairn init` scaffolds ignore suggestions (#615), and status and todo paths render relative to the project (#616, #620).

### Coordination and hooks

- Fact-store writes and reads are hardened (`dec.coord-fact-write-once`, #643). Auto-PR merge pins to the gated head (#629), blocked hook output points to remediation (#624), and configured decisions pointers are honored (#622).

### Wire format

- Query and web UI `/api/*` JSON payloads move from `schema_version` 3 to 12: change progress (#508), `deferred_by` (#531), `strict_green` (#532), `parked_by` (#533), the todo relationship schema (#570), the pending queue (#574), reverse provenance edges (#576), and normalized artefact paths (#617).

## v0.9.0

### Agent pack

- Cairn now installs and maintains its own agent guidance as an owned pack: `cairn pack install`, `update`, `status`, and `uninstall` (#466), plus `cairn pack resolve` and `campaign` (#468). An ownership manifest records a SHA-256 for each owned pack asset, so install, update, status, and uninstall report a file you edited and leave it alone, and only a file still matching what the packager wrote is ever refreshed or retired.
- Two harness adapters ship: `claude` installs under `.claude/`, and `omp` installs under `.omp/`. The OMP adapter carries a retained live-host validation record (`res.pack-omp-adapter-validation`, OMP 17.1.3); the Claude adapter is the tree this repository installs and dogfoods, with no separate capture retained. A first install with no selector detects the host, and afterwards every verb binds to the harness the manifest records, refusing a `--harness` that disagrees with it. The exception is `cairn pack campaign end`, which releases a campaign without consulting the manifest so an unreadable one cannot strand a project (#471, #483).
- `cairn init` bootstraps through that same lifecycle instead of writing skill files itself, and `--wire` now works on the brownfield path (#467).
- `cairn pack campaign` pins the resolved entry and its declared closure as immutable bytes, so a pack edit landing mid-session cannot reach a running campaign. A changed procedure halts before work rather than silently taking effect (#468).

### Agent guidance

- `cairn-dev` is now a compact router that loads at most one just-in-time reference for the task at hand, rather than a single large always-loaded document. When no route fits it stays put and uses the query surface directly. The loop procedure is split into required skills with declared typed exits, and fails closed when one is missing (#460).
- Plan reconciliation is a required step inside the landing commit, so the next session reads a corrected plan from main with no external memory (#461).
- `docs/spec.md` becomes fallback narrative rather than the read surface: reads are graph-first (#469).
- The shipped apply, propose, and archive skills use host-language gates and claim-matched proof rather than assuming a Rust project (#459).

### Fixes

- The pack lifecycle, the campaign lock, and `cairn init` now reject, at validation time, any path that leaves the project root and any path reached through a symlink. The lifecycle and campaign reads additionally refuse a file type whose read can block; init's own scaffolding inspects its destinations by metadata without reading them. Previously `pack uninstall` could delete a file outside the project, `pack install` and `campaign end` could write or remove state outside it, and `init` could scaffold outside it through a symlinked `.cairn`. Of these, only the `init` scaffold escape reached a released binary; the rest are in code that never shipped. A bounded parent-component race remains between the check and the act, which is documented rather than claimed away (#484).
- `cairn init --wire` now refuses a non-regular agent instructions file instead of reading it. Previously a FIFO at that path could block the command forever. This one did reach the released v0.8.0 binary (#487).
- An owned pack file that exists but cannot be read is now reported as modified and left alone, rather than classified as missing and overwritten by `pack install` or `pack update`, or silently dropped from the `uninstall` report (#487).
- Brownfield discovery sanitises node ids it derives from directory names, and declares the contracts it writes (#476, #481).
- Artefact status writes replace their file atomically (#480).
- An unsupported long option on a command that declares its own flag set is now rejected with a usage error instead of being silently ignored, and the help metadata behind that check is guarded against drift. Options treated as global are still accepted everywhere (#479).
- Remediation copy is centralised, and packaged includes are anchored so no embedded asset can be dropped from a release (#472, #473).
- A deferred spec-rule decision that no longer resolves now raises a warning rather than silently suppressing its finding (#474).
- The `CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID` message now renders its copy body with the rule, spec anchor, and rejected decision substituted; the lookup previously missed the `.body` segment and rendered the literal copy key.
- The web explorer reloads changed or newly added artefacts without a restart (#475).

### Known boundary

- `cairn scan` reports one deferred Info finding on this repository. It is retained deliberately by `dec.revisit-trigger-correlator-deferred` as the honest tracker for a designed-but-unbuilt spec rule, and is not a defect.

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
