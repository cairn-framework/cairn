# Change System Capability Spec

## ADDED Requirements

### Requirement: Carry a suggested-edges queue inside the change directory

Cairn SHALL recognise a new file class `suggested-edges.json` located at `meta/changes/<change-id>/suggested-edges.json` (or the live-tree equivalent at `openspec/changes/<change-id>/suggested-edges.json`). The file is a sibling of `proposal.md`, `blueprint.delta`, `design.md`, and `tasks.md` and carries a queue of AI-suggested blueprint edges awaiting human triage. The file SHALL NOT be a fifth delta operation; the existing four-operation vocabulary (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`) SHALL remain the only delta operations.

#### Scenario: Queue file is state-versioned

- **GIVEN** a change directory containing a `suggested-edges.json` file
- **WHEN** a reader inspects the file
- **THEN** the first field is `version` (integer)
- **AND** the value of `version` is `1` for queues produced by this phase
- **AND** the second field is `entries` (array)

#### Scenario: Each entry carries source, target, relation, and triage state

- **GIVEN** any entry in the queue
- **WHEN** a reader parses the entry
- **THEN** the entry contains `source` (node ID), `target` (node ID), `relation` (string), and `triage_state` (enum)
- **AND** the entry MAY contain `confidence` (number), `provenance` (object referencing a trace sidecar), and `triage_note` (string)

#### Scenario: Triage state defaults to pending for newly-emitted entries

- **GIVEN** an entry just written by a producer
- **WHEN** a reader inspects the entry's `triage_state`
- **THEN** the value is `pending` unless a human has explicitly transitioned it to `accepted`, `rejected`, or `deferred`

#### Scenario: Queue is a sibling, not a delta operation

- **GIVEN** a change directory containing both `blueprint.delta` and `suggested-edges.json`
- **WHEN** the change archives
- **THEN** the four-operation merge order (`RENAMED`, `REMOVED`, `MODIFIED`, `ADDED`) applies to `blueprint.delta` only
- **AND** the suggested-edges queue is NOT consumed as a fifth operation
- **AND** the suggested-edges queue is preserved into the archived change directory for audit

### Requirement: Block archive on untriaged suggested edges via validate-strict

The `cflx openspec validate <change>` command SHALL read any `suggested-edges.json` file in the change directory and report the count of entries whose `triage_state` is `pending`. Without `--strict`, the count SHALL surface as a warning. With `--strict`, a non-zero pending count SHALL cause the command to fail with error code `CC002` and a non-zero exit, naming the count and the file path. This block is the structural guarantee that AI suggestions cannot land without explicit human assent.

#### Scenario: Validate without --strict surfaces count as warning

- **GIVEN** a change directory whose queue contains two entries with `triage_state: pending`
- **WHEN** the user runs `cflx openspec validate <change>` without `--strict`
- **THEN** the output names the count of pending entries
- **AND** the command exits with code `0` if no other validation problem exists

#### Scenario: Validate --strict fails with CC002 on pending entries

- **GIVEN** the same change directory
- **WHEN** the user runs `cflx openspec validate <change> --strict`
- **THEN** the command fails with error code `CC002`
- **AND** the output names the count of pending entries and the file path
- **AND** the command exits with a non-zero status

#### Scenario: Validate --strict passes when all entries are non-pending

- **GIVEN** a change directory whose queue contains only entries with `triage_state` of `accepted`, `rejected`, or `deferred`
- **WHEN** the user runs `cflx openspec validate <change> --strict`
- **THEN** the suggested-edges check does NOT fail the command
- **AND** the command exits with code `0` if no other validation problem exists

#### Scenario: Absent queue file is not an error

- **GIVEN** a change directory with no `suggested-edges.json` file
- **WHEN** the user runs `cflx openspec validate <change> --strict`
- **THEN** the suggested-edges check is skipped
- **AND** the command's exit status reflects only the other validation checks
