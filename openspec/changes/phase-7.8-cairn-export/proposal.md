# Proposal: Phase 7.8 Cairn Export

**Change Type**: hybrid

## Dependencies

- `phase-7.8.0-tests` (required; ships the test contract this phase grades against, enabling planned stubs group-by-group).
- `phase-7.5c-verification-states` (queued, not blocking): unrelated subsystem; export reads graph state, not verification state.

Execution: this phase is lifecycle-orthogonal to the main pipeline. It does not block, and is not blocked by, any main-pipeline phase. It MUST run after `phase-7.8.0-tests` apply. It MAY apply in parallel with `phase-7.5c-verification-states` or the feature phases (`phase-8-summariser`, `phase-9-brownfield`, `phase-10-distribution`). Archive when the new `cairn export` command, the JSON schema, the Markdown renderer, and the cli capability spec delta are in place and the strict Rust gate battery passes.

## Sequencing

This phase has no hard ordering constraint against other queued phases except its own test pre-phase (`phase-7.8.0-tests`). The `cairn export` command reads the current scanner output (nodes, edges, artefacts, change directory state) regardless of the cflx lifecycle stage. Export does not interact with the verification battery, the apply-stage codex, or the accept-stage gate. A blocked or failed verification does not change export behaviour; export always exits 0 on a successful render.

The phase ships in isolation after its own test pre-phase and is parallel-safe with the main-pipeline pre-phase + feature pairs.

## Problem/Context

CAIRN's current CLI surface (per `openspec/specs/cli/spec.md`) exposes per-query commands (`cairn get`, `cairn neighbourhood`, `cairn rationale`, `cairn changes`, `cairn status`, and so on). Each command renders one slice of map state. There is no single command that emits the full graph plus artefact corpus as one machine-readable payload, and no single command that emits a flattened human-readable report suitable for paste into an issue, a discussion thread, or an LLM context window.

Two consumers want this surface today.

1. **External tools and agents.** A coding agent or an external linter consuming CAIRN state needs a stable JSON envelope to parse. Today the agent has to call several CLI commands and stitch the output, each call paying the parse cost of `./cairn.blueprint`. A single `cairn export --format json` emits one payload with `schema_version`, full node list, full edge list, full artefact list, and active-change summaries.
2. **Humans reviewing graph state.** A reviewer auditing a change, a contributor writing a discussion comment, or a maintainer producing a status snapshot wants a flattened Markdown report. The webui at `src/ui_assets/` covers interactive review, but a copy-paste friendly report is missing. The `map.md` consolidated artefact (per the v0.7 terminology rename) is a different shape: it consolidates archived spec deltas. A flattened export of *current* graph state is a distinct artefact.

Both formats read the same scanner output. A single export command serving both formats avoids parallel implementations and keeps schema drift out of the design.

## Proposed Solution

Add four things.

1. A new top-level CLI command `cairn export` exposed through `src/cli/mod.rs` and dispatched to a new `src/cli/export.rs` renderer.
2. A `--format <json|md>` flag that selects the payload shape. JSON is the default-correct primary format; Markdown is a transformation over the same in-memory model.
3. A `--output <path>` flag, required at first, that writes the payload to the named file. No default destination ships in this phase. Stdout writing is reserved for a follow-on phase once usage patterns settle.
4. A new requirement in `openspec/specs/cli/spec.md` describing the `cairn export` command, its formats, its schema-version contract, and its lifecycle-orthogonal exit behaviour.

The JSON shape is flat:

```json
{
  "schema_version": 1,
  "generated_at": "<RFC 3339 timestamp>",
  "blueprint_path": "<path to the loaded blueprint>",
  "nodes": [...],
  "edges": [...],
  "artefacts": [...],
  "changes": [...]
}
```

`nodes`, `edges`, `artefacts`, and `changes` reuse the serialisation already used by existing per-query JSON commands (`cairn get --json`, `cairn neighbourhood --json`, `cairn changes --json`). The export renderer composes those serialisations into one envelope; it does not introduce new field names.

The Markdown shape is a single document with four H2 sections (`## Nodes`, `## Edges`, `## Artefacts`, `## Active Changes`) rendered from the JSON model. Nodes are grouped by parent system or container. Edges list the source node, the verb (per spec section 7), and the target node. Artefacts list each direct type with a one-line summary. Active changes list the change id, the proposal title, and the current state.

The phase does NOT introduce a CSV or any binary format (PPTX, DOCX, XLSX). CSV is deferred to a later phase if a concrete consumer demands it. PPTX and DOCX were rejected as out-of-scope for the kernel renderer in the source stronghold.

## Acceptance Criteria

- The `cairn export` command exists in the CLI registry in `src/cli/mod.rs` and dispatches to the export renderer.
- The command accepts `--format json` and `--format md`. The default value of `--format` is `json`.
- The command accepts `--output <path>` and writes the rendered payload to that path. The flag is required; running `cairn export` without `--output` exits with a non-zero status and a labelled human-readable error naming the missing flag.
- The command accepts `--file <path>` and `--changes-dir <path>` via the existing `parse_args` helper, matching every other CLI query command.
- The JSON payload's first field is `schema_version`, set to integer `1`.
- The JSON payload includes `generated_at` (RFC 3339), `blueprint_path`, `nodes`, `edges`, `artefacts`, and `changes` fields, in that order.
- The JSON payload's `nodes`, `edges`, `artefacts`, and `changes` field shapes match the serialisations already produced by `cairn get --json`, `cairn neighbourhood --json`, and `cairn changes --json`. Pinned via `insta` snapshots.
- The Markdown payload is a single document with `## Nodes`, `## Edges`, `## Artefacts`, and `## Active Changes` H2 sections, in that order.
- The Markdown payload contains no em-dashes. A Markdown rendering that introduces an em-dash is a defect.
- A successful render exits with code `0` regardless of whether the underlying graph contains lint findings, drift, or rationale tensions. Findings are out of scope for export.
- A render failure (missing blueprint file, write error to `--output`, malformed graph) exits with code `1` and a `CairnError` carrying an error code from the registry.
- The cli capability spec at `openspec/specs/cli/spec.md` adds one ADDED Requirement covering the export command, with scenarios for JSON output, Markdown output, the required `--output` flag, the schema-version contract, and the exit-code contract.
- All strict Rust gates pass: `cargo build` (zero warnings), `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx openspec validate phase-7.8-cairn-export --strict`.

## Out of Scope

- A `--format csv` mode. Deferred to a later phase if a concrete consumer requests it.
- A `--format pptx` or `--format docx` mode. Rejected as out-of-scope for the kernel renderer.
- Stdout writing without `--output`. Deferred to a follow-on phase once usage patterns settle.
- A default `--output` destination (such as `target/cairn-export/` or `openspec/exports/`). The "Assets stays in provenance chain" question (whether tracked exports are right) is its own research pass; until that resolves, no default is safer than the wrong default.
- A `--scope phase|spec|all` flag for partial export. The phase ships full-graph export only. Partial export waits on a concrete consumer.
- A `--depth N` flag for Markdown truncation. The phase emits the full graph in Markdown form. Truncation is the consumer's job.
- Webui settings-pane integration. The export command is CLI-only in this phase.
- Rationale tension reporting, lint findings, drift summaries, or any verification state in the export payload. Those surface through their own dedicated commands. Export is a snapshot of structure, not a status report.
- Interaction with `cflx accept`, the verification battery, or the apply stage. Export is lifecycle-orthogonal; it does not gate, block, or report any cflx state.
- Promoting the JSON shape to a kernel artefact type. The shape is a CLI wire format, not an artefact.
