# Query capability spec

## Purpose

Define map query semantics shared by the Rust CLI, MCP server, LSP server,
and future integrations. Query behaviour is defined over Cairn's reconciled
map, not over implementation-specific command output.

## Requirements

### Requirement: Answer structural map queries

The query layer SHALL expose typed queries over the in-memory map.

#### Scenario: Get by ID

- **GIVEN** a map containing a node with ID `saas.api.auth`
- **WHEN** `get` is called with `saas.api.auth`
- **THEN** it returns the node ID, name, description, tags, paths, state, and attached artefact metadata
- **AND** the return shape is stable across invocations

#### Scenario: Get by name fallback

- **GIVEN** a map containing one node named `Auth`
- **WHEN** `get` is called with `Auth`
- **THEN** the query layer resolves the unambiguous name to its stable ID
- **AND** returns the same node data as an ID lookup

#### Scenario: Neighbourhood returns connected nodes

- **GIVEN** a node with inbound and outbound edges
- **WHEN** `neighbourhood` is called for that node
- **THEN** the result includes the central node
- **AND** includes inbound and outbound edge entries with connected node metadata

#### Scenario: Dependency queries follow edge direction

- **GIVEN** a map with declared dependency edges
- **WHEN** `dependents` or `depends` is called
- **THEN** `dependents` returns nodes that edge into the target
- **AND** `depends` returns nodes the target edges to

#### Scenario: Order returns dependency tiers

- **GIVEN** an acyclic map graph
- **WHEN** `order` is called
- **THEN** it returns dependency-tier groups with tier `0` containing nodes with no outbound dependencies in scope

#### Scenario: Order detects cycles without poisoning basic queries

- **GIVEN** a map graph containing a dependency cycle
- **WHEN** `order` is called
- **THEN** the query fails with a structural error naming cycle participants
- **AND** basic node and neighbourhood queries can still read the otherwise valid map

### Requirement: Preserve machine-readable schemas

Queries exposed through CLI JSON, MCP, and LSP-backed APIs SHALL use stable
structured response models.

#### Scenario: JSON-compatible query result is versioned

- **GIVEN** any successful query result
- **WHEN** the result is serialized for an agent-facing interface
- **THEN** it includes a schema version
- **AND** no human formatting artefacts are included in the structured data

#### Scenario: Query structs are protocol-neutral

- **GIVEN** a query response is produced by the library
- **WHEN** it is rendered by the CLI or returned by a later protocol wrapper
- **THEN** both surfaces use the same typed response model
- **AND** no surface parses another surface's human-readable output
