---
id: dec.webui-ai-vision-loop-declined
nodes:
  - cairn.ui
status: accepted
date: 2026-06-23
---

# Webui AI-vision iteration loop declined

## Context

cairn-y7p proposed an iterative testing loop for the cairn webui: drive a
headless browser to screenshot the rendered UI, feed the image to an AI vision
model, apply its fix instructions to `src/ui_assets/`, reload, and repeat until
the UI meets acceptance criteria.

The bead's deterministic, decidable-from-source half shipped in two prior units
and is now enforced on every commit, push, and CI run:

- `dec.webui-design-token-gate` (PR #145): `scripts/check-design-tokens.sh`
  blocks hardcoded hex/rem in `src/ui_assets/style.css`.
- `dec.webui-a11y-static-audit-gate` (PR #148): `scripts/check-a11y.sh` blocks
  WCAG img-alt, positive-tabindex, lang, title, and zoom-disable regressions on
  the hand-authored web surfaces.

What remained was only the browser -> AI-vision-critique -> patch -> reload loop.
That remainder cannot be built without two prerequisites that are the
maintainer's to grant, not the loop's to guess:

1. Sanction to add a Node + Playwright/puppeteer toolchain to this deliberately
   `package.json`-less, single-binary Rust repo (adds `node_modules` and CI
   cost).
2. An AI vision provider and API config. None exists, it is likely paid, and a
   non-deterministic model inside a gate conflicts directly with the repo's
   deterministic-gates convention (`dec.toolchain-lint-strictness`, and the
   local-hooks-over-paid-CI posture).

## Decision

Decline the AI-vision loop and close cairn-y7p as resolved. The two deterministic
gates are the durable, in-convention outcome of the bead; the vision loop is out
of scope for a deterministic, low-dependency Rust repo and will not be built
absent an explicit maintainer reversal of both prerequisites above.

## Rationale

A non-deterministic, paid, network-dependent vision model gating UI changes is
the opposite of how every other cairn gate works (config-existence checks, static
source analysis, no linter invocation, no network). Adopting it would erode the
single-binary, deterministic ethos the project has repeatedly chosen. The
deterministic gates already catch the regressions that matter and are decidable
from source (design-token drift, the a11y invariants), so the bead's stated
acceptance criteria are substantially met without the vision component.

This record exists so the dev loop does not re-derive the same blocked seed each
session: the question was raised, escalated to the maintainer, and answered.

## Consequences

- cairn-y7p is closed. The webui's enforced UI-quality surface is exactly the two
  deterministic gates; there is no browser or AI tooling in the repo.
- Reopening the vision loop requires a superseding decision that records the
  maintainer's sanction for both a Node/Playwright toolchain and a vision
  provider.
- A future deterministic visual-regression approach (for example a pixel-diff
  against a checked-in baseline rendered by an already-present tool) is not
  precluded; it would be a new unit of work, not this one.
