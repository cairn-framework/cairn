# Cairn Design System

Canonical UI reference for Cairn. The five files in this directory are the authority: tokens, fonts, components, a live showcase, and this README. Every surface in Cairn grounds on these tokens. If a color, size, or duration needs to change, it changes here first.

The system runs **two lanes**. They share the scales and the file layout; they do not share faces, ground, or component vocabulary.

| Lane | Surfaces | World | Prefix |
|---|---|---|---|
| Product | webui (`src/ui_assets/`), consoles, anything a user operates | Calibrated instrument. Warm dark stone, geological state vocabulary as motif. `dec.webui-design-authority` | unprefixed and `--ci-*` / `--ui-*` |
| Marketing | `docs/index.html` and every outward page | The airworthiness record: the paperwork that makes an aircraft legal to fly. `dec.marketing-visual-world` | `--mk-*` / `.mk-*` |

The marketing lane is additive. It renames and removes nothing, and every selector it adds is `.mk-*` scoped, so a change to it cannot change how the product lane renders. It is not weightless: `src/ui/mod.rs` embeds `tokens.css` and `components.css` wholesale, so the web UI serves roughly 38 KB it does not use. That is accepted in exchange for one authority (`dec.marketing-visual-world`). Section 23 of `index.html` renders the lane live.

The product lane's metaphor is geological: stone and paper, weight you can read, strata that earn their place. The taxonomy is load-bearing in both lanes: blueprint, map, provenance chain, authority chain, hinge. These words appear verbatim in the UI and in this design system; they are not decoration.

## File structure and load order

```
docs/design-system/
  fonts.css        Google Fonts imports. Product lane: Source Serif 4, IBM Plex Sans, IBM Plex Mono. Marketing lane: Archivo, Courier Prime
  tokens.css       :root custom properties, [data-theme="light"] overrides, reduced-motion
  components.css   Every styled surface, referencing tokens by name
  index.html       Single-page showcase with TOC, swatches, type samples, glossary
  README.md        This file
```

Load order (always):

```html
<link rel="stylesheet" href="/docs/design-system/fonts.css">
<link rel="stylesheet" href="/docs/design-system/tokens.css">
<link rel="stylesheet" href="/docs/design-system/components.css">
```

`fonts.css` is kept separate so consumers can swap to self-hosted files later without touching the rest.

## How to consume

### Marketing site (GitHub Pages, static HTML)

Link the three stylesheets in order. No build step required. The files open directly in a browser.

```html
<link rel="stylesheet" href="docs/design-system/fonts.css">
<link rel="stylesheet" href="docs/design-system/tokens.css">
<link rel="stylesheet" href="docs/design-system/components.css">
```

### Rust web UI (embedded assets)

Inline the token surface at compile time so the binary stays self-contained:

```rust
const DESIGN_TOKENS: &str = include_str!("../../docs/design-system/tokens.css");
const DESIGN_COMPONENTS: &str = include_str!("../../docs/design-system/components.css");
```

Fonts can either be loaded from Google Fonts (quickest) or bundled as static assets served alongside the binary.

### Any other surface

Treat `tokens.css` as the contract. If a component needs a value, it reads `var(--token-name)`. Never copy hex values into components.

## Theme switching

Themes are toggled by setting `data-theme` on the `<html>` element.

```html
<html data-theme="light">  <!-- paper theme -->
<html>                      <!-- default dark stone -->
```

The showcase (`index.html`) persists the choice to `localStorage` under the key `cairn-ds-theme`. Consumers may reuse that key or define their own; the token surface does not care.

Reduced motion is honored automatically: `@media (prefers-reduced-motion: reduce)` zeroes every `--dur-*` token to 0.001ms. Components that reference durations via `var(--dur-*)` or `var(--fast|--med|--slow)` get the behavior for free.

## Do-not-fork rule

Components reference tokens only. Adding a new color, duration, or size means adding a token to `tokens.css` first, then consuming it in `components.css`. Hardcoded hex values in component CSS are a bug.

Verification in CI (or locally):

```
grep -c '#[0-9a-fA-F]\{6\}' docs/design-system/components.css
# must return 0
```

`index.html` may contain hex strings as swatch content (human-readable labels like `#141310` shown next to the chip), never as applied styles. All applied colors in the showcase go through tokens.

## Token reconciliation summary (Stage 2)

The token surface now merges current webui and landing usage with the greenfield seed corpus.

- Preserved every token name currently referenced by `src/ui_assets/style.css` and `docs/index.html`.
- Added component scope tokens for the redesigned regions:
  - `--line-1` and `--line-2`
  - `--ui-shell-max-width`, `--ui-shell-min-height`, `--ui-shell-min-height-mobile`
  - `--ui-workspace-main-fr`, `--ui-workspace-sidebar-fr`, `--ui-workspace-sidebar-min`
  - `--ui-node-min-width`, `--ui-outline-width`, `--ui-outline-offset`, `--ui-orphan-tilt`
- Kept compatibility aliases and legacy aliases for shell and typography usage.
- No required token names used by those consumers were removed.
- Renames introduced: none.
- Removed only the temporary `--ui-shell-breakpoint` in favour of explicit mobile shell height.

Stage 2 validation confirmed no missing token references in combined `src/ui_assets/style.css`, `docs/index.html`, and `docs/design-system/components.css`.

Stage 4 (eval-loop verification) adjustments:

- `--ci-chalk-faint` raised from `#6f6b60` to `#a29d90` at the root scope (the light theme keeps its own override) so faint ink meets the 4.5:1 contrast floor on every chassis surface it appears against.
- Added `--tap-min: 44px`, the minimum touch-target size; `components.css` applies it to interactive controls (query input, chips, actions, rail and channel tabs) at viewports of 900px and below.
- `.query-chip`, `.query-action`, `.edge-row`, `.edge-dir`, and `.edge-target` are component classes in their own right (previously scoped under `.query-rail` and `.node-depth-plate`); modules may render them in any region and they carry explicit surface, ink, and border treatment so no browser default leaks through.
- `.blueprint-plate pre` wraps long source lines (`pre-wrap`) instead of widening the frame.
- `.channel-bar .channel-empty` styles the channel empty state with faint ink.
- Added `--ui-channel-height: 116px`: the fixed height of the bottom channel bar. The shell grid gives the workspace the remaining fraction, the state legend renders as a slim single-row strip, and the channel body scrolls internally so the page frame never grows.
- Evidence-rail body content (artefact chips, source excerpts, `code`/`pre`) wraps inside the rail's bounded well instead of widening the frame; on tap surfaces those chips keep the `--tap-min` minimum.

Layout overhaul (2026-07) adjustments:

- Added `--ui-query-search-max-width: 608px`: the cap on the query rail's search input so it stays readable while the segmented filter groups share the row.
- `--ui-shell-max-width` widened to `min(100%, 1720px)` and `--ui-channel-height` raised to `200px` so the shell fills wide viewports and channel bar items stay readable.
- Added `--ci-ember: #d98a70` and pointed `--error` at it so error and warning severities read as distinct colours (ember vs amber) while staying in the warm drift family; contrast-checked at 4.5:1+ on all chassis surfaces.

## Component class inventory

Canonical webui classes in this stage:

- `.instrument-shell`
- `.status-bezel`
- `.query-rail`
- `.graph-canvas`
- `.node-module` with `.synced`, `.ghost`, `.orphaned`, `.drift`
- `.evidence-rail`
- `.node-depth-plate`
- `.lineage-plate`
- `.blueprint-plate`
- `.channel-bar`
- `.state-legend`

Marketing lane classes:

- `.mk-page` (add `.mk-page--inset` when the lane renders inside another page), `.mk-sheet`
- `.mk-header` / `.mk-header-in` / `.mk-mark` / `.mk-header-doc` / `.mk-nav`
- `.mk-plate`, `.mk-display`, `.mk-lede`, `.mk-prose`, `.mk-note`, `.mk-aside`
- `.mk-label` (pre-printed field caption), `.mk-rec` (what is written into a field), and the ink utilities `.mk-ink-firm` / `.mk-ink-soft`
- `.mk-btn` with `.mk-btn--stamp` and `.mk-btn--quiet`
- `.mk-tag` with `.mk-tag--svc`, `--uns`, `--def`; `.mk-tag-status`, `.mk-tag-title`, `.mk-tag-body`, `.mk-tag-sig`. A modifier sets only `--tag-ink` and `--tag-paper`
- `.mk-chip` with the same four variants
- `.mk-strip`, `.mk-strip-head`, `.mk-strip-cols`, `.mk-strip-foot`, `.mk-row` with `.mk-row--drift`, `--ghost`, `--orphaned`, and `.mk-stripe` (an empty element; the seam is its background and the two painted halves are its pseudo-elements)
- `.mk-entry` / `.mk-entry-summary` / `.mk-entry-no` / `.mk-entry-title` / `.mk-entry-gist` / `.mk-entry-more` / `.mk-entry-body`
- `.mk-panel`, `.mk-panel-head`, `.mk-panel-file`, and the output inks `.mk-o-dim`, `.mk-o-key`, `.mk-o-warn`
- `.mk-block`, `.mk-block-head`, `.mk-block-no`, `.mk-block-title`, `.mk-block-lede`, `.mk-block-body`
- `.mk-ledger` with `.mk-ledger-clear`, `.mk-trail`, `.mk-crs`, `.mk-ratings` / `.mk-rating`
- `.mk-schematic` (structure diagram) with `.mk-schematic-head`, `-title`, `-note`, `-body`, `-foot`; inside the body `.mk-sch-col` with `--l` / `--r`, `.mk-sch-node` with `--linked`, `.mk-sch-bus` with `--l` / `--r`, and `.mk-sch-hub` with `.mk-sch-hub-name` / `.mk-sch-hub-note`. It draws adjacency and kernel membership only. Each column has one spine gathering its node stubs and one rule into the hub. A `--linked` node draws a rule to the node above it in the same column. Edge direction and reason are never drawn, so every real instance must be paired with a record that prints them. Below 900px the hub moves to the top and its connectors are dropped, so the hub note must state the connection count in words. The body carries `role="img"` and an `aria-label` describing the whole graph
- `.mk-copyline` with `.mk-copyline--wrap`
- `.mk-footer`, `.mk-footer-in`, `.mk-footer-nav`, `.mk-footer-brand`, `.mk-kofi`
- Composition: `.mk-hero`, `.mk-stack` with `.mk-stack--tight`, `.mk-tags-grid`, `.mk-columns`
- Social card: `.mk-card`, `.mk-card-top`, `.mk-card-body`, `.mk-card-foot`

## Marketing lane

Authority: `dec.marketing-visual-world`. The world is the airworthiness record, the
paperwork that makes an aircraft legal to fly. It is not a mood. It supplies a
grammar Cairn already has:

| Cairn concept | Record equivalent | Token |
|---|---|---|
| `synced` | Serviceable. Green tag, signed. | `--mk-svc` |
| structural error | Unserviceable. The record is void, the scan halts. | `--mk-uns` |
| interface contradiction | Unserviceable. The fitted part does not match the record, the merge is grounded. | `--mk-uns` |
| rationale tension | Deferred defect. Raised and tracked, does not ground it. | `--mk-def` |
| `ghost` | Not fitted. Declared on the build sheet, no part installed. | dashed, no colour |
| `orphaned` | Unlogged part. Fitted, and nobody signed for it. | `--mk-def` plus tilt |
| drift | A torque stripe whose halves stopped lining up. | `.mk-row--drift` |
| decision | Certificate of release to service. | `.mk-crs` |

### Rules that bind every marketing surface

- **Ground is paper.** `--mk-paper` (top copy), `--mk-paper-2` (ruled band),
  `--mk-paper-3` (deeper tint). Dark is reserved for instrument insets
  (`--mk-panel`), which is where command output and blueprint source live. A dark
  marketing page is not a variant of this lane; it is a different lane.
- **Two inks, and the distinction is load-bearing.** `--mk-ink-print` is
  pre-printed form ink and states what was *declared*. `--mk-ink-pen` is
  ballpoint and states what was *found*. Never use one for the other's job.
- **Colour strategy is committed, not accented.** Status colour is codified, so
  it carries whole regions: a tag, a row, a rating cell. Scattering it as
  highlights breaks the grammar.
- **Type.** `--mk-font-form` is Archivo, at `--mk-wdth-plate` (78) for placards
  and `--mk-wdth-text` (100) for running text. `--mk-font-record` is Courier
  Prime and sets every entry, identifier, path, and command. No serif appears in
  this lane.
- **Disclosure is structure.** A block that carries a raw record (command
  output, source, an artefact) uses `.mk-entry`, a native `<details>` that reads
  complete at its summary line. A block whose whole content is the argument
  stays open. Disclosure is never scripted: every surface must read complete
  with JavaScript disabled.
- **One authored motion.** `.mk-strip` settles once, on arrival: rows land, the
  drift row's stripe slips out of alignment, the red chip stamps. Nothing else on
  the page animates beyond quiet state change. The strip renders its final state
  when scripting is off and when `prefers-reduced-motion: reduce` is set.
- **No colour is hardcoded.** Marketing pages link `tokens.css` and read
  `var(--mk-*)`, exactly as `dec.landing-design-token-conformance` requires.
  `scripts/check-design-tokens.sh` gates it.

### Marketing lane tokens

| Group | Tokens |
|---|---|
| Paper | `--mk-paper`, `--mk-paper-2`, `--mk-paper-3` |
| Ruling | `--mk-rule`, `--mk-rule-soft`, `--mk-rule-hard` |
| Instrument inset | `--mk-panel`, `--mk-panel-2`, `--mk-panel-rule` |
| Inks | `--mk-ink-print`, `--mk-ink-print-2`, `--mk-ink-pen`, `--mk-ink-panel`, `--mk-ink-panel-2` |
| Serviceable | `--mk-svc`, `--mk-svc-deep`, `--mk-svc-tint`, `--mk-svc-lamp` |
| Unserviceable | `--mk-uns`, `--mk-uns-deep`, `--mk-uns-tint` |
| Deferred | `--mk-def`, `--mk-def-deep`, `--mk-def-tint`, `--mk-def-lamp` |
| Type | `--mk-font-form`, `--mk-font-record`, `--mk-wdth-plate`, `--mk-wdth-text`, `--mk-t-micro`, `--mk-t-label`, `--mk-t-small`, `--mk-t-entry`, `--mk-t-body`, `--mk-t-lede`, `--mk-t-h4`, `--mk-t-h2`, `--mk-t-h1`, `--mk-track-plate`, `--mk-track-stamp`, `--mk-track-tight` |
| Elevation | `--mk-lift-sheet`, `--mk-lift-copy`, `--mk-lift-crs`, `--mk-lift-panel` |
| Layout | `--mk-sheet-max`, `--mk-gutter`, `--mk-measure`, `--mk-hole`, `--mk-stripe-w`, `--mk-stripe-slip`, `--mk-stripe-half`, `--mk-stripe-half-narrow` |
| Motion | `--mk-dur-quiet`, `--mk-dur-open`, `--mk-dur-settle`, `--mk-dur-stamp`, `--mk-dwell`, `--mk-ease-settle`, `--mk-ease-stamp` |
| Social card | `--mk-card-w`, `--mk-card-h`, `--mk-t-card` |

The lane shares the canonical `--s-*`, `--r-*`, `--line-*`, and `--tap-min`
primitives with the product lane. It does not share `--dur-*`: its one authored
moment needs its own timings, so it defines `--mk-dur-*`.

Every ink and status pairing clears 4.5:1 against every surface it is permitted
on. The `-lamp` variants exist because status colour needs a lighter value to
clear that floor on `--mk-panel`.

### Social card

`docs/assets/social-card.html` is the source for `docs/assets/social-card.png`,
the `og:image` and `twitter:image` the landing page points at.
`tests/landing_assets.rs` gates that both meta tags exist, ride the Pages
origin, and resolve to a committed file.

The card is built from `.mk-card` and the lane's own components, never from a
product screenshot: the web UI is mid-overhaul (`todo.ui-asset-refresh`), and a
stale screenshot is the defect `dec.marketing-visual-world` exists to fix. Its
specimen strip carries the same synthetic labelling the landing page uses.

Regenerate after any change to the lane or to the source: open
`docs/assets/social-card.html` at exactly 1200 by 630 with no device pixel ratio
scaling, and screenshot it to `docs/assets/social-card.png`.

## Token naming conventions

| Prefix        | Meaning                                                             |
|---------------|---------------------------------------------------------------------|
| `--stone-*`   | Layered surfaces (0 bedrock, 5 peak)                                |
| `--paper-*`   | Inverted paper tones for callouts                                   |
| `--seam-*`    | Solid strata lines between stones                                    |
| `--stroke-*`  | Alpha strokes that sit on any surface                                |
| `--ink-*`     | Text colors (char, aged, faded, mist, ghost)                        |
| `--prov-*`    | Provenance chain accent (evidence in)                                |
| `--auth-*`    | Authority chain accent (rules out)                                   |
| `--hinge-*`   | The decision hinge where chains meet                                 |
| `--drift`     | Advisory tension signal                                              |
| `--block`     | Blocking contradiction signal                                        |
| `--settled`   | Reconciled signal                                                   |
| `--synced`    | Reconciliation state: declaration matches source reality              |
| `--ghost`     | Reconciliation state: declared path or target is absent               |
| `--planned`   | Reconciliation state: declared in blueprint, path not yet built       |
| `--orphaned`  | Reconciliation state: source exists but no node owns it               |
| `--line-*`    | Primitive border widths for shell, rail, and node accents              |
| `--ui-*`      | Component layout and state tokens for webui regions                   |
| `--t-*`       | Type scale (micro, small, body, lede, title, h3, h2, h1, display)    |
| `--s-*`       | Spacing scale (4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 96 / 128)     |
| `--r-*`       | Radius scale (2 / 6 / 10 / 14 / 9999)                               |
| `--lift-*`    | Drop shadows (resting, lifted, floating)                             |
| `--inset-*`   | Inner highlights and wells                                          |
| `--dur-*`     | Motion durations (tick, quick, settle, reveal, breathe, build)       |
| `--ease-*`    | Motion easings (settle, stack, lift, paper)                          |
| `--font-*`    | Font families (serif, sans, mono)                                    |
| `--mk-*`      | Marketing lane. See the marketing lane section below                  |

Numeric aliases (`--ink-1` through `--ink-4`, `--fast`, `--med`, `--slow`) remain for legacy shell code. New code should prefer the named tokens.

## When to update each file

| Change                                           | File to touch                      |
|--------------------------------------------------|------------------------------------|
| New color, spacing, radius, duration, easing     | `tokens.css` (add, never remove)   |
| New font family or new weight                    | `fonts.css`                        |
| New component or variant                         | `components.css`                   |
| Showing a new component in the reference         | `index.html`                       |
| User-facing CLI or UI strings                    | `copy.toml` (verbal authority)     |
| Documentation, consumer instructions, conventions | `README.md` (this file)            |

Breaking changes to token names require a coordinated sweep. The safer pattern: add the new name as an alias, migrate consumers, remove the old name later.

## Voice

CAIRN's audience spans career developers and people building with AI tools (including non-devs). User-facing vocabulary should prefer plain, concise English. Accuracy is the floor: do not flatten load-bearing technical taxonomy.

The bar: "would a non-dev feel nervous typing this command or reading this doc?" Not: "what's the simplest possible word."

### Rules

- No em-dashes (U+2014) in any user-facing text. Use period, colon, comma, or parenthesis.
- One idea per sentence. Short sentences over compound clauses.
- Commands appear in backtick code spans (`cairn scan`), not quotes.
- Preserve technical terms that carry distinct meaning: blueprint, map, neighbourhood, reconciler, provenance chain, authority chain, hinge, artefact, drift, ghost, orphaned, synced.
- Placeholders use `{name}` syntax in copy.toml; rendered with the actual value at display time.
- Headings use sentence case, not title case.

### Review checklist

- [ ] No em-dashes anywhere in the change.
- [ ] Every user-facing string lives in `copy.toml`, not inline in source.
- [ ] Placeholder names match the data field they substitute (e.g. `{node}` for node ID, `{path}` for file path).
- [ ] Plain English: would a first-time user understand the message without reading the spec?
- [ ] Technical terms from the load-bearing set are used precisely (not paraphrased or simplified).
- [ ] CTA (call to action) tells the user what to do next, not just what went wrong.

### Tone registers

Four emotional registers are defined as components (see `components.css` tone cards):

| Register | When | Colour token |
|----------|------|--------------|
| Arrival | First load, archive complete, new view | `--prov-2` (amber) |
| Clarity | Query result, chain trace, contract open | `--auth-2` (verdigris) |
| Reassurance | Lint clean, drift resolved, scan settles | `--settled` (moss) |
| Unease | Drift detected, orphan surfaced, cycle found | `--drift` (clay) |

## Terminology

This design system uses the phase 2.6 terminology:

- `blueprint` (the declarative file), `.blueprint` (the extension)
- `map` (the reconciled view), `map.md` (the generated snapshot)

Pre-rename names from earlier spec revisions (see the phase 2.6 rename proposal for the full list) must not appear in current-tense UI copy. Archived phases may reference them as historical record. See `/AGENTS.md` for the full terminology state and the load-bearing vocabulary set (interface hash, rationale tension vs. interface contradiction, ghost / synced / orphaned / drift, neighbourhood, reconciler, hinge, artefact).

## Further reading

- `/docs/spec.md` for the canonical spec (provenance and authority chains, reconciler interface, artefact types)
- `/AGENTS.md` for the terminology section and repo-level conventions
- `/openspec/changes/phase-2.6-terminology-rename/` for the rename rationale and the rules this design system follows

## Motion section update

`components.css` now ends with a dedicated restrained motion section.
It animates these selectors:

- `.node-module` and `.node-shell`
- `.dependency-link path`
- `.evidence-rail .rail-body > *`
- `.channel-bar .channel-body`
- `.query-chip`, `.query-action`, `.rail-tab`, `.channel-tab`

The following tokens were added in `tokens.css`:

- `--motion-fast`
- `--motion-edge`
- `--motion-panel`
- `--motion-subtle`
- `--motion-ease`

Reduced-motion mode has explicit zero-motion overrides.
Default mode uses calm, restrained transitions with no scale jumps.

## 2026-08-02: over-harness console lanes

- Added `.console-workspace` and `.console-lane` (with `-head`, `-title`, `-source`, `-body`) in `components.css`: the read-only over-harness console composition (signature queue, frontier, work DAG) inside the bounded workspace. No new tokens; lane bodies scroll internally per the bounded-workspace rule. Reference markup in `index.html` section 22.
