---
node: cairn.root
status: open
created: 2026-07-17
---
# Herdr Plugin Feasibility

## Problem

Cairn's live state (map, findings, backlog, agent activity) is invisible in
herdr unless a session hand-rolls a dashboard pane, as the wave dashboards of
2026-07-16/17 did with an ad-hoc JSON file plus a Python renderer. An
experimental herdr plugin (or plugin-like integration) could make cairn state
a first-class pane.

## Investigation questions (research first, no implementation)

1. Standalone versus harness-reliant: what can a herdr plugin do on its own
   (poll `cairn status/lint --json`, watch map.json, run `cairn watch`)
   versus what needs the AI harness to push events (tool-call hooks)?
2. What plugin/extension surface does herdr actually expose today (inspect
   `herdr --help`, pane report-agent / report-metadata, integration command
   group), and what is merely a pane running a process?
3. State-file fidelity: can the pane state deterministically match the files
   (map.json, meta/todos, .cairn/state) rather than an agent's claims: e.g.
   fswatch on map.json triggering re-render, or `cairn watch` as the pane
   process?
4. Deterministic updates on AI tool use: options for updating the dashboard
   whenever an agent calls cairn (MCP wrapper emitting events, shell shim,
   or harness hooks) and their portability across harnesses.
5. Which views earn a pane: wave/task dashboard, findings stream, map/graph
   summary, backlog. Rank by value against the hand-rolled dashboard.

## Deliverable

A research artefact (meta/research/) comparing the options with a
recommended increment path, plus follow-up todos for whatever is worth
building. Experimental: cheap probes over designs.
