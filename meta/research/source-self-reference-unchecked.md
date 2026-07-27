---
id: res.source-self-reference-unchecked
nodes:
  - cairn.kernel.artefacts
  - cairn.tests
date: 2026-07-27
method: primary
---

# A source may cite itself as its own evidence, and nothing detects it

Measured on 2026-07-27 while conforming the bootstrap fixture's source
filenames to `dec.artefact-filename-rule`.

## What was measured

`validate_sources` in `src/artefacts/registry/validate/mod.rs` branches on
`verification` and inspects `file:` in only two of three arms:

- `Verified` hashes the local file and compares it to `sha256`.
- `External` requires `file:` to parse as a URL (`CAIRN_SOURCE_EXTERNAL_URL`).
- `Unverified` emits `CAIRN_SOURCE_UNVERIFIED` (Info) and reads `file:` not at
  all.

So for an unverified source, `file:` may hold any string, including the
artefact's own path. `src.review-adversarial-1` in the bootstrap fixture held
exactly that: `file: ./meta/sources/review-adversarial-1.md`, which is the path
the artefact itself occupies once its filename follows the naming rule. A source
record whose evidence pointer resolves to the source record carries no evidence.

The fixture made this invisible twice over. The bootstrap blueprint declares
`contract`, `decisions`, and `research` pointers but no `sources` pointer, so
those nine files are never loaded into an `ArtefactSet`. Before the change,
`cairn --file tests/fixtures/cairn-bootstrap/cairn.blueprint scan --strict`
reported 22 findings, of which 7 `CAIRN_ARTEFACT_POINTER_MISSING`, 6
`CAIRN_CONTRACT_MISSING`, and 9 `CAIRN_RECONCILE_LANGUAGE_UNKNOWN`. After the
change the same command reports the same 22 in the same distribution. Neither
`CAIRN_ARTEFACT_FILENAME_DRIFT` nor any `CAIRN_SOURCE_*` code appears in either
run, because no reconciler ever reaches the directory.

## Corpus shape that constrains the fix

Of the nine sources in that directory, five are `unverified`, and three of
those already encode a missing pointer as `file: null` (`karpathy-llm-wiki`,
`dual-graph-codex-compact`, `dlthub-map-first`). So `null` is an available
encoding that introduces no new convention.

`src.review-adversarial-1` is cited by four decisions in the fixture, which
prices the option of changing its id to dodge the path collision.

`todo.bootstrap-fixture-artefact-filenames` named three candidate resolutions.
They are weighed, and one is chosen, in `dec.source-file-never-self`. This
artefact holds the measurements that decision rests on and does not restate the
argument.

## What this does not prove

The gap is general, not fixture-local: any repository can declare an unverified
source whose `file:` is its own path, and `cairn scan` will stay silent. This
research does not size that check or argue its severity. It is tracked as
`todo.source-self-reference-finding`.

Nor does it argue that unloadable artefact directories should be detected. That
is a wider question about pointer coverage, and the bootstrap fixture is
deliberately partial.

## Consequence in this repository

Because no reconciler covers the directory, conformance is held by
`tests/fixtures_smoke.rs` instead: `test_bootstrap_fixture_sources_are_named_for_their_ids`
pins the nine ids and checks each filename against its own id, and
`test_bootstrap_fixture_sources_do_not_cite_themselves` checks the `file:`
values. The id set is pinned rather than derived from the directory so that
deleting a source fails the test. A directory-derived check would have been
satisfied by an empty directory, which is the outcome the todo's acceptance
explicitly ruled out.
