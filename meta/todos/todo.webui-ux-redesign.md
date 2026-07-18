---
node: cairn.ui
status: open
created: 2026-07-18
---

# Webui UX Redesign

Redesign the webui UX-first with the design-studio loop: derive what
the UI should be from what cairn does and how it is used, codify the
canonical design system, and implement it as modular components.
Authority: `dec.webui-ux-first-redesign` (2026-07-18), refining
`dec.webui-design-direction`.

## Problem

The current webui was refined in place (state vocabulary, counters,
legend, guidance: PRs #419, #421, #422) but its interaction
architecture was never designed from cairn's purpose. The canvas
renders every node as a large card in one vertical column, roughly 5
of 25 dogfood nodes visible per 1440x900 screen: the map is a list
behind endless scroll. The greenfield mocks (branch
`design-studio-greenfield`, worktree `cairn-ds-greenfield`, under
`harness-output/mocks/`) prove a bounded whole-graph workspace is
achievable at roughly 5x the density. Owner direction (2026-07-18):
design from what it should be, not from what exists.

## Task

Four stages, run via the design-studio loop
(github.com/george-rd/design-studio). The greenfield lane is the
method: the UX phase must not anchor on the current UI.

### 1. UX definition from cairn's jobs

Derive the information architecture from what cairn is supposed to do
and how it could be used, not from any existing screen:

- Enumerate the jobs: orient in an unfamiliar codebase, inspect a
  node's depth (files, symbols, contracts), trace the two-chain
  topology (provenance evidence into a decision hinge, authority rules
  out of it), review findings and drift, follow active changes and
  the backlog.
- Choose the layout and IA that serve those jobs. The two mocks and
  the current canvas are scored evidence, not the spec; no structure
  is prescribed in advance (`dec.webui-ux-first-redesign`).
- Two settled outcomes constrain whatever emerges: bounded desktop
  workspace (only designated panels scroll internally), and the full
  dogfood graph legible in one 1440x900 viewport without scrolling.
- Deliverable: a UX brief plus static mocks rendering the real frozen
  fixtures, evaluated in the loop before any implementation.

### 2. Codify the design system

Rebuild `docs/design-system/` as the canonical system in the
Calibrated Instrument direction, seeded from the exploration branch's
codified output (`harness-output/design-dna.md`, `tokens.css`, the
skill under `harness-output/design-system/skill/`):

- `tokens.css`: one authoritative set reconciling current and
  greenfield tokens; no fork.
- `components.css`: a component class for every UI region the UX brief
  defines, each demonstrated in the live reference `index.html`.
- `README.md` and the live reference updated in the same commit as
  every token/component change, per the design-system contract.
- Gates keep passing throughout (`scripts/check-design-tokens.sh`,
  biome); the landing page and other token consumers stay conformant
  with the reconciled set.
- Resolve the stale `docs/design-system/NEXT_SESSION.md` handoff: fold
  its still-valid items into the UX brief or discard them explicitly,
  then retire the file.

### 3. Implement as modular components

Rebuild `src/ui_assets/` to the UX brief, component-based as an
outcome: components with cohesive responsibilities, explicit data and
event interfaces, and independently testable rendering, styled by
reusable design-system primitives. The component design chooses the
boundaries; do not force one module per UI region or mirror the
existing 9-module split.

Preserve capabilities, not today's controls. The redesign must keep
every job the current webui serves: select a node and navigate its
edges, bring the selection into view, read node state at a glance
(the adopted state vocabulary stays as design language), search and
query the graph, review findings, inspect the blueprint source,
follow active changes, and use the UI on narrow screens. The UX phase
may replace or discard any of today's specific patterns (command
palette, drawer, modal, legend placement, topbar arrangement) when
its job analysis supports a better interaction; no landed control is
sacred.

Do not foreclose the future two-way exploration (webui feedback/notes
flowing to the harness via the `cairn feedback` / `.cairn/feedback.md`
seam): keep rendering and data access seams clean enough that a write
path could be added later. Building that path is out of scope.

### 4. Verify with the eval loop

Run the design-studio evaluate lane (or the visual harness under
`harness/`) against the implemented webui, not just mocks: adversarial
interaction gate (selection, edge navigation, search, toggles), zero
console errors, no page-frame overflow at 390px and 683px.

## Downstream, out of scope here

Public assets bake in the webui's look and go stale after this lands:
README (`webui.gif`, `design-system.png`, `landing-full.png`), landing
hero (`webui.mp4`, poster and og:image `webui-graph.png`),
`docs/assets/screenshots/`, visual harness baselines. Tracked in
`todo.ui-asset-refresh`, which this todo unblocks.

## Acceptance

- A UX brief exists, derived from cairn's jobs, with evaluated mocks;
  the chosen IA is justified against those jobs, not against the
  current UI.
- `docs/design-system/` presents one reconciled canonical system:
  tokens, components, fonts, live reference, README, all consistent;
  token gate and biome pass; landing stays conformant.
- Bounded workspace: no endless page scroll; only designated panels
  scroll internally, verified by headless screenshot.
- The full dogfood graph (25 nodes) is legible in one 1440x900
  viewport without canvas scrolling.
- Implementation is component-based: cohesive responsibilities,
  explicit data/event interfaces, independently testable rendering,
  reusable design-system primitives; boundaries chosen by the design.
- Every current capability survives in some form: node selection and
  edge navigation, selection brought into view, at-a-glance state,
  graph search/query, findings review, blueprint inspection, change
  visibility, narrow-screen use. The specific controls may differ.
- Visual harness gates pass; adversarial interaction gate clean.
- Nothing in the implementation forecloses a later two-way feedback
  seam.

dec:dec.webui-ux-first-redesign
dec:dec.webui-design-direction
