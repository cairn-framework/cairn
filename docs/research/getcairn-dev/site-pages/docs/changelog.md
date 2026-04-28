# Version History

**Source:** https://www.getcairn.dev/docs/changelog
**Captured:** 2026-04-28

## Version History

Release notes and feature updates. Cairn follows semantic versioning: MAJOR.MINOR.PATCH.

### 1.1.1-beta. April 2026

#### Lens and UI Polish

A focused polish pass on three surfaces identified as friction points after the Editability Release: the Behavior lens, the Change Review modal, and the command palette. The underlying data model and AI pipeline contract remain unchanged.

**Behavior lens right-click parity**

- State nodes, canvas, and transition table rows now expose right-click menus matching the Architecture lens (since v1.1.0-beta)
- State menu options: Edit, Add transition from/to here, Delete (with cascade preview), Delete with AI review
- Final states omit "Add transition from here"
- Canvas: Add state
- Transition row: Edit, Delete, Delete with AI review
- Keyboard Delete/Backspace on selected states triggers cascade-aware deletion, matching Architecture canvas behavior
- AI-driven menu items disable during behavior simulation (same pattern as Architecture during pipeline execution)

**Change Review modal redesign**

- Two-pane layout: warnings and AI reasoning on left; operations and What's Next on right
- Independent scrolling in both panes; single-column layout below 880px viewport
- Per-section "Expand all descriptions" chip in headers; per-row click-to-expand for long descriptions (two-line default clamp)
- Items with attached warnings render expanded automatically
- Item-ID references in warnings are clickable, jumping to corresponding operations with section expansion and brief row highlight
- Bottom scroll-progress fill tracks right pane position
- Keyboard 'e' cycles all right-pane sections expanded or collapsed

**Command palette polish**

- Auto-growing textarea accommodates lengthy prefilled prompts from "What's Next?" pills (capped at ~6 lines, scrolls beyond)
- "Prefilled from review modal" caption beneath input indicates injected prompts
- Pipeline progress displays specialist-specific duration hints ("typically 30 to 60s for decomposition work")
- Clarification questions render as horizontal chip buttons with explanatory banner, original prompt visibility, and progress footer
- **Preserved:** closing palette during pipeline execution keeps the run alive; re-open via status indicator chip

**Coming next**

- State and Transition Inspector field editing
- Pipeline cancellation
- Manual creation forms for transitions and states
- "Pick and choose" individual operation acceptance in Change Review modal

### 1.1.0-beta. April 2026

#### Editability Release

Version 1.0.0-beta required Command+K and AI for model changes. 1.1.0-beta enables direct editing of every artifact and provides deterministic right-click or Inspector paths alongside the command palette.

**Direct editing**

- Click-to-edit on all Inspector fields: node names, descriptions, requirement priorities, interface protocols, state timing, signal metadata
- Edits commit immediately
- Verification records now include an editable Title field (AI-populated on creation)

**Right-click menus**

- Tree right-click: Add child, Rename, Decompose with AI, Delete (with cascade preview), Delete with AI review
- Architecture canvas right-click: Add child, Decompose with AI, Connect to (manual), Connect with AI, Delete (with cascade preview), Delete with AI review

**Cascade previews**

- Delete operations with downstream dependents display preview of affected items: trace links, child nodes, dependent verifications, orphaned signals
- Signals use lighter two-click arm-then-confirm pattern (no cross-entity dependents)

**Interfaces and signals**

- Manual interface creation via canvas right-click, Connect to (manual) or via parent node Inspector, plus New Interface
- Inline-editable signals within interfaces: collapsed row per signal, click chevron to expand full editing grid
- Two-click Remove on signals (first click arms, second click within three seconds removes)

**AI pipeline visibility**

- Pipeline status chip in toolbar shows running/error/clarification-needed/ready states
- Router emits structured clarification questions (text, single-choice, multi-choice) directly in palette when confidence is low
- Answering re-runs the prompt with appended answers

**Coming next**

- Refactor .cairn.zip export for clean GitHub round-tripping
- Manual creation forms for states and transitions
- Right-click parity on Behavior lens
- Curated unit dropdowns on signal metadata

### 1.0.0-beta. April 2026

#### Initial Release

Cairn's first public release representing the primary feature set at launch.

**Features**

- 12 analytical lenses: Overview, Brief, Visuals, Requirements, Architecture, Causality, Completeness, Narrative, Dendritic, Behavior, Verification, Operational
- Hierarchical system modeling with five node types: system, subsystem, assembly, part, external
- Requirements management with six types: functional, performance, interface, safety, environmental, constraint
- Interface and signal definitions with protocol and rate metadata
- State machine editor with states, transitions, guards, actions, timing annotations
- Engineering properties catalog with budget rollups (mass, power, cost) and TRL tracking
- AI property suggestions with per-item accept/dismiss workflow
- Verification records with test, analysis, demonstration, inspection methods
- Traceability tool with satisfies, verifies, derives, depends_on, implements link types
- AI decomposition and generation via Command+K command palette
- 17 AI specialists routed through 5-stage pipeline (Router, Context, Specialist, Validator, Review)
- ChangeSet governance: AI proposes, user reviews operation-by-operation before applying
- 2D concept renders via Gemini image generation with 6 style kits
- 3D mesh generation via AI-authored Three.js code with MeshBuilder
- Monte Carlo simulation via in-browser Pyodide (Python) with AI-generated scripts
- Export: JSON, Markdown, CSV, full ZIP with assets
- AI-powered export: PPTX presentations and DOCX reports via Claude Skills API
- Local-first architecture with IndexedDB/Dexie storage
- Quality linter with 16+ automated rules across nodes, requirements, interfaces, traces
- Supabase authentication with magic link sign-in

**Methodology**

- Four Questions framework for systematic model interrogation
- Harney's Pyramid of Causality for technology dependency and maturity analysis
- Dendritic decision tracking with pruned alternatives as first-class entities
- Systemigram narrative visualization with mainstay identification
- Three-axis completeness scoring based on Pace/Loper simulation fidelity dimensions

**Roadmap**

- Cloud sync for multi-device workflows
- Real-time collaboration
- GitHub export with ChangeSet-to-commit mapping
- AI-assisted trade study tool with Pugh/AHP methodology
