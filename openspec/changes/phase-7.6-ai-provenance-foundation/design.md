# Design: Phase 7.6 AI Provenance Foundation

## References

- `docs/spec.md` §3 ("The two chains: provenance and authority"): canonical definition of the provenance chain (source to research to decision) and the authority chain (decision to blueprint to contract to code), and the hinge that joins them.
- `docs/spec.md` §3.4 ("Current-state authority"): defines the framework's role as "a fence around the authority chain and a navigator for the provenance chain". This phase ships the navigator's per-run trace and the fence's accept-time gate against unreviewed AI output.
- `docs/spec.md` §3.5 ("Layer ordering of enforcement, configuration, and AI"): the rule "AI assists authoring; AI does not substitute for the reconciler" is the load-bearing constraint behind the must-triage primitive.
- `docs/strongholds/getcairn-cross-check-A.md`: the cross-check that scoped this phase. Findings F1 to F8, recommendations R1 to R7, and decisions D1 to D7 are the source of truth for what this phase contains and why.
- `docs/strongholds/getcairn-cross-check-integrated.md`: cross-bundle synthesis confirming Bundle A and Bundle B parallel-shippable and slotting Bundle A as Wave 2 of the integrated roadmap.
- `docs/strongholds/oq4-phase9-rescope-timing.md`: timing analysis for the downstream phase-9 rescope commit, choosing Option B (rescope after this phase's design.md ratifies token-level identifiers).
- `openspec/specs/changes/spec.md`: existing change-isolation primitive that the suggested-edges queue file lives inside.
- `openspec/specs/cli/spec.md`: existing CLI ergonomic conventions (human and `--json` output modes; library-backed semantics).
- `openspec/specs/query/spec.md`: existing query-layer conventions for typed responses with stable schema versions.
- `openspec/conventions.md` §3 ("State Versioning"): every state file written by this phase begins with a `version` integer.
- `openspec/registries/error-codes.md`: the registry where `CC002` is allocated.
- `openspec/changes/archive/phase-2.6-terminology-rename/`: prior-art pattern for hybrid-change-type proposals that touch multiple existing capability areas plus add a new one.
- `openspec/changes/phase-7.5c-verification-states/`: prior-art pattern for a new capability area sibling to existing ones, with a new error code allocated in the same family.

## Two-chain framing

The four primitives shipped here are not arbitrary. Each maps to a specific spec §3 obligation.

- The trace sidecar (C6) lives on the **provenance chain** as a per-run record of how the workflow got from its inputs to its outputs. It is the navigator's evidence trail. Without it, the provenance chain has no per-run granularity once a phase archives.
- The suggested-edges queue (C8.a) lives on the **hinge** between provenance and authority. The entries record *proposed* authority-chain additions (new blueprint edges) along with their *provenance* (which AI run produced them). The two chains meet inside the queue file: provenance fields say "where did this come from", authority fields say "what does this propose to add".
- The accept-time block (C8.b) is the **fence** around the authority chain. It refuses to admit any hinge entry into the authority chain without explicit human assent. CLAUDE.md's third principle ("AI does not substitute for the reconciler") and `docs/spec.md` §3.5 are the upstream norms; the gate is the mechanically-checkable enforcement.
- The islands query (C5.a) is a **navigator** primitive over the authority chain. It exposes disconnected components so a reader can see where the chain breaks. AI-assisted authoring will routinely produce disconnected pieces; this primitive surfaces them.

This phase does not flatten the two chains into a six-layer stack. It adds primitives to each chain at the appropriate depth.

## Capability area: `provenance-foundation`

A new top-level capability area is added at `openspec/specs/provenance-foundation/spec.md` for the trace sidecar concept. The sidecar is per-archived-change but is NOT a kernel artefact (per CLAUDE.md's cairn/cflx separation). Putting it under `specs/changes` would falsely suggest the kernel owns it; putting it under `specs/cli` would conflate the data with its renderer. A new capability area is the architecturally honest move and matches the existing pattern of capability-shaped specs (`reconciliation`, `graph-explorer`, `multi-target`, `mcp`).

The area name `provenance-foundation` is preferred over alternatives (`cflx-trace`, `cflx-workflow`) for two reasons. First, the area covers the conceptual primitive (per-run evidence trail), not the tool that emits it; future workflow runners or reviewers can also consume this area. Second, the name aligns with the spec §3.1 vocabulary; future provenance-related primitives (e.g., source-link verification) can land in the same area without renaming.

## C6. Trace sidecar

### Location and naming

The sidecar lives at `<archive-root>/<phase>/.cflx-trace.json`. The leading dot keeps it visually separated from the human-facing files (`proposal.md`, `design.md`, `tasks.md`) inside the archived change directory. The `<archive-root>` reference is abstract; whether the canonical path is `openspec/changes/archive/` or `meta/changes/archive/` (a pre-existing inconsistency between spec text and the live tree, F3) is decoupled here. When that inconsistency is resolved by a future cleanup phase, the sidecar location follows automatically.

The sidecar is written during cflx workflow execution by the workflow runner. It is read by `cflx trace <phase>` for inspection. The cairn library defines the JSON schema and provides a typed reader that returns a `TraceSidecar` value; the writer is in cflx, not in cairn.

### Schema

```json
{
  "version": 1,
  "phase": "phase-X.Y-name",
  "stages": {
    "propose": { ... StageRecord ... },
    "apply":   { ... StageRecord ... },
    "accept":  { ... StageRecord ... },
    "archive": { ... StageRecord ... }
  },
  "prompts": []
}
```

A `StageRecord` carries: `model_id` (string identifying the model that ran the stage, or null for stages that ran no model), `tokens_in` (integer or null), `tokens_out` (integer or null), `latency_ms` (integer), `success` (boolean), `error_message` (string or null), `started_at` (ISO 8601 timestamp), `ended_at` (ISO 8601 timestamp).

The four stage keys are fixed (`propose`, `apply`, `accept`, `archive`). They are cairn-native cflx vocabulary. They are NOT inherited from any external roadmap or vendor-specific naming.

The top-level `prompts` array is reserved for a future opt-in flag (`cflx trace <phase> --include-prompts`) that persists prompt content. In this phase the array is always empty. The schema accepts the field today so a future phase can populate it without bumping `version`.

### CLI surface

`cflx trace <phase>` reads the sidecar from disk (resolving `<archive-root>` via the same path convention used by `cflx openspec` subcommands) and renders it. Human output is a labelled per-stage breakdown showing stage name, success or failure, model identity, tokens, latency, and error message if any. The `--json` mode prints the sidecar payload directly with the `schema_version` field promoted to the top level per `openspec/specs/cli/spec.md` requirement "Produce stable human and JSON output".

`cflx trace` is a renderer; it does not own semantics. The sidecar's contents are owned by the cflx workflow runner that wrote them. This matches `openspec/specs/cli/spec.md` requirement "Keep CLI backed by shared services".

### What this phase does not ship

- Prompt content persistence (deferred per cross-check A D6).
- A streaming or live-update form of the sidecar (the sidecar is written at archive time as a complete record, not incrementally).
- An MCP tool registration for `cflx_trace`. Cross-check A open question 6 raised this as a possibility; this phase defers it because the MCP registry pattern is not blocking and the CLI surface is sufficient for the auditability claim.

## C8.a. Suggested-edges queue

### File class

The file lives at `openspec/changes/<change>/suggested-edges.json`. The path matches the live tree (which uses `openspec/changes/`, F3); the file is a sibling of `proposal.md`, `blueprint.delta`, `design.md`, and `tasks.md`.

The file is JSON, not TOML. JSON is more machine-friendly for programmatic consumers (CI dashboards, future webui surfaces) and the human-friendly view comes from a future `cflx triage-edges` command. The format choice resolves cross-check A open question 5.

### Schema

```json
{
  "version": 1,
  "entries": [
    {
      "source": "saas.api.auth",
      "target": "saas.api.identity",
      "relation": "depends_on",
      "confidence": 0.82,
      "provenance": {
        "trace_phase": "phase-9-brownfield",
        "stage": "propose"
      },
      "triage_state": "pending",
      "triage_note": null
    }
  ]
}
```

The `triage_state` enum has four values: `pending` (initial state for newly-emitted entries), `accepted` (a human has reviewed and assented), `rejected` (a human has reviewed and refused), `deferred` (a human has reviewed and parked the entry without committing to either accept or reject; equivalent to "needs more info"). Only `accepted` entries are eligible to materialise as blueprint edges in a downstream apply step; this phase does not implement that materialisation, but the schema records the intent.

The `provenance` object is optional and points back to a trace sidecar entry (the change phase that produced this suggestion plus the stage within it). It is empty for entries authored manually for testing. The future suggest engine populates it.

### Library API

The cairn library exposes a `SuggestedEdges` value with: `read_from_change(change_dir: &Utf8Path) -> Result<Option<SuggestedEdges>, CairnError>` (returns `None` if no file exists), `write_to_change(change_dir: &Utf8Path, value: &SuggestedEdges) -> Result<(), CairnError>`, and `count_pending(&self) -> usize`. The reader validates the schema version per `openspec/conventions.md` §3 rule 4: a higher version than understood fails with a clear error; a lower version applies the migration chain (no migrations exist in this phase because the schema is at v1).

The API lives in a new module `src/suggested_edges/` and is re-exported from `src/lib.rs`. The module is small (estimated 100 to 200 LOC) and adheres to the file-size limits in `openspec/conventions.md` §2.

### Why a new file class, not a new delta operation

The existing `blueprint.delta` operations (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`) are a manifest of decisions already taken. Suggested edges are a queue of decisions awaiting human triage. Conflating queue semantics into the manifest would force a fifth operation, complicate every existing tool that walks the delta, and erase the distinction between "this AI proposed it" and "this human committed to it". The terminology-rename phase explicitly preserved the four-operation vocabulary; this phase preserves it for the same reason.

The change-isolation primitive in `openspec/specs/changes/spec.md` already isolates proposed modifications inside `meta/changes/<change-id>/` directories until archive. The suggested-edges file rides inside that primitive without modifying it; the primitive's safety carries over to the queue.

## C8.b. Accept gate untriaged-block

### Integration point

The check lives inside `cflx openspec validate <change> --strict`. Per CLAUDE.md's verification battery, that call is the structural correctness gate run before every phase merges. The validate-strict surface already inspects the change directory for structural correctness; "does this change have untriaged suggested-edges?" is structurally indistinguishable from "is this change valid?" and so is the natural extension point.

The check is implemented in cflx's openspec subcommand, not in the cairn kernel. The kernel provides the reader API (above) that cflx calls; cflx provides the gate logic. This respects CLAUDE.md's cairn/cflx separation directive.

### Behaviour

- Without `--strict`: `cflx openspec validate <change>` reads any `suggested-edges.json` present, counts pending entries, and prints the count as a warning. Exit code stays zero on this finding alone.
- With `--strict`: the same read happens, and a non-zero pending count causes the call to fail with error code `CC002` and a non-zero exit. The error message names the count and the file path. A reader of the failure can navigate directly to the file to triage.

The strict-versus-loose discipline matches existing behaviour: `--strict` is the CI-and-pre-push posture, the loose form is for local exploration.

### Why error code `CC002`

The `CC -- Changes` category is the natural home: the consumer of the gate is `cflx openspec validate`, the validate call is part of the change-archive flow, and the change-archive flow is the `C` (Changes) category per `openspec/conventions.md` §1. `CC001` was allocated by phase-7.5c (verification blocked by upstream dependency); this phase claims `CC002`. The allocation is sequential per the registry's stated rules.

A new category letter (e.g., `V` for Verification, `Q` for Queue) is overkill for a single code. If a future phase introduces a richer queue-related taxonomy, that phase can add a category and migrate; this phase ships one code in the established family.

### What this phase does not ship

- The interactive `cflx triage-edges <change>` command. Authors transition entries away from `pending` via a text editor. A later phase MAY ship the interactive surface; the queue schema is forward-compatible.
- An auto-accept policy (e.g., "accept all entries with confidence above 0.95"). Cross-check A's load-bearing argument is structurally non-bypassable triage; auto-accept on confidence would re-introduce the silent-landing failure mode. This phase ships only the manual-only path.
- Tightening the gate to fail on any non-`accepted` state. The current behaviour is "fail on `pending` only"; `rejected` and `deferred` entries do not block accept. A future phase MAY tighten this once operational evidence shows the cases that should be terminal.

## C5.a. Architectural islands query and verb-edge display

### Two CLI surfaces, one library traversal

Cross-check A R5 argues that the disconnected-subgraph use case has two distinct shapes: anchored ("show me the orphan-side connections of this node") and whole-graph ("show me all the disconnected components"). Conflating them under one flag forces the no-anchor case to invent a synthetic anchor or break neighbourhood semantics.

The library exposes one new query and one extended query.

- `islands(&Map) -> Vec<Island>`: returns the connected-component breakdown of the whole map. Each `Island` carries a node count and a representative node ID (the lexicographically smallest ID in the component, for determinism).
- `neighbourhood(&Map, anchor: NodeId, opts: NeighbourhoodOpts)`: the existing query gains an `include_orphans: bool` field on `NeighbourhoodOpts`. When `true`, the response includes nodes reachable from the anchor only via reverse-direction edges that the default traversal skips.

Both queries use a shared graph traversal helper to avoid duplicating the connected-component algorithm. The helper is internal to the query module.

### CLI surfaces

- `cairn islands` calls the library `islands` query and renders the breakdown. Human output lists each island on its own line with count and representative ID. `--json` mode emits a `{ "schema_version": 1, "islands": [...] }` payload.
- `cairn neighbourhood <node> --include-orphans` extends the existing CLI command with the new flag, which sets `include_orphans: true` on the underlying query call. The flag matches the existing `--include-changes` flag pattern (`openspec/specs/changes/spec.md` line 79).

The two commands are independent; using one does not constrain the other.

### Verb-edge display

The graph explorer in `src/ui_assets/` already renders edge labels (per `openspec/specs/graph-explorer/spec.md` requirement covering edge highlighting). This phase changes the default rendering so verb-form labels (e.g., `depends on`, `implements`, `reviews`) display by default rather than label keys. The change is a single token swap in the explorer's renderer plus a fixture update; estimated under 50 LOC. No spec delta is needed because edge labels are already first-class in the data model.

The verb-edge change rides as a single task under section 4 of `tasks.md`. It does not bump the graph-explorer spec.

## State versioning across all four primitives

Three on-disk artefacts are written or extended by this phase: the trace sidecar, the suggested-edges queue, and the islands query response (which is in-memory but pinned via insta snapshots for the JSON wire format). All three start at `version = 1` and follow the migration-function pattern in `openspec/conventions.md` §3.

The schema-version pinning is enforced by insta snapshot tests per `openspec/conventions.md` §5: every public JSON wire format pinned via insta. The three pinned snapshots cover the trace sidecar, the suggested-edges file, and the `cairn islands --json` response.

## Testing

Three test surfaces beyond the standard build/lint battery.

1. The trace-sidecar reader. Unit tests deserialise a minimal sidecar fixture, assert the four stage keys parse, assert the empty `prompts` array round-trips, and assert that an unknown higher `version` fails with a clear error. An integration test runs `cflx trace <phase>` against a fixture and snapshot-asserts both human and JSON output.
2. The suggested-edges queue. Unit tests round-trip a queue file with one entry per `triage_state` value, assert `count_pending` returns the expected count, and assert the schema-version mismatch error path. An integration test runs `cflx openspec validate <change> --strict` against a fixture change containing one pending entry and asserts the call fails with `CC002`.
3. The islands query and `--include-orphans` neighbourhood. Unit tests build a small map with two disconnected components and assert the `islands` query returns the correct breakdown. An integration test runs `cairn islands` and `cairn neighbourhood <node> --include-orphans` against a fixture and snapshot-asserts both human and JSON output.

The verb-edge display change is covered by an existing graph-explorer fixture snapshot; updating the snapshot to show verb-form labels is part of the rider task.

## Atomic commit groupings

The four sub-components are independent enough that they can land in any order, but the natural groupings for review-narrative clarity are:

1. **Group A**: trace sidecar schema, library reader, `cflx trace` CLI command, and its tests.
2. **Group B**: suggested-edges file class, library API, schema, and its tests.
3. **Group C**: validate-strict gate extension and `CC002` allocation, plus its tests; depends on Group B.
4. **Group D**: islands library query and `--include-orphans` neighbourhood extension; CLI surfaces; verb-edge display rider.

Within each group, individual tasks may land as separate commits; the cflx-runner enforces the group-level boundary if the phase declares it. Commits are sized per the graphite-pr discipline (one logical unit, target under 250 lines added plus removed, hard cap 400).

## Forward compatibility

- The suggested-edges schema reserves the `provenance` field for population by the future suggest engine. That engine ships in a later phase and writes a non-empty `provenance` object that points at this phase's trace sidecar. The two phases are decoupled at the schema level.
- The trace sidecar schema reserves the `prompts` array for a future opt-in. That field stays at `version = 1` if and only if the future phase appends data without changing the rest of the layout; otherwise the future phase increments `version` and provides `migrate_v1_to_v2`.
- The `triage_state` enum is closed at four values in this phase. A future phase MAY add `auto_accepted` (for a confidence-threshold policy) or `escalated` (for a "needs second reviewer" policy) by incrementing the schema version.
- The `cairn islands` CLI command is the no-anchor form. A future phase MAY add a `--component-of <node>` flag that returns only the island containing the named node; the underlying query already supports this and the addition is purely a CLI flag.

## What this phase does not do

The proposal's "Out of scope" section enumerates the deferred surfaces (suggest engine, `cflx triage-edges`, prompt persistence, MCP tool registration, sub-codes, graph-explorer UI for islands, kernel-artefact promotion, path normalisation, reconciliation/artefacts validate cleanup). Two design-level non-goals are noted here for completeness.

- The four-operation delta vocabulary in `openspec/specs/changes/spec.md` is preserved. Suggested edges are a queue alongside the delta, not a fifth operation. The terminology-rename phase made the same call for the same reason.
- The graph-explorer's spec is not bumped. The verb-edge default-rendering change is a single token swap inside an existing requirement and does not warrant a spec delta.
