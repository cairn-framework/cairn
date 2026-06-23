---
id: dec.webui-a11y-static-audit-gate
nodes:
  - cairn.ui
status: accepted
date: 2026-06-23
---

# Webui accessibility static-audit gate

## Context

The hand-authored web surfaces (the webui shell `src/ui_assets/index.html`, the
preact app `src/ui_assets/app.js`, and the landing page
`docs/landing/index.html`) already practise good accessibility hygiene: the app
labels its icon-only buttons with `aria-label`, marks the inspector
`aria-live`, and groups the zoom controls with `role="group"`. Both document
shells declare `lang`, a `<title>`, and a zoom-friendly viewport, and the one
landing `<img>` carries an `alt`. Nothing enforced any of this. biome (added by
cairn-xiw) lints `app.js` with its recommended rule set, which has no
accessibility rule and cannot see markup inside `htm` template literals at all,
and it does not lint the `.html` surfaces. A regression (an uncaptioned image,
a positive tabindex, a dropped `lang`, a zoom-disabling viewport) would pass
every gate.

This is the deterministic half of cairn-y7p. The bead's larger scope, a
browser -> AI-vision-critique -> patch loop, stays blocked on two maintainer
prerequisites (a Node/Playwright toolchain in this package.json-less Rust repo,
and a paid AI vision provider that conflicts with the repo's deterministic-gates
convention). A static a11y audit needs neither and is the natural next unit.

## Decision

Add a standalone repository gate, `scripts/check-a11y.sh`, that fails when a web
surface violates a statically decidable, WCAG-aligned accessibility invariant.
Element-level checks run on every surface; document-level checks run only on full
HTML documents (those with an `<html>` root), so JS/htm fragments are exempt from
them:

- WCAG 1.1.1: every `<img>` carries an `alt` attribute (tag-aware, so a
  multi-line `<img>` is judged as one tag).
- WCAG 2.4.3: no positive `tabindex` (1+) overrides the natural focus order
  (`0` and `-1` are allowed).
- WCAG 3.1.1: a document's `<html>` declares a `lang`.
- WCAG 2.4.2: a document has a `<title>`.
- WCAG 1.4.4: a document's viewport meta does not disable pinch zoom
  (`user-scalable=no` or `maximum-scale=1`).

HTML and block comments are stripped first so markup mentioned in prose does not
trip the gate. Wire it into the same three places that already gate the webui:
the pre-commit config, the CI `webui` job, and the Makefile `check` target.
Cover its behaviour with `tests/check_a11y.rs`.

## Rationale

This mirrors `dec.webui-design-token-gate` exactly: a project-health gate that
lives alongside `scripts/check-design-tokens.sh` and `scripts/check-file-sizes.sh`,
not a cairn feature. It deliberately stays outside cairn's kernel: per
`dec.toolchain-lint-strictness`, cairn inspects lint *configuration existence*
and never invokes a linter. A repo script is the right home for a specific
accessibility rule, and it is config-free and dependency-free (POSIX sh + awk +
grep), keeping the repo's low-dependency, single-binary ethos intact.

The checks are limited to invariants that are genuinely decidable from source
text without rendering. Colour contrast, dynamic ARIA state, and focus-trap
behaviour need a real DOM and a headless browser, which is precisely the blocked
half of cairn-y7p; this gate does not attempt them.

## Consequences

- An uncaptioned image, a positive tabindex, a missing `lang` or `<title>`, or a
  zoom-disabling viewport on a gated surface now blocks commit, push, and CI,
  with the offending file and rule reported.
- A future surface (a new `.html` page, a new asset) is added to the default
  target list in the script, or scanned in isolation via `CAIRN_A11Y_TARGET`.
- The remaining cairn-y7p scope is now exactly the AI-vision loop, still blocked
  on the two maintainer prerequisites recorded on the bead.
