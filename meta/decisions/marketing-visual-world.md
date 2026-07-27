---
id: dec.marketing-visual-world
nodes:
  - cairn.ui
status: accepted
date: 2026-07-27
related:
  - dec.landing-design-token-conformance
  - dec.webui-design-direction
  - dec.webui-design-token-gate
---

# Marketing lane visual world: the airworthiness record

## Context

The marketing surface at `docs/index.html` shared one visual world with the
product web UI: warm dark stone, geological framing, Source Serif 4 paired with
IBM Plex. `dec.webui-design-direction` chose "Calibrated Instrument" for the web
UI and kept the geological vocabulary only as a state motif.
`dec.landing-design-token-conformance` bound the landing page to the canonical
token surface so it could not fork the palette.

Two things forced a rebuild rather than a refinement.

First, the incumbent page proved nothing. Its hero demonstration was a video of
the web UI, and `todo.ui-asset-refresh` records that this asset is stale: it
shows a UI that is itself scheduled for overhaul. A Persuade surface whose only
demonstration is a recording of something being replaced is claiming, not
proving.

Second, the world had converged on the aesthetic that AI-authored interfaces
land on by default: near-black ground, hairline rules, italic serif display,
small tracked mono labels. It read as the category, not as Cairn.

The maintainer was asked, on 2026-07-27, how far the visual world could move and
answered full replacement: the name Cairn, the stacked-stone mark, and the
load-bearing taxonomy are fixed; palette, typography, and composition are open.
The maintainer also confirmed the web UI needs its own separate overhaul, so the
marketing lane leads and the product lane follows later.

## Evidence

The direction was derived through the Impeccable design skill's new-work
procedure, installed project-locally (git-excluded, not vendored).

Seven candidate worlds were derived from the audience's own documentation and
verification traditions, ordered by resonance: as-built redline, surveyor's level
book, admiralty chart with notices to mariners, ISO normative standard sheet,
airworthiness record, topographic survey sheet, printer's proof-mark errata. The
geological section and the calibrated instrument were excluded as incumbent.

`concept-seed.mjs --scope direction --mode persuade` (key `0a39cd78`) assigned
index 5 of that ordered list: the airworthiness record. Six catalogue
challengers were dealt and weighed on audience identification and product
clarity. All six lost:

- Fillmore handbill, copper lantern souk: no verification grammar, product
  clarity fails.
- Salt-stiffened treasure map: navigation by inference contradicts a map whose
  whole claim is precision.
- Azulejo station hall: one permitted ink cannot encode three distinct finding
  severities.
- Green-phosphor terminal: high audience identification, but it is the category
  rut this rebuild exists to refuse, and monochrome cannot carry severity.
- CRT arcade pixel: wrong register for a merge gate.

## Decision

The marketing lane's visual world is **the airworthiness record**: the paperwork
that makes an aircraft legal to fly.

The fit is structural, not decorative. An aircraft is airworthy because a record
asserts that every fitted part matches the type certificate, and because every
deviation has been raised, classified, and signed for by a named person. Cairn
makes the same assertion about a codebase. The world supplies a grammar Cairn
already has and the previous world did not:

| Cairn concept | Record equivalent |
|---|---|
| `synced` | Serviceable. Green tag, signed. |
| structural error | Unserviceable. The record itself is void, the scan halts. |
| interface contradiction | Unserviceable. Fitted part does not match the record, the merge is grounded. |
| rationale tension | Deferred defect. Amber tag, raised and tracked, does not ground it. |
| `ghost` | Not fitted. Declared on the build sheet, no part installed. |
| `orphaned` | Unlogged part. Fitted to the aircraft, nobody signed for it. |
| drift | A torque stripe whose two halves stopped lining up. |
| decision | Certificate of release to service. Reads the evidence, issues the authority. |

The torque stripe is the signature device. A paint mark is drawn across a
fastener and its housing; if the two halves stop lining up, the fastener moved.
On the page, every declared module carries a stripe across the boundary between
what was declared and what is on disk. Drift is visible before a word is read.

Committed system rules:

- **Colour strategy: committed.** Aviation status colours are codified, so they
  carry whole regions rather than appearing as accents. Serviceable green,
  unserviceable red, deferred amber.
- **Light ground.** Chosen from the use scene, not the category: a licensed
  engineer under hangar lighting, filling in a log on paper. The ground is
  carbonless copy paper. Dark is reserved for instrument insets, which is where
  command output and blueprint source live.
- **Two inks.** Pre-printed form ink states what was declared. Ballpoint blue
  states what was found. The distinction is load-bearing and is used
  consistently.
- **Typography.** Archivo (variable weight and width) for every printed
  element: placards use the condensed width, body uses the normal width. Courier
  Prime for every entry, identifier, path, and command, because a line-printer
  face is what a record is actually set in. No serif appears on the marketing
  lane.
- **Progressive disclosure is the page structure**, not an effect. The page
  reads complete top to bottom at its summary level; the raw record sits one
  deliberate interaction below it. Every block that carries a raw record (real
  command output, real blueprint source, real artefacts) is a native
  `<details>` log entry that reads complete at its summary line. Blocks whose
  whole content is the argument, such as the finding tags and the paper trail,
  stay open: hiding them would hide the point, not defer it. Disclosure is
  never scripted, so the page is complete with JavaScript disabled.
- **One authored motion.** The reconciliation strip settles once on entry:
  aligned stripes hold, the drift row's stripe slips out of alignment, the red
  tag stamps. Everything else is quiet state change.

Honesty rules the page inherits from the product record:

- The synthetic `auth` system used for the worked example is labelled as a
  specimen wherever it appears.
- Cairn's own reconciled numbers are real, regenerated from `cairn context`, and
  labelled as Cairn's own log.
- The stale web UI recording and screenshots are not used.
- Unshipped reconcilers are marked not yet certified.

## Scope of the font change

`AGENTS.md` named Source Serif 4, IBM Plex Sans, and IBM Plex Mono as the font
authority for all UI work. That rule predates the existence of two lanes. It is
now scoped: those three families remain the authority for the product web UI,
and the marketing lane uses Archivo with Courier Prime. `AGENTS.md` and
`docs/design-system/README.md` are updated in the same commit so the declaration
matches the shipped surface. Leaving the rule unamended would be drift of exactly
the kind this project exists to catch.

## Rationale

Building a second design system was rejected.
`dec.landing-design-token-conformance` exists precisely to stop the landing page
forking the palette, and `AGENTS.md` makes `docs/design-system/` the single
authority. The marketing world therefore lands inside the canonical system as an
additive lane: `--mk-*` tokens in `tokens.css`, `.mk-*` components in
`components.css`, both showcased in the design-system reference and documented in
its README. No existing token is renamed or removed and every new selector is
`.mk-*` scoped, so nothing in the lane can change how the web UI renders, and
the web UI can adopt the lane later or not at all.

The lane is not free for the product, and that is accepted rather than hidden.
`src/ui/mod.rs` embeds `tokens.css` and `components.css` wholesale into the
stylesheet the web UI serves, so the marketing lane adds roughly 38 KB
unminified to that response. The alternative, a separately linked marketing
stylesheet, would split the single authority
`dec.landing-design-token-conformance` exists to protect. If the payload ever
matters, the fix is to split the served bundle at build time, not to fork the
system.

The alternative, refining the incumbent page, was rejected on the evidence above:
its proof asset is stale and its aesthetic is the category default. Impeccable's
own guidance is that refinement and redesign must not be split into polish on a
discarded look.

## Consequences

- `docs/index.html` is rebuilt from scratch on `.mk-*` components. Its colour
  values continue to come from `tokens.css`, so
  `dec.landing-design-token-conformance` and `scripts/check-design-tokens.sh`
  continue to hold unchanged.
- `docs/design-system/fonts.css` loads Archivo and Courier Prime in addition to
  the three product families. Marketing pages pay for two extra families; the web
  UI, which inlines tokens and components rather than `fonts.css`, does not.
- The web UI now visibly diverges from the marketing surface. That is accepted
  and time-boxed: `todo.ui-asset-refresh` already owns the product-lane refresh,
  and this decision is the reference it should reconcile against.
- The dark theme toggle is dropped from the marketing lane. The world is a paper
  record with instrument insets; a dark paper record is not a thing, and a toggle
  that inverts it would break the two-ink grammar. The web UI keeps its themes.
- Future marketing surfaces (case pages, docs landing) inherit this lane rather
  than inventing a third world.
