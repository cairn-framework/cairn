# Agent pack canonical foundation specification

## ADDED Requirements

### Requirement: Canonical byte ownership

The pack manifest MUST declare a bundle version and exactly one canonical source for each logical entry id plus explicit mode. The canonical baseline MUST cover the five core skills, all three `cairn-dev` references, `/cairn-loop`, recovery, and landing assets.

#### Scenario: Maintainer verifies the mechanical migration

- **GIVEN** the canonical manifest and checked-in Claude outputs
- **WHEN** the renderer runs in check mode
- **THEN** all eleven emitted files match their canonical source bytes exactly and no emitted file contains an injected generated marker

### Requirement: Claude adapter remains pure data

Harness-specific structure MUST be represented only by adapter rows that reference canonical entry-mode pairs and declare repository-relative destinations. The manifest MUST NOT encode scheduling, state machines, workflow edges, lifecycle state, or prompt authority.

#### Scenario: Consumer resolves the current Claude baseline

- **GIVEN** the Claude adapter rows
- **WHEN** rows are resolved against canonical ownership
- **THEN** each row produces one deterministic destination from one canonical byte source without interpreting the entry mode

### Requirement: Producer uniqueness

The renderer MUST reject more than one producer for a normalised emitted destination. Within a harness, it MUST also reject more than one producer for a logical entry-mode pair. Different harnesses MAY adapt the same canonically owned entry-mode pair.

#### Scenario: Manifest contains a collision

- **GIVEN** two rows with a colliding destination or harness entry-mode key
- **WHEN** manifest validation runs
- **THEN** validation fails before writes and the diagnostic names both rows, the conflicting key, and the manifest correction

### Requirement: Writes remain contained

The renderer MUST reject lexical escapes before filesystem writes. It MUST expose reusable resolved-path and symlink-containment validation for later installer preflight. Resolved validation MUST reject an existing component whose canonical location leaves the repository root.

#### Scenario: Destination escapes lexically

- **GIVEN** an adapter destination containing an absolute, root, prefix, or parent component
- **WHEN** the render plan is validated
- **THEN** validation fails before any destination mutation and identifies the row, path, and project-relative correction

#### Scenario: Destination escapes through resolution

- **GIVEN** a destination or existing ancestor that resolves outside the repository root
- **WHEN** containment validation runs
- **THEN** validation fails before the write and identifies the escaping path and correction

### Requirement: Deterministic drift gate

The dev-only renderer MUST provide non-mutating check mode and explicit write mode. Equal canonical input and manifest data MUST produce the same ordered render plan and output bytes on every run. Check mode MUST fail on missing or changed emitted output and name the regeneration command.

#### Scenario: Checked-in output drifts

- **GIVEN** one rendered file differs from its canonical source
- **WHEN** check mode runs
- **THEN** it exits unsuccessfully, names the destination, and directs the maintainer to the explicit write mode

### Requirement: Rendered consumers remain buildable

Existing `include_str!` consumers MUST continue to compile bytes from checked-in rendered `.claude` paths. Generated-file metadata MUST be out of band and scoped to manifest-enumerated destinations in `.gitattributes`.

#### Scenario: Package consumers compile

- **GIVEN** a clean renderer check
- **WHEN** the Cairn targets and package checks compile
- **THEN** the bundled core skills are sourced from the rendered byte-identical files and no canonical tooling crate is shipped in the user CLI package
