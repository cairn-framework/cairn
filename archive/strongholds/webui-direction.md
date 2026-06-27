# Stronghold: webui direction (read-mostly graph explorer vs write-surface)

## 1. Current capability inventory

The cairn webui at `src/ui/` and `src/ui_assets/` is a read-only graph explorer. It binds `127.0.0.1:3000`, serves a single-page Preact application, and exposes the following HTTP surface:

| Method | Endpoint | Purpose |
|---|---|---|
| GET | `/api/meta` | Schema version and runtime metadata |
| GET | `/api/status` | Node count, edge count, finding count, interface hash |
| GET | `/api/graph` | Full graph (nodes + edges) for canvas layout |
| GET | `/api/lint` | Reconciler findings with severity and node attribution |
| GET | `/api/node/:id` | Node record (kind, name, description, paths, state, tags) |
| GET | `/api/node/:id/contract` | Contracts attached to the node |
| GET | `/api/node/:id/decisions` | Decision artefacts with frontmatter |
| GET | `/api/node/:id/todos` | Todo artefacts |
| GET | `/api/node/:id/research` | Research artefacts |
| GET | `/api/node/:id/sources` | Source artefacts |
| GET | `/api/node/:id/rationale` | Rationale text for the node |
| GET | `/api/depends/:id` | Outbound dependency edges |
| GET | `/api/dependents/:id` | Inbound dependency edges |
| GET | `/api/blueprint` | Raw blueprint source for read-only view |

There are no POST, PUT, PATCH, or DELETE endpoints. The server does not mutate project state, auth tokens, or the filesystem.

The frontend renders:

- An interactive SVG canvas with pan, zoom, fit, and minimap.
- System, Container, Module, Actor, and stray-module divider nodes.
- Ownership and dependency edges with description labels.
- Reconciliation state badges (`synced`, `ghost`, `orphaned`) on modules.
- A chain-balance widget per module (provenance vs authority strength bars).
- A right-hand inspector panel showing: blueprint snippet, prose-nudge banner, paths, chain-balance numerics, artefact counts, and collapsible sections for Contracts, Decisions, Todos, Research, Sources, Depends on, and Dependents.
- A decision-detail sub-panel that renders condition and rationale sections.
- An empty-state inspector showing map-level stats and recent findings.
- A findings rollup panel with scope toggles (whole map / selected node) and category chips.
- A command palette (⌘K) for searching nodes by id, name, or kind.
- A changes drawer that surfaces all reconciler findings as cards.
- A blueprint source modal with syntax highlighting.

Every user action is either navigation (select node, open panel) or read-only inspection. The only data-modifying affordance in the entire UI is `navigator.clipboard.writeText`, used by the Copy button on prose-nudge CTAs.

## 2. The decision

**Question:** Should the cairn webui remain a read-mostly graph explorer, or should it evolve into a write-surface where users author, edit, and mutate project state?

**Why this matters.** Four sub-components from the getcairn.dev adoption analysis are gated on this decision. Resolving it prevents designing those features twice (once for a read-only UI, once for a writable one).

**Criteria.** The right call depends on how users actually open the webui today and how they will use it tomorrow. Two user intents are possible:

1. **Understanding the graph.** The user wants to see structure, trace dependencies, read decisions, and inspect reconciliation findings. The webui is a map. Work happens elsewhere (CLI, editor, git).
2. **Doing work in the browser.** The user wants to author decisions, edit contracts, fix findings, propose changes, and accept artefacts without leaving the tab.

The honest answer today is that we do not have enough usage data to decide with confidence. The webui has existed since Phase 2.5 but has no telemetry, no user interviews, and no onboarding funnel that would tell us which intent dominates.

## 3. Mapping the four gated sub-components

The four deferred items are C2.d, C3.e, C13.f, and C12.f. Below is what each becomes under each branch.

### C2.d: "Fix with AI" button on the prose-nudge banner

- **Read-mostly branch:** The banner renders a copy-pasteable CLI command (for example, `cairn refine <node-id>`). A Copy button lets the user paste it into their terminal. The button does not attempt an in-browser mutation.
- **Write-surface branch:** The button triggers an in-browser action. This requires: a POST endpoint to enqueue a suggestion, a job-status polling mechanism, a progress indicator, and an error-recovery surface. The button must handle auth (if the suggest engine requires an API key), rate limiting, and partial failure.

### C3.e: Per-row "Fix" button in the findings panel

- **Read-mostly branch:** Each finding row renders a CLI snippet that addresses the finding class (for example, `cairn lint --fix` or `cairn onboard --module <id>`). The row is actionable but hands off to the CLI.
- **Write-surface branch:** Each row gets an in-UI Fix button that mutates the project directly. This requires: write endpoints per finding class, a transaction or rollback primitive (since a fix may touch both blueprint and source files), and a confirmation step because findings can have false positives.

### C13.f: In-webui CTA actions (empty-state primary buttons)

- **Read-mostly branch:** Empty states name the next concrete action as a CLI command. For example, "Your repo has no blueprint yet. Run `cairn init` to draft one." The CTA is a copy-to-clipboard button, not a form submission.
- **Write-surface branch:** Empty states surface in-UI forms: a "Create System" button opens a modal that POSTs a new blueprint fragment, a "Attach Contract" button opens a contract template picker, and so on. This requires form validation, schema-aware templates, and write endpoints for every artefact type.

### C12.f: Settings-pane export UI

- **Read-mostly branch:** Export is CLI-first. `cairn export --format json` produces the artefact. The webui optionally surfaces a "How to export" help panel with the command syntax. No file is generated in the browser.
- **Write-surface branch:** The webui gets a settings pane with format pickers (JSON, Markdown, CSV), scope toggles (phase / spec / all), and a download button that triggers server-side generation and streams the file to the browser. This requires: export endpoints, streaming response handling, and a destination policy (downloads vs provenance-chain storage).

## 4. Recommendation and rationale

**Recommendation: keep the webui read-mostly for the next 12 months.** Ship the CLI-handoff fallbacks for all four gated sub-components now. Do not build write endpoints, form surfaces, or in-browser mutation flows until at least one of the following preconditions is met:

1. A concrete user workflow surfaces that cannot be served by CLI handoff. For example: a designer who does not use the terminal but needs to read decisions; a product manager who wants to approve changes in a browser tab rather than in a PR review.
2. The webui becomes the primary onboarding surface. If first-time users meet cairn through the browser before they install the CLI, write affordances become necessary for activation.
3. A remote or hosted cairn instance exists. If cairn runs on a server where users do not have shell access, the webui must become the write surface by default.

**Rationale.**

- **Honesty.** The current webui has zero write infrastructure. Adding it is not a feature; it is a platform expansion (auth, sessions, CSRF, input validation, error recovery, optimistic UI, rollback). That expansion is only justified when the user evidence demands it.
- **Focus.** The CLI is cairn's native authoring surface. `cairn init`, `cairn refine`, `cairn onboard`, `cairn lint`, and `cairn accept` are all designed for terminal workflows. Diluting that focus by rebuilding a parallel browser surface fragments maintenance and creates drift between CLI and webui behaviour.
- **Safety.** Write surfaces increase the attack surface and the corruption surface. A read-only webui cannot accidentally mutate a blueprint, delete a decision, or enqueue a broken suggestion. Keeping it read-only aligns with the kernel's enforcement value: deterministic, typed, two-chain primitives that live in version-controlled files, not in a browser's transient state.
- **Speed to ship.** C2.d, C3.e, C13.f, and C12.f can all ship as copy-pasteable CLI commands in the next UX polish pass. This unblocks Bundle B (UX foundation) and Bundle E (export starter) without waiting for a write-surface architecture decision.

**What to watch.** Track two signals over the next 12 months:

1. How often does the webui get opened vs how often does the CLI get invoked? If the ratio shifts toward browser-first usage, revisit this decision.
2. Do user requests for in-browser editing outnumber requests for richer read-only visualizations (for example, re-center on any node, verb-labelled edges, prerequisite/enables widget)? If the former dominates, write-surface becomes the priority.

Until then, the webui's job is to help users understand the graph. Work stays in the terminal.

## 5. What each deferred sub-component becomes once the direction is fixed

If the decision ever flips to write-surface, the four gated items become:

- **C2.d** → a POST `/api/suggest` endpoint + job polling UI + progress indicator.
- **C3.e** → a POST `/api/fix` endpoint per finding class + confirmation modal + rollback primitive.
- **C13.f** → form components for each artefact type + POST endpoints + template picker + validation layer.
- **C12.f** → a settings pane with export configuration + streaming download endpoint + provenance-chain storage policy.

If the decision stays read-mostly, the four items become:

- **C2.d** → a Copy button on a CLI command string (already supported by the existing `CopyButton` component).
- **C3.e** → a Copy button on a per-finding CLI snippet.
- **C13.f** → a Copy button on an empty-state CLI command string.
- **C12.f** → a help panel documenting `cairn export` flags and examples.

Both paths are fully specified. The only work blocked by this stronghold is the write-surface expansion itself.

---

*Status: decision recorded. Revisit when user evidence or product topology changes.*
