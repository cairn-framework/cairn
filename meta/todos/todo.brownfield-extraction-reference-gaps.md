---
node: cairn.kernel.cli
status: open
created: 2026-08-10
---

# Fix the two gaps the external extraction run found in the shipped reference

Raised by `todo.brownfield-extraction-external-run`, whose evidence is
`res.brownfield-extraction-external-run` sections 1 and 7. Neither gap fires a
revisit trigger of `dec.brownfield-extraction-mechanism`; both are prose defects
in `references/task-brownfield-decision-extraction.md` that cost the run real
work. The fired trigger is a separate unit,
`todo.onboard-decisions-document-code-link`.

## Task

Both items are edits to the same reference, in the canonical tree and the
`.claude` mirror, kept byte-identical.

1. **Step 0 assumes a System block the brownfield entry point never writes.**
   The reference tells the author to check that "the System block declares both
   artefact directories". The blueprint `cairn init --from-code --apply` writes
   is a flat list of discovered Containers and Modules with no System block, so
   the external run had to restructure the file by hand before the flow could be
   followed as written. Say what a brownfield author should do when there is no
   System block. Note that artefact pointers are collected from any node at any
   depth (`src/artefacts/registry/io.rs`, `pointers`), so a System wrapper is a
   convention rather than a parser requirement; the reference should say which
   it is recommending and why.

2. **Section 3's read-each-candidate step is load-bearing and does not say why.**
   The index reports sibling documents in one flat list with no status and no
   supersession. On `rancher/turtles` at the recorded commit, ADR 0009 reversed
   accepted ADR 0005, ADRs 0008 and 0011 retired half of ADR 0003, and ADR 0011
   superseded ADR 0010, none of it visible in the wire. A draft written from a
   single entry was wrong and had to be withdrawn. Name that hazard in section 3
   with the run as the worked example, so the step reads as a guard rather than
   as general advice.

## Non-goals

No change to the command surface and no change to the evidence index wire: the
index reporting a document's ADR status would only repeat what the document
claims, and on this evidence those claims are wrong. The document-to-code
binding gap is `todo.onboard-decisions-document-code-link`.

## Acceptance

- Both items are fixed in
  `tools/agent-pack/content/skills/cairn-dev/references/task-brownfield-decision-extraction.md`
  and in the `.claude` mirror, byte-identical.
- The pack determinism and router tests still pass without a size re-pin that
  hides an unintended edit.
- `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cairn scan --strict` all pass.
