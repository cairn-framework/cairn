# Interfaces and Signals

**Source:** https://www.getcairn.dev/docs/interfaces
**Captured:** 2026-04-28

Interfaces define how nodes connect and communicate. They're the contracts between subsystems, what flows across boundaries and under what conditions.

## What an Interface Represents

An interface connects exactly two nodes. It represents a physical connection, a data exchange, a power transfer, or a material flow. Interfaces are bidirectional containers, the direction of individual flows is defined by the signals inside them.

## Signals

A signal is a single flow within an interface:

- **Name**: What's being transferred ("position_data", "motor_power")
- **Type**: data, power, physical, or thermal
- **Direction**: source -> target, target -> source, or bidirectional
- **Unit**: Engineering unit (V, A, Mbps, L/min)

An interface might carry multiple signals:

```
Interface: Power Subsystem <-> Drivetrain
- motor_power (power, 48V, ->)
- motor_current (power, 0-120A, ->)
- fault_status (data, boolean, <-)
- temperature (data, C, <-)
```

## Creating Interfaces

**Architecture lens**: Drag from one node's port to another.

**Command+K command**:

- "Add interfaces to the power subsystem"
- "Create a data interface between navigation and control"

**Decomposition side effect**: When AI decomposes a node, it often proposes interfaces between the new children.

## Architecture Visualization

The Architecture lens renders interfaces as edges between nodes. Hover to see signal summaries. Click to inspect full definitions. Line thickness reflects signal count; color reflects signal type.
