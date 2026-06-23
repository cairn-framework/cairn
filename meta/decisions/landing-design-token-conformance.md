---
id: dec.landing-design-token-conformance
nodes:
  - cairn.ui
status: accepted
date: 2026-06-23
---

# Landing page sources colour from the design system, not a fork

## Context

The marketing landing page (`docs/landing/index.html`) is bound to the same
design-system tokens (`docs/design-system/tokens.css`) as the webui, and
`docs/design-system/README.md` forbids forking the palette. The page instead
defined its own inline copy of every colour token across two theme blocks
(`:root` dark, `[data-theme="light"]`). That fork had drifted: the dark theme
still matched canonical byte-for-byte, but the light theme had diverged on 24 of
the 26 colour tokens the page uses (warmer paper stones, lighter amber and
verdigris accents, lower-alpha washes). CLAUDE.md also asks marketing surfaces to
pull the design system in via `<link>`. The page already referenced every colour
through `var(--token)`; only the definitions were forked.

`dec.webui-design-token-gate` anticipated this: its consequences note that a
token-conformance need for `docs/landing/` extends `scripts/check-design-tokens.sh`
rather than adding a new mechanism.

## Decision

Link the canonical stylesheet from the landing `<head>`
(`<link rel="stylesheet" href="../design-system/tokens.css">`, which resolves
under GitHub Pages because `pages.yml` deploys all of `docs/`) and delete the
inlined colour token definitions from both theme blocks. Keep the page-specific
non-colour tokens inline (type scale, spacing, radii, motion, font stacks): the
landing intentionally runs a larger marketing type scale than the webui, so those
are deliberate divergences, not palette drift.

Extend `scripts/check-design-tokens.sh` to check the landing as a second default
target, stripping HTML comments as well as CSS comments so a hex mentioned in
prose does not trip the gate. Trigger the pre-commit hook on the landing path
too. Pin the result with a real-file regression test in
`tests/check_design_tokens.rs`.

## Rationale

Deleting the fork is load-bearing, not cosmetic: the inline `<style>` block
follows the linked stylesheet in source order, so while the page redefined the
colour tokens inline they overrode canonical and the fork persisted. Removing
them lets canonical win, while the kept inline non-colour `:root` tokens still
override canonical for type and motion (as intended).

The dark (default) view is byte-identical: all 26 colour tokens the page uses had
dark values equal to canonical, so computed styles do not change. The light view
reconciles to canonical, a deliberate, maintainer-approved change to the live
marketing site (its accents and paper tones shift toward the design system). The
loop escalated this through AskUserQuestion before shipping, because `pages.yml`
auto-deploys `docs/` on merge: it is a real outward change to a public page, the
maintainer's call, and the maintainer gave the GO.

## Consequences

- The landing can no longer drift from the design system on colour: a forked or
  hardcoded hex now blocks commit, push, and CI, reported with file and line.
- The light marketing theme now matches the canonical paper palette; future
  palette edits propagate to the landing automatically through the link.
- Page-specific type, spacing, and motion tokens remain a sanctioned divergence
  and stay out of the gate's scope (it checks colour and rem only).
- A pre-existing defect is now more visible but out of this unit's scope: the
  hero image (`docs/images/webui-v2-empty.png`, referenced at
  `docs/landing/index.html:877`) is missing, and the og/twitter images still
  point at the old `dev` branch. Tracked as a separate follow-up.
