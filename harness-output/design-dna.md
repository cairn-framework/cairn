# Design DNA — Calibrated Instrument

## Name and essence

**Calibrated Instrument.** The cairn graph explorer is a precision readout for architecture: a warm graphite chassis, one calibrated signal colour, and dense typographic plates. It reads the truth of a codebase the way a flight deck reads an engine: quiet, exact, silent until something drifts.

## Principles

1. The instrument is calm by default; drift is the only alarm. A clean map must look serene.
2. Density is a virtue. Maintainers want readouts, not whitespace theatre.
3. Machine identity and human names are typographically distinct (mono for hashes, ids, paths; grotesque for names).
4. Every state carried by colour is also carried by shape (keel, outline style, tilt).
5. Selection focuses; it never hides. De-emphasis dims to readable, never to broken.
6. Chrome frames the canvas; it never competes with it.

## Aesthetic and creative tension

A precision instrument WITH warmth. Rams-era restraint executed in warm graphite (never blue-black), soft chalk ink (never pure white), and a single calibrated teal-green signal. The tension is resolved by material warmth plus typographic craft instead of decoration.

## Colour language

- **Chassis steps** (three close warm greys): bezel, deck, canvas well. Depth comes from these steps, not shadows.
- **Ink**: chalk and two dimmer steps. Never pure white or black.
- **Signal (teal-green)**: interaction, selection, the synced state family. One hue, two strengths.
- **Alarm (instrument amber)**: exclusively drift (ghost, orphaned, warnings). If amber is visible, something needs attention.
- Nothing else may carry hue. Info-severity findings are chalk with an outlined badge.

## Typography

- UI and node names: an engineering grotesque (Helvetica Neue stack in the mock), medium weight for identity.
- Hashes, node ids, paths: monospace, one size step down, dimmer ink.
- Micro-labels: uppercase, 9.5px, wide tracking (0.2em+), faint ink.
- Tabular figures everywhere a number appears; counters use number-over-label instrument gauge form.

## Spatial rhythm

Fine baseline discipline: hairline rules group readout rows; breathing space, not boxes, separates groups. The bezel, rail, deck grid (canvas roughly two thirds, inspector one third), and findings channel are fixed horizontal bands. Margin calibration ticks, never a full grid mesh.

## Signature motifs

- **State keel**: a 3px left edge on every node module; solid signal = synced, hollow or double = ghost, amber plus label tilt = orphaned (tilt borrowed from the Strata Survey candidate, whose state vocabulary scored best).
- **Annunciator lamp**: a single rectangular status chip in the bezel; SYNCED in signal, DRIFT in amber.
- **Container brackets**: thin brackets group a container's modules on the canvas instead of nested boxes.

## Motion

State changes snap with a short exponential ease-out. The annunciator changes by a single luminance step. No bounce, no parallax, no elastic easing. Respect prefers-reduced-motion; every animation has a static equivalent.

## Voice and tone

Plain language, British spelling, no em-dashes. Labels are nouns (Registers, Dependencies, Decision lineage). Empty states state the fact and the next command (from copy.json), never apologise.

## Applying the system

- New surfaces start from the chassis steps and the two accent roles; add tokens before adding values.
- Any list of facts becomes a ruled key-value plate (kv table), right-aligned values, tabular numerals.
- Any new state must define its keel treatment and its non-colour channel before it ships.
- Dependency direction is always labelled (OUT and IN tags), not implied.

## Anti-goals

- No glassmorphism, glow borders, gradient text, purple-cyan gradients.
- No pure #000 or #fff; no blue-black greys.
- No identical card grids or cards nested in cards.
- No hero-metric dashboard layouts.
- No second accent hue; amber is an alarm, not a brand colour.

## Provenance

Codified 2026-07-12 from the design-studio greenfield run in cairn-ds-greenfield (branch design-studio-greenfield). Two directions were built and evaluated against frozen real fixtures (map.json, 25 nodes, 27 edges): Strata Survey (geological metaphor, final wA 7.8) and Calibrated Instrument (final wA 8.0, winner). Iteration history and per-zone scores: harness-output/scores.json, critique-1..3.md. Screenshots captured in headless Chrome at 1440 and 390.
