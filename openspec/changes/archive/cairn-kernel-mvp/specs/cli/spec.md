# CLI capability spec

## ADDED Requirements

### Requirement: Expose the query layer as a command-line tool

The CLI SHALL expose `get`, `neighbourhood`, `dependents`, `depends`, and `order` as subcommands operating on a DSL file loaded from disk.

#### Scenario: Default file resolution

- **GIVEN** a working directory containing a `cairn.dsl` file
- **WHEN** the user runs `cairn get some-node`
- **THEN** the CLI loads `./cairn.dsl` by default

#### Scenario: Custom file via flag

- **GIVEN** the user passes `--file ./other.dsl`
- **WHEN** any subcommand runs
- **THEN** the CLI loads the provided path instead of the default

#### Scenario: Parse error surfaces cleanly

- **GIVEN** a malformed DSL file
- **WHEN** any subcommand runs
- **THEN** the CLI prints the parse error to stderr with source position
- **AND** exits with code 1
- **AND** does not attempt to execute the query

#### Scenario: Node-not-found exits cleanly

- **GIVEN** a valid DSL file that does not contain a node matching the user's query argument
- **WHEN** the user runs `cairn get nonexistent`
- **THEN** the CLI prints "Node not found: nonexistent" to stderr
- **AND** suggests the three closest matching IDs by edit distance
- **AND** exits with code 1

### Requirement: Produce output readable both to humans and to agents

The CLI SHALL provide a default human-readable output format and a `--json` flag producing machine-readable output with a stable schema.

#### Scenario: Human output uses structured layout

- **GIVEN** a successful query
- **WHEN** the CLI renders the result without `--json`
- **THEN** the output uses clearly labelled sections (node header, tags line, artefacts list, edges grouped by direction)
- **AND** avoids ANSI colour codes unless stdout is a TTY
