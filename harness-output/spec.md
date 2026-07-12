# Spec — Cairn Graph Explorer (greenfield design run)

## Purpose and audience

A read-only orientation instrument for a codebase's architecture graph. It answers: what exists, how is it connected, what state is it in, and why is it shaped this way. Users are a Rust monorepo maintainer checking drift and health, and contributors (human and AI) orienting before a change. They read code daily, value density over whitespace-as-decoration, and trust instruments over dashboards.

## Aesthetic direction and creative tension

Two candidate poles, each a full direction with its own creative tension:

1. **Geological / cairn-stones** — "A survey field-notebook WITH the calm of a museum plate." The product's own metaphor made literal: stacked stones, strata bands, survey-marker vocabulary (benchmarks, triangulation, elevation ticks). Warm mineral neutrals, engraved-plate typography, sediment layering as information hierarchy. Risk to defeat: theme-park kitsch.
2. **Refined instrument / technical** — "A precision instrument WITH warmth." Flight-deck restraint: cool graphite surfaces, one calibrated accent, tabular numerals, hairline rules, dense readouts. Risk to defeat: generic dev-tool dark dashboard (the AI slop pole).

## Feature set

Core (must render, from real frozen data):
- Graph canvas: 25 nodes (system, container, module), 27 dependency edges plus ownership containment; pan or a legible static layout; node states (synced, ghost, orphaned) distinguishable.
- Inspector panel: selected node identity, kind, state, paths, file and symbol counts, dependency edges in and out with descriptions, linked decisions and rationale with dates and status.
- Status header: node and edge counts, findings count, interface hash, reconciliation state summary.
- Findings surface: the lint findings list (renders the single info finding from map.json; empty state from copy.json when filtered clean).

Distinctive:
- The reconciliation story is the hero: "declared vs real" must read at a glance.
- Decision lineage visible from the node, not buried in a modal.

## Technical stack

Static HTML mocks, one file per direction plus a shared inlined data script. No build step, no network, openable via file://. Vanilla JS + SVG for the graph canvas.

## Expected zones

1. Status header (top strip)
2. Graph canvas (dominant region)
3. Inspector panel (right column)
4. Command surface (search or filter affordance, keyboard hints, view controls)
5. Findings strip or drawer

## Reference points

Survey monuments and geological survey plates; Swiss cartography (direction 1). Dieter Rams instrumentation, aircraft EFIS readouts, Berg blog-era hardware UIs (direction 2).

## Anti-goals (aggressive)

- No glassmorphism, no purple-cyan gradients, no gradient text, no glow borders.
- No identical card grids; no cards nested in cards.
- No pure #000 or #fff anywhere.
- No Inter-by-default typography voice; type must be chosen, not defaulted.
- No dashboard hero-metric layout (big number, small label, accent swoosh).
- No toy data: every visible node, edge, count and finding comes from the frozen fixtures.
- Direction 1 must not become a parchment theme park; direction 2 must not become another dark dev dashboard.
