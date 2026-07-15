---
id: res.architecture-modularity-audit
nodes: [cairn.root]
date: 2026-07-15
method: primary
---

# Architecture modularity audit

## Method

Primary measurement of the cairn repo on branch `architecture-modularity-audit`
(tree matching `origin/main` at audit start). No refactors performed.

Sources of measurement:

1. Line counts via `wc -l` / `wc -c` over `src/**/*.{rs,js,css}`.
2. Git change-hotspots via `git log --since='6 months ago' --numstat
   --format=COMMIT` for paths under `src/`, plus a 30-day cross-check. The
   repository's entire `src/` history (324 commits, 2026-04-15 to 2026-07-14)
   falls inside the six-month window, so six-month counts equal all-history
   counts; the 30-day window is the non-trivial recent signal.
3. Co-change pairs: files that appear together in the same commit (pairs with
   2 to 25 files per commit to skip bulk renames), computed for both windows.
4. Coupling: parse of `use crate::...` (including multi-line brace forms) into
   top-level module fan-in / fan-out; cycle detection on that directed graph.
5. Blueprint conformance: declared edges in `cairn.blueprint` versus realised
   `crate::` imports; path ownership via `cairn files` and `cairn onboard`.
6. WebUI state inventory: every `useState` in `src/ui_assets/app.js` attributed
   to its enclosing function.
7. Near-term first-impression todos read for file touchpoints
   (`accept-language-aware-gates`, `per-command-help`, `todo-listing`,
   `neighbourhood-edges`).

British spelling throughout. Em-dashes and en-dashes avoided.

## 1. File sizes and god-module candidates

Confirmed line and byte counts (worktree, 2026-07-15):

| File | Lines | Bytes | Size gate |
|---|---:|---:|---|
| `src/ui_assets/style.css` | 2729 | 56 087 | **not gated** (non-`.rs`) |
| `src/cli/mod.rs` | 2205 | 83 368 | allow-listed |
| `src/ui_assets/app.js` | 2013 | 80 963 | **not gated** (non-`.rs`) |
| `src/map/query.rs` | 1104 | 39 625 | allow-listed |
| `src/cli/render/remediate.rs` | 1054 | 39 471 | allow-listed |
| `src/summariser/store.rs` | 740 | 25 834 | allow-listed |
| `src/query_api/change_queries.rs` | 692 | 26 014 | allow-listed |
| `src/ui/mod.rs` | 691 | 24 335 | allow-listed |
| `src/query_api/mod.rs` | 555 | 20 297 | allow-listed |
| `src/scanner/mod.rs` | 522 | 19 312 | allow-listed |

Convention (`docs/conventions.md` section 2): soft split at 300 lines, hard
cap 500 lines unless `// cairn:allow-large-module reason: ...` is present.
`scripts/check-file-sizes.sh` only walks `src/**/*.rs`, so the two webui
flagships (`app.js`, `style.css`) grew past 2000 lines with zero gate signal.

### `src/cli/mod.rs` composition

The file is large (2205 lines) but not pure production. Verified structure
(brace-matched `#[cfg(test)]` modules, 2026-07-15):

- Lines 1 to 301: production dispatch (`run`, early command branches).
- Lines 303 to 421: change helpers still in production.
- Lines 422 to 462: small `#[cfg(test)] mod delegation_tests` (41 lines).
- Lines 463 to 976: **more production** after the first test block
  (`parse_args` ~476, `run_project_command` ~523, `render_loaded_project_command`
  ~591, `all_command_names` ~844, `help_text` ~881, suggestion helpers).
- Lines 977 to 2205: large `#[cfg(test)] mod tests` (1229 lines).

True split: **935 production / 1270 test** lines out of 2205. Accounting
rule (reproducible): every line from a top-level `#[cfg(test)]` attribute
through the matching `mod` closing brace counts as test, with string-aware
brace matching so raw-string fixtures inside `mod tests` do not end the
module early. That yields `delegation_tests` 422-462 (41 lines) plus `tests`
977-2205 (1229 lines) = 1270 test; 2205 - 1270 = 935 production. (Excluding
the two attribute lines and two `mod` headers would give 1266 test, the
reviewer's approximate figure; this report includes those four lines as
test.) The first `#[cfg(test)]` is **not** the production/test boundary;
substantial production code continues after `delegation_tests`. The allow-list reason remains accurate ("CLI
dispatch hub; natural seam is per-command modules which already exist for
newer commands"). Production dispatch already delegates many commands to
`src/cli/commands/*` (todo, decision, hook, wire, workspace, ...). Residual
hot path in the hub: global `--help` short-circuit at `src/cli/mod.rs:85-86`,
command registry/help text, project/change dispatch that still lives here,
and the large colocated test suite.

### `src/ui_assets/app.js` composition

Single IIFE-style Preact+htm module with section markers:

| Section | Approx. start line |
|---|---:|
| Utilities / copy helpers | 32 |
| Layout (`buildLayout`) | 298 |
| Brand mark | 459 |
| Top bar | 479 |
| Graph canvas | 548 |
| Inspector building blocks | 903 |
| Findings rollup | 1402 |
| Command palette | 1500 |
| Changes drawer | 1645 |
| Blueprint modal | 1687 |
| App root | 1734 |

`useState` inventory (23 total):

| Owner | Count | Cells |
|---|---:|---|
| `App()` | 15 | `graph`, `lint`, `meta`, `error`, `selectionId`, `selectedDecision`, `detail`, `hoveredId`, `cmdOpen`, `drawerOpen`, `blueprintOpen`, `blueprint`, `blueprintFocus`, `showFindings`, `bootTick` |
| `GraphCanvas` | 2 | `viewport`, `panState` |
| `FindingsPanel` | 2 | `scope`, `activeCategory` |
| `CommandPalette` | 2 | `q`, `activeIdx` |
| `CopyButton` | 1 | `copied` |
| `Section` | 1 | `open` |

Feature-local state already exists at the leaf components. The residual
problem is **file-level monolith ownership**: every feature still lives in one
2013-line file, so two agents editing different features always collide on
`app.js` regardless of state shape.

### Top-level Rust module size (sum of `.rs` lines)

| Module | Total lines | File count | Files over 500 |
|---|---:|---:|---:|
| `cli` | 11 256 | 36 | 4 |
| `brownfield` | 4108 | 12 | 0 |
| `query_api` | 4012 | 14 | 2 |
| `scanner` | 3944 | 10 | 2 |
| `changes` | 3563 | 12 | 1 |
| `map` | 3469 | 9 | 1 |
| `summariser` | 3319 | 10 | 2 |
| `artefacts` | 3114 | 12 | 1 |

`cli` is the largest subtree by a wide margin; much of that is intentional
command/render fan-out already present under `commands/`, `render/`,
`format/`, `export/`.

## 2. Change-hotspots (git churn)

Primary window: `--since='6 months ago'` (audit date 2026-07-15). That window
contains **324** commits touching `src/`, dated 2026-04-15 to 2026-07-14, which
is the entire `src/` history of this repository. Six-month and all-history
counts are therefore identical; all-history is not reported as a separate
table. A 30-day cross-check (`--since='30 days ago'`, 144 commits, 2026-06-15
to 2026-07-14) is included below to show whether the same files remain hot in
recent work. Module-churn SUMS below are full-tree numstat aggregations and
can differ by small deltas (±1 to 11) from path-scoped `git rev-list --count`;
file-level ranks and co-change pairs are exact.

### Six-month window (primary): top files by commit count

| Commits | File |
|---:|---|
| 81 | `src/cli/mod.rs` |
| 30 | `src/scanner/mod.rs` |
| 29 | `src/lib.rs` |
| 27 | `src/ui_assets/app.js` |
| 26 | `src/query_api/mod.rs` |
| 22 | `src/cli/commands.rs` (historical path; later split) |
| 21 | `src/ui_assets/style.css` |
| 19 | `src/reconcile/code.rs` |
| 18 | `src/cli/render/project.rs` |
| 17 | `src/query_api/registry.rs` |
| 15 | `src/ui/api.rs` (deleted after spine flip; historical) |
| 14 | `src/ui/server.rs` |
| 14 | `src/query_api/change_queries.rs` |
| 12 | `src/cli/render/remediate.rs` |
| 12 | `src/map/query.rs` |

### Six-month window: top-level module churn (sum of file-commit counts)

| Sum | Module |
|---:|---|
| 333 | `cli` |
| 122 | `query_api` |
| 92 | `scanner` |
| 76 | `reconcile` |
| 69 | `artefacts` |
| 58 | `brownfield` |
| 56 | `ui_assets` |
| 54 | `changes` |
| 53 | `summariser` |
| 48 | `map` |
| 48 | `ui` |

### Six-month window: top co-change pairs

| Count | Pair |
|---:|---|
| 15 | `src/cli/mod.rs` + `src/query_api/mod.rs` |
| 14 | `src/ui_assets/app.js` + `src/ui_assets/style.css` |
| 14 | `src/cli/mod.rs` + `src/lib.rs` |
| 11 | `src/query_api/mod.rs` + `src/query_api/registry.rs` |
| 11 | `src/cli/commands.rs` + `src/cli/mod.rs` (historical) |
| 8 | `src/ui/api.rs` + `src/ui/server.rs` (historical) |
| 8 | `src/cli/mod.rs` + `src/query_api/registry.rs` |
| 8 | `src/cli/mod.rs` + `src/scanner/mod.rs` |
| 7 | `src/ui/mod.rs` + `src/ui_assets/app.js` |
| 7 | `src/ui/server.rs` + `src/ui_assets/app.js` |

### 30-day cross-check (recent pressure)

144 commits in the last 30 days. Rankings shift but the same offenders stay
near the top:

| Commits (30d) | File |
|---:|---|
| 32 | `src/cli/mod.rs` |
| 18 | `src/cli/render/project.rs` |
| 15 | `src/ui_assets/app.js` |
| 15 | `src/query_api/mod.rs` |
| 12 | `src/cli/render/remediate.rs` |
| 11 | `src/ui/api.rs` (historical) |
| 11 | `src/cli/render/node.rs` |
| 10 | `src/cli/format/json.rs` |
| 10 | `src/cli/render/mod.rs` |
| 10 | `src/cli/commands/mod.rs` |
| 10 | `src/ui/server.rs` |
| 9 | `src/ui_assets/style.css` |
| 9 | `src/scanner/mod.rs` |

30-day module churn (sum of file-commit counts): `cli` 208, `query_api` 78,
`scanner` 49, `changes` 41, `artefacts` 40, `ui` 34, `map` 29, `ui_assets` 24.

30-day top co-change pairs:

| Count | Pair |
|---:|---|
| 10 | `src/cli/mod.rs` + `src/query_api/mod.rs` |
| 8 | `src/ui/api.rs` + `src/ui/server.rs` (historical) |
| 7 | `src/cli/commands/mod.rs` + `src/cli/mod.rs` |
| 7 | `src/ui_assets/app.js` + `src/ui_assets/style.css` |
| 7 | `src/cli/mod.rs` + `src/cli/render/project.rs` |
| 7 | `src/cli/render/mod.rs` + `src/cli/render/project.rs` |
| 6 | `src/ui/mod.rs` + `src/ui_assets/app.js` |
| 6 | `src/ui/server.rs` + `src/ui_assets/app.js` |

Interpretation:

- `cli/mod.rs` is the single worst merge hotspot in the Rust tree (81 commits
  in six months; still 32 in the last 30 days; co-changes with query registry
  and scanner).
- WebUI is a **paired** hotspot: `app.js` and `style.css` move together 14
  times in six months and 7 times in 30 days; agents editing "look" and
  "behaviour" still collide across two large files.
- Recent pressure concentrates even more on the CLI render subtree
  (`render/project.rs` 18, `render/remediate.rs` 12, `render/node.rs` 11 in
  30 days), which is consistent with the completed simplify-architecture
  programme landing presentation changes through render modules.
- The completed simplify-architecture programme (todo.simplify-architecture,
  closed 2026-07-12) already removed the `ui/api.rs` + `ui/serialise.rs`
  parallel spine; co-change on those paths is legacy signal that still appears
  in both windows because the flips landed inside them.

## 3. Coupling (who imports whom)

Top-level `use crate::` fan-out / fan-in (self-imports excluded; multi-line
brace forms included; type-path noise from nested `use crate::map::graph::{...}`
is present in raw counts, so module-level sets below are the reliable signal).

### Fan-out (unique top-level module targets)

High fan-out presentation / orchestration modules:

- `cli` reaches: `artefacts`, `blueprint`, `map`, `query_api`, `scanner`,
  `hooks`, `ui`, `summariser`, `verification`, `state`, `changes`,
  `workspace`, `error` (broad surface; expected for the user-facing hub).
- `scanner` reaches: `artefacts`, `blueprint`, `map`, `persist`, `reconcile`,
  `state` (orchestration hub; matches blueprint intent).
- `query_api` reaches: `artefacts`, `map`, `scanner`, `hooks`, `changes`,
  `summariser`, `blueprint`, `reconcile` (spine; matches one-spine design).
- `brownfield` reaches: `blueprint`, `error`, `map`, `reconcile`,
  `suggested_edges`, `summariser`.

### Fan-in (unique importers)

| Module | Fan-in | Importers (strict module-level `use crate::<name>` / `use crate::<name>::...`) |
|---|---:|---|
| `map` | 12 | artefacts, brownfield, changes, cli, hooks, lsp, query_api, reconcile, scanner, summariser, watch, workspace |
| `blueprint` | 10 | artefacts, brownfield, changes, cli, hooks, map, query_api, reconcile, scanner, summariser |
| `scanner` | **2** | `reconcile` (`use crate::scanner::config` in `src/reconcile/target.rs`), `ui` (`use crate::scanner` in `src/ui/mod.rs`) |
| `artefacts` | 7 | changes, cli, hooks, map, query_api, scanner, summariser |
| `persist` | 5 | changes, scanner, suggested_edges, summariser, workspace |

Strict module-level fan-in for a target `T` counts only import statements whose
path begins `use crate::T` or `use crate::T::...`. It does **not** count
grouped brace imports (`use crate::{ ..., T, ... }`) or path-qualified body
uses (`crate::T::...` with no matching `use`).

**Scanner separately (not in the fan-in column above):**

- Grouped brace imports that name `scanner` among other roots: `changes`,
  `cli`, `hooks`, `query_api`, `summariser`, `workspace` (and further
  `cli`/`query_api` subfiles that re-import via braces).
- Path-qualified body uses without adding new modules beyond the above:
  `crate::scanner::scan` / `load_project` in `cli/commands/{onboard,watch}.rs`,
  `crate::scanner::state::BlueprintSnapshot` in several `cli/render` tests and
  `query_api/handlers/bundle.rs`.

`map` is the structural centre of the crate: high fan-in is expected for a
graph domain model, but it means edits to shared map types have wide blast
radius.

### Dependency cycles (crate-level)

Three cycles detected on the `use crate::` graph:

1. `artefacts -> map -> artefacts`
2. `query_api -> summariser -> query_api`
3. `reconcile -> scanner -> reconcile`

None of these is a runaway web of cycles across the whole crate; they are
local mutual dependencies. They still violate the unidirectional-deps
principle and raise parallel-edit risk on the shared edges (especially
`query_api`/`summariser` and `reconcile`/`scanner`).

### Effect scatter

Side-effect concentration is mostly healthy:

- Accept gates (`cargo build` / `clippy` / `fmt` / `test`) live in
  `src/cli/accept.rs` (449 lines), not mixed through map query pure logic.
- `src/map/query.rs` is largely pure graph traversal over in-memory `Graph`.
- Filesystem and process effects cluster in CLI command modules, scanner IO,
  changes apply, and UI server. Residual concern is **module size**, not
  effects threaded through pure cores.

`src/cli/mod.rs` still imports `std::fs` and runs integration-style tests that
write trees; that is test effect scatter inside the hub file, not production
logic contamination of `map`.

## 4. Blueprint conformance

### Declared edges vs realised imports

`src/map/build.rs::validate_edges` (lines 100-133) only checks that both
edge endpoints exist as nodes. It never checks that a declared edge is
realised as an import or call. Confirmed:

```text
fn validate_edges(graph: &mut Graph, edges: &[Edge]) {
    for edge in edges {
        if !graph.nodes.contains_key(&edge.from)
            || !graph.nodes.contains_key(&edge.to) {
            // CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT
            ...
        }
        // then records inbound/outbound; no realisation check
    }
}
```

Many declared blueprint edges **are** realised (scanner->blueprint/map/reconcile,
map->blueprint/artefacts, ui->scanner, mcp->query_api via module rename
`query`/`query_api`). Some declared edges are realised indirectly (CLI reaches
scanner/hooks/ui through multi-line `use crate::{ scanner, ui, ... }` and
command modules) even when a naive single-line import scan under-counts them.

Undeclared but real edges (reach-arounds / missing blueprint edges), examples:

- `cli -> query_api` (heavy; 12+ import sites) is the one-spine design but the
  blueprint names the edge `cairn.kernel.cli -> cairn.kernel.query` (id
  mismatch between node id `query` and rust module `query_api` is
  documentation friction, not a code bug).
- `brownfield -> summariser`, `brownfield -> reconcile`,
  `brownfield -> suggested_edges` are real and not all declared.
- `query_api <-> summariser` cycle is not declared as a pair of edges.
- `reconcile -> scanner` closes a cycle not declared as such.

### Path ownership gaps

- Blueprint node `cairn.ui` claims `path "./src/ui"` only.
- `src/ui_assets/{app.js,style.css,index.html,vendor/*}` are loaded via
  `include_str!` from `src/ui/mod.rs:26-41` but are **not** listed as
  blueprint `path` entries.
- `cairn onboard` reports "No orphaned files found" because non-Rust assets
  are outside the code reconciler's orphan model. Result: the webui flagship
  is invisible to path-ownership and size gates simultaneously.
- `cairn files cairn.ui` returns only `src/ui/{mod,server,wire}.rs`.

### Intra-module health

No scan finding exists for:

- file length beyond the shell size gate (and that gate is `.rs`-only),
- fan-in / fan-out thresholds,
- god-module multi-responsibility,
- unrealised declared edges.

A node may own a 2000-line file and still scan clean. That is the measured
root cause of uncaught drift, matching the audit todo's diagnosis.

## 5. Parallel-edit boundaries

Question: can two agents edit different features without touching the same
file or shared state cell?

| Surface | Parallel-safe today? | Worst collision |
|---|---|---|
| WebUI features (canvas vs palette vs findings vs inspector) | **No** | Single `app.js` (2013 lines, 27 commits); paired `style.css` (2729 lines, 21 commits); 14 co-changes |
| WebUI leaf component state | Partially | Leaf `useState` is already local; App-level 15 cells are independent domains but co-located |
| CLI near-term todos | Mostly yes | See sequencing note |
| CLI hub (`cli/mod.rs`) | No for hub edits | 81 commits; any help/dispatch/test change lands here |
| `map` shared types | Fragile | Fan-in 12; type shape changes fan out |
| Accept gates | Yes | Isolated in `src/cli/accept.rs` |
| Todo listing | Mostly | `src/cli/render/artefacts.rs` + `query_api` handlers; not the hub |
| Neighbourhood edges | Mostly | `src/map/query.rs` + render; node is `cairn.kernel.query` |

Worst shared-file collision risks (priority order):

1. `src/ui_assets/app.js` for any concurrent webui work.
2. `src/ui_assets/style.css` co-edited with app.js.
3. `src/cli/mod.rs` for help/dispatch/registry/tests.
4. `src/query_api/mod.rs` + `registry.rs` when adding commands (co-change 11
   with each other; 15 with cli/mod.rs).

## 6. Evaluation against principles

Principles used (not a prescribed store shape):

1. Explicit state and transitions.
2. Functional core + effects at the boundary.
3. Unidirectional module dependencies (no cycles).
4. Owned modules with clear contracts.
5. Clear state ownership boundaries (feature-local or global).

| Candidate | Explicit state | Effects at boundary | Unidirectional deps | Clear ownership | Verdict |
|---|---|---|---|---|---|
| `app.js` monolith | Partial (hooks, no transitions table) | Fetch effects in App boot; mostly ok | N/A (single file) | **Fails** (one file owns all features) | Split by feature module; keep leaf-local state |
| `cli/mod.rs` hub | N/A | Effects in command modules; tests scatter FS | Fan-out high but mostly downward | Partial (commands/* exists; hub still fat) | Extract tests; keep surgical hub edits; no big-bang |
| `map/query.rs` | Pure query types | Clean | map->artefacts cycle at module level | Single responsibility (queries) with many ops | Optional split by query family later; not first-impression blocking |
| `query_api` spine | Explicit registry | Good | Cycle with summariser | Owned spine after simplify-architecture | Break cycle when next summariser work lands; not urgent |
| Accept / scanner orchestration | Explicit steps | Effects concentrated | reconcile<->scanner cycle | Owned | Cycle is real but local |

## 7. WebUI state architecture: A vs B

Flagship instance: `src/ui_assets/app.js` (2013 lines, 23 `useState`, 15 in
`App()`).

| Criterion | (A) Feature-local reducers / small state machines, modules owned by feature | (B) Single global TEA-style Model/Update store |
|---|---|---|
| **1. Parallel-editability** | **Wins.** Two agents can own `graph-canvas.js` and `command-palette.js` without merging the same file once the monolith is split. Leaf state already local (`GraphCanvas`, `FindingsPanel`, `CommandPalette`). App-level cells map cleanly onto feature owners (selection, overlays, boot data, blueprint modal). | **Loses for parallel agents.** Every feature transition lands in one `update` function / one Model type. Relocates the merge hotspot from "the monolith file" to "the global store file" unless the store is itself sharded (which collapses toward A). |
| **2. Coupling / fan-in of state owner** | State owner fan-in is per feature. Changing findings filters does not typecheck against canvas viewport. Cross-feature events (select node from palette) need narrow explicit props or tiny event bus; that is real coupling but localised. | Single Model is high fan-in: every view depends on the store shape. One field rename can touch every feature module. |
| **3. Change-hotspot concentration** | After split, churn should distribute across feature files. `app.js`+`style.css` co-change (14) still needs CSS discipline (feature-scoped styles or clear CSS sections), but JS collision drops. | Concentrates JS churn into Model/Update. Measured hotspot (27 commits on app.js) would become commits on the store unless carefully namespaced. |

### Verdict: **Option A (feature-local)**

Evidence that decides it:

1. Parallel-editability is the audit's primary pain ("parallel vibe-mode agents
   collide"). Option B does not solve file-level collision unless the store is
   sharded, which is Option A by another name.
2. Existing code already leans A at the leaves (8 of 23 `useState` cells are
   outside `App`). The missing step is **file extraction**, not centralisation.
3. A single global store is **not** automatically more parallel-friendly; the
   measured co-change and commit concentration on one file show that
   consolidating ownership increases hotspot risk.
4. Premature centralisation is explicitly out of scope for this audit; the
   measurements do not produce a counter-argument strong enough to override
   that caution.

What A is **not**: "leave 15 useState cells forever" or "no shared state".
Shared boot data (`graph`, `lint`) and selection are legitimate App-level (or
small shared) state. The prescription is:

- Extract feature modules (canvas, inspector, findings, palette, blueprint
  modal, top bar) into separate files under `src/ui_assets/`.
- Keep feature-local UI state in those modules.
- Pass shared selection/boot data as props (or a thin context) without a
  global action enum for every keystroke.

Do not adopt Redux/Elm/Zustand as a dependency for this; Preact hooks already
express the needed local state.

## 8. Self-guardrail recommendation

### Why drift went uncaught (confirmed)

1. Edge validation is endpoint-only (`validate_edges`); no realisation check;
   no intra-module health signal.
2. `scripts/check-file-sizes.sh` is `.rs`-only; `app.js` and `style.css` exempt.
3. `src/ui_assets/*` is not a blueprint `path`, so ownership and orphan
   machinery never see the flagship files.

### Recommendation: **yes, make cairn catch this itself** (two cheap signals)

| Signal | Effort | Evidence it would have fired |
|---|---|---|
| **Extend size gate beyond `.rs`** to cover blueprint-owned and known webui paths (`src/ui_assets/**/*.{js,css}`, optionally other non-Rust owned paths) with the same 500-line / allow-list protocol | Low (shell script generalisation; allow-list comment style for JS/CSS) | Would have failed on `app.js` (2013) and `style.css` (2729) long before this audit |
| **Oversized-module scan finding** (Warning or Info) when a node-owned file exceeds N lines without allow-list, emitted by scanner/map integrity | Medium (new finding code, contract, tests) | Same files, plus surfaces the issue in `cairn scan` / remediate, not only CI shell |

Recommend implementing the **size-gate extension first** (highest ROI, matches
existing mechanism). Follow with an optional scan finding if the team wants
the signal inside `cairn scan` for non-CI agent loops.

Not recommended now:

- Full import-graph edge-realisation validation (high cost, false positives on
  dynamic dispatch and re-exports; the simplify-architecture spine already
  reduced the worst duplication).
- Hard fan-in/fan-out quotas without a longer baseline (map's fan-in 12 is
  structural, not a defect by itself).

## 9. Sequencing note (first-impression work)

Near-term todos and measured touchpoints:

| Todo | Node | Primary files | Needs pre-split? |
|---|---|---|---|
| `accept-language-aware-gates` | `cairn.kernel.cli` | `src/cli/accept.rs` | **No.** Already isolated (449 lines). |
| `per-command-help` | `cairn.kernel.cli` | `src/cli/mod.rs:85-86` help short-circuit, `help_text`, copy.toml | **No full split.** Surgical edit to the hub; do not block on extracting tests first. |
| `todo-listing` | `cairn.kernel.cli` | `src/cli/render/artefacts.rs`, query_api artefact handlers | **No.** Outside the hub file. |
| `neighbourhood-edges` | `cairn.kernel.query` | `src/map/query.rs`, human renderer | **No.** Single-agent friendly; map/query allow-listed for multi-query hub reasons. |
| WebUI first-impression (`webui-mobile-graph-nav`, `webui-simplicity-review`) | `cairn.ui` | `app.js` + `style.css` | **If two agents work webui in parallel, split first.** If one agent at a time, split can follow; still backlog it. |

### `cairn.kernel.cli` parallelism verdict

**Not a blocking pre-split for the listed first-impression batch.** Several
open todos sit on the node, but their file touchpoints are already mostly
disjoint (`accept.rs`, `render/artefacts.rs`, query handlers). The hub remains
a hotspot for help/dispatch work, yet a big-bang `cli/mod.rs` refactor would
delay first-impression delivery for little concurrency gain on this batch.

Optional later (not sequenced before first-impression): extract the large
`#[cfg(test)]` block from `cli/mod.rs` into `src/cli/tests/` to shrink the
hub's merge surface for future help/registry work.

### What to split before parallel webui work

If and only if multiple agents will touch the webui concurrently: extract
feature modules from `app.js` first (Option A). Otherwise continue serial
webui todos and land the modularisation backlog item immediately after.

Do **not** start a big-bang architecture refactor that blocks first-impression
work. Measurement does not justify it.

## 10. Prioritised modularisation backlog (summary)

Items justified by measurement (each becomes its own native todo):

1. **WebUI feature-module split** (`app.js` / paired CSS discipline) on
   `cairn.ui`. Highest parallel-edit ROI; Option A.
2. **Size gate for non-Rust paths** on project tooling / root. Would have
   caught the flagship drift automatically.
3. **Oversized-module scan finding** on `cairn.kernel.scanner` (or map
   integrity). Second self-guardrail; pairs with (2) for agent-visible signal.
4. **Blueprint path claim for `src/ui_assets`** under `cairn.ui`. Makes
   ownership and future gates able to see the flagship files.

Explicitly **not** filed as pre-work: full `cli/mod.rs` production rewrite,
global TEA store adoption, map/query multi-file split, cycle-breaking of
artefacts/map or query_api/summariser (track when those modules next change).

## Appendix: commands used

```text
wc -l / find line counts
git log --since='6 months ago' --numstat --format='COMMIT %H' -- src/
git log --since='30 days ago' --numstat --format='COMMIT %H' -- src/
python coupling / co-change scripts (session-local)
cairn files cairn.ui
cairn files cairn.kernel.cli
cairn status / todos / onboard / scan
rg / structural reads of app.js, cli/mod.rs, map/build.rs, check-file-sizes.sh
```
