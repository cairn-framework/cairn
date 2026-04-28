# getcairn.dev Export Parse Scout Analysis

Source: `docs/research/getcairn-dev/offshore-survey-usv-rov-0.1.0.cairn/`
Snapshot date: 2026-04-28
Scope: ontology extraction + comparison against the CAIRN framework.

---

## 1. Manifest

`manifest.json` (219 bytes, full content):

| Field | Value |
|---|---|
| `version` | `1.0` (export envelope version) |
| `exportedAt` | `2026-04-28T13:54:40.587Z` |
| `projectName` | `Offshore Survey USV-ROV` |
| `projectVersion` | `0.1.0` |
| `assetCount` | `2` |
| `totalAssetSizeBytes` | `943436` |
| `modelSizeBytes` | `198039` |

No feature flags, no schema-version fields beyond `version: "1.0"`. The deeper `formatVersion` / `schemaVersion` fields live inside `project.json` (see section 2). The `.cairn` extension is a directory bundle, not a single file.

---

## 2. Top-level shape of `project.json`

`formatVersion: 1`. `project.schemaVersion: "1.0.0"`. Root is a flat object with 19 keys, each holding either a metadata sub-object or a top-level array of records:

| Root key | Shape | Count |
|---|---|---|
| `project` | object (id, name, version, schemaVersion, rootNodeId, settings) | 1 |
| `genesis` | object (originalDescription, rounds[], finalDescription, buildResults) | 1 |
| `formatVersion` | scalar `1` | n/a |
| `nodes` | array of node records | 7 |
| `requirements` | array | 9 |
| `verifications` | array | 1 |
| `interfaces` | array | 2 |
| `traceLinks` | array | 0 |
| `dataTypes` | array | 1 |
| `units` | array | 6 |
| `useCases` | array | 0 |
| `states` | array | 0 |
| `transitions` | array | 0 |
| `simulations` | array | 0 |
| `assetRegistry` | array | 2 |
| `meshAssets` | array | 1 |
| `visualMetas` | array | 1 |
| `visualSeries` | array | 1 |
| `history` | array of changeset records | 7 |

Graph totals derivable from this snapshot:

- Total nodes (entities of any kind): 30 across all collections.
- Tree-graph nodes (system + subsystems): 7 (1 system, 6 subsystems).
- Edges / explicit links: 8 (6 parent-child via `parentId` + 2 interfaces). Trace-links collection is empty for this project.
- Max graph depth: 2 (system root → subsystem leaves). No grandchildren exist; the model is a one-deep decomposition.

The model is decidedly **flat in graph terms but rich in side-channel arrays**. Cross-cutting concerns (requirements, verifications, interfaces, narrative) live in their own top-level lists and reference nodes by id. There is no edges/links list per se: relationships are reified through `parentId`, `nodeId`, `requirementId`, `sourceNodeId`/`targetNodeId`, etc.

`project.settings.namingPrefixes` codifies the id namespace: `REQ-`, `IF-`, `S-`, `T-`, `VER-`, `UC-`. Subsystem ids in this export use `SUB-<name>` and the system uses `SYS-<slug>`, both outside the configured prefix table (purpose unclear; flagged for screenshot cross-check).

---

## 3. Node taxonomy

`nodes` records carry a `type` field (NOT `kind`; the Sauron brief asked for `kind`/`type`, only `type` exists). Common shape:

```
{ id, projectId, parentId, name, type, description, position,
  sortOrder, properties, createdAt, updatedAt, createdBy }
```

Distinct node types:

| Type | Count | Parent shape | Has `properties.brief` | Has `properties.params` | Has `_narrativeAnalysis` |
|---|---|---|---|---|---|
| `system` | 1 | parentId = null | yes | no | yes |
| `subsystem` | 6 | parentId = system | one of six does | all six | no |

There are no `module`, `actor`, `external_system`, or `container` types in this export. The decomposition stops at "subsystem". Concept-art and 3D mesh attach to the `system` node, not subsystems.

### Example payload (subsystem) — redacted

```jsonc
{
  "id": "SUB-USV",
  "projectId": "proj_moin7ykivqt3",
  "parentId": "SYS-o0bcqk",
  "name": "USV Platform",
  "type": "subsystem",
  "description": "...prose...",
  "position": { "x": 0, "y": 0 },
  "sortOrder": 0,
  "properties": {
    "params": [
      { "key": "mass_budget",         "label": "Mass Budget",         "value": 8000, "unitId": "UNIT-001",
        "category": "physical",      "source": "ai", "description": "..." },
      { "key": "power_budget",        "label": "Power Budget (Hotel Load)", "value": 15, "unitId": "UNIT-002",
        "category": "electrical",    "source": "ai", "description": "..." },
      { "key": "transit_speed_max",   "label": "Max Transit Speed",   "value": 10, "unitId": "UNIT-003",
        "category": "performance",   "source": "ai", "description": "..." },
      { "key": "sea_state_transit_max","label": "Max Transit Sea State","value": 6, "unitId": null,
        "category": "environmental", "source": "ai", "description": "..." }
    ]
  },
  "createdAt": "...", "updatedAt": "...", "createdBy": "ai"
}
```

### Example payload (system) — abbreviated

```jsonc
{
  "id": "SYS-o0bcqk",
  "parentId": null,
  "name": "Offshore Survey USV-ROV",
  "type": "system",
  "description": "...the canonical refined description...",
  "properties": {
    "brief": {
      "sections": [
        { "id": "purpose",        "title": "Purpose & Scope", "content": "...", "mode": "auto", "sourceRefs": [], "fingerprint": "" },
        { "id": "capabilities",   "title": "Capabilities",    "content": "..." },
        { "id": "interfaces",     "title": "Interfaces",      "content": "..." },
        { "id": "constraints",    "title": "Constraints",     "content": "..." },
        { "id": "assumptions",    "title": "Assumptions",     "content": "..." },
        { "id": "openQuestions",  "title": "Open Questions",  "content": "..." }
      ]
    },
    "_narrativeAnalysis": { /* see section 9 */ }
  }
}
```

`properties.params[]` is the universal subsystem parameter slot (key, label, value, unitId, category, source, description). `properties.brief.sections[]` is the universal narrative slot. `_narrativeAnalysis` (see section 9) is the AI-generated causality digest.

---

## 4. Edge / relationship taxonomy

There is no top-level `edges` array. Relationships are reified across the schema:

| Edge type | Source → Target | Count | Carrier |
|---|---|---|---|
| `parentId` (decomposition) | subsystem → system | 6 | `nodes[].parentId` |
| `nodeId` (requirement allocation) | requirement → node | 9 | `requirements[].nodeId` |
| `parentReqId` (requirement decomposition) | requirement → requirement | 3 | `requirements[].parentReqId` (REQ-007/8/9 → REQ-003) |
| `requirementId` (verification ↔ requirement) | verification → requirement | 1 | `verifications[].requirementId` |
| `interface` (subsystem ↔ subsystem) | subsystem → subsystem | 2 | `interfaces[]` (sourceNodeId/targetNodeId) |
| `signal` (intra-interface payload) | within an interface | 8 | `interfaces[].signals[]` |
| `unitId` reference | param → unit | many | `params[].unitId` |
| `dataTypeId` reference | signal → datatype | 8 | `signals[].dataTypeId` |
| `assetRegistry.linkedNodeId` | asset → node | 2 | both linked to `SYS-o0bcqk` |
| `meshAssets.{nodeId,scriptAssetId,gltfAssetId,conceptImageAssetId}` | mesh ↔ node + asset | 1 | mesh-osom58 |
| `visualMetas.{assetId,seriesId,nodeId,styleKitId}` | visual ↔ asset+node | 1 | vis-mmuvdz |
| `visualSeries.{nodeId,styleKitId}` | series → node | 1 | vs-6kj152 |
| `traceLinks` | (suggested explicit edge collection) | 0 | empty in this export |

Direction semantics for the populated edges:

- `parentId`, `parentReqId`: child references parent. Tree direction is up.
- `requirements.nodeId`: requirement allocates against a node (allocation, not decomposition).
- `interfaces.sourceNodeId → targetNodeId`: directed; semantics reinforced by `signals[].direction = "in" | "out"` from the source's perspective.
- `verifications.requirementId`: verification proves a requirement.

---

## 5. Requirement schema

Fields (all 12 keys observed in this export):

```
id, nodeId, parentReqId, title, description, type, rationale,
acceptanceCriteria[], priority, sortOrder, createdAt, createdBy
```

Enums observed in this dataset:

| Field | Values seen |
|---|---|
| `priority` | `must` (9 of 9). Schema likely supports more, but only `must` appears. |
| `status` | absent on requirements; status only appears on `verifications`. |
| `type` | `performance` (5), `functional` (2), `environmental` (1), `safety` (1). Likely additional values per schema (e.g., interface, regulatory) but unseen here. |
| `createdBy` | `ai` for all requirements. |

`acceptanceCriteria` is an array of plain-text strings (no structured measurement units inside the criterion). Quantitative thresholds are inlined into the prose ("≥ 10 knots", "Hs ≤ 2.5 m"). `parentReqId` enables requirement-to-requirement decomposition; in this export the LARS subsystem requirements (REQ-007, REQ-008, REQ-009) all derive from the system-level REQ-003.

### Example

```jsonc
{
  "id": "REQ-001",
  "nodeId": "SYS-o0bcqk",
  "parentReqId": null,
  "title": "USV Transit Speed",
  "description": "The Offshore Survey USV-ROV shall achieve a sustained transit speed of not less than 10 knots in sea states up to and including Sea State 4 (significant wave height Hs <= 2.5 m).",
  "type": "performance",
  "rationale": "A minimum transit speed of 10 knots is required to reach survey sites 50-200 km offshore within an operationally acceptable transit window...",
  "acceptanceCriteria": [
    "The USV sustains >= 10 knots over a 30-minute measured run in SS4 conditions (Hs <= 2.5 m) during sea trials.",
    "Speed is measured as GPS-derived speed over ground averaged over the trial run, with no more than +/-0.5 knot variation."
  ],
  "priority": "must",
  "sortOrder": 0,
  "createdAt": "2026-04-28T14:30:00.000Z",
  "createdBy": "ai"
}
```

---

## 6. Verification schema

Fields (9 keys):

```
id, requirementId, method, title, description, status, results, createdAt, createdBy
```

Method and status enums (only one verification record exists in this export, so the enum domain is partially inferred from the brief context):

| Field | Values seen | Brief states canonical set |
|---|---|---|
| `method` | `test` | Test / Analysis / Demonstration / Inspection |
| `status` | `draft` | Passed / Planned / Draft / Failed / Blocked |

The single record is a user-authored placeholder ("Hahaha" description, status `draft`). No evidence linkage fields are present in the verification record itself: `results` is a single free-text string and there is no `evidenceAssetId`, `evidenceUrl`, or `evidenceArtefactId` in the schema. Evidence in this snapshot is therefore only narrative; binary or structured proof attachments are not modeled.

Verifications are linked to requirements via `requirementId`. There is no link to a node, transition, simulation, or use case from a verification record.

### Example (full record)

```jsonc
{
  "id": "VER-fp596o",
  "requirementId": "REQ-001",
  "method": "test",
  "title": "Own Verification",
  "description": "Hahaha",
  "status": "draft",
  "results": "",
  "createdAt": "2026-04-28T13:34:22.538Z",
  "createdBy": "user"
}
```

---

## 7. Trace-link schema

`traceLinks` is an empty array (`[]`) in this export. Schema cannot be inferred directly from this dataset. Direction and provenance fields (suggested vs manual) are not visible. Flagged for screenshot cross-check.

What we *can* say: the schema has a dedicated top-level slot for trace links separate from requirements, interfaces, and history. Given the surrounding ontology, it likely holds explicit cross-collection edges (e.g., requirement ↔ interface, requirement ↔ verification, requirement ↔ asset) that are not already implicit through foreign-key fields. The fact that the LARS work-stream populated requirements, interfaces, and verifications without producing any trace-links suggests trace-links may be opt-in or used for non-trivial cross-cutting links rather than the implicit ones.

---

## 8. Interface (ICD) schema

Fields (10 keys):

```
id, projectId, sourceNodeId, targetNodeId, name, protocol, description,
signals[], createdAt, createdBy
```

Each `signals[]` element:

```
{ id, name, dataTypeId, direction, rate }
```

Enums observed:

- `direction`: `in`, `out` (interpreted from the source node's perspective).
- `rate`: free-form string. Values seen: `continuous`, `100 Hz`, `10 Hz`, `1 Hz`, `on_change`. No structured rate object.
- `protocol`: free-form string. Values seen: `HVDC power + fibre-optic data`, `Ethernet / IP`. Not a closed enum.

`dataTypeId` references `dataTypes[]`. In this export only `DT-001 "analog" / scalar / number` is defined, so all 8 signals share that placeholder data type (effectively a stub). Real data-type richness (units, ranges, structured payloads) is not populated for this project.

### Example

```jsonc
{
  "id": "IF-001",
  "sourceNodeId": "SUB-PWR",
  "targetNodeId": "SUB-ROV",
  "name": "ROV Tether Power & Data",
  "protocol": "HVDC power + fibre-optic data",
  "description": "Carries high-voltage DC power (5-20 kW) and bidirectional fibre-optic data...",
  "signals": [
    { "id": "sig-001-1", "name": "tether_hvdc_power",     "dataTypeId": "DT-001", "direction": "out", "rate": "continuous" },
    { "id": "sig-001-2", "name": "rov_video_sonar_data",  "dataTypeId": "DT-001", "direction": "in",  "rate": "continuous" },
    { "id": "sig-001-3", "name": "rov_control_commands",  "dataTypeId": "DT-001", "direction": "out", "rate": "100 Hz" },
    { "id": "sig-001-4", "name": "rov_telemetry",         "dataTypeId": "DT-001", "direction": "in",  "rate": "10 Hz" }
  ],
  "createdAt": "2026-04-28T13:15:00.000Z",
  "createdBy": "ai"
}
```

There is no `interfaceHash` field on the interface record itself. A string of the form `IF-001|IF-002` does appear inside the system node's `_narrativeAnalysis.interfaceHash` field (concatenated id list, not a content hash). That is the only thing called a "hash" in the export.

---

## 9. Generation pipeline metadata

Two interlocking provenance surfaces:

### 9.1 `history[]` changesets (in-project audit log)

Each changeset record carries:

```
id, projectId, timestamp, author, summary, prompt, specialist, promptVersion,
operations[], proposedBy, proposedAt, approvedBy, approvedAt, pipelineTrace
```

`operations[]` items are CRUD entries: `{ op, collection, entityId, before, after }`. Op types observed: `create` (25), `update` (3). Collections touched: `nodes` (9), `requirements` (9), `units` (6), `interfaces` (2), `dataTypes` (1), `verifications` (1).

Specialists observed: `architect` (1), `brief` (2), `requirements` (2). Two changesets have `specialist: null` and `author: "user"` (manually authored verification + an unprompted entry); the other five are `author: "ai"` with `proposedBy: "ai:<specialist>"` and an explicit `approvedBy` step (most often `approvedBy: "user"` with an `approvedAt` timestamp). This is a propose / approve workflow, not auto-apply.

`promptVersion` namespacing observed: `architect-v1.0`, `brief-v1.0`, `requirements-v1.0`, `requirements-LARS-v1.0`. Versioned per-specialist prompts.

### 9.2 `pipelineTrace` (per-changeset run telemetry)

Present on the AI-authored changesets:

```
{ id, timestamp, userPrompt, stages[], totalDurationMs, totalTokensUsed,
  estimatedCost, outcome }
```

Stages observed in this dataset: `router` → `context` → `specialist` → `validator`. Always all four stages, in that order, for AI changesets that produced applied operations.

`outcome` observed: `applied`. (Other values likely exist but are unseen.)

Each `stages[]` entry: `{ stage, model, inputTokens, outputTokens, durationMs, success }`. Models observed: `claude-haiku-4-5-20251001` for `router`, `claude-sonnet-4-6` for `specialist`, `null` model for `context` and `validator` (deterministic, no LLM call).

### 9.3 `_narrativeAnalysis` (causality / story layer)

Attached only to the system root node (`SYS-o0bcqk.properties._narrativeAnalysis`):

```
{ generatedAt, interfaceHash, mainstaySentence, cards[], connections[],
  islands[], layout, mainstayPath }
```

- `mainstaySentence`: a single English sentence summarising the canonical causal chain across subsystems. In this project: "The Power Generation & Distribution energises and commands the ROV Vehicle, whose video and sonar returns the Autonomy, Control & Mission Payload edge-processes and compresses, which the Communications & Data Link relays as processed survey data..."
- `cards[]`: prose narrative cards with `{ title, body, highlightNodes[] }`. Each `highlightNodes` entry is `{ name, role }` where `role` is a free-form noun phrase (e.g., "Energy Source", "Decision Engine", "Data Relay").
- `connections[]`: `{ interfaceId, sourceNodeId, sourceNodeName, targetNodeId, targetNodeName, protocol, role, verbPhrase }`. `role` enum observed: `mainstay`. `verbPhrase` is generated English ("energises and commands"). This is the "causality" layer that makes the system narratable.
- `interfaceHash`: pipe-joined list of interface ids, not a cryptographic hash.

### 9.4 `genesis` (pre-build interview)

Captures the inception interview:

```
{ createdAt, originalDescription, rounds[], finalDescription, finalName,
  confidence, buildResults }
```

`rounds[]` holds the interactive Q+A: `{ round, questions[], refinedDescription, confidence, timestamp }` and `questions[]` carry `{ id, question, context, options[], answer }`. Three rounds totalling 12 questions in this export. `buildResults` records the post-build summary: `subsystemCount`, `interfaceCount`, `totalNodeCount`, `briefSectionCount`, `buildDurationMs`. `confidence` ranges 0..1; final value 0.82.

Pipeline categories from `usage.json` (cost-by-category): `inception`, `pipeline`, `visual`, `mesh`. Pipeline subcategories: `router`, `architect`, `brief`, `requirements`, `interfaces`, `causality`, `narrative`, plus `digest` (visual). Inception calls have no subcategory; mesh and visual aggregate at the category level.

---

## 10. Asset inventory

Listing only (no PNG/JPG/GLB content read):

| Filename | Size (bytes) | mimeType | Asset id | Category | Linked node |
|---|---|---|---|---|---|
| `assets/ast-6c1m9y.jpg` | 924,934 | `image/jpeg` | `ast-6c1m9y` | `visual` | `SYS-o0bcqk` |
| `assets/ast-hkr795.bin` | 18,502 | `text/javascript` (per registry) | `ast-hkr795` | `mesh` | `SYS-o0bcqk` |

Notes:

- The mesh asset is registered as `text/javascript` with filename `offshore_survey_usv-rov_mesh.js` but stored on disk with a `.bin` extension. Likely the JS source for a procedural mesh-builder DSL (the captured `meshAssets[0].promptUsed` snippet shows `mb.defineMaterial(...)` calls, consistent with a custom mesh-build sandbox).
- Both assets carry `source: "ai-generated"` and `createdBy: "ai"`.
- A `.DS_Store` (8,196 bytes) is present at the bundle root but is macOS metadata, not a CAIRN artefact.

`assetRegistry[]` schema fields: `id, projectId, name, fileName, mimeType, size, category, linkedNodeId, linkedArtifactId, tags[], description, source, createdAt, createdBy`. `linkedArtifactId` is null for both assets in this export (suggesting it can target sub-collection records like requirements or verifications; unused here).

---

## 11. Usage data

`usage.json` is a flat array of 26 metering records. Each entry:

```
{ projectId, timestamp, provider, model, category, subcategory,
  inputTokens, outputTokens, totalTokens, estimatedCostUsd,
  durationMs, label, metadata }
```

Aggregates for this project:

| Category | Calls | Total tokens | Total cost (USD) | Notes |
|---|---|---|---|---|
| `inception` | 4 | 17,930 | 0.1228 | Genesis interview rounds (Sonnet). |
| `pipeline` | 18 | 123,857 | 0.7101 | Router + specialist runs for architect, brief, requirements, interfaces, causality, narrative. |
| `visual` | 2 | 1,517 | 0.0429 | Haiku digest + Gemini image gen. |
| `mesh` | 2 | 19,335 | 0.2546 | Two Sonnet 3D mesh script generations (max output 8192 tokens each). |
| **Total** | **26** | **162,639** | **~1.130** | Single project run. |

Subcategory distribution: `router` (7), `requirements` (4), `architect` (2), `brief` (2), `causality` (1), `digest` (1), `interfaces` (1), `narrative` (1), null (7 covering inception, mesh, and image-gen which have no subcategory).

Providers: `anthropic` (25 calls), `gemini` (1, image generation).

Models: `claude-sonnet-4-6` (specialists, mesh, narrative, brief), `claude-haiku-4-5-20251001` (routers + visual digest), `gemini-3.1-flash-image-preview` (visual). Image generation has zero token counts and a flat $0.04 charge; the metadata field carries `{ aspectRatio, imageSize, transport }`.

There is no per-stage breakdown here (those live in `pipelineTrace.stages[]` inside `history[]`). `usage.json` is the billing/observability surface, denormalised. `pipelineTrace` is the in-project audit surface.

---

## 12. Mapping to our framework

| Their concept | Our concept | Notes |
|---|---|---|
| `nodes` of type `system` / `subsystem` | `blueprint` primitives: System / Container / Module / Actor | They have only two levels (system + subsystem). We have a richer four-tier blueprint taxonomy plus explicit Actor for external roles. Their decomposition stops where ours begins to bite. |
| `requirements[]` (with priority, type, rationale, acceptanceCriteria) | `contract` artefact | Shape is close: both are typed normative statements with rationale and acceptance criteria. Our `contract` is one of six artefact direct-types and is the kernel-typed form; theirs is a bespoke top-level collection with parent-child decomposition (REQ-007/8/9 → REQ-003). |
| `genesis.rounds[]` (Q+A interview, refinedDescription, confidence) | `research` artefact + Source layer of the provenance chain | Their inception captures elicitation history. We treat that as research (curated evidence) feeding into Decisions. They terminate inception in `finalDescription`; we would persist it as a research artefact tied to a hinge Decision. |
| Their explicit `decisions` | `decision` artefact (the hinge of provenance + authority chains) | **They have no `decisions` collection.** Decisions are implicit inside `history[]` changesets (proposed by AI specialist, approved by user with `approvedBy`/`approvedAt`). The decision *event* is captured; the decision *artefact* (with rationale, alternatives, status, hinge obligations) is not first-class. This is the largest ontology gap. |
| `verifications[]` (method: test/analysis/demonstration/inspection; status: passed/planned/draft/failed/blocked) | `cflx accept` evidence types | Shape parallels phase verification. They name the four classical V&V methods explicitly. Our `cflx accept` battery (cargo build, clippy, test, fmt-check, validate strict) is concrete and tooling-bound; theirs is declarative and human-asserted. They lack structured evidence linkage (no `evidenceAssetId`); we tie evidence to commits/CI. |
| `traceLinks[]` (empty in this export) | `provenance-chain` edges + `interface hash` | Their schema has the slot but does not populate it for this project. Implicit links via `parentReqId`, `nodeId`, `requirementId` cover most of what our provenance chain demands. They have no content-addressable `interface hash`: their `interfaceHash` is a pipe-joined id string, not a checksum. Drift-detection is therefore not possible from their export. |
| `interfaces[]` + `signals[]` (ICD) | `contract` artefact + `interface hash` | Shape maps cleanly to our interface-as-contract. They model signals with direction + rate + dataTypeId; we additionally compute a content hash to gate commits when the underlying code drifts. They have no equivalent gate. |
| Generation pipeline (router → context → specialist → validator → outcome) plus `history[]` propose/approve | `cflx apply` + `archive` flow | Their `propose → approve → applied` resembles our `apply → accept → archive`. `proposedBy: "ai:architect"` parallels our codex-driven `cflx apply`. `approvedBy: "user"` with `approvedAt` corresponds to a manual accept gate. They do not have an archive step that consolidates specs across phases (we do, into `openspec/specs/`). |
| `_narrativeAnalysis` (mainstaySentence, cards, connections with verbPhrase, islands, layout) | (No direct equivalent) closest to our `map.md` plus narrative consolidations | They generate a causal-story digest from the interface graph: a "mainstay" path through subsystems with English verb phrases. We generate `map.md` as a structural snapshot but do not currently produce a narrative-causal layer. **Distinctive to them.** |
| Their causality pyramid (genesis → architect → requirements → interfaces → causality → narrative) | Our two chains | Their pyramid is sequential, AI-driven, and bottom-up-after-the-fact. Our model has two chains meeting at the Decision hinge: provenance (Source → Research → Decision) flowing in, authority (Decision → Blueprint → Contract → Code) flowing out. They collapse evidence and norms into a single linear pipeline; we keep them distinct. Their `genesis` is roughly our Source + Research; their `architect` + `requirements` + `interfaces` are roughly our Blueprint + Contract; their `causality` + `narrative` are post-hoc explanation layers we do not model. They have no equivalent to Code, because there is no implementation surface in the export. |

### Asymmetric observations worth a sticky note

1. They model **AI provenance natively** (specialist, promptVersion, pipelineTrace, model, token counts, estimated cost). We do this through cflx telemetry, not as kernel data. Adoption of explicit `pipelineTrace` per artefact would harden our authority chain.
2. They have **no Code layer.** The output is documentation about a future system. Our authority chain anchors at Code; this is the wedge that lets us gate commits.
3. Their `_narrativeAnalysis.mainstayPath` is genuinely interesting: an AI-derived primary causal spine across the interface graph. We could compute an analogue across our blueprint + contracts.
4. Their `interfaceHash` is a name, not a hash. A migration to content-addressable hashes would let them detect drift the way we do.
5. Their `traceLinks` collection exists but is unpopulated. They appear to under-use explicit cross-cutting edges in favour of foreign-key fields. We rely heavily on explicit edges for the provenance chain.
6. Their requirement `priority` enum is uniformly `"must"` in this dataset; the schema may support more, but the project-level practice is single-tier. We do not currently model contract priority; their convention may be worth borrowing if user-facing.
7. Their `units` collection (kg, W, m/s, mm, m, Mbps) and structured `params[]` with unitId references give them **typed parameters**. Our contracts do not currently carry this dimensional metadata.

---

## 13. Project export vs Settings export: schema diff

Source for this section: `/Users/george/repos/cairn/docs/research/getcairn-dev/export-from-settings/offshore-survey-usv-rov-2026-04-28.json` (88 KB) compared against the project export at `offshore-survey-usv-rov-0.1.0.cairn/project.json` (198 KB).

### 13.1 Top-level key diff

| Key | Project export | Settings export | Notes |
|---|---|---|---|
| `_exportNote` | absent | present (string) | Settings adds: "Binary asset files (images, documents) are stored locally and not included in this export. Use the Asset Browser to download them individually." |
| `history` | present (7 changesets, ~110 KB) | absent | Settings export drops the entire mutation log, including `pipelineTrace`, `prompt`, `promptVersion`, `specialist`, `proposedBy`, `approvedBy`. |
| `formatVersion` | `1` | `1` | Same. |
| `project` | present | present | Identical (same id, name, version, schemaVersion, settings). |
| `genesis` | present | present | Identical structure (createdAt, originalDescription, rounds[], finalDescription, finalName, confidence, buildResults). |
| `nodes` | 7 | 7 | Same shape and ids. |
| `requirements` | 9 | 9 | Same. |
| `verifications` | 1 | 1 | Same. |
| `interfaces` | 2 | 2 | Same. |
| `traceLinks` | 0 | 0 | Same (empty in both). |
| `dataTypes` | 1 | 1 | Same. |
| `units` | 6 | 6 | Same. |
| `useCases` | 0 | 0 | Same. |
| `states` | 0 | 0 | Same. |
| `transitions` | 0 | 0 | Same. |
| `simulations` | 0 | 0 | Same. |
| `assetRegistry` | 2 | 2 | Same metadata records. Binary blobs not bundled per `_exportNote`. |
| `meshAssets` | 1 | 1 | Same. |
| `visualMetas` | 1 | 1 | Same. |
| `visualSeries` | 1 | 1 | Same. |

### 13.2 Record-level key sets (verified)

For every record collection that exists in both exports, the field set is byte-identical:

- `nodes[].keys` = `[createdAt, createdBy, description, id, name, parentId, position, projectId, properties, sortOrder, type, updatedAt]` (same in both).
- `requirements[].keys` = `[acceptanceCriteria, createdAt, createdBy, description, id, nodeId, parentReqId, priority, rationale, sortOrder, title, type]` (same in both).
- `verifications[].keys` = `[createdAt, createdBy, description, id, method, requirementId, results, status, title]` (same in both).
- `interfaces[].keys` = `[createdAt, createdBy, description, id, name, projectId, protocol, signals, sourceNodeId, targetNodeId]` (same in both).
- `assetRegistry[].keys` = `[category, createdAt, createdBy, description, fileName, id, linkedArtifactId, linkedNodeId, mimeType, name, projectId, size, source, tags]` (same in both).

Sampled REQ-001 from both exports is bit-for-bit identical (same description, rationale, acceptanceCriteria, timestamps).

The `_narrativeAnalysis` block on the system root node persists across both exports: its causal cards, mainstaySentence, connections, and `interfaceHash: "IF-001|IF-002"` value are present in the settings export too. This means the AI-derived narrative survives the "shareable" view; only the raw mutation log is stripped.

### 13.3 What the settings export omits vs adds

Omits (present only in project export):

- `history[]` (7 changesets) and everything reachable through it: `pipelineTrace`, `userPrompt`, `prompt`, `promptVersion`, `specialist`, `proposedBy`, `proposedAt`, `approvedBy`, `approvedAt`, `operations[].before`, `operations[].after`, per-stage model/token/duration telemetry.
- Binary asset payloads (the project export ships them in `assets/`; the settings export drops them and points the user at a manual asset-browser fetch via `_exportNote`).

Adds (present only in settings export):

- `_exportNote` advisory string.
- Sibling pretty-printed views (`*.md` and `*.csv`) that do not exist in the project bundle.

### 13.4 Hypothesis confirmed

The settings export is a "shareable / portable" view: it preserves the entire current-state graph (nodes, requirements, verifications, interfaces, datatypes, units, narrative analysis, asset metadata, genesis interview) but strips the audit trail and the binary blobs. A recipient cannot replay the build, recover the AI prompts, or recompute cost from the settings export alone. They can fully reconstruct the system model and re-render the markdown / CSV views.

This is consistent with the workflow split: project export is the engineering archive (full provenance for cflx-style replay or audit), settings export is the deliverable (dropped to an external reviewer or downstream tool).

---

## 14. CSV column inventory

`offshore-survey-usv-rov-2026-04-28.csv`: 9 data rows, single header row, 7 columns. The CSV represents the full requirements list with verification status rolled in.

### 14.1 Columns

| CSV column | Source JSON path | Notes |
|---|---|---|
| `ID` | `requirements[].id` | Direct. |
| `Title` | `requirements[].title` | Direct. |
| `Description` | `requirements[].description` | Long requirement prose, quoted where embedded commas exist. |
| `Type` | `requirements[].type` | `performance` / `environmental` / `functional` / `safety` in this dataset. |
| `Priority` | `requirements[].priority` | `must` for all rows. |
| `Node` | resolved `nodes[]` where `nodes[].id == requirements[].nodeId` then `.name` | The CSV uses the human-readable node *name*, not the node id. REQ-001 to REQ-006 carry "Offshore Survey USV-ROV" (system); REQ-007 to REQ-009 carry "Launch & Recovery System" (subsystem). |
| `Verification Status` | aggregated from `verifications[]` filtered by `requirementId` | REQ-001 shows `draft` (matches the single VER-fp596o record). All other requirements show `none` (i.e., absence of any matching verification). |

### 14.2 Fields they consider primary (and what they omit)

Included as primary: id, title, description, type, priority, owning node (by name), verification status.

Omitted from the CSV but present in JSON: `rationale`, `acceptanceCriteria[]`, `parentReqId`, `sortOrder`, `createdAt`, `createdBy`. The CSV is therefore a **review-grade summary** rather than a complete export. Notable omissions:

- `parentReqId` decomposition is invisible: REQ-007/8/9's link back to REQ-003 is not in the CSV. A reviewer cannot reconstruct the requirement tree from the CSV alone.
- `acceptanceCriteria[]` is the formally testable content; not exporting it makes the CSV unsuitable as a verification handover.
- No interfaces, no nodes-as-such, no verifications-as-rows. The CSV is requirements-only.

This shape suggests the CSV is intended for stakeholder review meetings or import into requirements-management tools that key off ID + title + type + priority, not for round-tripping or as a system-of-record.

---

## 15. Markdown-render mapping

`offshore-survey-usv-rov-2026-04-28.md` (8.8 KB) is the canonical "render project as doc" output. Each section maps to a specific JSON path:

| MD section / element | JSON source path | Render notes |
|---|---|---|
| Title `# Offshore Survey USV-ROV` | `project.name` | Direct. |
| `**Version:** 0.1.0` | `project.version` | Direct. |
| `**Schema:** 1.0.0` | `project.schemaVersion` | Direct. |
| `## System Overview` (3 prose paragraphs) | the `system` node's `description` field (i.e., `nodes[] where type == "system"` then `.description`) | Identical to `genesis.finalDescription`; appears that the system node's description is seeded from the genesis output. |
| `## Architecture` heading | structural; no direct field. | |
| `### <subsystem name>` (one per subsystem) | `nodes[] where type == "subsystem"` ordered by `sortOrder`; heading uses `.name`, body prose uses `.description` | All 6 subsystems are rendered, in `sortOrder` order (USV, PWR, ROV, LARS, COMMS, AUTO). |
| `> **Type:** subsystem · **ID:** SUB-XXX` | `nodes[].type` and `nodes[].id` | Renders as a blockquote line under each subsystem. |
| Per-subsystem **Interfaces** table | `interfaces[] where sourceNodeId == subsystem.id OR targetNodeId == subsystem.id` | Table columns: Interface (`name`), From → To (resolved source/target node names), Protocol (`protocol`), Signals (concatenated `signals[].name (direction)`). The same interface row appears under both endpoints (e.g., IF-001 shows on both Power Generation and ROV Vehicle subsystems). USV Platform and LARS subsystems have no Interfaces table because they are not endpoints of either IF-001 or IF-002. |
| `## Requirements` heading | structural | |
| `### Offshore Survey USV-ROV` group | `requirements[] where nodeId == SYS-o0bcqk`, sorted by `sortOrder` | Group heading is the *node name*, not the node id. |
| `### Launch & Recovery System` group | `requirements[] where nodeId == SUB-LARS` | Subsystem grouping appears only when there are allocated requirements. The other 4 subsystems (USV, PWR, ROV, COMMS, AUTO) have zero requirements allocated and therefore no group. |
| Requirements table columns | `requirements[].{id, title, type, priority}` | Direct. |
| `## Verification` table | `verifications[]` joined to `requirements[]` for the title prefix | Columns: `Requirement` (formatted `<id> (<title>)`), `Method` (`method`), `Status` (`status`), `Description` (`description`). |

### 15.1 JSON fields suppressed in the human-readable MD

Present in JSON, deliberately not surfaced in the MD render:

- `requirements[].description`, `rationale`, `acceptanceCriteria[]`, `parentReqId`. The MD shows only ID/title/type/priority. The full requirement text is in the JSON and CSV but not the MD: a reviewer reading the MD has the *names* of requirements, not their content. (Purpose unclear; flagged for screenshot cross-check. Plausible: the MD is an architecture overview, with requirements detail expected to be reviewed via the CSV or in-app.)
- `verifications[].results` and `createdAt`/`createdBy`. The MD verification table omits results and authorship.
- `nodes[].properties.params[]` (the entire typed-parameter set per subsystem with unit references). None of the subsystem mass/power/speed/sea-state values from `params[]` make it into the MD.
- `nodes[].properties.brief.sections[]` (purpose, capabilities, interfaces, constraints, assumptions, openQuestions). The system-level brief sections are NOT used to render the MD overview; the MD overview uses the system node's top-level `description` instead. The ROV Vehicle subsystem also has a `brief` block in JSON, none of which appears in the MD.
- `nodes[].properties._narrativeAnalysis` (cards, mainstaySentence, connections with verbPhrases, islands). The entire causality narrative layer is suppressed in the MD render.
- `interfaces[].signals[].dataTypeId` and `rate`. The MD signals column shows only `name (direction)`, dropping rate (e.g., `100 Hz`, `continuous`, `on_change`) and data type linkage.
- `assetRegistry[]`, `meshAssets[]`, `visualMetas[]`, `visualSeries[]`, `units[]`, `dataTypes[]`. No mention of any visual, mesh, or unit metadata.
- `genesis.rounds[]` (the inception interview Q+A). The interview history is dropped, even though the `finalDescription` is used.
- `traceLinks[]` (empty here, but no section is reserved for it). The MD has no Trace Links section at all in this render. If trace links were populated, the rendering behaviour is unclear from this dataset; flagged for screenshot cross-check.
- `useCases[]`, `states[]`, `transitions[]`, `simulations[]`. None populated, none rendered, no reserved heading. Whether these collections render when populated cannot be determined from this snapshot.

### 15.2 Net observation

The MD render is an **architecture overview document**, not a requirements specification or a verification report. It surfaces: project header, system narrative, subsystem decomposition, per-subsystem interface tables, requirements list (titles only), verifications list (single-line entries). It deliberately skips: full requirement prose, acceptance criteria, typed parameters, narrative analysis cards, units, data types, assets, mesh, and the genesis interview transcript. The CSV complements the MD by carrying the full requirement description but adds nothing about subsystems or interfaces. Together the MD + CSV approximate the "shareable deliverable" shape; the project-export JSON remains the only round-trippable artefact.
