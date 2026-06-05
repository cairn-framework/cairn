# Semantic Reconciliation Capability Spec

## ADDED Requirements

### Requirement: Emit Info-severity findings for advisory states

The reconciler SHALL emit `Finding` values with `severity = FindingSeverity::Info` for advisory states that the producer can observe but that today render as silence. The two seed sites are orphaned-file states (a source file exists under the project tree but no node owns its path) and unverified-contract states (a node declares a contract pointer but the contract has not been reconciled against an interface hash). The `Info` variant SHALL NOT be inferred at render time. The producer is the source of truth for severity classification.

#### Scenario: Info variant is defined on the kernel enum

- **GIVEN** Phase 7.7 has archived
- **WHEN** a reader inspects `src/map/graph.rs`
- **THEN** `FindingSeverity` exposes the variants `Error`, `Warning`, and `Info`
- **AND** every existing `match FindingSeverity` site handles the `Info` arm exhaustively without `_` catch-alls

#### Scenario: Orphaned-file state emits an Info finding

- **GIVEN** a project tree that contains a source file under a path no node owns
- **WHEN** `cairn lint` runs
- **THEN** the reconciler emits a `Finding` with `severity = FindingSeverity::Info`
- **AND** the finding's `code` field identifies the orphaned-file producer per the error-codes registry

#### Scenario: Unverified-contract state emits an Info finding

- **GIVEN** a node that declares a contract pointer for which no interface hash has been reconciled
- **WHEN** `cairn lint` runs
- **THEN** the reconciler emits a `Finding` with `severity = FindingSeverity::Info`
- **AND** the finding's `code` field identifies the unverified-contract producer per the error-codes registry

#### Scenario: Info findings do not block hooks or gates

- **GIVEN** a finding stream that contains one or more `Info`-severity findings and no `Error`-severity findings
- **WHEN** the structural hook or `cflx accept` runs
- **THEN** the hook and gate pass
- **AND** the `Info` findings remain visible in the panel and the prose-nudge banner

#### Scenario: Info findings round-trip through serde_json

- **GIVEN** a `Finding` with `severity = FindingSeverity::Info`
- **WHEN** the value is serialised to JSON via `/api/lint` and deserialised back
- **THEN** the round-trip preserves the variant exactly
- **AND** the JSON payload uses the same string representation pattern as `Error` and `Warning`
