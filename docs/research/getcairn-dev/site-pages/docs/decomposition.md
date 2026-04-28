# System Decomposition

**Source:** https://www.getcairn.dev/docs/decomposition
**Captured:** 2026-04-28

Decomposition is the core activity in Cairn. You start with a system-level idea and progressively break it into smaller, more manageable pieces until each piece is well-understood.

## The Hierarchy

Every model is a tree. The hierarchy follows a natural pattern:

System -> Subsystem -> Assembly -> Part

A **System** is your top-level product, the thing you're designing. "Autonomous Delivery Rover" or "Portable Weather Station." There's exactly one system node per project.

**Subsystems** are major functional groups within the system. For a rover, these might be Power, Navigation, Drivetrain, Cargo, Communications, and Thermal Management. Each subsystem has a distinct responsibility.

**Assemblies** are collections of parts that work together as a unit. A "Battery Assembly" might contain cells, a battery management system, thermal insulation, and a housing. Assemblies are optional. Many models skip this level.

**Parts** are the leaves of the tree, individual components you'd find on a bill of materials. A motor, a sensor, a microcontroller, a connector.

## External Actors

Not everything in your model is inside your system boundary. **External** nodes represent systems that interact with yours but aren't part of it:

- A ground control station that sends commands
- A user who receives deliveries
- A power grid that provides charging
- A GPS satellite constellation

External nodes appear in your architecture and interface diagrams, but you don't decompose them further. They're outside your design authority.

## When to Decompose

Decompose when a node is too complex to reason about as a unit. Signs you need to go deeper:

- The node has multiple distinct functions ("it does X and Y and Z")
- Requirements are getting tangled (some apply to one part, some to another)
- You can't estimate properties (mass, power) without knowing what's inside
- Different team members would own different pieces

Don't decompose just because you can. Every level adds complexity. A node with one child is a sign you've gone too far.

## When to Stop

Stop when further decomposition doesn't add clarity. Signs you're at the right depth:

- The node maps to something you'd buy or build as a unit
- Requirements are clean and scoped
- You can estimate properties with reasonable confidence
- The node's behavior is understandable without knowing its internals

For early-phase work, stopping at subsystems or assemblies is often enough. You can always decompose further later.

## AI-Assisted Decomposition

The Command+K command palette is the primary way to ask AI for a decomposition. Select a node and type:

"Decompose into battery, charging, and power distribution"

"Break this down into sensor, processor, and actuator subsystems"

The AI generates a ChangeSet with proposed child nodes, initial property estimates, and sometimes interfaces between them. Review each operation before accepting.

Right-clicking a node in the tree or on the Architecture canvas exposes the same action under **Decompose with AI**, the same flow without opening the palette. To add a single named child by hand instead, right-click and choose **Add child**; the new node appears immediately, ready to rename and edit in the Inspector.
