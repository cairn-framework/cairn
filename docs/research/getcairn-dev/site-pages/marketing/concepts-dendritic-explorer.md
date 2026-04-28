# Dendritic Explorer: Deep-Sea Survey AUV

**Source:** https://www.getcairn.dev/concepts/dendritic-explorer
**Captured:** 2026-04-28

## First-Principles System Design

**System Overview**
- 53 total nodes
- 39 active nodes
- 14 pruned nodes
- 3 levels of decomposition

### Mission Parameters

Deep-Sea Survey AUV operating at 3,000m depth with 72-hour endurance in fully autonomous mode.

### Physics Constraints

**Pressure Envelope**
30.4 MPa hydrostatic pressure represents the dominant structural constraint.

**Energy Budget**
Finite power with no resupply capability. Every watt becomes a critical design decision.

**Communication**
RF transmission is ineffective underwater; acoustic and timing systems are the only viable options.

**Navigation**
Without GPS or landmarks, the system relies on dead reckoning with corrections.

**Propulsion and Hydrodynamics**
"Cube-law physics. Speed costs power cubically."

### Mission Requirements

**Sensing Payload**
Survey-grade ocean floor mapping is the vehicle's primary purpose.

**Autonomy and Fault Management**
Isolated operation demands comprehensive self-sufficient decision-making capabilities.

## First-Principles Analysis

An autonomous undersea vehicle designed for deep-ocean bathymetric and geological surveys, operating at 3,000 meters continuously for 72 hours without human intervention. The three coupled constraints (hydrostatic pressure, finite energy, and operational isolation) form an irreducibly interconnected design space where optimizing one axis necessarily affects the others.

### Key Metrics

| Metric | Value |
|--------|-------|
| Operating Depth | 3,000 m |
| Hydrostatic Pressure | ~300 atm |
| Mission Endurance | 72 hours |
| Survey Speed | 2 to 3 knots |
| Displacement | ~800 kg |
| Length | ~4.5 m |
