---
node: cairn.kernel.scanner
status: done
created: 2026-08-08
---

# Contract asserted numeral drift

Contract prose can retain registry counts or schema and cache version numerals
after code changes, with no deterministic gate identifying stale facts.

## Task

1. Define the source-of-truth mapping for contract counts and version facts.
2. Add a scanner finding for stale asserted numerals that have a deterministic
   source and preserve stable migration and protocol-version obligations.
3. Cover the finding with focused scanner tests and document its code.

## Acceptance

- `cairn scan --strict` reports deterministic asserted-numeral drift with a
  stable finding code and does not flag stable version semantics.
- Tests cover both stale and current source-backed assertions.
