# Lens Workflows

**Source:** https://www.getcairn.dev/docs/lens-workflows
**Captured:** 2026-04-28

Each lens in Cairn shows your model from a different angle. This section explains when to reach for each lens and what you'll accomplish there.

## Overview Lens

Your dashboard. A high-level snapshot of project health.

**When to use it:**

- Starting a work session (what needs attention?)
- Preparing for a review (are we on track?)
- Checking budget status (mass, power, cost)

**What you'll do:**

- Scan budget bars for overruns (red) or margin (green)
- Review recent AI activity and pending ChangeSets
- Jump to problem areas via the node list

The Overview lens doesn't edit your model. It orients you before diving into detail work.

## Architecture Lens

Shows interfaces. How nodes connect and communicate.

**When to use it:**

- Defining interfaces between subsystems
- Reviewing signal flows (data, power, physical)
- Checking for missing or orphan connections

**What you'll do:**

- Drag between node ports to create interfaces
- Right-click any node for the contextual menu (add, connect, decompose, delete, all with cascade-aware safety)
- Click interfaces to view/edit signals
- Use Command+K: "Add interfaces to this subsystem"

Tip: Create interfaces early. They clarify boundaries before you've detailed the internals.

## Requirements Lens

Manages requirements scoped to the selected node and its children.

**When to use it:**

- Adding requirements to a subsystem
- Reviewing requirement coverage
- Allocating parent requirements to children

**What you'll do:**

- Filter by requirement type (functional, safety, etc.)
- Use Command+K: "Generate safety requirements for battery handling"
- Review verification status column

Requirements are scoped. Select the system node to see all, or a subsystem to see just that branch.

## Behavior Lens

Visualizes state machines for the selected node.

**When to use it:**

- Modeling operating modes and transitions
- Analyzing startup/shutdown sequences
- Documenting fault handling logic

**What you'll do:**

- Right-click any state, the canvas, or a transition row for the contextual menu (add state, add transition from/to here, edit, delete with cascade preview)
- Add states via toolbar or Command+K
- Draw transitions by dragging between states
- Edit transition triggers and state descriptions from the Inspector (state type / timing and transition guards / actions / timing become editable in a follow-up release)

Focus on nodes with meaningful operating modes. Controllers, power systems, anything with fault states.

## Causality Lens

Renders Harney's Pyramid. A technology dependency view.

**When to use it:**

- Identifying technology risks
- Understanding what depends on what
- Reviewing TRL distribution

**What you'll do:**

- View the pyramid with nodes layered by abstraction
- Review TRL color-coding (red = low maturity = risk)
- Trace from goals at top to physics at bottom

The Causality lens is read-only. It visualizes structure you've built elsewhere. It answers: "What's holding this up?"

## Completeness Lens

Finds gaps in your model.

**When to use it:**

- Before milestone reviews (is the model ready?)
- Prioritizing where to add detail next
- Auditing model quality

**What you'll do:**

- Review gap inventory by category
- Click gaps to navigate directly to the problem node
- Use Command+K: "What's missing from this subsystem?"

Completeness is relative to your phase. Early concept work tolerates gaps. Pre-CDR, very few.

## Narrative Lens

Generates a systemigram. A visual story of how your system works.

**When to use it:**

- Explaining the system to stakeholders
- Identifying the "mainstay" (critical path)
- Creating documentation graphics

**What you'll do:**

- View the auto-generated systemigram
- See the mainstay path highlighted
- Export the diagram for presentations

If the story doesn't make sense, your model structure might need work.

## Dendritic Lens

Shows your decision tree. Including the paths you didn't take.

**When to use it:**

- Reviewing trade study decisions
- Explaining why alternatives were rejected
- Onboarding new team members to history

**What you'll do:**

- View active nodes vs. pruned nodes
- Click pruned nodes to see rejection rationale
- Filter by decision type (physics, engineering, mission)

Pruned nodes aren't deleted. They're preserved context. The Dendritic lens makes that context visible.

## Verification Lens

Tracks test coverage and verification status.

**When to use it:**

- Planning verification activities
- Checking requirement coverage before reviews
- Recording test results

**What you'll do:**

- View coverage metrics per requirement
- Add verification records (test, analysis, demo, inspection)
- Link verifications to requirements

Low coverage isn't bad in early phases. Before release, gaps need attention.
