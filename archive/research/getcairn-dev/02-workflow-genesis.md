# Workflow: project genesis

## What this is

Their before-the-build flow. The user supplies a rough free-text intent, then the AI conducts up to three rounds of clarifying questions, builds a "system brief", and only then generates the first-pass model. The full question-and-answer transcript is preserved as a sidebar artefact labelled "**Preserved as provenance**" before any structural work begins. This page documents what the captured screenshots show and notes where the underlying mechanism is opaque.

For the contextual UI ([03-information-architecture.md](./03-information-architecture.md)) and the command palette that takes over after build ([06-command-palette.md](./06-command-palette.md)), see those files. For why this is the highest-leverage borrow target for us, see [08-borrow-list.md](./08-borrow-list.md#1-confidence-bounded-multi-round-interview-as-proposal-genesis).

## The three-round interview

### Round structure

The captured session is on **round 3 of 3** (**from screenshot 01**). The header shows `ROUND 3 OF 3` as a chip and the page title "Shape the starting model" with subtitle "Answer focused questions so Cairn can reduce ambiguity and generate a stronger first-pass architecture."

Each round presents a small batch of focused questions. Round 3 in the captured session shows a 5-question new-batch counter (`READINESS QUESTIONS REVIEWED · 5 NEW`) and a `PROGRESS 0 / 3` counter (**from screenshot 01**). The previous research stronghold reports the rounds being 5+4+3 = 12 questions (**verified** by the prior scout's hands-on inspection). The captured screenshots only show round 3, so the 5+4+3 sequencing cannot be re-verified from this session alone.

### Question shape

Each question is a single one-sentence prompt with 4 to 5 multiple-choice options plus a free-text "Different option" fallback (**from screenshots 01 and 02**). Two captured questions:

1. "What is the USV's primary power source, and how is power delivered to the ROV via the tether?" with options spanning diesel-genset, hybrid diesel-electric, full-battery, and undefined.
2. "What happens if the communications link to the ROC is lost, and what is the required safe-state behaviour for both the USV and the ROV?" with options spanning hold-station-with-timeout, escalate-RTH-waypoint, continue-current-mission, and undefined.

The questions are domain-grounded (USV/ROV terminology, COLREGS regulation references) and multi-faceted: each one bundles a hard architectural decision with a downstream-implication framing.

### Architecture signals (auto-extracted)

The right rail of the question view shows an `ARCHITECTURE SIGNALS` panel that lists three plain-language inferences derived from prior answers (**from screenshot 02**):

1. "A light work-class ROV at up to 1000 m depth requires significant tether power (typically 5-20 kW delivered at high voltage)."
2. "Regulatory framework shapes the entire autonomy and safety architecture."
3. "Comms loss handling is a top-level safety requirement that directly drives the onboard autonomy architecture."

These are **inferred** to be auto-extracted from interview answers (the panel sits in the question view, not on the post-build screen). Their public docs reportedly describe "Architecture signals" as inferred structural constraints that pre-bias generation (**verified** by prior scout against the methodology page). The mechanism is closed; only the surfacing is observable.

### Confidence score

The next-step screen ("Ready to build") shows a chip `82% confidence` next to the BRIEF DESCRIPTION header (**from screenshot 03**). The prior stronghold reports a `78% → 82%` rise across rounds (**verified** by prior scout). Whether this is a hard gate (blocks build below threshold) or a soft indicator (build allowed at any point) is **unknown** from the public surface. The `Build →` button is enabled in the captured screen, suggesting either threshold met or no gate.

### Working system brief (live)

A `WORKING SYSTEM BRIEF` panel runs alongside the question flow (**from screenshot 01**). It already contains a partial system description (`Offshore Unmanned Survey System ...`) in round 3, suggesting the brief is being authored progressively as answers are submitted, not all at once at the end. The brief's final form is shown post-interview (see next section).

## "Ready to build": the genesis snapshot

After interview submission, the user lands on a page titled `Ready to build` (**from screenshot 03**). Three side-by-side columns:

### Left column: project shape

`PROJECT NAME: Offshore Survey USV-ROV` followed by a quick-summary bullet: `1 Root system node, 4-7 subsystems, Key interfaces, System brief`. Action: `Build →`. Below, a `Back to interview` link.

This is a fixed-shape generation contract: every generated model has a root system, 4 to 7 subsystems, key interfaces, and a system brief. See [04-node-model.md](./04-node-model.md) for what a "subsystem" entails. Whether that 4-to-7 count is a hard rule or a heuristic default is **unknown** from public materials.

### Centre column: the genesis transcript

Header: `PROJECT GENESIS · 3 ROUNDS · Preserved as provenance` (**from screenshot 03**). Below, a numbered list of `KEY DECISIONS` with timestamped Q-and-A pairs. The captured screen shows roughly 12 entries, matching the prior scout's report of 5+4+3 rounds. Each entry compresses one question and the chosen answer.

This panel is the load-bearing observation. The `Preserved as provenance` label is rendered in the UI, but **whether the genesis is queryable as a typed artefact, or just stored as display text, is unknown** from the public docs. Their docs reportedly describe the genesis as durably stored before any build action (**verified** by prior scout). What we cannot verify:

- Is it queryable from inside the app via lens or filter?
- Is it linked to specific generated nodes (so a node's existence can be traced back to the question that produced it)?
- Is it exportable in a structured format (JSON, markdown, anything machine-consumable)?
- Does it persist as immutable history if the user later overrides a decision?

These four gaps are the difference between "UI affordance" and "first-class artefact in a provenance chain." See [08-borrow-list.md](./08-borrow-list.md#2-genesis-record-as-a-first-class-provenance-artefact) for why this gap matters most when we adopt the pattern.

### Right column: brief description

The system brief in final form (**from screenshot 03**), tagged `82% confidence`. It is a multi-paragraph plain-English narrative: "The system is a hybrid diesel-electric Unmanned Surface Vessel (USV) and tethered light work-class Remotely Operated Vehicle (ROV) operated in concert from a shore-based Remote Operations Centre (ROC) over LEO satellite ..." with quantitative parameters (depth range 300 to 1000 m, transit speed 4 to 6 m/s, sea state 6).

The brief is the **input** to generation, not the **output**. Generation produces the structured model; the brief stays as the human-readable origin story.

## Build phase: progress checklist

After clicking `Build →`, a progress modal appears (**from screenshot 04**) titled `Building Offshore Survey USV-ROV...` with a `4s elapsed` counter. Five sequential steps:

1. **Creating project structure** (root node + metadata), checked.
2. **Generating system architecture** (decomposing into subsystems), active.
3. **Writing system brief**, pending.
4. **Saving project genesis**, pending.
5. **Ready to explore**, pending.

Two notes worth surfacing:

- **The genesis is saved as a discrete step.** Step 4 is "saving project genesis", separate from "creating project structure" and from "writing system brief". This argues that the genesis is a distinct persisted thing, not just an artefact of the brief writing. Whether that persistence is queryable remains **unknown**.
- **Progress is described as user-facing milestones, not log lines.** This is a UX choice (transparent staging without leaking implementation noise) worth lifting; see [09-design-influence.md](./09-design-influence.md).

## Workflow comparison: their flow vs our cflx flow

| Step | Their app | Our cflx |
|---|---|---|
| 1. Capture intent | Free-text prompt ("a USV with an ROV controlled from a remote ops centre") (**from screenshot 03 brief**) | Human or Architect agent authors `proposal.md` directly |
| 2. Refine intent | 3 rounds of AI clarifying questions, ~12 total. Each round updates a working brief and a confidence score. Architecture signals appear progressively. (**from screenshots 01, 02, 19**) | No equivalent. Architect authors design.md directly. |
| 3. Preserve intent as provenance | "Project Genesis · 3 rounds · Preserved as provenance". Q-and-A transcript labelled in UI. Persistence layer **unknown**. (**from screenshot 03**) | No equivalent. proposal.md is closest analogue but is not labelled as a provenance artefact. |
| 4. Generate first-pass structure | Build with 5-step progress checklist. Output: 1 root + 4 to 7 subsystems + key interfaces + system brief. (**from screenshots 03 and 04**) | `cflx apply`: codex agent executes tasks.md, runs verification gate battery. |
| 5. Review proposals | ChangeSets: per-node accept/skip on commit. (**verified** via prior scout, not directly captured here) | `cflx accept`: human reviews agent output, runs verification, merges worktree. |
| 6. Iterate | Round-numbered refinements; model persists. | Phase lifecycle: archive current, draft next proposal. |

The biggest structural parallel: both treat the human as a gating agent between AI-proposed changes and model state. The biggest gap: their entire pre-build UX (interview, brief, genesis) has no equivalent in our flow. See [08-borrow-list.md](./08-borrow-list.md) for borrow recommendations.

## Open questions about this layer

(Pulled from prior stronghold and from screenshot inspection; **unknown** unless re-verified.)

1. Is the confidence score a hard gate or a soft indicator?
2. Can the user re-enter the interview flow after build, or is genesis frozen at first build?
3. Is the genesis transcript queryable, exportable, or just a UI display panel?
4. Are the 12 questions templated or generated per-domain? The captured questions are domain-aware (COLREGS, ROV power) which suggests at least partial generation.
5. How do "architecture signals" map to specific questions and answers? Is the trace navigable inside the app?

These should be folded into next-pass research if a hands-on session is available.
