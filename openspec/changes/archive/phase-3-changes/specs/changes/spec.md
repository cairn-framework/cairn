# Change System Capability Spec

## ADDED Requirements

### Requirement: Isolate proposed modifications in change directories

Cairn SHALL treat `meta/changes/<change-id>/` directories as proposed modifications that do not affect current truth until archived.

#### Scenario: Active change is discovered

- **GIVEN** `meta/changes/add-notifications/proposal.md`
- **WHEN** the user runs `cairn changes`
- **THEN** the command lists `add-notifications`
- **AND** includes the proposal title and operation counts

#### Scenario: Default query ignores active changes

- **GIVEN** an active change that adds a module
- **WHEN** the user runs `cairn get <added-module-id>` without change flags
- **THEN** the command reports the node is not in current truth

### Requirement: Apply deltas atomically at archive

The archive command SHALL apply blueprint and artefact deltas atomically or leave the repository unchanged.

#### Scenario: Archive succeeds

- **GIVEN** a valid change directory
- **WHEN** the user runs `cairn archive <change>`
- **THEN** Cairn applies renamed, removed, modified, and added operations in that order
- **AND** runs a validation scan with the archiving change excluded from active-change discovery
- **AND** moves the change to `meta/changes/archive/YYYY-MM-DD-<change>/`
- **AND** refreshes generated outputs so the archived change is no longer listed as active
- **AND** appends an archive event to `.cairn/log.md`

#### Scenario: Archive rolls back on scan failure

- **GIVEN** a change that introduces a structural error
- **WHEN** the user runs `cairn archive <change>`
- **THEN** Cairn restores all mutated files
- **AND** leaves the active change directory in place
- **AND** exits with code `1`

#### Scenario: Archive output excludes merged change

- **GIVEN** a valid change directory
- **WHEN** `cairn archive <change>` succeeds
- **THEN** subsequent `cairn changes` output does not list the archived change as active
- **AND** generated status output reflects current truth after the archive move

### Requirement: Rename propagates references through a reviewable change

The rename command SHALL generate a change directory that updates all known references without mutating current truth.

#### Scenario: Rename creates a proposed change

- **GIVEN** a node `saas.api.auth`
- **WHEN** the user runs `cairn rename saas.api.auth saas.api.identity`
- **THEN** Cairn creates `meta/changes/rename-saas.api.auth-to-saas.api.identity/`
- **AND** writes a `blueprint.delta` containing the rename
- **AND** includes modified artefact copies for frontmatter references to the old ID
- **AND** leaves the main tree unchanged

### Requirement: Provide change-aware queries

The CLI SHALL expose explicit queries for proposed modifications.

#### Scenario: Show explains a change

- **GIVEN** an active change directory
- **WHEN** the user runs `cairn show <change>`
- **THEN** the command prints proposal text
- **AND** prints blueprint delta summary
- **AND** prints artefact operation summary

#### Scenario: Neighbourhood includes proposed operations by flag

- **GIVEN** an active change modifying a neighbour of `saas.api.auth`
- **WHEN** the user runs `cairn neighbourhood saas.api.auth --include-changes`
- **THEN** the response includes the proposed operation with an operation label
