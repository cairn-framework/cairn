---
node: cairn.ui
status: open
created: 2026-07-11
---

# WebUI: progressive disclosure fixes from AutoDocs comparison

Comparison review completed 2026-07-11 (Cairn webui src/ui_assets versus AutoDocs dashboard, cloned locally). Verdict: navigation model is fine and simpler than theirs; the first screen is overloaded by unexplained encodings and a power-user inspector shown by default. Fix with progressive disclosure; cut nothing Cairn-unique (findings overlay, reconciliation states, chain balance, provenance sections, ProseNudge all stay).

Concrete changes, in priority order:

1. ModuleInspector (app.js ~1033-1192): collapse all nine artefact sections by default (Decisions open only when present); for a node with zero artefacts show one calm summary line instead of nine empty sections; move the chain-balance widget below prose and paths. Biggest first-run win.
2. Unify findings into one surface: keep the bottom-right drawer (app.js ~1607-1643) as canonical; merge or retire the separate FindingsPanel (~1364-1456); the Map overview's "Recent findings" becomes a link into the drawer.
3. Chain banner (app.js ~759-763): make the Provenance/Hinge/Authority legend a dismissible first-session coach-mark with a one-line explanation, not a permanent unexplained label.
4. On node select, focus the canvas on the node plus its depends/dependents neighbourhood (data already fetched, app.js ~178-188) instead of relying on the whole-map splay; borrowed from AutoDocs' scoped per-file graph.
5. Minimap (app.js ~831-848): add hover labels or demote to a toggle; 48 unlabeled dots are cryptic.
6. Map overview: add one prominent primary action line ("⌘K to query, or click any stone"); AutoDocs' single-obvious-action is the lesson.

Preserve list (moat, do not cut): findings severity overlay and breathing animation, synced/planned/ghost/orphaned states and counts, chain-balance model (explain, don't remove), artefact provenance sections, drift findings, ProseNudge, breadcrumb ancestry, dependency-versus-ownership edges.

Acceptance: first-run screen shows map, overview panel, collapsed findings drawer, coach-mark; clicking an artefact-less node shows a single summary, not nine empty sections; findings reachable from exactly one canonical surface; all webui gates pass (design tokens, biome, a11y, visual harness).

Note: the webui's state architecture (2,013-line `app.js` monolith with scattered component-local `useState`, no central store) is a known modularity concern tracked as a flagship instance under `todo.architecture-modularity-audit`. That audit measures coupling, change-hotspots, and parallel-edit boundaries first, then evaluates clean-architecture / Elm-Architecture principles (explicit, testable state transitions; effects at boundaries; clear state ownership boundaries that may be feature-local or global) via an outcome-neutral comparison of feature-local reducers / small state machines versus a single global TEA-style Model/Update store. Do not assume the fix is a single global store (that can just relocate the merge hotspot). Keep this simplicity-review todo focused on the UX progressive-disclosure work it already scopes.
