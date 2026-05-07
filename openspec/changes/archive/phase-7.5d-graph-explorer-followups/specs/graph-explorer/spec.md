# Graph Explorer Capability Spec

## ADDED Requirements

### Requirement: Prerequisite for / Enables widget

The node detail panel SHALL render a derived "Prerequisite for / Enables" widget that lists authority-chain neighbours of the selected node, sourced from blueprint edges via the existing `neighbourhood` query and filtered client-side by edge type. The widget is a derived UI surface, not a new artefact type. Decision-attached obligations are out of scope in this version.

#### Scenario: Widget renders authority-chain neighbours

- **GIVEN** a node with one inbound and two outbound authority-chain edges
- **WHEN** the user opens the node detail panel
- **THEN** the inbound neighbour appears under "Prerequisite for" and both outbound neighbours appear under "Enables"

### Requirement: Re-center on click viewport primitive

Clicking a node SHALL re-center the graph viewport on that node as an in-session viewport operation, with no URL parameter or stored view-state persistence in this version.

#### Scenario: Click re-centers the viewport

- **GIVEN** a node positioned away from the viewport center
- **WHEN** the user clicks the node
- **THEN** the layout center moves to the clicked node and the URL is unchanged

### Requirement: Verb labels render on dependency edges by default

Dependency edge labels SHALL render as visible verb annotations in the default graph view, alongside the selection-driven highlight already specified by the existing `Dependency edge labels` scenario. Label typography and color SHALL come from `docs/design-system/tokens.css`.

#### Scenario: Verb label visible without selection

- **GIVEN** a dependency edge with a label such as "reads user records"
- **WHEN** the graph explorer loads
- **THEN** the verb label renders before any node is selected, styled from design-system tokens

### Requirement: Inspector chrome consistency across artefact types

The node detail panel SHALL render every artefact type with consistent header and footer chrome around a type-specific middle slot. Chrome SHALL consume `docs/design-system/tokens.css` for spacing, radius, color, and typography, and SHALL NOT change `CairnResponse` shapes.

#### Scenario: Chrome is uniform across types

- **GIVEN** the panel rendering a contract, a decision, and a research artefact in turn
- **WHEN** each artefact section opens
- **THEN** header and footer chrome are identical across types and only the middle slot differs
