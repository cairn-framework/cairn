---
id: scanner-gate-artefact-links
nodes:
  - cairn.kernel.artefacts
  - cairn.kernel.scanner
status: proposed
date: 2026-06-26
informed_by: [dec.artefact-organization-and-provenance]
---

# Scanner gate: artefact link integrity

## Problem

docs/conventions.md section 10 ("Artefact organization and provenance links") documents three
provenance rules as author-side POLICY ONLY — the scanner is currently silent on violations:

1. **Duplicate id** — two artefacts sharing an id across the dec/res/src union both load; the
   duplicate is collapsed silently. No finding is raised.
2. **Typed-prefix non-conformance** — an artefact's `id` prefix (`dec.`/`res.`/`src.`) must
   match its type; a mismatch is read verbatim and causes silent mis-routing.
3. **Unwired artefact** — a file placed in a subfolder of a pointer directory (e.g.
   `meta/decisions/kernel/foo.md`) is silently ignored by the non-recursive loader; the author
   gets no feedback that their artefact was not loaded.

These gaps mean the convention cannot be mechanically enforced, undermining the two-chain model.

## Proposed change

Add three new scanner findings:

- `CAIRN_ARTEFACT_DUPLICATE_ID` (Error): raised when two loaded artefacts share an id within the
  combined dec/res/src id namespace.
- `CAIRN_ARTEFACT_ID_PREFIX_MISMATCH` (Warning): raised when an artefact's id does not start
  with the expected prefix for its type.
- `CAIRN_ARTEFACT_UNWIRED` (Warning): raised when a `.md` file exists under a pointer-directory
  subtree but is not loaded (non-recursive loader dropped it).

New error codes must be assigned from docs/registries/error-codes.md before implementation.

## Acceptance criteria

- `cairn scan --strict` exits 1 when any of the three conditions is present.
- All three findings appear in `cairn scan` output with the file path and a clear message.
- `cargo test` passes including new unit tests for each finding.
- docs/registries/error-codes.md updated with the new codes.
- The existing 0-finding baseline on this repo is preserved.
