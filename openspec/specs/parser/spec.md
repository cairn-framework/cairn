# Parser capability spec

## Requirements

### Requirement: Parse the v0.6 DSL grammar

The parser SHALL accept a `.dsl` file conforming to the Cairn v0.6 grammar (section 7 of the kernel spec) and produce an AST, or raise a structured parse error.

#### Scenario: Valid DSL with nested containers

- **GIVEN** a `.dsl` file declaring a System containing two Containers, each containing Modules with IDs, paths, and artefact pointers, plus edges between Modules
- **WHEN** the parser runs against the file
- **THEN** it produces an AST with the declared System at the root, Containers as children, Modules as leaves, and edges as a sibling array
- **AND** every node in the AST carries its source position (line and column of declaration)

#### Scenario: Missing required field

- **GIVEN** a `.dsl` file where a Module declaration omits the `id` field
- **WHEN** the parser runs
- **THEN** it raises a structured parse error with the source position of the offending Module, the message "Module 'X' is missing required field: id", and exit code 1

#### Scenario: Unknown keyword

- **GIVEN** a `.dsl` file containing a top-level declaration of `Service Foo` (not a valid keyword)
- **WHEN** the parser runs
- **THEN** it raises a parse error naming the unknown keyword and listing valid alternatives (`System`, `Container`, `Module`, `Actor`)

### Requirement: Validate IDs and edges after parse

After producing the AST, the parser SHALL verify ID uniqueness, path uniqueness across leaf nodes, ID format, and edge endpoint existence.

#### Scenario: Duplicate IDs

- **GIVEN** a `.dsl` file declaring two Modules with the same ID
- **WHEN** the parser runs
- **THEN** it raises a structural error naming both source positions and the duplicated ID

#### Scenario: Edge to unknown node

- **GIVEN** a `.dsl` file declaring an edge `foo.bar -> foo.baz "desc"` where `foo.baz` is not declared
- **WHEN** the parser runs
- **THEN** it raises a structural error naming the unknown ID and the source position of the edge

#### Scenario: Invalid ID format

- **GIVEN** a `.dsl` file with a Module declared `id "FooBar"` (violates lowercase-only rule)
- **WHEN** the parser runs
- **THEN** it raises a structural error showing the expected format pattern

#### Scenario: Multi-target path as list

- **GIVEN** a `.dsl` file where a Module declares `path ["./apps/core-rust", "./apps/core-ts"]`
- **WHEN** the parser runs
- **THEN** the resulting AST node carries a path array with both entries
- **AND** each path is validated for uniqueness across the project

#### Scenario: Single-path convenience

- **GIVEN** a `.dsl` file where a Module declares `path "./apps/core"` (no brackets)
- **WHEN** the parser runs
- **THEN** the AST node carries a path array with one entry
- **AND** downstream code does not need to special-case single vs multi path
