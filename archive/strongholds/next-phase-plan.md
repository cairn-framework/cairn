# Next phase plan: cairn hardening and webui repair

## 1. What we fixed today (2026-06-03)

### 1.1 Reconciler duplication bug (critical)

**Problem:** `cairn lint` produced 138 identical findings from 6 unique orphaned files.

**Root cause:** `reconcile_targets()` in `src/scanner/mod.rs` called the Rust reconciler once per target. Each reconciler scans the entire project root, so N targets produced N copies of every finding.

**Fix:** Cache reconciler results by `Language` so each reconciler runs exactly once per scan. Findings are collected once per cached run.

**Result:** `cairn lint` now exits 0 with 6 info findings (3 demo example orphans + 3 legitimate src/ orphans).

### 1.2 Blueprint drift (3 orphaned modules)

**Problem:** `src/sse.rs`, `src/state/mod.rs`, and `src/watch.rs` existed in code but were not declared in `cairn.blueprint`.

**Fix:** Added all three as top-level modules under the `cairn` System node:
- `cairn.sse` — SSE event stream parser for Gas City integration
- `cairn.state` — pluggable state persistence backend (filesystem + beads)
- `cairn.watch` — watch mode with finding-change events

Created `meta/decisions/close-blueprint-drift.md` to satisfy the `CH001` architecture hook.

**Result:** `cairn hook all` now passes. Blueprint models the full source tree.

### 1.3 Stale installed binary

**Problem:** `~/.cargo/bin/cairn` was from May 11, predating all 118 commits from the feature branch.

**Fix:** Ran `cargo install --path . --force` to update the installed binary.

**Result:** `cairn lint`, `cairn hook all`, and `cairn scan` all use the current code.

### 1.4 Dogfood gate script

**Problem:** No automated enforcement that cairn can lint itself. CI did not run cairn against the repo.

**Fix:** Created `scripts/dogfood.sh` that runs `cairn lint` and `cairn hook all`. Wired into:
- `.git/hooks/pre-push` — runs on every push
- `.github/workflows/dogfood.yml` — runs on every PR and push to dev/main

**Result:** Local and CI enforcement are identical. A developer cannot push broken blueprint state.

## 2. What remains broken

### 2.1 Webui module nodes do not render (confirmed)

**Symptom:** System and Container nodes render correctly on the canvas. Module nodes are invisible. Edge labels float in space with no attached nodes. Inspector panel and command palette work correctly.

**Evidence:**
- API returns correct graph data (23 nodes, 25 edges after blueprint fix)
- `/api/graph` includes all module nodes with correct `kind`, `name`, `parent`, `state`
- `buildLayout()` places modules at correct coordinates
- `ModuleNode` component is defined and referenced in the render loop
- No console errors visible from code inspection

**Suspected causes (unverified):**
1. CSS `animation-fill-mode: both` on `.canvas-node.settling` might leave nodes at `opacity: 0` or `transform: scale(0)` if the animation keyframes are malformed in some browser environments
2. Preact/htm might be silently dropping `ModuleNode` renders if `key` values collide or props are undefined
3. The `artefactCountsById` useMemo only populates counts for the selected node, leaving all others with `null` counts; while `ModuleNode` has a fallback, there might be a subtle reactivity issue
4. The webui server (`cairn ui`) runs `load_project()` on every API request. On slower machines or with larger projects, this causes request timeouts. The server is single-threaded and serializes all requests

**Next step:** Browser debugging session. Open the webui in a browser with dev tools, inspect the SVG DOM for missing `<g>` elements, check the Console for JS errors, and verify the Network tab shows successful `/api/graph` responses.

### 2.2 Webui server reliability

**Problem:** `cairn ui` binds the port and prints the URL, but curl requests to the server often return empty responses or time out. The server process may exit silently when backgrounded.

**Suspected cause:** The server uses a blocking accept loop with `thread::sleep(Duration::from_millis(25))` between polls. If the process is backgrounded via shell job control, signals might interrupt the sleep or the accept loop. Additionally, `load_project()` is called synchronously on every API request, which blocks the server for 3-4 seconds.

**Fix needed:**
1. Cache the `ScanResult` in the server and only re-scan when the blueprint file changes (watch with mtime or fsnotify)
2. Add a write timeout to the TCP stream
3. Handle backgrounding gracefully (ignore SIGHUP, or use `nohup` pattern)

## 3. Next priorities (ranked by leverage)

### Priority 1: Fix webui module rendering

Without this, the webui is unusable. The graph canvas is the primary value proposition.

**Tasks:**
1. Open webui in browser with dev tools
2. Inspect SVG DOM: are module `<g>` elements present but invisible, or absent entirely?
3. Check Console for JS errors
4. Verify Network tab: does `/api/graph` return valid JSON?
5. Fix the root cause (CSS, JS data binding, or Preact key issue)
6. Add a test: `tests/webui_rendering.rs` that starts the server, fetches `/api/graph`, and asserts that the HTML response contains at least one `.canvas-node[data-kind="module"]` element

### Priority 2: Cache scan results in the webui server

**Problem:** Every API request triggers a full project scan (~3 seconds). This makes the webui feel sluggish and can cause timeouts.

**Tasks:**
1. Add a `cached_scan: Option<(ScanResult, SystemTime)>` field to `Server`
2. In `load_project()`, check if the cached result is fresh (e.g., < 5 seconds old, or blueprint file mtime unchanged)
3. Return cached result if fresh, otherwise scan and cache
4. Add a `Cache-Control: max-age=5` header to API responses
5. Test: verify that two rapid requests to `/api/status` return quickly

### Priority 3: Add webui to CI

**Problem:** The webui has zero test coverage in CI. Breakages go unnoticed.

**Tasks:**
1. Add a CI step that builds the webui assets (`cargo build` already includes them)
2. Start `cairn ui` in the background during CI
3. Run `curl` against `/api/status`, `/api/graph`, `/api/lint` and assert 200 responses
4. Run a headless browser test (Puppeteer or Playwright) that opens the webui and asserts at least one module node is visible

### Priority 4: Demo example blueprint drift

**Problem:** `examples/demo/src/api/lib.rs`, `examples/demo/src/auth/lib.rs`, and `examples/demo/src/db/lib.rs` are orphaned. These are part of the demo example, not the main cairn repo.

**Options:**
1. Add a `cairn.blueprint` to the `examples/demo/` directory that owns these files
2. Add `examples/demo/` to `.cairnignore` or the scanner's built-in ignores
3. Accept the info findings as expected (they only show in `cairn lint`, not `cairn hook all`)

**Recommendation:** Option 1. The demo should be a complete, self-contained cairn project.

### Priority 5: Webui write-surface evaluation

The stronghold decision (`docs/strongholds/webui-direction.md`) records a read-mostly direction for 12 months. Before committing to this, we need evidence.

**Tasks:**
1. Track webui open frequency vs CLI invocation frequency (telemetry or shell history analysis)
2. Interview 2-3 users: do they open the webui to understand the graph or to do work?
3. If evidence supports write-surface, spec the POST endpoints and form components
4. If evidence supports read-mostly, implement the CLI-handoff fallbacks for C2.d, C3.e, C13.f, C12.f

## 4. Immediate next session

1. **Browser debugging:** Open `cairn ui` in a real browser, inspect the SVG DOM, find why modules don't render
2. **Fix the rendering bug:** One-line CSS fix or data-binding fix
3. **Verify:** Screenshot the fixed canvas showing all 23 nodes
4. **Commit and push:** Include the fix, the dogfood script, and the CI workflow

## 5. Long-term roadmap

| Phase | Work | Status |
|---|---|---|
| Phase 7.6 | AI Provenance (snapshot tests, reconciler integration) | ~60% shipped, 18 tasks pending (mostly obsolete due to cflx retirement) |
| Phase 7.7 | UX Foundation (webui, command palette, inspector) | Shipped but webui rendering broken |
| Phase 8 | Summariser | Shipped |
| Phase 9 | Brownfield (init --from-code, refine, onboard) | ~40% shipped, docs missing |
| Phase 10 | Distribution (LSP de-scoped) | Complete |
| **Next** | **Hardening (dogfood CI, webui fix, scan caching)** | **In progress** |
| Post-next | AI-narrative + stamping (C7.a provenance-block schema) | Blocked on C7.c line-range vs content-anchor decision |
| Future | Real LSP (diagnostics, completion, hover) | Unbuilt, needs spec |

---

*Plan written: 2026-06-03. Revisit after webui rendering fix.*
