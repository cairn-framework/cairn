# Product overview: getcairn.dev

## What this is

getcairn.dev is an AI-augmented Model-Based Systems Engineering (MBSE) platform aimed at hardware and systems engineers building rovers, satellites, drones, and similar engineered systems. It is **not** related to this repo's CAIRN framework beyond the shared name. Its tagline "Chat vanishes. The model stays." captures its core value prop: AI conversation produces a persistent structured engineering model rather than ephemeral text.

This page is the entry point for understanding the product. For the workflow, see [02-workflow-genesis.md](./02-workflow-genesis.md). For their concept-to-our-concept mapping, see [07-ontology-comparison.md](./07-ontology-comparison.md).

## Identity and positioning

- **Tagline.** "Chat vanishes. The model stays." (**verified** from front page).
- **Working theme.** "From rough idea to structured model" (**verified** from front page subtitle).
- **Audience.** Systems engineers, hardware product teams, and adjacent disciplines that already think in subsystems, interfaces, and requirements (**verified** by example domain coverage in their docs: rover, satellite, drone, USV/ROV).
- **Pricing model.** Hosted SaaS application with a Pricing page and "Open App" CTA (**verified** from front-page nav).
- **Audience tone.** Non-developer accessible but assumes a systems-engineering mental model. Treats taxonomy as self-evident through running examples rather than defining terms upfront (**verified** by docs voice).

The fixture domain that recurs across their public materials and the trial captured here is an **Offshore Unmanned Survey System** combining a USV (unmanned surface vessel), an ROV (remotely operated vehicle), a Launch and Recovery System (LARS), Communications, Power Generation, and an Autonomy/Control/Mission Payload subsystem. The user's hands-on trial used this exact fixture (**from screenshots 01 to 24**).

## Source-code status

**Closed source.** (**verified** by prior scout). Searches for `getcairn`, `cairn MBSE`, and the version string `v0.3.1` produced no public repository. The GitHub org `github.com/getcairn` exists but contains one unrelated Django interview project last updated 2022, and the org website is `www.getcairn.com` (an unrelated outdoor subscription box). The `getcairn.dev` MBSE tool is a separate organisation with no public source. The "v0.1.0" model-version string visible in the running app's status bar (**from screenshot 11**) is a per-project model version, not a public release tag.

Implication for our research: we can describe behaviour observed in the UI but we cannot verify their underlying mechanism. Where we infer mechanism from observed behaviour, we mark it **inferred** explicitly.

## Surface map

What the public surface offers, at a glance.

### Marketing front (`getcairn.dev`)

- Tagline plus four-step loop: **Describe, Decompose, Inspect, Refine** (**verified**).
- Single rover example threaded through every section (**verified**).
- Primary nav: Story, Demo, Concepts, Blog, Docs, Pricing, Open App (**verified**).
- Marketing copy is sparse and fragment-heavy.

### Documentation (`getcairn.dev/docs/`)

- **Getting Started:** What is Cairn?, Quick Start, Key Concepts.
- **Methodology:** Lens Paradigm, AI Governance, Manual vs AI Paths, Four Questions, Dead Paths.
- **Core Concepts:** Decomposition, Nodes & Properties, Requirements, Interfaces, State Machines, Verification.
- **Workflows:** Lens Workflows.
- **Reference:** Lenses, Tools, AI Pipeline, Entity Types, Keyboard Shortcuts.
- **Data & Privacy:** Local-First Architecture.
- **Appendices:** FAQ, Glossary, Changelog. (All **verified** via prior scout.)

### Running app

The trial captured a logged-in session with a project named `Offshore Survey USV-ROV` at model version `v0.1.0` (**from screenshot 11**). The app surfaces 10 top-nav tabs, a left-rail system tree, an 8-tool sidebar, a context-aware command palette invoked by `⌘K`, and a persistent `Quality 73` score plus `1 pending review` counter in the status bar. See [03-information-architecture.md](./03-information-architecture.md) for the full chrome inventory.

## Their model in one paragraph

Their mental model is a **single linear traceability chain** (**inferred** from docs IA and from the four-step loop). The system is decomposed top-down from a system brief into subsystems, components, and interfaces. Requirements attach to nodes. Verification closes the loop by linking evidence to requirements. Multiple "lenses" (analytical views) are applied to the same underlying graph without duplicating data. Maturity is tracked per node along a Draft / Planned / Verified axis (**verified** via prior scout). This is **not** isomorphic to our two-chain provenance / authority topology; see [07-ontology-comparison.md](./07-ontology-comparison.md) for the term-by-term mapping and where the abstractions diverge.

## What is unique to them

(Both verbs and shapes that we do not have.)

- A confidence-bounded AI clarifying interview (rounds, percentages, "Ready to build" gate). See [02-workflow-genesis.md](./02-workflow-genesis.md).
- A four-stage generation pipeline visible to the user as a state machine (`ROUTE → CONTEXT → GENERATE → VALIDATE`). See [06-command-palette.md](./06-command-palette.md#generation-pipeline-route--context--generate--validate).
- A three-axis fidelity radar (Entities, Processes, Relationships) with per-node and per-system aggregation. See [05-completeness-and-causality.md](./05-completeness-and-causality.md).
- Inline pedagogical book quotes (Harney, Pace) appearing in side rails to teach the user the mental model. See [09-design-influence.md](./09-design-influence.md).
- Per-property `· ai ·` tagging that distinguishes generated from authored values. See [04-node-model.md](./04-node-model.md#properties-and-ai-tagging).

## What is unique to us

(Capabilities our framework has that getcairn.dev does not appear to.)

- Two chains meeting at a hinge: a provenance chain (source to research to decision) flowing one direction and an authority chain (decision to blueprint to contract to code) flowing the other, with the decision artefact carrying obligations both ways. Their model is single-directional traceability; ours is bidirectional with formal hinge semantics.
- Mechanical enforcement at commit time via the pre-commit hook plus the reconciler. Their ChangeSet flow is human-gated but advisory; ours blocks commits on drift.
- Ghost / orphaned / synced detection as a structural state of every declared item.
- A pluggable reconciler interface (their AI pipeline appears fixed at four stages).
- A blueprint as a human-authored declarative source of truth (their model is AI-generated and AI-refined; the human reviews but does not author the schema).
- Multi-target reconciliation, agent-first output (`--json`, map.md, hooks), domain-agnostic scope.

See [08-borrow-list.md](./08-borrow-list.md) for which of their patterns we should consider lifting and which we should not.
