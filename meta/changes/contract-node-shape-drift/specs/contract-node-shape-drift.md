# Contract Node-Shape Drift Spec

Acceptance criteria for the enforcer unit that follows this change. Rationale
lives in `../design.md`; this file is the binding contract.

## ADDED Requirements

### Requirement: Warning-tier finding for contract node-shape drift

The scanner SHALL emit `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` at Warning severity for
an eligible node whose recorded baseline differs from its current blueprint
shape. Eligible means: the node holds a baseline entry, the blueprint declares
it, and its contract currently loads. The full conjunction is stated under
"Comparison and migration semantics" and governs every emission clause in this
file. The finding asserts that the contract has not been reviewed against the
current shape; it asserts nothing about whether the prose is still accurate. The
finding's `node` is the drifted node id and its `target` is the comma-separated
list of changed fields, drawn from `kind`, `parent`, `edges`, in that order. The
enforcer's commit allocates the code's registry number in
`docs/registries/error-codes.md` and fills the `Code` cell of the existing
`docs/registries/spec-rules.md` row, per `docs/conventions.md` rule 2. The
finding resolves its user-facing text from `[findings.codes]` in
`docs/design-system/copy.toml`.

#### Scenario: A drifted baseline fails only the strict gate

- **GIVEN** an eligible node whose baseline `parent` differs from the blueprint
- **WHEN** the user runs `cairn scan`
- **THEN** one `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` finding is reported at Warning
- **AND** the command exits with code `0`
- **AND** `cairn scan --strict` on the same repository exits with code `1`

#### Scenario: The finding names the changed fields in canonical order

- **GIVEN** a node whose baseline differs from the blueprint in both `parent` and
  `kind`
- **WHEN** the finding is emitted
- **THEN** its `node` is the drifted node id
- **AND** its `target` is `kind, parent`, in that order, whatever order the
  comparison discovered them in

#### Scenario: The message carries no unresolved placeholders

- **GIVEN** an emitted `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` finding
- **WHEN** a reader inspects `finding.message`
- **THEN** the text is resolved through
  `crate::copy::lookup("findings.codes.CAIRN_CONTRACT_NODE_SHAPE_DRIFT.body")`,
  the leaf key, with `{node}` and `{target}` substituted at the emitting site
- **AND** the message contains neither `{node}` nor `{target}`
- **AND** no finding text for this code is hardcoded in Rust source

Rationale: no renderer interpolates copy slots. The CLI prints
`finding.message` verbatim (`src/cli/format/render.rs`) and the web UI prints
`item.message` (`src/ui_assets/channel-bar.js`).

### Requirement: Versioned contract-baseline state file

The enforcer SHALL read node-shape baselines from
`.cairn/state/contract-baselines.json`, using the same `version`/`nodes` envelope
as `blueprint-snapshot.json` with `version` serialised first, at version `1`.
Each entry is keyed by node id and records exactly three fields: `kind`, a JSON
string; `parent`, a JSON string or `null` for a node with no parent; and `edges`,
a JSON array of node-id strings, sorted and deduplicated. The entry type is a
reduced record, not `NodeFingerprint`, whose mandatory `paths` field would both
be written and be required on read.

#### Scenario: An absent file is not an error

- **GIVEN** a repository with no `.cairn/state/contract-baselines.json`
- **WHEN** the scanner loads baselines
- **THEN** it yields an empty baseline set
- **AND** reports no error

#### Scenario: An unsupported version is an error

- **GIVEN** a `.cairn/state/contract-baselines.json` whose `version` is not `1`
- **WHEN** the scanner loads baselines
- **THEN** it returns an error, matching `read_blueprint_snapshot`'s behaviour
  for `blueprint-snapshot.json`

#### Scenario: `paths` is absent from the written shape

- **GIVEN** a baseline written for a node whose blueprint declaration lists paths
- **WHEN** a reader inspects the entry
- **THEN** it holds exactly `kind`, `parent`, and `edges`
- **AND** the file round-trips through the reduced record without loss

#### Scenario: A root node round-trips with a null parent

- **GIVEN** a baseline written for a node with no parent
- **WHEN** a reader inspects the entry
- **THEN** its `parent` is JSON `null`, not an empty string or an absent key
- **AND** the entry round-trips through the reduced record without loss

## MODIFIED Requirements

### Requirement: Baseline recording in `src/summariser/accept.rs`

`accept()` SHALL record the accepted node's current shape into
`.cairn/state/contract-baselines.json` after its post-write scan succeeds and
before it returns. It is the accept-time writer; the only other sanctioned
writer is the non-generative re-record surface named under "Prerequisite" below.
The scanner never writes the file.

#### Scenario: A successful accept records the baseline

- **GIVEN** a draft accepted for a node with no prior baseline
- **WHEN** `accept()` returns successfully
- **THEN** the file holds one entry for that node, carrying its current `kind`,
  `parent`, and `edges`

#### Scenario: A rolled-back accept records nothing

- **GIVEN** an accept whose post-write scan fails and whose contract is rolled
  back
- **WHEN** the call returns
- **THEN** the baseline file is byte-identical to its prior state

#### Scenario: Any failed commit step rolls the whole accept back

- **GIVEN** an accept whose post-write scan succeeds, so the new contract text is
  already installed
- **WHEN** any later fallible step fails: writing
  `.cairn/state/contract-baselines.json`, or the draft-store overwrite that marks
  the draft accepted
- **THEN** the call returns an error
- **AND** the contract text, the draft's lifecycle state, and the baseline file
  are all restored to their prior values, exactly as for a failed post-write scan
- **AND** no state is left where a contract is installed with no baseline, or a
  baseline is recorded for a draft that never became accepted

The implementation may write the baseline before or after the draft-store
overwrite; whichever order it picks, a failure at any step undoes every earlier
write in the same call.

#### Scenario: Re-accepting overwrites only its own node

- **GIVEN** a baseline file holding entries for two nodes
- **WHEN** a second draft is accepted for the first node
- **THEN** that node's entry is overwritten
- **AND** the other node's entry is unchanged

#### Scenario: Scanning never writes the file

- **GIVEN** a repository holding a baseline file
- **WHEN** the user runs `cairn scan`, which writes `interface-hashes.json` and
  `blueprint-snapshot.json`
- **THEN** `.cairn/state/contract-baselines.json` is byte-identical to its prior
  state, whether or not the scan reported drift

### Requirement: Comparison and migration semantics

A node SHALL be compared only when it holds a baseline entry, is declared by the
blueprint, and has a contract that currently loads. A node failing any of those
SHALL never be compared and SHALL never produce a finding, however its
declaration changes. Entries are created only by an explicit write from a
sanctioned writer; no automatic backfill is performed at any point, on upgrade or
on any later scan.

#### Scenario: Upgrading produces no findings

- **GIVEN** a repository holding contracts but no baseline file
- **WHEN** the user runs `cairn scan` for the first time after upgrading
- **THEN** no `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` finding is reported
- **AND** the same holds on every later scan until a baseline is recorded

#### Scenario: A path-only edit produces no finding

- **GIVEN** an eligible node holding a baseline
- **WHEN** only its declared `paths` change
- **THEN** no finding is reported, consistent with
  `check_blueprint_change_decisions`, which leaves path-only edits ungated

#### Scenario: A shape edit produces exactly one finding

- **GIVEN** an eligible node holding a baseline
- **WHEN** its `kind`, its `parent`, or its sorted outbound edge set changes
- **THEN** exactly one finding is reported for that node

#### Scenario: Re-recording clears the finding

- **GIVEN** a node reporting the finding
- **WHEN** its baseline is re-recorded at the current shape
- **THEN** the next scan reports no finding for that node

#### Scenario: A baseline for a node the blueprint no longer declares is inert

- **GIVEN** a baseline file holding an entry whose node id is absent from the
  blueprint, because the node was removed or renamed
- **WHEN** the user runs `cairn scan`
- **THEN** the entry is ignored: no comparison, no finding, and no load error
- **AND** the entry is left in place, since the scanner never writes the file

#### Scenario: A baseline for a node whose contract was removed is inert

- **GIVEN** a node holding a baseline whose contract pointer has since been
  removed from the blueprint, or whose contract file no longer loads
- **WHEN** its `kind`, `parent`, or edge set changes and the user runs
  `cairn scan`
- **THEN** no `CAIRN_CONTRACT_NODE_SHAPE_DRIFT` finding is reported, because
  there is no contract to review
- **AND** the entry is left in place; pruning it is the re-record surface's
  business, never the scanner's

#### Scenario: A node id reused for a different node is compared afresh

- **GIVEN** a baseline entry for a node id that the blueprint now declares with a
  different shape and a contract that loads
- **WHEN** the user runs `cairn scan`
- **THEN** the entry is compared like any other, because a node id is the only
  identity the baseline records

## Prerequisite

The non-generative re-record surface tracked by
`meta/todos/todo.contract-baseline-rerecord-surface.md` (node
`cairn.summariser`) SHALL land before the enforcer, and the
`docs/registries/spec-rules.md` row SHALL stay `pending` until both have. Proof
that re-recording is otherwise unreachable is in `../design.md`, under
"Prerequisite".
