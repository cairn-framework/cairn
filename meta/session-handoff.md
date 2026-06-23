# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

- **Deterministic slice of `cairn-y7p` shipped** (PR #145, merge `82932f9`):
  landed `dec.webui-design-token-gate` plus `scripts/check-design-tokens.sh`.
  - The webui stylesheet rule ("source every colour / rem size from a
    `var(--token)`, never a literal") was stated in CLAUDE.md, AGENTS.md, and the
    stylesheet header but enforced by nothing (biome's recommended rules cannot
    express it). The new gate strips CSS comments and `url(...)` refs, then fails
    on a hardcoded hex colour (exact 3/4/6/8-digit, identifier-bounded) or a rem
    value (incl. leading-decimal `.5rem` and negative `-1.5rem`).
  - Wired into the three places that already gate the webui asset: pre-commit,
    the CI `webui` job, and the Makefile (`check` + new `tokens-check`). Behaviour
    pinned by `tests/check_design_tokens.rs`.
  - Deliberately a repo script, not a cairn feature: per
    `dec.toolchain-lint-strictness`, cairn inspects lint *config existence* and
    never invokes a linter. Mirrors `scripts/check-file-sizes.sh`.
  - CodeRabbit review (1 actionable + 1 nitpick: rem detector needed a left
    boundary) addressed in `c7ac66a`; stale review dismissed; CI green
    (`check`/`dogfood`/`hooks`/`webui`/CodeRabbit all pass; `claude-review` is the
    known non-blocking hang on unprotected `main`).
- **`cairn-81c` filed then re-diagnosed** (see below). Net: no code change, an
  accurate diagnosis and a ready-to-execute plan recorded in the bead.

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on `main`.
- No open PRs. No P0/P1/P2.
- Backlog (all remaining work is blocked on a maintainer decision):

  | Bead | Unit | Why it is blocked |
  | --- | --- | --- |
  | `cairn-y7p` | webui AI fix loop (remainder) | deterministic slice now landed; the remaining browser->AI-critique->patch loop still needs (a) sanction for a Node + Playwright/puppeteer toolchain in this `package.json`-less Rust repo and (b) an AI vision provider/API (none exists; likely paid; conflicts with the deterministic-gates convention). The a11y static-audit half of the deterministic slice is still open as a smaller unit. |
  | `cairn-2z9` | spike: beads as first-class task layer | `in_progress`; its ruling bends the markdown-artefact invariant (spec.md:11), a maintainer architectural call. |
  | `cairn-81c` | landing page design-token conformance | re-diagnosed (below). The fix is scoped and ready but changes the PUBLIC auto-deployed marketing site, so it needs a maintainer GO. |

## cairn-81c: re-diagnosed, ready for a GO

The "52 hardcoded hex" premise was inaccurate. All 52 hex in
`docs/landing/index.html` are inline CSS custom-property *definitions* across two
theme blocks (`:root` dark, `[data-theme="light"]`); the page has ZERO hardcoded
colour usages and already references `var(--token)` everywhere.

The real issue: the landing *forks* the design-system palette inline (against the
`docs/design-system/README.md` do-not-fork rule and CLAUDE.md "marketing via
`<link>`"), and the fork has drifted. Dark theme: 26/26 tokens match canonical
exactly. Light theme: 21/26 drifted (small warm-tone deltas, e.g. `--stone-5`
`#ffffff` vs canonical `#fcf8ed`).

Scoped fix (in the bead notes): add `<link rel="stylesheet"
href="../design-system/tokens.css">` (Pages serves all of `docs/`, so the
relative path resolves); delete the inlined colour token defs from both theme
blocks; keep page-specific non-colour tokens (`--t-*`, `--s-*`,
`--r-edge/stone/round`, `--ease/--fast/--med/--slow`, `--font-*`, the rgba
`--*-wash`). Do not link `components.css` (bespoke page, collision risk) or
`fonts.css` (page already links Google Fonts). Effect: default dark view
pixel-identical; light view reconciles to canonical. The page then has zero
hardcoded hex and can be added to `scripts/check-design-tokens.sh`.

Why it needs a GO: `pages.yml` deploys `docs/` on merge to `main`, so this is a
real outward change to the live marketing page (the light theme shifts). The loop
stopped here rather than push a guessed visual change to a public site.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
- The token gate runs on any CSS via `CAIRN_DESIGN_TOKENS_TARGET=<file> sh scripts/check-design-tokens.sh`.
