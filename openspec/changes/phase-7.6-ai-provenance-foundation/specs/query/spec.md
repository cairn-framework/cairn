# Query capability spec

## ADDED Requirements

### Requirement: Answer disconnected-subgraph queries

The query layer SHALL expose two surfaces over the disconnected-subgraph use case. The whole-graph form is `islands`, returning the connected-component breakdown of the entire map. The anchored form is the existing `neighbourhood` query extended with an `include_orphans` option that includes nodes reachable from the anchor only via reverse-direction edges that the default traversal would skip. Both surfaces SHALL share an internal connected-component traversal helper to avoid duplicate algorithms.

#### Scenario: Islands returns one entry per connected component

- **GIVEN** a map containing two disconnected components
- **WHEN** `islands` is called
- **THEN** the result contains two entries
- **AND** each entry carries a node count and a representative node ID
- **AND** the representative is the lexicographically smallest node ID within that component

#### Scenario: Islands handles the trivial single-component case

- **GIVEN** a map whose nodes form a single connected component
- **WHEN** `islands` is called
- **THEN** the result contains exactly one entry
- **AND** the entry's node count equals the total number of nodes in the map

#### Scenario: Neighbourhood with include_orphans surfaces inbound-only neighbours

- **GIVEN** a node with one outbound edge and one inbound-only neighbour that the default neighbourhood does not return
- **WHEN** `neighbourhood` is called for that node with `include_orphans: true`
- **THEN** the result includes both the outbound-edge neighbour and the inbound-only neighbour
- **AND** the same call with `include_orphans: false` includes only the outbound-edge neighbour

#### Scenario: Islands query response is versioned

- **GIVEN** any successful `islands` query result
- **WHEN** the result is serialized for an agent-facing interface
- **THEN** it includes a `schema_version` field
- **AND** no human formatting artefacts are included in the structured data
