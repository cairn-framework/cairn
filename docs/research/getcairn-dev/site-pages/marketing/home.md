# Cairn. From rough idea to structured model

**Source:** https://www.getcairn.dev/
**Captured:** 2026-04-28

## AI-native model-based systems engineering

Chat vanishes. The model stays.

Every other AI tool hands you a transcript. When the window closes, the structure goes with it. Cairn keeps it, as an inspectable, editable engineering model.

### Generic AI chat vs. Cairn

**Generic AI chat** produces a conversation transcript that expires when the window closes.

**Cairn** persists a structured system model you own and edit, containing:
- System tree (26 nodes shown example)
- 18 requirements
- 7 interfaces
- 50% verified

## Describe

You type rough. Cairn picks up structure.

Describe what you're building in plain language. The AI synthesizes:
- Subsystems and components
- Requirements
- Interfaces

The system clarifies ambiguities and makes reasonable assumptions while building the starting tree.

## Decompose

A brief becomes structure.

The AI interview surfaces subsystems, components, and interfaces as explicit, editable nodes. Each with an ID that survives refinement.

Example system tree for Autonomous Delivery Rover:
- Power subsystem (Battery Pack, Power Distribution, Thermal Management)
- Drive subsystem (Hub Motors, Motor Controllers, Suspension)
- Sensing subsystem (LiDAR, Stereo Camera, IMU, Wheel Encoders)
- Compute subsystem (Main SBC, Safety MCU)
- Payload subsystem (Cargo Bay, Locking Mechanism)
- Comms subsystem (LTE Modem, GNSS Receiver)

## Inspect

One model. Twelve lenses.

Every view is the same underlying model, filtered differently:
- Overview
- Brief
- Visuals
- Requirements
- Architecture
- Causality
- Completeness
- Narrative
- Dendritic
- Behavior
- Verification
- Operational

Example Requirements lens shows 18 total requirements with verification status.

## Depth

Real engineering work. Not just text.

2D concept renders, 3D mesh assets, and Monte Carlo simulation are all tied to the structured model. Every artifact stays traceable back to a component, requirement, or decision.

Artifacts include:
- 2D renders (AI-generated profile views)
- 3D meshes (live, interactive)
- Simulation (Monte Carlo analysis)

## Refine

AI proposes. You commit.

Structural changes arrive as reviewable ChangeSets, one decision at a time. Accept and the model updates instantly. Skip and nothing changes. The model is never edited out from under you.

Example pending changes:
- Add thermal probe
- Update SOC accuracy requirement
- Formalize CAN bus interface
- Promote torque-vectoring to behavior
- Add cargo locking requirement

## Trace

Every requirement, traced to its verification.

Pick any requirement. Cairn shows:
- Which component implements it
- How it's verified
- What the results say

Example trace for REQ-001 (28.4V bus plus or minus 5%):
- Implemented by Power Distribution (C.01.02)
- Verified by HIL bench log #4421
- Results: 28.38 to 28.42 V over 30 min

## Thesis

The model is the artifact, not the conversation.

AI should contribute structure to a persistent, inspectable engineering model. Not generate disposable text that vanishes when the chat window closes.

### What stays:
- System tree
- Requirements
- Interfaces
- Verifications
- Change history
- Evidence

### What vanishes:
- Chat transcripts
- Throwaway markdown
- Context window state
- Prompt scaffolding

### Core principles:

- **The model stays visible**. Every node, interface, and requirement is explicit and inspectable.
- **Built for serious work**. Twelve lenses, 2D plus 3D plus simulation, full traceability.
- **Immediate and self-contained**. Direct and fast, runs locally, no setup ceremony.

## Your turn

What are you building?

Describe the system in plain English. Rough is fine. Cairn refines it from here.

Command+Enter to begin. No account required to try.

Or pick one of the templates:
- Rover
- CubeSat
- Battery enclosure
- Drone swarm

## Internal links

- [Home](/)
- [Story](/story)
- [Demo](/demo)
- [Concepts](/concepts)
- [Blog](/blog)
- [Docs](/docs)
- [Pricing](/pricing)
- [Sign in](/signin)
- [About](/about)
- [Privacy](/privacy)
- [Terms](/terms)
