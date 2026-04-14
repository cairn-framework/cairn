---
id: dec.change-directories
nodes: [cairn.kernel.changes, cairn.kernel.reconciliation]
status: accepted
date: 2026-04-13
revisited: 2026-04-13
revisit_triggers:
  - "Change directories become cluttered in practice (too many active changes at once)"
  - "A workflow pattern emerges that change directories cannot express"
  - "OpenSpec's delta model evolves in ways that invalidate the semantics adopted here"
informed_by:
  - type: research
    id: res.related-work-survey
  - type: source
    id: src.openspec-repo
  - type: source
    id: src.openspec-deepwiki
---

# dec.change-directories: Isolate proposed modifications in change directories with delta operations

## Context

v0.4 left the "accepted but not yet realized" state ambiguous. A decision marked accepted implied the system should change, but the DSL and contracts might not yet reflect it. An agent reading the ontology could not cleanly answer "what does the system look like today?" because the answer depended on whether you read the DSL (current) or the ADR queue (intended).

Earlier drafts considered solving this with ADR status lifecycles (`accepted → realized`). That moved the problem into frontmatter without solving it.

OpenSpec demonstrates a cleaner answer: isolate proposed modifications in their own directory, untouched until explicitly merged.

## Decision

Proposed modifications live in `./meta/changes/<change-name>/`. The directory mirrors the main `./meta/` tree and contains only the artefacts the change touches. Each artefact declares its operation (added, modified, removed, renamed) in frontmatter. DSL changes use a `dsl.delta` file with section markers.

At archive time, the archiver applies deltas in the order RENAMED → REMOVED → MODIFIED → ADDED, then runs the scanner. If any structural error or interface contradiction results, the archive aborts with full rollback.

The DSL in the main tree is current-state truth. Decisions in the main tree are active. Anything in a change directory is proposed but not yet in effect.

## Consequences

- The ADR status lifecycle simplifies. There is no "accepted but unrealized" state because a decision that is not yet in effect lives in a change directory. Once merged, it is active.
- The scanner ignores change directories when building the ontology. Queries default to current truth; change-aware queries are opt-in.
- Cross-cutting changes that touch many artefacts are expressible as a single coherent proposal rather than scattered edits.
- Cairn adopts OpenSpec's delta semantics directly. If OpenSpec's model evolves, Cairn must decide whether to track it or diverge.
- Cairn and OpenSpec can coexist. Projects can use OpenSpec for change workflow and Cairn for structural reconciliation, sharing the change directory pattern.
