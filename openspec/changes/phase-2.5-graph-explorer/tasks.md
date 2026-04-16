# Tasks: Phase 2.5 Graph Explorer

## 1. Embedded Web Server

- [ ] 1.1 Add `cairn ui` subcommand to the CLI with `--port` (default 3000) and `--no-open` flags.
- [ ] 1.2 Integrate an embedded HTTP server (Axum or Warp) that serves bundled static assets.
- [ ] 1.3 Embed HTML/CSS/JS assets in the binary using `rust-embed` or `include_dir`.
- [ ] 1.4 On startup, open the default browser to the server URL (skip if `--no-open`).
- [ ] 1.5 Handle port conflicts with a clear error message.
- [ ] 1.6 Graceful shutdown on Ctrl+C.

## 2. Query Bridge API

- [ ] 2.1 Implement `GET /api/graph` — delegates to `neighbourhood(root, transitive=true)`.
- [ ] 2.2 Implement `GET /api/node/:id` — delegates to `get(id)`.
- [ ] 2.3 Implement `GET /api/node/:id/contract` — delegates to `contract(id)`.
- [ ] 2.4 Implement `GET /api/node/:id/decisions` — delegates to `decisions(id)`.
- [ ] 2.5 Implement `GET /api/node/:id/todos` — delegates to `todos(id)`.
- [ ] 2.6 Implement `GET /api/node/:id/research` — delegates to `research(id)`.
- [ ] 2.7 Implement `GET /api/node/:id/sources` — delegates to `sources(id)`.
- [ ] 2.8 Implement `GET /api/node/:id/rationale` — delegates to `rationale(id)`.
- [ ] 2.9 Implement `GET /api/dependents/:id` and `GET /api/depends/:id`.
- [ ] 2.10 Implement `GET /api/lint` — delegates to `lint()`.
- [ ] 2.11 Implement `GET /api/status` — delegates to `status()`.
- [ ] 2.12 Implement `GET /api/meta` — returns `schema_version` and available command list.
- [ ] 2.13 All endpoints return the same typed JSON structs as `cairn --format json`.

## 3. Graph View (Browser)

- [ ] 3.1 Implement hierarchical (layered) graph layout with edge-crossing minimisation.
- [ ] 3.2 Render nodes with type badges (system/container/module/actor), name, and stable ID.
- [ ] 3.3 Render ownership edges as solid lines, dependency edges as dashed lines with arrowheads.
- [ ] 3.4 Implement pan and zoom controls.
- [ ] 3.5 Implement collapse/expand for container nodes (collapsed shows child count badge).
- [ ] 3.6 Implement node selection: click highlights node, connected edges, and shows dependency labels.
- [ ] 3.7 Implement tag-based node coloring for domain identification.
- [ ] 3.8 For 200+ node graphs, default to systems-and-containers-only view with drill-down.

## 4. Node Detail Panel

- [ ] 4.1 Implement side panel that opens on node click, showing type, name, ID, and description.
- [ ] 4.2 Load artefact data lazily via API calls when a node is selected.
- [ ] 4.3 Render artefact types as accordion sections with colored indicators.
- [ ] 4.4 Expand the first artefact (contract) by default.
- [ ] 4.5 Implement Next/Back navigation with layer counter.
- [ ] 4.6 Implement generic artefact template for unknown artefact types (forward-compatibility).
- [ ] 4.7 Close panel on re-click or close button; clear selection and edge highlights.

## 5. Integrity Overlay

- [ ] 5.1 Fetch `GET /api/lint` on graph load.
- [ ] 5.2 Render structural errors as red badges on affected nodes.
- [ ] 5.3 Render interface contradictions as amber badges.
- [ ] 5.4 Render rationale tensions as gray badges.
- [ ] 5.5 Clicking a badge opens the finding detail in the node detail panel.

## 6. UI Maintenance Contract

- [ ] 6.1 Document the UI Maintenance Contract in a `ui-contract.md` file in the project root or `meta/` directory.
- [ ] 6.2 Define schema version tracking: `GET /api/meta` returns `schema_version`; UI warns on mismatch.
- [ ] 6.3 Define forward-compatibility rules: unknown fields ignored, missing fields show placeholders, unknown artefact types use generic template.
- [ ] 6.4 Define Phase 3 addendum requirement: temporal navigation UI deliverable.
- [ ] 6.5 Define Phase 7 addendum requirement: MCP transport adapter deliverable.
- [ ] 6.6 Define per-phase compatibility note requirement for any `CairnQuery`/`CairnResponse` schema changes.

## 7. Required Verification

- [ ] 7.1 `cargo build` passes with zero warnings.
- [ ] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 7.3 `cargo fmt --check` passes.
- [ ] 7.4 `cargo test` — all tests pass.
- [ ] 7.5 `cargo test --locked` passes.
- [ ] 7.6 `cairn ui --port 0` starts the server and returns a valid port (integration test).
- [ ] 7.7 Headless browser test: graph renders nodes, click opens detail panel, artefact navigation works.
- [ ] 7.8 Scale test: 200-node fixture renders within 2 seconds with no overlapping labels.
- [ ] 7.9 Lint overlay test: deliberate structural errors show correct badges.
- [ ] 7.10 Forward-compatibility test: unknown artefact type renders with generic template.
