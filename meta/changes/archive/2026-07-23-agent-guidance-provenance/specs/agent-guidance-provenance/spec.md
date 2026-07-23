# Agent guidance provenance specification

## ADDED Requirements

### Requirement: Pinned external source scope

The source artefact MUST pin commit
`226c8d35fb6ea3ed55467753dba6dea2b5fd5778` and MUST list exactly these six
concrete reviewed paths:

- `playbooks/improve-harness.md`
- `docs/whole-job/README.md`
- `docs/continuous-maintenance/README.md`
- `docs/just-in-time-context/README.md`
- `docs/authority/README.md`
- `sources/scripts/validate_manifest.py`

#### Scenario: Reader audits the source pin

- **GIVEN** `meta/sources/harness-engineering.md`
- **WHEN** a reader inspects the pinned commit and path list
- **THEN** the commit and six concrete paths are present and no additional
  reviewed-path claim is required for this unit

### Requirement: Five-mechanism research with limits

The research artefact MUST cite `src.harness-engineering` and MUST cover whole-job
accountability, just-in-time routing, continuous-loop contract, claim-boundary
proof, and manifest integrity. Each mechanism MUST state a precise limit. The
manifest section MUST not overclaim one owner for every destination or content
hashing for every evidence kind.

#### Scenario: Reader audits mechanism limits

- **GIVEN** `meta/research/harness-engineering.md`
- **WHEN** a reader inspects the five mechanism sections
- **THEN** each section names an evidence anchor and an explicit limit, and the
  synthesis keeps scheduling, repetition, runtime, and parallelism external

### Requirement: Proposed refining decision without authority move

The decision artefact MUST remain `status: proposed`, MUST formally refine
`dec.loop-command-harness-model`, MUST restate all eight accepted clauses, and
MUST treat owner acceptance as a sanction for a later router-playbooks
migration. It MUST NOT require supersession, MUST NOT mark the older decision
superseded, and MUST NOT move loop authority on acceptance alone.

#### Scenario: Acceptance does not cut over authority

- **GIVEN** `meta/decisions/unified-cairn-dev-entry.md` remains proposed or is
  later accepted without the migration unit landing
- **WHEN** a reader asks where loop authority lives
- **THEN** standalone `/cairn-loop` remains the sole normative one-iteration
  procedure under accepted `dec.loop-command-harness-model`

### Requirement: Provenance-only unit

This change MUST author only the Source, Research, proposed Decision, and
native change record for the unit. It MUST NOT implement router, pack, query,
adapter, or runtime changes.

#### Scenario: Repository surface check

- **GIVEN** the completed change
- **WHEN** a reader inspects production code and pack surfaces
- **THEN** no authority-moving implementation has landed in this unit
