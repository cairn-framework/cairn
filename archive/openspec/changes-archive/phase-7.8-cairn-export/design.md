# Design: Phase 7.8 Cairn Export

## References

- `docs/spec.md` section 3 (the two chains: provenance and authority): the topology export must serialise without flattening. Nodes and edges live on the structural spine; artefacts attach to nodes and carry the chain semantics. Export preserves both.
- `openspec/specs/cli/spec.md`: the cli capability spec, with three existing requirements (kernel queries as commands, stable human and JSON output, CLI backed by shared services). The export delta adds one requirement consistent with all three.
- `src/cli/mod.rs`: the CLI command registry and dispatch. Lines 168 to 225 dispatch the existing query commands. The export command is added in the same dispatch table and uses the existing `parse_args` helper for `--file` and `--changes-dir` resolution.
- `openspec/conventions.md` sections 1 (error code registry), 3 (state versioning), 4 (shared type conventions), 5 (testing conventions): the export renderer follows all four.
- `docs/strongholds/getcairn-cross-check-E.md`: the source stronghold that scoped this phase. Decisions D1 (`cairn export` not `cflx export`), D4 (single commit, JSON-first), D6 (lifecycle-orthogonal), R5 (defer schema versioning to design time, ship `schema_version: 1`), R6 (`--output` required, no default), R8 (Markdown is a flattened decision-log report, not `map.md`).

## Module Layout

A new file is added at `src/cli/export.rs`. It contains the export renderer entrypoint plus two private helpers (`render_json`, `render_markdown`). The file stays under the 500-line module limit per `openspec/conventions.md` section 2; if growth pressure emerges, the JSON and Markdown renderers split into sibling files (`src/cli/export/json.rs`, `src/cli/export/markdown.rs`) and `export.rs` becomes a `mod.rs`.

The CLI registry in `src/cli/mod.rs` adds one new command entry for `export`. The `parse_args` helper already supports `--file` and `--changes-dir`; the `--format` and `--output` flags are added to the export command's local arg parser. No global flag changes are required.

The export renderer is a pure function over the existing scanner output. It reuses the serialisation produced by `cairn get --json`, `cairn neighbourhood --json`, and `cairn changes --json`. No new wire format is introduced for individual node, edge, artefact, or change records; the export envelope composes those existing shapes.

## The `cairn export` Command

The command shape is:

```
cairn export --format <json|md> --output <path> [--file <path>] [--changes-dir <path>]
```

The `--format` flag defaults to `json`. The `--output` flag is required; running the command without `--output` exits with code `1` and a labelled human-readable error naming the missing flag.

The command does NOT accept the global `--json` flag. The cli spec's existing `--json` requirement covers the *meta* output mode (does the CLI emit a JSON object at the process boundary). Export's `--format json` is the *payload* shape. The two are distinct; conflating them would mean either nesting the export payload inside a meta envelope (adding a level of indirection consumers do not want) or treating `--json` as a no-op alias for `--format json` (creating two flags that mean the same thing, which is confusing). The cleaner contract is: the export command writes the format-specific payload directly to `--output` and emits no other process-boundary output beyond a labelled status line on stderr.

The renderer reads the in-memory map state via the same library API the existing query commands use. The export envelope is constructed in memory, then serialised to JSON or Markdown depending on `--format`, then written to `--output`.

## JSON Schema

The JSON envelope shape is:

```json
{
  "schema_version": 1,
  "generated_at": "2026-04-30T14:00:00Z",
  "blueprint_path": "./cairn.blueprint",
  "nodes": [
    { "id": "...", "kind": "...", "parent": "...", ... }
  ],
  "edges": [
    { "from": "...", "verb": "...", "to": "..." }
  ],
  "artefacts": [
    { "id": "...", "type": "...", "node": "...", ... }
  ],
  "changes": [
    { "id": "...", "title": "...", "state": "..." }
  ]
}
```

The `schema_version` field is an integer starting at `1`, per `openspec/conventions.md` section 3. Schema migration follows the standard `migrate_v1_to_v2` pattern in the renderer module if a future phase changes the layout. The first-field-position requirement in section 3 applies: `schema_version` is emitted before `generated_at`, `blueprint_path`, and the four collection fields.

The `nodes` collection serialises each node in the same shape `cairn get --json` produces for a single node. The `edges` collection serialises each edge in the same shape `cairn neighbourhood --json` produces for a single edge. The `artefacts` collection serialises each artefact in the shape established by `openspec/specs/artefacts/spec.md` and the existing artefact JSON output. The `changes` collection serialises each active change with its id, title, and state, matching the shape `cairn changes --json` produces.

No new field names are introduced. The export envelope is a composition over existing wire formats. Pinning is via `insta` snapshots in the integration test suite.

The `generated_at` field is an RFC 3339 timestamp produced at render time. It is not derived from any persisted state; it records when the snapshot was taken. The field is included so consumers can distinguish two exports of the same graph at different times without comparing payload bytes.

The `blueprint_path` field records the path the renderer loaded, after `--file` resolution. This makes the export self-describing: a consumer reading the JSON knows which blueprint produced it.

## Markdown Shape

The Markdown payload is a single document with the following structure:

```markdown
# Cairn Export

Generated: 2026-04-30T14:00:00Z
Blueprint: ./cairn.blueprint

## Nodes

### <parent system or container id>

- `<node-id>` (`<kind>`): <one-line summary, if present in the blueprint>

## Edges

- `<from>` `<verb>` `<to>`

## Artefacts

### Decisions

- `<artefact-id>`: <title or one-line summary>

### Todos

- `<artefact-id>`: <title>

(other direct types follow the same H3 grouping)

## Active Changes

- `<change-id>` (`<state>`): <proposal title>
```

Nodes are grouped by parent system or container. The grouping uses the parent's id as the H3 heading. Top-level nodes (those without a parent) are grouped under an H3 with the literal text `(top level)`.

Edges are emitted as a flat unordered list under `## Edges`. Each line names the source node, the verb (per `docs/spec.md` section 7), and the target node. No grouping. The order matches the order in the JSON `edges` array.

Artefacts are grouped by direct type. Each direct type (`contract`, `decision`, `todo`, `research`, `review`, `source`) gets its own H3. Direct types with no artefacts are omitted; the H3 only appears when at least one artefact of that type exists. Within each type, artefacts are listed in id order.

Active changes are emitted as a flat unordered list under `## Active Changes`. Each line names the change id, its current state (in parentheses), and the proposal title. The order matches the order in the JSON `changes` array.

The Markdown payload contains no em-dashes. The renderer uses `". "`, `": "`, `", "`, or parentheses as separators per `CLAUDE.md`'s em-dash ban. A Markdown rendering that produces an em-dash is a defect.

This shape is distinct from the consolidated `map.md` artefact (per the v0.7 terminology rename). `map.md` is the consolidated post-archive map produced by spec-merge tooling. The export Markdown is a flattened *current-state* report, suitable for paste into an issue, a discussion thread, or an LLM context window. The two artefacts have different purposes and different shapes; they must not be confused.

## Output Path Handling

The `--output` flag is required in this phase. No default destination ships. The decision is conservative: defaulting to a tracked location (such as `openspec/exports/`) or an untracked location (such as `target/cairn-export/`) bakes a policy choice into the kernel that has not been validated against operational evidence. The "Assets stays in provenance chain" research is the proper home for that decision; until it resolves, no default is safer than the wrong default.

The `--output` value is treated as a `camino::Utf8PathBuf` per `openspec/conventions.md` section 4. Relative paths resolve against the current working directory. Parent directories are NOT created automatically; if the parent does not exist, the renderer fails with a write error and exits with code `1`. The user is expected to ensure the destination is writable.

If the file at `--output` already exists, the renderer overwrites it without prompting. The kernel does not own destination policy; if a consumer wants append-or-fail semantics, that consumer wraps the command at the shell level.

## Exit Code Contract

The command exits with code `0` on a successful render, regardless of whether the underlying graph contains lint findings, drift, rationale tensions, or any other diagnostic. Export is a snapshot of structure, not a status report.

The command exits with code `1` on any of the following conditions:

- The `--output` flag is missing.
- The `--format` value is not `json` or `md`.
- The `--file` path does not resolve to a readable blueprint.
- The `--changes-dir` path does not resolve to a readable directory.
- The blueprint fails to parse.
- The output file cannot be written (parent missing, permission denied, disk full, and so on).

In all error cases, the renderer surfaces a `CairnError` with an appropriate error code from the registry. No new error code is allocated in this phase; the existing parser, scanner, and IO error codes cover the failure modes.

## Backing Library Service

Per the cli capability spec's third requirement, the CLI is a rendering and process boundary over shared library services. The export command follows that pattern: the renderer logic lives in `src/cli/export.rs` (CLI boundary) and delegates to a typed library service for the in-memory model construction. The service is reused for any future protocol wrapper (MCP, LSP, webui endpoint) that wants to emit the same export payload without parsing CLI text.

The service signature is roughly:

```rust
pub fn build_export(
    file: &Utf8Path,
    changes_dir: &Utf8Path,
) -> Result<ExportEnvelope, CairnError>;
```

`ExportEnvelope` is the shared in-memory model. It derives `Debug`, `Clone`, `serde::Serialize`, and `serde::Deserialize` per `openspec/conventions.md` section 4. The CLI renderer calls `build_export`, then serialises the envelope to JSON or Markdown.

## Testing

Tests cover four surfaces.

1. **Service unit test.** `build_export` against a fixture blueprint, asserting the envelope contains the expected nodes, edges, artefacts, and changes. Located in `src/cli/export.rs` under `#[cfg(test)] mod tests`.
2. **JSON snapshot.** An integration test under `tests/export.rs` runs `cairn export --format json --output <tempdir>/out.json` against a fixture and snapshots the output via `insta`. The snapshot pins the schema. Per `openspec/conventions.md` section 5, public JSON wire formats are pinned via `insta`.
3. **Markdown snapshot.** An integration test under `tests/export.rs` runs `cairn export --format md --output <tempdir>/out.md` against the same fixture and snapshots the output via `insta`. The snapshot includes an em-dash detection assertion: the Markdown body must not contain a U+2014 character.
4. **Exit code test.** An integration test that runs `cairn export` without `--output` and asserts exit code `1` and the expected error message naming the missing flag. A second test runs `cairn export --format invalid --output <tempdir>/out` and asserts exit code `1` and an error naming the invalid format value.

Fixtures live under `tests/fixtures/phase-7.8/`. The fixture is a minimal blueprint with two nodes, one edge, two artefacts (a decision and a todo), and one active change directory. Sufficient for both JSON and Markdown snapshots to exercise the grouping logic.

## Forward Compatibility

The `--format` flag accepts `json` and `md` in this phase. The parser is structured to accept additional values without a syntax break. A future phase MAY add `--format csv` (deferred per the source stronghold) or other formats by extending the match expression in the renderer. Existing consumers reading `--format json` or `--format md` are unaffected.

The JSON envelope's `schema_version` field is the migration handle. A future phase that changes the envelope shape MUST increment the version and provide a `migrate_v1_to_v2` function per `openspec/conventions.md` section 3. Consumers checking `schema_version` before parsing are forward-compatible by construction.

The Markdown shape is not versioned. Markdown is a human format; if the shape changes in a future phase, downstream tooling that screen-scrapes the Markdown output is expected to re-adjust. The JSON envelope is the canonical machine-readable format; Markdown is a presentation layer over it.

## What This Phase Does Not Do

- Does not introduce a `cairn-export` Cargo workspace member. The renderer is a thin CLI module, not a separate crate.
- Does not change the existing per-query JSON shapes (`cairn get --json`, `cairn neighbourhood --json`, `cairn changes --json`). The export envelope composes those shapes; it does not redefine them.
- Does not modify `openspec/specs/artefacts/spec.md`. Artefact serialisation is already specified there; export reuses it.
- Does not modify `openspec/specs/changes/spec.md` or any other capability spec beyond the cli capability. The export command is a CLI-surface concern.
- Does not interact with the verification battery, `cflx accept`, or the apply-stage codex. Export is lifecycle-orthogonal.
- Does not write to `.cairn/state/`. Export is read-only over scanner state and writes only to the user-specified `--output` path.
- Does not produce a `cairn-export.json` file at any default location. The `--output` flag is required.
