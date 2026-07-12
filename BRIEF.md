# Brief: Cairn Graph Explorer

## What cairn is

Cairn is a tool that maps a codebase's architecture as a navigable graph. It
models systems, containers, modules, and actors as nodes connected by
dependency edges. Each node carries depth: code targets, contracts, and linked
artefacts (decisions, todos, research), plus a temporal history of changes and
decision lineage. Cairn continuously reconciles the declared map against the
real code and reports drift as findings.

## Purpose of the graph explorer (the surface to design)

A read-only web surface for inspecting the architecture graph: nodes, their
relationships (dependency edges, containment), each node's reconciliation
state (synced, ghost, orphaned), its findings, and the decisions and research
linked to it. The explorer answers: what exists, how is it connected, what is
its state, and why is it shaped this way.

It is an orientation instrument, not an editor. Users open it to understand a
system and to judge what is safe to change.

## Target users

- The maintainer of a growing Rust monorepo, checking project health and drift.
- Contributors (human and AI) orienting themselves in an unfamiliar codebase
  before making a change: finding a node, reading its rationale, tracing its
  dependents.

## Data

Real frozen data is provided and must be rendered, not replaced with toy
samples:

- `map.json` at the repo root: the real frozen architecture graph.
- `harness/fixtures/api/`: captured responses for `/api/status`, `/api/graph`,
  `/api/node/*`, `/api/depends/*`, `/api/dependents/*`, `/api/lint`,
  `/api/meta`, `/api/blueprint`.
- `harness/fixtures/assets/copy.json`: user-facing copy strings.

Treat the fixtures as read-only.

## Constraints

- Static HTML mocks, openable directly in a browser with no build step.
- No live network access.
- Plain-language, no em-dashes in user-facing copy.
