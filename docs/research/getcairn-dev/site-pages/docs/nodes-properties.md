# Nodes and Properties

**Source:** https://www.getcairn.dev/docs/nodes-properties
**Captured:** 2026-04-28

Nodes serve as fundamental building blocks in Cairn's modeling system, with each node representing a discrete component at a particular level of system abstraction.

## Node Components

A node contains several key elements:

- **Name**: Brief identifier for the node
- **Type**: Categorization such as system, subsystem, assembly, part, or external
- **Description**: Natural language explanation of the node's purpose and function
- **Parent**: The containing node (absent for root nodes)
- **Properties**: Engineering parameters including mass, power consumption, and cost

## Editing Capabilities

Node fields are directly editable through the Inspector panel. Users can modify titles, descriptions, types, and properties with immediate commitment, no review process required.

Right-click options on nodes enable renaming, child creation, AI-assisted decomposition, and deletion. The Architecture canvas provides a "Connect to" feature for interface wiring. Deletion operations display a cascade preview showing affected requirements, interfaces, states, and trace links before confirmation.

## Properties System

Properties function as key-value pairs with engineering semantics:

| Property | Unit | Category | Typical Use |
|----------|------|----------|-------------|
| mass | kg | physical | Weight budgets |
| power_nominal | W | electrical | Power budgets |
| power_peak | W | electrical | Worst-case analysis |
| cost | USD | economic | Cost rollups |
| mtbf | hours | reliability | Reliability analysis |
| operating_temp_min | C | thermal | Environmental limits |
| operating_temp_max | C | thermal | Environmental limits |

## Automatic Rollups and AI Features

Selected properties automatically aggregate from child nodes to parents. The Overview lens displays budget comparisons between allocated and actual values. An AI suggestion feature proposes contextually relevant properties based on node characteristics.
