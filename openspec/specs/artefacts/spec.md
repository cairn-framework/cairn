# Artefact Capability Spec

## Requirements

### Requirement: Load all v1 artefact types

The scanner SHALL load contracts, todos, decisions, reviews, research, and sources into the ontology.

#### Scenario: Artefact registry loads configured pointers

- **GIVEN** a node with pointers to contract, todos, decisions, reviews, research, and sources
- **WHEN** `cairn scan` runs
- **THEN** the artefact registry loads each supported type
- **AND** attaches parsed records to the node or global provenance index

#### Scenario: Missing required frontmatter fails

- **GIVEN** an artefact file missing a required frontmatter field
- **WHEN** the loader parses the file
- **THEN** it emits a structural error naming the field and file path

### Requirement: Enforce artefact integrity rules

Artefact validation SHALL classify findings according to the two-chain model.

#### Scenario: Decision provenance is linked

- **GIVEN** a decision with `informed_by` entries pointing to research and source IDs
- **WHEN** validation runs
- **THEN** every referenced research or source ID resolves
- **AND** unresolved references produce rationale tensions

#### Scenario: Source checksum is verified

- **GIVEN** a `verified` source with a local file and SHA-256
- **WHEN** validation runs
- **THEN** the computed hash matches the sidecar value
- **AND** a mismatch is reported as a structural error

#### Scenario: Review subtype is validated

- **GIVEN** a review artefact
- **WHEN** validation runs
- **THEN** `review_type` is one of `human`, `agent_introspective`, or `agent_cross_model`
- **AND** a missing `review_type` defaults to `human`

### Requirement: Query artefacts by node and rationale chain

The CLI SHALL expose artefact queries that preserve provenance and authority boundaries.

#### Scenario: Rationale returns the why chain

- **GIVEN** a node with accepted decisions linked to research and sources
- **WHEN** the user runs `cairn rationale <node>`
- **THEN** the response includes accepted decisions for the node and direct neighbours
- **AND** includes linked research
- **AND** includes linked sources

#### Scenario: Status composes current work

- **GIVEN** a project with open todos and scan log entries
- **WHEN** the user runs `cairn status`
- **THEN** the response includes open todos grouped by node
- **AND** includes recent `.cairn/log.md` entries
- **AND** includes an active changes section

#### Scenario: Neighbourhood defaults remain lean

- **GIVEN** a node with every artefact type attached
- **WHEN** the user runs `cairn neighbourhood <node>` without include flags
- **THEN** the response includes contracts and accepted decisions
- **AND** excludes todos, research, reviews, deprecated decisions, and active changes
