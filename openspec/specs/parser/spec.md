# Parser capability spec

## Purpose

Define parser behaviour for the active Rust implementation of the Cairn DSL.
The parser is authoritative for turning authored architectural intent into
typed AST structures that later phases reconcile with artefacts and reality.

## Requirements

### Requirement: Parse the Cairn DSL grammar

The parser SHALL accept a `.dsl` file conforming to the Cairn grammar and
produce a typed AST, or raise a structured parse error.

#### Scenario: Valid DSL with nested nodes

- **GIVEN** a `.dsl` file declaring a System containing Containers and Modules with IDs, paths, artefact pointers, tags, and edges
- **WHEN** the parser runs
- **THEN** it produces an AST preserving node nesting and edge declarations
- **AND** every parsed item carries a source span

#### Scenario: Unknown declaration keyword

- **GIVEN** a `.dsl` file containing a top-level declaration `Service Foo`
- **WHEN** the parser runs
- **THEN** it raises a parse error naming the unknown keyword
- **AND** lists valid declaration keywords

#### Scenario: Missing required node field

- **GIVEN** a Module declaration omits its required `id`
- **WHEN** the parser runs
- **THEN** it raises a structured parse error with the offending source span

### Requirement: Validate structural references after parse

After producing the AST, the parser and ontology builder SHALL verify structural
identity and references before exposing a successful ontology.

#### Scenario: Duplicate IDs fail

- **GIVEN** a `.dsl` file declaring two nodes with the same ID
- **WHEN** validation runs
- **THEN** it raises a structural error naming the duplicated ID and source spans

#### Scenario: Edge to unknown node fails

- **GIVEN** a `.dsl` file declaring an edge to an undeclared node ID
- **WHEN** validation runs
- **THEN** it raises a structural error naming the unknown ID and edge span

#### Scenario: Path list is normalized

- **GIVEN** a Module declares `path ["./core-rust", "./core-ts"]`
- **WHEN** the parser runs
- **THEN** the AST stores the paths as an ordered path list
- **AND** downstream code does not need to special-case single-path and multi-path syntax
