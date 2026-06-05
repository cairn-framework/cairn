# Brownfield Capability Spec

## ADDED Requirements

### Requirement: Generate initial Cairn state from code

Cairn SHALL create a reviewable change directory for first-time adoption of an existing codebase.

#### Scenario: Discovery does not require existing blueprint

- **GIVEN** a repository without `cairn.blueprint`
- **WHEN** `cairn init --from-code` runs
- **THEN** Cairn scans supported source files using repository-wide discovery
- **AND** does not require claimed node paths from an existing map

#### Scenario: Candidate heuristics are deterministic

- **GIVEN** a repository directory with three supported source files and mostly internal imports
- **WHEN** `cairn init --from-code` extracts candidates
- **THEN** the directory becomes a candidate node
- **AND** confidence is computed from the documented coupling score bands

#### Scenario: Init creates brownfield change

- **GIVEN** a repository without `cairn.blueprint`
- **WHEN** the user runs `cairn init --from-code`
- **THEN** Cairn creates `openspec/changes/brownfield-init/`
- **AND** writes a proposal
- **AND** writes `blueprint.delta` with proposed nodes and edges
- **AND** writes stub contracts
- **AND** does not write main `cairn.blueprint`

#### Scenario: Existing generated change is protected

- **GIVEN** `openspec/changes/brownfield-init/` already exists
- **WHEN** the user runs `cairn init --from-code`
- **THEN** Cairn exits with code `1`
- **AND** does not overwrite the existing change

#### Scenario: Force replaces generated change

- **GIVEN** `openspec/changes/brownfield-init/` already exists
- **WHEN** the user runs `cairn init --from-code --force`
- **THEN** Cairn replaces the existing generated brownfield change directory atomically
- **AND** does not modify main `cairn.blueprint` or main `openspec/specs/` artefacts

### Requirement: Refine existing Cairn state from code changes

Cairn SHALL propose deltas against an existing blueprint.

#### Scenario: Refine proposes additions

- **GIVEN** a project with existing `cairn.blueprint`
- **AND** new source directories are present
- **WHEN** the user runs `cairn refine`
- **THEN** Cairn creates a change directory
- **AND** the `blueprint.delta` contains added nodes or edges only for detected changes

#### Scenario: Refine does not replace current truth

- **GIVEN** a project with existing `cairn.blueprint`
- **WHEN** `cairn refine` completes
- **THEN** current-truth queries still read the original blueprint
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

### Requirement: Suggest cross-cutting edges through the phase 7.6 queue

The brownfield generator SHALL emit AI-suggested edges into the suggested-edges queue file class shipped by `phase-7.6-ai-provenance-foundation`, populating provenance and leaving every entry in `pending` state for human triage.

#### Scenario: Suggest engine writes to the queue file

- **GIVEN** a brownfield change being authored by `cairn init --from-code` against a repository with multiple modules
- **WHEN** the suggest engine identifies a cross-cutting edge that the deterministic extractor did not infer
- **THEN** the edge is written as an entry in `openspec/changes/<change>/suggested-edges.json`
- **AND** the entry's `triage_state` is `pending`
- **AND** the entry's `provenance.trace_phase` names the running phase

#### Scenario: Suggest engine never auto-accepts

- **GIVEN** the suggest engine produces an edge with high computed confidence
- **WHEN** the entry is written to the queue file
- **THEN** the `triage_state` is `pending`
- **AND** no auto-accept policy promotes the entry to `accepted` without a human action

#### Scenario: Pending entries block archive through the CC002 gate

- **GIVEN** a brownfield change containing one or more pending entries in `suggested-edges.json`
- **WHEN** `cflx openspec validate <change> --strict` runs
- **THEN** the call fails with error code `CC002`
- **AND** the failure message names the pending count and the queue file path

#### Scenario: Refine emits suggested edges into the refine change directory

- **GIVEN** a project with an existing `cairn.blueprint` and new source directories that introduce cross-cutting edges
- **WHEN** the user runs `cairn refine`
- **AND** the suggest engine identifies at least one cross-cutting edge that the deterministic extractor did not infer
- **THEN** the entry is written to `openspec/changes/<refine-change>/suggested-edges.json` with `triage_state: "pending"`
- **AND** the entry's `provenance.stage` is `propose`
- **AND** archive of the refine change is blocked by `CC002` until every entry is triaged off `pending`

#### Scenario: Force-init aborts when pending entries exist

- **GIVEN** `openspec/changes/brownfield-init/` already exists and its `suggested-edges.json` contains one or more entries with `triage_state: "pending"`
- **WHEN** the user runs `cairn init --from-code --force`
- **THEN** Cairn aborts with a non-zero exit and a message naming the pending entries
- **AND** does not overwrite the existing change directory
- **AND** instructs the user to triage pending entries (or pass an explicit override flag) before re-running force-init

This guards the human triage queue: silent wipe on `--force` would destroy unreviewed AI suggestions and break the load-bearing constraint that triage is non-bypassable.

### Requirement: Run multi-round elicitation for brownfield onboarding

The `cflx-proposal` skill SHALL support a multi-round interview mode for brownfield onboarding sessions, persist intermediate state inside the change directory, resume across invocations, and write a final genesis transcript on completion.

#### Scenario: Session persists across invocations

- **GIVEN** an in-progress brownfield onboarding interview with answered turns and outstanding questions
- **WHEN** the session is suspended and the skill is re-invoked against the same change directory
- **THEN** the runner detects the existing session state inside the change directory
- **AND** resumes at the next outstanding turn rather than restarting

#### Scenario: Final transcript lands at the conventional path

- **GIVEN** a brownfield onboarding interview that the human marks complete
- **WHEN** the runner finalises the session
- **THEN** the transcript is written to `openspec/changes/<id>/research/genesis.md`
- **AND** the transcript carries the user-visible Q/A turns and the final premise
- **AND** the genesis artefact's `nodes` field carries the change ID as a placeholder per `openspec/conventions.md` Section 9

#### Scenario: Session state never leaks outside the change directory

- **GIVEN** any state of an in-progress interview session
- **WHEN** the runner persists or reads session data
- **THEN** all reads and writes happen inside `openspec/changes/<change>/research/`
- **AND** no session state is written to the main `openspec/specs/` tree or to `cairn.blueprint`

### Requirement: Resolve project-declared templates for stub authoring

The brownfield generator SHALL read project-declared contract templates and apply matching templates when drafting stub contracts, falling back to the built-in stub when no template matches.

#### Scenario: Matching template guides stub authoring

- **GIVEN** a project config declaring a contract template whose match rule covers a generated candidate
- **WHEN** the brownfield generator drafts the stub contract for that candidate
- **THEN** the draft uses the template's required headers and optional sections
- **AND** summariser-supplied content fills the body sections per the documented precedence rule

#### Scenario: Non-matching candidates fall back to built-in stub

- **GIVEN** a project config declaring no template that matches a generated candidate
- **WHEN** the brownfield generator drafts the stub contract for that candidate
- **THEN** the draft uses the built-in minimum-viable stub
- **AND** authoring completes without error

#### Scenario: Ill-formed templates do not block authoring

- **GIVEN** a project config containing a template whose body fails to parse
- **WHEN** the brownfield generator runs init or refine
- **THEN** the runner logs a warning naming the offending template
- **AND** authoring continues using the built-in stub for affected candidates

### Requirement: Populate decision-attached obligations when the schema supports them

When decision artefacts in this phase carry an `obligations` field, the brownfield generator SHALL populate it for AI-suggested decisions and surface the populated field in the generated change directory for human triage.

#### Scenario: Obligations are populated when the field exists

- **GIVEN** decision artefacts in this phase declare an `obligations` field
- **WHEN** the brownfield generator emits an AI-suggested decision
- **THEN** the decision artefact carries the obligations identified by the summariser
- **AND** the generated change directory exposes the obligations alongside the decision body

#### Scenario: Obligations are reviewable before archive

- **GIVEN** an AI-suggested decision with populated obligations in a generated change
- **WHEN** the human reviews the change directory before archive
- **THEN** the obligations field is editable and removable
- **AND** archive applies only the human-reviewed obligations

#### Scenario: Obligations population is a no-op when the field is absent

- **GIVEN** decision artefacts in this phase do not declare an `obligations` field
- **WHEN** the brownfield generator emits an AI-suggested decision
- **THEN** the decision artefact uses the existing schema without an obligations field
- **AND** no obligations-related output is produced for that decision
