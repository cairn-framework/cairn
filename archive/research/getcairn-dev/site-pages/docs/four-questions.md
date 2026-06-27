# The Four Questions Your Model Must Answer

**Source:** https://www.getcairn.dev/docs/four-questions
**Captured:** 2026-04-28

Every system model faces the same four fundamental questions. Cairn's analytical lenses are designed to make these questions answerable.

## Question 1: What's Underneath?

Systems depend on layers of prerequisite knowledge, technologies, and capabilities. Consider how an autonomous rover relies on LIDAR, which depends on laser physics and signal processing, which in turn depend on digital electronics.

The **Causality Lens** visualizes your system as a technological pyramid, from foundational knowledge through enabling technologies to your capstone system. Each layer is colored by maturity (TRL), revealing gaps and identifying which capabilities remain research problems.

## Question 2: What's Missing?

Models are inherently incomplete. Gaps exist when nodes lack behavior models, verification records, or connections to related subsystems.

The **Completeness Lens** computes three fidelity scores per node:

- Entity coverage (defined vs. implied nodes)
- Process coverage (modeled behaviors and states)
- Relationship coverage (interfaces and trace links)

Radar charts and heatmaps make gaps visible, with one-click options to address deficiencies.

## Question 3: What Does It Mean?

Systems transform inputs to outputs through processing chains, yet most models show structure without explaining this transformation narrative.

The **Narrative Lens** identifies the primary transformation chain from input to output and renders it as a readable diagram. AI generates descriptive phrases for connections, creating stakeholder-friendly explanations of system purpose.

## Question 4: How Did We Get Here?

Engineering involves decision-making among alternatives. Rejected design choices (the "dead paths") often disappear from institutional memory.

The **Dendritic Lens** preserves pruned alternatives as first-class nodes with explicit reasons, first principles, and dependencies. Decision history remains inspectable indefinitely.

## Reinforcing Interaction

These four lenses work together, each catching insights others miss, making models self-auditing.
