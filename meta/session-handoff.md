# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

- **`cairn-y7p` a11y deterministic slice landed and merged** (PR #148, squash
  merge `b38d8eb`): added `scripts/check-a11y.sh`, a deterministic,
  dependency-free accessibility gate over the hand-authored web surfaces
  (`src/ui_assets/index.html`, `src/ui_assets/app.js`,
  `docs/landing/index.html`), mirroring the accepted `dec.webui-design-token-gate`
  pattern. It sidesteps the "DOM is preact-rendered" objection by checking
  source text (including the `htm` template literals in `app.js`), not a
  rendered DOM, so it is neither fragile nor open-ended.
  - Checks, all statically decidable WCAG criteria the surfaces already satisfy.
    Element-level (every surface): 1.1.1 every `<img>` has `alt` (tag-aware,
    multi-line safe; empty `alt=""` decorative passes), 2.4.3 no positive
    `tabindex`. Document-level (full HTML documents only): 3.1.1 `<html lang>`,
    2.4.2 a `<title>`, 1.4.4 viewport does not disable pinch zoom.
  - Wired into the same three places as the token gate: pre-commit, the CI
    `webui` job, and the Makefile `check` target (+ an `a11y-check` phony).
    Covered by `tests/check_a11y.rs` (table-driven behaviour + real-surface
    conformance). Recorded in `dec.webui-a11y-static-audit-gate`.
  - Pre-submit review caught real bugs before merge: the adversarial reviewer
    subagent found a zoom-regex false-positive (`maximum-scale=1.5` wrongly
    blocked) and a `data-tabindex` false-positive; CodeRabbit found that
    document detection matched any `<html` substring rather than the root tag.
    All three were fixed and pinned with regression tests; CodeRabbit then
    approved.
  - Gates green: `cargo fmt`/`clippy -D warnings`/`test`/`doc -D warnings`,
    `biome check`, `cairn scan` + `cairn hook all` clean. CI `check`, `webui`,
    `hooks`, `dogfood` and CodeRabbit all green (`claude-review` is the known
    non-blocking hang on unprotected `main`).
- **Beads export caught up.** Appended the landing note to `cairn-y7p` and
  regenerated `.beads/issues.jsonl` via `bd export` (one line changed; no
  unexpected beads, no interactions drift).

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on `main`.
- No open PRs.
- Both deterministic halves of `cairn-y7p` are now done: the design-token gate
  (PR #145/#146) and this a11y gate (PR #148). All remaining work is blocked on
  a maintainer decision.

  | Bead | Unit | Why it is blocked |
  | --- | --- | --- |
  | `cairn-y7p` | webui AI fix loop (remainder) | Only the browser -> AI-critique -> patch -> reload loop is left. It needs (a) sanction for a Node + Playwright/puppeteer toolchain in this `package.json`-less Rust repo and (b) an AI vision provider/API (none exists; likely paid; conflicts with the deterministic-gates convention). Both are maintainer calls the bead says cannot be guessed. |
  | `cairn-2z9` | spike: beads as first-class task layer | `in_progress`; its ruling bends the markdown-artefact invariant (`spec.md:11`), a maintainer architectural call. |

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
- The web surfaces' a11y is now pinned by `scripts/check-a11y.sh` +
  `tests/check_a11y.rs`; add new hand-authored surfaces to the script's default
  target list (or scan one in isolation via `CAIRN_A11Y_TARGET`).
