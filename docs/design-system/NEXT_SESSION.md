# Next session handoff notes

Outstanding feedback on the visual pass shipped in commit `f040288`. Future agents: read this first before making UI changes in this repo.

## Landing page: cognitive-overload on hero

The current blueprint-sheet SVG at the top of `docs/landing/index.html` (the "SHEET · 01 / AUTH · NEIGHBOURHOOD" inline SVG in the hero) is too cramped. Too many nested labels, dimension lines, scale bar, title block, legend, benchmark markers, and module cards all fighting for attention in a small viewport slot. It reads as chaotic rather than precise.

The user pointed at the **Blueprint → Map** side-by-side panel (the "authored · cairn.blueprint" vs "reconciled · map.md" two-column block further down the page) as the direction to pull the rest of the landing toward. That panel is calm, spacious, each row has room, and the hierarchy is obvious.

### What to change
- Simplify the hero SVG drastically. Fewer labels. No legend in the sheet. No dimension lines. Keep the modules, the AUTH boundary, and a single drift indicator. Nothing else.
- Increase vertical breathing room throughout the landing. Section padding is currently using `--s-10` (128px) which sounds large but visually still feels compressed because content inside sections is dense. Audit inner spacing.
- Let the reader's eye land on one thing per section, not six.
- Keep the Blueprint → Map panel as the visual benchmark. Other sections should match its calm-density ratio, not exceed it.

## Webui: inspiration reference

User wants the CAIRN webui (`src/ui_assets/`) to take visual inspiration from:

> https://claude.ai/design/p/d25f8bd4-7c08-43a3-b2be-ff78f7edc2b5?file=index.html

This is a Claude Design share link. Future agent should fetch that bundle (same pattern as the two bundles already extracted into `/tmp/claude-501/cairn-design/` in the prior session: WebFetch against the share URL returns a gzipped tar archive, extract with `tar -xzf`, read `README.md` first, then follow the design prototype files).

### Constraints
- Apply that inspiration through the **existing CAIRN design system** (`docs/design-system/tokens.css` + `components.css`). Do not fork tokens or introduce a new palette. If the reference uses a different color system, translate it through the warm-stone / ink / chain-accent token vocabulary we already have.
- Preserve all functional behaviour in `src/ui_assets/app.js`. Do not modify the JS layer.
- Preserve the 5 test-critical strings: `Graph Explorer`, `detail-panel`, `loadArtefacts`, `renderArtefacts`, `just now`. They are matched verbatim by `tests/graph_explorer.rs`.
- Cognitive overload is the governing concern here too. If the reference design is denser than our current webui, err toward our calmer direction. The blueprint → map panel is the density reference for all CAIRN surfaces.

## Workflow

When the next session picks this up:

1. Read this file first.
2. Fetch the referenced Claude Design bundle.
3. Propose a plan before implementing (the user prefers exploring before diving in for non-trivial UI work).
4. Dispatch implementation agents with `general-purpose` subagent_type (not `feature-dev:code-architect`, which lacks Write tools).
5. Verify `cargo build`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check` all green before committing.
6. Commit cleanly (no unstaged files left) so cflx runs can start from a clean tree.
