---
id: dec.native-task-state-and-agent-guidance
nodes:
  - cairn.root
status: accepted
date: 2026-06-25
informed_by: [res.native-task-state-gap]
related: [dec.beads-task-layer, dec.bd-upgrade-plan, dec.lsp-diagnostics-server]
revisit_triggers:
  - "A maintainer ratifies the storage-backend decision (filesystem default, bd optional) that is currently memory-only; this decision should then `related` it and the AGENTS.md reconciliation can cite it by id"
  - "A code-map extension is proposed: index symbol definitions (enum variants, struct fields, function signatures, call sites) so cairn can answer source-structure questions like 'what variants does NodeState have?' or 'how is node state derived?' in one call instead of manual source reads. The problem it solves: agents investigating repo state currently grep source files for structural facts that a symbol index would return deterministically, duplicating effort and inviting drift between the graph and the code. Research direction: extend the scanner to emit a per-symbol index (definition, kind, file, span, variants/fields) alongside the existing blueprint + artefact graph, and expose it via `cairn` query surfaces so `cairn rationale` and a new `cairn symbols <query>` can return structural facts the way they already return provenance facts. This is deferred because it is a larger capability bet and rulings 1 to 3 close the immediate gap without it."
  - "bd setup regenerates the AGENTS.md beads blocks and overwrites a reconciled guidance edit; the fix becomes pin-or-regenerate"
---

# Native task state, visual unbuilt nodes, and cairn-first agent guidance

## Context

`res.native-task-state-gap` found three converging problems. The runtime
`NodeState` enum (`src/map/graph.rs:9`) has only `Synced`, `Ghost`, and
`Orphaned`; a blueprint node whose declared path is not yet on disk is marked
`Ghost`, conflating "not yet built (intentional plan)" with "code disappeared
(drift)". The webui cannot render unbuilt plan nodes differently from drift, so
a maintainer cannot see which nodes are part of the plan but not yet
implemented. Separately, `AGENTS.md` mandates `bd` for all task tracking while
three accepted decisions (`dec.bead-github-sync`, `dec.beads-task-layer`,
`dec.bd-upgrade-plan`) say cairn has a native Todo artefact type and beads are an
optional, read-only derived view. And the investigation that surfaced all of this
was slower than it should be because agents grep files for state questions that
`cairn rationale` / `context` / `scan` answer in one call.

## Decision

Four rulings. This decision rules the direction only; implementation lands via a
cairn change (`meta/changes/native-task-state/`), not here.

1. **Add a `Planned` node state; split it from `Ghost`.** The spec
   (`docs/spec.md` §10.1) already defines `ghost` as "declared in blueprint,
   path does not yet exist (planned but unimplemented)", which is a healthy
   forward-looking state. But the webui renders `ghost` with alarm semantics
   (`var(--ghost)`, dashed outline, breathing animation, warning wash), so a
   maintainer sees "broken/drift" where the spec means "not yet built". This
   is the conflation to fix. Two implementation paths exist; the dev plan
   should evaluate both against the spec:

   - **Path A (new variant):** Extend `NodeState` with a `Planned` variant.
     The source of truth for "intentionally unbuilt" cannot be path absence
     alone (that is what `Ghost` already keys on). It must be an explicit
     marker: either a blueprint tag (e.g. `planned`), a blueprint field, or a
     node-level flag. The build derivation (`src/map/build.rs:49`) consumes that
     marker: a node is `Planned` when its path is absent AND the explicit marker
     is present; absent-without-marker stays `Ghost`. This path requires a spec
     amendment (§10.1 gains a fourth state).
   - **Path B (no new variant):** Keep three `NodeState` variants. Fix the
     webui rendering to distinguish "ghost with no findings" (planned, healthy:
     calm colour, solid outline, no breathing) from "ghost with drift findings"
     (code disappeared: alarm colour, dashed, breathing) using the finding data
     already on the node. The spec's definition already says ghost = "planned
     but unimplemented"; the bug is the visual language, not the state model.
     This path requires no spec change and no new enum variant.

   Both paths update the serialise layer (`src/ui/serialise.rs:17`,
   `src/cli/export/json.rs:138`, `src/query_api/handlers/project.rs:71`) and the
   webui legend and node rendering (`src/ui_assets/app.js:286,540,832`) so a
   maintainer can see plan-but-unbuilt nodes at a glance. The dev plan must
   resolve Path A vs Path B (spec amendment vs rendering fix) before
   implementation begins; the adversarial review should stress-test both
   against the spec's two-chain model.

2. **Reconcile task-tracking guidance to the native Todo type; beads become
   optional.** The authority for task tracking is the cairn native Todo artefact
   type (markdown under `meta/todos/<node>/`, surfaced per node in the webui
   Todos panel), per `dec.beads-task-layer`. Beads (`bd`) are an optional
   power-backend and a read-only derived navigational view, not the mandatory
   tracker. AGENTS.md guidance should be reconciled to the accepted decisions:
   the "use bd for ALL task tracking" mandate is narrowed to "bd is optional; use
   cairn's native Todo artefact for durable per-node task tracking, and bd only
   when you want its richer workflow state". The unratified "filesystem default"
   convention should become a decision so the guidance has a single authority to
   cite. This does not remove bd support; it corrects the mandate to match the
   decisions.

3. **Add a cairn-first agent guidance rule for state questions.** Agents should
   query cairn (`cairn rationale <node>`, `cairn context`, `cairn scan`,
   `cairn lint`) before grepping files for "what is the state of X / what
   decisions affect Y" questions. This is a workflow rule, fileable as an AGENTS.md
   line or a bead, and is the low-cost half of the investigation-friction fix. It
   does not require new cairn capability, only that agents know to use the
   capability that exists.

4. **A code-map extension for source-structure questions is a separate future
   direction.** The source-structure half of the investigation (enum variants,
   derivation logic, call sites) is outside cairn's current model.
   `dec.lsp-diagnostics-server` begins a source surface but is
   diagnostics-oriented. A code-map that indexes symbol definitions would let
   cairn answer source-structure questions deterministically. This is recorded as
   a revisit trigger, not pursued here, because it is a larger capability bet and
   the first three rulings close the immediate gap without it.

## Rationale

- Ruling 1 fixes a real conflation in the shipped state model: the same `Ghost`
  value covers intentional future and unexpected drift, so the graph cannot
  encode plan state and the UI cannot show it. Adding `Planned` is a small,
  localised change to the enum, the derivation, the serialise layer, and the
  webui rendering, and it directly answers the maintainer's request to see
  unbuilt nodes in a different colour.
- Ruling 2 makes the guidance agree with the decisions it already contradicts.
  The native Todo type exists and is first-class; the mandate to use bd for all
  tracking is the stale part, and correcting it removes the ambiguity a new
  contributor hits first.
- Ruling 3 is nearly free: the capability exists, agents just do not know to use
  it first. A guidance rule captures the lesson from this session, where manual
  greps duplicated what `cairn rationale` returns in one call.
- Ruling 4 is deferred because the first three close the immediate gap, and a
  code-map is a separate, larger capability with its own design surface.

## Trade-offs

- A fourth `NodeState` variant widens the serialise and render surface and
  requires the webui legend, node fill, and status-dot logic to handle it. The
  cost is bounded (one variant, one colour, one legend entry) and the benefit
  (plan state visible on the map) is the maintainer's explicit ask.
- Reconciling AGENTS.md may be overwritten if `bd setup` regenerates the beads
  blocks. The fix is pin-or-regenerate, recorded as a revisit trigger, not
  solved here.
- The "filesystem default" convention is memory-only; ratifying it as a decision
  is named as a revisit trigger so this decision does not cite a convention that
  is not yet a decision.

## Consequences

- A cairn change (`native-task-state`) should implement ruling 1 (the `Planned`
  state, derivation split, serialise, webui rendering) and ruling 2 (the AGENTS.md
  guidance reconciliation and, if the maintainer sanctions, the storage-backend
  decision).
- Ruling 3 (the cairn-first guidance rule) can land in the same change or as a
  small standalone bead.
- `res.native-task-state-gap` is satisfied: the state gap, the guidance gap, and
  the investigation friction are all ruled on.