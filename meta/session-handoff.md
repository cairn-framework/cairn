# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

- **`cairn-s2t` landed and merged** (PR #147, squash merge `2a01dae`): fixed
  the landing page's broken image references. `docs/landing/index.html` pointed
  the hero `<img>`, `og:image`, and `twitter:image` at
  `docs/images/webui-v2-empty.png`, a file that was never committed
  (`docs/images/` does not exist), and the social-card URLs rode the retired
  `dev` branch, so the hero rendered as broken alt-text and the social cards
  404'd.
  - Fix: all three now point at `docs/assets/screenshots/webui-graph.png`, a
    committed 1440x900 PNG of the Cairn Graph Explorer webui that matches the
    hero alt text. The hero uses the relative path; `og:image`/`twitter:image`
    use the Pages origin (`https://cairn-framework.github.io/cairn/assets/
    screenshots/webui-graph.png`), consistent with the existing `og:url` and
    correct because `pages.yml` deploys `docs/`. Hero `width`/`height` corrected
    from `2880x1800` to the asset's true intrinsic `1440x900` (same ratio).
  - The "maintainer call" the bead flagged (which screenshot, which URL form)
    was determined by the page's own content: the alt text and figcaption name
    the webui graph explorer (`webui-graph.png`), and `og:url` already fixes the
    Pages origin. The prior session had recorded exactly this candidate fix.
  - New regression test `tests/landing_assets.rs` (under the `cairn.tests` node):
    `local_asset_references_resolve` checks every relative `src`/`href` resolves
    on disk; `social_card_images_exist_and_are_not_stale` checks the social-card
    URLs contain neither `webui-v2-empty` nor `/dev/`, ride the Pages origin, and
    map to an existing `docs/` file. Fails before the fix, passes after.
  - Gates green: `cargo build` / `clippy -D warnings` / `fmt` clean,
    `cargo test --locked` 1373 passed, `cairn scan` + `cairn hook all` clean.
    Pre-submit `/reforge` + `/debate` (independent reviewer subagent) returned
    PASS with no blocking findings; its two non-blocking notes (word-boundary
    attribute matching, `Vec::retain`) were applied. CI `check`, `webui`,
    `hooks`, `dogfood` and CodeRabbit all green (`claude-review` is the known
    non-blocking hang on unprotected `main`).
- **Beads export caught up.** The committed `.beads/issues.jsonl` on `main` was
  stale: it still showed `cairn-81c` open and lacked `cairn-s2t`. Regenerating
  via `bd export` closed `cairn-s2t`, flipped `cairn-81c` to closed (it merged
  last iteration), and added the missing audit lines. This also fixes the
  `cairn context` backlog reader, which had been surfacing the stale `cairn-81c`.

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on `main`.
- No open PRs.
- Backlog: all remaining work is blocked on a maintainer decision.

  | Bead | Unit | Why it is blocked |
  | --- | --- | --- |
  | `cairn-y7p` | webui AI fix loop (remainder) | the deterministic design-token slice landed (PR #145 + #146). The remaining browser -> AI-critique -> patch loop needs (a) sanction for a Node + Playwright/puppeteer toolchain in this `package.json`-less Rust repo and (b) an AI vision provider/API (none exists; likely paid; conflicts with the deterministic-gates convention). A standalone a11y static-audit of `src/ui_assets` is a possible smaller unit, but is a fragile/open-ended gate (the DOM is preact-rendered in `app.js`), so it was not taken. |
  | `cairn-2z9` | spike: beads as first-class task layer | `in_progress`; its ruling bends the markdown-artefact invariant (`spec.md:11`), a maintainer architectural call. |

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
- The landing page's asset references are now pinned by `tests/landing_assets.rs`;
  add new local assets under a path that resolves from `docs/landing/`.
