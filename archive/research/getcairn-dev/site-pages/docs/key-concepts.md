# Key Concepts for New Users

**Source:** https://www.getcairn.dev/docs/key-concepts
**Captured:** 2026-04-28

Before diving deeper, here are the five ideas that will help everything else make sense.

## The Node Hierarchy

Every model is a tree of nodes. The hierarchy follows a natural decomposition pattern:

System -> Subsystem -> Assembly -> Part

A **System** is your top-level product or project. **Subsystems** are major functional groups (Power, Navigation, Communications). **Assemblies** are collections of parts that work together. **Parts** are the leaves, individual components you'd find on a bill of materials.

There's also **External**, nodes that represent systems outside your boundary that your system interacts with (a ground control station, a user, a power grid).

You don't have to use all levels. A simple system might only need System -> Subsystem -> Part. The hierarchy is there when you need it.

## The Lens Paradigm

Cairn doesn't have separate "requirements view" and "architecture view" and "behavior view" as different documents. Instead, you have _one model_ viewed through _multiple lenses_.

Select a node. Switch tabs. Each lens shows that node's requirements, or its state machine, or its child architecture, or its verification status. The model doesn't change, only your perspective on it.

This means requirements, architecture, and behavior stay connected. Change a node's name and it updates everywhere. Add an interface and it appears in both the Architecture lens and the Interface Summary in the Brief.

## ChangeSet Governance

When you ask the AI to do something (decompose a subsystem, generate requirements, add states), it doesn't directly modify your model. Instead, it produces a **ChangeSet**, a list of proposed operations:

```
create node "Battery Pack" as child of "Power Subsystem"
create requirement "REQ-PWR-001: Capacity" on "Battery Pack"
create interface between "Battery Pack" and "Power Distribution"
```

You review each operation. Accept, reject, or edit before accepting. Only when you apply the ChangeSet does your model change.

This is the core governance mechanism. AI proposes, you decide. Every change is traceable. Nothing happens without your approval.

## Two Paths for Changing the Model

Cairn gives you two first-class ways to change the model: AI assistance for large or open-ended work, and direct editing for known, surgical changes. Both routes write to the same model; the difference is just whether you spell out the change yourself or let an AI specialist draft it.

The **Command+K command palette** is the primary AI surface. Press Command+K (Ctrl+K on Windows) and type what you want in natural language:

```
"Add safety requirements for battery thermal runaway"

"Decompose into sensor array, processor, and housing"

"Generate states for startup, active, and shutdown modes"
```

The request is scoped to your currently selected node. The AI figures out which specialist to use (requirements, architecture, behavior, etc.), assembles the right context, and produces a ChangeSet for your review.

**Direct editing** covers everything you'd rather do yourself. The Inspector panel turns every field into a click-to-edit control: title, description, priority, protocol, signal rate, committing your change immediately. Right-click any item in the system tree or the Architecture canvas to add a child, rename, or delete (with a cascade preview of what else goes); the canvas additionally lets you connect nodes.

Use whichever path matches the work in front of you. AI is faster when the change is large or you don't yet know its exact shape; direct editing is faster when you do.

## The Four Analytical Lenses

Beyond the standard lenses (Requirements, Architecture, Behavior), Cairn has four analytical lenses that compute insight from your model:

- **Causality**. What prerequisite technologies does this node depend on? What knowledge foundation supports it?
- **Completeness**. What's missing? Which parts of the model are thin on requirements, behavior, or relationships?
- **Narrative**. What does this system actually do? What's the main transformation chain from input to output?
- **Dendritic**. How did we get here? What alternatives were considered and rejected? What were the decision points?

These four lenses answer questions that traditional MBSE tools can't. They're covered in depth in the Methodology section.
