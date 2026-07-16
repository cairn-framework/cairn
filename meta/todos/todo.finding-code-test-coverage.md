---
node: cairn.tests
status: done
created: 2026-07-16
---

# Finding Code Test Coverage

Of 103 distinct CAIRN_* codes emitted in `src/`, about 20 (IO and
plumbing tiers, for example CAIRN_ARTEFACT_READ_FAILED and the
CAIRN_RECONCILE_PARSE_* family) have no test that triggers them, and
nothing fails when a new code ships untested; they drifted in silently.

Add `tests/finding_code_coverage.rs`: a meta-test that scans non-test
`src/` for emitted CAIRN_* string literals, scans `tests/` and embedded
cfg(test) modules for asserted codes, and fails on any emitted code that
is neither asserted nor listed in an explicit documented allowlist. Seed
the allowlist with the currently uncovered codes and burn it down in
follow-ups.

Motivation: `res.a2ui-analysis` finding 4. A declarative conformance
corpus (a2ui's shape) was refuted as redundant; this meta-test is the
surviving kernel.

Overlap: `todo.error-codes-registry-completeness` tracks the sibling
guard (emitted code with no registry entry in
`docs/registries/error-codes.md`). Both need the same scan of emitted
codes; land together or share a helper. No change proposal needed.

## Resolution (2026-07-16)

Added `tests/finding_code_coverage.rs`, sharing one AST-based scan (via
`tree-sitter-rust`) with `todo.error-codes-registry-completeness`'s
registry test: `scan_codebase()` builds the emitted/asserted sets both
tests use.

The real audit found 99 distinct emitted `CAIRN_*` codes (not 103; the
codebase has moved on since this todo was filed) and 35 untested, not
~20: the estimate here was a rough read, not a full scan. The 35-entry
burn-down list lives in `UNCOVERED_ALLOWLIST` in the test file, with a
reason grouped per subsystem in the constant's doc comment. `cargo test
--test finding_code_coverage` also fails on any allowlist entry that is
stale (no longer emitted, or now covered by a test), so the list can't
silently rot upward either.
