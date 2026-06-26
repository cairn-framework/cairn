# Information architecture

## What this is

The visible surface of the running app. Top-bar tabs, left-rail system tree, sidebar tools, status bar, and the chrome that frames every other workflow. This page is the structural inventory; the workflows that use the chrome live in [02-workflow-genesis.md](./02-workflow-genesis.md), [04-node-model.md](./04-node-model.md), [05-completeness-and-causality.md](./05-completeness-and-causality.md), and [06-command-palette.md](./06-command-palette.md).

## App-level chrome

### Top breadcrumb and identity

`Cairn / Offshore Survey USV-ROV` (**from screenshot 06**). The product name "Cairn" is the only branding that surfaces in-app; the project name is the active context.

### Top-right actions

`REVIEW 1` chip (likely 1 review pending; the same count surfaces in the status bar as `1 pending review`), `QUALITY 73` chip (a project-level quality score), `Command` button (opens the command palette, also bindable to `⌘K`), and a settings gear (**from screenshot 06**).

The persistent `Quality 73` chip is one of the more interesting pieces of chrome. The score is an aggregated project-level number that updates as nodes are added, properties are filled, and interfaces are wired. Its calculation is **unknown** (closed source). Per-node quality is exposed separately in the system tree as a percentage; see below.

### Status bar (footer)

Verbatim, left to right (**from screenshot 11**):

```
● Model loaded   Offshore Survey USV-ROV   v0.1.0
|  7 nodes   0 reqs   2 interfaces   0 states
   📎 1 file   ● Quality: 73   ◆ 1 pending review   ☾   ⌘K to command
```

Every primary count appears in the status bar: nodes, requirements, interfaces, state machines, attached files, quality score, pending review counter. The `1 pending review` chip is orange-bordered, signalling a pending action (**from screenshot 11**).

This is a strong UX pattern: the entire model state is summarised in one row, always visible, machine-counter style. Worth noting in [09-design-influence.md](./09-design-influence.md#status-bar-as-model-summary).

## The 10 top-nav tabs

Captured tab list (**from screenshot 06**, left to right):

1. **Overview** (active). Shows the system root, system budgets, subsystem grid, interfaces, visuals, attachments. See [04-node-model.md](./04-node-model.md).
2. **Brief.** **inferred** to show the system brief from genesis. Content not captured.
3. **Visuals.** **inferred** to show all attached visual assets (the Overview tab has a Visuals subsection with a `View all →` link, captured in screenshot 07).
4. **Requirements.** **inferred** to be the requirements lens. Content not captured directly, but a `6 REQUIREMENTS` count chip appears in screenshot 21 in what looks like the requirements review surface.
5. **Architecture.** **inferred** to be the architecture lens (decomposition, hierarchy view).
6. **Causality.** Active in screenshot 12. Shows the causal pyramid for a node, with dependencies, prerequisites, and interfaces. See [05-completeness-and-causality.md](./05-completeness-and-causality.md#causality-pyramid).
7. **Completeness.** Active in screenshots 13 and 14. Shows the three-axis fidelity radar. See [05-completeness-and-causality.md](./05-completeness-and-causality.md#three-axis-fidelity-radar).
8. **Narrative.** **inferred**. Content not captured. Likely an exportable prose narrative built from the model.
9. **Dendritic.** **inferred**. Content not captured. The term suggests a tree-shaped or branching view.
10. **Verification.** **inferred** to be the verification lens. Content not captured. Their docs describe verification as the closing loop of traceability.

### Tabs vs lenses: a discrepancy

The prior stronghold reports their docs advertising **twelve "lenses"** (**verified** via docs page `getcairn.dev/docs/reference/lenses` per prior scout). The running app surfaces **ten tabs**. The mismatch is not necessarily a contradiction: lenses may be the underlying conceptual primitive while tabs are a curated UI grouping, or two of the twelve lenses may be unsurfaced in the current build, or the docs may be slightly out of date. Carrying both numbers as data points; reconciling them requires either a new docs scrape or a hands-on confirmation from the user. **unknown** until then.

## Left rail: system tree

Visible in the captured project root view (**from screenshot 06** and zoomed in **screenshot 17**). Tree structure for the Offshore Survey project:

```
Offshore Survey USV-ROV          40%   yellow status dot
├── USV Platform                   0%   red dot
├── Power Generation & Distribution  11%   red dot   (currently selected)
├── ROV Vehicle                    11%   red dot
├── Launch & Recovery System        0%   red dot
├── Communications & Data Link     11%   red dot
└── Autonomy, Control & Mission Payload  11%   red dot
```

Notable structural features:

- **Per-node completeness percentages** roll up to a parent-level rollup (40% for the root). The mathematical aggregation is **unknown** but the parent's percentage is higher than any single child, suggesting a non-trivial weighted blend rather than a simple average.
- **Status-dot colour coding.** Red below some threshold, yellow above. Green is plausible at full completeness but not captured. **Inferred** thresholds; closed source.
- **Diamond glyph** to the left of each row marks node type or expand state. Closed glyph for unexpanded, open arrow for expanded.

## Left rail: tools panel

Below the system tree, a `TOOLS` heading with eight named tools in two columns (**from screenshot 06**):

| Column 1 | Column 2 |
|---|---|
| Quality | Simulation |
| History | Usage |
| Trace | Types |
| Assets | Settings |

What each likely does (all **inferred** from naming, none directly captured):

- **Quality.** Surfaces the per-node and per-system quality breakdown. The Completeness tab is one face of this; Quality may aggregate quality plus completeness plus pending reviews.
- **History.** Versioned model history. The status bar shows `v0.1.0` as a model version, suggesting the History tool exposes the version log.
- **Trace.** Traceability views (requirement to component to verification).
- **Assets.** All attached non-structural assets (images, documents). Mirrors the Visuals tab.
- **Simulation.** Likely a simulation/analysis surface (Monte Carlo, sea-state envelope, mass-budget flow). Their marketing has rendered demos.
- **Usage.** **unknown.** Plausibly a project-level usage view (where each node is referenced) or a billing/usage view.
- **Types.** **inferred** to be the entity-type registry editor. Their docs reference an "Entity Types" reference page.
- **Settings.** Project settings.

Below tools: `Export Project / Import Project / ← Projects` (project-level actions plus return-to-list).

## Right rail: contextual side panel

When a node is selected, a side panel slides in from the right showing that node's metadata. Captured forms:

- **Node detail.** Full property sheet, budgets, attachments, interfaces, action buttons. See [04-node-model.md](./04-node-model.md).
- **Causal position panel.** When the Causality tab is active. Shows prerequisites, dependencies, gaps below, plus a contextual book quote. (**from screenshot 12**.)
- **Completeness panel.** When the Completeness tab is active. Shows overall score, entity/process/relationship coverage breakdown, contextual book quote. (**from screenshots 15 and 16**.)

The same node can be inspected via different panels depending on the active tab. The side panel is **inferred** to be the same component re-skinned per tab.

## The "Inspect" affordance

The system root view includes a small `Inspect` button beneath the title (**from screenshot 06**). Inferred to open a deeper inspection surface for the active node. Not captured directly.

## Empty-state pattern

When a subsystem has no children, attachments, or interfaces, the detail page shows an empty-state CTA (**from screenshot 08**):

```
Start decomposing Launch & Recovery System
Break this node into subsystems, add requirements, or define interfaces.

  Open Command Palette
```

The orange `Open Command Palette` button is the single primary CTA. Most actions are funneled through the palette rather than the chrome. See [06-command-palette.md](./06-command-palette.md).

This is a notable UX choice. Empty states are a CTA-only mode that points at one action: the palette. Worth noting in [09-design-influence.md](./09-design-influence.md#empty-state-design).
