# CLI capability spec

## Purpose

Define the command-line interface for the active Rust implementation. The CLI is
the primary user and agent entrypoint for querying, scanning, validating, and
operating on Cairn map state.

## Requirements

### Requirement: Expose kernel queries as command-line commands

The CLI SHALL expose the active Rust kernel query surface over a blueprint file loaded
from disk.

#### Scenario: Default file resolution

- **GIVEN** a working directory containing `cairn.blueprint`
- **WHEN** the user runs `cairn get some.node`
- **THEN** the CLI loads `./cairn.blueprint` by default

#### Scenario: Custom file via flag

- **GIVEN** the user passes `--file ./other.blueprint`
- **WHEN** any query command runs
- **THEN** the CLI loads the provided path instead of the default

#### Scenario: Node-not-found exits cleanly

- **GIVEN** a valid blueprint file without a node matching the query argument
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

### Requirement: Provide a full-graph export command

The CLI SHALL provide a `cairn export` command that emits the current map state, edge set, artefact corpus, and active-change list as a single payload in either machine-readable JSON or human-readable Markdown form. The command is a rendering boundary over a shared library service per the existing "CLI backed by shared services" requirement; it does not introduce new wire formats for individual records, it composes the existing per-record serialisations into one envelope.

#### Scenario: Default format is JSON

- **GIVEN** a working directory containing `cairn.blueprint`
- **WHEN** the user runs `cairn export --output ./out.json`
- **THEN** the command renders the export envelope as JSON
- **AND** writes the JSON payload to `./out.json`
- **AND** exits with code `0`

#### Scenario: Markdown format is selected via flag

- **GIVEN** a working directory containing `cairn.blueprint`
- **WHEN** the user runs `cairn export --format md --output ./out.md`
- **THEN** the command renders the export envelope as a Markdown document with `# Cairn Export` H1 and four H2 sections (`## Nodes`, `## Edges`, `## Artefacts`, `## Active Changes`) in that order
- **AND** writes the Markdown payload to `./out.md`
- **AND** exits with code `0`

#### Scenario: JSON envelope carries a schema version

- **GIVEN** a successful `cairn export --format json` run
- **WHEN** a reader inspects the written file
- **THEN** the first key of the top-level JSON object is `schema_version`
- **AND** the value is the integer `1`
- **AND** the object also contains `generated_at`, `blueprint_path`, `nodes`, `edges`, `artefacts`, and `changes` fields, in that order

#### Scenario: Markdown payload contains no em-dashes

- **GIVEN** a successful `cairn export --format md` run
- **WHEN** a reader inspects the written file
- **THEN** the document contains no U+2014 character
- **AND** any separators use period, colon, comma, or parentheses

#### Scenario: Output flag is required

- **GIVEN** the user runs `cairn export --format json` without `--output`
- **WHEN** the CLI parses the arguments
- **THEN** the command reports a labelled human-readable error naming the missing `--output` flag
- **AND** exits with code `1`

#### Scenario: Invalid format value is rejected

- **GIVEN** the user runs `cairn export --format csv --output ./out.csv`
- **WHEN** the CLI parses the arguments
- **THEN** the command reports a labelled human-readable error naming the rejected format value
- **AND** exits with code `1`

#### Scenario: Export is lifecycle-orthogonal

- **GIVEN** the working directory contains active changes, lint findings, drift, or rationale tensions
- **WHEN** the user runs `cairn export --format json --output ./out.json`
- **THEN** the command renders the snapshot regardless of any diagnostic state
- **AND** exits with code `0`
- **AND** the payload contains the current node, edge, artefact, and change records without filtering

#### Scenario: Render delegates to a shared library service

- **GIVEN** the export command runs
- **WHEN** the renderer assembles the envelope
- **THEN** it calls the typed library service `build_export(file, changes_dir) -> Result<ExportEnvelope, CairnError>`
- **AND** later protocol wrappers (MCP, LSP, webui) can call the same service without parsing CLI text
