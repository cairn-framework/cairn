# Requirements

**Source:** https://www.getcairn.dev/docs/requirements
**Captured:** 2026-04-28

Requirements define what a node must do or how well it must perform. They're the bridge between stakeholder needs and engineering implementation.

## Requirement Types

Cairn supports six requirement types:

**Functional**: What the node must do.

"The navigation subsystem shall determine vehicle position to within 10cm accuracy."

**Performance**: How well it must do it.

"The system shall achieve 99.5% delivery success rate under normal operating conditions."

**Interface**: Constraints on connections with other nodes.

"The power subsystem shall provide 48V DC to the drivetrain via a standard automotive connector."

**Safety**: Hazards that must be mitigated.

"The battery pack shall include thermal runaway protection that triggers at 60C."

**Environmental**: Operating conditions the node must withstand.

"All external components shall operate in ambient temperatures from -20C to +50C."

**Constraint**: Design boundaries that limit solution space.

"Total system mass shall not exceed 45kg."

## Scoping to Nodes

Every requirement belongs to exactly one node. When you select a node and open the Requirements lens, you see only that node's requirements. The Completeness lens flags nodes with no requirements.

## AI Generation Patterns

The Command+K palette generates requirements scoped to your selected node:

"Add performance requirements for range and speed"

"Generate safety requirements for battery handling"

The AI proposes requirements as a ChangeSet. Review each one. Is the shall-statement testable? Is the threshold reasonable? Does it belong on this node or a child?

## Editing Requirements

Open the Inspector to edit a requirement directly. Title, description, type, priority, and acceptance criteria are all click-to-edit fields. Your change commits immediately. Use this for fixing wording, retyping a misclassified requirement, or sharpening the threshold on a draft.

Delete from the Inspector. The cascade preview lists every trace link and verification that would be removed alongside the requirement, so you can see the impact before you confirm.
