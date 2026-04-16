# Distribution Capability Spec

## ADDED Requirements

### Requirement: Provide LSP access to Cairn ontology facts

Cairn SHALL provide an LSP server over the same parser, ontology, and query APIs as CLI and MCP.

#### Scenario: LSP binary root is strict

- **GIVEN** Phase 10 has been implemented
- **WHEN** a headless agent inspects the `cairn-lsp` package configuration
- **THEN** the package is a member of the Cargo workspace
- **AND** its `[lints]` section contains `workspace = true`
- **AND** the `cairn-lsp` binary root contains no crate-level lint attributes

#### Scenario: Diagnostics match lint

- **GIVEN** a project with parser, structural, interface, and tension findings
- **WHEN** an editor requests diagnostics through `cairn-lsp`
- **THEN** the diagnostics match `cairn lint --json` finding codes and spans

#### Scenario: Hover returns node context

- **GIVEN** a DSL edge references `saas.api.auth`
- **WHEN** the editor requests hover on the ID
- **THEN** the response includes node name, description, state, paths, artefact counts, and findings summary

#### Scenario: Definition resolves edge endpoint

- **GIVEN** a DSL edge references `saas.api.auth`
- **WHEN** the editor requests definition for that ID
- **THEN** the response points to the node declaration span

### Requirement: Package agent integrations

Cairn SHALL document and package agent-facing integrations around existing CLI and MCP behavior.

#### Scenario: Claude Code setup is documented

- **GIVEN** a fresh checkout
- **WHEN** the user reads distribution documentation
- **THEN** it explains how to run `cairn`
- **AND** how to start `cairn-mcp`
- **AND** how agents consume project context and rules

#### Scenario: Example project exercises major capabilities

- **GIVEN** the packaged example project
- **WHEN** release validation runs against it
- **THEN** it exercises DSL parse, artefacts, changes, hooks, MCP queries, summariser disabled/default behavior, brownfield fixture generation, LSP diagnostics, and fixture non-code reconciler observations

### Requirement: Expose reconciler extension points

Cairn SHALL document and test non-code reconciler registration.

#### Scenario: Fixture reconciler contributes observations

- **GIVEN** a fixture non-code reconciler is registered
- **WHEN** `cairn scan` runs
- **THEN** its observations enter the ontology through the shared reconciler interface
- **AND** query and diagnostic output can include those observations
- **AND** it does not create new DSL nodes outside the brownfield change workflow
