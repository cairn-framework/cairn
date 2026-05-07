# Tasks: Phase 7.8 Cairn Export

## 1. Library Service

- [x] 1.1 Define `ExportEnvelope` in `src/cli/export.rs` with fields `schema_version: u32`, `generated_at: String`, `blueprint_path: Utf8PathBuf`, `nodes: Vec<NodeRecord>`, `edges: Vec<EdgeRecord>`, `artefacts: Vec<ArtefactRecord>`, `changes: Vec<ChangeRecord>`. Reuse the `NodeRecord`, `EdgeRecord`, `ArtefactRecord`, and `ChangeRecord` shapes already produced by the existing query commands; do not redefine them.
- [x] 1.2 Derive `Debug`, `Clone`, `serde::Serialize`, `serde::Deserialize`, `PartialEq`, and `Eq` on `ExportEnvelope` per `openspec/conventions.md` section 4.
- [x] 1.3 Implement `pub fn build_export(file: &Utf8Path, changes_dir: &Utf8Path) -> Result<ExportEnvelope, CairnError>` in `src/cli/export.rs` that loads the blueprint, runs the scanner, reads the active changes directory, and assembles the envelope.
- [x] 1.4 Set `schema_version` to integer `1` and emit it as the first field in the serialised JSON form per `openspec/conventions.md` section 3.
- [x] 1.5 Set `generated_at` to a UTC RFC 3339 timestamp captured at the start of `build_export`. Use `chrono` or `time` per workspace dependency conventions.
- [x] 1.6 Add a unit test under `#[cfg(test)] mod tests` in `src/cli/export.rs` that calls `build_export` against a fixture and asserts the envelope's collection counts match the fixture.

## 2. CLI Command and Flag Parsing

- [x] 2.1 Register `export` in the CLI command table in `src/cli/mod.rs`, dispatching to a new `cli::export::run` entrypoint.
- [x] 2.2 Add a local arg parser for the export command that accepts `--format <json|md>` (default `json`), `--output <path>` (required), `--file <path>` (optional, falls through to `parse_args`), and `--changes-dir <path>` (optional, falls through to `parse_args`).
- [x] 2.3 Reject `--format` values other than `json` or `md` with a labelled human-readable error and exit code `1`.
- [x] 2.4 Reject a missing `--output` flag with a labelled human-readable error naming the missing flag and exit code `1`.
- [x] 2.5 Surface flag-parsing failures through `CairnError` using existing error codes; do not allocate new codes in this phase.

## 3. JSON Renderer

- [x] 3.1 Implement a private `render_json(envelope: &ExportEnvelope) -> Result<String, CairnError>` helper in `src/cli/export.rs` that serialises the envelope via `serde_json::to_string_pretty`.
- [x] 3.2 Confirm via unit test that the first key of the serialised string is `"schema_version"`. The test parses the string back and asserts the key order via `serde_json::Value` iteration on the top-level object.
- [x] 3.3 Add an integration test under `tests/export.rs` that runs `cairn export --format json --output <tempdir>/out.json` against a fixture under `tests/fixtures/phase-7.8/` and snapshots the output file contents via `insta`.
- [x] 3.4 Pin the JSON wire format with the snapshot per `openspec/conventions.md` section 5.

## 4. Markdown Renderer

- [x] 4.1 Implement a private `render_markdown(envelope: &ExportEnvelope) -> Result<String, CairnError>` helper in `src/cli/export.rs` that produces the document with `# Cairn Export` H1, `Generated:` and `Blueprint:` lines, and four H2 sections (`## Nodes`, `## Edges`, `## Artefacts`, `## Active Changes`) in that order.
- [x] 4.2 Group nodes under H3 headings by parent system or container id. Top-level nodes appear under an H3 with the literal text `(top level)`.
- [x] 4.3 Render edges as a flat unordered list under `## Edges`. Each line names the source node, the verb, and the target node in the order they appear in the envelope's `edges` collection.
- [x] 4.4 Group artefacts under H3 headings by direct type (`contract`, `decision`, `todo`, `research`, `review`, `source`). Direct types with zero artefacts are omitted; the H3 only appears when at least one artefact of that type exists.
- [x] 4.5 Render active changes as a flat unordered list under `## Active Changes`. Each line names the change id, the state in parentheses, and the proposal title.
- [x] 4.6 Add a unit test that asserts the rendered Markdown contains no U+2014 (em-dash) character. The renderer uses `". "`, `": "`, `", "`, or parentheses as separators per `CLAUDE.md`'s em-dash ban.
- [x] 4.7 Add an integration test under `tests/export.rs` that runs `cairn export --format md --output <tempdir>/out.md` against the same fixture and snapshots the output via `insta`.

## 5. Output Path Handling

- [x] 5.1 Implement file write logic that opens the path supplied via `--output` and writes the rendered payload, overwriting any existing content.
- [x] 5.2 Surface write failures (parent directory missing, permission denied, disk full) through `CairnError` and exit with code `1`.
- [x] 5.3 Do not create parent directories automatically; a missing parent is a write error.
- [x] 5.4 Add an integration test that runs `cairn export --format json` without `--output` and asserts exit code `1` and an error message naming the missing flag.
- [x] 5.5 Add an integration test that runs `cairn export --format json --output <nonexistent-parent>/out.json` and asserts exit code `1` and a write error.

## 6. CLI Capability Spec Delta

- [x] 6.1 Add the `cairn export` requirement to `openspec/specs/cli/spec.md` via the spec delta under `specs/cli/spec.md` in this change directory.
- [x] 6.2 Confirm the cli capability spec delta validates under `cflx openspec validate phase-7.8-cairn-export --strict` with no warnings.

## 7. Required Verification

- [x] 7.1 `cargo build` passes with zero warnings.
- [x] 7.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 7.3 `cargo fmt --check` passes.
- [x] 7.4 `cargo test` passes.
- [x] 7.5 `cargo test --locked` passes.
- [x] 7.6 `cflx openspec validate phase-7.8-cairn-export --strict` passes.
- [x] 7.7 No em-dashes appear anywhere in `openspec/changes/phase-7.8-cairn-export/`, in `tests/fixtures/phase-7.8/`, or in any rendered Markdown produced by the export command. Verified by `grep -rn` for the U+2014 character over the change directory and the snapshot files.
