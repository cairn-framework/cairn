# Provenance Foundation Capability Spec

## ADDED Requirements

### Requirement: Persist a per-archived-change trace sidecar

Cairn SHALL define a per-archived-change trace sidecar at `<archive-root>/<phase>/.cflx-trace.json` that records workflow execution metadata for the four cairn-native cflx stages `propose`, `apply`, `accept`, and `archive`. The cairn library SHALL define the schema and provide a typed reader. The cflx workflow runner SHALL be the writer; the cairn kernel SHALL NOT write the sidecar.

#### Scenario: Sidecar is state-versioned

- **GIVEN** an archived change directory containing a `.cflx-trace.json` file
- **WHEN** a reader inspects the file
- **THEN** the first field is `version` (integer)
- **AND** the value of `version` is `1` for sidecars produced by this phase

#### Scenario: Sidecar covers the four cairn-native stages

- **GIVEN** a sidecar produced by a successful cflx run
- **WHEN** a reader parses the file
- **THEN** the `stages` map contains exactly the keys `propose`, `apply`, `accept`, and `archive`
- **AND** each value is a stage record carrying `model_id`, `tokens_in`, `tokens_out`, `latency_ms`, `success`, `error_message`, `started_at`, and `ended_at`

#### Scenario: Prompt content is reserved but empty in this phase

- **GIVEN** any sidecar produced by this phase
- **WHEN** a reader inspects the top-level `prompts` field
- **THEN** the field exists as an array
- **AND** the array is empty
- **AND** the schema accepts a populated array in a future phase without bumping `version`

#### Scenario: Higher version than understood fails with a clear error

- **GIVEN** a sidecar carrying a `version` value greater than the reader's supported version
- **WHEN** the cairn library reader attempts to parse the file
- **THEN** the call returns a clear error naming the expected and found versions
- **AND** the error does NOT silently ignore the unknown payload

### Requirement: Render the trace sidecar through `cflx trace`

The cflx workflow runner SHALL provide a `cflx trace <phase>` CLI command that reads the sidecar for the named archived phase and renders human and `--json` output per the CLI capability spec's "Produce stable human and JSON output" requirement. The command SHALL be a pure renderer; semantics SHALL remain owned by the writer of the sidecar and the cairn library schema.

#### Scenario: Default human output is labelled per stage

- **GIVEN** an archived phase with a populated sidecar
- **WHEN** the user runs `cflx trace <phase>` without `--json`
- **THEN** the output lists each stage on its own labelled section
- **AND** each section names the model identity, token counts, latency, success flag, and error message if any

#### Scenario: JSON output is the schema with promoted version

- **GIVEN** the same archived phase
- **WHEN** the user runs `cflx trace <phase> --json`
- **THEN** stdout contains exactly one JSON object
- **AND** the object includes a `schema_version` field with value `1`
- **AND** no terminal colour codes or extra logging are emitted

#### Scenario: Missing sidecar exits cleanly

- **GIVEN** an archived phase with no `.cflx-trace.json` file in its directory
- **WHEN** the user runs `cflx trace <phase>`
- **THEN** the command reports the missing sidecar with a clear message
- **AND** exits with code `1`

#### Scenario: Trace command does not own semantics

- **GIVEN** the `cflx trace` CLI command
- **WHEN** the command runs
- **THEN** it delegates to the cairn library's `TraceSidecar` reader
- **AND** does not parse or interpret the sidecar payload outside of that reader
