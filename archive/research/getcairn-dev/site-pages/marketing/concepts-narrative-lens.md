# System Narrative

**Source:** https://www.getcairn.dev/concepts/narrative-lens
**Captured:** 2026-04-28

The systemigram reads your model as a story. Nodes become noun phrases, interfaces become verb phrases. The diagonal **mainstay** tells the central purpose; branches show supporting flows.

- Mainstay (primary narrative)
- Branch flow (supporting)
- Cross-connection

"The **Solar Array** generates power, which the **Power Distribution** delivers to the **Nav Computer**, enabling **path planning** that commands **Hub Motors** to traverse terrain, while the **Comms Processor** streams telemetry to the **Ground Station**."

### The Mainstay

The primary narrative arc reads from top-left to bottom-right: Solar Array generates power, which Power Distribution delivers to the Nav Computer, which commands Hub Motors to traverse terrain. This is the core energy-to-motion transformation chain, the reason the rover exists.

### Perception Branch

Three sensor inputs converge on the Nav Computer: LiDAR scans terrain for obstacles, GPS localizes position globally, and IMU stabilizes attitude estimation. This sensor fusion enables the decision engine to plan paths through unknown environments.

### Communication Branch

The Nav Computer reports status to the Comms Processor, which streams telemetry to the Ground Station (external). This closes the human-in-the-loop: operators can monitor and override the autonomous system remotely.

### Cross-Connections

Thermal Management appears as a support system. It cools the Battery Pack but doesn't participate in the primary transformation chain. Power Distribution also feeds Hub Motors directly (dashed line), showing that the energy bus crosses multiple narrative branches.

Systemigram methodology: Boardman (1994), Blair, Boardman and Sauser (2007). "The systemigram combines narrative discussion with a one-page diagram modeling narrative as nodes and links." McDermott, Ch.22 (Loper 2015).
