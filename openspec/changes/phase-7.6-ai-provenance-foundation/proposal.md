# Proposal: Phase 7.6 AI Provenance Foundation

**Change Type**: hybrid

## Dependencies

- `phase-7.6.0-tests` (required; ships the test contract this phase grades against by removing `#[cflx_planned]` group-by-group).
- `phase-7.5c-verification-states` (queued; precedes this phase only by being earlier in the 7.x chain, no shared surface).

Execution: MUST run BEFORE any phase that introduces an AI suggest engine for blueprint edges (currently planned to live inside `phase-9-brownfield` per the integrated roadmap). The four primitives this phase ships (trace sidecar, suggested-edges queue, untriaged-block accept gate, architectural-islands query) are loadbearing prerequisites for that future suggest engine. Archives when the new capability area, the spec deltas in `specs/changes`, `specs/cli`, and `specs/query`, the new error code `CC002`, and the strict Rust gate battery all pass.

## Sequencing

This phase has a soft ordering constraint relative to the rest of the campaign.

1. `phase-7.5c-verification-states` (queued).
2. `phase-7.6-ai-provenance-foundation` (this phase).
3. `phase-9-brownfield` rescope commit (docs-only, lands after this phase's design.md ratifies the deferred token-level identifiers per `docs/strongholds/oq4-phase9-rescope-timing.md`).
4. `phase-9.0-tests` apply.
5. `phase-9-brownfield` apply.

The rescope commit timing is decided in `docs/strongholds/oq4-phase9-rescope-timing.md` (Option B with one tightening: rescope after this phase's design.md ratifies the file format, capability-area name, and gate-error code). That commit is not authored inside this phase; it is a later docs-only proposal-update commit on `dev` and is recorded here only so the campaign-level ordering is explicit.

## Problem/Context

Cairn's framework role per `docs/spec.md` §3.4 is "a fence around the authority chain and a navigator for the provenance chain." The current implementation provides the fence and the navigator for human-authored blueprint and artefacts. It does not yet provide the four primitives that any future AI-assisted authoring tool needs in order to ship without violating CLAUDE.md's third positive principle ("AI assists authoring; AI does not substitute for the reconciler") or the spec's two-chain topology (`docs/spec.md` §3.1 to §3.3).

Four gaps motivate this phase, all sourced from `docs/strongholds/getcairn-cross-check-A.md`.

1. **No durable per-run trace.** The cflx workflow today emits build/lint/test logs as transient pre-commit-time output. No durable per-change record names the model that ran each stage, the tokens consumed, the latency observed, or whether each stage succeeded. A reviewer reading an archived change six months from now cannot see what the workflow did, only that it merged. This blocks the auditability claim implicit in the provenance chain.
2. **No safe queue for AI-suggested blueprint edges.** When a future phase introduces an AI suggest engine, that engine produces edge proposals that look structurally identical to deterministically-extracted edges. The change-isolation primitive in `openspec/specs/changes/spec.md` covers the proposal-versus-archived distinction, but it does not cover per-entry must-triage discipline. The current delta vocabulary (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`) is a manifest, not a queue; conflating queue semantics into the manifest would force a fifth operation and break compatibility with existing tooling.
3. **No accept-time block on untriaged AI suggestions.** Phase 9's current safety model is "the human can delete bad outputs before archive" (`openspec/changes/phase-9-brownfield/proposal.md` line 33). That works for candidate nodes (a candidate is either kept or deleted, both deterministic) but not for AI-suggested edges where the safety question is "did a human read this and assent?", not "did a human leave it in the file?". Without a structurally non-bypassable block at accept time, untriaged AI suggestions silently land.
4. **No way to see disconnected pieces of a blueprint.** Once AI-assisted authoring is used to grow blueprints from existing code, blueprints will routinely contain disconnected subgraphs (the brownfield extractor finds them; future AI tools will find more). The existing query layer in `openspec/specs/query/spec.md` exposes `get`, `neighbourhood`, `dependents`, `depends`, and `order`. None answer "show me nodes not reachable from any anchor" or "show me the whole-graph component breakdown". A user reading a freshly-extracted blueprint has no command to surface the disconnection problem.

This phase ships the minimum infrastructure to close the four gaps without entangling itself with any specific suggest engine. The suggest engine itself is out of scope; it lives in `phase-9-brownfield` per the integrated roadmap.

## Proposed Solution

Add five things, organised by sub-component.

### C6. Trace sidecar and `cflx trace` CLI surface

A new capability area `specs/provenance-foundation` defines a per-archived-change trace sidecar. The sidecar lives at `<archive-root>/<phase>/.cflx-trace.json`, where `<archive-root>` is whatever path the canonical change-archive root resolves to. The sidecar carries metadata only in this phase: stage names from cairn-native vocabulary (`propose`, `apply`, `accept`, `archive`), per-stage model identity, per-stage token counts, per-stage latency, per-stage success flag, per-stage error message if any, plus a top-level `version` field per `openspec/conventions.md` §3 state versioning. Prompt content persistence is out of scope; the schema reserves an optional `prompts` field for a future opt-in but ships empty.

A new CLI command `cflx trace <phase>` pretty-prints the sidecar. The command is a pure renderer; it does not own semantics. It supports `--json` mode per `openspec/specs/cli/spec.md` requirement "Produce stable human and JSON output". The `cflx trace` surface lives in cflx (the workflow runner), not in cairn (the kernel), respecting CLAUDE.md's cairn/cflx separation directive. The cairn library defines the schema; cflx writes it during workflow execution and reads it via this command.

### C8.a. Suggested-edges queueing format

A new file class `suggested-edges.json` lives at `openspec/changes/<change>/suggested-edges.json` as a sibling of `proposal.md`, `blueprint.delta`, and `design.md`. The file is JSON with a `version` integer first per state-versioning, then a list of entries. Each entry carries: `source` (node ID), `target` (node ID), `relation` (string), `confidence` (optional float), `provenance` (optional object referencing the `cflx trace` run that produced the entry, populated by the future suggest engine and empty in entries authored manually for testing), `triage_state` (enum: `pending` / `accepted` / `rejected` / `deferred`), and `triage_note` (optional string).

`triage_state` defaults to `pending` for newly-emitted entries. The default plus the C8.b accept gate together form the must-triage primitive: a queue entry is only allowed to land in archive if a human has explicitly transitioned its `triage_state` away from `pending`.

This phase ships the file class, the schema, the JSON parser, and a small library API for reading and writing the file. It does NOT ship the suggest engine itself, the `cflx triage-edges` interactive command, or any accept-time mutation of the file. Those are deferred to the suggest-engine phase or its companion phase.

### C8.b. Accept gate untriaged-block

The `cflx openspec validate <change> --strict` call (which serves as the accept-time structural gate per CLAUDE.md's verification battery) is extended to read any `suggested-edges.json` file present in the change directory. If any entry has `triage_state: pending`, the call fails with a new error code `CC002` ("untriaged suggested edges remain in change") and a message naming the count of pending entries plus the path to the file.

The check is strict-mode-only. Without `--strict`, the same call surfaces the count as a warning but does not fail. This matches the strict-versus-loose discipline established for other validate-time checks. The error code lives in the existing `CC -- Changes` category of `openspec/registries/error-codes.md` (the next sequential allocation after `CC001` from phase 7.5c).

### C5.a. Architectural islands query and verb-edge display

Two CLI surfaces address the disconnected-subgraph use case.

1. `cairn neighbourhood <node> --include-orphans` extends the existing `neighbourhood` command with a flag that includes orphan-side connections (nodes reachable from the anchor only via edges that the default traversal would skip). This complements the existing `--include-changes` flag pattern from `openspec/specs/changes/spec.md`.
2. `cairn islands` is a new whole-graph CLI command that returns the connected-component breakdown of the blueprint, listing each component with its node count and a representative node ID per component. Both commands surface the same underlying graph traversal in the query layer.

A new requirement in `openspec/specs/query/spec.md` (`Answer disconnected-subgraph queries`) defines the query semantics. A new requirement in `openspec/specs/cli/spec.md` (`Surface architectural islands and orphan inclusion`) defines the CLI ergonomics.

Verb-edge display is a small UI rider. The graph explorer in `src/ui_assets/` already knows about edge labels per `openspec/specs/graph-explorer/spec.md`; this phase changes the default rendering so verb-form labels (e.g., `depends on`, `implements`, `reviews`) are shown by default rather than label keys. The change is a token-only adjustment in the explorer's renderer; no spec delta is required because edge labels are already first-class in the data model. The change is captured as a single task in `tasks.md` under the C5.a section.

### Cross-cutting

All four sub-components share a state-versioned schema header convention (per `openspec/conventions.md` §3). The trace sidecar, the suggested-edges queue, and any new state files written by this phase all begin with a `version` integer.

The phase does NOT touch `openspec/specs/artefacts`, `openspec/specs/reconciliation`, or `openspec/specs/graph-explorer`. The artefact taxonomy stays as it is; suggested edges are a new file class inside the existing change-isolation primitive, not a new artefact subtype. The reconciler is unchanged; suggested edges are not reconciler output. The graph explorer's spec is unchanged; the verb-label tweak is inside its existing capability bounds.

## Acceptance Criteria

- A new capability area `openspec/specs/provenance-foundation/spec.md` is added by this phase via the standard ADDED requirements pattern in `specs/provenance-foundation/spec.md`.
- The trace sidecar at `<archive-root>/<phase>/.cflx-trace.json` carries a `version` field as the first key per `openspec/conventions.md` §3 and records per-stage metadata for the four cairn-native stage names `propose`, `apply`, `accept`, `archive`. Prompt content is not persisted in this phase.
- The CLI command `cflx trace <phase>` renders the sidecar in human and `--json` output modes per `openspec/specs/cli/spec.md` requirement "Produce stable human and JSON output".
- The file class `openspec/changes/<change>/suggested-edges.json` is recognised as a sibling of `proposal.md` inside a change directory; the schema starts with `version = 1` and lists per-entry records with `source`, `target`, `relation`, `confidence`, `provenance`, `triage_state`, and `triage_note` fields.
- The `triage_state` field is an enum with values `pending`, `accepted`, `rejected`, `deferred`; new entries default to `pending`.
- `cflx openspec validate <change> --strict` fails with error code `CC002` and a non-zero exit when any entry in `suggested-edges.json` has `triage_state: pending`. Without `--strict`, the same call surfaces the count as a warning but does not fail.
- The error code `CC002 -- untriaged suggested edges remain in change -- phase-7.6` is allocated in `openspec/registries/error-codes.md` under the existing `CC -- Changes` heading.
- The library query layer answers a new `islands` query (returning the connected-component breakdown) and an `--include-orphans` form of `neighbourhood`. Both queries return responses with stable schema versions per `openspec/specs/query/spec.md` requirement "Preserve machine-readable schemas".
- The CLI exposes `cairn islands` and `cairn neighbourhood <node> --include-orphans` over the new query layer surface.
- The graph explorer's default edge-label rendering shows verb-form labels (e.g., `depends on`) rather than label keys.
- The phase does NOT add a `verification`, `suggested-edge`, or `trace` artefact subtype to `openspec/specs/artefacts/spec.md`. The new file classes live as siblings of existing change-directory files, not as new artefact types.
- All strict Rust gates pass: `cargo build` (zero warnings), `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx openspec validate phase-7.6-ai-provenance-foundation --strict`.

## Impact

### Affected specs

- `openspec/specs/provenance-foundation/spec.md` (new capability area; ADDED requirements covering the trace sidecar and the `cflx trace` CLI surface).
- `openspec/specs/changes/spec.md` (ADDED requirement for the suggested-edges file class and the accept-time untriaged-block gate, both inside the existing change-isolation primitive).
- `openspec/specs/cli/spec.md` (ADDED requirement for the `cairn islands` command, the `--include-orphans` flag on `cairn neighbourhood`, and the `cflx trace` rendering surface).
- `openspec/specs/query/spec.md` (ADDED requirement for the disconnected-subgraph query semantics shared by both CLI surfaces).

### Affected registries

- `openspec/registries/error-codes.md` gains `CC002` under the `CC -- Changes` heading.

### Out of scope

- The AI suggest engine itself (deferred to `phase-9-brownfield` per the integrated roadmap; this phase ships the queue file class, not the producer).
- The interactive `cflx triage-edges <change>` command for transitioning entries away from `pending`. The accept gate enforces presence of non-pending state; how a human sets that state is deferred. In the meantime entries can be transitioned by a text editor.
- Prompt content persistence in the trace sidecar. The schema reserves an optional `prompts` field for a future opt-in but ships empty in this phase.
- Splitting `CC002` into multiple sub-codes per missing-triage cause. One code ships now; sub-codes wait on operational evidence.
- A graph-explorer UI for browsing islands or for triaging suggested edges. The CLI is the supported surface in this phase.
- Promoting any of the four primitives to a kernel artefact type. The kernel taxonomy in `openspec/specs/artefacts/spec.md` is unchanged.
- Resolving the `meta/changes/` versus `openspec/changes/` path inconsistency between spec text and the live tree. This phase uses `openspec/changes/` (matching the live tree) and references `<archive-root>` abstractly so a future cleanup phase can normalise without churn here.
- Resolving the pre-existing strict-validate failures in `openspec/specs/reconciliation/spec.md` and `openspec/specs/artefacts/spec.md`. This phase touches neither area.
