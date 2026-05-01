# Graph Explorer Capability Spec

## Purpose

The Graph Explorer provides an interactive web-based visualization of a Cairn project's structural graph, rendering nodes, ownership and dependency edges, and attached artefacts through a locally served interface. It enables users to browse the codebase architecture, inspect node details and their associated artefacts, and view integrity findings overlaid on the graph, consuming all data exclusively via the query API.

## Requirements

### Requirement: Serve the graph explorer via cairn ui

The `cairn ui` command SHALL start an embedded web server serving the graph explorer and open it in the default browser.

#### Scenario: Start the graph explorer

- **GIVEN** a valid cairn project with a parsed blueprint and artefacts
- **WHEN** the user runs `cairn ui`
- **THEN** an HTTP server starts on a local port
- **AND** the default browser opens to the graph explorer URL
- **AND** the terminal displays the URL and a message indicating the server is running

#### Scenario: Start on a custom port

- **GIVEN** a valid cairn project
- **WHEN** the user runs `cairn ui --port 4200`
- **THEN** the server starts on port 4200
- **AND** the browser opens to `http://localhost:4200`

#### Scenario: Start without opening browser

- **GIVEN** a valid cairn project
- **WHEN** the user runs `cairn ui --no-open`
- **THEN** the server starts and displays the URL
- **AND** the browser is not opened automatically

#### Scenario: Port conflict

- **GIVEN** port 3000 is already in use
- **WHEN** the user runs `cairn ui --port 3000`
- **THEN** the command exits with an error naming the port conflict

#### Scenario: Graceful shutdown

- **GIVEN** the server is running
- **WHEN** the user presses Ctrl+C
- **THEN** the server shuts down cleanly and the process exits with code 0

### Requirement: Render the structural graph

The graph view SHALL display all nodes and edges from the map with type-appropriate visual treatment.

#### Scenario: Display node hierarchy

- **GIVEN** a project with systems, containers, and modules
- **WHEN** the graph explorer loads
- **THEN** every node appears with its type badge, name, and stable ID
- **AND** ownership edges connect parent nodes to their children
- **AND** dependency edges are visually distinct from ownership edges (dashed vs solid)

#### Scenario: Dependency edge labels

- **GIVEN** a dependency edge with a label (e.g., "reads user records")
- **WHEN** the user selects either the source or target node
- **THEN** the edge highlights and its label becomes visible

#### Scenario: Large graph handling

- **GIVEN** a project with 200+ nodes
- **WHEN** the graph explorer loads
- **THEN** the initial view shows systems and containers only
- **AND** modules appear when the user expands a container
- **AND** no labels overlap and the layout completes within 2 seconds

### Requirement: Node detail panel with artefact drill-down

Clicking a node SHALL open a detail panel showing all artefact types attached to that node.

#### Scenario: Open node detail

- **GIVEN** the graph is displayed
- **WHEN** the user clicks a node
- **THEN** the detail panel appears showing the node's type, name, stable ID, and description
- **AND** all artefact types attached to the node are listed as expandable sections
- **AND** the first artefact section (contract, if present) is expanded by default

#### Scenario: Navigate artefact layers

- **GIVEN** the detail panel is open for a node with contract, decisions, and research artefacts
- **WHEN** the user clicks "Next" from the contract layer
- **THEN** the decisions layer expands and the contract layer collapses
- **AND** the layer counter updates (e.g., "2 / 3")

#### Scenario: Close node detail

- **GIVEN** the detail panel is open
- **WHEN** the user clicks the same node again or clicks a close button
- **THEN** the detail panel closes
- **AND** the node deselects and edge highlights clear

### Requirement: Integrity overlay

The graph explorer SHALL display `cairn lint` findings as visual indicators on affected nodes and edges.

#### Scenario: Structural error indicator

- **GIVEN** a node with a structural error (e.g., duplicate ID, broken pointer)
- **WHEN** the graph loads
- **THEN** the node displays a red severity badge
- **AND** clicking the badge shows the error detail in the node detail panel

#### Scenario: Interface contradiction indicator

- **GIVEN** a node where code diverges from its contract hash
- **WHEN** the graph loads
- **THEN** the node displays an amber severity badge

#### Scenario: Rationale tension indicator

- **GIVEN** a node with an orphan research artefact or missing source link
- **WHEN** the graph loads
- **THEN** the node displays a gray advisory badge

### Requirement: Query-consumer architecture

The UI SHALL consume cairn query output exclusively with no separate data path.

#### Scenario: All data flows through query API

- **GIVEN** the graph explorer is running
- **WHEN** any data is displayed (nodes, edges, artefacts, findings)
- **THEN** that data was obtained via the query bridge API (`/api/*`)
- **AND** the query bridge delegates to the same library functions that back the CLI

#### Scenario: Graph endpoint uses explorer graph response

- **GIVEN** the graph explorer loads its canvas data
- **WHEN** it requests `GET /api/graph`
- **THEN** the bridge delegates to the library `graph()` query response
- **AND** the response contains typed `nodes` and `edges`
- **AND** each edge declares whether it is an `ownership` or `dependency` edge
- **AND** this explorer-specific response shape is versioned through `GET /api/meta`

#### Scenario: Forward-compatible artefact rendering

- **GIVEN** a future phase adds a new artefact type not known to the UI
- **WHEN** the query returns this artefact for a node
- **THEN** the UI renders it using the generic artefact template (title + frontmatter + body)
- **AND** no error is shown

#### Scenario: Schema version mismatch

- **GIVEN** the query API reports a schema_version newer than the UI was built for
- **WHEN** the UI loads
- **THEN** a non-blocking warning banner appears indicating a version mismatch
- **AND** the UI continues to function using forward-compatible rendering

### Requirement: UI Maintenance Contract

Phase 2.5 SHALL document the UI Maintenance Contract governing cross-phase UI compatibility.

#### Scenario: Phase with altered query response

- **GIVEN** a future phase modifies the shape of a `CairnResponse` struct
- **WHEN** that phase's acceptance criteria are written
- **THEN** the criteria include a UI compatibility note confirming existing rendering is unaffected or specifying the required UI change

#### Scenario: Phase 3 temporal addendum

- **GIVEN** Phase 3 introduces `cairn changes` and `cairn show` with proposed-vs-current semantics
- **WHEN** Phase 3's spec is finalised
- **THEN** Phase 3 includes a UI deliverable section specifying temporal navigation requirements (split view or overlay for proposed changes)

#### Scenario: Phase 7 transport addendum

- **GIVEN** Phase 7 introduces the MCP server as the preferred query transport
- **WHEN** Phase 7's spec is finalised
- **THEN** Phase 7 includes a UI deliverable section specifying the transport adapter switch from CLI-exec to MCP
