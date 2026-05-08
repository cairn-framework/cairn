# Bootstrap fixture: finding-class demonstrators

This directory holds intentional triggers for the two finding classes CAIRN distinguishes:

- **Rationale tension**: advisory, non-blocking, surfaces unresolved provenance links. Driven by `meta/_demo/todos/todo.tension-demo.md` (a todo whose `node:` references a non-existent ID, producing `CAIRN_TODO_ORPHAN_NODE` at Warning severity).
- **Blocking finding** (Error severity): driven by `meta/_demo/reviews/rev.contradiction-demo.md` (a review whose `node:` references a non-existent ID, producing `CAIRN_REVIEW_UNKNOWN_NODE` at Error severity).

Neither demonstrator is a real artefact. They exist purely so the canonical self-description proves the kernel routes the two severity channels distinctly.

## How to activate

The default `cairn.blueprint` excludes `_demo/` so the regular scan path stays clean. Activate the demonstrators with the demo variant blueprint:

```sh
cd test/fixtures/cairn-bootstrap
cairn --file cairn-with-demo.blueprint scan
cairn --file cairn-with-demo.blueprint hook all
```

The first surfaces both findings in the report; the second exits code 1 and prints the hook's blocking-vs-advisory classification.

## Caveats

The "Error-severity" demonstrator above triggers `CAIRN_REVIEW_UNKNOWN_NODE`, which the hook command classifies as a structural error (it appears in `structural_findings`, see `src/hooks/mod.rs`). A *strict* interface contradiction in the kernel's vocabulary is `CAIRN_INTERFACE_HASH_CHANGED`, emitted by `interface_findings` when `.cairn/state/interface-hashes.json` disagrees with the freshly-computed hash. That code path is dynamic (state-file driven) rather than static (contract-body driven), and the contract loader does not currently parse interface signatures from contract bodies (`src/artefacts/contract.rs` parses YAML frontmatter only).

The fixture exercises the **advisory-vs-blocking severity distinction** rather than the **structural-vs-interface code-path distinction**. The follow-up to introduce contract-body interface parsing (so that a static demo contract can trigger a true interface contradiction) is filed as a separate issue.

## Acceptance for issue #54

- `cairn --file cairn-with-demo.blueprint hook all` produces:
  - At least one Warning-severity finding (`CAIRN_TODO_ORPHAN_NODE`, advisory, never blocks)
  - At least one Error-severity finding (`CAIRN_REVIEW_UNKNOWN_NODE`, blocking, exit code 1)
- `cairn scan` (default `cairn.blueprint`) does not load `_demo/` and the regular scan path stays unchanged.
