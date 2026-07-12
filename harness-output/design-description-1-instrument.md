# Design description, iteration 1 — Direction B: Calibrated Instrument (refined technical)

Rejected sibling concepts (one line each):
- "Terminal noir": all-monospace dark hacker console; rejected as the generic dev-dashboard slop pole.
- "Blueprint cyanotype": white lines on blueprint blue; rejected as a costume that fights legibility at density.

## Concept

The explorer is a precision instrument in the Rams tradition: a calm, warm-grey chassis with one calibrated signal colour. It reads like a flight-management readout for architecture: dense, exact, silent until something drifts. Warmth comes from material greys (slightly warm, never blue-black), generous optical alignment, and typographic craft, not from decoration.

## Layout geometry

A slim instrument bezel across the top: product mark left; centre shows a live reconciliation readout (nodes, edges, findings, interface hash) as a row of labelled counters, like an engine display; right holds the clean-state annunciator, a small rectangular lamp that reads SYNCED and would read DRIFT in alarm conditions. The main deck: graph canvas as the primary instrument window, left, roughly two thirds; inspector as a readout column, right. The command surface is a thin rail directly under the bezel: a search field with a keyboard hint, kind filters as flat toggle switches, zoom controls. A findings channel docks at the bottom of the canvas, one line tall when clean, expanding when populated.

## Graph canvas as instrument window

The canvas ground is a shade deeper than the chassis, with a faint calibration grid of tick marks at the margins only, never a full grid mesh. Nodes are rounded-rectangle modules with a thin state-keel along their left edge; the system node is a wider master module, containers are section headers drawn as bracketed frames grouping their modules. Edges are hairline vectors with small directional chevrons at midpoint; edge descriptions surface on the inspector, not as canvas clutter. Selection brings a fine focus ring plus dimming of non-neighbours, like selecting a channel on a mixer. States: synced keels are the signal green-grey; ghost modules render as outline-only with a hollow keel; orphaned modules get an amber keel and an off-grid tilt of their label tag. A compact legend at the canvas corner shows the three keels.

## Colour

Warm graphite chassis in three close steps (bezel, deck, canvas well). Ink is soft chalk, never pure white. One signal accent: calibrated teal-green for interaction, selection, and the synced state family. One alarm accent: instrument amber strictly for drift and warnings. Nothing else may carry hue. Info findings are chalk with an outlined badge.

## Typography

A engineering grotesque with real character (narrow, slightly squared) for UI and node names; tabular figures everywhere numbers appear; a monospaced face reserved for hashes, ids, and paths, so machine identity is visually distinct from human names. Uppercase micro-labels with wide tracking for the bezel counters.

## Spatial rhythm

A strict modular rhythm: everything aligns to a fine baseline; readout rows in the inspector are tight, ruled by faint hairlines, grouped by breathing space rather than boxes. Density is a virtue: the inspector should feel like a well-set data plate, not a card stack.

## Motion

Instrument-like: state changes snap with a short exponential ease; the annunciator lamp changes state with a single subtle luminance step. No bounce, no parallax.

## Mood

The user should feel they are reading the truth from a calibrated device: quiet, exact, slightly austere but warm to the touch. Trust through restraint.
