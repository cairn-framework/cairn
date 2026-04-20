# Terminology Rename Capability Spec

## Requirements

### Requirement: Authored blueprint file

The framework SHALL recognize the authored declarative architecture file by the extension `.blueprint` and refer to it in all user-facing surfaces as the "blueprint."

#### Scenario: Parsing a blueprint file

- **GIVEN** a project directory containing `cairn.blueprint`
- **WHEN** the user runs `cairn scan`
- **THEN** the parser loads `cairn.blueprint` as the authored structural declaration
- **AND** the CLI refers to it as "blueprint" in all prompts, errors, and help text

#### Scenario: Rejecting the legacy extension

- **GIVEN** a project directory containing only `cairn.dsl` (no `cairn.blueprint`)
- **WHEN** the user runs `cairn scan`
- **THEN** the CLI reports that no blueprint file was found
- **AND** the error message suggests renaming to `cairn.blueprint`

#### Scenario: Collision between legacy and new extension

- **GIVEN** a project directory containing BOTH `cairn.dsl` and `cairn.blueprint`
- **WHEN** the user runs `cairn scan`
- **THEN** the parser loads `cairn.blueprint` and ignores `cairn.dsl`
- **AND** the CLI emits a warning that `cairn.dsl` is unused and should be removed

### Requirement: Map terminology for the reconciled graph

The reconciled graph of blueprint, artefacts, and reality-layer data SHALL be referred to in all user-facing documentation, CLI output, and generated files as the "map."

#### Scenario: Displaying the reconciled graph to users

- **GIVEN** a scanned project with a current reconciliation state
- **WHEN** the user runs `cairn status` or `cairn neighbourhood <node>`
- **THEN** the CLI output refers to the reconciled graph as "map" in prose
- **AND** no user-facing output uses the term "ontology"

### Requirement: Generated map snapshot filename

The scanner SHALL emit the generated snapshot of the current reconciled map to `map.md` at the project root.

#### Scenario: Scan produces a map snapshot

- **GIVEN** a project with a valid blueprint
- **WHEN** the scanner runs to completion
- **THEN** a file named `map.md` exists at the project root containing the current map summary
- **AND** no file named `index.md` is produced by the scanner

### Requirement: Preserved technical taxonomy

The rename SHALL NOT affect the load-bearing technical taxonomy defined in spec v0.6. The full list of preserved terms is enumerated in the proposal's Out of Scope section.

#### Scenario: Finding classes remain distinct

- **GIVEN** a scan produces findings of multiple classes
- **WHEN** the CLI reports them
- **THEN** `structural error`, `interface contradiction`, and `rationale tension` are reported as three distinct classes
- **AND** no rename collapses `rationale tension` into `contradiction`

#### Scenario: Reconciler interface retained

- **GIVEN** a domain plugin registers a reconciler
- **WHEN** the scanner dispatches to it
- **THEN** the plugin implements the `Reconciler` trait
- **AND** the trait remains distinct from the scanner engine and the `scan` CLI verb

#### Scenario: Change-directory delta semantics retained

- **GIVEN** a proposed modification in `meta/changes/<change-id>/`
- **WHEN** the change is applied
- **THEN** delta operations `ADDED`, `MODIFIED`, `REMOVED`, `RENAMED` are recognized verbatim
- **AND** the directory is referred to as a "change," not a "draft" or "proposal"

### Requirement: Spec version bump

The rename SHALL bump the spec version header from `v0.6` to `v0.7` to mark the vocabulary boundary.

#### Scenario: Spec version reflects the rename

- **GIVEN** the completed rename
- **WHEN** a reader opens `docs/spec.md`
- **THEN** the version header reads `v0.7`
- **AND** §0 Vocabulary defines `blueprint` and `map` as the primary terms
