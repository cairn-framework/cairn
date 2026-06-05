---
name: "Cairn Dev Loop"
description: Run one iteration of cairn development, driven by cairn's own graph queries and gates
category: Workflow
tags: [workflow, cairn, dogfood]
---

Run one iteration of the Cairn Dev Loop: develop cairn, using cairn. The full
loop is documented in `docs/agent/cairn-dev-workflow.md`; load it for phase
detail. This command is the orchestrator.

**Input**: the argument after `/cairn-loop` is a description of the change to
make this iteration. If none is given, ask what to build with the
**AskUserQuestion tool** before proceeding.

**Setup**

Cairn must reflect current source. Build and put it on PATH first:

```bash
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

**Steps**

Work the seven phases in order. Do not advance until a phase's exit criterion is
met. Track progress with `bd` (this repo's tracker), not TodoWrite.

1. **Orient** — `cairn context` and `cairn lint --json`. Record the baseline
   finding count (target zero) and name the node(s) you will touch. Triage any
   pre-existing findings before adding work.

2. **Scope** — for each target node: `cairn neighbourhood <node>
   --include-todos --include-changes`, `cairn rationale <node>`, and
   `cairn dependents <node> --transitive`. Respect accepted decisions. Write a
   verifiable success criterion.

3. **Propose** — for substantial work, invoke the `cairn-propose` skill to
   scaffold a change, then `cairn-apply`. For small surgical work, state the
   success criterion inline and plan any decision artefact.

4. **Implement** — make the smallest change that satisfies the criterion
   (`karpathy-guidelines` skill). New files must fall under a node `path` or get
   a new Module. New cross-module calls get a blueprint edge. CLI strings go in
   `docs/design-system/copy.toml`. Run `cairn scan` whenever you add or move
   files.

5. **Verify** — the gate. Run `cargo build`, `cargo clippy --all-targets
   --all-features -- -D warnings`, and `cargo test` when Rust changed; always run
   `cairn scan` (zero findings) and `cairn hook all` (exit 0). Fix the cause of
   any error finding. Never bypass hooks.

6. **Record** — if structure changed or a non-obvious tradeoff was made, write a
   decision artefact (`meta/decisions/`). Confirm intent with `cairn rationale`.

7. **Land** — commit and push via Graphite (`docs/agent/graphite.md`). Before a
   PR, run `/reforge` then `/debate` on the diff per `CLAUDE.md`.

**Output**

Summarize the iteration: the success criterion, the nodes touched, the final
`cairn scan` finding count, the gate results, and whether the work landed. If a
gate blocked, report the finding and where it stuck rather than waving it
through. Then offer to loop again for the next iteration.

**Guardrails**

- A clean `cairn scan` (zero findings) is the target state. A finding is a
  blocked iteration, not a formality.
- Reconcile drift by fixing the code or the blueprint, never by ignoring a file.
- Do not contradict an accepted decision without writing a superseding one.
