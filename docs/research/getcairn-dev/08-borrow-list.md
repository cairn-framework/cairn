# Borrow list: affordances worth porting back

## What this is

Concrete affordances from getcairn.dev that map cleanly onto our framework's surface, ranked by adoption priority. Each entry has a short heading, a one-paragraph rationale, a citation back to a screenshot or doc, and an H/M/L priority.

Priorities are pragmatic: H means we should plan a phase to adopt it, M means it's worth a small experiment, L means log and revisit. Where a candidate would not fit our two-chain topology, the entry is marked **skip** with reasoning.

For the data-model context behind these candidates, see [07-ontology-comparison.md](./07-ontology-comparison.md). For the design-and-voice cues that complement these structural borrows, see [09-design-influence.md](./09-design-influence.md).

---

## H. Confidence-bounded multi-round interview as proposal genesis

Their pre-build flow runs three rounds of clarifying questions, refining a working brief and a numeric confidence score (78% → 82% in the captured run) before any structural generation happens. The full Q-and-A transcript is preserved as an artefact labelled `Preserved as provenance`. We have no equivalent. Our `cflx apply` starts from a human-authored proposal.md; there is no AI-driven elicitation loop in front of it. Adopting an interview entrypoint as an optional `cflx interview` mode would let us capture intent before the architect-author step and persist the elicitation as a `research` artefact tied to the resulting `decision`. This is the strongest single borrow target. **Source:** screenshots 01, 02, 03 plus [02-workflow-genesis.md](./02-workflow-genesis.md).

## H. Causality pyramid as a navigation surface

Their causality lens renders a five-tier pyramid (System / Domain Technologies / Parts and Materials / Instruments and Connections / Knowledge Foundation). Each tier surfaces gaps as inline "GAP" tiles with a needs-decomposition status, and the user can refocus the pyramid on any node. The pyramid is genuinely good as a learnable navigation primitive: a non-specialist sees "what depends on what" without having to read the full graph. We have a richer two-chain topology but no equivalent visual scaffold. Adopting the pyramid as a reading lens (not as kernel data) would give us a learnable surface for the authority chain. **Source:** screenshots 30, 31, 33, 36, 37 plus [marketing/concepts.md](./site-pages/marketing/concepts.md). **Priority H.**

## H. Three-axis fidelity radar with prose nudge banners

Their Completeness lens scores every node on three axes: Entities (children defined), Processes (behaviors defined), Relationships (interfaces and requirements defined). The chart is paired with a yellow inline banner that translates the numeric finding into a plain-English diagnostic ("the model knows what it is, but not what it does") plus a one-click `Fix with AI →` action. This is two patterns combined: the multidimensional scoring (refusing a single completeness number) and the auto-generated prose nudge that names the deficiency. Both are borrowable independently. The scoring axes map onto our blueprint primitives plus contracts plus state-of-evidence. The prose-nudge surface is a thin layer over a templated string but the UX is strong. **Source:** screenshots 13, 14, 15, 16, 20, 45 plus [marketing/concepts-model-completeness.md](./site-pages/marketing/concepts-model-completeness.md). **Priority H.**

## H. Quality Check panel with errors / warnings / info severity buckets

A dedicated `Quality Check` surface aggregates findings across the whole model into typed severity buckets (errors / warnings / info) with a single headline count, filter chips, and per-row `Fix` actions. Findings are grouped by category (`COMPLETENESS`, `TRACEABILITY` are the two captured siblings). This is a portable pattern for any spec linter UI and maps directly onto our reconciler's findings surface. Our reconciler already produces structured findings; we lack a surface that aggregates them with severity buckets, scope toggles ("Entire Model" vs "This Node"), and inline remediation. A `cflx check` command with `--json` plus a webui surface would close the gap. **Source:** screenshot 61. **Priority H.**

## M. Verification methods taxonomy (Test / Analysis / Demonstration / Inspection)

The classical V&V four-method vocabulary is exposed as a closed enum on every verification record. We have one specific operationalisation (the `cflx accept` gate battery) which is mostly Test plus Analysis. Adopting the four-method taxonomy on our `decision` and `contract` artefacts would give us a richer language for what evidence a contract requires before it counts as verified. The taxonomy is a label change with a small downstream reconciler change; the borrow is mostly vocabulary. **Source:** screenshot 28 plus [docs/verification.md](./site-pages/docs/verification.md). **Priority M.**

## M. Verification status taxonomy (Passed / Planned / Draft / Failed / Blocked)

Their verification record carries a status enum spanning lifecycle states: Draft (under construction), Planned (committed but not run), Passed, Failed, Blocked. We currently treat verification as a binary at commit time (the gate either passes or fails). Adopting a five-state lifecycle for verification artefacts would let us model planned-but-not-yet-run verifications as first-class, useful for phase planning where a verification is committed in a phase but executed later. **Source:** screenshot 28 plus [docs/verification.md](./site-pages/docs/verification.md). **Priority M.**

## M. Suggest Trace Links as a separate affordance

Their traceability matrix view distinguishes manual `+ Add Link` from AI-driven `+ Suggest Links`. The suggest action runs through the same four-stage pipeline (Route → Context → Generate → Validate) as other AI specialist runs. The split is honest: manual is fast and known, suggested is slower and probabilistic. We have provenance and authority chain edges but no `cflx suggest-links` affordance. Adding one would let an AI specialist propose cross-cutting edges (contract ↔ research, decision ↔ source) for human review, separately from the deterministic edges declared in the blueprint. **Source:** screenshots 29, 49. **Priority M.**

## M. Per-call usage panel (tokens, cost, model, category breakdown)

The Usage panel surfaces a per-call ledger: provider, model, category (Pipeline / Inception / Visual / 3D Mesh), subcategory (Router classification / Architect specialist / etc.), tokens, dollar cost, latency. Aggregated across 26 calls in the fixture: $1.13 total. Our cflx already records per-phase usage telemetry but we do not surface it as a queryable rolled-up view. Adding a `cflx usage` command with category/subcategory aggregation, plus a webui Usage panel, would give the user the same transparency. The pattern is portable: separate `pipelineTrace` (per-changeset audit) from `usage.json` (denormalised billing view). **Source:** screenshot 58 plus [_export-analysis.md §11](./_export-analysis.md). **Priority M.**

## M. Two-tier export (Quick local versus AI-powered narrative)

Their settings export splits exports by where the work happens: local-and-instant (JSON, Markdown, Requirements CSV) versus AI-rendered-and-async (PPTX, DOCX) saved to project Assets. The split is not by format but by sync-versus-async and download-versus-internal-asset. Maps onto our surface as "raw graph export" (JSON, map.md, contract bundles) versus "narrative artefacts" (decision rationales rendered to PDF or slide deck). The "saved to your Assets" pattern keeps generated documents inside the provenance chain rather than letting them escape to the filesystem; this is the harder part to copy and the more valuable. **Source:** screenshot 64 plus [export-from-settings/offshore-survey-usv-rov-2026-04-28.md](./export-from-settings/offshore-survey-usv-rov-2026-04-28.md). **Priority M.**

## M. Suggestion chips on node detail (Mass Budget, Power Budget, Design Life, Technology Maturity, Cost Budget)

Below the typed property rows on a node, a row of `+ ...` chips offers AI-staged candidate properties for one-click addition (`+ Mass Budget`, `+ Power Budget`, `+ Max Speed`, `+ Design Life`, `+ Technology Maturity`, `+ Cost Budget`, plus `Show all (13)`). Adopt-or-discard at the property level rather than batch. The mechanism is a per-type schema plus context-aware ranking. We could mirror this for our contracts: a node's blueprint type implies a candidate set of obligations the user might want; chips offer one-click addition. **Source:** screenshot 35 plus [04-node-model.md](./04-node-model.md). **Priority M.**

## M. AI badge on every generated value

Every property value carries an inline `· ai ·` token in its category line. Authored values would lack the tag. The user knows at a glance which numbers are commitments and which are AI proposals awaiting validation. Maps onto our reconciler's job of distinguishing the authored blueprint from the generated map. We currently track this distinction at the artefact level (declared vs scanned) but not at the property-or-field level inside an artefact. Per-field AI tagging would let the user see "the title was authored, but the rationale was AI-suggested" inside a single decision artefact. **Source:** screenshots 09, 10, 35. **Priority M.**

## M. Causal Position widget per node

Each node detail panel shows a `CAUSAL POSITION` section with two complementary framings: `Prerequisite for: <upstream>` (what depends on this) and either a `System capstone` tag with an `Enabled by:` list (when looking at the root) or a `No children, pyramid layer incomplete` warning (when leaves stop too early). Pairs naturally with the causality pyramid borrow above. Implements as a derived view over our authority chain edges: for any node, surface "what contracts require this" and "what this contract enables". **Source:** screenshots 36, 37. **Priority M.**

## M. Requirements panel This-node versus All-descendants scope toggle

A two-segment pill toggle on the Requirements panel switches between viewing only the requirements directly attached to the current node and viewing all requirements in the subtree below. The header shows aggregate counts split by category (`3 items / 2 performance / 1 functional`). This is a clean way to scope graph queries to local-or-subtree without forcing the user to choose at navigation time. We have neighbourhood queries but no equivalent surface affordance. **Source:** screenshot 44. **Priority M.**

## M. Three-axis traceability matrix (Architecture / Behavior / Verification)

The traceability matrix exposes three coverage axes: Architecture (`% traced`), Behavior (`% covered`), Verification (`% verified`). Each requirement gets a row with `SATISFIED BY` and `VERIFIED BY` columns and a status pill (Fully traced / Partial / Gap). Coverage stats sit at the top of each axis tab. We track related concepts (synced/ghost/orphaned, verified/external/unverified) but as artefact states, not as coverage axes per requirement. Adopting the per-axis percentage view would give us a higher-level rollup that an external reviewer can scan. **Source:** screenshot 49. **Priority M.**

## M. AI Reasoning panel with token and cost transparency

Every applied changeset has a per-batch AI reasoning panel that explains the specialist's selection logic across the whole batch (e.g., "I selected six requirements that collectively cover ..."). The footer shows aggregated tokens and dollar cost (`~13.2k tokens · ~$0.0790`). Pairs with the per-stage pipeline trace and the named-specialist role badges. Worth borrowing as part of our cflx archive output: every applied phase carries an explanation of why the agent chose this approach plus per-stage cost. **Source:** screenshots 22, 25, 26 plus [06-command-palette.md](./06-command-palette.md). **Priority M.**

## L. Systemigram with Generate Panel and AI-generated narrative layer

The narrative tab generates a systemigram (Boardman-style narrative diagram) with verb-phrase-labelled edges, plus a `mainstaySentence`, `cards[]`, and per-connection `verbPhrase`. We have no analogue. The pattern is portable; the value is genuine for stakeholder communication. The reason this is L rather than H is that it sits squarely in the "narrate the graph" capability that does not gate any commit. Adoption would be additive expressivity rather than load-bearing. **Source:** screenshots 54, 56 plus [marketing/concepts-narrative-lens.md](./site-pages/marketing/concepts-narrative-lens.md). **Priority L.**

## L. 2D gallery with six render style kits (Photorealistic / Blueprint / Concept Art / Clay / Isometric / Exploded)

Their Visualize surface offers six named render styles for AI-generated concept imagery, each with a category label (RENDER / TECHNICAL / ARTISTIC / SCHEMATIC). The pattern is portable: any system that generates derived visual artefacts could expose a style kit. For us, the equivalent might be "render the authority chain as a Mermaid graph with style X" or "render the decision rationale as a slide-deck template Y". The category framing is the borrowable shape, not the specific styles. **Source:** screenshot 41. **Priority L.**

## L. 3D viewer with named pipeline stages (Preparing context / Loading concept image / Waiting for specialist / Validating code / Building geometry)

The 3D mesh generation surface exposes a five-stage progress indicator with explicit stage names that go beyond "thinking..." or a percentage bar. Pairs with their general transparency commitment (visible models per stage, wall-clock per stage, honest "typical" range copy). Adopting named-stage progress for our long-running cflx operations is a small UX upgrade with high trust return. The pattern beats hidden percentage bars. **Source:** screenshots 42, 43, 62. **Priority L.**

## L. Genesis transcript as immutable provenance record

The `Project Genesis · 3 rounds · Preserved as provenance` panel surfaces the full Q-and-A transcript from the inception interview, with `KEY DECISIONS` numbered, before any structural generation. This is partially already covered by the H-priority "interview as proposal genesis" entry above, but stands as a distinct borrow if we adopt the interview without the durable transcript. The persistence layer (queryable artefact vs UI display) is **unknown** from the public surface; what we adopt is the user-facing labelling and the hard separation between elicitation and generation. **Source:** screenshot 03. **Priority L.**

## L. Domain-flex via AI normalisation, not schema enrichment

A single user experiment fed a software-domain PRD (OpenSpine, an event-driven runtime substrate for governed agents, with Lyra as the first product) into the platform and got a same-shape Round 1 of 3 interview back, with 72% confidence and software-flavoured decomposition questions about deployment target and operator profile. The screenshot is at `screenshots/65-software-domain-prd-decomposition.png`. This suggests, with a sample size of 1, that their domain flexibility is supplied by the AI specialists at inference rather than by a richer schema. We do not yet know whether our framework (kernel-fence plus AI assistance plus typed artefact taxonomy) achieves comparable domain flexibility, or whether forcing a richer schema would help or hurt. Validating this would require at least three distinct-domain experiments on getcairn.dev to see whether the decomposition retains software-domain shape downstream or homogenises into a hardware silhouette. Log as observation. Do not adopt or reject yet. **Source:** screenshot 65. **Priority L (low confidence, low priority).**

## Skip: 3D mesh generation pipeline

Their 3D mesh pipeline is genuinely impressive (description plus concept image plus Claude Vision plus a custom MeshBuilder DSL plus glTF export). The captured fixture shows a 14,784-vertex tugboat model rendered in-app. We are not in the geometry domain. The borrow would not fit our two-chain topology and would require a substantial scope expansion. Skip; revisit only if we expand into hardware-design support.

## Skip: Six-requirement-type taxonomy as our contract type

Their requirement type enum (functional / performance / interface / safety / environmental / constraint) is closed and aerospace-flavoured. Our `contract` is broader (interface contracts in code, declared obligations in non-code domains, etc.). Mapping our contracts onto six aerospace-shaped types would lose the broader scope. Their taxonomy is right for them and wrong for us. Skip.

## Skip: Genesis confidence-score gate as a hard threshold

Their confidence pill (78% → 82%) is plausibly a soft indicator and possibly a hard gate (the public surface does not reveal which). Adopting a hard confidence gate on our `cflx interview` mode would force the AI to commit to a number that is not meaningfully calibrated. Soft indicator only, if we adopt the interview at all. Skip the hard-gate variant.

## Skip: Per-property unit registry at the kernel

Their `units[]` collection plus `dataTypes[]` plus `params[].unitId` references give them dimensional metadata at the property level. Our scope is broader (code plus non-code domains) so a units registry doesn't fit cleanly at the kernel. Belongs in a domain extension if we adopt anything; the kernel itself should not carry SI factors. Skip at the kernel.

## Skip: Twelve-lens fixed taxonomy

Their docs describe twelve named analytical lenses (Overview, Brief, Visuals, Requirements, Architecture, Causality, Completeness, Narrative, Dendritic, Behavior, Verification, Operational). The running app surfaces ten as tabs. The taxonomy is closed and product-specific. Our neighbourhood and query system is programmatic and extensible. Adopting their fixed lens set would constrain our query surface for no gain. Skip.
