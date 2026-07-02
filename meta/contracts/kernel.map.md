---
node: cairn.kernel.map
interface:
  - "pub fn build_graph( ast: &Ast, root: &Path, contracts: &ContractSet, claimed_files: &mut BTreeMap<String, Vec<String>>, external_findings: Vec<Finding>, ) -> Graph"
  - "pub fn cycle_findings(graph: &Graph) -> Vec<Finding>"
  - "pub fn topological_order(graph: &Graph) -> Result<Vec<String>, Vec<Finding>>"
---

# Contract: cairn.kernel.map

## Purpose

The map module owns the in-memory dependency graph: it builds a flattened
`Graph` from the parsed blueprint, contracts, and reconciled claims, runs the
structural integrity checks, and serves typed read queries over the result. It
is the shared data structure every downstream surface (scanner checks, query
API, hooks, changes) reads from.

## Public interface

- `build_graph(ast, root, contracts, claimed_files, external_findings)`: the
  builder. Inserts nodes, then runs `validate_edges`, `validate_ids`,
  `validate_path_ties`, `validate_contracts`, `validate_test_coverage`, and
  `validate_contract_coverage`.
- `Graph`, `NodeRecord`, `EdgeRef`, `NodeState`, `Finding`, `FindingSeverity`:
  re-exported graph types. `Finding` carries a stable `code`, severity, and
  span, and implements `Display` plus `Error`.
- `integrity::cycle_findings` and `integrity::topological_order`: DFS cycle
  detection (white/gray/black colouring) and a topological sort that returns
  cycle findings on a cyclic graph.
- `query`: typed read services, `get`, `neighbourhood`,
  `neighbourhood_with_options`, `files`, `depends`, `dependents`, `graph`,
  `order`, `lint`, and `islands`, returning typed response structs.
- `ISLANDS_SCHEMA_VERSION`: domain-layer schema version for `IslandsResponse`.

## Invariants

- `contract_coverage::validate_contract_coverage` emits
  `CAIRN_CONTRACT_LEAF_UNCOVERED` (registry code CK003) only for a synced leaf
  node that owns code and declares no contract pointer; containers, ghost
  nodes, fileless leaves, and nodes tagged `no-contract` are exempt. The
  finding is a Warning, advisory unless promoted by `cairn scan --strict`.
- `test_coverage::validate_test_coverage` emits `CAIRN_TEST_COVERAGE_MISSING`
  for synced Rust modules lacking a `#[cfg(test)]` marker, with the same ghost
  and tag exemptions.
- `topological_order` returns `Err(cycle findings)` when a cycle exists rather
  than panicking, so construction never blocks on a cyclic graph.
- Query functions return a `Finding` (or cycle findings) when a node cannot be
  resolved, never a partial result.

## Dependencies

Outgoing blueprint edges: `cairn.kernel.map -> cairn.kernel.blueprint`
(consumes the parsed `Ast`, `Node`, `Edge`, `NodeKind`, `Span`) and
`cairn.kernel.map -> cairn.kernel.artefacts` (validates contracts via
`ContractSet`). Inbound: the scanner, query, hooks, changes, ui, reconcile, and
brownfield modules all read the graph and its findings.

## Tests

Unit tests live in `#[cfg(test)] mod tests` blocks within each file:
`build.rs` (builder and edge/id/path validation), `integrity.rs` (cycles and
topological order), `contract_coverage.rs` (CK003 exemption cases),
`test_coverage.rs`, `graph.rs`, and `query.rs` (each typed query, islands, and
neighbourhood options).
