# State Machines and Behavior

**Source:** https://www.getcairn.dev/docs/behavior
**Captured:** 2026-04-28

State machines model how a node behaves over time. What modes it can be in, what triggers transitions between modes, and what actions occur during those transitions.

## When to Model Behavior

Model behavior when:

- The node has distinct operating modes (startup, active, standby, shutdown)
- Mode transitions are triggered by specific events or conditions
- You need to analyze timing, sequencing, or fault response

Skip when the node is always in one mode or the behavior is trivial.

## States

A state represents a mode the node can be in. Each state has a name, type (initial, normal, or final), and description. Common patterns:

- **Startup sequence**: Initial -> Initializing -> Ready
- **Operational modes**: Ready <-> Active <-> Standby
- **Fault handling**: Any state -> Fault -> Recovery -> Ready
- **Lifecycle**: Active -> Shutdown -> Off (final)

## Transitions

A transition connects two states with:

- **Trigger**: What event causes it ("power_on", "fault_detected")
- **Guard**: Condition that must be true ("[battery > 20%]")
- **Action**: What happens during the transition ("initialize sensors")

## Editing States and Transitions

Click any state or transition in the Behavior lens to open it in the Inspector. The state's name, description, and transition trigger are click-to-edit; remaining fields are read-only display and become editable in a follow-up release.

Delete a state from the Inspector. The cascade preview lists every transition pointing to or from the state so you can see what else will go before you confirm.

## Right-Click Menus on the Behavior Lens

Right-click a state node, the canvas background, or a transition table row to open the contextual menu:

- **State node**: Edit, Add transition from here, Add transition to here, Delete, Delete with AI review. Final states omit "Add transition from here."
- **Canvas background**: Add state.
- **Transition table row**: Edit, Delete, Delete with AI review.

Keyboard Delete / Backspace on a selected state triggers cascade-aware delete. Transitions are deletable from the context menu only.

AI-driven menu items are disabled while a behavior simulation is active. Manual items (Edit, Delete) stay enabled.

## The Behavior Lens

The Behavior lens renders state machines as visual diagrams. Rounded rectangles for states, arrows for transitions, with labels showing trigger [guard] / action. States are draggable for layout.

## AI Generation

The Command+K palette generates state machines:

- "Add states for startup, active, and shutdown"
- "Generate a state machine for fault handling"

Watch for missing transitions (can you get stuck?), missing guards (can conflicting transitions both fire?), and overly fine-grained states.
