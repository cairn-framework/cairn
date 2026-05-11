# Artefact Frontmatter Schemas

Complete YAML frontmatter schemas for all cairn artefact types. Each artefact is a markdown file with YAML frontmatter delimited by `---`.

## Decision

Decisions are the hinge of the two-chain model. They carry obligations from the provenance chain (evidence) into the authority chain (rules).

```yaml
---
id: dec.<short-name>           # required, unique across the project
nodes: [node.id.one, node.id.two]  # required, blueprint node IDs this decision covers
status: accepted               # required: proposed | accepted | deprecated | superseded
date: 2026-05-11               # required, ISO date
revisited: 2026-06-01           # optional, date of last review
revisit_triggers:               # optional, conditions that should trigger re-review
  - "If the API surface exceeds 20 endpoints"
  - "If a second reconciler language is added"
informed_by: [res.analysis]     # optional, research IDs that informed this decision
supersedes: [dec.old-approach]  # optional, decision IDs this replaces
refines: [dec.broad-strategy]   # optional, decision IDs this narrows
related: [dec.sibling]          # optional, related but not superseding decisions
---

Free-form markdown body explaining:
- What was decided
- Why (the rationale)
- What alternatives were considered
- What constraints or tradeoffs apply
```

**File location:** Place in the node's declared `decisions` directory, or `meta/decisions/` by convention.

**Status transitions:**
- `proposed` -> `accepted`: Decision ratified
- `accepted` -> `deprecated`: Decision no longer applies
- `accepted` -> `superseded`: Replaced by a newer decision (set `supersedes` on the new one)

## Todo

```yaml
---
node: node.id                   # required, the blueprint node this todo is for
status: open                    # required: open | in_progress | done | blocked
created: 2026-05-11             # required, ISO date
satisfies: change-id            # optional, links to an openspec change
---

Description of what needs to be done.
```

**File location:** Place in the node's declared `todos` directory, or `meta/todos/` by convention.

## Research

Research items are evidence in the provenance chain. They collect findings that inform decisions.

```yaml
---
id: res.<short-name>            # required, unique
nodes: [node.id]                # required, nodes this research is about
sources: [src.paper, src.repo]  # required, source IDs this research draws from
date: 2026-05-11                # required, ISO date
---

Analysis, findings, and conclusions from the research.
```

**File location:** Place in the node's declared `research` directory, or `meta/research/` by convention.

## Source

Sources are the raw evidence at the base of the provenance chain. They point to external documents, codebases, papers, or conversations.

```yaml
---
id: src.<short-name>            # required, unique
file: <path-or-url>             # required, location of the source material
verification: verified          # required: verified | external | unverified
type: <free-text>               # required, describes the kind of source (paper, repo, conversation, spec)
date: 2026-05-11                # required, ISO date
---

Optional notes about the source.
```

**Verification levels:**
- `verified`: Source has been reviewed and confirmed accurate
- `external`: Source is from an external authority (e.g., a published spec)
- `unverified`: Source has not been independently verified

## Review

Reviews are attestations about a node's quality or compliance.

```yaml
---
node: node.id                   # required, the node being reviewed
review_type: human              # required: human | agent_introspective | agent_cross_model
date: 2026-05-11                # required, ISO date
reviewer: <name-or-id>          # required, who performed the review
---

Review findings and assessment.
```

**Review types:**
- `human`: Reviewed by a person
- `agent_introspective`: AI agent reviewed its own output
- `agent_cross_model`: Different AI model reviewed the output

## Contract

Contracts are the spine of the authority chain. They declare what a module's interface must look like.

```yaml
---
node: node.id                   # required, the node this contract governs
---

Free-form body describing the module's interface contract, invariants, and obligations.
```

**File location:** Referenced by the node's `contract` field in the blueprint. Typically at `meta/contracts/<module-name>.md`.

## The provenance chain

Artefacts connect in a chain: **Source -> Research -> Decision -> (Blueprint + Contract + Code)**

- Sources provide raw evidence
- Research analyzes sources and draws conclusions
- Decisions cite research (via `informed_by`) and commit to an architectural choice
- The decision's `nodes` field connects it to blueprint nodes
- Contracts on those nodes enforce the decision mechanically

When authoring, build the chain bottom-up: ensure sources exist before citing them in research, and research exists before citing it in decisions.
