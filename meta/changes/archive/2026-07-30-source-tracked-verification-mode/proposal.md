# Proposal: source-tracked-verification-mode

## Motivation

`dec.source-tracked-verification` (accepted 2026-07-29, PR #528 sheet W3) ruled
a fourth source verification mode, `tracked`: a live in-repo path the project
reads as it stands, with resolution and containment checked and no hash. The
mode exists in the decision but not in the code or the docs, and the two source
records the decision names still carry `unverified` plus a perpetual Info
advisory. This change is the implementation unit,
`todo.source-tracked-verification-mode`.

## Outcome

A source artefact can declare `verification: tracked` and be validated by the
scanner: its `file:` must resolve inside the repository root (metadata probe,
no byte read, directories allowed), a declared `sha256` is rejected under the
new code `CAIRN_SOURCE_SHA256_UNEXPECTED` (CA040), and a resolving tracked
source produces no finding. Both `src.query-api-draft-generation` and
`src.summariser-accept-path` read `verification: tracked`, and `cairn lint` on
this repository reports no `CAIRN_SOURCE_UNVERIFIED` finding.

## Acceptance boundary

The scanner and query API surface: `cairn lint --json` on this repository, the
unit tests over `validate_sources` and `parse_source_verification`, the wire
string from `source_verification`, and the `fix_sources` remediation action.

## Evidence

- `cargo test` passes with new cases covering the todo's Acceptance table:
  tracked parse, resolving file and directory paths producing no finding,
  `CAIRN_SOURCE_READ_FAILED` for missing path, absolute path, `..` traversal,
  bare `./`, and an escaping symlink, `CAIRN_SOURCE_SHA256_UNEXPECTED` with the
  `fix_sources` action, unknown-value invalid finding unchanged, and the wire
  value `"tracked"`.
- `./target/debug/cairn lint --json` reports zero `CAIRN_SOURCE_UNVERIFIED`
  findings and `./target/debug/cairn scan --strict` exits 0.

## Out of scope

- `todo.source-self-reference-finding` implementation. Its check has not
  landed, so this change only amends its Acceptance wording from three
  verification values to four, per the todo body.
- Re-anchoring the spec-rules rows that are already mis-anchored independently
  of this change (for example `spec:476`, `spec:474`). Only rows this change's
  spec edit moves are re-checked.
- Any git-index awareness. `tracked` deliberately does not claim git tracks the
  path (decision Rationale).
