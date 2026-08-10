---
node: cairn.brownfield
status: open
created: 2026-08-10
---

# Close the document-to-code binding gap the external extraction run exposed

Raised by `todo.brownfield-extraction-external-run`, whose evidence is
`res.brownfield-extraction-external-run` section 5. That run fired one limb of
revisit_trigger 2 of `dec.brownfield-extraction-mechanism` ("cannot preserve a
real node binding"). The trigger is recorded, not absorbed: this todo is the
response, and the mechanism ruling stands until it lands. The two reference
prose defects the same run found are a separate unit,
`todo.brownfield-extraction-reference-gaps`.

## Task

Decide and implement how a document-derived decision reaches the code it
governs.

The binding rule resolves the path of the evidence, so every ADR under
`docs/adr/` binds to whichever node declares that directory, never to the
subsystem the decision constrains. Two observations from the run, both
machine-visible:

- Against the blueprint exactly as `cairn init --from-code --apply` derives it,
  all 19 ADR documents came back unbound, because the derivation declares only
  code directories. The reference's answer for that state ("Disambiguate
  blueprint ownership first, or leave it") leaves the operator to invent a node
  the derivation did not produce.
- After a node claimed `docs/adr` and the extracted draft landed, `cairn scan`
  reported `CAIRN_PROVENANCE_NO_DECISION` for all 11 code nodes and none for
  the documentation node. The extraction satisfied provenance for the docs node
  and left the code exactly as uncovered as before.

Pick one of two answers and say which in the artefact that carries it:

1. The index joins a `document` entry to candidate `code-target` entries, for
   example by reporting the code paths the document names, so the drafter has a
   machine-visible link rather than a manual one. This moves the wire and its
   `schema_version`.
2. The reference instructs the author to bind by hand from a `code-target`
   entry's `node` when the document governs code, and says how to choose among
   several. This leaves the wire alone and puts the burden in prose.

Leaving the wire silent and the reference vague is not an answer.

## Non-goals

No new command noun, and no LLM inference on this path
(`dec.brownfield-extraction-mechanism`, the verified constraint). The
System-block prerequisite and the read-each-candidate hazard belong to
`todo.brownfield-extraction-reference-gaps`.

## Acceptance

- One of the two answers above is implemented, and the choice is stated in a
  decision or in the reference prose rather than left implicit.
- If the wire changes, `schema_version` moves and the snapshot coverage moves
  with it.
- `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cairn scan --strict` all pass.
