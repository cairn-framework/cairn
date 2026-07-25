---
node: cairn.root
status: blocked
created: 2026-07-22
---

# Agent Guidance Program

Umbrella for the Harness Engineering derived agent-guidance and agent-pack
work. It gives the maintainer one master backlog to orchestrate and keeps the
child units in a reviewed dependency order. It is not a Cairn scheduler:
sequencing lives here as reviewed prose and in each child's blocked or open
status, and an external harness drives repetition per `dec.no-orchestrator`,
`dec.loop-command-harness-model`, and `dec.native-todos-first`.

## Goal

Ship the reconciled agent-guidance strategy: a measured baseline, a canonical
harness-neutral pack, one logical `cairn-dev` entry with an explicit loop mode,
apply, proof, and authority guidance, evaluated treatment, an OMP adapter, and
a campaign-reconciliation step, each landed as its own reviewed unit.

## Completion condition

Closes when every child below reaches a terminal disposition (`done`, or removed
from scope) and the umbrella itself is then set `done`. Cairn's Todo status
surface is `open|in_progress|done|blocked` with no `dropped` state, so a drop is
a superseding decision, not a status. `refines` is informational and does not
override an accepted ruling, so a drop that contradicts an accepted decision
uses `supersedes`: author it with `cairn decision new`, restate the target's
surviving obligations inside it, get it to `status: accepted` with `supersedes:`
the target, set that target to `status: superseded` (docs/conventions.md section
10, else CAIRN_DECISION_SUPERSEDES_STATUS), and only then set the child `done`
with a `Dropped:` body note citing the decision.
The treatment verdict resolves `todo.agent-pack-omp-publication`: an accepted
`retain` decision unblocks it (publish); a `remove` verdict drops it via a new accepted decision
that supersedes `dec.agent-pack-packaging` (restating its surviving obligations,
omitting the OMP-target ruling) with that target marked `superseded`, before
publication is set `done`.
A `revise` verdict is not terminal, but this evaluation round still lands `done`
(it completed its evaluation): treatment authors a revision follow-up todo and a
successor evaluation round (`cairn todo new <slug> --node cairn.kernel.cli`, the
successor listing the follow-up in its `Depends on`), adds both to the Waves list
below, records the required changes as a decision, and sets this round `done`.
The campaign continues through the successor round, never by reopening a landed
unit; publication stays `blocked` until a round returns `retain` or `remove`.
Once every child is `done` or accepted-removed, the final
reconciliation sets the umbrella `done` (`cairn todo set agent-guidance-program
done`).

## How to orchestrate this

Two primitives, different jobs:

- Loop against this master todo for the SEQUENCE, supervised. The work is
  inherently ordered and research-adaptive, so it wants one accountable
  trajectory per unit, with fresh context and a reconciliation step between
  units. Cairn has no native programme-membership filter today, so an
  unattended fixed-MISSION `/loop N` cannot reliably scope to this programme:
  topic or slug-prefix matching is not a typed set (an "agent-guidance and
  agent-pack" prefix would omit `agent-context-bundle`,
  `agent-context-bundle-evaluation`, and `spec-authority-retirement`, and could
  catch unrelated todos). Run it supervised instead: between iterations the
  operator or a thin wrapper reads `cairn status` and this umbrella's Waves,
  picks the next eligible child deterministically (lowest wave number, then
  slug), and starts a fresh `/cairn-loop` session with that child's exact slug
  as the MISSION. That uses the loop command's own named-unit branch, so it is
  not a competing selector: the command plus its skills still perform and gate
  the one unit; the operator or wrapper only supplies which eligible child and
  repeats. Each iteration materialises that child into a change, lands one
  squash commit, reconciles the remaining plan, and ends with one terminal
  token. The harness owns repetition.
  Loop mode loads the selected unit's body (`meta/todos/todo.<slug>.md`) before
  Scope and fails closed if it is missing, unreadable, or anchored to a
  different node, so its Scope, Depends on, and Acceptance bind without the
  MISSION having to ask (landed by `todo.agent-guidance-router-playbooks`).
- Use `workflowz` WITHIN a unit for parallel COVERAGE. `workflowz` is an OMP
  magic keyword that builds a one-shot deterministic multi-subagent `task`
  workflow for research, adversarial review, or migration fan-out. It does not
  carry campaign state across sessions and is not a sequencer. Use it inside a
  single unit for parallel discovery or the mandatory adversarial review.

Do not run the whole programme as one `workflowz` fan-out: the units are
dependency-ordered and each unit's evidence can change later units, which a
single parallel wave cannot honour.

Unattended static-MISSION looping is deferred: it needs a native scoped
selector (`cairn next --scope <umbrella>` or a typed todo parent/child edge),
which does not exist today. Until then the supervised per-child invocation
above is the definitive current strategy.

## Readiness

A child is `blocked` until its own dependencies are done. `cairn next` has no
programme filter, so the operator or wrapper (not the loop) chooses which
eligible child to hand in, reading `cairn status` and this umbrella. Blocked
status keeps not-yet-eligible children out of selection. A child is set `open`
once all of its dependencies are done, by the loop's `cairn-loop-reconcile`
step inside the landing unit's commit (`dec.loop-reconcile-step`).

## Waves

Waves group children by dependency tier for reading; they are not
synchronisation barriers. A child becomes eligible when its own listed
dependencies are done, not when every sibling in its wave is done, so tiers can
overlap. The operator or wrapper picks among eligible children in a fixed order
for reproducibility (lowest wave number, then slug). Campaign units run serially
through the single authoritative `/cairn-loop` worktree (`../cairn-loop`), whose
Isolation and preflight contract owns one loop worktree and halts on more than
one open loop PR; parallel-worktree execution of independent children needs a
separately ratified runner that preserves that contract and is out of scope
here.

Wave 1 (done, 2026-07-24):

- todo.agent-guidance-provenance (done; produced `dec.unified-cairn-dev-entry`,
  accepted by the owner 2026-07-23)
- todo.agent-guidance-baseline (done; `res.agent-experiment-linklint` plus the
  `archive/strongholds/agent-guidance-baseline/` evidence bundle)
- todo.agent-pack-canonical-foundation (done; `tools/agent-pack`)
- todo.agent-context-bundle (done; inventory, fixed sample rule, candidate list)

Wave 2 (done, 2026-07-25):

- todo.agent-context-bundle-evaluation (done, PR #456). Recommends composing
  existing verbs and NOT building `context_projection_v1`. Raised two
  out-of-programme todos: `todo.node-symbol-coverage` (the measured binding
  constraint, which no programme child consumes and which needs its own decision
  before it changes `get --symbols`) and
  `todo.brownfield-init-invalid-node-id`. Neither is a prerequisite for any
  child, so neither joins a wave.
- todo.agent-guidance-apply-proof-authority (done, PR #459)
- todo.agent-guidance-router-playbooks (done, PR #460). Landed the migration
  `dec.unified-cairn-dev-entry` sanctioned: `cairn-dev` loop mode is now the
  canonical loop authority and `/cairn-loop` is adapter transport. Also completed
  `dec.loop-command-harness-model` point 2 (the `cairn-loop-scope` and
  `cairn-loop-implement` procedure skills and the ordered required-asset
  closure), and retired `karpathy-guidelines` under
  `dec.retire-karpathy-guidelines-skill`.

Wave 3 (open as of 2026-07-25; dependencies satisfied by wave 2):

- todo.agent-pack-claude-bootstrap (done). Split under the sizing rule into
  three children, all landed: todo.pack-install-lifecycle (done, PR #466) added
  the manifest-owned `cairn pack install|update|status|uninstall` family;
  todo.pack-init-delegation (done, PR #467) made `cairn init` install through
  that same engine and extended `--wire` to the brownfield path;
  todo.pack-campaign-resolver (done, PR pending at authoring time) added
  `cairn pack resolve` and the `campaign start|verify|end` lock that pins the
  resolved closure as immutable bytes for a campaign.
- todo.agent-guidance-campaign-reconciliation (done, PR #461). Added
  `cairn-loop-reconcile` to the loop-mode required asset closure under
  `dec.loop-reconcile-step`, so end-of-unit reconciliation is enforced procedure
  inside the landing unit's commit rather than a maintainer habit between units.
  It also widened the Rust gates to `--workspace`, which had been leaving every
  `tools/agent-pack` test unrun.
- todo.spec-authority-retirement (done). Ratified `dec.spec-authority-retirement`:
  reads are graph-first and `docs/spec.md` is fallback narrative. The write-side
  routing table landed in `docs/conventions.md` section 11 rather than the spec,
  the spec's actuator wording now says operator where the operation is
  agent-capable, and its open-questions section names gap decisions as the home
  for new questions while keeping the four older ones, which belong to no single
  node, in place. Blueprint grammar and artefact schemas stay in the spec, on
  record as blocked on a canonical-home decision.

Wave 4 (in progress as of 2026-07-26; dependencies satisfied by wave 3):

- todo.agent-guidance-treatment-evaluation (blocked again, PR #470). The unit
  reached the loop and found its prerequisite evidence absent: the sealed
  confirmation prompts and ground truth were kept outside the repository by
  design (`archive/strongholds/agent-guidance-baseline/manifest.json` lists them
  and the runner authentication databases under `excluded`), so no session can
  produce the terminal verdict from this checkout. It landed the evidence
  boundary instead, as `res.agent-guidance-treatment-evaluation-blocker` and the
  proposed `dec.agent-guidance-treatment-evaluation-blocked`, and set itself
  `blocked`. Unblocking it is an owner action, not a loop unit: supply a
  runnable authenticated worker epoch plus the sealed materials, then set it
  `open`. Accepting that decision is a separate owner action.
- todo.agent-pack-omp-adapter (open; implement and validate, after the Claude
  bootstrap and router). Independent of the treatment blocker: this unit
  completes at a validated, unpublished adapter.

Wave 5 (after OMP adapter completion and the treatment retain verdict):

- todo.agent-pack-omp-publication (publish the OMP adapter only after treatment
  retains the revised pack)

Out of programme: todo.blueprint-authorability-eval keeps its own dependency on
todo.example-corpus-scan-assertions.

## Shared invariants

- One unit, one change, one squash commit, one PR, per the cairn-pr-landing
  workflow.
- Cairn never schedules or repeats; the harness owns iteration.
- Ownership manifests stay file ledgers; no workflow schema.
- Loop mode requires explicit selection; broad `cairn-dev` matching never
  activates it.
- No accepted decision is contradicted without a superseding decision that
  restates its surviving obligations, with the superseded target marked
  `superseded` (`refines` is informational and does not override a ruling).
- No em-dashes; British spelling.

## End-of-unit reconciliation

Cairn has no native dependency edge between todos, so this obligation is
procedural rather than typed. Since `dec.loop-reconcile-step`, it runs INSIDE
the landing unit: `cairn-dev` loop mode loads the selected unit's body before
Scope, and loads `cairn-loop-reconcile` after Verify and before Land. Its edits
are staged into that unit's single commit, so the next fresh session reads the
reconciled plan from main with no external memory and no intervening maintainer
commit.

What the landing unit owes this umbrella, per that recipe:

1. Record what the unit's evidence showed (Research artefact), and a Decision
   when a rule changed.
2. Set each dependant `open` once every entry in its `Depends on` list is done.
   If the unit created or revealed a new prerequisite, add it to an appropriate
   Waves tier so the selector can reach it, add it to the dependant's `Depends
   on` list, and set that dependant `blocked` until it is delivered.
   For a child gated on a treatment verdict (publication), do not use the generic
   done-rule: a treatment round is `done` on `revise` too, and a retain record
   may still be `proposed`. Keep it `blocked` and open it explicitly only after
   verifying an accepted (`status: accepted`) `retain` decision; on a `revise`
   round spawn the successor and leave it `blocked`; on an accepted `remove`
   decision, drop it.
3. Correct downstream todo or spec bodies the evidence invalidated (`cairn todo
   set` changes only status), including this Waves list.
4. If every other child is now `done` or accepted-removed, close the umbrella:
   `cairn todo set agent-guidance-program done`.

What stays with the operator is selection, which is scheduling and belongs
outside Cairn (`dec.no-orchestrator`): read `cairn status` and these Waves, pick
the eligible child with the lowest wave number then lowest slug, and hand that
exact slug to a fresh loop session. A maintainer may still land a small
tracking-only commit on main to correct something no landing unit could see, but
it is no longer the mechanism that keeps the plan current.
