# AI-Native Model-Based Systems Engineering

**Source:** https://www.getcairn.dev/story
**Captured:** 2026-04-28

Navigate the system, _not views._

Cairn is where AI operates through structured, reviewable changes, not chat. Every node in your system tree can be viewed through multiple lenses. Requirements, architecture, behavior, visuals, causality, all scoped to where you are.

- 9 Lenses per node
- 3 AI Agents per simulation
- $0.04 per sim run
- 200ms re-execution

## The Problem

Powerful tools. Painful experiences.

A $2 to 4B market dominated by incumbents with deep capabilities but universally poor UX. Only 5% of organizations have fully transitioned. The barriers: learning curve, cost, collaboration failures.

| Product | Company | UX | AI |
|---------|---------|-----|-----|
| Cameo / CATIA Magic | Dassault | Poor | None |
| Rhapsody | IBM | Dated | AI Hub v1.1 |
| Enterprise Architect | SPARX | Dated | None |
| Capella | Eclipse | Good | None |
| Cairn | AI-native | Modern | Native |

## The Lens Paradigm

One node, infinite perspectives.

Click any node in the system tree and switch between lenses. Requirements, architecture, behavior, verification, visuals, causality, all scoped to where you are.

## AI Agent Pipeline

Structured operations, not conversations.

Every AI action flows through a five-stage pipeline. Six domain specialists, each with focused context. The output is a ChangeSet you review operation-by-operation.

1. Router (Haiku). Classify intent, choose specialist
2. Context Assembly (Code). Fetch only what's needed
3. Specialist (Sonnet). Domain expert execution
4. Validator (Hybrid). Consistency and quality check
5. User Review (UI). Accept, reject, or cherry-pick

## Visuals and 3D

See your system before it exists.

Generate 2D concept art via Gemini with six style kits:
- Photorealistic
- Blueprint
- Concept Art
- Clay Render
- Isometric
- Exploded View

### 3D Mesh Pipeline

1. Description plus concept image, Claude Vision
2. MeshBuilder code (~200-400 lines)
3. Static analysis (security sandbox)
4. In-browser execution (~50ms)
5. Three.js viewer plus orbit controls
6. Export as glTF 2.0

## Simulation Agent

AI generates the simulation. You tweak and re-run.

Three agents read your model, extract parameters, generate Python, interpret results against requirements. Edit any parameter and re-execute in 200ms. Zero AI calls on re-run.

Example metrics:
- Endurance: 4.2 hr (passes REQ-008)
- Avg Power: 968 W (62% motors)
- Charge Time: 115 min (warning REQ-009)
- Deliveries: 8/shift (2 charges)

## Causality Lens

What must exist before this can be realized?

Based on Dr. Robert C. Harney's Pyramid of Causality. Cairn computes prerequisite layers from your model, capstone through knowledge foundation. Gaps surface as actionable warnings.

"If one or more levels are missing, the capstone is unlikely to be created." (Harney, Technology Evaluation, Ch. 1)

### Pyramid Layers
- Domain Technologies
- Components
- Parts
- Interfaces
- Knowledge

### Gap Detection

Warning: Thermal Mgmt, 0 children

Click gap, Command+K pre-filled with decomposition prompt

### Node Maturity

Maps to Technology Readiness Levels. Color-codes the pyramid per-node:
- production
- prototype
- concept
- mature

## Properties and Budgets

Domain-aware. Hierarchical. Actionable.

~30 property specs across 8 engineering domains, seeded by AI during decomposition. Mass, power, cost roll up through the tree. Budget bars show usage vs. allocation.

- Mass: 42.6 / 50 kg (85%)
- Power (peak): 1185 / 1500 W (79%)
- Cost (est.): $18.4K / $25K (74%)

## Onboarding

From blank idea to instrumented model in minutes.

Start from scratch or pick a template. The AI Inception Interview asks architecture-shaping questions, refines your description, then auto-decomposes.

### Templates

- Delivery Rover (Full Example)
- Orbital Data Center (Advanced)
- Insulin Pump (Advanced)
- Home Energy (Starter)
- Ag Drone Fleet (Intermediate)
- Subsea ROV (Intermediate)

### Inception Flow

1. Describe. Natural-language system description
2. Interview. AI asks 3 to 5 architecture-shaping questions
3. Refine. Iterative improvement (confidence at least 0.8)
4. Decompose. Auto-generate subsystems, interfaces, brief
5. Explore. Workspace with full model ready

## Market Position

Five gaps define the opportunity.

A $2 to 4B market growing 10 to 16% annually. Only 5% fully adopted. SysML v2 creates a switching window. No VC-backed AI-MBSE startups identified.

| Gap | Status | Impact | Priority |
|-----|--------|--------|----------|
| AI-Assisted Decomposition | Unoccupied | Zero production tools | Very High |
| Behavioral Model Generation | Unoccupied | No tool generates state machines from NL | Very High |
| Structured AI Operations | Unexplored | ChangeSet for review, no precedent | High |
| Modern MBSE UX | Unmet | No VS Code or Figma-like experience | High |
| Traceability Automation | Nascent | Req to arch to ver remains manual | Medium |

Market metrics:
- $2 to 4B market size (10 to 16% CAGR)
- 5% fully transitioned to MBSE
- 0 VC-backed AI-MBSE startups

---

AI-native systems engineering. _Structured. Reviewable. Yours._

The thinking phase before the formal model. From blank idea to instrumented, simulation-ready system architecture, in minutes, not months.

[Try the Rover Demo](/demo)
