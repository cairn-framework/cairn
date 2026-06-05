# The Cairn Dev Loop

How to iterate on cairn, using cairn. This is the canonical development loop for
this repo: a repeatable sequence that drives every code change through cairn's
own graph queries and gates. Load this file when starting a development
iteration, or invoke `/cairn-loop` to run it.

The premise is dogfooding. Cairn's job is to keep a declared architecture map in
sync with real code and to carry provenance for why the structure is shaped the
way it is. So the loop uses cairn to orient before touching code, to scope the
blast radius, and to gate the result. The framework verifies its own
development.

## Prerequisites

Cairn must reflect the current source, so build before you query:

```bash
cargo build --release
export PATH="$PWD/target/release:$PATH"   # so `cairn` resolves to this build
cairn --version
```

Rebuild whenever you change cairn's own reconcile or scan logic, otherwise the
graph you query is stale relative to the code you just wrote.

## The loop at a glance

| Phase | Question it answers | Primary tool |
|---|---|---|
| 1. Orient | What state is the graph in right now? | `cairn context`, `cairn lint` |
| 2. Scope | What does this change touch, and why is it shaped this way? | `cairn neighbourhood`, `cairn rationale`, `cairn dependents` |
| 3. Propose | What am I building, and what counts as done? | `cairn-propose` skill, or a decision artefact |
| 4. Implement | Make the change, keep the map honest | edit code + `cairn.blueprint` |
| 5. Verify | Did I introduce drift or break the build? | `cargo` gates, `cairn scan`, `cairn hook all` |
| 6. Record | Why did this change happen? | decision artefact, `cairn rationale` |
| 7. Land | Commit and push the iteration | `gt` / git |

Each phase has an exit criterion. Do not advance until it is met. When the
iteration lands, loop back to phase 1 for the next one.

## Phase 1: Orient

Understand the graph before adding to it. Triage existing findings first so you
never confuse pre-existing drift with damage your change caused.

```bash
cairn context              # nodes, edges, artefacts, finding counts
cairn lint --json          # structured findings; errors block, warnings advise
cairn status               # active changes, snapshot state
```

Exit criterion: you know the current finding count (the target baseline is
zero), and you can name the node or nodes you are about to touch.

## Phase 2: Scope

Pull the neighbourhood around the change so you size the blast radius and respect
prior decisions.

```bash
cairn neighbourhood <node> --include-todos --include-changes
cairn rationale <node>                 # decisions, research, sources behind it
cairn dependents <node> --transitive   # who breaks if this node's interface moves
cairn depends <node> --transitive      # what this node leans on (cycle check)
```

If `cairn rationale` surfaces an accepted decision that constrains the area, work
within it or write a superseding decision in phase 6. Do not silently contradict
the provenance chain.

Exit criterion: you have a written, verifiable success criterion and you know
which modules and edges the change affects.

## Phase 3: Propose

Capture intent before code. Match the proposal weight to the change.

- Substantial work (new module, cross-cutting behaviour, interface change): use
  the `cairn-propose` skill to scaffold a change under `openspec/changes/` with
  proposal, design, and tasks. Then drive it with `cairn-apply`.
- Small surgical work (a bugfix, a doc, a contained refactor): skip the change
  directory. State the success criterion inline and, if it alters structure,
  plan the decision artefact you will write in phase 6.

Exit criterion: the intent is recorded somewhere durable, not only in your head.

## Phase 4: Implement

Make the smallest change that satisfies the criterion (see the
`karpathy-guidelines` skill). Keep the map honest as you go:

- New source file: confirm it falls under an existing node's `path` in
  `cairn.blueprint`. If not, extend a node's `path` or declare a new Module.
- New cross-module call: add the edge in the blueprint,
  `from.id -> to.id "relationship label"`, and check `cairn depends` for cycles.
- User-facing CLI string: add it to `docs/design-system/copy.toml` and wire it
  via `copy::lookup(...)`. Do not hardcode messages in Rust.

Run `cairn scan` mid-flight whenever you add or move files, so orphans surface
immediately rather than at the gate.

Exit criterion: the change compiles and the blueprint still describes the tree.

## Phase 5: Verify

This is the gate. Run the language gates and the cairn gates together.

```bash
# Code gates (run when Rust changed)
cargo build                                          # zero warnings
cargo clippy --all-targets --all-features -- -D warnings
cargo test

# Graph gates (run for every change)
cairn scan                 # zero findings is the target
cairn hook all             # exit 0 means the commit is safe
```

`cairn scan` clean means no orphaned files, no duplicate IDs, no dangling edges,
no interface drift. `cairn hook all` is the strictest gate and matches what CI
enforces. If either reports an error finding, read it and fix the underlying
cause. Do not bypass hooks (`--no-verify` and `SKIP=` are forbidden in this
repo).

Exit criterion: all gates green. A failing gate is a blocked iteration, not a
formality to wave through.

## Phase 6: Record

If the change altered structure or made a non-obvious tradeoff, write a decision
artefact so the next iteration can read why.

```yaml
---
id: dec.<short-name>
nodes: [<node.id>]
status: accepted
date: <YYYY-MM-DD>
---

Context, the decision, and the rationale.
```

Provenance in cairn is all-or-nothing once any node declares a `decisions`
pointer: the moment one decision exists, every leaf node without a covering
decision raises `CAIRN_PROVENANCE_NO_DECISION` (a non-blocking warning). So
wiring decisions into the blueprint is a deliberate, repo-wide commitment, not
an incremental one. Until the repo makes that commitment, keep decision records
in `meta/decisions/` as durable prose and confirm with `cairn rationale <node>`
once a `decisions` pointer is added.

Exit criterion: a reader can reconstruct why this iteration happened.

## Phase 7: Land

Commit and push through the repo's VCS workflow (Graphite; see
`docs/agent/graphite.md`). Re-run `cairn scan` one last time if you amended after
verifying.

```bash
gt create -m "<type>(<scope>): <subject>"
gt submit --stack --publish --no-interactive
```

Before submitting a PR, run `/reforge` then `/debate` on the diff per
`CLAUDE.md`. Skip that only for a single-line documentation change.

Exit criterion: the work is pushed and `git status` shows the branch up to date
with its remote. Then loop back to phase 1.

## When the loop reveals a finding it cannot clear

Sometimes phase 5 surfaces drift that is correct intent the blueprint has not
caught up to (a deliberately added module, a renamed path). That is the loop
working: the map disagreed with the code. Reconcile by updating the blueprint to
match the new reality, write the decision in phase 6 explaining the structural
move, and re-scan. Never silence a finding by ignoring the file; either the code
or the declaration is wrong, and the loop's job is to force that choice into the
open.
