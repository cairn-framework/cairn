---
node: cairn.kernel.artefacts
status: blocked
created: 2026-07-27
---

# A source citing its own path should be a finding

## Problem

`validate_sources` in `src/artefacts/registry/validate/mod.rs` inspects `file:`
only for `Verified` (sha256 against the local file) and `External` (must parse
as a URL). The `Unverified` arm emits `CAIRN_SOURCE_UNVERIFIED` at Info and
never looks at `file:` at all.

So an unverified source may declare `file:` as its own artefact path. That
records no evidence: the pointer resolves to the record. It was found in
`tests/fixtures/cairn-bootstrap/meta/sources/`, where the id-derived filename
rule made the collision reachable by rename rather than by typo
(`res.source-self-reference-unchecked`).

`dec.source-file-never-self` states the rule. This todo is the enforcement, and
it is blocked until that decision is accepted.

## Scope

Add a finding for a source whose `file:`, normalised against the repository
root, resolves to `source.path`. Applies to every `verification` value: a
`Verified` source hashing itself is the same defect, and `External` already
errors on a non-URL, so the new code only has to cover what `CAIRN_SOURCE_*`
leaves open.

Register the code in `docs/registries/error-codes.md`, its user-facing text in
`docs/design-system/copy.toml` under `[findings.codes]`, and route it in
`src/query_api/handlers/remediate.rs` alongside the other `CAIRN_SOURCE_*`
codes.

Severity is the open question. Warning makes `cairn scan --strict` fail on it,
which matches how `CAIRN_ARTEFACT_FILENAME_DRIFT` was argued
(`dec.artefact-filename-rule`). Info matches `CAIRN_SOURCE_UNVERIFIED`, which is
the neighbouring check. Decide it in the change, do not default to it.

## Depends on

- `dec.source-file-never-self` reaching `status: accepted`. It is `proposed`: a
  loop iteration may not ratify a repository-wide artefact rule. Building the
  check before the rule is ratified would gate every adopting repository on an
  unaccepted decision.

## Acceptance

- A source whose `file:` resolves to its own path produces the new finding, with
  a unit test in `src/artefacts/registry/validate/tests.rs` covering all three
  `verification` values.
- A source whose `file:` is `null`, a URL, or a different local path produces
  no new finding.
- The code appears in `docs/registries/error-codes.md` and `copy.toml`, and
  `tests/` coverage for finding-code registration still passes.
- `cairn scan` on this repository reports no new finding.
- Once the check ships, `test_bootstrap_fixture_sources_do_not_cite_themselves`
  in `tests/fixtures_smoke.rs` is redundant only if the bootstrap blueprint
  gains a `sources` pointer. It does not have one, so keep the test and say why
  in the change.
