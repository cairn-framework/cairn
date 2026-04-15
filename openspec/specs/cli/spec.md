# CLI capability spec

## Purpose

Define the command-line interface for the active Rust implementation. The CLI is
the primary user and agent entrypoint for querying, scanning, validating, and
operating on Cairn ontology state.

## Requirements

### Requirement: Expose kernel queries as command-line commands

The CLI SHALL expose the active Rust kernel query surface over a DSL file loaded
from disk.

#### Scenario: Default file resolution

- **GIVEN** a working directory containing `cairn.dsl`
- **WHEN** the user runs `cairn get some.node`
- **THEN** the CLI loads `./cairn.dsl` by default

#### Scenario: Custom file via flag

- **GIVEN** the user passes `--file ./other.dsl`
- **WHEN** any query command runs
- **THEN** the CLI loads the provided path instead of the default

#### Scenario: Node-not-found exits cleanly

- **GIVEN** a valid DSL file without a node matching the query argument
- **WHEN** the user runs `cairn get nonexistent`
- **THEN** the CLI reports the missing node with deterministic closest-match suggestions
- **AND** exits with code `1`

### Requirement: Produce stable human and JSON output

Every CLI command SHALL provide labelled human-readable output and a `--json`
mode with stable machine-readable schemas.

#### Scenario: Human output is structured

- **GIVEN** a successful query
- **WHEN** the CLI renders without `--json`
- **THEN** output uses labelled sections appropriate to the command
- **AND** avoids ANSI colour codes unless stdout is a TTY

#### Scenario: JSON output is clean

- **GIVEN** a successful query
- **WHEN** `--json` is passed
- **THEN** stdout contains exactly one JSON object
- **AND** the object includes a schema version
- **AND** no terminal colour codes or extra logging are emitted

### Requirement: Keep CLI backed by shared services

The CLI SHALL be a rendering and process boundary over shared library services,
not the sole owner of query semantics.

#### Scenario: CLI command delegates to library service

- **GIVEN** a CLI query command
- **WHEN** the command runs
- **THEN** it delegates to a typed library query or service API
- **AND** later protocol wrappers can call the same API without parsing CLI text
