---
node: cairn.kernel.cli
status: blocked
created: 2026-07-22
---

# Agent Guidance Router and JIT Playbooks

## Priority

P1. This is the primary guidance intervention.

## Depends on

`todo.agent-guidance-baseline`, `todo.agent-pack-canonical-foundation`, and
the accepted refining decision proposed by `todo.agent-guidance-provenance`.
This unit consumes that decision; it does not re-author it.

## Recommended model, decision required

There is one logical public entry for Cairn-guided development: `cairn-dev`.
Its default interactive mode is a compact index and router, not an
orchestrator and not a manual. It reads target-local authority, queries Cairn
for structural truth, classifies the session need, and loads one private task
reference just in time.

Its explicit `loop` mode resolves to the existing one-unit, fail-closed loop
contract and terminal tokens. Interactive routing may show the adapter-native
invocation but must never infer, load, or invoke autonomous loop mode. The user
or outer harness selects it explicitly. Ralph, OMP, Claude, or another external
harness owns repetition; the mode performs exactly one fresh-session iteration
on the unit it is given.

## Scope

- Do not move authority until the refining decision from
  `todo.agent-guidance-provenance` is accepted. That decision preserves every
  semantic clause of `dec.loop-command-harness-model` and changes only the
  canonical location of loop authority from a standalone `/cairn-loop` to
  explicit `cairn-dev` loop mode. This unit implements the migration the
  decision sanctions; it never authors a second, competing authority artefact.

- Keep the emitted guide to target-authority precedence, the first Cairn
  orientation query, the strict gate, and the route to `cairn-dev`.
- Turn `cairn-dev` into the compact router with a measured first-turn and
  advertised-metadata ceiling derived from the baseline.
- Store general task playbooks as non-discoverable JIT references, not public
  competing skills and not embedded in the router.
- Add focused references for bug investigation, refactoring, architecture
  discovery, and feature implementation. Each maps the task to exact Cairn
  query sequences and names when source/LSP inspection is still required.
- Fold a pre-Scope body-load into the loop mode: load and validate the selected
  unit's todo body (`meta/todos/todo.<slug>.md`) before Scope so its Scope,
  Depends on, and Acceptance bind, and fail closed if it is unavailable. Today's
  `/cairn-loop` selects the slug but scopes only via `neighbourhood`, `rationale`,
  and `deps` and renders todos as status and path, so the body is otherwise never
  guaranteed to reach the executing session.
- Complete `dec.loop-command-harness-model` point 2: extract the loop mode's
  Scope and Implement plus Test recipes into required `cairn-loop-scope` and
  `cairn-loop-implement` procedure skills (working names), declare typed exits,
  and fail closed when either is unavailable. Define one ordered required-asset
  closure containing scope, implement, recovery, landing, and every transitive
  prompt asset; both adapters and campaign locks consume that same closure.
- State that `map.json` and `map.md` are generated review snapshots, never
  routine agent context.
- Add the node-not-found recovery ladder: exact id, error suggestions and
  suffix aliases, path lookup, then filesystem search.
- Require span-windowed source reads. Use the existing JSON `end_line` field
  rather than expanding the human text renderer inside this intervention.

## Acceptance

- General sessions discover only the `cairn-dev` entry and load no task
  reference before routing requires it.
- Adapter tests prove each route loads exactly its intended reference.
- Loop mode requires explicit user or harness selection and never activates
  from broad `cairn-dev` skill matching.
- Every adapter-native loop invocation resolves to the same canonical
  `cairn-dev` loop mode, which remains authoritative and fail-closed unless
  its scope, implement, recovery, and landing procedures are all available.
- The loop mode loads and validates the selected unit's todo body before Scope
  and fails closed when it is unavailable.
- Router and JIT token costs fit the preregistered ceiling.
- No accepted decision or target-repository instruction is duplicated as a
  competing authority.
