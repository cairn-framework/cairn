---
node: cairn.root
status: done
created: 2026-07-17
---
# Herdr Live Dashboard

## Problem

The wave dashboard used in the 2026-07-16/17 sessions tracked task/agent
phase and runtime from a hand-maintained JSON file: useful, but it shows the
orchestrator's claims, not cairn's ground truth, and it dies with the
session.

## Task (depends on todo.herdr-plugin-feasibility findings)

Build the smallest durable version: a pane process that renders cairn state
deterministically from files/commands (status, findings, backlog, optionally
a map summary), updating on change rather than on agent assertion. Reuse the
wave-dashboard renderer pattern if it survives the feasibility comparison.
Keep the orchestrator's task/phase overlay as a separate, clearly-labelled
layer if kept at all.

## Acceptance

A dashboard pane whose findings/backlog counts always match `cairn lint
--json` and `cairn status --json` output at read time, surviving across
sessions, documented for reuse. Meet this by re-deriving both command outputs
as part of each rendered snapshot (the pane never shows a count it did not
just derive), displaying the collection time alongside the counts.

## Findings pointer

`res.herdr-plugin-feasibility` probed the options and recommends this unit
proceed as a hybrid standalone pane: consume `cairn watch` for finding deltas
(ground truth; it polls internally every `--interval` and emits only the
delta) and poll `cairn status --json` for backlog and next-step state, because
`cairn watch` emits finding events only and cannot
keep backlog matched to this todo's acceptance. The renderer calls `herdr pane
report-metadata --token errors=N --token warnings=N --token info=N --token
todos=N` so counts surface at workspace level. fswatch is unavailable and the
map.json re-render path is unwired today, so `cairn watch` plus a status poll
is the substitute. Per-tool attribution is a separate follow-up
(todo.herdr-cairn-tool-attribution), not a blocker for this unit.
