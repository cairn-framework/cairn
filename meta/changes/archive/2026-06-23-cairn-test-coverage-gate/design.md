# Design: Test-coverage integrity gate

Resolves the five open questions from spike `cairn-a8z`.

## 1. Detection mechanism: convention heuristic (a) now, contract `## Tests` (c) deferred

The two beads conflict on mechanism and the conflict is resolved here, not
silently. `cairn-87n` asks for `#[cfg(test)]` content detection; spike
`cairn-a8z` recommends a hybrid with the contract `## Tests` section (spec.md:313)
as the norm layer.

**(c) contract `## Tests` is the more cairn-idiomatic mechanism** — language-
agnostic, fits the two-chain authority model (contract = norm, reconciler
verifies reality), and quiet by default (only flags declared-but-unresolved test
paths). It is the right eventual norm. But it is **not dogfoodable today**:
cairn's own blueprint authors zero contract pointers (verified — no node carries
a `contract` pointer; the repo has deliberately not made the repo-wide
provenance/contract commitment noted in AGENTS.md). A contract-based gate would
emit zero findings on cairn's own repo and could not be validated through the
dev loop, whose premise is that the framework verifies its own development.
Adopting it now is either a no-op feature or a massive out-of-scope expansion
(authoring contracts for all nodes plus the provenance commitment).

**(a) `#[cfg(test)]` content scan is Rust-only but dogfoods immediately.** It
keys off data the reconciler already produces (`node.files`), needs no new
artefact surface, and surfaces real gaps in the code that exists. The default-
noise risk is low for cairn: nearly every module already has inline
`#[cfg(test)]`; a pre-check shows one testless entry point, covered at module
granularity by its sibling files. Per-node opt-out is the `no-test-coverage`
tag (the "skip integration-only modules" hatch from `cairn-87n`).

**Decision: ship (a) now; (c) is the documented upgrade path.** When the repo
(or any project) authors contracts with `## Tests` sections, the check should
verify declared test paths resolve against reality — the same contract->code
conformance pattern already used for interface hashes. That requires contracts
to exist first, a prerequisite absent today. Until then (a) is the honest cut:
it works on the code in front of us.

**(b) import-edge via LSP** is the most accurate option but depends on Phase 5/10
LSP plumbing that does not exist. Out of scope.

## 2. State trigger: ghost-exempt, synced-required

Modelled on the contract-pointer rule verbatim (`build.rs:194-198`):

```
severity = if node.state == Ghost { exempt (no finding) } else { Warning }
```

Only nodes that own files and have reconciled source are checked. Internal
nodes with no own files are skipped (they have no code to test). This is the
ghost -> synced parity the spike asked to confirm: a planned module (ghost)
owes no tests; the moment code lands (synced), the test surface is expected.

## 3. Severity and gate: loose default, existing `--strict` knob

Finding severity is `Warning`. The existing CLI `--strict` flag
(`cli/mod.rs:401-405`) already promotes any Warning to a non-zero exit for
`scan`/`lint`. That is the strict gate. No new config key, no new hook class.

- Default (`cairn scan`): finding surfaces, exit 0 (advisory).
- Strict (`cairn scan --strict`): finding surfaces, exit 1 (gate).
- Tension hook: Warning findings already flow into the tension hook
  (`hooks/mod.rs:180-195`), so `cairn hook all` surfaces them without blocking.

This matches the spec's loose-by-default precedent: todo coverage is
informational (spec.md:339), edge divergence is advisory (spec.md:635).

## 4. Finding type and error code

- **String code**: `CAIRN_TEST_COVERAGE_MISSING` (follows the `CAIRN_*` prefix
  used by `CAIRN_CONTRACT_MISSING`, `CAIRN_INTERFACE_HASH_CHANGED`).
- **Registry code**: `CH002` (CH = Hooks category; `CH001` is the
  architecture-decision gate).
- **Severity**: `Warning`.
- **Fields**: `node` = the offending node id; `path` = the first claimed source
  file (for traceability); message names the missing marker.

## 5. Boundary: existence only

Cairn asserts the test surface EXISTS (the `#[cfg(test)]` marker is present). It
never runs tests (spec.md:837), never evaluates whether they pass, and never
judges whether they are meaningful (spec.md:699, fence-not-proof). No new
artefact type: the finding reuses the existing `Finding` struct and the existing
tension/strict enforcement channels. The per-node opt-out is a blueprint tag,
`no-test-coverage`, not a new artefact.

## Placement

New private function `validate_test_coverage(&mut Graph, &Path)` in
`src/map/build.rs`, called from `build_graph` immediately after
`validate_contracts`. It needs `root` (to read files) and the graph (nodes with
state + files), both already in scope. Private functions do not affect the
public interface hash, so no contract or hash-state update is required.

## Dogfooding

Running the check on cairn itself will flag any genuinely-testless module. The
expected exempt set: `cairn.tests` (the `tests/` integration crate, which IS the
test surface and uses no `#[cfg(test)]`), and proc-macro crates if applicable.
These take the `no-test-coverage` tag. Any other flag is a real gap to close or
a real exemption to record.
