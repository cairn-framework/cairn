# Brownfield decision extraction

Goal: turn decisions an existing codebase already made implicitly into proposed
Decision artefacts the maintainer can accept or reject.

Cairn indexes a bounded evidence set and binds each path to the blueprint node
that owns it. It performs no inference (`dec.brownfield-extraction-mechanism`).
You select which evidence expresses a real decision and write the proposal.

## 0. Prerequisites

Use this only in an onboarded project whose `cairn.blueprint` loads without
structural errors: the command errors rather than binding a draft against a stub.
Before authoring, check that the System block declares both artefact directories
(`cairn init` seeds only `todos`):

```
decisions "./meta/decisions"
research "./meta/research"
```

Without these pointers Cairn neither validates the files nor lists the draft in
`cairn pending`.

## 1. Index the evidence

```bash
cairn onboard decisions            # human-readable report
cairn onboard decisions --json     # the stable index for this workflow
```

The index reads a closed evidence set and nothing else: files under `docs/adr/`
and `docs/decisions/`, README sections headed Decision, Rationale, or Invariant,
source comments carrying the literal `// invariant:` or `# invariant:` marker, and
the code targets brownfield discovery reports. It does not read arbitrary prose,
call a model, draft narrative, or touch the blueprint.

## 2. Read the wire

Under `data`: `schema_version`, `bound`, `unbound`, `bound_count`,
`unbound_count`. Every entry carries:

| Field | Meaning |
|---|---|
| `kind` | `document`, `readme-section`, `invariant-comment`, or `code-target` |
| `path` | project-relative, forward-slashed |
| `line` | one-based line, or `null` for whole-file evidence |
| `detail` | the document title, heading text, invariant text, or candidate id |

A `bound` entry adds `node`: the most-specific blueprint node declaring that path,
already validated against the loaded graph. That id is the `--node` argument in
step 4. An `unbound` entry carries no `node` key at all: either no eligible
declared path claims it, or equally specific declarations tie and the resolver
refuses to pick a winner. Disambiguate blueprint ownership first, or leave it.

A `code-target` entry's `detail` is the path-derived discovery candidate id, which
is evidence only. Never pass it as a node id: only the `node` field is a node.

## 3. Select what is really a decision

The index is bounded, not selective, and most entries are context. Read each
candidate at its `path` and `line`, and keep only what records a choice with
consequences: an option taken over a named alternative, a constraint the code must
keep, a trade-off someone accepted. Drop restatements of what the code obviously
does.

One decision per choice. Evidence spread over several paths arguing one choice is
one decision citing several paths.

## 4. Record the evidence, then write the draft

Record the selected evidence as a Research artefact first, so the draft cites
evidence rather than asserting it. There is no research writer command: hand-author
`meta/research/<slug>.md`, quoting the evidence and keeping every `path` and `line`
the report gave you. List the node ids the bound entries resolved one per line, and
carry no trailing comment in the frontmatter: a comment after an inline `[a, b]`
list makes that list parse as empty.

```yaml
---
id: res.<slug>
nodes:
  - <node-id>
method: primary
date: <YYYY-MM-DD>
---
```

`method: primary` is load-bearing: research with no `meta/sources/` citation and no
`method: primary` raises the Error finding `CAIRN_RESEARCH_MISSING_SOURCES`. The
evidence is first-hand, so that is the honest claim. Never invent a source artefact
to satisfy the check.

Then create the decision with the existing writer:

```bash
cairn decision new <slug> --node <id> --informed-by res.<slug>
```

`<id>` is the `node` field from the bound entry, verbatim. The writer does not
re-resolve ownership, and a later scan only checks that the node exists, so a wrong
but real node id survives every gate. The command writes the typed frontmatter,
`status: proposed`, and the standard sections. Edit the generated body and the
permitted provenance fields only. Do not write a decision file by hand and do not
add a second writer. Keep the report's evidence paths and node ids in the body, so
a reader can re-derive the draft from the same evidence.

You may set, on the generated draft:

- `informed_by`, pointing at the research artefact above;
- `revisit_triggers`, as queryable conditions for reconsideration;
- `ratification: local` or `ratification: binding`, or no `ratification` field at
  all. An absent value defaults to `binding`, and an explicit `local` claim is
  subject to the full tier shape rules, not a shortcut around them.

Leave every extracted decision at `status: proposed`.
Do not set `status: accepted`, `ratified_by`, `receipts`, or `supersedes`.

Do not use `cairn gap`. An extracted decision is a choice already made, not an
unresolved implementation question; `cairn gap` would write a separate `gap: true`
proposal and stand a `CAIRN_GAP_UNRESOLVED` warning until that question is
resolved and accepted, or the file is deleted.

## 5. Hand the draft to the maintainer

`cairn scan` proves only that the artefact is well-formed, never that the evidence
or the draft is faithful. So keep the `cairn onboard decisions` report that
produced each draft with it, and put the draft to the maintainer for acceptance or
rejection. Never accept your own extracted decision.

## Verify

```bash
cairn scan
cairn pending          # every extracted draft, still proposed
```

Plus the repository's own gates.
