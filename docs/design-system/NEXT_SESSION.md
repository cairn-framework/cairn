# Next session handoff notes

State: landing calm-density pass landed (commit `5291360`). Webui CSS-only pass landed (commit `ab5f23c`). Both need a follow-up for reasons below.

## Webui: redo the pass with JS in play, against real CAIRN data

The first pass read "preserve functional behaviour in `app.js`" as "do not modify `app.js`" and dropped most of what made the reference mockup distinct. The next pass should widen the scope.

### What the mockup actually wanted (dropped last pass, needed next pass)

Bundle at `/tmp/claude-501/cairn-design-webui-ref/` (fetch via curl if gone; see bottom). Also pasted at `.claude/references/webui-design/` on disk locally (gitignored).

- **Hinge layout.** Not a flat 2-column graph. Provenance-heavy modules on the left, authority-heavy modules on the right, decisions on the central spine. Spatially encodes the two-chain model.
- **Hinge diagram in decision detail.** 3-column grid inside the inspector when a decision is selected: provenance sources left, the hinge decision as a vertical gradient bar center, authority contracts right.
- **Chain bars in inspector.** Two 4px progress tracks per node: provenance weight in amber, authority weight in sage. Low density, high signal.
- **Chain rails on every chain-aware card/row.** Not just `.node`. Artefact cards, decision rows, change entries all get the left-amber / right-sage 2px rails.
- **Decision chips on leader lines off graph nodes.** SVG overlay: small chips floating on lines pointing at decisions that shaped each node. Drops progressive disclosure onto the canvas.
- **Command trigger center bar.** Topbar slot needs to become a real `⌘K` search/query trigger, not just the meta strip.
- **Changes drawer.** Bottom dock that toggles open to show change cards horizontally. Surfaces pending changes without forcing a sidebar.

### Data source for next pass: CAIRN's own map, not a fake auth platform

The preview data I fed in last session (`src/ui_assets/api/{meta,graph,lint}`, gitignored) was a fictional auth module I invented. The webui's reason for existing is to show CAIRN's own reconciled map. Use one of:

1. **Live binary.** `cd test/fixtures/cairn-bootstrap && cairn ui` starts the real server against the bootstrap fixture (CAIRN describing itself). Preferred for visual iteration; guarantees the data shape matches what real users see.
2. **Captured snapshot.** Run `cairn ui` once, curl `/api/graph` + `/api/meta` + `/api/lint`, save JSON into `src/ui_assets/api/` (gitignored) for a fast static-preview loop. Necessary only if the binary path is blocked.

Mockup's `data.js` has a richer shape than CAIRN currently emits (systems > containers > modules, explicit chain weights, counts per artefact type, owner metadata, `lastTouched`). Decide early whether to:
- Expand CAIRN's kernel output to carry the missing fields (`chains.{provenance,authority}` weights, `counts.{decisions,contracts,changes,todos}`, `owner`, `lastTouched`), or
- Derive them client-side in `app.js` from what CAIRN already emits, or
- Render the hinge/chain visuals against what's available today and note the gaps.

Keep: the 5 test-critical strings. `Graph Explorer`, `detail-panel`, `loadArtefacts`, `renderArtefacts`, `just now`. `cargo test graph_explorer` must stay green.

## Landing: hero still doesn't sell what CAIRN does

Current hero trimmed to 4 auth modules inside an AUTH boundary. Calmer than before, but the user's exact read: "doesn't really give a sense of why someone should use CAIRN, or how it does its map. it's just 4 auth boxes connected to each other."

Plan we agreed: once the webui is rebuilt against the mockup + real data, use a screenshot of the actual webui as the hero image. The hero then literally shows the product rather than inventing a diagram for it.

Blocker: webui has to be visually real before that screenshot is worth pulling into the hero. So the webui pass above is the gating item.

Also still open: `Read the spec` buttons (hero x2, footer x1) open `docs/spec.md` as raw markdown because `.nojekyll` disables Jekyll rendering. Fix is either repointing at the GitHub blob URL or generating an HTML-rendered spec. User hasn't picked a direction.

## Constraints that carry forward

- Use only `docs/design-system/tokens.css` + `components.css`. No new palette.
- No em-dashes in any prose. Replace with period, colon, comma, parenthesis.
- Font authority: Source Serif 4 + IBM Plex Mono + IBM Plex Sans.
- Calm density still governs. The blueprint->map side-by-side panel on the landing is the benchmark.

## Workflow

1. Read this file first.
2. Read `.claude/references/webui-design/` (pasted locally, gitignored) OR refetch the bundle:
   ```
   mkdir -p /tmp/claude-501/cairn-design-webui-ref && \
   curl -sL "https://api.anthropic.com/v1/design/h/eyPs2Wy_c2yH_lqQXerZUQ?open_file=index.html" \
     -o /tmp/claude-501/cairn-design-webui-ref/bundle.tar.gz && \
   tar -xzf /tmp/claude-501/cairn-design-webui-ref/bundle.tar.gz \
     -C /tmp/claude-501/cairn-design-webui-ref/
   ```
3. Verify visually as you go using the `claude-in-chrome` MCP (not Playwright; the plugin drives the user's real Chrome). Start `python3 -m http.server 8767` in `src/ui_assets/` after creating the `assets/` + `api/` scratch dirs, or just run `cairn ui` in the bootstrap fixture.
4. Verify `cargo build`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`, `cargo test` all green before committing.
5. Atomic commits per track. Don't bundle landing + webui in one commit.
