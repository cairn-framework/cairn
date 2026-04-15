# Summariser Capability Spec

## ADDED Requirements

### Requirement: Generate drafts through an optional backend

Cairn SHALL support a configurable summariser that is disabled by default.

#### Scenario: Disabled summariser does not call backend

- **GIVEN** summariser mode is `disabled`
- **WHEN** an interface contradiction is detected
- **THEN** Cairn records the contradiction
- **AND** does not generate a draft

#### Scenario: Configured backend creates draft

- **GIVEN** a summariser backend is configured
- **AND** an interface contradiction exists for a node
- **WHEN** the user runs `cairn summarise <node>`
- **THEN** Cairn builds grounded prompt inputs
- **AND** stores a pending draft under `.cairn/state/summariser/`

### Requirement: Require explicit draft resolution

Cairn SHALL require accept, edit, or discard before generated text affects contracts.

#### Scenario: Accept applies draft

- **GIVEN** a pending contract draft
- **WHEN** the user runs `cairn draft accept <draft-id>`
- **THEN** Cairn replaces the target contract with draft text
- **AND** records the current interface hash

#### Scenario: Discard leaves contradiction unresolved

- **GIVEN** a pending contract draft
- **WHEN** the user runs `cairn draft discard <draft-id>`
- **THEN** Cairn marks the draft discarded
- **AND** leaves the original interface contradiction unresolved

#### Scenario: Generation never auto-applies

- **GIVEN** a backend returns generated text
- **WHEN** draft generation completes
- **THEN** no contract file is modified until a resolution command runs
