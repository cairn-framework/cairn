# Cairn webui UX brief

## Product job

Cairn is a read-only orientation instrument for an unfamiliar codebase. The workspace must let a maintainer move from a truthful map to a useful explanation without losing the map. The visual direction is Calibrated Instrument: warm graphite chassis, chalk ink, one teal signal role, and amber reserved for drift. The graph fixture is the source of every visible node, edge, count, finding, and artefact claim in the mock.

## Jobs and information architecture

### 1. Orient in an unfamiliar codebase

**Need:** establish scale, reconciliation health, ownership shape, and the main dependency neighbourhood in one glance.

**IA choice:** the opening screen is a bounded instrument workspace, not a sequence of pages. A status bezel gives node, dependency, finding, and interface-hash readouts. The workbench puts the whole graph in the dominant left canvas, with ownership brackets and labelled dependency links. The right evidence rail stays visible so orientation does not require leaving the map. A compact findings channel anchors the health story at the bottom.

### 2. Inspect a node's depth

**Need:** answer what a node owns, which files and symbols it represents, which contracts apply, and which edges enter or leave it.

**IA choice:** selecting a node focuses it without hiding neighbours. The evidence rail becomes the selected node's depth readout: identity and state first, then paths and file counts, then labelled IN and OUT edge lists. A node's exact id and paths use mono type so machine identity is not confused with its human name. The rail has an internal scroll region; the page frame never grows.

### 3. Trace provenance evidence into a decision hinge and authority rules out of it

**Need:** understand why a node exists, which accepted decision shaped it, and which authority or contract constrains implementation.

**IA choice:** the evidence rail includes a lineage plate directly beneath node facts. It renders three linked stages in one reading order: evidence, decision hinge, authority. Selecting a stage opens its artefact text in the same rail rather than a modal. The blueprint tab sits beside lineage as a source inspection mode. This keeps the reason for a node adjacent to its graph identity.

### 4. Review findings and drift

**Need:** locate broken or uncertain areas, distinguish clean state from alarm state, and know what needs attention next.

**IA choice:** the status bezel announces reconciliation state and findings count. The bottom channel is a fixed findings and drift surface with an internal list. It can switch between Findings, Drift, Changes, and Backlog without replacing the map. The adopted vocabulary is stable: `synced` is calm signal, `ghost` is a dashed outline for declared but absent structure, `orphaned` is an amber tilted marker for discovered but undeclared structure, and `drift` is the amber alarm family. The frozen fixture contains synced nodes only, so the legend documents the other treatments without fabricating node records.

### 5. Follow active changes and the backlog

**Need:** see whether the map is changing and choose the next area to inspect.

**IA choice:** Changes and Backlog are channels in the bottom surface, not separate navigation destinations. Their empty state states the fact and the next available read-only action. A selected change or todo can point back to a node in the same graph. The mock keeps these channels visible even when the frozen snapshot has no active change record.

## Screen inventory

The design has one primary screen with named modes. Modes preserve the graph and swap only the evidence content.

1. **Instrument overview:** status bezel, query rail, whole-graph canvas, selected-node evidence rail, and findings channel.
2. **Node depth mode:** the graph remains visible while the evidence rail shows identity, state, paths, file and symbol counts, contracts, and IN or OUT neighbours.
3. **Lineage mode:** the evidence rail shows evidence, decision hinge, and authority stages with artefact title, id, date, status, and a short body excerpt.
4. **Blueprint source mode:** the evidence rail shows the frozen blueprint path and syntax-highlighted source excerpt with a copyable mono presentation.
5. **Findings and drift mode:** the bottom channel expands internally to show finding severity, code, affected node, and remediation context. The canvas remains available for selecting the affected node.
6. **Changes and Backlog mode:** the bottom channel switches to active changes or open work. An item can select its referenced node; an empty snapshot is explicit.
7. **Narrow instrument mode:** at 390px the same modes become stacked internal regions. The graph remains a compact overview first, then the selected evidence and channel content.

## Region layout at 1440 x 900

The application frame is exactly viewport-bound with `height: 100dvh` and `overflow: hidden`. No page-level scrolling is permitted. Only the evidence rail, source plate, graph canvas detail layer, and bottom channel list may scroll internally.

- **Bezel, 56px high:** Cairn identity, reconciliation annunciator, and four tabular counters. This answers scale and health before interaction.
- **Query rail, 42px high:** graph search, query syntax hint, state and kind filters, and a bring-selection-into-view action. It is one grouped command surface, not scattered controls.
- **Workbench, 686px high:** a 2:1 split. The left graph canvas is about 936px wide and the right evidence rail about 456px wide. Both have their own bounded wells.
  - **Graph canvas:** a labelled canvas heading, ownership brackets, a compact whole-graph node layout, and dependency links. Every graph node is a selectable instrument module. The static overview places all 24 frozen nodes in the first viewport.
  - **Evidence rail:** selected node identity and state, depth facts, IN and OUT edge navigation, lineage stages, and Blueprint or Source mode. The rail body is the designated scroll region.
- **Findings channel, 116px high:** a fixed lower band with four channel tabs and a compact list or explicit empty state. It is the only bottom surface and remains discoverable while graph work continues.

The graph is intentionally the largest region because orientation and topology are the first job. The evidence rail is wide enough for paths and artefact titles because node depth and provenance are the second and third jobs. The header and channel are compact readouts, not dashboard hero metrics.

## Interaction model

### Selection and focus

- Clicking or pressing Enter on a node selects it and updates the evidence rail.
- Selection changes the teal signal keel and a clear outline. Non-neighbours dim to readable chalk, never to hidden or disabled.
- The selected node remains in the DOM and in the viewport. `Bring into view` centres it within the graph well when a future larger graph needs internal navigation.
- Escape clears the query or returns the evidence rail to the overview without hiding the graph.
- Focus rings use the signal colour and are visible for keyboard navigation.

### Edge navigation

- Each selected node shows labelled `IN` and `OUT` edge rows in the evidence rail.
- Activating an edge row selects its destination or source node and records the direction label in the rail heading.
- Ownership is visually bracketed and dependency links are labelled in the edge list. Direction is never inferred from colour alone.

### Search and query

- The query rail accepts an id, name, path fragment, or state token.
- Search updates the visible match count and marks matching nodes. Enter selects the first match. Arrow keys cycle matches. Clear restores the full map.
- Kind and state controls are filters on the graph data, not navigation away from it. The state legend explains the result when a filter returns zero records.

### Evidence modes

- The rail's `Depth`, `Lineage`, and `Blueprint` tabs are modes over the current selection. Switching tabs does not clear selection.
- The bottom channel's `Findings`, `Drift`, `Changes`, and `Backlog` tabs are independent of the rail. Selecting an item brings its node into focus when a node id is available.
- This is read-only. There is no feedback editor or mutation affordance in this redesign.

## State vocabulary

| State | Visual treatment | Meaning and job support |
| --- | --- | --- |
| synced | solid teal keel, calm chalk body | Declared and real. Supports confident orientation. |
| ghost | dashed outline and hollow keel | Declared but absent. Signals an authority or implementation gap. |
| orphaned | amber keel, tilted marker, amber label underline | Real structure without a declaration. Directs drift review. |
| drift | amber annunciator and warning treatment | A family label for attention-required reconciliation or finding state. |

Colour is paired with shape, label, and line treatment. The fixture only supplies `synced`; ghost and orphaned are legend vocabulary until real records exist.

## Narrow-screen strategy at 390px

The frame remains viewport-bound and never creates horizontal page overflow. The order changes to preserve the jobs:

1. A 48px compact bezel keeps Cairn, annunciator, and the primary count visible.
2. A 36px query rail keeps search and one filter button available.
3. A compact graph overview uses four narrow columns and six rows. All frozen nodes remain visible in the first graph region. Node names may use a short label with the full id in a title and the evidence rail.
4. The selected evidence rail follows the graph as a bounded internal panel. Its depth, lineage, and blueprint content scroll inside the panel.
5. The findings, changes, and backlog channel is a final bounded strip with one active tab and a count.

The graph is not reduced to a hidden drawer because orientation still matters. Details are progressively disclosed in the rail because node depth and provenance require reading space. Touch targets remain at least 32px high, and keyboard focus remains supported.

## Stage 3 component inventory

Each component owns one cohesive responsibility, receives explicit data, and emits named events. Components do not reach into one another's DOM.

### `InstrumentShell`

- **Responsibility:** viewport-bound frame, region sizing, mode coordination, and responsive ordering.
- **Data:** `viewport`, `activeRailMode`, `activeChannel`, `selection`, `query`.
- **Events:** `onRailModeChange(mode)`, `onChannelChange(channel)`, `onSelectionChange(nodeId)`, `onQueryChange(query)`.

### `StatusBezel`

- **Responsibility:** identity, reconciliation annunciator, and tabular status counters.
- **Data:** `projectName`, `status`, `stateCounts`.
- **Events:** `onStatusDetailsRequest()`.

### `QueryRail`

- **Responsibility:** graph search, query parsing affordance, kind or state filters, and selection focus action.
- **Data:** `nodes`, `query`, `filters`, `selectionId`.
- **Events:** `onQueryChange(query)`, `onFilterChange(filters)`, `onBringIntoView(nodeId)`, `onClearQuery()`.

### `GraphCanvas`

- **Responsibility:** whole-graph layout, ownership brackets, dependency links, node focus, keyboard navigation, and viewport focus.
- **Data:** `nodes`, `edges`, `selectionId`, `matches`, `filters`, `stateVocabulary`.
- **Events:** `onNodeSelect(nodeId)`, `onEdgeRequest(edgeId)`, `onBringIntoView(nodeId)`, `onCanvasKeyNavigate(direction)`.

### `NodeModule`

- **Responsibility:** one node's readable identity and state treatment.
- **Data:** `node`, `isSelected`, `isMatch`, `isNeighbour`, `stateVocabulary`.
- **Events:** `onSelect(nodeId)`, `onFocus(nodeId)`.

### `EvidenceRail`

- **Responsibility:** selected node depth, edge navigation, and evidence mode host.
- **Data:** `selection`, `nodes`, `edges`, `depth`, `lineage`, `blueprint`.
- **Events:** `onNeighbourSelect(nodeId, edgeId)`, `onEvidenceModeChange(mode)`, `onSourceOpen(path)`.

### `NodeDepthPlate`

- **Responsibility:** identity, state, paths, files, symbols, contracts, and labelled IN or OUT rows.
- **Data:** `node`, `incomingEdges`, `outgoingEdges`, `contracts`.
- **Events:** `onEdgeSelect(edgeId)`, `onPathOpen(path)`.

### `LineagePlate`

- **Responsibility:** evidence to decision hinge to authority chain, including artefact metadata.
- **Data:** `evidenceArtefacts`, `decisionArtefacts`, `authorityArtefacts`.
- **Events:** `onArtefactSelect(artefactId)`, `onBlueprintRequest()`.

### `BlueprintPlate`

- **Responsibility:** read-only blueprint source inspection for the selected node or project.
- **Data:** `sourcePath`, `sourceText`, `highlightedRanges`, `selectionId`.
- **Events:** `onSourceCopyRequest(path)`, `onLineageRequest(nodeId)`.

### `ChannelBar`

- **Responsibility:** findings, drift, active changes, and backlog review in a fixed bottom band.
- **Data:** `activeChannel`, `findings`, `driftItems`, `changes`, `backlog`.
- **Events:** `onChannelChange(channel)`, `onItemSelect(itemId, nodeId)`, `onChannelExpand()`.

### `StateLegend`

- **Responsibility:** explain state treatment with non-colour shape cues.
- **Data:** `stateVocabulary`, `availableStates`.
- **Events:** `onStateFilterRequest(state)`.

## Mock data contract

`studio/mocks/data.js` is generated from the frozen API fixture files. The graph supplies 24 nodes and 27 dependency edges plus ownership edges, with all node ids, names, descriptions, paths, files, and states intact. The status fixture supplies the reconciliation counters and interface hash. The blueprint fixture supplies the source inspection text. Node decision payloads supply the real Brownfield decision title, id, date, status, and body excerpt. The lint fixture supplies the findings state. Empty changes and backlog states are labelled as snapshot facts, never as fabricated records.

## Acceptance checks for this stage

- At 1440x900 the document frame has no page-level scroll, and every frozen node is present in the DOM and inside the viewport.
- At 390x844 the page frame has no horizontal overflow, and every frozen node remains inside the viewport in the compact overview.
- The mock has no network dependency and no console errors.
- All visible labels use British spelling and no em-dashes. No pure black or pure white values are introduced.
