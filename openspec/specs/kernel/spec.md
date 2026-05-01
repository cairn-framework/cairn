# Kernel Capability Spec

## Purpose

The Kernel capability parses the Cairn blueprint into a typed AST, reconciles declared structure against actual code reality through a trait-based reconciler interface, and assembles a queryable map graph. It serves as the foundational data layer and CLI query surface that all other Cairn capabilities build upon.

## Requirements

### Requirement: Parse the Cairn blueprint into a typed AST

The kernel SHALL parse the `docs/spec.md` section 7 blueprint grammar into typed Rust AST structures with stable source spans.

#### Scenario: Valid blueprint parses

- **GIVEN** a valid `cairn.blueprint` containing systems, containers, modules, tags, paths, contract pointers, and edges
- **WHEN** the parser runs
- **THEN** it returns an AST preserving node nesting, stable IDs, tags, fields, and edge descriptions
- **AND** every parsed item carries a source span

#### Scenario: Internal file ownership opt-in parses

- **GIVEN** a valid `cairn.blueprint` containing a Container with `path "./apps/api"` and `owns-files: true`
- **WHEN** the parser runs
- **THEN** the AST node records the internal ownership opt-in
- **AND** the source span for the `owns-files` field is preserved

#### Scenario: Malformed blueprint reports location

- **GIVEN** a blueprint file with an unterminated node declaration
- **WHEN** the parser runs
- **THEN** it returns a parse error containing file, line, column, expected token, and encountered token

### Requirement: Load Phase 1 configuration and ignore rules

The kernel SHALL load `cairn.config.yaml` when present while remaining forward-compatible with later config sections.

#### Scenario: Missing config uses defaults

- **GIVEN** a project without `cairn.config.yaml`
- **WHEN** `cairn scan` runs
- **THEN** the kernel uses built-in ignore defaults
- **AND** context and rules are empty

#### Scenario: Config ignore entries are applied

- **GIVEN** `cairn.config.yaml` contains `reconcilers[].config.ignore` entries
- **WHEN** the code reconciler scans files
- **THEN** those ignore entries are composed with built-in defaults, `.gitignore`, and `.cairnignore`
- **AND** protected Cairn paths are never ignored

#### Scenario: Later config sections are forward-compatible

- **GIVEN** `cairn.config.yaml` contains unknown top-level sections
- **WHEN** Phase 1 loads config
- **THEN** unknown sections do not fail validation
- **AND** known Phase 1 fields remain available to scanner and later query layers

### Requirement: Build a queryable map graph

The kernel SHALL transform the parsed blueprint, contract artefacts, and reconciler reports into a map graph.

#### Scenario: Graph indexes are available

- **GIVEN** a parsed project
- **WHEN** map construction completes
- **THEN** callers can resolve nodes by ID
- **AND** callers can resolve nodes by exact name when unambiguous
- **AND** callers can query parent, children, inbound edges, outbound edges, and claimed paths

#### Scenario: Internal node does not own files by default

- **GIVEN** a Container with `path "./apps/api"` and a child Module with `path "./apps/api/auth"`
- **AND** a Rust file exists directly under `./apps/api`
- **WHEN** `cairn lint` runs
- **THEN** the direct Container file is reported as orphaned because the Container lacks `owns-files: true`

#### Scenario: Internal node owns files by opt-in

- **GIVEN** a Container with `path "./apps/api"` and `owns-files: true`
- **AND** a Rust file exists directly under `./apps/api`
- **WHEN** `cairn lint` runs
- **THEN** the file is claimed by the Container
- **AND** descendant Module files still resolve by most-specific path

#### Scenario: Structural errors block map success

- **GIVEN** a project with duplicate IDs, path ties, invalid edge endpoints, or broken contract pointers for synced leaf nodes
- **WHEN** map construction or linting runs
- **THEN** the kernel reports structural errors with stable error codes
- **AND** CLI commands that require a valid map exit with code `1`

#### Scenario: Ghost-node missing contract is advisory

- **GIVEN** a ghost leaf node that declares a contract path whose file is missing
- **WHEN** map construction or linting runs
- **THEN** the kernel reports a warning with a stable code
- **AND** does not fail map construction solely because the ghost node contract is missing

#### Scenario: Dependency cycle does not block basic queries

- **GIVEN** a project whose blueprint edges contain a dependency cycle
- **WHEN** the user runs `cairn get <node>` or `cairn neighbourhood <node>`
- **THEN** the command can return data from the otherwise valid map
- **AND** the cycle remains available as a finding for `lint`, `order`, and later hook reuse

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
- **THEN** the map marks the node `ghost`
- **AND** the finding is reflected in `map.md`

#### Scenario: Reality file is unclaimed

- **GIVEN** a Rust source file under a claimed container path that no leaf module owns
- **WHEN** `cairn lint` or `cairn scan` runs
- **THEN** the kernel reports an orphaned-file structural error

### Requirement: Load contract artefacts only

Phase 1 SHALL implement contract artefact loading while excluding all other artefact types.

#### Scenario: Contract is attached to a node

- **GIVEN** a module with `contract "./meta/contracts/api/auth.md"`
- **AND** the contract frontmatter contains `node: saas.api.auth`
- **WHEN** the map is built
- **THEN** the contract is attached to `saas.api.auth`
- **AND** `cairn contract saas.api.auth` returns the parsed contract content

#### Scenario: Later artefact pointer is not interpreted

- **GIVEN** a module that declares `todos`, `decisions`, `research`, `reviews`, or `sources`
- **WHEN** Phase 1 builds the map
- **THEN** those pointers are retained as raw metadata
- **AND** no Phase 2 artefact validation is performed

### Requirement: Expose kernel CLI queries

The CLI SHALL expose the Phase 1 kernel query surface with human and JSON output.

#### Scenario: CLI uses shared query services

- **GIVEN** a Phase 1 CLI command
- **WHEN** the command executes
- **THEN** it calls the shared library query or service API
- **AND** does not implement query semantics only inside CLI rendering code

#### Scenario: Command registry records safety class

- **GIVEN** Phase 1 command metadata is registered
- **WHEN** the registry is inspected
- **THEN** `get`, `neighbourhood`, `contract`, `files`, `dependents`, `depends`, `order`, and `lint` are marked `read_only`
- **AND** `scan` is marked `mutating`

#### Scenario: Query commands succeed

- **GIVEN** a valid reconciled map
- **WHEN** the user runs `get`, `neighbourhood`, `contract`, `files`, `dependents`, `depends`, or `order`
- **THEN** the command returns the requested data
- **AND** `--json` returns a stable machine-readable schema

#### Scenario: Order reports cycles

- **GIVEN** a reconciled map whose dependency edges contain a cycle
- **WHEN** the user runs `cairn order`
- **THEN** the command exits with code `1`
- **AND** reports the cycle participants with a stable error code

#### Scenario: Scan writes generated outputs

- **GIVEN** a valid project
- **WHEN** the user runs `cairn scan`
- **THEN** Cairn writes `map.md`
- **AND** appends `.cairn/log.md`
- **AND** writes `.cairn/state/interface-hashes.json`

#### Scenario: Lint groups findings

- **GIVEN** a project with structural errors and warnings
- **WHEN** the user runs `cairn lint`
- **THEN** the output groups findings by class
- **AND** structural errors cause exit code `1`
