# Quick Start

**Source:** https://www.getcairn.dev/docs/quick-start
**Captured:** 2026-04-28

Get from zero to a working system model in under five minutes.

## Prerequisites

A modern browser (Chrome, Firefox, Safari, Edge) and a free account. The free tier includes a limited number of nodes and AI calls, enough to explore the platform and build a small model.

## Step 1: Describe Your System

Open the app and you'll see a text area asking what you want to build. Type a plain-language description of your system. It can be rough:

"An autonomous delivery rover with LIDAR navigation, hub motors, and a lockable cargo compartment"

"A portable weather station with solar power, cellular connectivity, and temperature/humidity/pressure sensors"

Don't worry about structure or completeness, that's what the next steps will help you figure out.

## Step 2: AI Interview

Click **Continue with AI** to start a short architecture interview. The AI asks 3 to 5 questions designed to surface important constraints and decisions:

- What's the operating environment?
- What are the critical performance requirements?
- What interfaces does it need with external systems?

Select from the options the AI suggests, or type your own answer in plain language. Each round refines the system description and increases confidence. You'll typically go through 2 or 3 rounds before the AI has enough context to propose a structure.

## Step 3: Review the Initial Decomposition

When you click **Build**, the AI generates:

- A root system node with your system description
- 4 to 7 subsystems with initial property estimates
- Key interfaces between subsystems
- A narrative brief summarizing the system

This appears as a **ChangeSet**, a list of proposed additions to your model. Review each item. Accept what looks right, reject what doesn't, or edit before accepting. Nothing changes until you approve it.

## Step 4: Explore with Lenses

Your model now has structure. Select any node in the system tree (left sidebar) and explore it through different lenses (top tabs):

- **Overview**. Dashboard with health metrics and children
- **Requirements**. Functional, performance, and safety requirements
- **Architecture**. Visual graph of child nodes and interfaces
- **Behavior**. State machines for operational modes
- **Causality**. What prerequisite technologies does this need?
- **Completeness**. What's missing from this part of the model?

Each lens shows the same model from a different perspective. The data doesn't change, only how you view it.

## Step 5: Refine Your Model

Three paths handle different kinds of change. Pick whichever matches the work in front of you.

**Command palette** for AI-assisted changes. Large-scale decomposition, generating multiple requirements at once, drafting a state machine. Press Command+K (or Ctrl+K on Windows) and type a natural-language request scoped to your selected node:

"Decompose the power subsystem into battery, charging, and power distribution"

"Add thermal requirements for outdoor operation"

"Generate a state machine for the navigation controller"

The AI produces a ChangeSet. Review it operation-by-operation before applying.

**Inspector panel** for direct edits. Renaming a subsystem, fixing a description, adjusting a requirement's priority. Click the field you want to change and type. Edits commit immediately.

**Right-click menu** for contextual operations. Adding a child, deleting a node, connecting two nodes. Right-click any item in the system tree or the Architecture canvas. The menu lists the operations available in context, and deletions show a cascade preview of what else will be removed.

## What's Next

You now have a working model. From here you can:

- Dive deeper into any subsystem (right-click, Decompose with AI, or select it and press Command+K)
- Generate visual renders of components (Visuals lens)
- Run simulations to validate power or mass budgets (Simulation tool)
- Export to PowerPoint or Word for stakeholder review
