---
id: res.native-task-state-gap
nodes:
  - cairn.root
date: 2026-06-25
method: primary
---

# The native task-state gap: conflated plan-state, stale task guidance, no unbuilt signal

## The question

A maintainer asked two things at once: (1) does cairn need beads (`bd`) for task
tracking, or did we decide it does not? and (2) cairn should have a native way to
show subtasks and plan state, ideally visible on the graph (a node that is part
of the blueprint but not yet implemented should read differently from a node
whose code disappeared). This note grounds both against the shipped model and the
current repo guidance, because the answer to each informs the other.

## Finding 1: NodeState conflates "not yet built" with "code disappeared"

The runtime node-state model is `src/map/graph.rs:9`:

```rust
pub enum NodeState {
    /// Declared path exists and has claimed files.
    Synced,
    /// Declared path or contract target is currently absent.
    Ghost,
    /// Source reality exists but no eligible node owns it.
    Orphaned,
}
```

The derivation in `src/map/build.rs:49` is:

```rust
let state = if node.paths.is_empty()
    || node.paths.iter().any(|path| root.join(path).exists())
    || !files.is_empty()
{
    NodeState::Synced
} else {
    NodeState::Ghost
};
```

So a blueprint node whose declared path does not yet exist on disk is marked
`Ghost`. That single state covers two genuinely different situations:

- **Planned, not yet built.** The node is in the blueprint because we intend to
  build it. The path is absent because the work has not started. This is a
  healthy forward-looking plan state, not a defect.
- **Ghost / drift.** The path used to exist (or was expected to exist) and the
  code is gone or never landed. This is the integrity signal `Ghost` was named
  for.

There is no `Planned` (or `Unbuilt`) variant. The graph cannot distinguish
"intentional future" from "unexpected absence", so the UI cannot render them
differently, and the webui legend (`src/ui_assets/app.js:832`) offers only
"synced / ghost / orphaned". A maintainer looking at the map cannot see which
nodes are part of the plan but not yet implemented; they all collapse into the
same ghost styling and the same drift semantics.

## Finding 2: AGENTS.md mandates beads for ALL task tracking, but the project moved past that

`AGENTS.md` carries two injected Beads blocks (a managed integration block and a
Codex setup block). Both state, in bold equivalent: "Use `bd` for ALL task
tracking. Do NOT use TodoWrite, TaskCreate, or markdown TODO lists." The managed
block's Conservative profile says use `bd` for tracking but do not commit/push
unless asked.

Three accepted decisions and one recorded project convention contradict or
narrow this:

- **`dec.bead-github-sync` (accepted).** Rejects a second source of truth and
  keeps bead data in the local Dolt store, not the cairn graph.
- **`dec.beads-task-layer` (accepted, 2026-06-23).** Beads are a **read-only,
  derived navigational view**, not a cairn Todo source. The repo's first-class
  Todo artefact type (`meta/todos/`, surfaced per node in the webui Todos panel)
  is the native mechanism; beads are surfaced *alongside* it, never minted into
  it. If a maintainer later wants beads promoted to a genuine Todo source, that
  is a separate change gated on a ratified spec amendment.
- **`dec.bd-upgrade-plan` (accepted).** Treats bd as a pinned tool with a
  deferred upgrade, not as load-bearing infrastructure.
- **Recorded project convention (2026-06-24):** "The default backend is the
  filesystem with a uniform cairn interface. bd (Go binary) is no longer the
  default and becomes an optional power-backend." No matching decision file lives
  in `meta/decisions/` (search for "storage backend" / "optional power" returns
  nothing), so the convention is memory-only and not yet ratified as a decision.

The practical result: an agent reading `AGENTS.md` is told beads are mandatory
for tracking, while the project's own accepted decisions say cairn has a native
Todo artefact type and beads are an optional derived view. The guidance and the
decisions disagree, and the disagreement is exactly the ambiguity a new
contributor hits first.

## Finding 3: cairn already has a native Todo artefact type, unused

`src/artefacts/registry/types.rs:81` defines `TodoStatus`; `load_todos`
(`src/artefacts/registry/mod.rs`) reads markdown under `meta/todos/<node>/` into
`Vec<Todo>`; the query API serialises it and the webui renders a per-node Todos
panel. `dec.beads-task-layer` confirms this is the first-class mechanism. The
repo simply does not populate `meta/todos/` today, so the panel is empty and the
native path looks absent when it is merely unused.

## What this implies for the graph

The maintainer's request, "I should be able to see visibly, like nodes etc in a
different colour, if they are part of the plan but not yet implemented", maps to
a real gap in `NodeState`. Today the only "absent" signal is `Ghost`, which
means drift. A plan state (`Planned`: declared in blueprint, path not yet
present, no drift finding) would let the webui render unbuilt nodes in a
distinct colour and let the legend distinguish "intended future" from "code
gone". This is native state in cairn's own reconcile model, not a beads feature.

## What this implies for task tracking

The task-tracking guidance should be reconciled to the accepted decisions: cairn
has a native Todo artefact type (markdown under `meta/todos/`, surfaced in the
webui); beads are an optional power-backend and a read-only derived view, not the
mandatory tracker. The unratified "filesystem default" convention should become a
decision so the guidance has a single authority to point at.

## Finding 4: the investigation itself is a symptom (cairn should fetch this deterministically)

Reconciling the beads guidance against the accepted decisions took manual greps
across `meta/decisions/`, `src/`, and `AGENTS.md`. Two distinct investigations
were folded into one slow pass:

- **Artefact-graph questions** (which decisions affect task tracking on
  `cairn.root`?) cairn already answers in one call: `cairn rationale cairn.root`
  returns every attached decision. I used it for `cairn.ui` but fell back to
  grepping decision files for `cairn.root`. That fallback is a workflow gap, not
  a tool gap.
- **Source-structure questions** (what variants does `NodeState` have, how is
  state derived in `build.rs`?) are outside cairn's model. Its graph is blueprint
  structure plus artefacts, not a symbol or source index.

Two follow-ups follow from this split:

1. **Agent guidance / intercept.** A rule (AGENTS.md line or a cairn-owned
   intercept analogous to how `rtk` shadows `grep`) that tells any agent to
   query cairn first (`rationale`, `context`, `scan`, `lint`) for state-of-things
   questions before grepping files. This is a workflow issue, fileable as a bead.
2. **Code-map extension.** `dec.lsp-diagnostics-server` (accepted) begins a
   source surface but is diagnostics-oriented, not a definition/variant index. A
   code-map that indexes symbol definitions would let cairn answer the
   source-structure half deterministically, closing the gap that forced the
   manual source reads here.

## Honesty caveat

The `NodeState` derivation and the `Ghost` semantics are source-verified. The
"filesystem default" convention is memory-only (no decision file); I did not find
a contradicting decision, but absence of a file is not proof one was never
written elsewhere. The AGENTS.md blocks are auto-injected by `bd setup`, so they
will reappear if regenerated, which means reconciling the guidance is a
"regenerate or pin" problem, not a one-time edit.