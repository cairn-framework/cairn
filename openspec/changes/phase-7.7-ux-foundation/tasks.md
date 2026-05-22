# Tasks: Phase 7.7 UX Foundation

## 1. Kernel: FindingSeverity::Info

- [x] 1.1 Add the `Info` variant to `FindingSeverity` in `src/map/graph.rs` and update the type's derived traits to keep `Debug`, `Clone`, `PartialEq`, `Eq`, `serde::Serialize`, and `serde::Deserialize` intact.
- [x] 1.2 Update every `match FindingSeverity` site across `src/`, `tests/`, and `src/ui/api.rs` so the new arm is handled exhaustively without `_` catch-alls.
- [x] 1.3 Update producers for orphaned-file states and unverified-contract states to emit `Finding` values with `severity = FindingSeverity::Info` instead of staying silent.
- [x] 1.5 Add a reconciler integration test asserting that an orphaned-file fixture produces at least one `Info` finding under `cairn lint`.

## 2. Centralised copy file

- [x] 2.1 Create `docs/design-system/copy.toml` with the two top-level sections `[empty-states]` and `[findings]`.
- [ ] 2.2 Populate `[empty-states]` with the keys `node-no-paths`, `node-no-contracts`, `node-no-decisions`, `node-no-todos`, `node-no-research`, `node-no-sources`, `node-no-outbound`, `node-no-inbound`, `map-clean`, `search-no-matches`, `cli-no-blueprint`, `cli-clean-map`. Each entry has `heading`, `body`, and `cta` fields written in plain English without em-dashes.
- [ ] 2.3 Populate `[findings]` with entries for `CE001` through `CE010`, `CT001`, and `CT002`. Each entry has `heading`, `body`, and `cta` fields and supports the `{node}` and `{target}` placeholders where relevant.
- [x] 2.4 Add a copy-lookup helper module (location: `src/cli/copy.rs` or a shared crate module) that reads the file via `include_str!` and returns `{heading, body, cta}` for a given key, with a console-warning fallback for missing keys.
- [ ] 2.5 Update `docs/design-system/README.md` "When to update each file" table to add the new file as the verbal-language authority.

## 3. Voice section and review checklist

- [ ] 3.1 Add a "Voice" section to `docs/design-system/README.md` covering the em-dash ban, the plain-English bar, the taxonomy preservation rule, and the audience target.
- [ ] 3.2 Append a voice review checklist under the same section as a bulleted list, with at least the six bullets listed in `design.md`.
- [ ] 3.3 Add a brief reference from `CLAUDE.md`'s voice section to the new design-system Voice section so the two locations stay aligned.

## 4. CLI: cairn check subcommand

- [x] 4.1 Add the `cairn check` subcommand to the CLI definition in `src/cli/mod.rs` accepting an optional positional `node` argument.
- [x] 4.2 Implement `cairn check` to call `query::lint(graph)` and render results through the existing `src/cli/format.rs` rendering layer with inspection semantics (always exits zero).
- [x] 4.3 When a node argument is supplied, filter the finding stream to findings whose `node` field equals the requested node ID before rendering.
- [ ] 4.4 Add an integration test under `tests/check.rs` that runs `cairn check` over a fixture map containing one Error, one Warning, and one Info finding, asserting that all three are rendered, the exit code is zero, and the rendered text is stable.
- [ ] 4.5 Add an integration test that runs `cairn check <node>` over the same fixture, asserting that only findings on that node are rendered.

## 5. Empty-state component

- [ ] 5.1 Add a new empty-state component class to `docs/design-system/components.css` consuming only existing tokens (`--stone-3`, `--seam-thin`, `--font-serif`, `--font-sans`, `--font-mono`, `--t-title`, `--t-body`, `--t-small`, `--ink-char`, `--ink-aged`, `--ink-faded`, `--ink-mist`, `--s-2`, `--s-3`, `--s-5`, `--r-large`, `--r-stone`, `--inset-sky`, `--inset-well`).
- [ ] 5.2 Add a section to `docs/design-system/index.html` showing the component using the same token consumption rules.
- [ ] 5.3 Update `docs/design-system/README.md` "When to update each file" table entry as required by the component addition.
- [x] 5.4 Replace the ten inline empty-state strings in `src/ui_assets/app.js` with empty-state component instances reading copy from `docs/design-system/copy.toml` by surface-state key.
- [x] 5.5 Add a JS-side copy-lookup helper in `src/ui_assets/app.js` (or an adjacent helper file) that resolves surface-state keys to `{heading, body, cta}` and logs a console warning when a key is missing.
- [x] 5.6 Update `src/cli/mod.rs` no-args path so it renders the `[empty-states.cli-no-blueprint]` content when no blueprint file is present in the working directory, preserving the existing legacy `.dsl` migration warning behaviour.
- [x] 5.7 Update `src/cli/format.rs` clean-map output so it renders the `[empty-states.cli-clean-map]` content instead of the previous "Findings:\nNone\n" line.
- [x] 5.8 Add a snapshot test under `insta` covering the new CLI empty-state CTAs for both the no-blueprint path and the clean-map path.

## 6. Findings rollup panel

- [x] 6.1 Add the Findings rollup panel surface to `src/ui_assets/app.js`. Mount the panel as a top-level explorer surface (drawer, sidebar tab, or panel) using existing chrome patterns.
- [x] 6.2 Render three severity buckets (`Error`, `Warning`, `Info`) with count badges using existing `.pill` variants in `components.css`. If a neutral pill variant is required for `Info`, add it to `components.css` consuming only existing tokens.
- [x] 6.3 Render a scope toggle (whole map / single node) that becomes active when a node is selected and filters the finding stream to that node when set.
- [x] 6.4 Render category filter chips for each finding-code prefix family present in the current finding stream, derived dynamically so new code families surface without panel changes.
- [x] 6.5 Confirm the panel reads exclusively from `/api/lint` (no separate data path) and that no terminal colour codes or extra logging leak into the rendered output.
- [x] 6.6 Add a UI smoke test (existing webui test pattern) covering the three-bucket render against a fixture finding payload that includes one finding per severity.

## 7. Prose-nudge banner

- [x] 7.1 Add the prose-nudge banner at the top of the node-detail panel in `src/ui_assets/app.js`. The banner appears when the selected node has at least one finding.
- [x] 7.2 Look up the highest-severity finding's copy entry from the `[findings]` section of the centralised copy file using the finding's `code` as the key.
- [ ] 7.3 Substitute `{node}` and `{target}` placeholders in the body field at render time using a small helper that falls through unknown placeholders unchanged.
- [ ] 7.4 Render the call-to-action snippet in a code block using `--font-mono` and a copy button reusing existing pill or button components from `components.css`.
- [x] 7.5 When multiple findings on a node share the same highest severity, render the lowest-numbered code's nudge (deterministic ordering).
- [ ] 7.6 Add a UI smoke test asserting that a node with `CE001` plus `CT001` findings renders the `CE001` nudge (Error severity wins) with the substituted node name in the body.

## 8. Spec deltas

- [x] 8.1 Author `openspec/changes/phase-7.7-ux-foundation/specs/cli/spec.md` covering the `cairn check` subcommand requirement and the CLI empty-state copy requirement.
- [x] 8.2 Author `openspec/changes/phase-7.7-ux-foundation/specs/graph-explorer/spec.md` covering the empty-state component, the Findings rollup panel, the prose-nudge banner, and the integrity overlay extension to three severity buckets.
- [x] 8.3 Author `openspec/changes/phase-7.7-ux-foundation/specs/reconciliation/spec.md` covering `FindingSeverity::Info` producer-side emission for orphaned-file and unverified-contract states.

## 9. Required Verification

- [x] 9.1 `cargo build` passes with zero warnings.
- [x] 9.2 `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features` passes.
- [x] 9.3 `cargo fmt --check` passes.
- [x] 9.4 `cargo test` passes.
- [x] 9.5 `cargo test --locked` passes.
- [ ] 9.6 `cflx openspec validate phase-7.7-ux-foundation --strict` passes.
- [ ] 9.7 No em-dashes (U+2014) in any file authored or edited by this phase. Verified by running `grep -rEn` against the same Unicode codepoint over the change directory and the touched design-system files.
- [ ] 9.8 No hardcoded hex values introduced in `docs/design-system/components.css`. Verified by `grep -c '#[0-9a-fA-F]\{6\}' docs/design-system/components.css` returning the same count as before this phase.
