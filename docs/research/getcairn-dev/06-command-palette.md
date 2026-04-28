# Command palette and AI pipeline

## What this is

The single most important interaction surface in the running app. Bound to `⌘K`, the palette is **context-aware**: its prefilled actions change with the active node. It is also the entry point to a four-stage AI generation pipeline (`ROUTE → CONTEXT → {GENERATE | REQUIREMENTS | ...} → VALIDATE`) that runs a named specialist agent with explicit per-stage models and wall-clock timing exposed to the user. Generated changes land in a "Review proposed changes" surface with warnings, AI reasoning, and an "Apply with Warnings" acceptance gate.

For the chrome around the palette, see [03-information-architecture.md](./03-information-architecture.md). For node-level actions invoked from the palette, see [04-node-model.md](./04-node-model.md). For the genesis flow that precedes any palette use, see [02-workflow-genesis.md](./02-workflow-genesis.md).

## Palette anatomy

### Context header

Every palette invocation shows the active context as a header chip (**from screenshots 05 and 24**):

```
◇ Context: Offshore Survey USV-ROV (system)
```

The format is `{glyph} Context: {node-name} ({node-type})`. When invoked on a subsystem the type would be `(subsystem)`, on a requirement `(requirement)`, etc. (**inferred** from the consistent pattern.) The diamond glyph `◇` is the contextual-action marker shared with all AI actions across the app.

### Free-text input plus prefill

A multi-line text input dominates the modal (**from screenshot 24**). It supports **prefill from review modal**: clicking an AI-suggested follow-up card prefills the input with that follow-up's exact text and shows the helper line:

```
↩ prefilled from review modal · review and press Enter to run, or edit first
```

This is the same surface used to (a) accept an AI-suggested follow-up verbatim, (b) edit one before running, or (c) type a free-form intent. One input, three modes.

### Contextual actions

Below the input, the palette lists prefilled action shortcuts with right-aligned action chips (**from screenshots 05 and 24**):

```
CONTEXTUAL ACTIONS

  Generate requirements for Offshore Survey USV-ROV          [GENERATE]
  Create a state machine for Offshore Survey USV-ROV         [GENERATE]
  Describe any changes you'd like to make...                 [FREEFORM]
```

When invoked from a subsystem with existing requirements, the actions adapt (**from screenshot 24**):

```
CONTEXTUAL ACTIONS

  Add verification methods for 6 unverified requirements     [VERIFY]
  Create a state machine for Offshore Survey USV-ROV         [GENERATE]
  Describe any changes you'd like to make...                 [FREEFORM]
```

The chip type names the operation class. Three classes observed: `GENERATE` (creates new structure), `VERIFY` (annotates existing structure with verification metadata), `FREEFORM` (escape hatch). The chip taxonomy is **inferred** to be a small fixed set; the underlying mapping to pipeline stage 3 is documented below.

### Footer affordances

`ESC close   ↩ submit` (**from screenshots 18, 24, 27**). Standard keyboard discoverability.

## AI-suggested follow-ups

After a successful generation lands, the next palette invocation can show "What's next?" follow-up cards (**from screenshot 21**):

```
WHAT'S NEXT?  AI-suggested follow-ups · click to prefill the command palette

  ┌────────────────────────┐  ┌──────────────────────────────┐
  │ mission endurance      │  │ USV Platform decomposition   │
  │ (fuel, battery, ...)   │  │ REQ-001 + REQ-005 → ...      │
  └────────────────────────┘  └──────────────────────────────┘
  ...etc, 5 cards total
```

(Five follow-up cards captured in the screenshot.)

The cards are auto-generated post-result suggestions, each one a fully-formed palette command. Click to prefill, edit if desired, submit to run.

This is a strong UX pattern. After every successful operation the user gets a buffet of next-step options grounded in the result, each one click-to-execute. The mechanism is **inferred** to be an AI post-process that proposes 5 next moves based on the just-completed change. Whether the cards are scored, ordered, or templated is **unknown**.

## Generation pipeline (ROUTE → CONTEXT → {stage3} → VALIDATE)

The single load-bearing piece of architecture in this product. When the user submits a palette command, a modal opens showing a four-stage pipeline as a horizontal state machine.

### Captured pipeline shapes

**For "Generate requirements for {system}"** (**from screenshot 18**):

```
   ◆            ○            ○            ○
ROUTE...    CONTEXT      GENERATE     VALIDATE

Engineering your changes... (3.6s)
typically 30-60s
```

**For "Generate requirements for {subsystem} decomposing REQ-003 ..."** (**from screenshot 27**):

```
   ○            ○            ○            ○
 ROUTE        CONTEXT    REQUIREMENTS    VALIDATE

Engineering your changes... (98.2s)
typically 20-40s for requirement generation
```

Note the differences:

- **Stage 3 is operation-specialised.** The first run was labelled `GENERATE` (a generic decompose-or-generate); the second was labelled `REQUIREMENTS` (a more specific operation). **Inferred** that stage 3 takes the operation class as its label.
- **Helper copy is operation-aware.** "Typically 30-60s" vs "typically 20-40s for requirement generation". The copy adapts to the operation kind. The 98.2s elapsed in the requirement run is far above the 20-40s typical, but the UI continues to expose the live counter rather than hiding the lag.

### Per-stage roles (inferred)

The closed-source product gives no public mechanism docs. From naming and from the post-run pipeline trace (next section), the stages plausibly do this:

- **ROUTE.** Decides which specialist agent and which downstream operation to dispatch. Uses the smallest fast model. **Inferred.**
- **CONTEXT.** Assembles the relevant slice of the model graph as input context. Reported as 0.0s in the captured trace, suggesting either a near-instant local query or just the time spent inside this stage relative to wall-clock granularity. **Inferred.**
- **{GENERATE | REQUIREMENTS | DECOMPOSE | ...}.** Runs the specialist agent with the assembled context. The heavy stage; takes most of the time. **Verified** that the label specialises to the operation class; **inferred** that the underlying logic is one specialist agent per class.
- **VALIDATE.** Runs validation against the generated artefacts. **Inferred** to check for structural well-formedness (every requirement is a valid "shall" statement, every node has required fields, no contradictions with existing model). The captured trace shows 46.7s for validation in one run, suggesting this is a real check, not a rubber-stamp.

The total elapsed for the captured 3-operation Decompose run was approximately `3.6s + 0.0s + 47.9s + 46.7s = 98.2s`, which matches the live counter on the in-flight modal exactly (**verified by cross-referencing screenshots 26 and 27**).

## Multi-agent architecture (inferred)

This is the architectural reveal. The "Review proposed changes" header (post-run) exposes pipeline metadata that is otherwise hidden during execution.

### Specialist agent role badge

The header of the review surface includes a coloured role badge:

```
[Requirements Specialist]  [3 operations]  ||  Router Haiku · 3.6s  →
   Context 0.0s  →  Requirements Sonnet · 47.9s  →  Validator 46.7s
```

(**from screenshots 25 and 26**.) Two badges sit before the pipeline trace:

- **Role badge.** `Requirements Specialist` is rendered as an orange-tinted chip. **Inferred** to be one of several named specialist agents, each owning a class of operation. Plausible siblings (none directly captured): `Decomposition Specialist`, `Verification Specialist`, `State Machine Specialist`, `Architecture Specialist`. **Mark inferred: only the Requirements Specialist is directly observable in captured screenshots.**
- **Operations badge.** `3 operations` (or `6 operations` in another captured run). **Inferred** to mean: a single specialist invocation runs N atomic sub-operations (e.g., decomposing REQ-003 into 3 child requirements is "3 operations"; generating 6 system-level requirements is "6 operations"). The agent invocation is therefore batched at the operation level, not at the request level.

### Per-stage model selection

The captured trace (**from screenshot 26**) explicitly names two models inline with stage names:

- **Router stage uses Haiku.** Latency 3.6s. The smallest fast model handles dispatch.
- **Requirements stage uses Sonnet.** Latency 47.9s. The heavier model handles the actual generation.
- **Validator stage** shows wall-clock 46.7s but does not name a model in the captured screenshots. **Unknown** which model runs validation.
- **Context stage** shows 0.0s and no model. **Inferred** to be a non-model graph-query step.

This is a meaningful design: the user can see at a glance which model handles which stage, and how long each took. It is honest, transparent, and locks the product to a specific model vendor (Anthropic, in this case) by exposing the model name in UI copy.

### Wall-clock timing as transparency

Every stage shows elapsed wall-clock seconds inline. Not a percentage, not a vague "thinking..." string, but a real number. Three signals to the user:

1. **What the AI is doing** (per stage).
2. **How long each piece took** (where the time went).
3. **Whether the run is on track** (compared to the operation-specific "typical" range).

The UX trade-off: when a run runs long (98.2s vs 20-40s typical), the live counter visibly drifts past the typical bound. This requires the product to commit to the honest framing instead of fading the typical estimate when overdue. The captured screenshot 27 shows them committing: the elapsed counter is large and present, the helper "typically 20-40s" remains visible, no apology. Real trust is built this way; faked progress bars are not.

### "3 operations" and atomicity

The operations badge implies the specialist runs N sub-operations atomically. Implications:

- **One review surface per agent invocation, not per sub-op.** All N operations land together as a single ChangeSet. The user accepts or discards the whole batch.
- **Per-sub-op warnings.** The captured 6-operation run shows 3 warnings, each tied to a specific generated requirement (REQ-004, REQ-006, plus a structural warning). Warnings are surfaced at the sub-op level inside the review surface.
- **AI Reasoning is per-batch.** The captured AI Reasoning panel describes the agent's selection logic across the whole batch ("I selected six requirements that collectively cover ..."). One reasoning panel per invocation, not per sub-op.

(**from screenshots 22 and 25**.)

### Token and cost transparency

The footer of the review surface shows `~13.2k tokens · ~$0.0790` (**from screenshot 22**). Token count and dollar cost surfaced inline. Plausibly aggregated across the four stages. The dollar cost rounding to 4 decimal places suggests this is computed from token counts and unit prices, not estimated.

This is unusually transparent for a SaaS product. Users see exactly what they are paying for per operation. Worth noting in [09-design-influence.md](./09-design-influence.md#cost-transparency).

## "Review proposed changes" acceptance gate

After the pipeline completes, the user is taken to a `Review proposed changes` surface (**from screenshot 25**). Layout:

- **Header.** Operation summary in plain English: "Generated 6 system-level requirements for the Offshore Survey USV-ROV covering transit performance, ROV depth rating, LARS sea state limit, communications loss response, collision avoidance, and satellite uplink data throughput."
- **Pipeline trace strip.** Same `Requirements Specialist | 6 operations | Router Haiku ... → Validator ...` row described above.
- **Two columns.** Left: `CONTEXT / WARNINGS (3) / AI REASONING`. Right: `6 REQUIREMENTS` with each requirement collapsed to its `id`, `type`, and one-line body, expandable.
- **Footer.** Bottom-left: `Discard changes ~13.2k tokens · ~$0.0790`. Bottom-right: orange `Apply with Warnings` button.

### "Apply with Warnings"

The button label is significant. The presence of warnings does not block apply. It mutates the button label to `Apply with Warnings`. The user accepts the warnings as they apply.

In our terminology, this is a non-blocking acceptance: equivalent to running `cflx accept --strict=false`. Their model treats warnings as advisory at apply time and trusts the user to adjudicate. Our reconciler distinguishes blocking interface contradictions from advisory rationale tensions; the same distinction here is per-warning, not per-class.

## What this implies for our model

(Term-mapping in [07-ontology-comparison.md](./07-ontology-comparison.md); architectural implications here.)

- Their **named specialist agents with role badges** are surface labels for what we already have semantically (Architect, Nazgul, Watcher, Uruk-hai). The labels are more learnable than our internal type strings. Worth exposing role labels in any future webui.
- Their **per-stage model selection visible in UI** (Haiku for routing, Sonnet for generation) is a UX choice we could match in our agent dispatch logging.
- Their **stage 3 specialisation** (`GENERATE` vs `REQUIREMENTS` vs probably more) is a generic skeleton with operation-typed payload. Our cflx phases have similar shape (apply / accept / archive) but as full lifecycle steps, not as sub-stages of one async operation. Different abstraction levels.
- Their **wall-clock per stage** is borrowable as a `cflx run --verbose` output choice and as a webui status surface.
- Their **token + cost surfacing** is borrowable for any webui or agent-output surface we build.
- Their **"Apply with Warnings"** vs our `cflx accept --strict` flag is the same distinction with different framing. Their UI mutates the button; ours mutates the command-line flag.

See [08-borrow-list.md](./08-borrow-list.md) for ranked borrow recommendations including pipeline transparency and the named-role pattern.
