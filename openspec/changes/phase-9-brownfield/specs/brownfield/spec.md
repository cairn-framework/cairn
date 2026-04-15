# Brownfield Capability Spec

## ADDED Requirements

### Requirement: Generate initial Cairn state from code

Cairn SHALL create a reviewable change directory for first-time adoption of an existing codebase.

#### Scenario: Candidate heuristics are deterministic

- **GIVEN** a repository directory with three supported source files and mostly internal imports
- **WHEN** `cairn init --from-code` extracts candidates
- **THEN** the directory becomes a candidate node
- **AND** confidence is computed from the documented coupling score bands

#### Scenario: Init creates brownfield change

- **GIVEN** a repository without `cairn.dsl`
- **WHEN** the user runs `cairn init --from-code`
- **THEN** Cairn creates `meta/changes/brownfield-init/`
- **AND** writes a proposal
- **AND** writes `dsl.delta` with proposed nodes and edges
- **AND** writes stub contracts
- **AND** does not write main `cairn.dsl`

#### Scenario: Existing generated change is protected

- **GIVEN** `meta/changes/brownfield-init/` already exists
- **WHEN** the user runs `cairn init --from-code`
- **THEN** Cairn exits with code `1`
- **AND** does not overwrite the existing change

### Requirement: Refine existing Cairn state from code changes

Cairn SHALL propose deltas against an existing DSL.

#### Scenario: Refine proposes additions

- **GIVEN** a project with existing `cairn.dsl`
- **AND** new source directories are present
- **WHEN** the user runs `cairn refine`
- **THEN** Cairn creates a change directory
- **AND** the `dsl.delta` contains added nodes or edges only for detected changes

#### Scenario: Refine does not replace current truth

- **GIVEN** a project with existing `cairn.dsl`
- **WHEN** `cairn refine` completes
- **THEN** current-truth queries still read the original DSL
- **AND** proposed changes appear only through change-aware queries

### Requirement: Keep human review authoritative

Brownfield output SHALL remain proposed until archived.

#### Scenario: False positive can be removed

- **GIVEN** generated brownfield output contains a candidate node
- **WHEN** the human deletes that node from the generated change before archive
- **THEN** archive applies only the remaining proposed operations
- **AND** Cairn does not regenerate the deleted candidate during archive

### Requirement: Expose brownfield commands through MCP

Brownfield commands SHALL register with the shared MCP query tool registry as mutation-capable tools.

#### Scenario: Brownfield MCP tools require mutating mode

- **GIVEN** the MCP server starts in default mode after Phase 9
- **WHEN** an MCP client lists tools
- **THEN** `cairn_init_from_code` is not listed
- **AND** `cairn_refine` is not listed

#### Scenario: Brownfield MCP tools appear in mutating mode

- **GIVEN** the MCP server starts with mutating tools enabled after Phase 9
- **WHEN** an MCP client lists tools
- **THEN** `cairn_init_from_code` is listed
- **AND** `cairn_refine` is listed
