---
id: dec.spec-authority-retirement
nodes:
  - cairn.root
status: accepted
date: 2026-07-25
informed_by:
  - res.harness-engineering
related:
  - dec.artefact-organization-and-provenance
  - dec.unified-cairn-dev-entry
  - dec.native-todos-first
---
# The spec is fallback narrative; the graph is the read surface

## Context

`docs/spec.md` runs to roughly a thousand lines and predates most of the graph. It was once the only
place a reader could learn what cairn is, so it accumulated four different kinds
of content: the conceptual model and its history, the blueprint grammar and
artefact schemas, rules the scanner now enforces, and open questions.

Three of those four now have owners that are queryable and, where the rule is
mechanical, enforced: `docs/registries/spec-rules.md` and
`docs/registries/error-codes.md` for rules, `cairn.blueprint` and
`meta/contracts/` for structure and subsystem design, `meta/decisions/` for
rationale and for questions raised against a node, `meta/todos/` and
`meta/changes/` for plans, and the agent pack's skills and just-in-time
references for procedure. `docs/registries/declared-items.md` tracks the status
of the spec's own numbered questions, which are older than the gap primitive.
The routing was understood but never written down, so new content still drifted
back into the spec, and the spec kept being read as if it were the current-state
authority.

The unresolved risk is the opposite failure: rewriting the spec wholesale would
destroy the narrative history and the two-chain explanation, which nothing else
carries and which readers genuinely need.

## Decision

Reads are graph-first. `docs/spec.md` is fallback narrative for humans and for
questions the graph cannot yet answer. Routine work never bulk-loads it; needing
it for a routine task is evidence that the graph, a contract, a registry, or a
reference is incomplete, and that gap is what gets fixed.

The write-side routing table is ratified and lives in `docs/conventions.md`
section 11, not in the spec: putting a new normative rule in the spec is exactly
the accumulation this decision stops. The spec gains no new workflow, plan, rule,
or normative subsystem design.

Sections migrate to pointers one at a time, and only after the owning primitive
is authoritative. Each collapsed section names a real, queryable owner. Narrative
history and the two-chain explanation are never collapsed.

Section 16 (open questions) is NOT collapsed. A question raised while building
against a node is a gap decision under `meta/decisions/`, and that is where new
ones go. The four still-open numbered questions predate the gap primitive and
belong to the design as a whole rather than to any node, so filing them against
one would attach a standing `CAIRN_GAP_UNRESOLVED` warning to a node that does
not own the question, and the warning would sit there for as long as the
question does. They stay in the spec, with their status tracked in
`docs/registries/declared-items.md`, until each is resolved or genuinely
anchored to an owning node. The third, the meta/ layout question, is already closed by
`dec.artefact-organization-and-provenance` and now reads as a pointer to it.

Section 7 (blueprint grammar) and section 8 (artefact schemas) stay in the spec.
They have no owner elsewhere: the grammar is implemented by the parser but not
described by it, and the artefact schemas are enforced field by field without a
single document that states them. Collapsing either requires a decision that
first names its canonical home. That decision is a prerequisite, not a follow-up
to be assumed.

Actuator wording in the spec says operator (human or agent) wherever the
operation is agent-capable or deterministic. It keeps saying human only where the
spec is deliberately reserving judgment to a person.

## Consequences

- `docs/conventions.md` section 11 is the routing authority. A future rule that
  belongs in a registry, a plan that belongs in a todo, or a procedure that
  belongs in a skill has a named home and no excuse to land in the spec.
- The spec shrinks by attrition rather than by rewrite. Each collapse is its own
  reviewable step with a named owner, so a wrong collapse is visible and
  revertible.
- Two sections stay put and are recorded as blocked on a canonical-home
  decision. That is a deliberate stop, not an omission: `todo.spec-authority-
  retirement` closes without them.
- The spec's own prose stops implying that a human must perform operations the
  CLI already performs, which is what made agents read it as a procedure manual.
