# Kernel Capability Spec

## ADDED Requirements

### Requirement: Parse the Cairn DSL into a typed AST

The kernel SHALL parse the `docs/spec.md` section 7 DSL grammar into typed Rust AST structures with stable source spans.

#### Scenario: Valid DSL parses

- **GIVEN** a valid `cairn.dsl` containing systems, containers, modules, tags, paths, contract pointers, and edges
- **WHEN** the parser runs
- **THEN** it returns an AST preserving node nesting, stable IDs, tags, fields, and edge descriptions
- **AND** every parsed item carries a source span

#### Scenario: Malformed DSL reports location

- **GIVEN** a DSL file with an unterminated node declaration
- **WHEN** the parser runs
- **THEN** it returns a parse error containing file, line, column, expected token, and encountered token

### Requirement: Build a queryable ontology graph

The kernel SHALL transform the parsed DSL, contract artefacts, and reconciler reports into an ontology graph.

#### Scenario: Graph indexes are available

- **GIVEN** a parsed project
- **WHEN** ontology construction completes
- **THEN** callers can resolve nodes by ID
- **AND** callers can resolve nodes by exact name when unambiguous
- **AND** callers can query parent, children, inbound edges, outbound edges, and claimed paths

#### Scenario: Structural errors block ontology success

- **GIVEN** a project with duplicate IDs, path ties, invalid edge endpoints, broken contract pointers, or dependency cycles
- **WHEN** ontology construction or linting runs
- **THEN** the kernel reports structural errors with stable error codes
- **AND** CLI commands that require a valid ontology exit with code `1`

### Requirement: Reconcile Rust code reality through a trait interface

The kernel SHALL define a domain-agnostic reconciler trait and provide a Phase 1 Rust code reconciler.

#### Scenario: Rust module path is synced

- **GIVEN** a leaf module with a path containing Rust source files
- **WHEN** `cairn scan` runs
- **THEN** the code reconciler reports claimed files
- **AND** computes a deterministic interface fingerprint for public Rust items
- **AND** marks the node `synced`

#### Scenario: Declared path is missing

- **GIVEN** a leaf module whose declared path does not exist
- **WHEN** `cairn scan` runs
- **THEN** the ontology marks the node `ghost`
- **AND** the finding is reflected in `index.md`

#### Scenario: Reality file is unclaimed

- **GIVEN** a Rust source file under a claimed container path that no leaf module owns
- **WHEN** `cairn lint` or `cairn scan` runs
- **THEN** the kernel reports an orphaned-file structural error

### Requirement: Load contract artefacts only

Phase 1 SHALL implement contract artefact loading while excluding all other artefact types.

#### Scenario: Contract is attached to a node

- **GIVEN** a module with `contract "./meta/contracts/api/auth.md"`
- **AND** the contract frontmatter contains `node: saas.api.auth`
- **WHEN** the ontology is built
- **THEN** the contract is attached to `saas.api.auth`
- **AND** `cairn contract saas.api.auth` returns the parsed contract content

#### Scenario: Later artefact pointer is not interpreted

- **GIVEN** a module that declares `todos`, `decisions`, `research`, `reviews`, or `sources`
- **WHEN** Phase 1 builds the ontology
- **THEN** those pointers are retained as raw metadata
- **AND** no Phase 2 artefact validation is performed

### Requirement: Expose kernel CLI queries

The CLI SHALL expose the Phase 1 kernel query surface with human and JSON output.

#### Scenario: Query commands succeed

- **GIVEN** a valid reconciled ontology
- **WHEN** the user runs `get`, `neighbourhood`, `contract`, `files`, `dependents`, `depends`, or `order`
- **THEN** the command returns the requested data
- **AND** `--json` returns a stable machine-readable schema

#### Scenario: Scan writes generated outputs

- **GIVEN** a valid project
- **WHEN** the user runs `cairn scan`
- **THEN** Cairn writes `index.md`
- **AND** appends `.cairn/log.md`
- **AND** writes `.cairn/state/interface-hashes.json`

#### Scenario: Lint groups findings

- **GIVEN** a project with structural errors and warnings
- **WHEN** the user runs `cairn lint`
- **THEN** the output groups findings by class
- **AND** structural errors cause exit code `1`
