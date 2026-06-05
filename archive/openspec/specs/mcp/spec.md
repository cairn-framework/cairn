# MCP Capability Spec

## Purpose

The MCP capability exposes Cairn's query layer as a Model Context Protocol server, enabling external clients to read the codebase graph, artefacts, and project context through a standardized tool interface.

## Requirements

### Requirement: Expose Cairn queries through MCP

Cairn SHALL provide an MCP server that wraps the existing query layer.

#### Scenario: MCP get uses shared query API

- **GIVEN** a valid project
- **WHEN** an MCP client calls `cairn_get` with a node ID
- **THEN** the server returns the same node data as CLI JSON output
- **AND** does not parse human-readable CLI output

#### Scenario: Tool registration covers read-only queries

- **GIVEN** the MCP server starts in default mode
- **WHEN** an MCP client lists tools
- **THEN** read-only core, artefact, docstring, status, rationale, and change-display tools are listed
- **AND** scan, archive, and rename mutation tools are not listed

#### Scenario: Tool registration is registry-backed

- **GIVEN** a query command is registered in the shared query tool registry with MCP metadata
- **WHEN** the MCP server starts
- **THEN** the server exposes the command according to its registered safety class
- **AND** no server-local static tool list is required for that command

### Requirement: Enforce strict crate attributes for the MCP binary

The `cairn-mcp` binary root SHALL use the same strict crate attributes as the Phase 0 crate roots.

#### Scenario: MCP binary root is strict

- **GIVEN** Phase 7 has been implemented
- **WHEN** a headless agent inspects the `cairn-mcp` package configuration
- **THEN** the package is a member of the Cargo workspace
- **AND** its `[lints]` section contains `workspace = true`
- **AND** the `cairn-mcp` binary root contains no crate-level lint attributes

### Requirement: Compose project context and rules into responses

MCP responses SHALL include project-level context and relevant rules from `cairn.config.yaml`.

#### Scenario: Config context is present

- **GIVEN** `cairn.config.yaml` contains a `context` block
- **WHEN** an MCP query succeeds
- **THEN** the response includes the context block in `project_context`

#### Scenario: Artefact rules are relevant

- **GIVEN** `cairn.config.yaml` contains a `rules.decision` block
- **WHEN** an MCP client calls `cairn_decisions`
- **THEN** the response includes the decision rule

### Requirement: Return stable structured errors

The MCP server SHALL expose Cairn errors with stable codes.

#### Scenario: Node is missing

- **GIVEN** a valid project without node `missing.node`
- **WHEN** an MCP client calls `cairn_get` for `missing.node`
- **THEN** the server returns an error code for missing node
- **AND** includes a human-readable message
- **AND** includes suggested matches when available
