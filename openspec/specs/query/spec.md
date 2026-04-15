# Query capability spec

## Requirements

### Requirement: Answer get, neighbourhood, dependents, depends, and order queries against the ontology

The query layer SHALL expose five typed queries over the in-memory ontology built from a parsed DSL file.

#### Scenario: get by ID

- **GIVEN** an ontology built from a valid DSL file containing a Module with ID `saas.api.auth`
- **WHEN** `get` is called with `saas.api.auth`
- **THEN** it returns an object with the node's ID, name, description, tags, path (which may be string or array), and artefact pointers
- **AND** the return shape is stable across invocations (safe for agent consumption)

#### Scenario: get by name fallback

- **GIVEN** an ontology containing a Module named "Auth" with ID `saas.api.auth`
- **WHEN** `get` is called with `"Auth"` (no ID prefix)
- **THEN** the query layer resolves the name to the ID and returns the same result as `get saas.api.auth`

#### Scenario: neighbourhood returns connected nodes

- **GIVEN** an ontology where `saas.api.auth` has one outbound edge to `saas.db` and one inbound edge from `saas.api.billing`
- **WHEN** `neighbourhood saas.api.auth` is called
- **THEN** the result contains the central node, plus an `outbound` array with one entry (target: `saas.db`, description from the edge) and an `inbound` array with one entry (source: `saas.api.billing`, description from the edge)
- **AND** each connected node's metadata is included inline, not just the ID

#### Scenario: dependents returns inbound edges

- **GIVEN** an ontology where `saas.db` is edged-into by `saas.api.auth` and `saas.api.billing`
- **WHEN** `dependents saas.db` is called
- **THEN** the result is a list containing both depending nodes with their metadata

#### Scenario: dependents transitive

- **GIVEN** an ontology where `saas.db` is depended on by `saas.api.auth`, which is itself depended on by `saas.api.admin`
- **WHEN** `dependents saas.db --transitive` is called
- **THEN** the result contains both `saas.api.auth` (direct) and `saas.api.admin` (transitive)
- **AND** the output distinguishes direct from transitive dependents

#### Scenario: depends returns outbound edges

- **GIVEN** an ontology where `saas.api.auth` edges to `saas.db` and `saas.crypto`
- **WHEN** `depends saas.api.auth` is called
- **THEN** the result is a list containing both target nodes with their metadata

#### Scenario: order returns dependency-tier groups

- **GIVEN** an ontology with a node graph that has three layers: Tier 0 with no outbound edges, Tier 1 depending only on Tier 0, Tier 2 depending on Tier 1
- **WHEN** `order` is called
- **THEN** the result is a list of tiers, tier 0 first, each tier a list of nodes whose outbound targets are all in prior tiers

#### Scenario: order detects cycles

- **GIVEN** an ontology with a cycle: A → B → C → A
- **WHEN** `order` is called
- **THEN** the query fails with a structural error naming all three nodes as cycle participants
- **AND** exits with code 1

#### Scenario: order with scope

- **GIVEN** an ontology containing nodes with IDs `saas.api.auth`, `saas.api.billing`, `infra.db`, `infra.cache`
- **WHEN** `order --scope saas.` is called
- **THEN** the result contains only nodes whose ID starts with `saas.`
- **AND** edges to out-of-scope nodes are ignored for tier computation

### Requirement: Support both human-readable and JSON output

Every query SHALL support a `--json` flag that produces a machine-readable representation with a stable schema.

#### Scenario: JSON output is a stable object

- **GIVEN** any query result
- **WHEN** `--json` is passed
- **THEN** stdout contains exactly one JSON object, no pretty-printing artefacts, no terminal colour codes, and no extra logging
- **AND** the schema is versioned (top-level `schema_version` field) for future compatibility
