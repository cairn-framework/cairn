# Hooks Capability Spec

## ADDED Requirements

### Requirement: Run structural, interface, and tension hooks

Cairn SHALL expose hook commands matching the enforcement classes in `docs/spec.md` section 11.

#### Scenario: Structural hook blocks errors

- **GIVEN** a project with structural errors
- **WHEN** the user runs `cairn hook structural`
- **THEN** the command prints the structural findings
- **AND** exits with code `1`

#### Scenario: Tension hook never blocks

- **GIVEN** a project with rationale tensions and no structural or interface failures
- **WHEN** the user runs `cairn hook tension`
- **THEN** the command prints the tensions
- **AND** exits with code `0`

#### Scenario: Combined hook preserves blocking semantics

- **GIVEN** structural findings, interface findings, and rationale tensions
- **WHEN** the user runs `cairn hook all`
- **THEN** structural and interface failures determine the exit code
- **AND** rationale tensions appear in the report without independently failing the hook

### Requirement: Detect active change conflicts before archive

Cairn SHALL detect conflicts between active change directories before archive time.

#### Scenario: Two changes modify the same node

- **GIVEN** two active changes that both modify `saas.api.auth`
- **WHEN** the structural hook runs
- **THEN** it reports an active-change conflict
- **AND** exits with code `1`

#### Scenario: Artefact operation collision is detected

- **GIVEN** two active changes that both modify `meta/contracts/api/auth.md`
- **WHEN** the structural hook runs
- **THEN** it reports the artefact path collision
- **AND** identifies both change IDs

#### Scenario: Archive cannot bypass active change conflicts

- **GIVEN** two active changes that conflict with each other
- **WHEN** the user runs `cairn archive <change>` directly
- **THEN** Cairn runs the active-change conflict detector before mutating files
- **AND** exits with code `1`
- **AND** leaves all active change directories and current-truth files unchanged
