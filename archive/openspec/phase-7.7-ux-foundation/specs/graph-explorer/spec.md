# Graph Explorer Capability Spec

## ADDED Requirements

### Requirement: Empty-state component with named next moves

The graph explorer SHALL render empty-state surfaces using a shared empty-state component that names the next move. The component SHALL consume tokens from `docs/design-system/tokens.css` only and SHALL NOT introduce hardcoded colors, sizes, or fonts. Empty-state copy SHALL be sourced from the centralised copy file at `docs/design-system/copy.toml`, keyed by surface state.

#### Scenario: Component is defined with token-only styling

- **GIVEN** Phase 7.7 has archived
- **WHEN** a reader inspects `docs/design-system/components.css`
- **THEN** an empty-state component class is defined consuming only existing tokens (for example `--stone-3`, `--seam-thin`, `--font-serif`, `--font-sans`, `--t-title`, `--t-body`, `--ink-char`, `--ink-aged`, `--s-3`, `--s-5`, `--r-large`)
- **AND** the count of hardcoded six-digit hex values in `components.css` matches the count from before this phase

#### Scenario: All ten inline empty-state strings are replaced

- **GIVEN** the webui at `src/ui_assets/app.js`
- **WHEN** the user opens any node-detail panel section that previously displayed an inline empty-state string
- **THEN** the section renders an empty-state component instance whose copy is sourced from the matching `[empty-states.*]` entry in `docs/design-system/copy.toml`
- **AND** no inline empty-state literal strings remain in `src/ui_assets/app.js` for the ten cross-check-enumerated sites

#### Scenario: Missing copy keys surface a console warning

- **GIVEN** a surface-state key not present in `docs/design-system/copy.toml`
- **WHEN** the webui copy-lookup helper resolves the key
- **THEN** the helper returns a default fallback `{heading, body, cta}` payload
- **AND** the helper logs a console warning naming the missing key so the gap is visible during development

#### Scenario: Empty-state copy is free of em-dashes

- **GIVEN** any `[empty-states.*]` entry consumed by the webui
- **WHEN** the entry is rendered at runtime
- **THEN** the rendered output contains no em-dash characters (U+2014)

### Requirement: Findings rollup panel with severity buckets and filters

The graph explorer SHALL provide a Findings rollup panel that groups the existing `/api/lint` finding stream into three severity buckets (`Error`, `Warning`, `Info`), exposes a scope toggle (whole map / single node), and exposes a category filter on finding-code prefix derived from the current finding stream. The panel SHALL consume `/api/lint` exclusively (no separate data path) per the existing query-consumer architecture.

#### Scenario: Three severity buckets render with count badges

- **GIVEN** the `/api/lint` finding stream contains at least one finding per severity (`Error`, `Warning`, `Info`)
- **WHEN** the panel mounts
- **THEN** three labelled bucket sections appear in severity order
- **AND** each bucket displays a count badge using existing pill component variants from `components.css`
- **AND** severity colors map to `--block` for `Error`, `--drift` for `Warning`, and an info-class token (`--orphaned` or `--ink-mist`) for `Info`

#### Scenario: Scope toggle filters to the selected node

- **GIVEN** the panel is mounted and a node is selected in the graph view
- **WHEN** the user sets the scope toggle to single-node
- **THEN** the panel filters the finding stream to findings whose `node` field equals the selected node ID
- **AND** the bucket counts update to reflect the filtered stream

#### Scenario: Scope toggle is disabled when no node is selected

- **GIVEN** the panel is mounted and no node is selected
- **WHEN** the user inspects the scope toggle
- **THEN** the toggle is disabled and the panel displays whole-map findings

#### Scenario: Category filter chips derive from the finding stream

- **GIVEN** the `/api/lint` finding stream contains findings whose codes use the `CE` and `CT` prefix families
- **WHEN** the panel mounts
- **THEN** filter chips appear for `CE` and `CT` only
- **AND** a future stream containing a new prefix family (for example `CS` for summariser) automatically surfaces a new chip without panel-side code changes

#### Scenario: Panel reads only from the query-consumer API

- **GIVEN** the panel is mounted
- **WHEN** any finding data is displayed
- **THEN** that data was obtained via `GET /api/lint`
- **AND** no other data path supplies the panel

### Requirement: Prose-nudge banner translates findings to plain English

The graph explorer SHALL render a prose-nudge banner at the top of the node-detail panel when the selected node has at least one finding. The banner SHALL look up the highest-severity finding's copy entry from the `[findings]` section of `docs/design-system/copy.toml` keyed by finding code, render heading and body with `{node}` and `{target}` placeholder substitution, and render the call-to-action as a copy-pasteable CLI snippet. The banner SHALL NOT introduce in-UI mutating actions; all calls-to-action stay as CLI handoffs.

#### Scenario: Banner renders the highest-severity finding's nudge

- **GIVEN** a selected node with one Error finding (`CE001`) and one Warning finding (`CT002`)
- **WHEN** the node-detail panel mounts
- **THEN** the banner renders the `[findings.CE001]` heading, body, and call-to-action
- **AND** the body has `{node}` and `{target}` placeholders substituted with the relevant node and target IDs

#### Scenario: Tie-break by lowest-numbered code

- **GIVEN** a selected node with two Error findings, `CE001` and `CE003`
- **WHEN** the node-detail panel mounts
- **THEN** the banner renders the `CE001` nudge
- **AND** the `CE003` finding remains visible in the Findings rollup panel

#### Scenario: Banner CTA is a copy-pasteable CLI snippet

- **GIVEN** any banner instance
- **WHEN** the call-to-action renders
- **THEN** the snippet appears in a code block using `--font-mono`
- **AND** a copy button reuses an existing pill or button component from `components.css`
- **AND** no in-UI mutating action (such as a "Fix with AI" button) appears in the banner

#### Scenario: Banner is hidden when the node has no findings

- **GIVEN** a selected node whose finding count is zero
- **WHEN** the node-detail panel mounts
- **THEN** the banner is not rendered

## MODIFIED Requirements

### Requirement: Integrity overlay

The graph explorer SHALL display `cairn lint` findings as visual indicators on affected nodes and edges across three severity buckets (`Error`, `Warning`, `Info`).

#### Scenario: Structural error indicator

- **GIVEN** a node with a structural error (for example duplicate ID, broken pointer)
- **WHEN** the graph loads
- **THEN** the node displays a severity badge using the `--block` color token
- **AND** clicking the badge shows the error detail in the node detail panel

#### Scenario: Interface contradiction indicator

- **GIVEN** a node where code diverges from its contract hash
- **WHEN** the graph loads
- **THEN** the node displays a severity badge using the `--drift` color token

#### Scenario: Rationale tension indicator

- **GIVEN** a node with an orphan research artefact or missing source link
- **WHEN** the graph loads
- **THEN** the node displays an advisory badge whose color token reflects the producer-side severity (`--drift` for `Warning`, `--orphaned` or `--ink-mist` for `Info`)

#### Scenario: Info-severity findings appear in the overlay

- **GIVEN** a node with an `Info`-severity finding emitted by the orphaned-file or unverified-contract producer
- **WHEN** the graph loads
- **THEN** the node displays the info-class advisory badge
- **AND** the badge does not block any hook, gate, or `cflx accept` outcome
