# Cairn webui v2 visual verification log

Verified via `mcp__claude-in-chrome` against http://127.0.0.1:4137/ serving the
`test/fixtures/cairn-bootstrap` project. Screenshots were captured iteratively;
image IDs from the MCP are listed below. Full images were viewed by the
implementer during the session; this file records the milestone checkpoints.

## Environment

- Server: `cargo run --bin cairn -- ui --port 4137 --no-open` run from
  `test/fixtures/cairn-bootstrap` after `cargo build`.
- Fixture stats (from `/api/status`): 12 nodes, 9 edges, 17 findings, 0 errors,
  17 warnings.
- Browser: Chrome via the `mcp__claude-in-chrome` MCP.

## Milestones

| # | Checkpoint | Screenshot ID | Console errors | Notes |
|---|-----------|---------------|----------------|-------|
| 1 | First render (cairn.kernel.changes selected from localStorage) | `ss_3624bxu0g` | none | All components wired; minor module-badge overlap caught. |
| 2 | Module inspector tuned (badge moved, recent-row widths) | `ss_8765i9b73` | none | Empty-inspector stat grid shows 10/10/0. Ghost badges clean. |
| 3 | ReconcilerInterface selected | `ss_63646n15z` | none | Blueprint card + paths + chain balance render end to end. |
| 4 | Inspector scrolled to sections | `ss_8851gvhq7` | none | All 7 accordion sections present; Research=1, others 0. |
| 5 | `⌘K` palette opened | `ss_496073yoa` | none | Fixed class collision with components.css `.palette` swatch. |
| 6 | Changes drawer open (17 findings) | `ss_2991pvt28` | none | Horizontal card strip, state badges, slug paths. |
| 7 | Blueprint modal open | `ss_8101l3zko` | none | Full source with syntax highlighting via `/api/blueprint`. |
| 8 | Decision detail selected from Kernel container | `ss_3193fb9eu` | none | Back button, status pill, rationale prose. |
| 9 | Hinge diagram visible in decision view | `ss_95291wc13` | none | Provenance / authority split, pivot rod, attached-module card. |

## Interactive checks

- Breadcrumb walks parent chain correctly (`cairn . kernel . reconciler`).
- Hover on a module illuminates connected edges and dims the rest
  (2 traced of 20 total edges observed on a mid-graph module).
- Command palette keyboard handler (`⌘K`) + `Escape` close works.
- Depends on / Dependents accordions populate from `/api/depends/:id` and
  `/api/dependents/:id`, showing name + reconciliation badge, clickable.
- Paths block always visible on module inspector (not in accordion).
- Decision detail condition parser falls back to full body when
  `## Rationale` header is absent (as in bootstrap fixture).

## Console state

Console was polled with `onlyErrors: true` after each milestone; no
errors or exceptions were recorded.

## API snapshots

See sibling files in this directory for raw API output used during
verification.
