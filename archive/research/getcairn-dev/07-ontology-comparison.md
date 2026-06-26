# Ontology comparison: getcairn.dev versus our framework

## What this is

A side-by-side mapping of getcairn.dev's data model against our two-chain provenance/authority topology. Sourced primarily from the export-parse stronghold ([_export-analysis.md](./_export-analysis.md)) backed by 34 screenshot analyses and the live offshore-survey project export.

For the product context this lives in, see [01-product-overview.md](./01-product-overview.md). For the borrow recommendations that follow from this comparison, see [08-borrow-list.md](./08-borrow-list.md).

## 1. Node taxonomy

### Their shape

Two surface types in the wild, plus latent options.

| Type | Count in fixture | Parent | Notes |
|---|---|---|---|
| `system` | 1 | null (root) | Carries `properties.brief.sections[]` and `_narrativeAnalysis`. |
| `subsystem` | 6 | system | Carries `properties.params[]` (typed engineering parameters with units). |

The fixture decomposition stops at depth 2 (system → subsystem → leaves). Their docs reference a richer hierarchy that the export does not exercise: `System → Subsystem → Assembly → Part`, plus an `External` actor type for systems outside the design boundary (per their entity-types reference and the Assembly choice captured in the create-node modal, screenshot 46). Their glossary names six node types in total: System, Subsystem, Assembly, Part, External, plus the implicit per-node `properties.brief` that the system carries.

Field shape (uniform across types):

```
{ id, projectId, parentId, name, type, description, position,
  sortOrder, properties, createdAt, updatedAt, createdBy }
```

The same record shape applies to every node. Differences across types are carried in `properties` (a free-form bag), not in the schema.

### Our shape

We split the kernel into two distinct primitive families.

**Blueprint primitives** (the authority-chain spine for the structural graph):

- `system`: top-level boundary.
- `container`: deployable unit inside a system.
- `module`: code-level unit inside a container.
- `actor`: external role or boundary entity.

**Artefact direct types** (the provenance-and-authority record forms):

- `contract`: typed normative statements (interface definitions, requirements, obligations).
- `decision`: hinge artefacts carrying obligations both inward (provenance) and outward (authority).
- `todo`: deferred work declared into the model.
- `research`: curated evidence feeding decisions.
- `review`: human or agent assessments of code or artefacts.
- `source`: external evidence inputs (links, citations, papers).

### Net difference

Their taxonomy is **single-graph and uniform**: one node shape, six type values mostly differentiated by content. Our taxonomy is **two-family and typed**: blueprint primitives and artefact direct types serve different roles, and the kernel enforces the distinction.

Theirs is more uniform but less expressive. Ours is more expressive but requires the user to learn two families. Their `system`/`subsystem` decomposition tree is closer to our blueprint than to our artefacts; their `requirements[]`, `verifications[]`, and `interfaces[]` collections are closer to our artefact types (contract specifically) than to their nodes.

## 2. Edge model

### Their shape

There is no top-level `edges` array. Relationships are reified as foreign-key fields scattered across collections.

| Edge type | Carrier | Direction | Count in fixture |
|---|---|---|---|
| Decomposition | `nodes[].parentId` | child → parent | 6 |
| Requirement allocation | `requirements[].nodeId` | requirement → node | 9 |
| Requirement decomposition | `requirements[].parentReqId` | child req → parent req | 3 |
| Verification linkage | `verifications[].requirementId` | verification → requirement | 1 |
| Interface (subsystem ↔ subsystem) | `interfaces[].sourceNodeId` + `targetNodeId` | source → target | 2 |
| Signal (intra-interface) | `interfaces[].signals[]` | within an interface | 8 |
| Param to unit | `params[].unitId` | param → unit | many |
| Asset linkage | `assetRegistry[].linkedNodeId` | asset → node | 2 |
| Trace links (collection slot, empty in fixture) | `traceLinks[]` | declared cross-cutting edges | 0 |

The `traceLinks` collection is schema-reserved but unpopulated in the captured project. The screenshots show it being used as a separate plane (the "Suggest Trace Links" button on the trace matrix view, screenshot 49) with semantic types `satisfies`, `verifies`, `derives`, `depends_on` (per their docs).

### Our shape

We have explicit graph edges, declared in the blueprint and reified in the map. Edges carry both **provenance semantics** (where evidence came from: source → research → decision) and **authority semantics** (where rules flow: decision → blueprint → contract → code). Edge identity is content-addressable: an interface contract carries a hash that detects drift when the underlying code changes.

### Net difference

They model graph-shape relationships through field-level foreign keys and reserve a single `traceLinks[]` collection for the cross-cutting edges that don't fit in fields. We model the entire relationship surface as first-class typed edges with bidirectional chain semantics. Theirs is leaner and easier to serialise. Ours is heavier but lets the reconciler operate on the edge set as a graph rather than as a join across tables.

## 3. Requirement schema (their `requirements[]` versus our `contract` artefact)

### Their shape

Twelve fields. Schema observed:

```
id, nodeId, parentReqId, title, description, type, rationale,
acceptanceCriteria[], priority, sortOrder, createdAt, createdBy
```

Enums:

- `priority`: `must` (uniformly in fixture; schema likely supports `should`, `could`, etc.).
- `type`: `performance`, `functional`, `environmental`, `safety` observed. Their docs add `interface` and `constraint` (six total).
- `createdBy`: `ai` for all nine fixture requirements.

`acceptanceCriteria` is an array of plain-text strings. Quantitative thresholds are inlined into prose (e.g., "≥ 10 knots", "Hs ≤ 2.5 m"). `parentReqId` enables requirement-to-requirement decomposition, exercised in the fixture (REQ-007/008/009 derive from REQ-003).

### Our shape

A `contract` artefact. The kernel enforces typed schema with explicit obligation, evidence linkage, and an interface hash that detects drift. Our contracts carry the same notional fields (title, description, rationale, acceptance language) plus content-addressable hashing and authority-chain edges that bind the contract to the blueprint primitive it constrains.

### Net difference

Shapes are close. Both treat a requirement as a typed normative statement with rationale and acceptance criteria. Theirs is bespoke (a top-level array plus a `requirements[]` collection); ours is one of six artefact direct types in a unified kernel. Their requirement-to-requirement decomposition (`parentReqId`) is a useful pattern we currently express as authority-chain edges between contracts.

A practical note: their `acceptanceCriteria` strings inline thresholds inside prose. Our contract model can encode thresholds structurally, but in current practice we also tend to inline. The tooling difference is whether the threshold is queryable. Theirs is not (it's prose). Ours can be (depending on how the contract was authored).

## 4. Verification schema (their `verifications[]` versus our `cflx accept` evidence)

### Their shape

Nine fields:

```
id, requirementId, method, title, description, status, results, createdAt, createdBy
```

Enums:

- `method`: `test`, `analysis`, `demonstration`, `inspection` (per docs and screenshot 28).
- `status`: `passed`, `planned`, `draft`, `failed`, `blocked` (per their docs and the requirements-table screenshot).

The single fixture record is a user-authored placeholder ("Hahaha", status `draft`, method `test`). Verifications link only to a requirement, never to a node, asset, transition, or use case directly.

Critical limitation observed: `results` is a single free-text string. There is no `evidenceAssetId`, `evidenceUrl`, or structured proof field. Evidence in their model is narrative, not structured.

### Our shape

`cflx accept` runs a verification gate battery on every phase: `cargo build` (zero warnings), `cargo clippy` with `-D warnings`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, and `cflx.py validate <phase> --strict`. Evidence is tied to commits, CI runs, and tooling output. Our reconciler also computes interface hashes that gate commits when code drifts from the contract.

### Net difference

Their verification is **declarative and human-asserted**: the user (or AI) writes a description of what would prove the requirement and marks status. Our verification is **operational and tooling-bound**: the gate battery runs and either passes or fails on real artefacts. Their four-method taxonomy (Test/Analysis/Demonstration/Inspection) is the classical V&V vocabulary. Our gate is one specific instantiation of "Test" plus "Analysis". Their taxonomy is broader; ours is more concrete.

Their structural gap: no evidence linkage. Verification is a record but not a proof. We close the loop with the reconciler.

## 5. Trace-link schema

### Their shape

`traceLinks[]` is empty in the captured project. From their docs the link types are `satisfies`, `verifies`, `derives`, `depends_on`, plus user-defined custom types. Screenshots show:

- A dedicated **Traceability Matrix** view (screenshot 49) with three axes: Architecture (traced), Behavior (covered), Verification (verified). Coverage stats reported in the captured fixture: `0% traced, 0% covered, 11% verified`.
- A **Suggest Trace Links** button distinct from a manual **Add Link** button. Implies AI-assisted link discovery as a separate affordance.
- A trace-link agent run captured in the route-stage screenshot (29): the prompt requests "satisfies" trace links between a requirement and components.

### Our shape

Provenance-chain edges and authority-chain edges, declared in the blueprint, reconciled at scan time. Cross-cutting edges (e.g., contract ↔ research, decision ↔ source) are first-class. The map exposes them as queryable neighbourhood relationships. `interface hash` provides content-addressable drift detection that their system does not have.

### Net difference

Their trace-links are a typed-edge collection layered atop a foreign-key-heavy schema. Most of their cross-cutting structure is implicit in foreign keys; trace-links are reserved for "the rest". Ours is uniformly explicit edges with hashing. Their suggested-versus-manual link distinction (with an AI specialist for suggestion) is a UX pattern we lack and could borrow as a `cflx suggest-links` affordance.

## 6. Interface model and "interfaceHash"

### Their shape

Ten fields per interface:

```
id, projectId, sourceNodeId, targetNodeId, name, protocol, description,
signals[], createdAt, createdBy
```

Each signal: `{ id, name, dataTypeId, direction (in|out), rate }`. Rate is free-form ("continuous", "100 Hz", "on_change"). Protocol is free-form ("HVDC power + fibre-optic data", "Ethernet / IP"). DataType references a `dataTypes[]` registry.

The `interfaceHash` field exists only inside the system node's `_narrativeAnalysis`. Its value in the fixture is `"IF-001|IF-002"`, a pipe-joined list of interface ids. **It is not a content hash.** It identifies which interfaces participated in the narrative analysis, nothing more.

### Our shape

Interfaces are typed `contract` artefacts that carry a content-addressable hash. Drift detection: when the code that implements the contract changes shape, the hash mismatches and the pre-commit hook blocks the commit until the contract is updated or the code is reverted. The hash is the load-bearing primitive in our authority chain.

### Net difference

This is **the largest substantive divergence between the two models**. Their `interfaceHash` is an identifier list, not a checksum. They cannot detect interface drift from their export. Migrating it to a content-addressable hash would unlock our gating behaviour for them; it is an entire missing capability, not just a label difference.

Worth flagging cleanly: a reader scanning their schema and seeing the field named `interfaceHash` may assume drift detection is supported. It is not. The name is misleading.

## 7. Narrative analysis: their `_narrativeAnalysis` (no analogue here)

### Their shape

Attached only to the system root node:

```
{ generatedAt, interfaceHash, mainstaySentence, cards[], connections[],
  islands[], layout, mainstayPath }
```

- `mainstaySentence`: an English summary of the canonical causal chain through the system. Fixture value: "The Power Generation & Distribution energises and commands the ROV Vehicle, whose video and sonar returns the Autonomy, Control & Mission Payload edge-processes and compresses, which the Communications & Data Link relays as processed survey data..."
- `cards[]`: prose narrative cards with `{ title, body, highlightNodes[] }`.
- `connections[]`: `{ interfaceId, sourceNodeId, sourceNodeName, targetNodeId, targetNodeName, protocol, role, verbPhrase }` where `verbPhrase` is generated English ("energises and commands") and `role` is currently `mainstay`.
- `islands[]`: subgraphs disconnected from the mainstay path.
- `layout`, `mainstayPath`: positional and traversal metadata for the rendered systemigram.

Visible in screenshot 56 (the rendered systemigram with curved arrows and verb labels) and screenshot 54 (the pre-generation panel showing "Generate Systemigram" with the explanatory copy "the AI will analyze the project's interface graph to identify the mainstay transformation chain, classify connections, and produce a readable narrative").

### Our shape

We have no direct analogue. Our `map.md` is a structural snapshot. We do not generate a causality narrative or mainstay path through the graph.

### Net difference

This is **distinctive to them**. The pattern is portable: an AI-derived primary causal spine across the interface graph, rendered as both prose and a node-and-arc diagram with verb-labelled edges. We could compute an analogue across our blueprint plus contracts. See [08-borrow-list.md](./08-borrow-list.md) for the borrow rationale.

## 8. Causality pyramid versus our two chains

### Their shape

Their **pyramid of causality** is a five-tier vertical decomposition (verified across screenshots 30, 36, 37, plus their docs and Harney citation):

```
         System (capstone)
            ↑
     Domain Technologies
            ↑
       Components
            ↑
     Parts and Materials
            ↑
   Instruments and Connections
            ↑
    Knowledge Foundation
```

(Five layers below the capstone in their docs; the captured screenshots show the active labels SYSTEM, DOMAIN TECHNOLOGIES, PARTS & MATERIALS, INSTRUMENTS & CONNECTIONS, KNOWLEDGE FOUNDATION rendered as horizontal lanes on the canvas.)

The pyramid is sequential, AI-driven, and bottom-up-after-the-fact: the user starts at the capstone (the system) and decomposes downward; the lower tiers materialise as supporting structure.

The screenshots also show:

- **Causal Position widgets** (screenshots 36, 37): per-node "Prerequisite for X" or "System capstone, Enabled by Y" framing.
- **Gap detection** (screenshot 30): "GAP" tiles inline in the pyramid for unresolved layers.
- **Refocus pyramid on this node** action (screenshot 37): the pyramid view can re-center on any node.

### Our shape

Two chains meeting at a hinge:

- **Provenance chain**: Source → Research → Decision (evidence flowing in).
- **Authority chain**: Decision → Blueprint → Contract → Code (rules flowing out).
- **Hinge**: the Decision artefact carries obligations in both directions.

### Net difference

Their pyramid collapses evidence and norms into a single linear pipeline. Ours keeps them distinct. Their `genesis` (interview rounds) is roughly our Source plus Research. Their `architect` plus `requirements` plus `interfaces` specialists are roughly our Blueprint plus Contract. Their `causality` plus `narrative` are post-hoc explanation layers we do not currently model.

Their model has **no equivalent to Code** because there is no implementation surface in their export. The pyramid stops at "knowledge foundation" (citations, parameter constraints). Our authority chain anchors at Code; this is the wedge that lets us gate commits.

Their pyramid is genuinely useful as a **navigation surface**: it gives the user a mental scaffold for "what depends on what". Our two-chain framing is more precise but less visual. The pyramid is a strong borrow candidate as a navigation lens; see [08-borrow-list.md](./08-borrow-list.md).

## 9. Decisions: implicit in their `history[]`, first-class for us

### Their shape

There is no `decisions` collection. Decision events are recorded inside `history[]` changesets:

```
{ id, projectId, timestamp, author, summary, prompt, specialist, promptVersion,
  operations[], proposedBy, proposedAt, approvedBy, approvedAt, pipelineTrace }
```

Each changeset is a propose-and-approve event. The decision *event* is captured (who proposed, who approved, when). The decision *artefact* (rationale, alternatives, status, hinge obligations) is not first-class. Their `pipelineTrace` records the per-stage model and token usage, useful as audit but not as a decision record.

### Our shape

`decision` is one of six artefact direct types. It carries rationale, alternatives, hinge obligations both ways (provenance inward, authority outward), and a status. Decisions are queryable, citable, and bind the two chains.

### Net difference

This is the largest **ontology gap**, not just a labelling one. They capture the audit trail of changes; we capture the rationale and obligations of decisions as durable artefacts. A reviewer reading their `history[]` can reconstruct what changed when, but cannot read why a decision was made or what it obligates without re-reading the original prompts. Our decision artefacts make the rationale legible and queryable.

## 10. Two-tier export model versus our `cflx archive`

### Their shape

Captured in screenshot 64 and the export-from-settings folder. Two distinct exports, gated by API-key availability:

| Tier | Trigger | Outputs | Destination |
|---|---|---|---|
| Quick Export (Local) | No API key required | JSON, Markdown, Requirements CSV | Direct download |
| Professional Export (AI-Powered) | API key required, ~30s latency | PPTX presentation, DOCX report | Saved to project Assets |

Plus a third archive form: the `.cairn` directory bundle (project export) which includes `manifest.json`, `project.json`, `usage.json`, and `assets/`. The settings-export variant strips `history[]` and binary blobs to produce a "shareable" view (see [_export-analysis.md §13](./_export-analysis.md) for the full diff).

The split is not by **format** but by **where the work happens** (sync local vs async LLM) and **where the result lands** (filesystem vs project-internal Assets).

### Our shape

`cflx archive` consolidates phase artefacts into the canonical specs (`openspec/specs/<area>/spec.md`) and moves the change directory into `openspec/changes/archive/`. There is no "AI-rendered deliverable" tier. There is no project-internal asset library distinct from the filesystem.

### Net difference

Their export model **separates raw graph dump from composed deliverable**. Ours collapses both into "archive the phase". Their split has UX merit: a reviewer can grab a JSON for round-trip use or a DOCX for a stakeholder meeting from the same surface. Their "saved to your Assets" pattern keeps generated documents inside the provenance chain rather than letting them escape to the filesystem.

Worth borrowing the conceptual split. The two-tier framing maps cleanly onto our surface as "raw graph export" (JSON, map.md, contract bundles) versus "narrative artefacts" (decision rationales rendered to PDF, blueprint walkthroughs to slides). See [08-borrow-list.md](./08-borrow-list.md).

## 11. Auxiliary registries: units, dataTypes, useCases, states, transitions

Their schema reserves five further top-level arrays beyond the seven core collections.

| Their collection | Fixture count | Our analogue |
|---|---|---|
| `units` | 6 (kg, W, m/s, mm, m, Mbps) | None at kernel level. Domain-specific. |
| `dataTypes` | 1 (DT-001 "analog" / scalar / number) | None at kernel level. |
| `useCases` | 0 | Closest analogue: research artefact narratives. |
| `states` | 0 | None directly. State machines are in their docs but unused in fixture. |
| `transitions` | 0 | Same. |
| `simulations` | 0 | None. |

The captured units screenshot (51) shows a six-row registry with `dimension` and `SI factor` columns and a two-tier kind taxonomy (`si` base versus `derived`). The dataTypes screenshot (50) shows a `Category` plus `Base Kind` two-level typing scheme.

These are **typed-parameter infrastructure** that our contracts do not currently carry. Their `params[]` references unitId, which gives them dimensional metadata. Our contracts can encode units in prose but do not enforce dimensional consistency.

A note on `states` and `transitions`: their app has the schema slot but the fixture doesn't exercise it, so the precise semantics of state-machine modelling are inferred from docs only. Their docs describe states with `typicalDuration` plus `durationUnit` fields, and transitions with `guard`, `action`, and `trigger` fields, mapping to a behavioural-modelling primitive we have no kernel-level analogue for.

## 12. Domain genericity: observed but not validated

A single user experiment (captured at `screenshots/65-software-domain-prd-decomposition.png`) fed a software-domain PRD into the platform. The PRD described **OpenSpine**, "a self-hostable, event-driven runtime substrate for governed agents", with a first product called **Lyra** (a Telegram-controlled personal assistant whose first guarded workflow is selected-thread email reply drafting against Gmail). This is unambiguously a software and agent-architecture domain, not the hardware MBSE positioning getcairn.dev's marketing implies.

The platform accepted the input without complaint and ran it through the same `Round 1 of 3 / Shape the starting model` flow with the same UI scaffolding: a `CURRENT UNDERSTANDING` block, a `REFINED DESCRIPTION` with a 72% confidence pill, and decomposition questions phrased for the new domain ("What is the intended deployment target and operator profile", multiple-choice plus free-text).

What the schema can be observed to do here:

- The two node types (`system`, `subsystem`) are flat by design. The schema does not encode domain richness directly; the typed-parameter infrastructure (`units[]`, `dataTypes[]`) and the `_narrativeAnalysis` are populated at inference time by the LLM specialists.
- Domain specificity in the captured fixture (USV, ROV, COLREGS) appears in `properties.params[]` values and in the `description` prose, not in the schema. The same schema can presumably absorb software-domain content into the same fields.

This suggests, with a sample size of 1, that getcairn.dev's domain flexibility is supplied by the AI specialists at inference rather than by a richer schema.

Counter-evidence to weigh against this reading:

- Their public marketing positions the product squarely in MBSE for hardware (rovers, satellites, drones, USV/ROV). The software-domain run may be an off-label use that produces hardware-shaped outputs (subsystems with mass-and-power budgets) even when the input is software-shaped.
- The captured screenshot only shows Round 1 of the interview. Whether the downstream decomposition retains software-domain shape (services, message flows, identity primitives) or homogenises into a hardware silhouette is not visible from a single mid-interview screenshot.
- A 72% confidence pill in Round 1 is moderate, not high. The platform may be flagging that the input is unusual for its trained behaviour.

What this does and does not tell us about our own architecture:

- It does not tell us whether our framework (kernel-fence, AI assistance, six-direct-type artefact taxonomy) achieves the same domain flexibility through the same mechanism. Our typed artefacts encode more semantic structure per node than their two-type uniform schema; whether that structure helps or hurts cross-domain reach is an open question.
- Validating their domain genericity would require at least three distinct-domain experiments (hardware system, software substrate, organisational process) and a comparison of the resulting model shapes. One sample is insufficient.

Treat this as a hypothesis worth tracking, not as evidence of either system's relative flexibility. Logged in [working-notes.md](./working-notes.md) under "Observation: software-domain PRD ingestion (single sample)".

## Net assessment

**Their model is a single-graph MBSE substrate with strong UX scaffolding around AI-assisted authoring.** Every primitive is a node-or-record in one of about a dozen typed arrays. Cross-cutting structure is reified through foreign-key fields (`nodeId`, `parentReqId`, `requirementId`) plus a single `traceLinks[]` collection for the rest. The schema is approachable, serialisable, and easy to render. The `_narrativeAnalysis` and the systemigram are genuine creative additions that we do not match.

**Our model is a two-chain framework with kernel-typed artefacts and content-addressable interface hashes.** The provenance chain (Source → Research → Decision) and authority chain (Decision → Blueprint → Contract → Code) meet at a hinge that carries obligations both ways. The reconciler operates on the edge set as a graph. Drift is detected at the contract level; commits are gated when the implementation diverges from the declaration.

**Where each gets it right:**

Their model gets *sequential causal narration* right. The pyramid of causality, the `_narrativeAnalysis` mainstay path, the systemigram with verb-phrase edges: these turn a structural model into something a non-specialist can read. Our two-chain framing is precise but invisible at the UI surface. We could borrow the pyramid as a navigation lens and the mainstay path as a derived view without changing kernel semantics.

Their model also gets *AI provenance native* right. `pipelineTrace.stages[]` carries per-stage model name, input and output token counts, latency, and a success flag. Per-property `· ai ·` tagging tells the user at a glance which values are user commitments and which are AI proposals. We do this through cflx telemetry but not as kernel data; adopting `pipelineTrace`-shaped per-artefact metadata would harden our authority chain.

**Where ours has structural advantages:**

We have a Code layer. Their model anchors at "knowledge foundation" (citations, parameter constraints) and stops there. The output of their tool is documentation about a future system. The output of ours is a model that gates the commit producing the system. The wedge is real: drift detection requires content-addressable hashes plus a reconciler running against a build, neither of which exists in their current shape.

We also have first-class decisions. Their `history[]` records the audit trail of changes with `proposedBy`/`approvedBy`/`approvedAt`. Our `decision` artefact captures rationale, alternatives, hinge obligations, and status as a queryable record. A reviewer reading our archive understands not just what changed but why and what it obligates. This is one of our largest expressive advantages.

We have ghost/synced/orphaned detection plus verified/external/unverified evidence states. Their model carries no equivalent of "declared but not yet implemented" or "implemented but no longer declared". Our reconciler surfaces those states; their schema does not encode them.

**Where the gap goes either way:**

Their typed-parameter infrastructure (`units[]`, `dataTypes[]`, `params[]` with unitId references) is genuinely useful for systems engineering and we have no kernel-level equivalent. Our scope is broader (code plus non-code domains) so a units registry doesn't fit cleanly at the kernel; it would belong in domain extensions if we adopted it.

Their `_narrativeAnalysis` could be implemented over our model. The shape of the input (interface graph plus typed edges) is compatible. The shape of the output (mainstay sentence plus verb-labelled connections plus narrative cards) is generic. We do not currently have a "narrate this graph" capability and gain expressivity by adding one.

The bottom line is that **the two systems are not competitors**. Theirs is an AI-assisted MBSE workspace anchored at human-readable documentation. Ours is a developer-and-agent framework anchored at gated commits. They share a philosophical core ("the model is the artefact, not the conversation") and diverge sharply on what "the artefact" runs against.
