# CLI capability spec

## ADDED Requirements

### Requirement: Surface architectural islands and orphan inclusion

The CLI SHALL expose two surfaces over the new disconnected-subgraph query semantics defined in the query capability spec. The whole-graph form is `cairn islands`. The anchored form is `cairn neighbourhood <node> --include-orphans`. Both surfaces SHALL render human and `--json` output per the existing "Produce stable human and JSON output" requirement and SHALL delegate to the library query layer per the existing "Keep CLI backed by shared services" requirement.

#### Scenario: Islands command returns whole-graph component breakdown

- **GIVEN** a blueprint whose map contains two disconnected components
- **WHEN** the user runs `cairn islands`
- **THEN** the output lists two islands
- **AND** each island shows its node count and a representative node ID
- **AND** the representative is the lexicographically smallest ID in the component

#### Scenario: Islands JSON output is versioned

- **GIVEN** the same blueprint
- **WHEN** the user runs `cairn islands --json`
- **THEN** stdout contains exactly one JSON object
- **AND** the object includes a `schema_version` field
- **AND** the object includes an `islands` array of `{ node_count, representative }` records

#### Scenario: Neighbourhood with --include-orphans surfaces reverse-only nodes

- **GIVEN** a node `saas.api.auth` with one outbound edge and one inbound edge from a node that the default neighbourhood traversal does not return
- **WHEN** the user runs `cairn neighbourhood saas.api.auth --include-orphans`
- **THEN** the response includes the inbound-only node
- **AND** the same query without `--include-orphans` does NOT include that node

#### Scenario: Both forms delegate to the library query

- **GIVEN** the CLI commands `cairn islands` and `cairn neighbourhood --include-orphans`
- **WHEN** either command runs
- **THEN** it delegates to a typed library query function
- **AND** later protocol wrappers can call the same function without parsing CLI text

### Requirement: Render archived-phase trace sidecars via `cflx trace`

The cflx workflow runner SHALL expose a `cflx trace <phase>` CLI command that reads the trace sidecar for the named archived phase and renders human and `--json` output. The command SHALL follow the CLI human-and-JSON output convention and SHALL delegate sidecar parsing to the cairn library's `TraceSidecar` reader.

#### Scenario: Trace human output names each stage

- **GIVEN** an archived phase with a populated `.cflx-trace.json` sidecar
- **WHEN** the user runs `cflx trace <phase>`
- **THEN** the output lists labelled sections for `propose`, `apply`, `accept`, and `archive`
- **AND** each section reports model identity, token counts, latency, success, and error message if any

#### Scenario: Trace JSON output is exactly the sidecar payload

- **GIVEN** the same archived phase
- **WHEN** the user runs `cflx trace <phase> --json`
- **THEN** stdout contains exactly one JSON object
- **AND** the object includes a `schema_version` field
- **AND** no terminal colour codes or extra logging are emitted

#### Scenario: Trace exits non-zero when the sidecar is missing

- **GIVEN** an archived phase with no `.cflx-trace.json` file
- **WHEN** the user runs `cflx trace <phase>`
- **THEN** the CLI reports the missing sidecar with a clear message
- **AND** exits with code `1`
