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

#### Scenario: Local command protocol is stable

- **GIVEN** summariser mode is `local_command`
- **WHEN** the user runs `cairn summarise <node>`
- **THEN** Cairn sends one `SummariserRequest` JSON object to the configured command stdin
- **AND** reads one `SummariserResponse` JSON object from stdout
- **AND** stores only `draft_text` as generated prose

#### Scenario: Backend failure does not create draft

- **GIVEN** the configured local command exits non-zero or exceeds `timeout_ms`
- **WHEN** the user runs `cairn summarise <node>`
- **THEN** Cairn reports a backend failure
- **AND** does not create or modify a draft

### Requirement: Require explicit draft resolution

Cairn SHALL require accept, edit, or discard before generated text affects contracts.

#### Scenario: Accept applies draft

- **GIVEN** a pending contract draft
- **WHEN** the user runs `cairn draft accept <draft-id>`
- **THEN** Cairn validates that the draft is a valid contract for the target node
- **AND** replaces the target contract with draft text
- **AND** records the current interface hash

#### Scenario: Edit creates editable draft without applying

- **GIVEN** a pending contract draft
- **WHEN** the user runs `cairn draft edit <draft-id>`
- **THEN** Cairn writes editable draft content under `.cairn/state/summariser/editable/`
- **AND** leaves the target contract unchanged

#### Scenario: Edited accept applies editable content

- **GIVEN** a pending contract draft with an editable draft file
- **WHEN** the user runs `cairn draft accept <draft-id> --edited`
- **THEN** Cairn validates that the editable draft file is a valid contract for the target node
- **AND** replaces the target contract with the editable draft file content
- **AND** records the current interface hash

#### Scenario: Invalid accepted draft rolls back

- **GIVEN** a pending contract draft whose text has invalid frontmatter or references a different node
- **WHEN** the user runs `cairn draft accept <draft-id>`
- **THEN** Cairn exits with code `1`
- **AND** restores the original contract file
- **AND** leaves the draft pending

#### Scenario: Discard leaves contradiction unresolved

- **GIVEN** a pending contract draft
- **WHEN** the user runs `cairn draft discard <draft-id>`
- **THEN** Cairn marks the draft discarded
- **AND** leaves the original interface contradiction unresolved

#### Scenario: Generation never auto-applies

- **GIVEN** a backend returns generated text
- **WHEN** draft generation completes
- **THEN** no contract file is modified until a resolution command runs

### Requirement: Expose summariser commands through MCP

Summariser commands SHALL register with the shared MCP query tool registry.

#### Scenario: Draft query tools are read-only MCP tools

- **GIVEN** the MCP server starts in default mode after Phase 8
- **WHEN** an MCP client lists tools
- **THEN** `cairn_drafts` is listed
- **AND** `cairn_draft_show` is listed

#### Scenario: Draft mutation tools require mutating MCP mode

- **GIVEN** the MCP server starts in default mode after Phase 8
- **WHEN** an MCP client lists tools
- **THEN** `cairn_summarise`, `cairn_draft_accept`, `cairn_draft_edit`, and `cairn_draft_discard` are not listed
