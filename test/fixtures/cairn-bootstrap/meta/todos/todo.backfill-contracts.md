---
node: cairn.kernel
status: open
created: 2026-05-08
---

# todo: backfill the remaining six kernel contracts

## Context

Issue #52 landed contracts for the four load-bearing modules: `cairn.kernel.parser`, `cairn.kernel.scanner`, `cairn.kernel.reconciler`, and `cairn.code-reconciler`. Six kernel modules still declare contract pointers that do not resolve on disk: `cairn.kernel.artefacts`, `cairn.kernel.changes`, `cairn.kernel.cli`, `cairn.kernel.hooks`, `cairn.kernel.query`, and `cairn.summariser`.

Each currently surfaces as a `CAIRN_CONTRACT_MISSING` warning under `cairn scan`. None block; together they are the difference between "fixture demonstrates a partial authority chain" and "fixture demonstrates a complete one."

## Done when

- Each of the six remaining contract pointers resolves to a contract file with valid frontmatter (`node:` matches the declarer; at least one `informed_by:` reference).
- `cairn scan` against the fixture reports zero `CAIRN_CONTRACT_MISSING` findings.
- Each new contract follows the shape established in #52: Interface, Invariants, Out-of-scope sections, grounded in actual `src/` code.

## Notes

- This todo is the canonical bootstrap-fixture todo for issue #53 (one of six artefact types). It is not a release blocker; it is a coverage gap that lowers the fixture's value as the canonical self-description.
- The Summariser contract (`cairn.summariser`) is the only optional module in the fixture; the contract body should preserve the `@optional` framing.
- Authoring six contracts in one commit is reasonable scope; alternatively one commit per contract preserves bisectability if review surfaces issues with individual modules.
