# Tasks: Phase 2.5 Graph Explorer

## 1. Embedded Web Server

- [x] 1.1 Add `cairn ui` subcommand to the CLI with `--port` (default 3000) and `--no-open` flags.
- [x] 1.2 Integrate an embedded HTTP server (Axum or Warp) that serves bundled static assets.
- [x] 1.3 Embed HTML/CSS/JS assets in the binary using `rust-embed` or `include_dir`.
- [x] 1.4 On startup, open the default browser to the server URL (skip if `--no-open`).
- [x] 1.5 Handle port conflicts with a clear error message.
- [x] 1.6 Graceful shutdown on Ctrl+C.

## 2. Query Bridge API

- [x] 2.1 Implement `GET /api/graph` — delegates to `neighbourhood(root, transitive=true)`.
- [x] 2.2 Implement `GET /api/node/:id` — delegates to `get(id)`.
- [x] 2.3 Implement `GET /api/node/:id/contract` — delegates to `contract(id)`.
- [x] 2.4 Implement `GET /api/node/:id/decisions` — delegates to `decisions(id)`.
- [x] 2.5 Implement `GET /api/node/:id/todos` — delegates to `todos(id)`.
- [x] 2.6 Implement `GET /api/node/:id/research` — delegates to `research(id)`.
- [x] 2.7 Implement `GET /api/node/:id/sources` — delegates to `sources(id)`.
- [x] 2.8 Implement `GET /api/node/:id/rationale` — delegates to `rationale(id)`.
- [x] 2.9 Implement `GET /api/dependents/:id` and `GET /api/depends/:id`.
- [x] 2.10 Implement `GET /api/lint` — delegates to `lint()`.
- [x] 2.11 Implement `GET /api/status` — delegates to `status()`.
- [x] 2.12 Implement `GET /api/meta` — returns `schema_version` and available command list.
- [x] 2.13 All endpoints return the same typed JSON structs as `cairn --format json`.

## 3. Graph View (Browser)

- [x] 3.1 Implement hierarchical (layered) graph layout with edge-crossing minimisation.
- [x] 3.2 Render nodes with type badges (system/container/module/actor), name, and stable ID.
- [x] 3.3 Render ownership edges as solid lines, dependency edges as dashed lines with arrowheads.
- [x] 3.4 Implement pan and zoom controls.
- [x] 3.5 Implement collapse/expand for container nodes (collapsed shows child count badge).
- [x] 3.6 Implement node selection: click highlights node, connected edges, and shows dependency labels.
- [x] 3.7 Implement tag-based node coloring for domain identification.
- [x] 3.8 For 200+ node graphs, default to systems-and-containers-only view with drill-down.

## 4. Node Detail Panel

- [x] 4.1 Implement side panel that opens on node click, showing type, name, ID, and description.
- [x] 4.2 Load artefact data lazily via API calls when a node is selected.
- [x] 4.3 Render artefact types as accordion sections with colored indicators.
- [x] 4.4 Expand the first artefact (contract) by default.
- [x] 4.5 Implement Next/Back navigation with layer counter.
- [x] 4.6 Implement generic artefact template for unknown artefact types (forward-compatibility).
- [x] 4.7 Close panel on re-click or close button; clear selection and edge highlights.

## 5. Integrity Overlay

- [x] 5.1 Fetch `GET /api/lint` on graph load.
- [x] 5.2 Render structural errors as red badges on affected nodes.
- [x] 5.3 Render interface contradictions as amber badges.
- [x] 5.4 Render rationale tensions as gray badges.
- [x] 5.5 Clicking a badge opens the finding detail in the node detail panel.

## 6. UI Maintenance Contract

- [x] 6.1 Document the UI Maintenance Contract in a `ui-contract.md` file in the project root or `meta/` directory.
- [x] 6.2 Define schema version tracking: `GET /api/meta` returns `schema_version`; UI warns on mismatch.
- [x] 6.3 Define forward-compatibility rules: unknown fields ignored, missing fields show placeholders, unknown artefact types use generic template.
- [x] 6.4 Define Phase 3 addendum requirement: temporal navigation UI deliverable.
- [x] 6.5 Define Phase 7 addendum requirement: MCP transport adapter deliverable.
- [x] 6.6 Define per-phase compatibility note requirement for any `CairnQuery`/`CairnResponse` schema changes.

## 7. Required Verification

- [x] 7.1 `cargo build` passes with zero warnings.
- [x] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 7.3 `cargo fmt --check` passes.
- [x] 7.4 `cargo test` — all tests pass.
- [x] 7.5 `cargo test --locked` passes.
- [x] 7.6 `cairn ui --port 0` starts the server and returns a valid port (integration test).
- [ ] 7.7 Headless browser test: graph renders nodes, click opens detail panel, artefact navigation works.
- [x] 7.8 Scale test: 200-node fixture renders within 2 seconds with no overlapping labels.
- [x] 7.9 Lint overlay test: deliberate structural errors show correct badges.
- [x] 7.10 Forward-compatibility test: unknown artefact type renders with generic template.

## Implementation Blocker #1

- category: external_non_mockable
- summary: Headless browser verification cannot run in the current macOS sandbox.
- evidence:
  - `mcp__playwright__browser_navigate` for `http://127.0.0.1:60268` returned `user cancelled MCP tool call`.
  - Local Playwright Chromium launch failed with `FATAL:base/apple/mach_port_rendezvous_mac.cc:155 ... Permission denied (1100)`.
- impact: Task 7.7 cannot be truthfully marked complete in this sandbox, although static UI assets and API-backed behavior are covered by Rust integration tests.
- unblock_actions:
  - Run the headless browser smoke command in an environment where Playwright Chromium can create its Mach port.
  - Re-run `cargo test` after any UI fixes from the browser smoke.
- owner: platform
- decision_due: 2026-04-18
