---
id: res.console-contrast-composited-measurement
nodes:
  - cairn.ui
date: 2026-08-10
method: primary
---

# What the composited contrast measurement exposed on the canvas

Recorded because `todo.console-contrast-honesty` predicted one defect and the
honest measurement found three, and because fixing them ran into a ceiling in
the ink palette that constrains the sibling unit
`todo.console-state-grammar`.

## Method

`harness/lib/audit.mjs` was changed to multiply the ancestor `opacity` chain and
composite the foreground over the resolved backdrop before computing the ratio,
in both the DOM pass and the SVG text pass. The visual harness
(`node harness/eval.mjs`, 16 scenarios, headless Chrome, frozen fixtures) was
then run against the unchanged stylesheet, so the numbers below are the shipped
surface measured honestly rather than a prediction.

## Result: one predicted defect, three real ones

`ux_defect_score` moved from 0 to 51 on the unchanged stylesheet:
17 contrast violations across the 5 scenarios that have a selection, which are
the only scenarios where `.node-shell.dimmed` exists. Three distinct
signatures, all of them node text under the shell's `opacity: 0.55`:

| Text | Declared colour | Undimmed | Composited |
|---|---|---|---|
| `.node-name` | `--ink-char` | 10.86:1 | 4.42:1 |
| `.node-description` | `--ink-aged` | 5.22:1 | 2.61:1 |
| `.node-id` | `--ink-faded` | 5.09:1 | 2.57:1 |

The parent todo predicted the first row at 4.42:1; the harness reproduces it
after flattening both foreground and card background through the shell onto the
`--stone-2` canvas, which is how CSS group opacity paints the pixels.
The two secondary rows are the larger failures: they were already at
the AA floor before the fade, so dimming pushed them furthest below it. Any
plan that treats the dimming defect as a single heading is understating it.

## Constraint: the dark surface has two AA-passing ink steps, not four

Against `--stone-3` (`#2e2d29`), the ink ramp measures:

| Token | Ratio | Passes 4.5:1 |
|---|---|---|
| `--ink-char` | 10.86:1 | yes |
| `--ink-aged` | 5.22:1 | yes |
| `--ink-faded` | 5.09:1 | yes |
| `--ink-mist` | 1.77:1 | no |
| `--ink-ghost` | 1.37:1 | no |

`--ink-aged` and `--ink-faded` are `#a49f92` and `#a29d90`: adjacent in the
token list, indistinguishable on screen, and identical for this purpose. So the
ramp offers exactly two usable emphasis levels above the AA floor, not four,
and "recess it by an ink step" can only mean `--ink-char` to `--ink-aged`.
Dimming is implemented that way, with the card's lift removed
(`box-shadow: none`) so the recession still reads on a node whose only text is
its id.

This is the constraint `todo.console-state-grammar` inherits. That unit has to
make every state pair distinguishable in greyscale, and it cannot spend ink
steps to do it: below `--ink-aged` the ramp stops being legible, so the
separation has to come from a non-text channel (border, keel, dot, shape) or
from new tokens authored against a measured ratio.

## Result after removing ancestor opacity

The same 16-scenario harness returned `ux_defect_score=0`,
`contrast_violations=0`, and `svg_contrast=0` after both stylesheet layers
replaced shell opacity with `--ink-aged` text and loss of lift. Selected-state
scenarios also assert that a dimmed node's computed colour resolves to
`--ink-aged`, differs from the selected node, and measures at least 4.5:1
against its rendered background. The zero therefore cannot be obtained by
deleting the dimmed treatment.

## Limits

Measured on the dark theme only, at the fixed harness viewports, against frozen
fixtures. The light theme is not audited by any gate: there `--ink-faded`
measures 2.76:1 on `--stone-3`, so the `.node-id` colour fails AA today,
independent of dimming. That is pre-existing and outside this unit; it is
recorded here because the next unit to touch these tokens will meet it.
