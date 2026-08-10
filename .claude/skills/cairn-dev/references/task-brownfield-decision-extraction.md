# Brownfield decision extraction

Goal: turn decisions a codebase already made implicitly into proposed Decision
artefacts the maintainer can accept or reject.

Cairn indexes an evidence set and binds each path to the node that owns it. It
performs no inference (`dec.brownfield-extraction-mechanism`).

## 0. Prerequisites

Use this only in an onboarded project whose `cairn.blueprint` loads without
structural errors: the command errors rather than binding a draft against a stub.
Check that both artefact directories are declared (`cairn init` seeds only
`todos`):

```
decisions "./meta/decisions"
research "./meta/research"
```

Without them Cairn neither validates the files nor lists the draft in `cairn
pending`. Cairn collects these pointers from any node at any depth
(`src/artefacts/registry/io.rs`, `pointers`), so a System block is a convention,
not a parser requirement. `cairn init --from-code --apply` writes no System block:
its output is a flat list of Containers and Modules. Wrap that list in a System and
declare both pointers there, keeping a project-wide claim off any single Container
or Module.

## 1. Index the evidence

```bash
cairn onboard decisions        # human-readable report
cairn onboard decisions --json # the stable wire
```

It reads a closed set and nothing else: files under `docs/adr/` and
`docs/decisions/`, README sections headed Decision, Rationale, or Invariant,
source comments carrying the literal `// invariant:` or `# invariant:` marker, and
the code targets brownfield discovery reports. It reads no other prose, drafts no
narrative, calls no model, and touches no blueprint.

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
already validated against the graph, and the `--node` argument in step 4.
An `unbound` entry carries no `node` key: either no eligible declared path claims
it, or equally specific declarations tie and the resolver refuses to pick.
Disambiguate ownership first, or leave it.

A `code-target` `detail` is a path-derived discovery candidate id, evidence only.
Never pass it as a node id: only the `node` field is a node.

## 3. Select what is really a decision

The index is bounded, not selective: most entries are context. Read every
candidate at its `path` and `line`, including ones you will not draft. The wire is
one flat list with no status and no supersession: on `rancher/turtles`, ADR 0009
reversed accepted ADR 0005, ADRs 0008 and 0011 retired half of ADR 0003, and
ADR 0011 superseded ADR 0010, and the wire showed none of those relationships. A
draft written from a single entry was wrong and was withdrawn once all 19 were
read (`res.brownfield-extraction-external-run`). This step is a guard, not
advice.

Keep only what records a choice with consequences: an option taken over a named
alternative, a constraint the code must keep, a trade-off accepted. Drop
restatements of what the code obviously does.

One decision per choice: evidence over several paths arguing one choice is one
decision citing all of them.

## 4. Record the evidence, then write the draft

Record the selected evidence as a Research artefact first, so the draft cites
evidence rather than asserting it. There is no research writer command:
hand-author `meta/research/<slug>.md`, quoting the evidence and keeping every
`path` and `line` the report gave. List the resolved node ids one per line, and
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

`method: primary` is load-bearing: research with neither a `meta/sources/` citation
nor `method: primary` raises `CAIRN_RESEARCH_MISSING_SOURCES`. The evidence is
first-hand, so that is honest. Never invent a source.

Then create the decision:

```bash
cairn decision new <slug> --node <id> --informed-by res.<slug>
```

`<id>` is the bound entry's `node` field, verbatim. The writer does not re-resolve
ownership, and a scan only checks the node exists, so a wrong but real id survives
every gate. It writes the typed frontmatter, `status: proposed`, and the standard
sections. Edit the generated body and the permitted provenance fields only. Never
hand-write a decision file and do not add a second writer. Keep the report's
evidence paths and node ids in the body, so the draft stays re-derivable.

You may set on the draft:

- `informed_by`, pointing at the research artefact above;
- `revisit_triggers`, as queryable reconsideration conditions;
- `ratification: local` or `ratification: binding`, or no field at all: absent
  defaults to `binding`, and an explicit `local` claim is subject to the full tier
  shape rules.

Leave every extracted decision at `status: proposed`.
Do not set `status: accepted`, `ratified_by`, `receipts`, or `supersedes`.

Do not use `cairn gap`. An extracted decision is a choice already made, not an open
question; `cairn gap` writes a `gap: true` proposal and stands a
`CAIRN_GAP_UNRESOLVED` warning until it is resolved and accepted, or deleted.

## 5. Hand the draft to the maintainer

`cairn scan` proves only that the artefact is well-formed, never that the evidence
or the draft is faithful. So keep the `cairn onboard decisions` report that
produced each draft with it, and put the draft to the maintainer for acceptance or
rejection. Never accept your own extracted decision.

## Verify

```bash
cairn scan
cairn pending   # every extracted draft, still proposed
```

Plus the repository's own gates.
