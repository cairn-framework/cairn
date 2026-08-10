---
node: cairn.kernel.cli
status: done
created: 2026-08-10
blocked_by: [todo.brownfield-onboard-decisions-index]
parent: todo.brownfield-extraction-flow
---

# Ship the cairn-dev brownfield decision-extraction authoring reference

Implementation unit split out of `todo.brownfield-extraction-flow` under the
sizing rule. This unit builds clauses 2 and 3 of
`dec.brownfield-extraction-mechanism`: the shipped harness authoring reference
and its pack distribution wiring. Blocked until
`todo.brownfield-onboard-decisions-index` lands, because the reference invokes
`cairn onboard decisions --json` and shipping guidance for a command that does
not exist is worse than shipping none.

## Task

Add the reference at the canonical path the ruling names:

```
tools/agent-pack/content/skills/cairn-dev/references/task-brownfield-decision-extraction.md
```

and its byte-identical checked-in mirror at
`.claude/skills/cairn-dev/references/task-brownfield-decision-extraction.md`.
The mirror is an input to `src/cli/commands/pack_assets.rs`: add the row to
`BASE_ASSETS` (this is ordinary cairn-dev guidance, not loop-mode procedure, so
not `LOOP_ASSETS`), where `all_assets` rewrites the adapter root for the `.omp`
destination. Add the canonical entry plus new Claude and OMP adapter rows to
`tools/agent-pack/manifest.toml`.

Both routers, `tools/agent-pack/content/skills/cairn-dev/SKILL.md` and
`.claude/skills/cairn-dev/SKILL.md`, add this exact route row, byte-identical in
both trees:

```
| Mine an existing codebase into proposed decisions | `references/task-brownfield-decision-extraction.md` |
```

Both shipped `references/command-reference.md` files (canonical and `.claude`)
describe `cairn onboard decisions --json` and the unchanged no-subcommand orphan
report.

The manifest additions invalidate the generated-file marker in `.gitattributes`,
the size-pinned `EXPECTED_CANONICAL` and `EXPECTED_CLAUDE` arrays in
`tools/agent-pack/tests/determinism_drift_tests.rs`, and the route reachability
checks in `tools/agent-pack/tests/router_route_tests.rs`.

The reference content invokes `cairn onboard decisions --json`, asks the harness
agent to interpret the returned code and document evidence, records the selected
evidence in a primary research artefact, and writes the decision body with:

```
cairn decision new <slug> --node <id> --informed-by <research-id>
```

`todo.brownfield-onboard-decisions-index` landed that wire on 2026-08-10, so the
reference describes what exists rather than a guess. Under `data` it carries
`schema_version`, `bound`, `unbound`, `bound_count`, and `unbound_count`. Every
entry carries `kind` (one of `document`, `readme-section`, `invariant-comment`,
`code-target`), `path`, `line` (null for whole-file evidence), and `detail`; a
`bound` entry adds `node`, and an `unbound` entry carries no `node` key at all.
The `node` on a bound entry is the `--node` argument to pass below. A
`code-target` entry's `detail` is the path-derived discovery candidate id, which
is evidence only: the reference must never pass it as a node id.

The onboard report validates the binding; `cairn decision new` does not
re-resolve graph ownership. The reference preserves the report's evidence paths
and resolved node ids in the draft, and leaves every extracted decision at
`status: proposed`. It may set `informed_by`, `revisit_triggers`, and
`ratification: local` or `ratification: binding` or no `ratification` field at
all, noting that an absent value defaults to `binding`
(`src/artefacts/registry/kinds.rs`) and that an explicit `local` claim is subject
to the full tier shape rules. It must not set `status: accepted`, `ratified_by`,
`receipts`, or `supersedes`, must not introduce a second decision writer, and
must not use `cairn gap`, because an extracted decision is not an unresolved
implementation question and must not raise `CAIRN_GAP_UNRESOLVED`.

The reference also carries the review handoff the ruling requires: a green scan
proves artefact integrity, not that a model selected the right prose, so the
agent retains the `cairn onboard decisions` report that produced each draft
alongside the proposed artefact, and puts the draft to the maintainer for
acceptance or rejection instead of accepting it.

## Non-goals

The deterministic command belongs to
`todo.brownfield-onboard-decisions-index`. The external-repository run belongs to
`todo.brownfield-extraction-external-validation`.

## Acceptance

- The reference exists at the canonical path, the `.claude` mirror is
  byte-identical, and the pack determinism drift test passes with the updated
  `EXPECTED_CANONICAL` and `EXPECTED_CLAUDE` arrays.
- Both `cairn-dev` routers carry the exact route row, byte-identical, and
  `tools/agent-pack/tests/router_route_tests.rs` proves the new route resolves to
  a shipped file.
- `tools/agent-pack/manifest.toml` carries the canonical entry plus Claude and
  OMP adapter rows, and `.gitattributes` marks the regenerated mirror.
- Both shipped `command-reference.md` files document `cairn onboard decisions
  --json` and the unchanged no-subcommand orphan report.
- The reference text states that drafts stay `status: proposed`, forbids
  `status: accepted`, `ratified_by`, `receipts`, `supersedes`, and `cairn gap`,
  and requires the evidence report to be retained with the draft and handed to
  the maintainer for acceptance or rejection.
- `cairn scan --strict` exits 0.
- On landing, set `todo.brownfield-extraction-external-validation` to `open`.
