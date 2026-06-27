# Design influence: visual and voice cues to absorb

## What this is

Visual and voice observations from getcairn.dev's marketing surface, docs, and running app, scoped to what is worth absorbing into our own surfaces. Shorter than the structural docs in this set; treats craft as the subject. For the structural borrows that complement these cues, see [08-borrow-list.md](./08-borrow-list.md).

The CAIRN project's brand authority lives in `docs/design-system/`. None of what follows is a recommendation to override our tokens; the absorption is at the level of patterns and rhetorical moves that compose well with our existing identity.

## Type system observations

Their typography pairs three voices:

- **Serif**, used for marketing headlines and major page titles. Looks like Geist Sans pairs with a serif body in long-form posts; the docs use a serif for body prose plus a sans for navigation chrome. Reads warm and editorial.
- **Sans**, used for UI chrome (tabs, button labels, status bar, side rail field labels in small caps).
- **Mono**, used for IDs (`SUB-LARS`, `REQ-001`, `IF-002`), code-like tokens (`Claude Sonnet 4` model chips), and stage labels (`ROUTE`, `CONTEXT`, `GENERATE`, `VALIDATE`).

The triplet (serif body for prose, sans for chrome, mono for technical tokens) is the same pattern our design system already uses (Source Serif 4 for headings and long-form, IBM Plex Sans for UI chrome, IBM Plex Mono for code and technical vocabulary). What is worth borrowing at the rhetorical level is not the specific typeface choices but the discipline of using mono on every typed identifier. They never inline a node id in body prose without monospacing; this is consistent across all 34 captured screens.

## Color palette feel

A warm, restrained palette dominated by:

- Cream / off-white parchment as the page ground. Not a pure white. Reads as paper-like.
- A rust-or-copper accent for primary actions (`Generate Systemigram`, `Build →`, `Apply with Warnings`), focused inputs, and highlighted selections.
- Sage / teal for `si` units, success dots, and the Process axis on the fidelity radar.
- Ochre for the Entity axis on the fidelity radar plus warning callouts.
- Purple for the Relationship axis.
- Neutral greys for body chrome, dividers, and inactive controls.

The palette is **deliberately low-contrast**. Where we use higher-contrast accents in the landing page, theirs leans toward editorial calm. The advantage: dense panels (system tree plus side rail plus main content plus status bar) feel breathable rather than cluttered. The trade-off: status indicators are subtle; users learn the colour code rather than reading at a glance.

We should not copy their palette directly. We should note that **panel-rich density without visual noise is a craft win** and worth attending to in any future webui surface we build.

## Layout density: panel-rich, dense but breathable

The captured project root view (screenshot 06) shows simultaneously:

- A 10-tab top-nav strip.
- A breadcrumb plus three quick-action chips (`REVIEW 1`, `QUALITY 73`, `Command`).
- A left rail with system tree plus 8-tool sidebar plus three project-level actions.
- A main content area with system header, system budgets, six subsystem cards in a 3-by-2 grid.
- A right side panel (when a node is selected).
- A status bar with seven counters plus a `⌘K` hint.

That is roughly twenty distinct UI primitives on a single screen. It works because of strict typographic hierarchy (small-caps muted labels for sections, serif headers for content, mono for IDs, sans for chrome) plus generous negative space between groups. The user can scan top-down without a hunt for primary action.

What we can absorb: **the discipline of section labelling**. Every group has a small-caps muted label above it (`SYSTEM TREE`, `TOOLS`, `BUDGETS`, `PROPERTIES`, `INTERFACES`, `ATTACHMENTS`, `CAUSAL POSITION`, `DEPENDENCIES`). The label is not decoration; it tells the user what the next chunk is for. We do this inconsistently; their consistency is a craft win.

## Voice in their docs: warm, narrative, anchored on examples

The docs voice is:

- **Plain and concise.** Average sentence length is short. No technical jargon without immediate paraphrase ("a Cairn is the stone markers hikers build to mark a path through complex terrain").
- **Narrative-first.** Concept pages open with a problem statement, then contrast, then walkthrough. The Lens Paradigm page begins with "Most engineering tools create separate documents for separate concerns."
- **Worked-example-anchored.** A single rover example threads through every front-page section. The docs use a delivery rover throughout. The reader builds intuition once and reuses it everywhere.
- **Honest about scope.** The /docs page says explicitly: "Cairn is not a replacement for enterprise MBSE tools like Cameo, DOORS, or Rhapsody." Bounding their own claim is rare and trust-building.

Worth absorbing: the **single-example discipline**. Our spec uses many small abstract examples. A single end-to-end worked example (the bootstrap fixture is a candidate) threaded through concepts, blueprint authoring, and query docs would make our newcomer surface more learnable. The prior stronghold flagged this as a borrow target; the screenshot evidence reinforces it.

## Microcopy patterns

A few specific text patterns recur and are worth naming:

- **`· ai ·` token.** Inline next to AI-generated values in property rows. Tiny, low-emphasis, but tells the user every time which numbers came from where.
- **"Suggest Trace Links" versus "Add Link".** Two buttons sit side by side; the wording makes it obvious that one is human-deterministic and the other is AI-probabilistic.
- **"Engineering your changes... (4.1s)"** plus **"typically 30-60s"**. The progress copy carries an elapsed counter and an honest expectation range. When the run goes long, the copy does not change ("typically 30-60s" stays visible past 60s); the user sees real numbers instead of a fade.
- **"the model knows what it is, but not what it does"**. The Completeness banner translates a numeric finding into a one-sentence diagnostic. Quotable, memorable, useful for any spec linter UI.
- **"AI proposes. You commit."** The AI Governance docs page summarises the entire interaction model in four words. Their thesis fits on a single line: "The model is the artefact, not the conversation."
- **"Apply with Warnings"**. The button label mutates when warnings are present. The user accepts the warnings as part of applying. A quiet, honest UX choice.
- **Section captions in italic on the right.** Section headers like `INSTRUMENTS & CONNECTIONS` carry a right-aligned italic gloss ("interfaces, protocols, and data channels"). The formal name plus the layperson definition co-occur. Useful when audience breadth is wide.
- **Pedagogical book quotes.** Side rails on Causality and Completeness include curated quotations (Harney's `Technology Evaluation` Ch. 1; Pace's chapter in Loper 2015) that teach the user how to interpret the metric. The quote is not decoration; it argues the case for the metric on the same screen as the metric.

The microcopy patterns we should absorb most: **the `· ai ·` token, the honest progress copy, and the per-finding prose nudge**. All three are small, all three are about transparency, all three compose with our existing voice.

## Things to explicitly not copy

Ranked rough by harm.

1. **Their fixed twelve-lens taxonomy as a UI primitive.** Their docs name twelve lenses; the running app shows ten tabs. The taxonomy is product-specific and closed. Our neighbourhood and query system is programmatic and extensible. Adopting a fixed lens set would constrain our query surface for no gain. (Already flagged as Skip in [08-borrow-list.md](./08-borrow-list.md).)

2. **Their hardware-flavoured property defaults (Mass Budget, Power Budget, Sea State).** The candidate-property chips on a node detail panel are right for marine robotics and wrong for code or non-code domains. The pattern (chips on the node, AI-staged candidates, one-click adopt) is borrowable; the specific candidates are not.

3. **The "Quick" versus "Professional" export framing.** The wording is marketing language layered on a technical distinction (sync local versus async LLM). The pattern is borrowable; the words "Quick" and "Professional" carry product-tier connotations that do not match our framework.

4. **The `· ai ·` token without an authored counterpart.** Adopting AI tagging on values without also tagging authored values risks giving the user a binary read of "AI versus blank", where blank reads as authored by default. We need to surface both states explicitly. Tag both.

5. **Hardcoded font choices in any new UI work.** Our design system carries Source Serif 4, IBM Plex Sans, and IBM Plex Mono as authority. Whatever cues we absorb from getcairn.dev's typography, the typefaces themselves come from our tokens.

6. **Marketing-language fragment headings.** Their front page uses fragment headings like "Describe", "Decompose", "Inspect", "Refine" that read as marketing rhythm rather than as documentation structure. Useful on a landing page, fragile in an explainer doc. We use full headings already; keep that.

## Status bar as model summary

One pattern worth singling out: their status bar (screenshot 11) reads like a machine-counter row. Verbatim:

```
Model loaded   Offshore Survey USV-ROV   v0.1.0
|  7 nodes   0 reqs   2 interfaces   0 states
|  1 file   Quality: 73   1 pending review   ⌘K to command
```

Every primary count appears in one row: nodes, requirements, interfaces, state machines, attached files, quality score, pending review counter. The row is always visible. The user can answer "how big is my model?" without leaving the active screen.

We should consider an analogue for any future cflx webui or interactive surface. The pattern is portable: pick the smallest set of running counters that sum the model's state, render them in mono, keep them visible. Our equivalents would be artefacts by type, contracts, ghost / synced / orphaned counts, plus a pending-review chip when applicable.

## Empty-state design

The captured empty subsystem detail page (screenshot 08) shows a single primary CTA pointing at the command palette:

```
Start decomposing Launch & Recovery System
Break this node into subsystems, add requirements, or define interfaces.

  Open Command Palette
```

No multiple options, no decision tree, no "Get started" wizard. Just the next action. The empty state assumes the user wants to do *something* and offers exactly one path forward. This is a strong UX choice that scales: every empty state in the app funnels to the palette, the palette is context-aware, the user learns one entrypoint.

We do not have a single canonical entrypoint analogous to the palette. For new surfaces we build, the lesson is: **pick the one action and offer it; do not branch**. Branches in empty states create decision fatigue at the worst moment.
