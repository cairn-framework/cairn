# Design: Phase 2.5 Graph Explorer

## References

- `docs/spec.md` section 12 for query interface and response formats.
- `docs/spec.md` section 8 for artefact type schemas.
- `docs/spec.md` section 6 for DSL node types and structural graph.
- `docs/spec.md` section 10 for integrity rules and contradiction classes.

## Architecture

The graph explorer follows the spec's principle that visual dashboards are "a distribution concern for a downstream tool that consumes Cairn's JSON output" (section 5). The UI is that downstream tool, built as a validation harness during kernel development.

```
┌─────────────────────────────────┐
│  cairn ui (Rust binary)         │
│  ┌───────────┐  ┌────────────┐  │
│  │ Embedded   │  │ Query      │  │
│  │ Web Server │  │ Bridge     │  │
│  │ (static    │  │ (calls     │  │
│  │  assets)   │  │  cairn lib │  │
│  │            │  │  directly) │  │
│  └─────┬─────┘  └─────┬──────┘  │
│        │              │          │
│        │   JSON API   │          │
│        └──────┬───────┘          │
└───────────────┼──────────────────┘
                │
         ┌──────┴──────┐
         │  Browser    │
         │  (bundled   │
         │   HTML/JS)  │
         └─────────────┘
```

### Embedded Web Server

The `cairn ui` subcommand starts a lightweight HTTP server (e.g., Axum or Warp) on a local port. Static assets (HTML, CSS, JS) are embedded in the binary via `include_dir` or `rust-embed`. No external build tools or Node.js required at runtime.

The server exposes a JSON API that delegates to the cairn library's query functions directly — the same functions that back the CLI. This avoids shell-exec overhead and gives the UI access to typed responses.

### Query Bridge

A thin HTTP handler layer that maps REST-like routes to cairn query functions:

```
GET /api/graph              → neighbourhood(root, transitive=true)
GET /api/node/:id           → get(id)
GET /api/node/:id/contract  → contract(id)
GET /api/node/:id/decisions → decisions(id)
GET /api/node/:id/todos     → todos(id)
GET /api/node/:id/research  → research(id)
GET /api/node/:id/sources   → sources(id)
GET /api/node/:id/rationale → rationale(id)
GET /api/dependents/:id     → dependents(id, transitive=false)
GET /api/depends/:id        → depends(id, transitive=false)
GET /api/lint               → lint()
GET /api/status             → status()
GET /api/meta               → schema_version, available_commands
```

All responses use the same typed JSON structs as the CLI's `--format json` output.

### Graph Layout Engine

The browser-side graph renderer uses a hierarchical (layered) layout algorithm for the structural graph:

- **Hierarchy-aware layering.** Systems at the top, containers in the middle, modules at the bottom. The layout engine (e.g., Dagre, ELK.js, or a custom layer-based algorithm) assigns nodes to tiers based on their containment depth, then minimises edge crossings within each tier.
- **Collapse/expand.** Container nodes can be collapsed to hide their children, reducing visual density. The collapsed state shows a node count badge.
- **Clustering.** Nodes with shared tags cluster visually. Tag-based coloring helps identify functional domains.
- **Scale handling.** For 200+ node graphs, the initial view shows systems and containers only. Modules appear on drill-down. This prevents the "hairball" problem.

Technology: DOM-based rendering using absolutely-positioned HTML elements for nodes (for rich text, badges, and click handling) and SVG overlays for edges. No heavy framework dependencies. The entire UI ships as a self-contained HTML/CSS/JS bundle embedded in the Rust binary.

### Node Detail Panel

A side panel that appears when a node is selected. Content is loaded lazily — clicking a node triggers API calls for that node's artefacts.

The panel uses an accordion layout:
- Each artefact type is a collapsible section with a colored indicator.
- The first available artefact (typically the contract) is expanded by default.
- Next/Back navigation steps through artefact types sequentially.
- Artefact content renders as formatted text extracted from the markdown frontmatter and body.

### Integrity Overlay

`GET /api/lint` returns all findings grouped by class. The UI maps these to visual indicators:

| Finding class | Visual treatment |
|---|---|
| Structural error | Red badge on affected node, pulsing border |
| Interface contradiction | Amber badge, dashed border on node |
| Rationale tension | Gray badge, subtle dotted border |

Clicking a badge opens the finding detail inline in the node detail panel.

### Forward-Compatibility

The UI Maintenance Contract requires forward-compatible rendering:

- **Unknown artefact types.** If a query returns an artefact type not in the UI's known list, render it with the generic template (title + frontmatter fields + body text).
- **Unknown fields.** Extra JSON fields in query responses are silently ignored.
- **Missing fields.** If an expected field is absent, the UI shows a placeholder ("Not available") rather than crashing.
- **Schema version check.** On startup, the UI reads `GET /api/meta` and warns if `schema_version` is newer than the UI was built for.

## Testing

- Unit tests for query bridge handlers: each route returns correctly shaped JSON.
- Integration test: `cairn ui --port 0` starts the server, returns a port, and the test fetches `/api/graph` and validates the response.
- Browser-level smoke test: a headless browser test loads the UI, verifies the graph renders nodes, clicks a node, and verifies the detail panel appears.
- Scale test: generate a fixture DSL with 200 nodes and verify layout completes within 2 seconds with no overlapping labels.
- Lint overlay test: introduce deliberate structural errors in a fixture and verify the UI shows the correct badges.
