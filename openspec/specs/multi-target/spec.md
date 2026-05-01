# Multi-Target Capability Spec

## Purpose

The Multi-Target capability allows a single Cairn module to declare multiple independent implementation paths, each reconciled as a distinct target with its own language, contract role, and interface hash. It preserves backward compatibility for single-path modules while adding per-target state tracking. The capability also detects and reports interface divergence across a module's targets during lint and hooks, distinguishing unintended asymmetries from intentional ones marked by metadata.

## Requirements

### Requirement: Reconcile every path in a path list

Cairn SHALL treat each path in a node path list as an independent target.

#### Scenario: Two targets reconcile

- **GIVEN** a module declares `path ["./core-rust", "./core-ts"]`
- **WHEN** `cairn scan` runs
- **THEN** Cairn creates two target records for the module
- **AND** dispatches each target to the matching language reconciler
- **AND** records target-level files and state

#### Scenario: Single path remains compatible

- **GIVEN** a module declares `path "./core-rust"`
- **WHEN** `cairn scan` runs
- **THEN** Cairn creates one target record
- **AND** existing single-path query behavior remains valid

#### Scenario: Contract role defaults

- **GIVEN** a target has no explicit target metadata
- **WHEN** target records are created
- **THEN** the target uses contract role `public_api`

#### Scenario: Explicit contract role is configured

- **GIVEN** `cairn.config.yaml` declares target metadata with `node`, `path`, and `contract_role`
- **WHEN** target records are created
- **THEN** the matching target uses the configured contract role

#### Scenario: Explicit language overrides extension detection

- **GIVEN** `cairn.config.yaml` declares target metadata with `node`, `path`, `language: typescript`, and `contract_role`
- **WHEN** target records are created
- **THEN** the matching target uses the configured TypeScript language
- **AND** file-extension language detection is not used for that target

#### Scenario: Unsupported explicit language fails

- **GIVEN** `cairn.config.yaml` declares target metadata with `language: elixir`
- **WHEN** target records are created
- **THEN** Cairn reports a configuration error listing Rust, TypeScript, Python, and Go as supported languages

### Requirement: Store per-target interface hashes

Cairn SHALL persist interface hashes by node and target.

#### Scenario: Interface hash is canonical

- **GIVEN** two TypeScript targets with the same exported declarations in different source order
- **WHEN** interface hashes are computed
- **THEN** the hashes are equal
- **AND** private non-exported declarations do not contribute to the hash

#### Scenario: Target hashes are written

- **GIVEN** a multi-target module
- **WHEN** scan writes `.cairn/state/interface-hashes.json`
- **THEN** the state contains one hash entry per target path

#### Scenario: Old hash state migrates

- **GIVEN** a Phase 1 state file with one hash per node
- **WHEN** Phase 6 scan runs
- **THEN** Cairn migrates the state to target-level hash entries
- **AND** preserves the previous hash for the matching target

### Requirement: Report multi-target divergence

Cairn SHALL report divergence between targets of the same module.

#### Scenario: Undocumented divergence blocks

- **GIVEN** two targets claim the same contract role
- **AND** their interface shapes differ
- **WHEN** `cairn hook interface` runs
- **THEN** Cairn reports an interface contradiction
- **AND** exits with code `1`

#### Scenario: Intentional asymmetry is advisory

- **GIVEN** target divergence is marked as intentional by supported metadata
- **WHEN** `cairn lint` runs
- **THEN** Cairn reports a rationale tension
- **AND** does not report an interface contradiction for that divergence
