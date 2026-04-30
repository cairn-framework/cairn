# External Cairn Docs Research
**Status:** done
**Last updated:** 2026-04-28
**Updated by:** Uruk-hai scout

## Summary

getcairn.dev is an unrelated product: an AI-augmented model-based systems engineering (MBSE) platform for hardware/systems engineers. It shares only the name "Cairn." The site's docs IA and some presentation patterns are worth studying as craft, but zero content transfers. Our internal docs are architecturally stronger and more precise. One IA pattern and one prose technique are worth borrowing.

## Same-project or different?

**Verdict: entirely different product.**

Evidence: getcairn.dev's front page tagline is "From rough idea to structured model" and describes "AI-native model-based systems engineering" for systems engineers building rovers, satellites, and drones. Core concepts are system trees, ChangeSets, and twelve analytical lenses. No mention of blueprints, maps, provenance chains, authority chains, artefacts (in our sense), reconcilers, scanners, hooks, or anything from our spec. The URL resolves to a hosted SaaS application with a Pricing page and "Open App" CTA. This repo's Cairn is a Rust CLI tool for developer repos; the two share only a geological metaphor and the word "cairn."

## getcairn.dev surface map

**Front page:** Tagline "Chat vanishes. The model stays." Four-step loop: Describe, Decompose, Inspect, Refine. Value prop is persistent engineering models vs. ephemeral AI chat. Primary nav: Story, Demo, Concepts, Blog, Docs, Pricing, Open App. Audience: systems engineers and hardware product teams, not software developers. Marketing copy is sparse and fragment-heavy. A single rover example runs through the entire page.

**Docs IA (getcairn.dev/docs/):**
- Getting Started: What is Cairn?, Quick Start, Key Concepts
- Methodology: Lens Paradigm, AI Governance, Manual vs AI Paths, Four Questions, Dead Paths
- Core Concepts: Decomposition, Nodes & Properties, Requirements, Interfaces, State Machines, Verification
- Workflows: Lens Workflows
- Reference: Lenses, Tools, AI Pipeline, Entity Types, Keyboard Shortcuts
- Data & Privacy: Local-First Architecture
- Appendices: FAQ, Glossary, Changelog

**Voice:** Accessible, metaphor-driven ("stone markers hikers build"), narrative-first. Plain language with just enough engineering precision. Paragraphs are short; headings are used to gate each concept. No em-dashes observed.

**Depth and density:** Concept pages open with a problem statement, then staged revelation: problem, contrast, walkthrough. Reference pages pair brief prose with structured data (requirement IDs, status tags, parameter lists). Code-to-prose ratio is low; the product is visual/interactive so prose does the heavy lifting in docs.

**Audience signals:** Non-developer accessible, but assumes systems engineering mental model (subsystems, interfaces, verification). Treats taxonomy as self-evident via a running example rather than defining terms upfront.

## This repo's docs surface map

| Path | Audience | IA | Strength |
|---|---|---|---|
| `README.md` | Developer, agent | Problem, what-it-does, quickstart, hooks, design system, reference links. 158 lines. | Strong hook ("your agent gets lost every session"), accurate CLI surface, good cross-reference density |
| `docs/spec.md` | Implementor, senior contributor | Vocabulary, spec maturity model, problem, two-chain framing, sections per domain. 913 lines. | Authoritative. Two-chain / hinge framing is precise and distinguishable from any competitor. Maturity level model (Declared/Designed/Implemented) is a genuine asset. |
| `docs/blueprint.md` | Developer authoring .blueprint | Blueprint grammar reference. 90 lines. | Compact. |
| `docs/artefacts.md` | Developer, agent | Artefact type reference. 120 lines. | Compact. |
| `docs/changes.md` | Developer, agent | Changes system reference. 106 lines. | Compact. |
| `docs/hooks.md` | Developer, CI integrators | Hook classes and CLI. 122 lines. | Covers exit codes and --json; agent-consumable. |
| `docs/design-system/README.md` | UI contributors, agents | Token/component consumption patterns. | Clear, load-order explicit, consumption patterns for two targets (static HTML, Rust embed). |
| `docs/landing/index.html` | General / marketing | Static HTML marketing page. | Audience-facing; not part of developer docs. |
| `openspec/conventions.md` | Implementors, codex agent | Error codes, subsystem categories, registry rules. | Normative and precise. |
| `openspec/specs/<area>/spec.md` | Implementors per area | Consolidated per-area specs. Areas: artefacts, changes, cli, foundation, graph-explorer, hooks, kernel, mcp, multi-target, parser, query, reconciliation, terminology-rename, testing-baseline. | Covers every build phase; good breadth. |

**Gaps:**
- No "what is Cairn" narrative page aimed at the broadening non-dev audience CLAUDE.md mentions.
- No standalone glossary (vocabulary is buried in spec.md §0, not surfaced as a quick-reference).
- No explicit "Getting Started" path that ends with a working example analogous to their rover walkthrough.
- The two-chain architecture (the most differentiating concept) lives 50 lines into spec.md; it is not surfaced in any shorter, standalone document.

**Duplication:** README.md and spec.md §1 both describe the problem; the README version is more accessible but spec.md's is more precise. Neither is redundant; they serve different readers.

## Gap analysis

### Worth borrowing

1. **Progressive disclosure IA (getcairn.dev/docs/ top-level structure).** Their docs split Methodology, Core Concepts, and Reference into distinct top-level sections. Our docs currently have no such separation: spec.md mixes conceptual framing with implementation-level design. A short "Concepts" layer (what the two chains are, what blueprint/map/artefact/hinge mean, with examples) sitting above the spec would serve the non-dev audience CLAUDE.md calls out, without flattening anything from the spec.

   Borrow: add a `docs/concepts/` directory with at most 3-4 short pages (two-chain overview, blueprint authoring intro, map query intro, artefact types). These link down to spec.md for depth, not the reverse. Source: getcairn.dev/docs/ IA split between Methodology and Core Concepts.

2. **Single running example as a thread.** Their rover example anchors every concept page. Our README uses a one-line auth-token example and our spec uses abstract module names. A concrete "bootstrap fixture as running example" (the `test/fixtures/cairn-bootstrap/` fixture already exists) could thread through concepts, blueprint grammar, and query docs without inventing new material.

   Borrow: in any new concepts or getting-started material, use the bootstrap fixture as the anchoring example. Source: getcairn.dev's rover thread running from front page through all doc sections.

3. **Explicit Glossary as a named doc.** getcairn.dev/docs/ lists a Glossary in Appendices. We have spec.md §0 Vocabulary, which is authoritative but buried. A `docs/glossary.md` that is a flat alphabetical list of every term from spec.md §0, with one-line definitions and links back to spec, would serve both the broadening audience and any AI agent doing cold-start context lookup.

   Borrow: extract spec.md §0 into a standalone `docs/glossary.md`. Low effort, high agent-legibility gain.

### Already covered better here

1. **Architectural precision.** Their docs introduce concepts through story and analogy without ever formally specifying enforcement semantics. Our spec.md §3 distinguishes mechanically-checkable authority chain links from advisory provenance chain tensions with genuine precision. No equivalent exists on getcairn.dev; their tool is visual and AI-driven with no formal verification model.

2. **Maturity level model.** spec.md §0.1 (Declared / Designed / Implemented) is a clean contribution to spec-writing practice. Their docs have no equivalent; they describe a shipped SaaS, so maturity tracking is irrelevant.

3. **Agent-first output format.** Our --json flag, map.md frontmatter, and hooks are designed for machine consumption. Their docs describe a GUI product with no equivalent agent-query surface.

4. **Terminology rigor.** Blueprint vs. map vs. artefact vs. change vs. neighbourhood are load-bearing distinctions enforced by the spec and CLAUDE.md. Their terminology (lens, ChangeSet, system tree) is looser and example-driven.

### Out of scope / don't borrow

1. **Their narrative cold-open marketing style on the docs home page.** getcairn.dev/docs/ opens with story. Our docs home is README.md, which is correctly developer-and-agent-facing. The marketing surface is docs/landing/index.html. Keep these separate.

2. **Their visual/interactive demo pattern.** Rover renders, 3D meshes, Monte Carlo graphs. Irrelevant to a CLI Rust tool.

3. **FAQ and Changelog as top-level doc sections.** At our current phase (v0.7 spec, active implementation) a changelog is tracked in openspec/ and git. Surfacing it as a doc section is premature and would require maintenance overhead without clear reader benefit.

4. **"Manual vs AI Paths" and "AI Governance" methodology pages.** These address their specific AI-augmented editing workflow. Our equivalent is AGENTS.md plus hook semantics. Already handled in a more appropriate form.

## Recommendation

**adopt-some**

Adopt these three, in order of effort:

1. **docs/glossary.md** (lowest effort, highest agent-legibility return). Extract spec.md §0 vocabulary into a flat alphabetical reference. Add any terms defined later in the spec that don't appear in §0. Link from README.md and from spec.md §0.

2. **Bootstrap fixture as running example.** When writing any new concepts or getting-started doc, anchor on `test/fixtures/cairn-bootstrap/` rather than abstract names. No structural change needed; this is a writing convention.

3. **docs/concepts/ directory** (medium effort, required before any public-facing docs push). Three pages maximum: (a) the two chains and the hinge, (b) authoring a blueprint (with bootstrap fixture walkthrough), (c) reading the map and querying it. These sit above spec.md for the non-dev / newcomer audience. They do not replace or duplicate spec.md; they link down into it.

Do not restructure the existing spec.md, openspec/, or any existing per-area spec. Those surfaces are implementor-facing and are working correctly. The gap is the newcomer/non-dev layer, not the implementor layer.

## Sources

- https://www.getcairn.dev/
- https://www.getcairn.dev/docs/
- https://www.getcairn.dev/docs/getting-started/what-is-cairn
- https://www.getcairn.dev/docs/getting-started/key-concepts
- https://www.getcairn.dev/docs/methodology/lens-paradigm
- https://www.getcairn.dev/docs/reference/lenses

---

## Re-research with workflow lens (2026-04-28)

**Status:** done_with_concerns
**Expanded by:** Uruk-hai scout (deep-dive dispatch)
**Note:** The prior section above was correct at the level it looked, getcairn.dev is a different product. This section goes deeper into the *workflow* layer revealed by app screenshots. The finding shifts from "zero overlap" to "structural parallels worth studying, though the underlying models diverge sharply."

### Source code

**Verdict: closed source. No public implementation found.**

Evidence:
- `https://github.com/getcairn` org exists but contains one unrelated repo (`apod-viewer`, a Django interview project, last updated 2022-01-12). The org website is `www.getcairn.com` (outdoor subscription box, entirely different entity, same handle).
- No source code for the getcairn.dev MBSE tool appears anywhere on GitHub under any search combination tried: `getcairn`, `cairn MBSE system brief project genesis`, `cairn v0.3.1`.
- The docs site's local-first architecture page says "runs locally, no setup ceremony, no cloud lock-in" but gives no repository URL or open-source statement.
- The changelog page is blank/not detailed enough to extract version history.
- **Conclusion:** closed source SaaS. The v0.3.1 branding is an app version, not a public release tag.

URLs confirmed live and crawled:
- `https://www.getcairn.dev/docs/getting-started/key-concepts`
- `https://www.getcairn.dev/docs/methodology/ai-governance`
- `https://www.getcairn.dev/docs/methodology/manual-and-ai-paths`
- `https://www.getcairn.dev/docs/methodology/four-questions`
- `https://www.getcairn.dev/docs/core-concepts/nodes-and-properties`
- `https://www.getcairn.dev/docs/core-concepts/interfaces-and-signals`
- `https://www.getcairn.dev/docs/reference/entity-types`
- `https://www.getcairn.dev/docs/reference/ai-pipeline`
- `https://www.getcairn.dev/docs/appendices/glossary`
- `https://www.getcairn.dev/docs/data-and-privacy/local-first-architecture`

### Concept map

All verdicts are honest. "Same idea" requires both the concept AND the enforcement mechanism to align; surface-label matches that differ in depth are called out.

| Their term | Source | This repo's candidate | Verdict | Delta |
|---|---|---|---|---|
| **Project Genesis · Preserved as provenance** (screenshot: QA history durably stored, labelled provenance) | App UI (screenshot) | Provenance chain (source → research → decision) | `partial overlap` | Their genesis = the QA transcript of the interview, preserved as a durable record before build. Our provenance chain is richer: it spans source artefacts, research artefacts, and decisions with formal hinge semantics. Their genesis is a single-event snapshot; ours is a live graph. |
| **3 rounds of clarifying questions → confidence score (78%, 82%)** | App UI (screenshot) | Research artefacts feeding the hinge | `partial overlap` | Their rounds are AI-driven interview loops with a numeric confidence gate. We have no equivalent confidence-bounded interview loop in cflx; proposals are authored top-down by an Architect agent. The UX pattern (iterate until confident, then proceed) is theirs alone. |
| **Root system node + 4-7 subsystems + key interfaces + system brief** | App "Ready to build" screen (screenshot); docs/reference/entity-types | Blueprint (`.blueprint`) + map (`map.md`) + contracts | `partial overlap` | Their output shape is fixed: SYS → SS (4-7) → C → IF + REQ. Our blueprint is richer and domain-agnostic: it declares the graph shape, artefact types, neighbourhood queries, reconciler rules. Their model is hardware/MBSE-specific (SYS.01, SS.01, C.01.01 ID scheme). Ours enforces no fixed decomposition depth. |
| **System brief** | App UI (screenshot); docs/methodology/manual-and-ai-paths: "identifies key technical domains and surfaces them as a starting tree" | Decision artefact / OpenSpec proposal | `partial overlap` | Their system brief is a paragraph-level natural language description refined to 82% confidence, used as input to generation. Our decision artefact carries formal obligations (authority chain), rationale, and interface contracts. Their brief is input; our decision is a hinge with downstream enforcement. |
| **ChangeSet** (reviewable batch of structural modifications; accept or skip per item) | docs/reference/entity-types; docs/methodology/ai-governance: "AI proposes. You commit." | cflx apply → accept cycle | `same idea, different name` | Both are human-gated proposal batches that update the model only on explicit commit. Their ChangeSet is UI-driven per node; our accept step is agent-driven per phase. Structural isomorphism is real. |
| **Twelve Lenses** (Overview, Requirements, Architecture, Verification, etc.) | docs/reference/lenses; docs/getting-started/key-concepts | Map queries / neighbourhood queries / scan lenses | `partial overlap` | Both apply multiple analytical views to a single underlying model without duplication. Their lenses are fixed (12 named), UI-rendered. Our neighbourhood and query system is programmatic and extensible. Their "Verification" lens maps directly to our reconciler + drift detection surface. |
| **Node maturity: Draft → Planned → Verified** | docs/core-concepts/nodes-and-properties: "Draft, Planned, Verified visible in requirements lens" | Artefact status: ghost / synced / orphaned; verified / external / unverified | `partial overlap` | Both track lifecycle state per element. Their maturity is linear (3 states, requirement-focused). Ours is two-dimensional: one axis is sync state (ghost/synced/orphaned), another is evidence state (verified/external/unverified). Richer, not equivalent. |
| **Traceability loop** (REQ → Component → Verification → Results) | docs/concepts; docs/reference/entity-types | Authority chain (Decision → Blueprint → Contract → Code) | `partial overlap` | Both link specification to implementation to evidence. Their traceability is requirement-centric and manually navigated via lenses. Ours is a formal directed chain with enforcement at commit time (pre-commit hook, reconciler). Enforcement is the key delta. |
| **Rounds** (numbered refinement iterations, model persists across rounds) | docs/methodology/manual-and-ai-paths: "round 03 shown" | Phase lifecycle in cflx (apply → accept → archive) | `same idea, different name` | Both are named, numbered iteration cycles that produce durable incremental model changes. Their rounds are conversational refinement; ours are code-change phases with formal verification gates. |
| **"The model stays. Chat vanishes."** | Front page tagline | map.md as durable machine-readable snapshot; artefacts as persistent declared items | `same idea, different name` | Both prioritise persistence of structured artefacts over ephemeral conversational context. This is the deepest philosophical alignment between the two products. |
| **AI interview / clarifying questions** | docs/methodology/manual-and-ai-paths: "AI interview surfaces structural elements through clarifying questions" | No equivalent in current cflx; proposals are architect-authored | `unique to them` | We have no confidence-bounded interview entrypoint. This is the single largest UX gap and the strongest borrow candidate. |
| **Architecture signals** (auto-extracted structural reasons from interview answers) | App UI screenshot ("Architecture signals" sub-section in round 3) | Rationale tensions (advisory) + contract obligations (blocking) | `partial overlap` | Their signals are inferred from interview answers and shown as pre-build context. Our rationale tensions are spec-declared and post-scan advisory findings. Their signals are inputs to generation; ours are outputs of reconciliation. |
| **Local-first architecture** | docs/data-and-privacy: "runs locally, no cloud lock-in" | Rust CLI, local filesystem, no network dependency | `same idea, different name` | Both are local-first by design. Their implementation is a desktop/web app with local persistence; ours is a Rust CLI. |

Concepts unique to this repo with no getcairn.dev equivalent:

- Provenance chain as a live directed graph (not a snapshot record)
- Authority chain with mechanical enforcement at commit time
- Hinge semantics (decision carries obligations in both directions)
- Ghost / orphaned artefact detection
- Reconciler as a pluggable interface
- Scanner vs. reconciler vs. scan as distinct concepts
- Interface hash (drift detection via content fingerprint)
- Neighbourhood as a graph-theoretic query primitive
- Blueprint grammar as a declarative authored file (not AI-generated)
- Multi-target reconciliation
- Hook system (SessionStart, PostToolUse, pre-commit)

### Workflow parallels

Step-by-step comparison of their app workflow (from screenshots) against our cflx lifecycle:

| Step | Their app | Our cflx |
|---|---|---|
| 1. Capture intent | User types free-form prompt ("a USV with an ROV, controlled from a remote ops centre") | Human or Architect agent authors `proposal.md` in `openspec/changes/<phase>/` |
| 2. Refine intent | 3 AI-driven rounds of clarifying questions (5+4+3 = 12 Qs); each round updates "current understanding" with a confidence score (78% → 82%) | No equivalent; Architect authors design.md and tasks.md directly. User reviews and approves. |
| 3. Preserve intent as provenance | "Project Genesis · 3 rounds · Preserved as provenance", QA transcript durably stored before any build action | No equivalent genesis record. Proposal.md is the closest analogue but it is not a QA transcript and is not explicitly labelled as provenance. |
| 4. Generate structure | AI generates: root system node, 4-7 subsystems, key interfaces, system brief. Progress checklist: creating project structure → generating system architecture → writing system brief → saving project genesis → ready to explore. | cflx apply: codex agent executes tasks.md, implements blueprint changes, adds/modifies artefacts, runs cargo build + clippy + fmt + tests. |
| 5. Review proposals | ChangeSets: per-node accept/skip, atomic model update on commit | cflx accept: human reviews agent output, runs verification gate battery, merges worktree. |
| 6. Iterate | "round 03": model persists across rounds, IDs stable, chat discarded | Next phase: archive current phase, draft new proposal.md for next change set. |
| 7. Query the model | 12 lenses (Requirements, Architecture, Verification, etc.) applied to the persistent model | `cairn scan`, map.md, neighbourhood queries, hook-injected context. |

**Biggest structural parallel:** both treat the human as a gating agent between AI-proposed changes and model state. Neither allows the AI to modify the authoritative model without explicit human commit. This is the deepest architectural alignment.

**Biggest structural gap:** they have a rich intent-capture front-end (3-round interview, confidence scoring, genesis preservation) before any generation. We go straight to Architect-authored proposals. The entire "before the build" UX is theirs; we don't have it.

### Borrow list (revised)

Ranked by leverage for a future MCP/plugin/webui entrypoint.

**1. Confidence-bounded multi-round interview as proposal genesis (highest leverage)**

Their pattern: user types rough intent → AI asks clarifying questions in rounds → confidence score rises → at threshold, "Ready to build" is shown → genesis record (full QA transcript) is preserved as provenance before any generation begins.

Mapped to our model: replace (or wrap) the manual `proposal.md` authoring step with an MCP-driven interview session. Each round of QA refines the proposal draft. When confidence reaches threshold (e.g., 80%), the interview output is committed to `openspec/changes/<phase>/proposal.md` plus a new `genesis.md` artefact (the raw QA transcript, explicitly typed as a research artefact in the provenance chain). The Architect agent then receives the genesis record as its primary context instead of a hand-authored brief.

This is **additive, not a replacement.** The existing OpenSpec proposal format stays; the interview is a new entrypoint that auto-drafts the proposal. Architects can still bypass it and author proposals directly.

Effort: medium (MCP tool + interview agent + genesis artefact type). Leverage: high: it closes the biggest UX gap for non-developer users and produces a richer provenance record than manual authoring.

**2. Genesis record as a first-class provenance artefact (medium leverage)**

Even without the full interview UI, the pattern of explicitly saving "why we built this, before we built it" as a durable, typed record (not just a commit message or proposal prose) is worth lifting. A `genesis` artefact type (or a `research` artefact with subtype `genesis`) in the provenance chain would make the source→research→decision chain navigable for any change, not just ones with a full interview transcript.

This is immediately implementable as a spec addition. No UI needed. High signal-to-effort ratio.

**3. Fixed-shape first-pass decomposition as a generation hint (lower leverage)**

Their "root node + 4-7 subsystems + key interfaces + system brief" is a generation contract that constrains the first-pass output to a useful default shape. For a future webui entrypoint, offering a similar "start with a skeleton: 1 root blueprint, 3-5 declared items, key contracts" as a default template would lower the blank-page problem for new users.

However: do not make this shape mandatory or permanent. Their fixed shape works for hardware MBSE; our domain is developer repos and arbitrary systems. The fixed shape is a default first-pass, not a schema constraint. The blueprint grammar already supports richer shapes; this is a UX affordance only.

**4. Numbered-round display in cflx output (low leverage, low effort)**

Their round numbering (round 01, round 02, round 03) gives users a clear sense of progress and a stable reference for "where we are." Our phase naming (phase-8.0, phase-9.0) is equivalent but less visible at the CLI output level. A minor UX improvement: cflx status output could display "Phase N of M" alongside the phase name.

### Things our model has that theirs lacks

1. **Formal enforcement.** Their model is advisory: ChangeSets are proposals, acceptance is manual, but there is no pre-commit enforcement hook that blocks a commit if a node drifts from its declared state. Our pre-commit hook + reconciler blocks commits on drift. Their "model is the artifact" is a UX principle; ours is mechanically enforced.

2. **Two-chain topology.** They describe a linear decomposition chain (Brief → Subsystems → Components → Interfaces → Requirements → Verification). They have no concept of a provenance chain running in the opposite direction, or a hinge point where evidence-bearing artefacts carry obligations downstream. Their traceability is upward-linking (requirement to evidence); ours models both directions with different enforcement semantics.

3. **Pluggable reconciler interface.** Their AI pipeline is fixed (Describe → Decompose → Inspect → Refine with 12 named lenses). Our reconciler is a pluggable interface; new domain-specific checks can be added without changing the kernel.

4. **Blueprint as authored source of truth.** Their model is AI-generated and AI-refined; the human reviews but does not author the schema. Our blueprint is a human-authored declarative file; the AI is a consumer and implementor, not the author. This is a philosophical difference: we treat the declaration as the authoritative artefact, not the generated model.

5. **Multi-target reconciliation.** No equivalent in their docs. Their model targets one system tree per project.

6. **Agent-first output (--json, map.md, hooks).** Their product is a GUI tool. Our entire output surface is designed for machine consumption by coding agents. No getcairn.dev equivalent.

7. **Ghost / orphaned detection.** We detect when declared items exist without implementation (ghost) or implementation exists without declaration (orphaned). They have no equivalent structural mismatch detection outside of requirement verification status.

8. **Domain-agnostic scope.** Their tool is explicitly MBSE for hardware systems (rover, satellite, drone examples throughout). Our framework targets any developer repo and is spec-designed to extend to non-code domains (orgs, research, processes).

### Open questions

These cannot be answered from the public site and require the user (human) to decide or investigate:

1. **Is there a private beta / access program for getcairn.dev?** Their docs are thorough but the app requires sign-up. A hands-on session would clarify whether the "Project Genesis · Preserved as provenance" label in the screenshots maps to a queryable artefact or just a UI display label with no downstream use.

2. **Who built getcairn.dev and what is their background?** If the founders come from a systems engineering / MBSE background (INCOSE, SysML), the provenance/authority chain terminology may be convergent vocabulary from that field rather than independent invention. If they come from a developer-tooling background, the overlap is more meaningful as independent convergence.

3. **Does getcairn.dev expose any API or MCP interface?** Their local-first claim and "no cloud lock-in" language suggests a local app, but if they expose a query interface, the lens/node model could be a borrow target for our neighbourhood query semantics.

4. **User decision: is the genesis artefact type worth adding to the spec now (pre-MCP/webui), or wait?** The concept is implementable as a spec-level addition (research artefact subtype = genesis) independent of any UI work. Adding it now would make the provenance chain more navigable immediately. The user should decide whether this goes into an upcoming phase or is held for the MCP phase.

5. **Confidence scoring mechanism.** Their 78% → 82% confidence scores across rounds are shown in the UI but not explained in any public doc page. Whether this is a hard gate (generation blocked below threshold) or a soft indicator affects how directly we could translate the pattern to our MCP interview flow.

### Additional sources (this section)

- `https://www.getcairn.dev/docs/getting-started/key-concepts`
- `https://www.getcairn.dev/docs/methodology/ai-governance`
- `https://www.getcairn.dev/docs/methodology/manual-and-ai-paths`
- `https://www.getcairn.dev/docs/methodology/four-questions`
- `https://www.getcairn.dev/docs/core-concepts/nodes-and-properties`
- `https://www.getcairn.dev/docs/core-concepts/interfaces-and-signals`
- `https://www.getcairn.dev/docs/reference/entity-types`
- `https://www.getcairn.dev/docs/reference/ai-pipeline`
- `https://www.getcairn.dev/docs/appendices/glossary`
- `https://www.getcairn.dev/docs/data-and-privacy/local-first-architecture`
- `https://github.com/getcairn` (org: unrelated outdoor subscription box, not the MBSE tool)
