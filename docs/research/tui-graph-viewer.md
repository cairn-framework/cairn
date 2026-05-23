# TUI Graph Viewer (issue #66)

Exploration of a terminal-native graph viewer as an alternative to the webui.

## Compatibility

The compatibility floor is vanilla terminals with no graphics extensions.
Anything that works in a basic ssh session or CI log must degrade to ASCII
art or compact text. Terminal graphics protocols (Kitty, iTerm2 inline
images, Sixel) are nice-to-have optimisations, not requirements.

| Tier | Target | Rendering |
|---|---|---|
| Must | xterm, macOS Terminal, Linux console | ASCII box-drawing + text |
| Should | iTerm2, Windows Terminal | Unicode box-drawing, optional 256-colour |
| Nice | Kitty, WezTerm, Ghostty | Graphics protocol for node icons or mini graphs |

The detection strategy: probe `TERM`, `TERM_PROGRAM`, and Kitty's
`QUERY_TERMINAL` OSC. If the terminal claims a graphics protocol, use it.
Otherwise fall back to ASCII. No runtime dependency on terminal-specific
libraries.

## Library choice

Three options were evaluated.

| Option | Pros | Cons |
|---|---|---|
| **ratatui** + custom graph layout | Full control, no JS, fast compile | Layout engine is ours to build |
| **mermaid-cli** (puppeteer/headless) | Rich output, proven graph types | Heavy dep (Chromium), slow, fragile |
| **termgraph** / similar Rust ASCII libs | Lightweight, purpose-built | Narrow feature set, may not support our graph model |

Recommended: **ratatui** for the TUI framework plus a small graph-to-ASCII
layout pass. CAIRN already has the graph data (nodes, edges, kinds) in Rust;
running it through a JS pipeline would be circular. ratatui handles input,
focus, and rendering. The layout pass is a simple tiered list:

```
cairn
├── kernel
│   ├── map
│   └── scanner
└── ui
    └── server
```

For mermaid output (useful for copy-paste into markdown), a separate
`cairn export --mermaid` command is cheaper than a full in-terminal mermaid
renderer. The TUI viewer and the export command can share the same
graph-to-text formatter.

## Scope

**Single-screen "current node + first-hop neighbours"** is the right scope
for a TUI. Full graph traversal with pan and zoom is what the webui is for.
The TUI viewer is an agent and quick-human tool, not a replacement for the
browser.

Target interaction model:

- Launch: `cairn graph <node>` opens a TUI showing the node, its properties,
  and its immediate dependencies/dependents.
- Navigate: arrow keys or `j/k` move between neighbours; `Enter` dives into
  the selected node; `q` or `Esc` quits.
- Filter: `f` toggles edge kind (ownership, dependency, provenance).
- Copy: `c` copies the current node's mermaid fragment to clipboard.

This keeps the implementation small (one screen, no scrolling canvas) and
fast to start (<100ms).

## Pros and cons

**Pros:**
- Faster than alt-tab to browser for quick lookups.
- No server process; works in ssh, CI, and container shells.
- Agent-friendly: an agent running `cairn graph <node>` gets structured text
  it can parse without starting a headless browser.
- Natural pair to `cairn context` and `cairn neighbourhood --json`.

**Cons:**
- Terminal compatibility is fragmented; graphics protocols are not universal.
- Narrow visual range: no colour-coded severity badges, no chain-balance
  widget, no interactive filtering beyond simple toggles.
- Another surface to maintain: keyboard shortcuts, accessibility, resizing.
- The webui already exists and is good; the TUI is a convenience, not a
  necessity.

## Verdict

Worth a small feasibility prototype (`cairn graph <node>` using ratatui with
ASCII fallback) to validate the interaction model. If the prototype feels
natural after a week of dogfooding, promote to a full CLI command. If not,
archive the note and keep the webui as the single supported viewer.

Blocked on: no blocking dependencies. Can start any time. Estimated effort:
2-3 days for prototype, 1-2 weeks for polished command.
