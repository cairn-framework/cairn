# Design: Phase 7.7 UX Foundation

## References

- `docs/strongholds/getcairn-cross-check-B.md`: full cross-check that scoped this phase, including the rename rationale, the `FindingSeverity::Info` decision, and the centralised-copy location.
- `docs/strongholds/getcairn-refined-batch-A.md`: source of the C2.c, C3.a-c, and C13.a-e refinements.
- `docs/strongholds/getcairn-roadmap-debate.md`: Bundle B scope statement at the roadmap level.
- `openspec/specs/cli/spec.md`: CLI rendering boundary, library-service delegation, JSON output convention.
- `openspec/specs/graph-explorer/spec.md`: query-consumer architecture, integrity overlay, node detail panel.
- `openspec/specs/reconciliation/spec.md`: producers of the existing `Finding` stream the panel and banner consume.
- `openspec/registries/error-codes.md`: finding-code registry consumed as the category-filter key.
- `docs/design-system/tokens.css`, `docs/design-system/components.css`, `docs/design-system/README.md`: token and component authority for all UI work.
- `src/map/graph.rs`: home of `Finding` and `FindingSeverity`; the `Info` variant lands here.
- `src/ui_assets/app.js`, `src/ui_assets/api/`: read-only webui surface that consumes `/api/lint`.
- `src/cli/format.rs`, `src/cli/mod.rs`: CLI rendering layer for findings and the no-args entrypoint.

## Kernel: `FindingSeverity::Info`

The `Finding` struct in `src/map/graph.rs` exposes a two-variant `FindingSeverity` (`Error`, `Warning`). This phase adds `Info` as a third variant. Every `match FindingSeverity` site in the kernel, the CLI render layer, the `/api/lint` serialiser, and any test fixtures gains an `Info => ...` arm. Adding the variant is a back-compatible enum extension under `openspec/conventions.md` section 3 state-versioning rules: existing on-disk JSON that does not yet contain `Info` continues to deserialise unchanged; new emissions of `Info` do not break consumers that match exhaustively (those consumers fail to compile until they handle the new arm, which is the correct fail-fast for an internal API).

Producers that today emit nothing for orphaned-file states and unverified-contract states start emitting `Info` findings. The cross-check identifies these as the natural seed sites; the implementation may discover additional advisory-state seam points and SHALL emit `Info` rather than upgrading to `Warning` for any state where the current behaviour is silence.

`Info` does not block hooks, gates, or `cflx accept`. The reconciliation spec's "structural hook remains passable when no blocking findings exist" requirement is unchanged: `Info` joins `Warning` on the non-blocking side; only `Error` is blocking.

## Centralised copy file: `docs/design-system/copy.toml`

The file lives at `docs/design-system/copy.toml`. The location matches the design-system authority pattern: tokens are visual language, copy is verbal language, both consumed via `include_str!` from the Rust webui binary (the same pattern that `tokens.css` and `components.css` already follow per `docs/design-system/README.md`).

TOML over JSON because the file is hand-edited under voice rules (the em-dash ban, plain-English bar, taxonomy preservation from `CLAUDE.md`). TOML carries comments and section headers cleanly; JSON does not.

The schema has two top-level sections.

```toml
# Empty-state copy keyed by surface state.
[empty-states.node-no-paths]
heading = "No paths declared."
body = "Author one or more `path` entries on this node, then run `cairn scan`."
cta = "Run `cairn scan` after editing"

[empty-states.node-no-contracts]
heading = "No contracts attached."
body = "Contracts capture the obligations this node owes its dependents."
cta = "Author a contract under `meta/contracts/<node>.md`"

# Prose nudges keyed by finding code.
[findings.CE001]
heading = "A declared edge has no observed source dependency."
body = "The blueprint says `{node}` depends on `{target}`, but no source file under `{node}`'s paths actually imports anything from `{target}`'s paths."
cta = "Run `cairn neighbourhood {node}` to see what paths actually depend on each other"
```

The `[findings.<CODE>]` entries support `{node}` and `{target}` placeholders. The webui performs the substitution at render time using a small lookup helper. The placeholder set is fixed at `{node}` and `{target}` for this phase; additional placeholders are forward-compatible (the helper falls through unknown placeholders unchanged).

The copy keys covered by this phase are:

- `[empty-states.node-no-paths]`, `[empty-states.node-no-contracts]`, `[empty-states.node-no-decisions]`, `[empty-states.node-no-todos]`, `[empty-states.node-no-research]`, `[empty-states.node-no-sources]`, `[empty-states.node-no-outbound]`, `[empty-states.node-no-inbound]`, `[empty-states.map-clean]`, `[empty-states.search-no-matches]`. These match the ten existing inline empty-state sites in `src/ui_assets/app.js`.
- `[empty-states.cli-no-blueprint]`, `[empty-states.cli-clean-map]`. These cover the CLI parallel empty-state CTAs for `cairn` with no args and `cairn lint` against a clean map.
- `[findings.CE001]` through `[findings.CE010]`, `[findings.CT001]`, `[findings.CT002]`. These cover every finding code currently allocated in `openspec/registries/error-codes.md`.

The webui parses the file at compile time. Two reasonable implementations exist: parse TOML in the embedded binary at startup using a TOML crate, or pre-parse to JSON at build time and embed the JSON. The choice is left to the implementer; both are forward-compatible and neither alters the spec contract.

## CLI: `cairn check`

The new subcommand lives in `src/cli/mod.rs` alongside the existing `cairn lint` and `cairn scan` arms. It calls `query::lint(graph)` (the same library function `cairn lint` calls), threads results through the existing `Finding` rendering layer in `src/cli/format.rs`, and prints labelled human-readable output. It always exits zero regardless of severity (inspection semantics, not gate semantics).

`cairn check` accepts an optional positional node argument. Without an argument, it reports findings for the whole map. With an argument, it filters the finding stream to findings whose `node` field matches the requested node ID, providing scope-toggle parity with the webui panel.

`cairn check --json` is explicitly out of scope per the cross-check D6 decision. Adding `--json` is a follow-on if a CI consumer materialises; today, `cairn lint --json` already covers any JSON consumer.

The two commands coexist deliberately. `cairn lint` retains gate semantics and continues to be the entrypoint pre-commit hooks invoke; `cairn check` carries inspection semantics and is the entrypoint a user invokes when reading the panel's text equivalent. Both delegate to the same library service, satisfying the `openspec/specs/cli/spec.md` rule that "CLI command delegates to library service."

## Empty-state component

The component class lives in `docs/design-system/components.css`. It composes existing tokens only (no new colors, sizes, or fonts). The structure is icon plus heading plus body plus call-to-action.

Token consumption:

- Surface: `--stone-3` (raised cards) with `--inset-sky` for the inner highlight.
- Border: `--seam-thin` for the default seam, `--seam-clear` on focus or hover.
- Heading: `--font-serif`, `--t-title`, `--ink-char`.
- Body: `--font-sans`, `--t-body`, `--ink-aged`.
- Call-to-action: `--font-mono`, `--t-small`, `--ink-faded` for the snippet line; the snippet wraps in an `inset` style box using `--inset-well`.
- Icon: reuse existing artefact glyphs from `components.css` (`.kind-todo`, `.kind-research`, etc.) at reduced opacity (`--ink-mist` color) for the empty-state context. No new icon set.
- Spacing: `--s-3` between icon and heading, `--s-2` between heading and body, `--s-3` between body and call-to-action; outer padding `--s-5`.
- Radius: `--r-large` for the outer surface; `--r-stone` for the inset call-to-action snippet box.

The component is one new class in `components.css` plus one or two helper classes for the call-to-action snippet treatment. The live reference at `docs/design-system/index.html` gains a section showing the component using the same token consumption rules.

## Webui sweep

`src/ui_assets/app.js` has ten inline empty-state strings the cross-check enumerated. Each gets replaced by an empty-state component instance whose copy is read from `docs/design-system/copy.toml` by surface-state key. The mapping is:

- `app.js:871` "No paths declared on this node." -> `[empty-states.node-no-paths]`.
- `app.js:920` "No contracts attached." -> `[empty-states.node-no-contracts]`.
- `app.js:926` "No decisions recorded." -> `[empty-states.node-no-decisions]`.
- `app.js:944` "No open todos." -> `[empty-states.node-no-todos]`.
- `app.js:950` "No research attached." -> `[empty-states.node-no-research]`.
- `app.js:956` "No sources cited." -> `[empty-states.node-no-sources]`.
- `app.js:962` "No outbound dependencies." -> `[empty-states.node-no-outbound]`.
- `app.js:968` "No inbound dependents." -> `[empty-states.node-no-inbound]`.
- `app.js:1092` and `app.js:1195` "Map is clean. No findings." -> `[empty-states.map-clean]`.
- `app.js:1158` "No matches." -> `[empty-states.search-no-matches]`.

Line numbers are indicative of the cross-check inputs; the implementation MAY refactor to avoid duplication (the duplicated "Map is clean. No findings." string folds into a single helper).

The webui includes a minimal copy-lookup helper that takes a surface-state key and returns `{heading, body, cta}`. Missing keys fall back to a default "Nothing here yet." pattern; missing keys SHALL emit a console warning so the gap is surfaced during development.

## CLI sweep

`src/cli/mod.rs` no-args path renders the `[empty-states.cli-no-blueprint]` content when no blueprint file is present in the working directory. The existing legacy-`.dsl` migration warning behaviour is preserved (per `CLAUDE.md` terminology section).

`src/cli/format.rs` clean-map path (the "Findings:\nNone\n" code path) renders the `[empty-states.cli-clean-map]` content instead. The existing JSON-output behaviour is unchanged.

The CLI does not embed the TOML file at compile time the same way the webui does. The CLI reads the same source file via `include_str!` from the cairn binary; the helper that translates a surface-state key to `{heading, body, cta}` lives in the cairn crate (the implementer chooses the module location; `src/cli/copy.rs` is the natural fit).

## Findings rollup panel

The panel lives in `src/ui_assets/app.js`. It mounts as a top-level surface in the explorer chrome (the implementer chooses the precise mount point: as a drawer, a sidebar tab, or a panel sibling to the existing changes drawer). It consumes `/api/lint` exclusively (no separate data path, per the graph-explorer spec's query-consumer architecture).

Three severity buckets: `Error`, `Warning`, `Info`. Each bucket renders as a labelled section with a count badge using the existing pill components in `components.css` (`.pill.drift` for `Warning`, `.pill.settled` repurposed for `Info` if visually appropriate, or a new neutral pill variant added to `components.css` consuming only existing tokens). The severity colors map to `--block` (Error), `--drift` (Warning), and `--orphaned` or `--ink-mist` (Info), per the tokens.css comment block.

Scope toggle: a two-state switch (whole map / single node). When a node is selected in the graph view, the toggle becomes available; the single-node setting filters the finding stream to findings whose `node` field matches the selected node. When no node is selected, the toggle is disabled and the panel shows whole-map findings.

Category filter: a multi-select on finding-code prefix. The panel exposes filter chips for each prefix family present in the current finding stream (`CE` for edge, `CT` for target, plus any future families). The chips are derived dynamically from the finding stream so new code families surface automatically without panel changes.

The panel does not introduce a re-run button. The deterministic reconciler is recomputed on the existing file-watch refresh path; the panel's data updates as `/api/lint` updates.

## Prose-nudge banner

The banner lives at the top of the node-detail panel in `src/ui_assets/app.js`. When the selected node has at least one finding, the banner renders the `[findings.<CODE>]` entry for the highest-severity finding (Error first, then Warning, then Info). Within a severity bucket, the lowest-numbered code wins (deterministic ordering).

The banner renders heading plus body plus a copy-pasteable CLI snippet. The body field supports the `{node}` and `{target}` placeholders; the snippet is rendered in a code block using `--font-mono` with a copy button (the copy button uses the existing pill or button component from `components.css`). The snippet text is the `cta` field from the copy file.

The banner does not include a "Fix with AI" button. The cross-check's D2 decision rules in: the webui write surface is out of scope for this phase, so all calls-to-action are CLI-handoff snippets, not in-UI mutating actions.

## Voice section update

`docs/design-system/README.md` gains a new "Voice" section. The section names the em-dash ban, the plain-English bar, the taxonomy preservation rule (`blueprint`, `map`, `map.md`, `interface contradiction`, `rationale tension`, etc. from `CLAUDE.md`), and the audience target ("people building with AI tools" including non-devs).

A voice review checklist follows the prose, formatted as bullets:

- Are user-facing strings free of em-dashes?
- Does each empty-state name the next move?
- Do finding nudges use plain English without flattening load-bearing taxonomy?
- Are command names quoted in backticks and stable across copy?
- Do CTAs render as runnable CLI snippets, not in-UI mutating actions?
- Is the highest-severity finding the one that gets the banner?

The README is the single voice-review artefact; no separate `voice.md` file ships. `CLAUDE.md` retains its voice section as the repo-level quick reference.

## Spec deltas

Three capability specs receive deltas.

`openspec/specs/cli/spec.md` gains an ADDED requirement covering the new `cairn check` subcommand: scope toggle via positional argument, inspection semantics (always exits zero), library-service delegation, no `--json` mode in this phase. It also gains an ADDED requirement covering CLI empty-state copy (the no-blueprint and clean-map CTAs).

`openspec/specs/graph-explorer/spec.md` gains an ADDED requirement covering the empty-state component, an ADDED requirement covering the Findings rollup panel (severity buckets, scope toggle, category filter), and an ADDED requirement covering the prose-nudge banner. It MODIFIES the existing "Integrity overlay" requirement to reference the three-bucket severity model (the existing two-bucket scenarios for structural error and interface contradiction stay; rationale tension expands to cover both `Warning` and `Info`).

`openspec/specs/reconciliation/spec.md` gains an ADDED requirement covering the `Info` severity producer-side emission for orphaned-file and unverified-contract states.

## Out of scope (and why)

- Webui write surface: per cross-check D2, this is a separate stronghold-level investigation; the CLI-handoff fallback is the only honest design under the current read-only `/api/`.
- `cairn check --json`: per cross-check D6, no consumer demand today; `cairn lint --json` already covers CI consumers.
- Custom empty-state illustrations: per cross-check Q3 default, reuse existing artefact glyphs; custom illustrations are a later marketing-and-onboarding push.
- Splitting `Info` into sub-codes: one variant ships now; sub-codes wait on operational evidence.
- Re-run button on the panel: file-watch refresh covers this; deterministic reconciler does not need a manual re-run.
- Renaming `cairn lint`: both commands ship; `cairn lint` retains gate semantics for hooks.

## Forward compatibility

The TOML schema is forward-compatible at both the section and field level: new top-level sections and new fields under existing sections do not break readers that look up specific keys. The `[findings.<CODE>]` placeholder set is fixed at `{node}` and `{target}` for this phase; new placeholders are forward-compatible because the lookup helper falls through unknown placeholders unchanged.

`FindingSeverity::Info` is an additive enum variant; existing on-disk JSON that lacks `Info` continues to deserialise. New consumers that match exhaustively must add an `Info` arm; this is mechanical and the compiler enforces it.

The `cairn check` subcommand is a new CLI entry; it does not alter the existing `cairn lint` contract. A future phase MAY add `--json` to `cairn check` without re-authoring the subcommand.

The Findings rollup panel derives its category filter chips from the finding stream itself, so a future phase that allocates a new finding-code family (for example `CS001` for summariser findings under phase 8) automatically surfaces a new filter chip without panel-side changes.
