# Semantic Reconciliation Capability Spec

## ADDED Requirements

### Requirement: Validate declared edges against observed dependencies

Cairn SHALL compare blueprint edges with source-level dependency observations and surface divergence as rationale tensions.

#### Scenario: Declared edge lacks observed dependency

- **GIVEN** the blueprint declares `saas.api.auth -> saas.db`
- **AND** the reconciler observes no dependency from owned Auth files to owned DB files
- **WHEN** `cairn scan` runs
- **THEN** Cairn reports an edge divergence rationale tension
- **AND** the structural hook remains passable when no blocking findings exist

#### Scenario: Observed dependency lacks declared edge

- **GIVEN** Auth source imports Billing source
- **AND** the blueprint has no edge from Auth to Billing
- **WHEN** `cairn lint` runs
- **THEN** Cairn reports a rationale tension naming both nodes and source span

### Requirement: Detect docstring drift against map facts

Cairn SHALL compare supported authored docstring facts with map facts.

#### Scenario: Rust module fact lines are parsed

- **GIVEN** a Rust `lib.rs` file with module-level `//!` comments containing `Cairn-ID: saas.api.auth`, two `Cairn-Depends:` lines, `Cairn-Tags: auth, api`, and `Cairn-Contract: ./meta/contracts/api/auth.md#Public interface`
- **WHEN** docstring drift detection runs
- **THEN** Cairn parses the exact case-sensitive fact keys
- **AND** combines multiple dependency lines into one dependency fact set

#### Scenario: Docstring dependency contradicts graph

- **GIVEN** a module docstring declares dependency `saas.db`
- **AND** the map has no edge from the module to `saas.db`
- **WHEN** `cairn scan` runs
- **THEN** Cairn reports docstring drift as a rationale tension

#### Scenario: Unknown docstring ID is advisory

- **GIVEN** a module docstring declares `Cairn-Depends: missing.node`
- **WHEN** docstring drift detection runs
- **THEN** Cairn reports a rationale tension with the source span

#### Scenario: Free-form prose is ignored

- **GIVEN** a module docstring contains prose with no supported Cairn fact lines
- **WHEN** docstring drift detection runs
- **THEN** Cairn does not attempt to validate the prose

### Requirement: Generate language-aware docstring templates

The CLI SHALL generate docstring templates grounded in map facts.

#### Scenario: Rust template is generated

- **GIVEN** a valid module node
- **WHEN** the user runs `cairn docstring <node> --language rust`
- **THEN** the command emits a Rust doc comment template
- **AND** includes node ID, description, declared dependencies, tags, and contract headings

#### Scenario: Unsupported language fails clearly

- **GIVEN** a valid module node
- **WHEN** the user runs `cairn docstring <node> --language elixir`
- **THEN** the command exits with code `1`
- **AND** lists Rust, Python, TypeScript, and Go as supported languages
