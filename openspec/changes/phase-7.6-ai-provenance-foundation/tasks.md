# Tasks: Phase 7.6 AI Provenance Foundation

Atomic-commit groupings: tasks within a numbered section MAY land as separate commits, but each top-level section (1, 2, 3, 4) SHOULD land as a coherent commit boundary. Section 3 depends on section 2; sections 1, 2, and 4 are mutually independent and may interleave. Verbatim per `openspec/conventions.md` §2 the per-commit ceiling is 250 lines added plus removed (hard cap 400).

## 1. Trace sidecar schema, library reader, and `cflx trace` CLI

- [x] 1.1 Add a new module `src/provenance/trace.rs` that defines a `TraceSidecar` value with fields `version` (integer, first), `phase` (string), `stages` (map keyed by `propose`, `apply`, `accept`, `archive` to `StageRecord`), and `prompts` (vector, empty in this phase).
- [x] 1.2 Define `StageRecord` carrying `model_id` (`Option<String>`), `tokens_in` (`Option<u64>`), `tokens_out` (`Option<u64>`), `latency_ms` (`u64`), `success` (`bool`), `error_message` (`Option<String>`), `started_at` (`String` ISO 8601), `ended_at` (`String` ISO 8601). Derive `Debug`, `Clone`, `PartialEq`, `Eq`, `serde::Serialize`, `serde::Deserialize` per `openspec/conventions.md` §4.
- [x] 1.3 Implement `TraceSidecar::read(path: &Utf8Path) -> Result<Self, CairnError>` that validates `version` per `openspec/conventions.md` §3 rule 4: a higher version than understood fails with a clear error naming expected and found versions.
- [x] 1.4 Re-export `TraceSidecar` from `src/lib.rs`. The cairn crate provides the schema and reader; cflx (the workflow runner) provides the writer in a separate codebase.
- [ ] ~~1.5 Add the `cflx trace <phase>` CLI subcommand that reads `<archive-root>/<phase>/.cflx-trace.json` and renders human and `--json` output per `openspec/specs/cli/spec.md` requirement "Produce stable human and JSON output".~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~1.6 Pin the `--json` output via an `insta` snapshot test against a fixture sidecar.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~1.7 Pin the human output via a separate `insta` snapshot test against the same fixture.~~ (Obsolete: cflx retired per decision #105)
- [x] 1.8 Add a unit test that constructs a sidecar with `version = 99` and asserts `TraceSidecar::read` returns a clear error.

## 2. Suggested-edges file class, schema, and library API

- [x] 2.1 Add a new module `src/suggested_edges/` with a `mod.rs` that re-exports the public API and a private internal submodule for parsing. Conform to file-size limits in `openspec/conventions.md` §2.
- [x] 2.2 Define `SuggestedEdges` with fields `version` (integer, first) and `entries` (vector of `SuggestedEdge`). Derive `Debug`, `Clone`, `PartialEq`, `Eq`, `serde::Serialize`, `serde::Deserialize`.
- [x] 2.3 Define `SuggestedEdge` carrying `source` (`NodeId`), `target` (`NodeId`), `relation` (`String`), `confidence` (`Option<f64>`), `provenance` (`Option<EdgeProvenance>`), `triage_state` (`TriageState`), `triage_note` (`Option<String>`).
- [x] 2.4 Define `EdgeProvenance` carrying `trace_phase` (`String`) and `stage` (`String`).
- [x] 2.5 Define the `TriageState` enum with variants `Pending`, `Accepted`, `Rejected`, `Deferred`, deriving the standard traits and serialising as lowercase strings per existing serde-rename convention used elsewhere in the cairn crate.
- [x] 2.6 Implement `SuggestedEdges::read_from_change(change_dir: &Utf8Path) -> Result<Option<Self>, CairnError>` that returns `Ok(None)` if no `suggested-edges.json` file exists in the directory, validates the schema version, and returns the parsed value.
- [x] 2.7 Implement `SuggestedEdges::write_to_change(change_dir: &Utf8Path, value: &Self) -> Result<(), CairnError>` that writes the file atomically.
- [x] 2.8 Implement `SuggestedEdges::count_pending(&self) -> usize` for the gate in section 3.
- [x] 2.9 Re-export `SuggestedEdges`, `SuggestedEdge`, `EdgeProvenance`, and `TriageState` from `src/lib.rs`.
- [x] 2.10 Add unit tests covering: round-trip serialisation of a queue with one entry per `TriageState` value; `count_pending` with mixed states; schema-version mismatch error path; `read_from_change` returning `Ok(None)` for an absent file.
- [ ] 2.11 Pin a representative queue file via an `insta` JSON snapshot test against a fixture.

## 3. Validate-strict accept gate untriaged-block

- [x] 3.1 Allocate `CC002 -- untriaged suggested edges remain in change -- phase-7.6` under the `CC -- Changes` heading in `openspec/registries/error-codes.md`.
- [x] 3.2 Add or extend a `CairnError` variant `UntriagedSuggestedEdges { change_id: String, pending_count: usize, file_path: Utf8PathBuf }` whose `.code()` returns `"CC002"` and whose `Display` impl names the count and the file path.
- [ ] ~~3.3 Extend the `cflx openspec validate <change>` command path to call `SuggestedEdges::read_from_change` on the change directory and surface `count_pending` as a warning when the count is non-zero (without `--strict`).~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~3.4 Extend the `--strict` mode of the same command path to fail with `CC002` and a non-zero exit code when `count_pending` is non-zero.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~3.5 Confirm that when `count_pending` is zero or no `suggested-edges.json` file is present, the validate call's behaviour is unchanged.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~3.6 Add an integration test that runs `cflx openspec validate <change> --strict` against a fixture change containing one entry with `triage_state: pending` and asserts the call exits non-zero with `CC002` in the output.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~3.7 Add a parallel integration test against a fixture where every entry has `triage_state: accepted` and asserts the call exits zero.~~ (Obsolete: cflx retired per decision #105)
- [x] 3.8 Add a unit test that constructs the new `CairnError` variant and asserts `.code() == "CC002"`.

## 4. Architectural islands query, `--include-orphans`, and verb-edge display

- [x] 4.1 Add an `islands(map: &Map) -> Vec<Island>` query to the query module. `Island` carries `node_count` (integer) and `representative` (`NodeId`, the lexicographically smallest ID in the component for determinism).
- [x] 4.2 Add an `include_orphans` field on the existing `NeighbourhoodOpts` value with a default of `false`. When `true`, the neighbourhood query includes nodes reachable from the anchor only via reverse-direction edges that the default traversal skips. (Delivered as `neighbourhood_with_options(graph, anchor, include_orphans)` rather than an Opts struct.)
- [x] 4.3 Share a connected-component traversal helper between the `islands` query and the `--include-orphans` neighbourhood path, internal to the query module.
- [x] 4.4 Add the `cairn islands` CLI command that calls the new library query and renders human and `--json` output. The `--json` schema is `{ "schema_version": 1, "islands": [{ "node_count": ..., "representative": "..." }] }`.
- [x] 4.5 Extend the `cairn neighbourhood <node>` CLI command with the `--include-orphans` flag that sets `include_orphans: true` on the underlying query.
- [ ] ~~4.6 Pin `cairn islands --json` output via an `insta` snapshot test against a fixture map with two disconnected components.~~ (Obsolete: cflx retired per decision #105)
- [ ] ~~4.7 Pin `cairn neighbourhood <node> --include-orphans --json` output via an `insta` snapshot test.~~ (Obsolete: cflx retired per decision #105)
- [ ] 4.8 Update the graph-explorer renderer in `src/ui_assets/` so the default edge-label rendering shows verb-form labels (e.g., `depends on`, `implements`, `reviews`) instead of label keys. Update any affected explorer fixture snapshots accordingly.

## 5. Required Verification

- [ ] 5.1 `cargo build` passes with zero warnings.
- [ ] 5.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [ ] 5.3 `cargo fmt --check` passes.
- [ ] 5.4 `cargo test` passes.
- [ ] 5.5 `cargo test --locked` passes.
- [ ] 5.6 `cflx openspec validate phase-7.6-ai-provenance-foundation --strict` passes.
