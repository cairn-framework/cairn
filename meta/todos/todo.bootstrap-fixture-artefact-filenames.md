---
node: cairn.tests
status: open
created: 2026-07-27
---

# Bootstrap fixture sources carry the forbidden filename prefix

## Problem

`dec.artefact-filename-rule` settled that a decision, research, or source
filename is its `id` with the typed prefix stripped.
`tests/fixtures/cairn-bootstrap/meta/sources/` still holds nine files that carry
the prefix:

```
src.adr-tools.md              src.karpathy-llm-wiki.md
src.akash-llm-project-wiki.md src.openspec-deepwiki.md
src.dlthub-ontology-first.md  src.openspec-repo.md
src.dual-graph-codex-compact.md src.review-adversarial-1.md
src.structurizr-dsl.md
```

Two of them disagree with their own `id`, which is the stronger half of the rule
and the more interesting defect. Both are residue of the phase 2.6 terminology
rename:

- `src.dlthub-ontology-first.md` declares `id: src.dlthub-map-first`.
- `src.structurizr-dsl.md` declares `id: src.structurizr-blueprint`.

`todo.artefact-filename-test-fixtures` conformed the fixtures that build project
trees in Rust and left these, because they are a different thing: static sample
content, not a fixture that writes a path.

## Why it matters, and why it is not urgent

Nothing loads them today. The bootstrap blueprint declares `contract`,
`decisions`, and `research` pointers but no `sources` pointer, so these files
are never read into an `ArtefactSet` and emit no `CAIRN_ARTEFACT_FILENAME_DRIFT`
finding. Measured on 2026-07-27, `cairn --file
tests/fixtures/cairn-bootstrap/cairn.blueprint scan --strict` reports 22
findings and not one of them is CA038.

So this is a latent defect, not a live one. It bites the day someone adds a
`sources` pointer to that blueprint, or copies the tree as a starting point for
a new fixture, which is exactly what a bundled bootstrap corpus invites.

## The judgment call

`src.review-adversarial-1.md` declares `file: ./meta/sources/review-adversarial-1.md`,
which is the exact path its conforming filename would occupy. Renaming it makes
the source record point at itself. That needs a decision, not a rename:

- Point `file:` at the real conversation transcript if one can be located.
- Or accept the self-reference for an `unverified` `conversation` source whose
  body *is* the evidence, and say so in the file.
- Or give the artefact a different slug so the two paths stay distinct.

Check whether `CAIRN_SOURCE_*` validation has anything to say about a `file:`
that resolves to the artefact itself before choosing.

## Task

Rename the nine files to the stem derived from each `id` (not from the current
filename, which is wrong for two of them). Resolve the self-reference above.
Update any prose that names an old path.

## Acceptance

- All nine source ids still exist, with their bodies unchanged. This is a
  rename, so deleting the directory satisfies nothing: no test names these
  files, which means the gates below cannot tell removal from remediation.
- Every file under `tests/fixtures/cairn-bootstrap/meta/sources/` is named for
  its own `id` with the `src.` prefix stripped.
- `src.review-adversarial-1.md`'s successor has a `file:` value that does not
  resolve to itself, or a body line explaining why it does.
- `cargo test` passes, and `cairn --file
  tests/fixtures/cairn-bootstrap/cairn.blueprint scan` reports no new finding
  code compared with before the change.
