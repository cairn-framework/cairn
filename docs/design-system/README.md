# Cairn Design System

Canonical UI reference for Cairn. The five files in this directory are the authority: tokens, fonts, components, a live showcase, and this README. Every surface in Cairn (marketing site, Rust web UI, future consoles) grounds on these tokens. If a color, size, or duration needs to change, it changes here first.

The metaphor is geological: stone and paper, weight you can read, strata that earn their place. The taxonomy is load-bearing: blueprint, map, provenance chain, authority chain, hinge. These words appear verbatim in the UI and in this design system; they are not decoration.

## File structure and load order

```
docs/design-system/
  fonts.css        Google Fonts imports (Source Serif 4, IBM Plex Sans, IBM Plex Mono)
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

## Token naming conventions

| Prefix        | Meaning                                                             |
|---------------|---------------------------------------------------------------------|
| `--stone-*`   | Layered surfaces (0 bedrock, 5 peak)                                |
| `--paper-*`   | Inverted paper tones for callouts                                   |
| `--seam-*`    | Solid strata lines between stones                                   |
| `--stroke-*`  | Alpha strokes that sit on any surface                               |
| `--ink-*`     | Text colors (char, aged, faded, mist, ghost)                        |
| `--prov-*`    | Provenance chain accent (evidence in)                               |
| `--auth-*`    | Authority chain accent (rules out)                                  |
| `--hinge-*`   | The decision hinge where chains meet                                |
| `--drift`     | Advisory tension signal                                             |
| `--block`     | Blocking contradiction signal                                       |
| `--settled`   | Reconciled signal                                                   |
| `--synced`    | Reconciliation state: declaration matches source reality            |
| `--ghost`     | Reconciliation state: declared path or target is absent             |
| `--orphaned`  | Reconciliation state: source exists but no node owns it             |
| `--t-*`       | Type scale (micro, small, body, lede, title, h3, h2, h1, display)   |
| `--s-*`       | Spacing scale (4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 96 / 128)      |
| `--r-*`       | Radius scale (2 / 6 / 10 / 14 / 9999)                               |
| `--lift-*`    | Drop shadows (resting, lifted, floating)                            |
| `--inset-*`   | Inner highlights and wells                                          |
| `--dur-*`     | Motion durations (tick, quick, settle, reveal, breathe, build)      |
| `--ease-*`    | Motion easings (settle, stack, lift, paper)                         |
| `--font-*`    | Font families (serif, sans, mono)                                   |

Numeric aliases (`--ink-1` through `--ink-4`, `--fast`, `--med`, `--slow`) remain for legacy shell code. New code should prefer the named tokens.

## When to update each file

| Change                                           | File to touch                      |
|--------------------------------------------------|------------------------------------|
| New color, spacing, radius, duration, easing     | `tokens.css` (add, never remove)   |
| New font family or new weight                    | `fonts.css`                        |
| New component or variant                         | `components.css`                   |
| Showing a new component in the reference         | `index.html`                       |
| User-facing CLI or UI strings                    | `copy.toml` (verbal authority)     |
| Documentation, consumer instructions, conventions | `README.md` (this file)           |

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

Pre-rename names from earlier spec revisions (see the phase 2.6 rename proposal for the full list) must not appear in current-tense UI copy. Archived phases may reference them as historical record. See `/CLAUDE.md` for the full terminology state and the load-bearing vocabulary set (interface hash, rationale tension vs. interface contradiction, ghost / synced / orphaned / drift, neighbourhood, reconciler, hinge, artefact).

## Further reading

- `/docs/spec.md` for the canonical spec (provenance and authority chains, reconciler interface, artefact types)
- `/CLAUDE.md` for the terminology section and repo-level conventions
- `/openspec/changes/phase-2.6-terminology-rename/` for the rename rationale and the rules this design system follows
