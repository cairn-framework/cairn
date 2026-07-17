---
node: cairn.root
status: blocked
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
sessions, documented for reuse.
