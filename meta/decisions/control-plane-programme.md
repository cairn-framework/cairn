---
id: dec.control-plane-programme
nodes:
  - cairn.root
  - cairn.ui
  - cairn.mcp
status: proposed
ratification: binding
date: 2026-08-02
informed_by:
  - res.inversion-convergence-minutes
refines: [dec.product-perimeter, dec.north-star-continuous-loop]
---

# Control-plane programme boundary

`status: proposed` is intentional: this binding decision is enqueued for maintainer signature and is not self-ratified.

## Context

The over-harness console is the human steering surface for a programme whose graph carries selection truth and whose driver and harness carry out work. Before this authoring unit, the console todo assigned authorship of the ownership boundary to the console unit itself, which made a maintainer signature a tail dependency. The console can ship its read and composition surfaces while this decision waits, but it cannot ship final write or dispatch behaviour without a signed boundary.

The adopted framing for that boundary is:

> build/delivery/runtime FACTS sit inside cairn's investigation boundary; scheduling, execution, supervision stay outside its actuation boundary

## Decision

Cairn decides what work exists and what is ready; a separate driver
starts that work; the console only shows state and records rulings.
The maintainer signs this split of jobs.

1. **Ownership.** Cairn owns policy and control: selection truth, dependencies, readiness, leases as declarative truth, declarative lease policy, lease facts, and dispatch policy. The driver owns assignment, lease acquisition, lease renewal, active lease state, and dispatch scheduling: when work starts, retries, and supervision. The harness owns execution: how work runs.
2. **Declarative control plane.** If cairn owns selection truth, dependencies, readiness, leases as declarative truth, declarative lease policy, lease facts, and dispatch policy, it is the declarative control plane by construction. This is declarative policy and record, not assignment, lease acquisition, renewal, active lease state, scheduling, execution, or supervision.
3. **Console write authority.** Once this proposed binding decision is signed and its required supersession lands, the control-plane console alone may write through sanctioned surfaces: `cairn todo set` and `cairn feedback` today, plus the paired `cairn todo link` and `cairn todo unlink` relationship verbs when `todo.todo-relationship-schema-implementation` lands. The general UI and MCP surfaces receive no part of this grant. `dec.cli-agent-workflow-consolidation` carries the live todo and feedback obligations; `dec.todo-relationship-model` governs the relationship semantics.
4. **Console dispatch authority.** The control-plane console dispatches nothing. The driver decides when work starts, handles retries, and supervises it; the harness executes it. The console may display and compose the facts those components expose.
5. **Unsigned boundary.** Until the maintainer signs this binding decision and resolves the required supersession, the control-plane console remains read-only and dispatches nothing. A signature accepts or rejects this stated boundary; silence never grants authority.
6. **Surviving general surface obligations.** The `cairn.ui` surface remains an embedded HTTP server serving a read-only graph explorer, and the `cairn.mcp` surface remains a Model Context Protocol server exposing cairn queries as tools. Both surfaces consume the same query API as the CLI to avoid semantic drift; UI assets live under `src/ui_assets/` and are served statically; MCP schema changes require updating `src/mcp.rs` and dependent clients. This decision grants no write authority to either general surface.

## What ratification must do

Accepting this is not a status flip alone. This proposal narrows the accepted `dec.user-surfaces` ruling that the web graph explorer is read-only, so ratification must resolve that contradiction in the graph:

- Mark `dec.user-surfaces` `status: superseded` and add `supersedes: [dec.user-surfaces]` here. Clause 6 carries every surviving obligation from `dec.user-surfaces`, including the embedded read-only explorer, query-only MCP wrapper, shared query API, static UI asset serving, and MCP schema and dependent-client update obligation. The two edits land together in the acceptance commit. `supersedes` only validates once the target is already marked.
- Until that acceptance commit, this proposal cannot retire the accepted ruling. The control-plane console remains read-only and dispatches nothing under the standing `dec.user-surfaces` ruling, which is the Task 1 fallback.

## The rubric

- **Tier**: `binding`. It rules the system root, the UI, and the MCP surface at once, refines two accepted decisions, and only the maintainer can sign it. The mechanical detail lives in Rationale.
- **Unblocks**: the console's signed write and dispatch ruling. The read-only console already shipped as the fallback; this signature is the only thing the wider console waits on.
- **Alignment**: Against `dec.cairn-mission` first, this decision protects the maintainable, investigable, extendable, and fit-for-purpose properties by making the console boundary explicit and queryable.
  - Goal 1: Agents keep working while selectable work exists because this decision is visible in the signature queue and the console keeps its read-only view while it waits.
  - Goal 2: Guardrails keep the result aligned because who owns the rules, who may write, and who starts work are recorded in the graph rather than left to session memory.
  - Goal 3: The maintainer signs only this split of jobs; the console's approved write commands and the driver's work keep day-to-day activity outside that signature.
  - Goal 4: The decision is enqueued before implementation, so a late signature is visible and the console can reroute to read-only work instead of creating a surprise signature.
  - Goal 5: This record names the target, and the signature queue now shows it.
- **Options considered**: (a) keep authorship inside the console unit, which leaves the signature on the campaign tail; (b) let the console own policy, control, and dispatch, which collapses the boundary; (c) split the jobs three ways: cairn holds the rules, the console writes only through the approved commands, and the driver starts work. This is the recommendation. The cost of rejecting option (c) is a stalled signature or a console that starts work nobody authorised.

## Rationale

`dec.product-perimeter` supplies the external-orchestration boundary and `dec.north-star-continuous-loop` supplies the queue rubric and maintainer-only binding signature. This decision refines those two accepted authorities for the control-plane console rather than superseding their ancestry in `dec.no-orchestrator`. `dec.user-surfaces` remains accepted until the ratification step above resolves the narrower console exception.

The north-star decision assigns assignment, lease acquisition, renewal, active lease state, and scheduling to the external consumer layer. This decision makes that refinement explicit by keeping lease truth, readiness, declarative lease policy, and recorded lease facts in cairn while the driver owns the lease lifecycle and dispatch. The harness owns execution.

The accepted UI and MCP obligations survive the proposed exception through clause 6. The sanctioned todo and feedback seams remain governed by `dec.cli-agent-workflow-consolidation`; relationship semantics and the future verbs remain governed by `dec.todo-relationship-model` and `todo.todo-relationship-schema-implementation`.

The adopted investigation and actuation framing is recorded in `res.inversion-convergence-minutes`; this decision adopts it, and the maintainer's signature is the rejection point if the wording or ownership is wrong.

## Consequences

- While this proposal is unsigned, it grants the control-plane console no write or dispatch authority; the implementation follows the read-only fallback in Task 1.
- Once signed and superseded as specified above, only this control-plane console receives the narrow sanctioned write grant; every `dec.user-surfaces` obligation restated in clause 6 remains in force.
- Cairn retains lease truth, readiness, declarative lease policy, and recorded facts; the driver owns assignment, lease lifecycle, active state, and dispatch, while the harness owns execution.
- This binding decision remains proposed in the signature queue until the maintainer resolves it, and the todo stays open to track that signature.
